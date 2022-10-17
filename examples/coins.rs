use coinpaprika_api::client::Client;
use coinpaprika_api::coins::{Coin, CoinDetails, Tweet};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    //
    // Get coins.
    //
    let coins: Vec<Coin> = client.coins().send().await?;

    println!(
        "first 3 coins: {:#?}",
        coins.iter().take(3).collect::<Vec<&Coin>>()
    );

    //
    // Get coin by coin_id.
    //
    let coin: CoinDetails = client.coin("btc-bitcoin").send().await?;

    println!("btc coin: {:#?}", coin);

    //
    // Get coin tweets by coin_id.
    //
    let tweets: Vec<Tweet> = client.twitter("btc-bitcoin").send().await?;

    println!(
        "first 3 tweets: {:#?}",
        tweets.iter().take(3).collect::<Vec<&Tweet>>()
    );

    Ok(())
}
