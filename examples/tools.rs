use coinpaprika_api::client::Client;
use coinpaprika_api::tools::PriceConversion;
use serde_json::Value;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    //
    // Get search data.
    //
    let search_query: Value = client
        .search("btc")
        .c(vec!["currencies", "icos", "people"])
        .modifier("symbol_search")
        .limit(3)
        .send()
        .await?;

    println!("search query result: {:#?}", search_query);

    //
    // Get price conversion from btc to eth
    //
    let price_conversion: PriceConversion = client
        .price_convert("btc-bitcoin", "eth-ethereum")
        .amount(100)
        .send()
        .await?;

    println!("price conversion result: {:#?}", price_conversion);

    Ok(())
}
