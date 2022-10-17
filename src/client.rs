use crate::error::Error;
use reqwest::{ClientBuilder, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

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

static DEFAULT_USER_AGENT: &str = "coinpaprika-api-rust-client";
static API_URL: &str = "https://api.coinpaprika.com/v1/";

#[derive(Debug)]
/// Response and request context for API call.
pub struct Response {
    /// HTTP response.
    pub response: reqwest::Response,
    /// HTTP request that resulted in this response.
    pub request: reqwest::Request,
}

pub struct Client {
    client: reqwest::Client,
    user_agent: &'static str,
}

impl Client {
    pub fn new() -> Self {
        // TODO: Handle api keys for accounts higher than "free" tier.

        Client {
            client: ClientBuilder::new()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to build client"),
            user_agent: DEFAULT_USER_AGENT,
        }
    }

    async fn request(&self, request: RequestBuilder) -> Result<Response, Error> {
        let request = request.header("User-Agent", self.user_agent).build()?;

        let response = self
            .client
            .execute(request.try_clone().expect(
                "Error can remain unhandled because we're not using streams, which are the try_clone fail condition",
            ))
            .await;

        match &response {
            Ok(response) => match response.status() {
                StatusCode::BAD_REQUEST => return Err(Error::InvalidRequestError),
                StatusCode::PAYMENT_REQUIRED => return Err(Error::InsufficientPlan),
                StatusCode::FORBIDDEN => return Err(Error::InvalidApiKey),
                StatusCode::NOT_FOUND => return Err(Error::InvalidParameter),
                StatusCode::TOO_MANY_REQUESTS => return Err(Error::RateLimitError),
                StatusCode::INTERNAL_SERVER_ERROR => return Err(Error::InternalServerError),
                _ => {}
            },
            Err(err) => {
                if err.is_connect() || err.is_timeout() {
                    return Err(Error::ApiConnectionError);
                }
            }
        };

        Ok(Response {
            response: response?,
            request,
        })
    }

    pub async fn tickers(&self, quotes: Option<Vec<&str>>) -> Result<Vec<Ticker>, Error> {
        let query = match quotes {
            Some(quote_array) => vec![("quotes", quote_array.join(","))],
            None => vec![],
        };

        let request: reqwest::RequestBuilder = self
            .client
            .get(format!("{}/tickers", API_URL))
            .query(&query);

        let response: Response = self.request(request).await?;

        let data: Vec<Ticker> = response.response.json().await?;

        return Ok(data);
    }
}
