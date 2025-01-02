use std::{collections::HashMap, fs};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChainInfo {
    pub chain: String,
    #[serde(skip_serializing)]
    pub provider_url: String,
    pub tokens: HashMap<String, String>, // token_symbol => token_address
    pub swap_router: String,
}

pub static CHAIN_INFOS: Lazy<Vec<ChainInfo>> = Lazy::new(|| {
    let content = fs::read_to_string("configs/chains.json").expect("Failed to read chains.json");
    serde_json::from_str(&content).expect("Failed to parse JSON")
});

pub fn get_chain_info(chain_name: &str) -> Option<ChainInfo> {
    CHAIN_INFOS
        .iter()
        .find(|info| info.chain == chain_name)
        .cloned()
}
