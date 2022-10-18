use crate::coins::{
    GetCoinEventsRequest, GetCoinExchangesRequest, GetCoinMarketsRequest,
    GetCoinOHLCHistoricalRequest, GetCoinOHLCLastFullDayRequest, GetCoinOHLCTodayRequest,
    GetCoinRequest, GetCoinsRequest, GetTwitterRequest,
};
use crate::error::Error;
use crate::global::GetGlobalRequest;
use crate::key::GetKeyInfoRequest;
use crate::tickers::{GetHistoricalTicksRequest, GetTickerRequest, GetTickersRequest};
use reqwest::{ClientBuilder, RequestBuilder, StatusCode};
use std::time::Duration;

static DEFAULT_USER_AGENT: &str = "coinpaprika-api-rust-client";
static API_URL: &str = "https://api.coinpaprika.com/v1/";
static API_URL_PRO: &str = "https://api-pro.coinpaprika.com/v1/";

#[derive(Debug)]
pub struct Response {
    pub response: reqwest::Response,
    pub request: reqwest::Request,
}

pub struct Client {
    pub client: reqwest::Client,
    pub api_url: &'static str,
    api_key: Option<String>,
    user_agent: &'static str,
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: ClientBuilder::new()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to build client"),
            api_url: API_URL,
            api_key: None,
            user_agent: DEFAULT_USER_AGENT,
        }
    }

    pub fn with_key(key: &str) -> Self {
        Client {
            client: ClientBuilder::new()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to build client"),
            api_url: API_URL_PRO,
            api_key: Some(String::from(key)),
            user_agent: DEFAULT_USER_AGENT,
        }
    }

    pub async fn request(&self, request: RequestBuilder) -> Result<Response, Error> {
        let mut request = request.header("User-Agent", self.user_agent);

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", api_key);
        }

        let request = request.build()?;

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
    // Key
    //
    pub fn key_info(&self) -> GetKeyInfoRequest {
        GetKeyInfoRequest::new(self)
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

    pub fn coin_ohlc_historical(&self, coin_id: &str) -> GetCoinOHLCHistoricalRequest {
        GetCoinOHLCHistoricalRequest::new(self, coin_id)
    }

    pub fn coin_ohlc_today(&self, coin_id: &str) -> GetCoinOHLCTodayRequest {
        GetCoinOHLCTodayRequest::new(self, coin_id)
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
