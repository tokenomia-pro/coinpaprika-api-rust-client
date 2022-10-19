use crate::client::{Client, Response};
use crate::error::Error;
use reqwest_middleware::RequestBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
/// Fiat currency
pub struct Fiat {
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Information about given exchange
pub struct Exchange {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub website_status: bool,
    pub api_status: bool,
    pub description: Option<String>,
    pub message: Option<String>,
    pub links: Value,
    pub markets_data_fetched: bool,
    pub adjusted_rank: Option<i32>,
    pub reported_rank: Option<i32>,
    pub currencies: i32,
    pub markets: i32,
    pub fiats: Vec<Fiat>,
    pub quotes: Value,
    pub last_updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Information about given exchange market
pub struct ExchangeMarket {
    pub pair: String,
    pub base_currency_id: String,
    pub base_currency_name: String,
    pub quote_currency_id: String,
    pub quote_currency_name: String,
    pub market_url: String,
    pub category: String,
    pub fee_type: String,
    pub outlier: bool,
    pub reported_volume_24h_share: f64,
    pub quotes: Value,
    pub last_updated: String,
}

/// Request for getting basic information about exchanges on coinpaprika.com
/// [/exchanges](https://api.coinpaprika.com/#tag/Exchanges/operation/getExchanges)
pub struct GetExchangesRequest<'a> {
    client: &'a Client,
    quotes: Vec<String>,
}

impl<'a> GetExchangesRequest<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            quotes: vec![],
        }
    }

    /// List of quotes to return. Up to 3 quotes at once. Currently allowed values:
    /// BTC, ETH, USD, EUR, PLN, KRW, GBP, CAD, JPY, RUB, TRY, NZD, AUD, CHF, UAH, HKD, SGD, NGN,
    /// PHP, MXN, BRL, THB, CLP, CNY, CZK, DKK, HUF, IDR, ILS, INR, MYR, NOK, PKR, SEK, TWD, ZAR,
    /// VND, BOB, COP, PEN, ARS, ISK
    pub fn quotes(&mut self, quotes: Vec<&str>) -> &'a mut GetExchangesRequest {
        self.quotes = quotes.iter().map(|&q| String::from(q)).collect();
        self
    }

    pub async fn send(&self) -> Result<Vec<Exchange>, Error> {
        let query = match self.quotes.len() {
            0 => vec![],
            _ => vec![("quotes", self.quotes.join(","))],
        };

        let request: RequestBuilder = self
            .client
            .client
            .get(format!("{}/exchanges", self.client.api_url))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Vec<Exchange> = response.response.json().await?;

        Ok(data)
    }
}

/// Request for getting basic information about a given exchange on coinpaprika.com
/// [/exchanges/{exchange_id}](https://api.coinpaprika.com/#tag/Exchanges/operation/getExchangeByID)
pub struct GetExchangeRequest<'a> {
    client: &'a Client,
    exchange_id: String,
    quotes: Vec<String>,
}

impl<'a> GetExchangeRequest<'a> {
    pub fn new(client: &'a Client, exchange_id: &str) -> Self {
        Self {
            client,
            exchange_id: String::from(exchange_id),
            quotes: vec![],
        }
    }

    /// List of quotes to return. Up to 3 quotes at once. Currently allowed values:
    /// BTC, ETH, USD, EUR, PLN, KRW, GBP, CAD, JPY, RUB, TRY, NZD, AUD, CHF, UAH, HKD, SGD, NGN,
    /// PHP, MXN, BRL, THB, CLP, CNY, CZK, DKK, HUF, IDR, ILS, INR, MYR, NOK, PKR, SEK, TWD, ZAR,
    /// VND, BOB, COP, PEN, ARS, ISK
    pub fn quotes(&mut self, quotes: Vec<&str>) -> &'a mut GetExchangeRequest {
        self.quotes = quotes.iter().map(|&q| String::from(q)).collect();
        self
    }

    pub async fn send(&self) -> Result<Exchange, Error> {
        let query = match self.quotes.len() {
            0 => vec![],
            _ => vec![("quotes", self.quotes.join(","))],
        };

        let request: RequestBuilder = self
            .client
            .client
            .get(format!(
                "{}/exchanges/{}",
                self.client.api_url, self.exchange_id
            ))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Exchange = response.response.json().await?;

        Ok(data)
    }
}

/// Request for getting a list of all available markets on a given exchange on coinpaprika.com
/// [/exchanges/{exchange_id}/markets](https://api.coinpaprika.com/#tag/Exchanges/paths/~1exchanges~1%7Bexchange_id%7D~1markets/get)
pub struct GetExchangeMarketsRequest<'a> {
    client: &'a Client,
    exchange_id: String,
    quotes: Vec<String>,
}

impl<'a> GetExchangeMarketsRequest<'a> {
    pub fn new(client: &'a Client, exchange_id: &str) -> Self {
        Self {
            client,
            exchange_id: String::from(exchange_id),
            quotes: vec![],
        }
    }

    /// List of quotes to return. Up to 3 quotes at once. Currently allowed values:
    /// BTC, ETH, USD, EUR, PLN, KRW, GBP, CAD, JPY, RUB, TRY, NZD, AUD, CHF, UAH, HKD, SGD, NGN,
    /// PHP, MXN, BRL, THB, CLP, CNY, CZK, DKK, HUF, IDR, ILS, INR, MYR, NOK, PKR, SEK, TWD, ZAR,
    /// VND, BOB, COP, PEN, ARS, ISK
    pub fn quotes(&mut self, quotes: Vec<&str>) -> &'a mut GetExchangeMarketsRequest {
        self.quotes = quotes.iter().map(|&q| String::from(q)).collect();
        self
    }

    pub async fn send(&self) -> Result<Vec<ExchangeMarket>, Error> {
        let query = match self.quotes.len() {
            0 => vec![],
            _ => vec![("quotes", self.quotes.join(","))],
        };

        let request: RequestBuilder = self
            .client
            .client
            .get(format!(
                "{}/exchanges/{}/markets",
                self.client.api_url, self.exchange_id
            ))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Vec<ExchangeMarket> = response.response.json().await?;

        Ok(data)
    }
}
