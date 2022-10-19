use coinpaprika_api::client::Client;
use coinpaprika_api::exchanges::{Exchange, ExchangeMarket};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    //
    // Get exchanges.
    //
    let exchanges: Vec<Exchange> = client.exchanges().quotes(vec!["PLN"]).send().await?;

    println!(
        "first 3 exchanges: {:#?}",
        exchanges.iter().take(3).collect::<Vec<&Exchange>>()
    );

    //
    // Get "binance" exchange.
    //
    let exchange: Exchange = client
        .exchange("binance")
        .quotes(vec!["CZK", "USD"])
        .send()
        .await?;

    println!("exchange info: {:#?}", exchange);

    //
    // Get "binance" exchange markets.
    //
    let exchange_markets: Vec<ExchangeMarket> = client
        .exchange_markets("binance")
        .quotes(vec!["GBP"])
        .send()
        .await?;

    println!(
        "first 3 exchange markets: {:#?}",
        exchange_markets
            .iter()
            .take(3)
            .collect::<Vec<&ExchangeMarket>>()
    );

    Ok(())
}
