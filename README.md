# Coinpaprika API

[![Logo](https://coinpaprika.com/static/files/df51e301.png#greywizard/rock-coin-web/assets/img/cp_logo-transparent.png)](https://api.coinpaprika.com/)

Coinpaprika API Rust library provides access to [Coinpaprika API](https://api.coinpaprika.com/)
for applications written in Rust programming language.

[Coinpaprika API](https://api.coinpaprika.com/) delivers precise & frequently updated market
data from the world of crypto: coin prices, volumes, market caps, ATHs, return rates and more.


## Usage

Put this in your Cargo.toml:

```toml
[dependencies]
coinpaprika_api = "0.1"
```

Then you can use it like this:

```rust
use coinpaprika_api::client::Client;
use coinpaprika_api::global::Global;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let global: Global = client.global().send().await?;
    println!("global: {:#?}", global);

    Ok(())
}
```

We include examples for each section of the API, located in `/examples` folder.

If you have an API key, `Client` struct has an additional constructor
`with_key`, that takes API key as an argument:

```rust
use coinpaprika_api::client::Client;
use coinpaprika_api::global::Global;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::with_key("<your-api-key-here>");

    let global: Global = client.global().send().await?;
    println!("global: {:#?}", global);

    Ok(())
}
```


## Supported Endpoints

- Key
    - [x] Get API key info

- Global
    - [x] Get market overview data

- Coins
    - [x] List coins
    - [x] Get coin by ID
    - [x] Get Twitter timeline tweets for a coin
    - [x] Get coin events by coin ID
    - [x] Get exchanges by coin ID
    - [x] Get markets by coin ID
    - [x] Get OHLC for the last full day
    - [x] Get historical OHLC
    - [x] Get today OHLC

- People
    - [x] Get person by ID

- Tags
    - [x] List tags
    - [x] Get tag by ID

- Tickers
    - [x] Get tickers for all active coins
    - [x] Get ticker for a specific coin
    - [x] Get historical ticks for a specific coin

- Exchanges
    - [x] List exchanges
    - [x] Get exchange by ID
    - [x] List an exchange markets

- Tools
    - [x] Search
    - [x] Price converter

- Contracts
    - [x] List contracts platforms
    - [x] Get all contract addressess for a given platform
    - [ ] Redirect to Ticker by contract address
    - [ ] Redirect to historical ticks by contract address

- Changelog
    - [x] Get id changelog for all coins

- Beta
    - [ ] List sentiment coins
    - [ ] Get sentiment data for a specific coin
    - [ ] Get historical sentiment data for a specific coin


## License

CoinpaprikaAPI Rust client is available under the MIT license. See the [LICENSE file](./LICENSE) for more info.
