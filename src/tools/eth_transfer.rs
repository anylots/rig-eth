use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{utils::parse_ether, Address, TxHash, B256},
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    transports::http::{Client, Http},
};
use anyhow::{anyhow, Result};
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{str::FromStr, sync::Arc};

use crate::chains::get_chain_info;

const MAX_AMOUNT: u128 = 10u128; //maximum amount in ETH

#[derive(Deserialize)]
pub struct ETHTransferArgs {
    chain: String,
    to_address: String,
    amount: String,
}

#[derive(Debug, thiserror::Error)]
#[error("ETH transfer error")]
pub struct ETHTransferError {
    message: String,
}

#[derive(Deserialize, Serialize)]
pub struct ETHTransfer;
impl Tool for ETHTransfer {
    const NAME: &'static str = "eth_transfer";

    type Error = ETHTransferError;
    type Args = ETHTransferArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "eth_transfer".to_string(),
            description: "Transfer ETH to a specific address".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
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
                        "description": "The amount of ETH to transfer"
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let chain_name = args.chain;
        let to_address = Address::from_str(&args.to_address).unwrap();
        let amount = u128::from_str(&args.amount).unwrap_or_default();
        println!(
            "chain_name: {}, to_address: {}, amount: {}",
            chain_name, to_address, amount
        );

        if amount > MAX_AMOUNT {
            println!(
                "amount = {} exceeds the safe value = {}",
                amount, MAX_AMOUNT
            );
            return Err(ETHTransferError {
                message: format!(
                    "amount = {} exceeds the safe value = {}",
                    amount, MAX_AMOUNT
                )
                .to_string(),
            });
        }

        let provider_url = get_chain_info(&chain_name)
            .ok_or(ETHTransferError {
                message: "get_chain_info none".to_string(),
            })?
            .provider_url;

        let result = transfer_eth(to_address, amount, provider_url).await;
        match result {
            Ok(h) => Ok(h.to_string()),
            Err(e) => Err(ETHTransferError {
                message: format!("transfer_eth error: {}", e),
            }),
        }
    }
}

async fn transfer_eth(
    to_address: Address,
    amount: u128,
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

    // Sync send transfer call.
    let tx_hash: std::result::Result<TxHash, anyhow::Error> = async move {
        let handle = tokio::task::spawn_blocking(move || {
            let result = tokio::runtime::Handle::current().block_on(async {
                let tx = TransactionRequest::default()
                    .with_to(to_address)
                    .with_value(parse_ether(&amount.to_string()).unwrap_or_default());

                // Send the transaction and listen for the transaction to be included.
                signer.send_transaction(tx).await
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
async fn test_transfer_eth() -> Result<()> {
    let to_address = Address::from_str("1CBd0109c7452926fC7cCf06e73aCC505A296cc7").unwrap();
    let tx_hash = transfer_eth(to_address, 10, String::from("http://localhost:8545")).await;
    println!("tx_hash:{}", tx_hash.unwrap().to_string());
    Ok(())
}

#[tokio::test]
async fn test_run_eth() -> Result<()> {
    use crate::chains::CHAIN_INFOS;
    use rig::completion::Prompt;
    use rig::providers::openai;

    // Create OpenAI client and model
    let openai_client = openai::Client::from_url("sk-xxxxx", "https://api.xxxxx.xx/");

    //Qwen/Qwen2.5-32B-Instruct
    //Qwen/Qwen2.5-72B-Instruct-128K
    let transfer_agent = openai_client
        .agent("Qwen/Qwen2.5-32B-Instruct")
        .preamble("You are a transfer agent here to help the user perform ETH transfers.")
        .context(&serde_json::to_string(&*CHAIN_INFOS).unwrap())
        .max_tokens(2048)
        .tool(ETHTransfer)
        .build();

    // Prompt the agent and print the response
    println!("Transfer ETH");
    println!(
        "Transfer Agent: {}",
        transfer_agent
            .prompt("Transfer 10 ETH to 0x1CBd0109c7452926fC7cCf06e73aCC505A296cc7 on base")
            .await?
    );
    Ok(())
}
