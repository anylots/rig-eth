use std::env;

use alloy::{
    providers::RootProvider,
    rpc::types::{TransactionReceipt, TransactionRequest},
    transports::http::{Client, Http},
};
use anyhow::anyhow;
use eip7702::send_7702_tx;
use local::send_eoa_tx;
use once_cell::sync::Lazy;

pub mod eip7702;
pub mod local;

pub static ACCONT_TYPE: Lazy<String> = Lazy::new(|| env::var("ACCONT_TYPE").unwrap());

pub async fn send_tx(
    request: TransactionRequest,
    provider: RootProvider<Http<Client>>,
) -> Result<TransactionReceipt, anyhow::Error> {
    match ACCONT_TYPE.as_str() {
        "eip7702" => send_7702_tx(request, provider).await,
        "local" => send_eoa_tx(request, provider).await,
        _ => return Err(anyhow!("unknown account type")),
    }
}
