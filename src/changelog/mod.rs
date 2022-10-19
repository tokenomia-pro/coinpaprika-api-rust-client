use crate::client::{Client, Response};
use crate::error::Error;
use reqwest_middleware::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Change made by coinpaprika.com moderators
pub struct Change {
    pub currency_id: String,
    pub old_id: String,
    pub new_id: String,

    /// RFC3999 (ISO-8601) format
    pub changed_at: String,
}

/// Request for getting coin id changes made by coinpaprika.com moderators
/// [/changelog/ids](https://api.coinpaprika.com/#tag/Changelog/operation/getChangelogIDs)
pub struct GetChangelogRequest<'a> {
    client: &'a Client,
    page: i32,
}

impl<'a> GetChangelogRequest<'a> {
    pub fn new(client: &'a Client, page: i32) -> Self {
        Self { client, page }
    }

    pub async fn send(&self) -> Result<Vec<Change>, Error> {
        let query: Vec<(String, String)> = vec![("page".to_string(), self.page.to_string())];

        let request: RequestBuilder = self
            .client
            .client
            .get(format!("{}/changelog/ids", self.client.api_url))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Vec<Change> = response.response.json().await?;

        Ok(data)
    }
}
