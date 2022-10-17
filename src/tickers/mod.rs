use crate::client::{Client, Response};
use crate::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticker {
    id: String,
    name: String,
    symbol: String,
    rank: isize,
    circulating_supply: i64,
    total_supply: i64,
    max_supply: i64,
    beta_value: f64,
    first_data_at: String,
    last_updated: String,
    quotes: Value,
}

pub struct GetTickersRequest<'a> {
    client: &'a Client,
    quotes: Vec<String>,
}

impl<'a> GetTickersRequest<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            quotes: vec![],
        }
    }

    pub fn with_quotes(&mut self, quotes: Vec<&str>) -> &'a mut GetTickersRequest {
        self.quotes = quotes.iter().map(|&q| String::from(q)).collect();
        self
    }

    pub async fn send(&self) -> Result<Vec<Ticker>, Error> {
        let query = match self.quotes.len() {
            0 => vec![],
            _ => vec![("quotes", self.quotes.join(","))],
        };

        let request: reqwest::RequestBuilder = self
            .client
            .client
            .get(format!("{}/tickers", self.client.api_url))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Vec<Ticker> = response.response.json().await?;

        Ok(data)
    }
}

pub struct GetTickerRequest<'a> {
    client: &'a Client,
    coin_id: String,
    quotes: Vec<String>,
}

impl<'a> GetTickerRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        Self {
            client,
            coin_id: String::from(coin_id),
            quotes: vec![],
        }
    }

    pub fn with_quotes(&mut self, quotes: Vec<&str>) -> &'a mut GetTickerRequest {
        self.quotes = quotes.iter().map(|&q| String::from(q)).collect();
        self
    }

    pub async fn send(&self) -> Result<Ticker, Error> {
        let query = match self.quotes.len() {
            0 => vec![],
            _ => vec![("quotes", self.quotes.join(","))],
        };

        let request: reqwest::RequestBuilder = self
            .client
            .client
            .get(format!("{}/tickers/{}", self.client.api_url, self.coin_id))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Ticker = response.response.json().await?;

        Ok(data)
    }
}
