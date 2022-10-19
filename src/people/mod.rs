use crate::client::{Client, Response};
use crate::error::Error;
use reqwest_middleware::RequestBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
/// Position of a person
pub struct Position {
    pub coin_id: String,
    pub coin_name: String,
    pub position: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Information about a person
pub struct Person {
    /// ID of person
    pub id: String,

    /// Name of person
    pub name: String,

    /// Description regarding who the person is
    pub description: String,

    /// Number of teams where person is a member
    pub teams_count: i32,

    /// Social media links
    pub links: Value,

    /// Positions the person holds in various projects
    pub positions: Vec<Position>,
}

/// Request for getting information about a person with the specified ID, related to the
/// cryptocurrency market. Using this endpoint you can get a description of the person, social
/// media links, number of teams she or he is involved in and the positions in those teams.
/// [/people/{person_id}](https://api.coinpaprika.com/#tag/People/operation/getPeopleById)
pub struct GetPersonRequest<'a> {
    client: &'a Client,
    person_id: String,
}

impl<'a> GetPersonRequest<'a> {
    pub fn new(client: &'a Client, person_id: &str) -> Self {
        Self {
            client,
            person_id: String::from(person_id),
        }
    }

    pub async fn send(&self) -> Result<Person, Error> {
        let request: RequestBuilder = self
            .client
            .client
            .get(format!("{}/people/{}", self.client.api_url, self.person_id));

        let response: Response = self.client.request(request).await?;

        let data: Person = response.response.json().await?;

        Ok(data)
    }
}
