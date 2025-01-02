use rig::{completion::ToolDefinition, tool::Tool};
use std::{str::FromStr, sync::Arc};

use crate::chains::get_chain_info;
use alloy::{
    network::EthereumWallet,
    primitives::{Address, TxHash, B256, U256},
    providers::{ProviderBuilder, RootProvider},
    signers::local::PrivateKeySigner,
    sol,
    transports::http::{Client, Http},
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

const MAX_AMOUNT: u128 = 10u128.pow(5);

#[derive(Deserialize)]
pub struct TransferArgs {
    chain: String,
    token_address: String,
    to_address: String,
    amount: String,
}

#[derive(Debug, thiserror::Error)]
#[error("ERC20 error")]
pub struct ERC20Error {
    message: String,
}

sol! {
    #[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
    #[sol(rpc)]
    interface IERC20 {
        function transfer(address to, uint256 amount) public returns (bool);
        function decimals() public view returns (uint8);
    }
}

#[derive(Deserialize, Serialize)]
pub struct ERC20Transfer;
impl Tool for ERC20Transfer {
    const NAME: &'static str = "erc20_transfer";

    type Error = ERC20Error;
    type Args = TransferArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "erc20_transfer".to_string(),
            description: "Transfer ERC20 tokens to a specific address".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "token_address": {
                        "type": "string",
                        "description": "The address of the ERC20 token contract"
                    },
                    "chain": {
                        "type": "string",
                        "description": "The chain name, such as arbitrum"
                    },
                    "to_address": {
                        "type": "string",
                        "description": "The receiving address"
                    },
                    "amount": {
                        "type": "string",
                        "description": "The amount of tokens to transfer"
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let chain_name = args.chain;
        let token_address = Address::from_str(&args.token_address).unwrap();
        let to_address = Address::from_str(&args.to_address).unwrap();
        let amount = u128::from_str(&args.amount).unwrap_or_default();
        println!(
            "chain_name: {}, token_address: {}, to_address: {}, amount: {}",
            chain_name, token_address, to_address, amount
        );

        if amount > MAX_AMOUNT {
            println!(
                "amount = {} exceeds the safe value = {}",
                amount, MAX_AMOUNT
            );
            return Err(ERC20Error {
                message: format!(
                    "amount = {} exceeds the safe value = {}",
                    amount, MAX_AMOUNT
                )
                .to_string(),
            });
        }

        let provider_url = get_chain_info(&chain_name)
            .ok_or(ERC20Error {
                message: "get_chain_info none".to_string(),
            })?
            .provider_url;

        let result = transfer_erc20(to_address, amount, token_address, provider_url).await;
        match result {
            Ok(h) => Ok(h.to_string()),
            Err(e) => Err(ERC20Error {
                message: format!("transfer_erc20 error: {}", e),
            }),
        }
    }
}

async fn transfer_erc20(
    to_address: Address,
    amount: u128,
    token_address: Address,
    provider_url: String,
) -> std::result::Result<B256, anyhow::Error> {
    // Read the private key from the environment variable
    // let private_key = env::var("PRIVATE_KEY").unwrap();

    // [RISK WARNING! Writing a private key in the code file is insecure behavior.]
    // The following code is for testing only. Set up signer from private key, be aware of danger.
    let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let signer: PrivateKeySigner = private_key.parse().expect("parse PrivateKeySigner");
    let wallet: EthereumWallet = EthereumWallet::from(signer.clone());

    // Create a http client to the EVM chain network.
    let provider: RootProvider<Http<Client>> =
        ProviderBuilder::new().on_http(provider_url.parse().expect("parse l1_rpc to Url"));

    // Create eth signer.
    let signer = Arc::new(
        ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_provider(provider.clone()),
    );

    // Create contract instance.
    let erc20 = IERC20::IERC20Instance::new(token_address, signer);

    // Sync send transfer call.
    let tx_hash: std::result::Result<TxHash, anyhow::Error> = async move {
        let handle = tokio::task::spawn_blocking(move || {
            let result = tokio::runtime::Handle::current().block_on(async {
                let decimal = erc20.decimals().call().await.unwrap()._0;
                erc20
                    .transfer(to_address, U256::from(amount * 10u128.pow(decimal.into())))
                    .send()
                    .await
            });
            result
        });
        match handle.await {
            Ok(Ok(tx)) => Ok(tx.tx_hash().clone()),
            Ok(Err(e)) => Err(anyhow!(format!("alloy rpc error: {}", e))), // sign_transaction
            Err(e) => Err(anyhow!(format!("tokio exec error: {}", e))),    // spawn_blocking
        }
    }
    .await;
    tx_hash
}

#[tokio::test]
async fn test_transfer_erc20() -> Result<()> {
    let to_address = Address::from_str("1CBd0109c7452926fC7cCf06e73aCC505A296cc7").unwrap();
    let token_address = Address::from_str("5FbDB2315678afecb367f032d93F642f64180aa3").unwrap();
    let tx_hash = transfer_erc20(
        to_address,
        10,
        token_address,
        String::from("http://localhost:8545"),
    )
    .await;
    println!("tx_hash:{}", tx_hash.unwrap().to_string());
    Ok(())
}

#[tokio::test]
async fn test_run() -> Result<()> {
    use crate::chains::CHAIN_INFOS;
    use rig::completion::Prompt;
    use rig::providers::openai;

    // Create OpenAI client and model
    let openai_client = openai::Client::from_url("sk-xxxxx", "https://api.xxxxx.xx/");

    //Qwen/Qwen2.5-32B-Instruct
    //Qwen/Qwen2.5-72B-Instruct-128K
    let transfer_agent = openai_client
        .agent("Qwen/Qwen2.5-32B-Instruct")
        .preamble("You are a transfer agent here to help the user perform ERC20 token transfers.")
        .context(&serde_json::to_string(&*CHAIN_INFOS).unwrap())
        .max_tokens(2048)
        .tool(ERC20Transfer)
        .build();

    // Prompt the agent and print the response
    println!("Transfer ERC20 tokens");
    println!(
        "Transfer Agent: {}",
        transfer_agent
            .prompt("Transfer 10 USDC to 0x1CBd0109c7452926fC7cCf06e73aCC505A296cc7 on base")
            .await?
    );
    Ok(())
}
