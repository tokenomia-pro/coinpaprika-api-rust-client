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
