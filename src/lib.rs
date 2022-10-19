//! # Coinpaprika API
//!
//! Coinpaprika API Rust library provides access to [Coinpaprika API](https://api.coinpaprika.com/)
//! for applications written in Rust programming language.
//!
//! [Coinpaprika API](https://api.coinpaprika.com/) delivers precise & frequently updated market
//! data from the world of crypto: coin prices, volumes, market caps, ATHs, return rates and more.
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

/// Client struct used for connecting with coinpaprika.com
pub mod client;

/// Requests for "Coins" section of the API
pub mod coins;

/// Possible errors Client can return
pub mod error;

/// Requests for "Global" section of the API
pub mod global;

/// Requests for "Key" section of the API
pub mod key;

/// Requests for "Tickers" section of the API
pub mod tickers;
