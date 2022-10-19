use crate::client::{Client, Response};
use crate::error::Error;
use reqwest_middleware::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Contract information
pub struct Contract {
    address: String,
    id: String,

    #[serde(rename = "type")]
    pub contract_type: String,
}

/// Request for getting all available contract platforms on coinpaprika.com
/// [/contracts](https://api.coinpaprika.com/#tag/Contracts/operation/getPlatforms)
pub struct GetContractPlatformsRequest<'a> {
    client: &'a Client,
}

impl<'a> GetContractPlatformsRequest<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn send(&self) -> Result<Vec<String>, Error> {
        let request: RequestBuilder = self
            .client
            .client
            .get(format!("{}/contracts", self.client.api_url));

        let response: Response = self.client.request(request).await?;

        let data: Vec<String> = response.response.json().await?;

        Ok(data)
    }
}

/// Request for getting all available contracts for a given platform on coinpaprika.com
/// [/contracts/{platform_id}](https://api.coinpaprika.com/#tag/Contracts/operation/getContracts)
pub struct GetContractsRequest<'a> {
    client: &'a Client,
    platform_id: String,
}

impl<'a> GetContractsRequest<'a> {
    pub fn new(client: &'a Client, platform_id: &str) -> Self {
        Self {
            client,
            platform_id: String::from(platform_id),
        }
    }

    pub async fn send(&self) -> Result<Vec<Contract>, Error> {
        let request: RequestBuilder = self.client.client.get(format!(
            "{}/contracts/{}",
            self.client.api_url, self.platform_id
        ));

        let response: Response = self.client.request(request).await?;

        let data: Vec<Contract> = response.response.json().await?;

        Ok(data)
    }
}
