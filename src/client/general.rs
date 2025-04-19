use reqwest::Client;
use url::Url;

use crate::errors::BinanceError;
use crate::types::general::{ExchangeInfo, Timestamp};

use super::get_base_url;

pub struct BinanceGeneralClient {
    client: Client,
    base_url: Url,
}

impl BinanceGeneralClient {
    pub fn new(client: Client, testnet: bool) -> Self {
        return Self {
            client,
            base_url: get_base_url(testnet),
        };
    }

    pub async fn ping(&self) -> Result<(), BinanceError> {
        let url = self.base_url.join("ping")?;
        self.client.get(url).send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn get_time(&self) -> Result<Timestamp, BinanceError> {
        let url = self.base_url.join("time")?;
        let resp = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<Timestamp>()
            .await?;
        return Ok(resp);
    }

    pub async fn get_exchange_info(&self, symbols: &[&str]) -> Result<ExchangeInfo, BinanceError> {
        let url = self.base_url.join("exchangeInfo")?;

        // ["BTCUSDT", "ETHUSDT"]
        let symbols_query = format!("[\"{}\"]", symbols.join("\",\""));

        let resp = self
            .client
            .get(url)
            .query(&[("symbols", symbols_query)])
            .send()
            .await?
            .error_for_status()?
            .json::<ExchangeInfo>()
            .await?;
        return Ok(resp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_client() {
        let client = BinanceGeneralClient::new(Client::new(), true);
        assert_eq!(
            client.base_url.as_str(),
            "https://testnet.binance.vision/api/v3/"
        );

        let client = BinanceGeneralClient::new(Client::new(), false);
        assert_eq!(client.base_url.as_str(), "https://api.binance.com/api/v3/");
    }

    #[tokio::test]
    async fn test_ping() {
        let client = BinanceGeneralClient::new(Client::new(), true);
        let result = client.ping().await;
        result.unwrap();
    }

    #[tokio::test]
    async fn test_get_time() {
        let client = BinanceGeneralClient::new(Client::new(), true);
        let result = client.get_time().await;
        assert!(result.unwrap().server_time > 0);
    }

    #[tokio::test]
    async fn test_get_exchange_info() {
        let client = BinanceGeneralClient::new(Client::new(), true);
        let result = client.get_exchange_info(&["BTCUSDT", "ETHUSDT"]).await;
        assert!(!result.unwrap().symbols.is_empty());
    }
}
