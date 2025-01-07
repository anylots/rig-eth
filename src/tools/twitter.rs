use std::{env, fs};

use rig::{completion::ToolDefinition, tool::Tool};
use rig_twitter::scraper::Scraper;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TransferArgs {
    chain: String,
    token_address: String,
    to_address: String,
    amount: String,
}

#[derive(Debug, thiserror::Error)]
#[error("TWProcError error")]
pub struct TWProcError {
    message: String,
}

#[derive(Deserialize, Serialize)]
pub struct TwitterTool;
impl Tool for TwitterTool {
    const NAME: &'static str = "twProc";

    type Error = TWProcError;
    type Args = TransferArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        todo!()
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Initialize components
        let mut scraper = Scraper::new().await.unwrap();
        let cookie_cache = fs::read_to_string(&env::var("TWITTER_COOKIE_STR").unwrap())
            .expect("Failed to read cookie");

        println!("cookie_cache");

        // scraper.set_from_cookie_string(&cookie_cache).await.unwrap();
        // println!("set_cookies");

        // scraper
        //     .send_tweet("Hello, Twitter!", None, None)
        //     .await
        //     .unwrap();

        Ok(String::from("send_tweet"))
    }
}
