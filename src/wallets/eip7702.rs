use std::sync::Arc;

use alloy::{
    consensus::SignableTransaction,
    dyn_abi::JsonAbiExt,
    json_abi::Function,
    network::{EthereumWallet, TransactionBuilder},
    primitives::{keccak256, TxKind},
    providers::{Provider, ProviderBuilder, RootProvider, WalletProvider},
    rpc::types::{TransactionReceipt, TransactionRequest},
    signers::{local::PrivateKeySigner, Signature, Signer},
    transports::http::{Client, Http},
};
use anyhow::anyhow;

// eip7702 tx
pub async fn send_7702_tx(
    request: TransactionRequest,
    provider: RootProvider<Http<Client>>,
) -> Result<TransactionReceipt, anyhow::Error> {
    // Read the private key from the environment variable
    // let private_key = env::var("PRIVATE_KEY").unwrap();
    let request_to_build = request.clone();

    // [RISK WARNING! Writing a private key in the code file is insecure behavior.]
    // The following code is for testing only. Set up signer from private key, be aware of danger.
    let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let priv_signer: PrivateKeySigner = private_key.parse().expect("parse PrivateKeySigner");
    let wallet: EthereumWallet = EthereumWallet::from(priv_signer.clone());

    let signer = Arc::new(
        ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_provider(provider.clone()),
    );

    let account = signer.default_signer_address();

    //  Access eoa's delegated code
    let input = Function::parse("nonce()")?.abi_encode_input(&vec![])?;
    let aa_req: TransactionRequest = TransactionRequest::default()
        .input(input.into())
        .to(account);
    let output = provider.call(&aa_req).await?;

    let mut preimage = vec![0u8];
    preimage.extend_from_slice(&output.to_vec());
    let to_address = match request.to.unwrap() {
        TxKind::Create => return Err(anyhow!("Need request.to param")),
        TxKind::Call(to) => to,
    };
    preimage.extend_from_slice(&to_address.to_vec());
    let data = request.input.data.unwrap();
    preimage.extend_from_slice(&data.to_vec());
    let value = request.value;
    preimage.extend_from_slice(&value.unwrap().to_be_bytes_vec());

    let digest = keccak256(&preimage);
    let sig: Signature = priv_signer.sign_hash(&digest).await?;

    let tx = request_to_build.build_unsigned().unwrap();
    let signed_tx = tx.eip1559().unwrap().clone().into_signed(sig);
    let pending_tx = signer.send_tx_envelope(signed_tx.into()).await?;

    pending_tx
        .get_receipt()
        .await
        .map_err(|e| anyhow!(e.to_string()))
}
