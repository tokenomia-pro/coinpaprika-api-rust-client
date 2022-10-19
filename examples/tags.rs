use coinpaprika_api::client::Client;
use coinpaprika_api::tags::Tag;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    //
    // Get tags.
    //
    let tags: Vec<Tag> = client.tags().additional_fields(vec!["icos"]).send().await?;

    println!(
        "first 3 tags: {:#?}",
        tags.iter().take(3).collect::<Vec<&Tag>>()
    );

    //
    // Get "blockchain-service" tag.
    //
    let tag: Tag = client
        .tag("blockchain-service")
        .additional_fields(vec!["coins", "icos"])
        .send()
        .await?;

    println!("tag info: {:#?}", tag);

    Ok(())
}
