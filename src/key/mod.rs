use crate::client::{Client, Response};
use crate::error::Error;
use reqwest_middleware::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Requests made and left stats
pub struct CurrentMonthUsage {
    /// Number of requests made in the current month. If a plan has no limit on the number of
    /// requests, e.g., `Enterprise` plan, then the value of `requests_made` is -1
    pub requests_made: i32,

    /// Number of requests left in the current month. If a plan has no limit on the number of
    /// requests, e.g., `Enterprise` plan, then the value of `requests_left` is -1
    pub requests_left: i32,
}

#[derive(Debug, Serialize, Deserialize)]
/// Monthly usage for the API key
pub struct KeyUsage {
    /// `limited plan` if the number of requests is limited in the current plan or `unlimited plan`
    /// if there is no limit on the number of requests
    pub message: String,

    /// Requests made and left stats
    pub current_month: CurrentMonthUsage,
}

#[derive(Debug, Serialize, Deserialize)]
/// API key information
pub struct KeyInfo {
    /// Name of the API plan
    pub plan: String,

    /// A date when the plan started in RFC3999 (ISO-8601) format
    pub plan_started_at: String,

    /// Status of the plan. There are 3 possible statuses: `active` - the subscription is active;
    /// `past_due` - the subscription payment failed. If payment is not made within 7 days, then
    /// the subscription will expire; `inactive` - the subscription is inactive.
    pub plan_status: String,

    /// API Customer Portal URL
    pub portal_url: String,

    /// Monthly usage for the API key
    pub usage: KeyUsage,
}

/// Request for getting API key information
/// [/key/info](https://api.coinpaprika.com/#tag/Key/paths/~1key~1info/get)
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
