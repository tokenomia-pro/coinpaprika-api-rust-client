use crate::client::{Client, Response};
use crate::error::Error;
use crate::exchanges::Fiat;
use chrono::prelude::*;
use reqwest_middleware::RequestBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
/// Basic information about cryptocurrencies on coinpaprika.com
pub struct Coin {
    /// ID of coin on coinpaprika.com
    pub id: String,

    /// Name of the cryptocurrency
    pub name: String,

    /// Symbol of the cryptocurrency
    pub symbol: String,

    /// Current ranking of the cryptocurrency. If `is_active` is false the `rank` is 0
    pub rank: isize,

    /// Flag indicating if the currency was added within the last 5 days
    pub is_new: bool,

    /// Flag indicating if the currency is active, which means that we can calculate the current
    /// price and volume
    pub is_active: bool,

    #[serde(rename = "type")]
    /// Type of the cryptocurrency. Currently supported values are `coin` and `token`
    pub coin_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Parent coin
pub struct Parent {
    pub id: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Tag assigned to a coin
pub struct CoinTag {
    /// ID of the tag
    pub id: String,

    /// Name of the tag
    pub name: String,

    /// Number of coins with this tag
    pub coin_counter: i32,

    /// Number of ico projects with this tag
    pub ico_counter: i32,
}

#[derive(Debug, Serialize, Deserialize)]
/// The cryptocurrency founding and/or developing team
pub struct Team {
    pub id: String,
    pub name: String,
    pub position: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Coin contract
pub struct Contract {
    /// The contract identifier, which is usually its address
    pub contract: String,

    /// ID of the contract platform. For Ethereum contracts it is `eth-ethereum`, for Tron
    /// `trx-tron`, etc.
    pub platform: String,

    #[serde(rename = "type")]
    /// Type of the contract. Currently supported values are: `ERC20`, `BEP2`, `TRC10`, `TRC20`,
    /// `Stellar Asset`, `Other`
    pub contract_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Coin whitepaper
pub struct Whitepaper {
    /// The whitepaper URL
    pub link: String,

    /// Link to the whitepaper thumbnail
    pub thumbnail: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Detailed, descriptive information about a single coin, without price or volume data.
pub struct CoinDetails {
    /// ID of coin on coinpaprika.com
    pub id: String,

    /// Name of the cryptocurrency
    pub name: String,

    /// Symbol of the cryptocurrency
    pub symbol: String,

    /// This field is deprecated. Use `contracts` field instead
    pub parent: Option<Parent>,

    /// Current coin ranking position on coinpaprika.com
    pub rank: isize,

    /// Flag indicating if the currency was added within the last 5 days
    pub is_new: bool,

    /// Flag indicating if the currency is active, which means that we can calculate the current
    /// price and volume
    pub is_active: bool,

    #[serde(rename = "type")]
    /// Type of the cryptocurrency. Currently supported values are `coin` and `token`
    pub coin_type: String,

    /// Logo image URL
    pub logo: String,

    /// The array of tags to which this coin was assigned on coinpaprika.com
    pub tags: Vec<CoinTag>,

    /// The cryptocurrency founding and/or developing team
    pub team: Vec<Team>,

    /// Text description of the cryptocurrency
    pub description: Option<String>,

    /// An important message about current status of the cryptocurrency
    pub message: String,

    /// Set to true if the cryptocurrency is Open Source project
    pub open_source: bool,

    /// Set to true if the cryptocurrency is supported by any hardware wallet
    pub hardware_wallet: bool,

    /// Launch date of the cryptocurrency
    pub started_at: Option<String>,

    /// Development status of the cryptocurrency - if it is a working project, beta version, just
    /// an idea, etc.
    pub development_status: Option<String>,

    /// Cryptocurrency proof type: Proof of Work, Proof of Stake, etc.
    pub proof_type: Option<String>,

    /// The cryptocurrency organization structure: centralized, decentralized, hierarchical, flat,
    /// etc.
    pub org_structure: Option<String>,

    /// Name of the hash algorithm used by the cryptocurrency
    pub hash_algorithm: Option<String>,

    /// This field is deprecated. Use `contracts` field instead
    pub contract: Option<String>,

    /// This field is deprecated. Use `contracts` field instead
    pub platform: Option<String>,

    /// Coin contracts
    pub contracts: Option<Vec<Contract>>,

    /// Social media links for coin
    pub links: Value,

    /// Contains all links of the `{coin_id}` coin together with statistics for some of them, e.g.
    /// number of twitter followers, reddit subscribers, telegram members or github repository
    /// stars and contributors
    pub links_extended: Value,

    /// Coin whitepaper
    pub whitepaper: Value,

