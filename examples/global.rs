use coinpaprika_api::client::Client;
use coinpaprika_api::global::Global;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    //
    // Get global market data.
    //
    let global: Global = client.global().send().await?;

    println!("global: {:#?}", global);

    Ok(())
}
