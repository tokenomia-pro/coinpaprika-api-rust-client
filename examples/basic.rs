use coinpaprika_api::client::{Client, Ticker};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let tickers: Vec<Ticker> = client.tickers(None).await?;

    println!("first ticker: {:#?}", tickers[0]);

    Ok(())
}