    /// Date of the first available ticker data for the coin. RFC3999 (ISO-8601) format
    pub first_data_at: String,

    /// Date of the last available ticker data for the coin. RFC3999 (ISO-8601) format
    pub last_data_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Tweet about given coin
pub struct Tweet {
    /// Publish date of the tweet. RFC3999 (ISO-8601) format
    pub date: String,

    /// Twitter profile user name
    pub user_name: String,

    /// Twitter profile user image URL
    pub user_image_link: String,

    /// Tweet content
    pub status: String,

    /// Flag indicating whether it is a retweet of someone else's tweet
    pub is_retweet: bool,

    /// Number of retweets of this tweet
    pub retweet_count: i32,

    /// Number of likes of this tweet
    pub like_count: i32,

    /// Tweet URL
    pub status_link: String,

    /// Tweet ID
    pub status_id: String,

    pub media_link: Option<String>,

    /// Link to Youtube video shared in this tweet
    pub youtube_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Event regarding given coin
pub struct CoinEvent {
    pub id: String,
    pub date: String,
    pub date_to: Option<String>,
    pub name: String,
    pub description: String,
    pub is_conference: bool,
    pub link: Option<String>,
    pub proof_image_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Exchange where a given coin is traded.
pub struct CoinExchange {
    pub id: String,
    pub name: String,
    pub fiats: Vec<Fiat>,
    pub adjusted_volume_24h_share: f64,
}

#[derive(Debug, Serialize, Deserialize)]
/// Market for a given coin.
pub struct CoinMarket {
    pub exchange_id: String,
    pub exchange_name: String,
    pub pair: String,
    pub base_currency_id: String,
    pub base_currency_name: String,
    pub quote_currency_id: String,
    pub quote_currency_name: String,
    pub market_url: Option<String>,
    pub category: String,
    pub fee_type: String,
    pub outlier: bool,
    pub adjusted_volume_24h_share: f64,
    pub quotes: Value,
    pub last_updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Open/High/Low/Close values with volume and market capitalization for given coin.
pub struct CoinOHLC {
    /// RFC3999 (ISO-8601) format
    pub time_open: String,

    /// RFC3999 (ISO-8601) format
    pub time_close: String,

    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub volume: Option<i64>,
    pub market_cap: Option<i64>,
}

/// Request for getting basic information about cryptocurrencies on coinpaprika.com:
/// [/coins](https://api.coinpaprika.com/#tag/Coins/paths/~1coins/get)
pub struct GetCoinsRequest<'a> {
    client: &'a Client,
}

impl<'a> GetCoinsRequest<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn send(&self) -> Result<Vec<Coin>, Error> {
        let request: RequestBuilder = self
            .client
            .client
            .get(format!("{}/coins", self.client.api_url));

        let response: Response = self.client.request(request).await?;

        let data: Vec<Coin> = response.response.json().await?;

        Ok(data)
    }
}

/// Request for getting detailed, descriptive information about a single coin, without price or
/// volume data. For price data, check the `/tickers` and `/tickers/{coin_id}` endpoints.
/// [/coins/{coin_id}](https://api.coinpaprika.com/#tag/Coins/operation/getCoinById)
pub struct GetCoinRequest<'a> {
    client: &'a Client,
    coin_id: String,
}

impl<'a> GetCoinRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        Self {
            client,
            coin_id: String::from(coin_id),
        }
    }

    pub async fn send(&self) -> Result<CoinDetails, Error> {
        let request: RequestBuilder = self
            .client
            .client
            .get(format!("{}/coins/{}", self.client.api_url, self.coin_id));

        let response: Response = self.client.request(request).await?;

        let data: CoinDetails = response.response.json().await?;

        Ok(data)
    }
}

/// Request for getting last 50 timeline tweets from the official Twitter profile for a given coin.
/// [/coins/{coin_id}/twitter](https://api.coinpaprika.com/#tag/Coins/paths/~1coins~1%7Bcoin_id%7D~1twitter/get)
pub struct GetTwitterRequest<'a> {
    client: &'a Client,
    coin_id: String,
}

impl<'a> GetTwitterRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        Self {
            client,
            coin_id: String::from(coin_id),
        }
    }

    pub async fn send(&self) -> Result<Vec<Tweet>, Error> {
        let request: RequestBuilder = self.client.client.get(format!(
            "{}/coins/{}/twitter",
            self.client.api_url, self.coin_id
        ));

        let response: Response = self.client.request(request).await?;

        let data: Vec<Tweet> = response.response.json().await?;

        Ok(data)
    }
}

