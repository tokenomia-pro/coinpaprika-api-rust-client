use crate::client::{Client, Response};
use crate::error::Error;
use reqwest_middleware::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentMonthUsage {
    pub requests_made: i32,
    pub requests_left: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyUsage {
    pub message: String,
    pub current_month: CurrentMonthUsage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyInfo {
    pub plan: String,
    pub plan_started_at: String,
    pub plan_status: String,
    pub portal_url: String,
    pub usage: KeyUsage,
}

pub struct GetKeyInfoRequest<'a> {
    client: &'a Client,
}

impl<'a> GetKeyInfoRequest<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn send(&self) -> Result<KeyInfo, Error> {
        let request: RequestBuilder = self
            .client
            .client
            .get(format!("{}/key/info", self.client.api_url));

        let response: Response = self.client.request(request).await?;

        let data: KeyInfo = response.response.json().await?;

        Ok(data)
    }
}
