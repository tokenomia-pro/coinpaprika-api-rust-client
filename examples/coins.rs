use coinpaprika_api::client::Client;
use coinpaprika_api::coins::{Coin, CoinDetails, CoinEvent, CoinExchange, CoinMarket, Tweet};
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

    //
    // Get coin events by coin_id.
    //
    let coin_events: Vec<CoinEvent> = client.coin_events("btc-bitcoin").send().await?;

    println!(
        "first 3 coin events: {:#?}",
        coin_events.iter().take(3).collect::<Vec<&CoinEvent>>()
    );

    //
    // Get coin exchanges by coin_id.
    //
    let coin_exchanges: Vec<CoinExchange> = client.coin_exchanges("btc-bitcoin").send().await?;

    println!(
        "first 3 coin exchanges: {:#?}",
        coin_exchanges
            .iter()
            .take(3)
            .collect::<Vec<&CoinExchange>>()
    );

    //
    // Get coin markets by coin_id.
    //
    let coin_markets: Vec<CoinMarket> = client
        .coin_markets("btc-bitcoin")
        .quotes(vec!["PLN", "CZK"])
        .send()
        .await?;

    println!(
        "first 3 coin markets: {:#?}",
        coin_markets.iter().take(3).collect::<Vec<&CoinMarket>>()
    );

    Ok(())
}