/// Request for getting events for a given coin.
/// [/coins/{coin_id}/events](https://api.coinpaprika.com/#tag/Coins/paths/~1coins~1%7Bcoin_id%7D~1events/get)
pub struct GetCoinEventsRequest<'a> {
    client: &'a Client,
    coin_id: String,
}

impl<'a> GetCoinEventsRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        Self {
            client,
            coin_id: String::from(coin_id),
        }
    }

    pub async fn send(&self) -> Result<Vec<CoinEvent>, Error> {
        let request: RequestBuilder = self.client.client.get(format!(
            "{}/coins/{}/events",
            self.client.api_url, self.coin_id
        ));

        let response: Response = self.client.request(request).await?;

        let data: Vec<CoinEvent> = response.response.json().await?;

        Ok(data)
    }
}

/// Request for getting exchanges where a given coin is traded.
/// [/coins/{coin_id}/exchanges](https://api.coinpaprika.com/#tag/Coins/paths/~1coins~1%7Bcoin_id%7D~1exchanges/get)
pub struct GetCoinExchangesRequest<'a> {
    client: &'a Client,
    coin_id: String,
}

impl<'a> GetCoinExchangesRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        Self {
            client,
            coin_id: String::from(coin_id),
        }
    }

    pub async fn send(&self) -> Result<Vec<CoinExchange>, Error> {
        let request: RequestBuilder = self.client.client.get(format!(
            "{}/coins/{}/exchanges",
            self.client.api_url, self.coin_id
        ));

        let response: Response = self.client.request(request).await?;

        let data: Vec<CoinExchange> = response.response.json().await?;

        Ok(data)
    }
}

/// Request for getting all available markets for a given coin.
/// [/coins/{coin_id}/markets](https://api.coinpaprika.com/#tag/Coins/paths/~1coins~1%7Bcoin_id%7D~1markets/get)
pub struct GetCoinMarketsRequest<'a> {
    client: &'a Client,
    coin_id: String,
    quotes: Vec<String>,
}

impl<'a> GetCoinMarketsRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        Self {
            client,
            coin_id: String::from(coin_id),
            quotes: vec![],
        }
    }

    /// List of quotes to return. Up to 3 quotes at once. Currently allowed values:
    /// BTC, ETH, USD, EUR, PLN, KRW, GBP, CAD, JPY, RUB, TRY, NZD, AUD, CHF, UAH, HKD, SGD, NGN,
    /// PHP, MXN, BRL, THB, CLP, CNY, CZK, DKK, HUF, IDR, ILS, INR, MYR, NOK, PKR, SEK, TWD, ZAR,
    /// VND, BOB, COP, PEN, ARS, ISK
    pub fn quotes(&mut self, quotes: Vec<&str>) -> &'a mut GetCoinMarketsRequest {
        self.quotes = quotes.iter().map(|&q| String::from(q)).collect();
        self
    }

    pub async fn send(&self) -> Result<Vec<CoinMarket>, Error> {
        let query = match self.quotes.len() {
            0 => vec![],
            _ => vec![("quotes", self.quotes.join(","))],
        };

        let request: RequestBuilder = self
            .client
            .client
            .get(format!(
                "{}/coins/{}/markets",
                self.client.api_url, self.coin_id
            ))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Vec<CoinMarket> = response.response.json().await?;

        Ok(data)
    }
}

/// Request for getting Open/High/Low/Close values with volume and market capitalization for the
/// last full day.
/// [/coins/{coin_id}/ohlcv/latest](https://api.coinpaprika.com/#tag/Coins/paths/~1coins~1%7Bcoin_id%7D~1ohlcv~1latest~1/get)
pub struct GetCoinOHLCLastFullDayRequest<'a> {
    client: &'a Client,
    coin_id: String,
    quote: Option<String>,
}

impl<'a> GetCoinOHLCLastFullDayRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        Self {
            client,
            coin_id: String::from(coin_id),
            quote: None,
        }
    }
    /// Returned data quote (available values: `usd` `btc`)
    ///
    /// Default: `"usd"`
    pub fn quote(&mut self, quote: &str) -> &'a mut GetCoinOHLCLastFullDayRequest {
        self.quote = Some(String::from(quote));
        self
    }

    pub async fn send(&self) -> Result<Vec<CoinOHLC>, Error> {
        let mut query: Vec<(&str, &str)> = Vec::new();

        if let Some(quote) = &self.quote {
            query.push(("quote", quote));
        }

        let request: RequestBuilder = self
            .client
            .client
            .get(format!(
                "{}/coins/{}/ohlcv/latest",
                self.client.api_url, self.coin_id
            ))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Vec<CoinOHLC> = response.response.json().await?;

        Ok(data)
    }
}

