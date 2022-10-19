use crate::client::{Client, Response};
use crate::error::Error;
use reqwest_middleware::RequestBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
/// Price data of a single cryptocurrency on coinpaprika.com
pub struct PriceConversion {
    pub base_currency_id: String,
    pub base_currency_name: String,
    pub base_price_last_updated: String,
    pub quote_currency_id: String,
    pub quote_currency_name: String,
    pub quote_price_last_updated: String,
    pub amount: i32,
    pub price: f64,
}

/// Request for getting currencies, exchanges, icos, people, tags on coinpaprika.com for a given
/// search query
/// [/search](https://api.coinpaprika.com/#tag/Tools/paths/~1search/get)
pub struct GetSearchRequest<'a> {
    client: &'a Client,
    q: String,
    c: Option<Vec<String>>,
    modifier: Option<String>,
    limit: Option<String>,
}

impl<'a> GetSearchRequest<'a> {
    pub fn new(client: &'a Client, q: &str) -> Self {
        Self {
            client,
            q: String::from(q),
            c: None,
            modifier: None,
            limit: None,
        }
    }

    /// One or more categories to search. Available options: `"currencies"`, `"exchanges"`,
    /// `"icos"`, `"people"`, `"tags"`
    ///
    /// Default: `["currencies", "exchanges", "icos", "people", "tags"]` (all categories are
    /// returned)
    pub fn c(&mut self, categories: Vec<&str>) -> &'a mut GetSearchRequest {
        self.c = Some(categories.iter().map(|&q| String::from(q)).collect());
        self
    }

    /// Set modifier for search results. Available options: `symbol_search` - search only by symbol
    /// (works for currencies only)
    pub fn modifier(&mut self, modifier: &str) -> &'a mut GetSearchRequest {
        self.modifier = Some(String::from(modifier));
        self
    }

    /// Limit of results per category (max `250`)
    ///
    /// Default: `6`
    pub fn limit(&mut self, limit: i32) -> &'a mut GetSearchRequest {
        self.limit = Some(limit.to_string());
        self
    }

    pub async fn send(&self) -> Result<Value, Error> {
        let mut query: Vec<(String, String)> = vec![("q".to_string(), self.q.to_string())];

        if let Some(c) = &self.c {
            query.push(("c".to_string(), c.join(",")));
        }

        if let Some(modifier) = &self.modifier {
            query.push(("modifier".to_string(), modifier.to_string()));
        }

        if let Some(limit) = &self.limit {
            query.push(("limit".to_string(), limit.to_string()));
        }

        let request: RequestBuilder = self
            .client
            .client
            .get(format!("{}/search", self.client.api_url))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Value = response.response.json().await?;

        Ok(data)
    }
}

/// Request for converting a set amount of base currency to quote currency
/// [/price-converter](https://api.coinpaprika.com/#tag/Tools/paths/~1price-converter/get)
pub struct GetPriceConversionRequest<'a> {
    client: &'a Client,
    base_currency_id: String,
    quote_currency_id: String,
    amount: String,
}

impl<'a> GetPriceConversionRequest<'a> {
    pub fn new(client: &'a Client, base_currency_id: &str, quote_currency_id: &str) -> Self {
        Self {
            client,
            base_currency_id: String::from(base_currency_id),
            quote_currency_id: String::from(quote_currency_id),
            amount: String::from("0"),
        }
    }

    /// Default: 0
    pub fn amount(&mut self, amount: i32) -> &'a mut GetPriceConversionRequest {
        self.amount = amount.to_string();
        self
    }

    pub async fn send(&self) -> Result<PriceConversion, Error> {
        let query: Vec<(&str, &str)> = vec![
            ("base_currency_id", self.base_currency_id.as_ref()),
            ("quote_currency_id", self.quote_currency_id.as_ref()),
            ("amount", self.amount.as_ref()),
        ];

        let request: RequestBuilder = self
            .client
            .client
            .get(format!("{}/price-converter", self.client.api_url))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: PriceConversion = response.response.json().await?;

        Ok(data)
    }
}
