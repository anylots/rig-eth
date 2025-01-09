use std::{env, fs};

use anyhow::{anyhow, Result};
use rig::{completion::ToolDefinition, tool::Tool};
use rig_twitter::scraper::Scraper;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct TweetArgs {
    tweet: String,
}

#[derive(Debug, thiserror::Error)]
#[error("TWToolError: {message}")]
pub struct TWToolError {
    message: String,
}

#[derive(Deserialize, Serialize)]
pub struct TwitterTool;
impl Tool for TwitterTool {
    const NAME: &'static str = "twitter_tool";

    type Error = TWToolError;
    type Args = TweetArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "twitter_tool".to_string(),
            description: "Post a Tweet".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "tweet": {
                        "type": "string",
                        "description": "The content of the tweet to be published"
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let tweet = args.tweet;
        println!("tweet: {}", tweet);

        // Initialize components
        let mut scraper = Scraper::new().await.unwrap();
        let cookie_cache = fs::read_to_string(&env::var("TWITTER_COOKIE_STR").unwrap())
            .expect("Failed to read cookie");

        // Sync send send_tweet.
        let tw_result: std::result::Result<String, anyhow::Error> = async move {
            let handle = tokio::task::spawn_blocking(move || {
                let result = tokio::runtime::Handle::current().block_on(async {
                    scraper.set_from_cookie_string(&cookie_cache).await.unwrap();
                    scraper.send_tweet(&tweet, None, None).await
                });
                result
            });
            match handle.await {
                Ok(Ok(m)) => Ok(m.to_string()),
                Ok(Err(e)) => Err(anyhow!(format!("twitter api error: {}", e))),
                Err(e) => Err(anyhow!(format!("tokio exec error: {}", e))),
            }
        }
        .await;
        match tw_result {
            Ok(m) => Ok(m.to_string()),
            Err(e) => Err(TWToolError {
                message: format!("twTool error: {}", e),
            }),
        }
    }
}

#[tokio::test]
async fn test_run_twitter() -> Result<()> {
    use dotenv::dotenv;
    use rig::completion::Prompt;
    use rig::providers::openai;

    dotenv().ok();

    // Create OpenAI client and model
    let openai_client = openai::Client::from_url("sk-xxxxx", "https://api.xxxxx.xx/");

    let twitter_agent = openai_client
        .agent("Qwen/Qwen2.5-32B-Instruct")
        .preamble("You are a Twitter agent, helping users to post tweets")
        .max_tokens(2048)
        .tool(TwitterTool)
        .build();

    // Prompt the agent and print the response
    println!("Twitter Agent");
    println!(
        "Twitter Agent: {}",
        twitter_agent.prompt("Please say hello to everyone").await?
    );
    Ok(())
}
