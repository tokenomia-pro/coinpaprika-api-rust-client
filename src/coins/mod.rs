use crate::client::{Client, Response};
use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Coin {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub rank: isize,
    pub is_new: bool,
    pub is_active: bool,

    #[serde(rename = "type")]
    pub coin_type: String,
}

pub struct GetCoinsRequest<'a> {
    client: &'a Client,
}

impl<'a> GetCoinsRequest<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn send(&self) -> Result<Vec<Coin>, Error> {
        let request: reqwest::RequestBuilder = self
            .client
            .client
            .get(format!("{}/coins", self.client.api_url));

        let response: Response = self.client.request(request).await?;

        let data: Vec<Coin> = response.response.json().await?;

        Ok(data)
    }
}
