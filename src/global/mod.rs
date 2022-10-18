use crate::client::{Client, Response};
use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Global market overview data
pub struct Global {
    /// Total market capitalization - sum of all cryptocurrency market capitalizations in USD
    pub market_cap_usd: i64,

    /// Total 24h volume - sum of all cryptocurrency volumes in USD
    pub volume_24h_usd: i64,

    /// Bitcoin market capitalization as a percentage of total market capitalization
    pub bitcoin_dominance_percentage: f64,

    /// This is number of active cryptocurrencies on our site (active in this case means that we
    /// have up-to-date price data for a coin). Total number of our cryptocurrencies is higher and
    /// may be obtained via counting elements in /coins endpoint.
    pub cryptocurrencies_number: i32,

    /// ATH (All Time High) value of market capitalization - the highest historical value of
    /// marketcap
    pub market_cap_ath_value: i64,

    /// ATH (All Time High) date of market capitalization
    pub market_cap_ath_date: String,

    /// ATH (All Time High) value of the 24h volume - the highest historical value of 24h volume
    pub volume_24h_ath_value: i64,

    /// ATH (All Time High) date of volume 24h
    pub volume_24h_ath_date: String,

    /// Percentage change in the market capitalization over the last 24h
    pub market_cap_change_24h: f64,

    /// Percentage change in the volume 24h over the last 24h
    pub volume_24h_change_24h: f64,

    /// Timestamp of the last data update
    pub last_updated: i64,
}

/// Request for getting global market overview data
/// [/global](https://api.coinpaprika.com/#tag/Global/paths/~1global/get)
pub struct GetGlobalRequest<'a> {
    client: &'a Client,
}

impl<'a> GetGlobalRequest<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn send(&self) -> Result<Global, Error> {
        let request: reqwest_middleware::RequestBuilder = self
            .client
            .client
            .get(format!("{}/global", self.client.api_url));

        let response: Response = self.client.request(request).await?;

        let data: Global = response.response.json().await?;

        Ok(data)
    }
}
