use std::sync::Arc;

use alloy::{
    network::EthereumWallet,
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::{TransactionReceipt, TransactionRequest},
    signers::local::PrivateKeySigner,
    transports::http::{Client, Http},
};
use anyhow::anyhow;

pub async fn send_eoa_tx(
    request: TransactionRequest,
    provider: RootProvider<Http<Client>>,
) -> Result<TransactionReceipt, anyhow::Error> {
    // Read the private key from the environment variable
    // let private_key = env::var("PRIVATE_KEY").unwrap();

    // [RISK WARNING! Writing a private key in the code file is insecure behavior.]
    // The following code is for testing only. Set up signer from private key, be aware of danger.
    let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let signer: PrivateKeySigner = private_key.parse().expect("parse PrivateKeySigner");
    let wallet: EthereumWallet = EthereumWallet::from(signer.clone());

    // Create eth signer.
    let signer = Arc::new(
        ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_provider(provider.clone()),
    );

    let pending_tx = signer.send_transaction(request).await?;

    pending_tx
        .get_receipt()
        .await
        .map_err(|e| anyhow!(e.to_string()))
}
