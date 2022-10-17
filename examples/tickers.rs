use coinpaprika_api::client::Client;
use coinpaprika_api::tickers::{HistoricalTick, Ticker};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    //
    // Get tickers.
    //
    let tickers: Vec<Ticker> = client.tickers().quotes(vec!["BTC", "USD"]).send().await?;

    println!("second ticker: {:#?}", tickers[1]);

    //
    // Get ticker by coin_id.
    //
    let ticker: Ticker = client
        .ticker("btc-bitcoin")
        .quotes(vec!["PLN"])
        .send()
        .await?;

    println!("btc ticker: {:#?}", ticker);

    //
    // Get historical ticks by coin_id.
    //
    let historical_ticks: Vec<HistoricalTick> = client
        .historical_ticks("btc-bitcoin")
        .start("2022-10-16")
        .end("2022-10-17")
        .interval("1d")
        .limit(10)
        .quote("btc")
        .send()
        .await?;

    println!("historical ticks: {:#?}", historical_ticks);

    Ok(())
}
