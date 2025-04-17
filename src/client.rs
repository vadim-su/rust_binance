use reqwest::Client;
use url::Url;

use crate::{
    errors::BinanceError,
    types::general::{
        AveragePrice, CompressedTrade, ExchangeInfo, Kline, KlineInterval, OrderBook, Timestamp,
        Trade,
    },
};

const API_VERSION: &str = "v3";

pub struct BinanceClient {
    client: Client,
    api_key: String,
    secret: String,
    base_url: Url,
}

impl BinanceClient {
    pub fn new(api_key: String, secret: String, testnet: bool) -> Self {
        let base_url = if testnet {
            format!("https://testnet.binance.vision/api/{API_VERSION}/")
                .parse()
                .unwrap()
        } else {
            format!("https://api.binance.com/api/{API_VERSION}/")
                .parse()
                .unwrap()
        };

        return Self {
            client: Client::new(),
            api_key,
            secret,
            base_url,
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

    pub async fn get_depth(
        &self,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<OrderBook, BinanceError> {
        let url = self.base_url.join("depth")?;
        let mut query = vec![("symbol", symbol.to_string())];

        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }

        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<OrderBook>()
            .await?;
        return Ok(resp);
    }

    pub async fn get_recent_trades(
        &self,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<Vec<Trade>, BinanceError> {
        let url = self.base_url.join("trades")?;
        let mut query = vec![("symbol", symbol.to_string())];

        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }

        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Trade>>()
            .await?;
        return Ok(resp);
    }

    pub async fn get_historical_trades(
        &self,
        symbol: &str,
        limit: Option<u32>,
        from_id: Option<u32>,
    ) -> Result<Vec<Trade>, BinanceError> {
        let url = self.base_url.join("historicalTrades")?;
        let mut query = vec![("symbol", symbol.to_string())];

        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(id) = from_id {
            query.push(("fromId", id.to_string()));
        }

        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Trade>>()
            .await?;
        return Ok(resp);
    }

    pub async fn get_compressed_trades(
        &self,
        symbol: &str,
        limit: Option<u32>,
        from_id: Option<u32>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<Vec<CompressedTrade>, BinanceError> {
        let url = self.base_url.join("aggTrades")?;
        let mut query = vec![("symbol", symbol.to_string())];

        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(id) = from_id {
            query.push(("fromId", id.to_string()));
        }
        if let Some(start) = start_time {
            query.push(("startTime", start.to_string()));
        }
        if let Some(end) = end_time {
            query.push(("endTime", end.to_string()));
        }

        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<CompressedTrade>>()
            .await?;
        return Ok(resp);
    }

    pub async fn get_klines(
        &self,
        symbol: &str,
        interval: KlineInterval,
        limit: Option<u32>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        timezone: Option<String>,
    ) -> Result<Vec<Kline>, BinanceError> {
        let url = self.base_url.join("klines")?;
        let mut query = vec![
            ("symbol", symbol.to_string()),
            ("interval", interval.to_string()),
        ];

        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(start) = start_time {
            query.push(("startTime", start.to_string()));
        }
        if let Some(end) = end_time {
            query.push(("endTime", end.to_string()));
        }
        if let Some(tz) = timezone {
            query.push(("timezone", tz));
        }

        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Kline>>()
            .await?;

        return Ok(resp);
    }

    pub async fn get_ui_klines(
        &self,
        symbol: &str,
        interval: KlineInterval,
        limit: Option<u32>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        timezone: Option<String>,
    ) -> Result<Vec<Kline>, BinanceError> {
        let url = self.base_url.join("uiKlines")?;
        let mut query = vec![
            ("symbol", symbol.to_string()),
            ("interval", interval.to_string()),
        ];

        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(start) = start_time {
            query.push(("startTime", start.to_string()));
        }
        if let Some(end) = end_time {
            query.push(("endTime", end.to_string()));
        }
        if let Some(tz) = timezone {
            query.push(("timezone", tz));
        }

        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Kline>>()
            .await?;

        return Ok(resp);
    }

    pub async fn get_average_price(&self, symbol: &str) -> Result<AveragePrice, BinanceError> {
        let url = self.base_url.join("avgPrice")?;
        let query = vec![("symbol", symbol.to_string())];

        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<AveragePrice>()
            .await?;

        return Ok(resp);
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

        let client = BinanceClient::new(api_key.to_string(), secret.to_string(), true);
        assert_eq!(client.api_key, api_key);
        assert_eq!(client.secret, secret);
        assert!(client.base_url.to_string().starts_with("https://testnet"));

        let client = BinanceClient::new(api_key.to_string(), secret.to_string(), false);
        assert!(
            client
                .base_url
                .to_string()
                .starts_with("https://api.binance.com")
        );
    }

    #[tokio::test]
    async fn test_ping() {
        let client = BinanceClient::new("test key".to_string(), "test secret".to_string(), true);
        let result = client.ping().await;
        result.unwrap();
    }

    #[tokio::test]
    async fn test_get_time() {
        let client = BinanceClient::new("test key".to_string(), "test secret".to_string(), true);
        let result = client.get_time().await;
        assert!(result.unwrap().server_time > 0);
    }

    #[tokio::test]
    async fn test_get_exchange_info() {
        let client = BinanceClient::new("test key".to_string(), "test secret".to_string(), true);
        let result = client.get_exchange_info(&["BTCUSDT", "ETHUSDT"]).await;
        assert!(!result.unwrap().symbols.is_empty());
    }

    #[tokio::test]
    async fn test_get_depth() {
        let client = BinanceClient::new("test key".to_string(), "test secret".to_string(), true);
        let result = client.get_depth("BTCUSDT", Some(5)).await;
        let depth = result.unwrap();
        assert!(!depth.bids.is_empty());
        assert!(!depth.asks.is_empty());
        assert_eq!(depth.bids.len(), 5);
        assert_eq!(depth.asks.len(), 5);

        let result = client.get_depth("BTCUSDT", None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_recent_trades() {
        let client = BinanceClient::new("test key".to_string(), "test secret".to_string(), true);
        let result = client.get_recent_trades("BTCUSDT", Some(5)).await;
        let trades = result.unwrap();
        assert!(!trades.is_empty());
        assert_eq!(trades.len(), 5);
    }

    #[tokio::test]
    async fn test_get_historical_trades() {
        let client = BinanceClient::new("test key".to_string(), "test secret".to_string(), true);
        let result = client.get_historical_trades("BTCUSDT", Some(5), None).await;
        let trades = result.unwrap();
        assert!(!trades.is_empty());
        assert_eq!(trades.len(), 5);
    }

    #[tokio::test]
    async fn test_get_historical_trades_with_from_id() {
        let client = BinanceClient::new("test key".to_string(), "test secret".to_string(), true);
        let result = client
            .get_historical_trades("BTCUSDT", Some(5), Some(1))
            .await;
        let trades = result.unwrap();
        assert!(!trades.is_empty());
        assert_eq!(trades.len(), 5);
    }

    #[tokio::test]
    async fn test_get_compressed_trades() {
        let client = BinanceClient::new("test key".to_string(), "test secret".to_string(), true);
        let result = client
            .get_compressed_trades("BTCUSDT", Some(5), None, None, None)
            .await;
        let trades = result.unwrap();
        assert!(!trades.is_empty());
        assert_eq!(trades.len(), 5);
    }

    #[tokio::test]
    async fn test_get_klines() {
        let client = BinanceClient::new("test key".to_string(), "test secret".to_string(), true);
        let result = client
            .get_klines("BTCUSDT", KlineInterval::OneDay, Some(5), None, None, None)
            .await;
        let klines = result.unwrap();
        assert!(!klines.is_empty());
        assert_eq!(klines.len(), 5);
    }

    #[tokio::test]
    async fn test_get_ui_klines() {
        let client = BinanceClient::new("test key".to_string(), "test secret".to_string(), true);
        let result = client
            .get_ui_klines("BTCUSDT", KlineInterval::OneDay, Some(5), None, None, None)
            .await;
        let klines = result.unwrap();
        assert!(!klines.is_empty());
        assert_eq!(klines.len(), 5);
    }

    #[tokio::test]

    async fn test_get_average_price() {
        let client = BinanceClient::new("test key".to_string(), "test secret".to_string(), true);
        let result = client.get_average_price("BTCUSDT").await;
        let average_price = result.unwrap();
        assert!(average_price.price > Decimal::from_str_exact("0.0").unwrap());
    }
}
