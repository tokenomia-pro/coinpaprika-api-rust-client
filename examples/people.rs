use coinpaprika_api::client::Client;
use coinpaprika_api::people::Person;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    //
    // Get person info.
    //
    let person: Person = client.person("vitalik-buterin").send().await?;

    println!("person info: {:#?}", person);

    Ok(())
}
