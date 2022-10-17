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

    println!("first ticker: {:#?}", tickers[0]);

    Ok(())
}
