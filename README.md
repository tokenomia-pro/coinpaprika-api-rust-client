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

## License

CoinpaprikaAPI is available under the MIT license. See the [LICENSE file](./LICENSE) for more info.
