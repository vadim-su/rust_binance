use reqwest::Client;

use crate::websocket::BinanceWebSocket;

use super::{
    account::BinanceAccountClient, general::BinanceGeneralClient, market::BinanceMarketClient,
    trading::BinanceTradingClient,
};

#[derive(Debug, Clone)]
pub struct BinanceClient {
    pub general: BinanceGeneralClient,
    pub market: BinanceMarketClient,
    pub treading: BinanceTradingClient,
    pub account: BinanceAccountClient,
    pub websocket: BinanceWebSocket,
}

impl BinanceClient {
    #[must_use]
    pub fn new(api_key: String, secret: String, testnet: bool) -> Self {
        let client = Client::new();
        return Self {
            general: BinanceGeneralClient::new(client.clone(), testnet),
            market: BinanceMarketClient::new(client.clone(), testnet),
            treading: BinanceTradingClient::new(
                client.clone(),
                api_key.clone(),
                secret.clone(),
                testnet,
            ),
            account: BinanceAccountClient::new(client, api_key, secret, testnet),
            websocket: BinanceWebSocket::new(testnet),
        };
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_client() {
        let api_key = "test_api_key";
        let secret = "test_secret";

        let _ = BinanceClient::new(api_key.to_string(), secret.to_string(), true);
    }
}
