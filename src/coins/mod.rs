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
    date: String,
    user_name: String,
    user_image_link: String,
    status: String,
    is_retweet: bool,
    retweet_count: i32,
    like_count: i32,
    status_link: String,
    status_id: String,
    media_link: Option<String>,
    youtube_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinEvent {
    id: String,
    date: String,
    date_to: Option<String>,
    name: String,
    description: String,
    is_conference: bool,
    link: String,
    proof_image_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fiat {
    name: String,
    symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinExchange {
    id: String,
    name: String,
    fiats: Vec<Fiat>,
    adjusted_volume_24h_share: f64,
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
