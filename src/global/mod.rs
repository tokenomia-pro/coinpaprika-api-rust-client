use crate::client::{Client, Response};
use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Global {
    market_cap_usd: i64,
    volume_24h_usd: i64,
    bitcoin_dominance_percentage: f64,
    cryptocurrencies_number: i32,
    market_cap_ath_value: i64,
    market_cap_ath_date: String,
    volume_24h_ath_value: i64,
    volume_24h_ath_date: String,
    market_cap_change_24h: f64,
    volume_24h_change_24h: f64,
    last_updated: i64,
}

pub struct GetGlobalRequest<'a> {
    client: &'a Client,
}

impl<'a> GetGlobalRequest<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn send(&self) -> Result<Global, Error> {
        let request: reqwest::RequestBuilder = self
            .client
            .client
            .get(format!("{}/global", self.client.api_url));

        let response: Response = self.client.request(request).await?;

        let data: Global = response.response.json().await?;

        Ok(data)
    }
}
