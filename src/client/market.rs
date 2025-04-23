use reqwest::Client;
use url::Url;

use crate::errors::BinanceError;
use crate::types::market::{
    AveragePrice, CompressedTrade, Kline, KlineInterval, OrderBook, Ticker, Ticker24, Ticker24Mini,
    TickerBook, TickerMini, TickerPrice, Trade, WindowSize,
};

use super::get_base_url;

#[derive(Debug, Clone)]
pub struct BinanceMarketClient {
    client: Client,
    base_url: Url,
}

impl BinanceMarketClient {
    pub fn new(client: Client, testnet: bool) -> Self {
        return Self {
            client,
            base_url: get_base_url(testnet),
        };
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

    pub async fn get_ticker_24hr(&self, symbols: &[&str]) -> Result<Vec<Ticker24>, BinanceError> {
        let url = self.base_url.join("ticker/24hr")?;

        let symbols_query = format!("[\"{}\"]", symbols.join("\",\""));
        let query = vec![("symbols", symbols_query)];

        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Ticker24>>()
            .await?;

        return Ok(resp);
    }

    pub async fn get_ticker_24hr_mini(
        &self,
        symbols: &[&str],
    ) -> Result<Vec<Ticker24Mini>, BinanceError> {
        let url = self.base_url.join("ticker/24hr")?;

        let symbols_query = format!("[\"{}\"]", symbols.join("\",\""));
        let query = vec![("symbols", symbols_query), ("type", "MINI".to_string())];

        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Ticker24Mini>>()
            .await?;

        return Ok(resp);
    }

    pub async fn get_ticker_trading_day(
        &self,
        symbols: &[&str],
    ) -> Result<Vec<Ticker>, BinanceError> {
        let url = self.base_url.join("ticker/tradingDay")?;

        let symbols_query = format!("[\"{}\"]", symbols.join("\",\""));
        let query = vec![("symbols", symbols_query)];

        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Ticker>>()
            .await?;

        return Ok(resp);
    }

    pub async fn get_ticker_trading_day_mini(
        &self,
        symbols: &[&str],
    ) -> Result<Vec<TickerMini>, BinanceError> {
        let url = self.base_url.join("ticker/tradingDay")?;

        let symbols_query = format!("[\"{}\"]", symbols.join("\",\""));
        let query = vec![("symbols", symbols_query), ("type", "MINI".to_string())];

        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<TickerMini>>()
            .await?;

        return Ok(resp);
    }

    pub async fn get_ticker_price(
        &self,
        symbols: &[&str],
    ) -> Result<Vec<TickerPrice>, BinanceError> {
        let url = self.base_url.join("ticker/price")?;

        let symbols_query = format!("[\"{}\"]", symbols.join("\",\""));
        let query = vec![("symbols", symbols_query)];

        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<TickerPrice>>()
            .await?;

        return Ok(resp);
    }

    pub async fn get_ticker_book(&self, symbols: &[&str]) -> Result<Vec<TickerBook>, BinanceError> {
        let url = self.base_url.join("ticker/bookTicker")?;

        let symbols_query = format!("[\"{}\"]", symbols.join("\",\""));
        let query = vec![("symbols", symbols_query)];

        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<TickerBook>>()
            .await?;

        return Ok(resp);
    }

    pub async fn get_rolling_window_price_change(
        &self,
        symbols: &[&str],
        window_size: WindowSize,
    ) -> Result<Vec<Ticker>, BinanceError> {
        let url = self.base_url.join("ticker")?;

        let symbols_query = format!("[\"{}\"]", symbols.join("\",\""));
        let query = vec![
            ("symbols", symbols_query),
            ("windowSize", window_size.to_string()),
        ];

        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Ticker>>()
            .await?;

        return Ok(resp);
    }

    pub async fn get_rolling_window_price_change_mini(
        &self,
        symbols: &[&str],
        window_size: WindowSize,
    ) -> Result<Vec<TickerMini>, BinanceError> {
        let url = self.base_url.join("ticker")?;

        let symbols_query = format!("[\"{}\"]", symbols.join("\",\""));
        let query = vec![
            ("symbols", symbols_query),
            ("windowSize", window_size.to_string()),
            ("type", "MINI".to_string()),
        ];

        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<TickerMini>>()
            .await?;

        return Ok(resp);
    }

    /// Получить исторические klines за указанный диапазон времени.
    /// Если данных больше лимита (1000), делает несколько запросов.
    pub async fn get_historical_klines(
        &self,
        symbol: &str,
        interval: KlineInterval,
        start_time: u64,
        end_time: u64,
        timezone: Option<String>,
    ) -> Result<Vec<Kline>, BinanceError> {
        let mut all_klines = Vec::new();
        let mut current_start = start_time;
        let max_limit = 1000;

        loop {
            let klines = self
                .get_klines(
                    symbol,
                    interval.clone(),
                    Some(max_limit),
                    Some(current_start),
                    Some(end_time),
                    timezone.clone(),
                )
                .await?;

            if klines.is_empty() {
                break;
            }

            // The last kline
            let last_kline = klines.last().unwrap();
            let last_open_time = last_kline.open_time;

            // Add only klines that do not exceed end_time
            let mut filtered_klines = klines
                .into_iter()
                .take_while(|k| k.open_time < end_time)
                .collect::<Vec<_>>();

            let fetched = filtered_klines.len();
            all_klines.append(&mut filtered_klines);

            // If less than max_limit or reached the end of the range, exit
            if fetched < max_limit as usize || last_open_time >= end_time {
                break;
            }

            // Next start is the open time of the last kline + 1ms
            current_start = last_open_time + 1;
        }

        Ok(all_klines)
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;

    #[test]
    fn test_new_client() {
        let client = BinanceMarketClient::new(Client::new(), true);
        assert_eq!(
            client.base_url.as_str(),
            "https://testnet.binance.vision/api/v3/"
        );

        let client = BinanceMarketClient::new(Client::new(), false);
        assert_eq!(client.base_url.as_str(), "https://api.binance.com/api/v3/");
    }

    #[tokio::test]
    async fn test_get_depth() {
        let client = BinanceMarketClient::new(Client::new(), true);
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
        let client = BinanceMarketClient::new(Client::new(), true);
        let result = client.get_recent_trades("BTCUSDT", Some(5)).await;
        let trades = result.unwrap();
        assert!(!trades.is_empty());
        assert_eq!(trades.len(), 5);
    }

    #[tokio::test]
    async fn test_get_historical_trades() {
        let client = BinanceMarketClient::new(Client::new(), true);
        let result = client.get_historical_trades("BTCUSDT", Some(5), None).await;
        let trades = result.unwrap();
        assert!(!trades.is_empty());
        assert_eq!(trades.len(), 5);
    }

    #[tokio::test]
    async fn test_get_historical_trades_with_from_id() {
        let client = BinanceMarketClient::new(Client::new(), true);
        let result = client
            .get_historical_trades("BTCUSDT", Some(5), Some(1))
            .await;
        let trades = result.unwrap();
        assert!(!trades.is_empty());
        assert_eq!(trades.len(), 5);
    }

    #[tokio::test]
    async fn test_get_compressed_trades() {
        let client = BinanceMarketClient::new(Client::new(), true);
        let result = client
            .get_compressed_trades("BTCUSDT", Some(5), None, None, None)
            .await;
        let trades = result.unwrap();
        assert!(!trades.is_empty());
        assert_eq!(trades.len(), 5);
    }

    #[tokio::test]
    async fn test_get_klines() {
        let client = BinanceMarketClient::new(Client::new(), true);
        let result = client
            .get_klines("BTCUSDT", KlineInterval::OneDay, Some(5), None, None, None)
            .await;
        let klines = result.unwrap();
        assert!(!klines.is_empty());
        assert_eq!(klines.len(), 5);
    }

    #[tokio::test]
    async fn test_get_ui_klines() {
        let client = BinanceMarketClient::new(Client::new(), true);
        let result = client
            .get_ui_klines("BTCUSDT", KlineInterval::OneDay, Some(5), None, None, None)
            .await;
        let klines = result.unwrap();
        assert!(!klines.is_empty());
        assert_eq!(klines.len(), 5);
    }

    #[tokio::test]
    async fn test_get_average_price() {
        let client = BinanceMarketClient::new(Client::new(), true);
        let result = client.get_average_price("BTCUSDT").await;
        let average_price = result.unwrap();
        assert!(average_price.price > Decimal::from_str_exact("0.0").unwrap());
    }

    #[tokio::test]
    async fn test_get_ticker_24hr() {
        let client = BinanceMarketClient::new(Client::new(), true);
        let result = client.get_ticker_24hr(&["BTCUSDT"]).await;
        let ticker = result.unwrap();
        assert!(ticker[0].open_price > Decimal::from_str_exact("0.0").unwrap());
    }

    #[tokio::test]
    async fn test_get_ticker_24hr_mini() {
        let client = BinanceMarketClient::new(Client::new(), true);
        let result = client.get_ticker_24hr_mini(&["BTCUSDT"]).await;
        let ticker = result.unwrap();
        assert!(ticker[0].open_price > Decimal::from_str_exact("0.0").unwrap());
    }

    #[tokio::test]
    async fn test_get_ticker_trading_day() {
        let client = BinanceMarketClient::new(Client::new(), true);
        let result = client.get_ticker_trading_day(&["BTCUSDT"]).await;
        let ticker = result.unwrap();
        assert!(ticker[0].open_price > Decimal::from_str_exact("0.0").unwrap());
    }

    #[tokio::test]
    async fn test_get_ticker_trading_day_mini() {
        let client = BinanceMarketClient::new(Client::new(), true);
        let result = client.get_ticker_trading_day_mini(&["BTCUSDT"]).await;
        let ticker = result.unwrap();
        assert!(ticker[0].open_price > Decimal::from_str_exact("0.0").unwrap());
    }

    #[tokio::test]
    async fn test_get_ticker_price() {
        let client = BinanceMarketClient::new(Client::new(), true);

        let result = client.get_ticker_price(&["BTCUSDT", "ETHUSDT"]).await;
        let ticker = result.unwrap();
        assert!(ticker[0].price > Decimal::from_str_exact("0.0").unwrap());
        assert!(ticker[1].price > Decimal::from_str_exact("0.0").unwrap());
        assert_eq!(ticker.len(), 2);
    }

    #[tokio::test]
    async fn test_get_ticker_book() {
        let client = BinanceMarketClient::new(Client::new(), true);
        let result = client.get_ticker_book(&["BTCUSDT", "ETHUSDT"]).await;
        let ticker = result.unwrap();
        assert!(ticker[0].ask_price > Decimal::from_str_exact("0.0").unwrap());
        assert!(ticker[1].ask_price > Decimal::from_str_exact("0.0").unwrap());
        assert_eq!(ticker.len(), 2);
    }

    #[tokio::test]
    async fn test_get_rolling_window_price_change() {
        let client = BinanceMarketClient::new(Client::new(), true);
        let result = client
            .get_rolling_window_price_change(&["BTCUSDT", "ETHUSDT"], WindowSize::Days(1))
            .await;
        let ticker = result.unwrap();
        assert!(ticker[0].open_price > Decimal::from_str_exact("0.0").unwrap());
        assert!(ticker[1].open_price > Decimal::from_str_exact("0.0").unwrap());
        assert_eq!(ticker.len(), 2);
    }

    #[tokio::test]
    async fn test_get_rolling_window_price_change_mini() {
        let client = BinanceMarketClient::new(Client::new(), true);

        let result = client
            .get_rolling_window_price_change_mini(&["BTCUSDT", "ETHUSDT"], WindowSize::Days(1))
            .await;

        let ticker = result.unwrap();
        assert!(ticker[0].open_price > Decimal::from_str_exact("0.0").unwrap());
        assert!(ticker[1].open_price > Decimal::from_str_exact("0.0").unwrap());
        assert_eq!(ticker.len(), 2);
    }
}
