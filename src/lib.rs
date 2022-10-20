//! # Coinpaprika API
//!
//! Coinpaprika API Rust library provides access to [Coinpaprika API](https://api.coinpaprika.com/)
//! for applications written in Rust programming language.
//!
//! [Coinpaprika API](https://api.coinpaprika.com/) delivers precise & frequently updated market
//! data from the world of crypto: coin prices, volumes, market caps, ATHs, return rates and more.
//!
//!
//! ## Usage
//!
//! Put this in your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! coinpaprika_api = "0.1"
//! ```
//!
//! Then you can use it like this:
//!
//! ```rust
//! use coinpaprika_api::client::Client;
//! use coinpaprika_api::global::Global;
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let client = Client::new();
//!
//!     let global: Global = client.global().send().await?;
//!     println!("global: {:#?}", global);
//!
//!     Ok(())
//! }
//! ```
//!
//! We include examples for each section of the API, located in `/examples` folder.
//!
//!
//! ## Supported Endpoints
//!
//! - Key
//!     - [x] Get API key info
//!
//! - Global
//!     - [x] Get market overview data
//!
//! - Coins
//!     - [x] List coins
//!     - [x] Get coin by ID
//!     - [x] Get Twitter timeline tweets for a coin
//!     - [x] Get coin events by coin ID
//!     - [x] Get exchanges by coin ID
//!     - [x] Get markets by coin ID
//!     - [x] Get OHLC for the last full day
//!     - [x] Get historical OHLC
//!     - [x] Get today OHLC
//!
//! - People
//!     - [x] Get person by ID
//!
//! - Tags
//!     - [x] List tags
//!     - [x] Get tag by ID
//!
//! - Tickers
//!     - [x] Get tickers for all active coins
//!     - [x] Get ticker for a specific coin
//!     - [x] Get historical ticks for a specific coin
//!
//! - Exchanges
//!     - [x] List exchanges
//!     - [x] Get exchange by ID
//!     - [x] List an exchange markets
//!
//! - Tools
//!     - [x] Search
//!     - [x] Price converter
//!
//! - Contracts
//!     - [x] List contracts platforms
//!     - [x] Get all contract addressess for a given platform
//!     - [ ] Redirect to Ticker by contract address
//!     - [ ] Redirect to historical ticks by contract address
//!
//! - Changelog
//!     - [x] Get id changelog for all coins
//!
//! - Beta
//!     - [ ] List sentiment coins
//!     - [ ] Get sentiment data for a specific coin
//!     - [ ] Get historical sentiment data for a specific coin
//!
//!
//! ## License
//!
//! CoinpaprikaAPI Rust client is available under the MIT license. See the [LICENSE file](./LICENSE) for more info.
//!

/// Client struct used for connecting with coinpaprika.com
pub mod client;

/// Possible errors Client can return
pub mod error;

//
// API Sections
//

/// Requests for "Key" section of the API
pub mod key;

/// Requests for "Global" section of the API
pub mod global;

/// Requests for "Coins" section of the API
pub mod coins;

/// Requests for "People" section of the API
pub mod people;

/// Requests for "Tags" section of the API
pub mod tags;

/// Requests for "Tickers" section of the API
pub mod tickers;

/// Requests for "Exchanges" section of the API
pub mod exchanges;

/// Requests for "Tools" section of the API
pub mod tools;

/// Requests for "Contracts" section of the API
pub mod contracts;

/// Requests for "Changelog" section of the API
pub mod changelog;
