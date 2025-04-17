# Rust Binance

A Rust library for interacting with the Binance API. This library provides a convenient way to access various Binance endpoints for trading and market data.

## Features

- Support for Binance API endpoints including market data and trading operations.
- Easy-to-use client for making API requests.
- Support for both mainnet and testnet environments.
- Error handling with custom error types.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust_binance = { git = "https://github.com/vadim-su/rust_binance.git" }
```

## Usage

```rust
use rust_binance::BinanceClient;

#[tokio::main]
async fn main() {
    let api_key = "your_api_key".to_string();
    let secret_key = "your_secret_key".to_string();
    let client = BinanceClient::new(api_key, secret_key, true); // true for testnet

    // Ping the server
    client.ping().await.unwrap();

    // Get server time
    let time = client.get_time().await.unwrap();
    println!("Server Time: {}", time.server_time);
}
```

## Endpoints

- **General Endpoints**: Ping, server time, exchange info.
- **Market Data**: Order book, recent trades, historical trades, compressed trades, klines/candlestick data, and average price.

## Testing

Run tests with:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
