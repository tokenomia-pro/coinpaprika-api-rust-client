use coinpaprika_api::client::Client;
use coinpaprika_api::contracts::Contract;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    //
    // Get contract platforms.
    //
    let contract_platforms: Vec<String> = client.contract_platforms().send().await?;

    println!(
        "first 3 contract platforms: {:#?}",
        contract_platforms.iter().take(3).collect::<Vec<&String>>()
    );

    //
    // Get all contracts on eth.
    //
    let contracts: Vec<Contract> = client.contracts("eth-ethereum").send().await?;

    println!(
        "first 3 contracts on eth: {:#?}",
        contracts.iter().take(3).collect::<Vec<&Contract>>()
    );

    Ok(())
}