/// Request for getting Open/High/Low/Close values with volume and market capitalization for any
/// date range. If the end date is the current day, data can change with every request until actual
/// close of the day at 23:59:59"
/// [/coins/{coin_id}/ohlcv/historical](https://api.coinpaprika.com/#tag/Coins/paths/~1coins~1%7Bcoin_id%7D~1ohlcv~1historical/get)
pub struct GetCoinOHLCHistoricalRequest<'a> {
    client: &'a Client,
    coin_id: String,
    start: String,
    end: Option<String>,
    limit: Option<String>,
    quote: Option<String>,
}

impl<'a> GetCoinOHLCHistoricalRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        let now: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`

        Self {
            client,
            coin_id: String::from(coin_id),
            start: format!("{}-{}-{}", now.year(), now.month(), now.day()),
            end: None,
            limit: None,
            quote: None,
        }
    }

    /// Start point for historical data
    ///
    /// Supported formats:
    /// * RFC3999 (ISO-8601) eg. 2018-02-15T05:15:00Z
    /// * Simple date (yyyy-mm-dd) eg. 2018-02-15
    /// * Unix timestamp (in seconds) eg. 1518671700
    pub fn start(&mut self, start: &str) -> &'a mut GetCoinOHLCHistoricalRequest {
        self.start = String::from(start);
        self
    }

    /// End point for historical data
    ///
    /// Default: `"NOW"`
    ///
    /// Supported formats:
    /// RFC3999 (ISO-8601) eg. 2018-02-15T05:15:00Z
    /// Simple date (yyyy-mm-dd) eg. 2018-02-15
    /// Unix timestamp (in seconds) eg. 1518671700
    pub fn end(&mut self, end: &str) -> &'a mut GetCoinOHLCHistoricalRequest {
        self.end = Some(String::from(end));
        self
    }

    /// Limit of result rows (max `366`)
    ///
    /// Default: `1`
    pub fn limit(&mut self, limit: i32) -> &'a mut GetCoinOHLCHistoricalRequest {
        self.limit = Some(limit.to_string());
        self
    }

    /// Returned data quote (available values: `usd` `btc`)
    ///
    /// Default: `"usd"`
    pub fn quote(&mut self, quote: &str) -> &'a mut GetCoinOHLCHistoricalRequest {
        self.quote = Some(String::from(quote));
        self
    }

    pub async fn send(&self) -> Result<Vec<CoinOHLC>, Error> {
        let mut query: Vec<(&str, &str)> = vec![("start", self.start.as_ref())];

        if let Some(end) = &self.end {
            query.push(("end", end));
        }

        if let Some(limit) = &self.limit {
            query.push(("limit", limit));
        }

        if let Some(quote) = &self.quote {
            query.push(("quote", quote));
        }

        let request: RequestBuilder = self
            .client
            .client
            .get(format!(
                "{}/coins/{}/ohlcv/historical",
                self.client.api_url, self.coin_id
            ))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Vec<CoinOHLC> = response.response.json().await?;

        Ok(data)
    }
}

/// Request for getting Open/High/Low/Close values with volume and market capitalization for the
/// current day. Data can change every each request until actual close of the day at 23:59:59.
/// [/coins/{coin_id}/ohlcv/today](https://api.coinpaprika.com/#tag/Coins/paths/~1coins~1%7Bcoin_id%7D~1ohlcv~1today~1/get)
pub struct GetCoinOHLCTodayRequest<'a> {
    client: &'a Client,
    coin_id: String,
    quote: Option<String>,
}

impl<'a> GetCoinOHLCTodayRequest<'a> {
    pub fn new(client: &'a Client, coin_id: &str) -> Self {
        Self {
            client,
            coin_id: String::from(coin_id),
            quote: None,
        }
    }

    /// Returned data quote (available values: `usd` `btc`)
    ///
    /// Default: `"usd"`
    pub fn quote(&mut self, quote: &str) -> &'a mut GetCoinOHLCTodayRequest {
        self.quote = Some(String::from(quote));
        self
    }

    pub async fn send(&self) -> Result<Vec<CoinOHLC>, Error> {
        let mut query: Vec<(&str, &str)> = Vec::new();

        if let Some(quote) = &self.quote {
            query.push(("quote", quote));
        }

        let request: RequestBuilder = self
            .client
            .client
            .get(format!(
                "{}/coins/{}/ohlcv/today",
                self.client.api_url, self.coin_id
            ))
            .query(&query);

        let response: Response = self.client.request(request).await?;

        let data: Vec<CoinOHLC> = response.response.json().await?;

        Ok(data)
    }
}
