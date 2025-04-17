use reqwest::Client;

use super::{general::BinanceGeneralClient, market::BinanceMarketClient};

pub struct BinanceClient {
    pub general: BinanceGeneralClient,
    pub market: BinanceMarketClient,
}

impl BinanceClient {
    pub fn new(api_key: String, secret: String, testnet: bool) -> Self {
        let client = Client::new();
        return Self {
            general: BinanceGeneralClient::new(
                client.clone(),
                api_key.clone(),
                secret.clone(),
                testnet,
            ),
            market: BinanceMarketClient::new(
                client.clone(),
                api_key.clone(),
                secret.clone(),
                testnet,
            ),
        };
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;

    #[test]
    fn test_new_client() {
        let api_key = "test_api_key";
        let secret = "test_secret";

        BinanceClient::new(api_key.to_string(), secret.to_string(), true);
    }
}
