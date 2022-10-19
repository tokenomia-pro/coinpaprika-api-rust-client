use crate::client::{Client, Response};
use crate::error::Error;
use reqwest_middleware::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Basic information about cryptocurrency tag
pub struct Tag {
    /// ID of the tag
    pub id: String,

    /// Name of the tag
    pub name: String,

    /// Number of coins with this tag
    pub coin_counter: i32,

    /// Number of ico projects with this tag
    pub ico_counter: i32,

    /// Description of the tag
    pub description: String,

    #[serde(rename = "type")]
    /// Type of tag
    pub tag_type: String,

    /// Coins associated with the tag
    pub coins: Option<Vec<String>>,

    /// ICOs associated with the tag
    pub icos: Option<Vec<String>>,
}

/// Request for getting basic information about cryptocurrencies tags (categories):
/// [/tags](https://api.coinpaprika.com/#tag/Tags/paths/~1tags/get)
pub struct GetTagsRequest<'a> {
    client: &'a Client,
    additional_fields: Vec<String>,
}

impl<'a> GetTagsRequest<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            additional_fields: vec![],
        }
    }

    /// Array of additional fields to include in query result for each tag. Currently supported
    /// values are: `"coins"` and `"icos"`.
    pub fn additional_fields(&mut self, additional_fields: Vec<&str>) -> &'a mut GetTagsRequest {
        self.additional_fields = additional_fields.iter().map(|&q| String::from(q)).collect();
        self
    }

    pub async fn send(&self) -> Result<Vec<Tag>, Error> {
        let query = match self.additional_fields.len() {
            0 => vec![],
            _ => vec![("additional_fields", self.additional_fields.join(","))],
        };

        let request: RequestBuilder = self
            .client
            .client
            .get(format!("{}/tags", self.client.api_url))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Vec<Tag> = response.response.json().await?;

        Ok(data)
    }
}

/// Request for getting basic information about a given cryptocurreny tag:
/// [/tags/{tag_id}](https://api.coinpaprika.com/#tag/Tags/paths/~1tags~1%7Btag_id%7D/get)
pub struct GetTagRequest<'a> {
    client: &'a Client,
    tag_id: String,
    additional_fields: Vec<String>,
}

impl<'a> GetTagRequest<'a> {
    pub fn new(client: &'a Client, tag_id: &str) -> Self {
        Self {
            client,
            tag_id: String::from(tag_id),
            additional_fields: vec![],
        }
    }

    /// Array of additional fields to include in query result for each tag. Currently supported
    /// values are: `"coins"` and `"icos"`.
    pub fn additional_fields(&mut self, additional_fields: Vec<&str>) -> &'a mut GetTagRequest {
        self.additional_fields = additional_fields.iter().map(|&q| String::from(q)).collect();
        self
    }

    pub async fn send(&self) -> Result<Tag, Error> {
        let query = match self.additional_fields.len() {
            0 => vec![],
            _ => vec![("additional_fields", self.additional_fields.join(","))],
        };

        let request: RequestBuilder = self
            .client
            .client
            .get(format!("{}/tags/{}", self.client.api_url, self.tag_id))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Tag = response.response.json().await?;

        Ok(data)
    }
}
