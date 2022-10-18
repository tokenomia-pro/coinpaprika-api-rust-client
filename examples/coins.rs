use coinpaprika_api::client::Client;
use coinpaprika_api::coins::{
    Coin, CoinDetails, CoinEvent, CoinExchange, CoinMarket, CoinOHLC, Tweet,
};
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

    //
    // Get coin open/high/low/close for the last full day by coin_id.
    //
    let coin_ohlc_last_full_day: Vec<CoinOHLC> = client
        .coin_ohlc_last_full_day("btc-bitcoin")
        .quote("usd")
        .send()
        .await?;

    println!(
        "bitcoin ohlc for last full day: {:#?}",
        coin_ohlc_last_full_day
    );

    //
    // Get historical open/high/low/close by coin_id.
    //
    let coin_ohlc_historical: Vec<CoinOHLC> = client
        .coin_ohlc_historical("btc-bitcoin")
        .start("2022-10-18")
        .end("2022-10-18")
        .limit(2)
        .quote("btc")
        .send()
        .await?;

    println!("{:#?}", coin_ohlc_historical);

    //
    // Get coin open/high/low/close for today by coin_id.
    //
    let coin_ohlc_today: Vec<CoinOHLC> = client
        .coin_ohlc_today("btc-bitcoin")
        .quote("btc")
        .send()
        .await?;

    println!("bitcoin ohlc for today: {:#?}", coin_ohlc_today);

    Ok(())
}
