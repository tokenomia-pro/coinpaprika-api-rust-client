use crate::client::{Client, Response};
use crate::error::Error;
use chrono::prelude::*;
use reqwest_middleware::RequestBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticker {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub rank: isize,
    pub circulating_supply: i64,
    pub total_supply: i64,
    pub max_supply: i64,
    pub beta_value: f64,
    pub first_data_at: String,
    pub last_updated: String,
    pub quotes: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalTick {
    pub timestamp: String,
    pub price: f64,
    pub volume_24h: i64,
    pub market_cap: i64,
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

    pub fn quotes(&mut self, quotes: Vec<&str>) -> &'a mut GetTickersRequest {
        self.quotes = quotes.iter().map(|&q| String::from(q)).collect();
        self
    }

    pub async fn send(&self) -> Result<Vec<Ticker>, Error> {
        let query = match self.quotes.len() {
            0 => vec![],
            _ => vec![("quotes", self.quotes.join(","))],
        };

        let request: RequestBuilder = self
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

    pub fn quotes(&mut self, quotes: Vec<&str>) -> &'a mut GetTickerRequest {
        self.quotes = quotes.iter().map(|&q| String::from(q)).collect();
        self
    }

    pub async fn send(&self) -> Result<Ticker, Error> {
        let query = match self.quotes.len() {
            0 => vec![],
            _ => vec![("quotes", self.quotes.join(","))],
        };

        let request: RequestBuilder = self
            .client
            .client
            .get(format!("{}/tickers/{}", self.client.api_url, self.coin_id))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Ticker = response.response.json().await?;

        Ok(data)
    }
}

pub struct GetHistoricalTicksRequest<'a> {
    client: &'a Client,
    coin_id: String,
    start: String,
    end: Option<String>,
    limit: Option<String>,
    quote: Option<String>,
    interval: Option<String>,
}

impl<'a> GetHistoricalTicksRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        let now: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`

        Self {
            client,
            coin_id: String::from(coin_id),
            start: format!("{}-{}-{}", now.year(), now.month(), now.day()),
            end: None,
            limit: None,
            quote: None,
            interval: None,
        }
    }

    pub fn start(&mut self, start: &str) -> &'a mut GetHistoricalTicksRequest {
        self.start = String::from(start);
        self
    }

    pub fn end(&mut self, end: &str) -> &'a mut GetHistoricalTicksRequest {
        self.end = Some(String::from(end));
        self
    }

    pub fn limit(&mut self, limit: i32) -> &'a mut GetHistoricalTicksRequest {
        self.limit = Some(limit.to_string());
        self
    }

    pub fn quote(&mut self, quote: &str) -> &'a mut GetHistoricalTicksRequest {
        self.quote = Some(String::from(quote));
        self
    }

    pub fn interval(&mut self, interval: &str) -> &'a mut GetHistoricalTicksRequest {
        self.interval = Some(String::from(interval));
        self
    }

    pub async fn send(&self) -> Result<Vec<HistoricalTick>, Error> {
        let mut query: Vec<(&str, &str)> = vec![("start", self.start.as_ref())];

        if let Some(end) = &self.end {
            query.push(("end", end));
        }

        if let Some(limit) = &self.limit {
            query.push(("limit", limit));
        }

        if let Some(quote) = &self.quote {
            query.push(("quote", quote));
        }

        if let Some(interval) = &self.interval {
            query.push(("interval", interval));
        }

        let request: RequestBuilder = self
            .client
            .client
            .get(format!(
                "{}/tickers/{}/historical",
                self.client.api_url, self.coin_id
            ))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Vec<HistoricalTick> = response.response.json().await?;

        Ok(data)
    }
}
