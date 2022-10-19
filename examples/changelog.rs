use coinpaprika_api::changelog::Change;
use coinpaprika_api::client::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::with_key("<api-key>");

    //
    // Get lates changes made by coinpaprika.com moderators.
    //
    let page: i32 = 1;
    let changes: Vec<Change> = client.changelog(page).send().await?;

    println!(
        "first 3 changes from page {}: {:#?}",
        page,
        changes.iter().take(3).collect::<Vec<&Change>>()
    );

    Ok(())
}
