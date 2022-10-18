use crate::client::{Client, Response};
use crate::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Parent {
    pub id: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub coin_counter: i32,
    pub ico_counter: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub position: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    pub contract: String,
    pub platform: String,

    #[serde(rename = "type")]
    pub contract_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Whitepaper {
    pub link: String,
    pub thumbnail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinDetails {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub parent: Option<Parent>,
    pub rank: isize,
    pub is_new: bool,
    pub is_active: bool,

    #[serde(rename = "type")]
    pub coin_type: String,

    pub logo: String,
    pub tags: Vec<Tag>,
    pub team: Vec<Team>,
    pub description: String,
    pub message: String,
    pub open_source: bool,
    pub hardware_wallet: bool,
    pub started_at: String,
    pub development_status: String,
    pub proof_type: String,
    pub org_structure: String,
    pub hash_algorithm: String,
    pub contract: Option<String>,
    pub platform: Option<String>,
    pub contracts: Option<Vec<Contract>>,
    pub links: Value,
    pub links_extended: Value,
    pub whitepaper: Value,
    pub first_data_at: String,
    pub last_data_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub date: String,
    pub user_name: String,
    pub user_image_link: String,
    pub status: String,
    pub is_retweet: bool,
    pub retweet_count: i32,
    pub like_count: i32,
    pub status_link: String,
    pub status_id: String,
    pub media_link: Option<String>,
    pub youtube_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinEvent {
    pub id: String,
    pub date: String,
    pub date_to: Option<String>,
    pub name: String,
    pub description: String,
    pub is_conference: bool,
    pub link: String,
    pub proof_image_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fiat {
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinExchange {
    pub id: String,
    pub name: String,
    pub fiats: Vec<Fiat>,
    pub adjusted_volume_24h_share: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinMarket {
    pub exchange_id: String,
    pub exchange_name: String,
    pub pair: String,
    pub base_currency_id: String,
    pub base_currency_name: String,
    pub quote_currency_id: String,
    pub quote_currency_name: String,
    pub market_url: Option<String>,
    pub category: String,
    pub fee_type: String,
    pub outlier: bool,
    pub adjusted_volume_24h_share: f64,
    pub quotes: Value,
    pub last_updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinOHLC {
    pub time_open: String,
    pub time_close: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub market_cap: i64,
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

pub struct GetCoinRequest<'a> {
    client: &'a Client,
    coin_id: String,
}

impl<'a> GetCoinRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        Self {
            client,
            coin_id: String::from(coin_id),
        }
    }

    pub async fn send(&self) -> Result<CoinDetails, Error> {
        let request: reqwest::RequestBuilder = self
            .client
            .client
            .get(format!("{}/coins/{}", self.client.api_url, self.coin_id));

        let response: Response = self.client.request(request).await?;

        let data: CoinDetails = response.response.json().await?;

        Ok(data)
    }
}

pub struct GetTwitterRequest<'a> {
    client: &'a Client,
    coin_id: String,
}

impl<'a> GetTwitterRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        Self {
            client,
            coin_id: String::from(coin_id),
        }
    }

    pub async fn send(&self) -> Result<Vec<Tweet>, Error> {
        let request: reqwest::RequestBuilder = self.client.client.get(format!(
            "{}/coins/{}/twitter",
            self.client.api_url, self.coin_id
        ));

        let response: Response = self.client.request(request).await?;

        let data: Vec<Tweet> = response.response.json().await?;

        Ok(data)
    }
}

pub struct GetCoinEventsRequest<'a> {
    client: &'a Client,
    coin_id: String,
}

impl<'a> GetCoinEventsRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        Self {
            client,
            coin_id: String::from(coin_id),
        }
    }

    pub async fn send(&self) -> Result<Vec<CoinEvent>, Error> {
        let request: reqwest::RequestBuilder = self.client.client.get(format!(
            "{}/coins/{}/events",
            self.client.api_url, self.coin_id
        ));

        let response: Response = self.client.request(request).await?;

        let data: Vec<CoinEvent> = response.response.json().await?;

        Ok(data)
    }
}

pub struct GetCoinExchangesRequest<'a> {
    client: &'a Client,
    coin_id: String,
}

impl<'a> GetCoinExchangesRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        Self {
            client,
            coin_id: String::from(coin_id),
        }
    }

    pub async fn send(&self) -> Result<Vec<CoinExchange>, Error> {
        let request: reqwest::RequestBuilder = self.client.client.get(format!(
            "{}/coins/{}/exchanges",
            self.client.api_url, self.coin_id
        ));

        let response: Response = self.client.request(request).await?;

        let data: Vec<CoinExchange> = response.response.json().await?;

        Ok(data)
    }
}

pub struct GetCoinMarketsRequest<'a> {
    client: &'a Client,
    coin_id: String,
    quotes: Vec<String>,
}

impl<'a> GetCoinMarketsRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        Self {
            client,
            coin_id: String::from(coin_id),
            quotes: vec![],
        }
    }

    pub fn quotes(&mut self, quotes: Vec<&str>) -> &'a mut GetCoinMarketsRequest {
        self.quotes = quotes.iter().map(|&q| String::from(q)).collect();
        self
    }

    pub async fn send(&self) -> Result<Vec<CoinMarket>, Error> {
        let query = match self.quotes.len() {
            0 => vec![],
            _ => vec![("quotes", self.quotes.join(","))],
        };

        let request: reqwest::RequestBuilder = self
            .client
            .client
            .get(format!(
                "{}/coins/{}/markets",
                self.client.api_url, self.coin_id
            ))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Vec<CoinMarket> = response.response.json().await?;

        Ok(data)
    }
}

pub struct GetCoinOHLCLastFullDayRequest<'a> {
    client: &'a Client,
    coin_id: String,
    quote: Option<String>,
}

impl<'a> GetCoinOHLCLastFullDayRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        Self {
            client,
            coin_id: String::from(coin_id),
            quote: None,
        }
    }

    pub fn quote(&mut self, quote: &str) -> &'a mut GetCoinOHLCLastFullDayRequest {
        self.quote = Some(String::from(quote));
        self
    }

    pub async fn send(&self) -> Result<Vec<CoinOHLC>, Error> {
        let mut query: Vec<(&str, &str)> = Vec::new();

        if let Some(quote) = &self.quote {
            query.push(("quote", quote));
        }

        let request: reqwest::RequestBuilder = self
            .client
            .client
            .get(format!(
                "{}/coins/{}/ohlcv/latest",
                self.client.api_url, self.coin_id
            ))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Vec<CoinOHLC> = response.response.json().await?;

        Ok(data)
    }
}
