use dotenv::dotenv;
use rig_twitter::scraper::Scraper;
use rig_twitter::search::SearchMode;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let mut scraper = Scraper::new().await?;
    let cookie_cache =
        fs::read_to_string(&env::var("TWITTER_COOKIE_STR")?).expect("Failed to read cookie");

    println!("cookie_cache");

    scraper.set_from_cookie_string(&cookie_cache).await?;
    println!("set_cookies");

    // scraper
    //     .login(
    //         env::var("TWITTER_USERNAME")?,
    //         env::var("TWITTER_PASSWORD")?,
    //         Some(env::var("TWITTER_EMAIL")?),
    //         Some(env::var("TWITTER_2FA_SECRET")?),
    //     )
    //     .await?;

    // Search tweets
    // let tweets = scraper
    //     .search_tweets("spacex", 10, SearchMode::Latest, None)
    //     .await?;

    // println!(
    //     "Search tweets result: {}",
    //     serde_json::to_string(&tweets).unwrap()
    // );

    scraper.send_tweet("Hello, Twitter!", None, None).await?;

    // println!("login successful");

    // let c_s = scraper.get_cookie_string().await.unwrap();

    // let file_path = Path::new("../twitter_cookie_str");

    // let mut file = File::create(file_path)?;

    // file.write_all(c_s.as_bytes())?;

    // println!("String has been saved to {:?}", file_path);

    // Search user profiles
    // let profiles = scraper.search_profiles("rust", 20, None).await?;

    Ok(())
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let mut scraper = Scraper::new().await?;

    println!(
        "TWITTER_USERNAME: {}",
        env::var("TWITTER_USERNAME").unwrap()
    );

    scraper
        .login(
            env::var("TWITTER_USERNAME")?,
            env::var("TWITTER_PASSWORD")?,
            Some(env::var("TWITTER_EMAIL")?),
            Some(env::var("TWITTER_2FA_SECRET")?),
        )
        .await?;
    println!("login successful");

    scraper
        .save_cookies("../twitter_account_cookies")
        .await
        .unwrap();

    Ok(())
}
