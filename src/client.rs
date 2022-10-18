use crate::coins::{
    GetCoinEventsRequest, GetCoinExchangesRequest, GetCoinMarketsRequest,
    GetCoinOHLCLastFullDayRequest, GetCoinRequest, GetCoinsRequest, GetTwitterRequest,
};
use crate::error::Error;
use crate::global::GetGlobalRequest;
use crate::tickers::{GetHistoricalTicksRequest, GetTickerRequest, GetTickersRequest};
use reqwest::{ClientBuilder, RequestBuilder, StatusCode};
use std::time::Duration;

static DEFAULT_USER_AGENT: &str = "coinpaprika-api-rust-client";
static API_URL: &str = "https://api.coinpaprika.com/v1/";

#[derive(Debug)]
pub struct Response {
    pub response: reqwest::Response,
    pub request: reqwest::Request,
}

pub struct Client {
    pub client: reqwest::Client,
    pub api_url: &'static str,
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
            api_url: API_URL,
            user_agent: DEFAULT_USER_AGENT,
        }
    }

    pub async fn request(&self, request: RequestBuilder) -> Result<Response, Error> {
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

    //
    // Global
    //
    pub fn global(&self) -> GetGlobalRequest {
        GetGlobalRequest::new(self)
    }

    //
    // Coins
    //
    pub fn coins(&self) -> GetCoinsRequest {
        GetCoinsRequest::new(self)
    }

    pub fn coin(&self, coin_id: &str) -> GetCoinRequest {
        GetCoinRequest::new(self, coin_id)
    }

    pub fn twitter(&self, coin_id: &str) -> GetTwitterRequest {
        GetTwitterRequest::new(self, coin_id)
    }

    pub fn coin_events(&self, coin_id: &str) -> GetCoinEventsRequest {
        GetCoinEventsRequest::new(self, coin_id)
    }

    pub fn coin_exchanges(&self, coin_id: &str) -> GetCoinExchangesRequest {
        GetCoinExchangesRequest::new(self, coin_id)
    }

    pub fn coin_markets(&self, coin_id: &str) -> GetCoinMarketsRequest {
        GetCoinMarketsRequest::new(self, coin_id)
    }

    pub fn coin_ohlc_last_full_day(&self, coin_id: &str) -> GetCoinOHLCLastFullDayRequest {
        GetCoinOHLCLastFullDayRequest::new(self, coin_id)
    }

    //
    // Tickers
    //
    pub fn tickers(&self) -> GetTickersRequest {
        GetTickersRequest::new(self)
    }

    pub fn ticker(&self, coin_id: &str) -> GetTickerRequest {
        GetTickerRequest::new(self, coin_id)
    }

    pub fn historical_ticks(&self, coin_id: &str) -> GetHistoricalTicksRequest {
        GetHistoricalTicksRequest::new(self, coin_id)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
