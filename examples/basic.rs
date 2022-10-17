use coinpaprika_api::client::Client;
use coinpaprika_api::tickers::Ticker;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let tickers: Vec<Ticker> = client
        .tickers()
        .with_quotes(vec!["BTC", "USD"])
        .send()
        .await?;

    println!("second ticker: {:#?}", tickers[1]);

    let ticker: Ticker = client
        .ticker("btc-bitcoin")
        .with_quotes(vec!["PLN"])
        .send()
        .await?;

    println!("btc ticker: {:#?}", ticker);

    Ok(())
}
