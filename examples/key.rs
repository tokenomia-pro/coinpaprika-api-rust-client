use coinpaprika_api::client::Client;
use coinpaprika_api::key::KeyInfo;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::with_key("<api-key>");

    //
    // Get key info.
    //
    let key: KeyInfo = client.key_info().send().await?;

    println!("key info: {:#?}", key);

    Ok(())
}
