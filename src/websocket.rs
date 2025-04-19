use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

use crate::{
    errors::BinanceError,
    types::{
        events::{AggTradeEvent, KlineEvent, MiniTickerEvent, TickerEvent, TradeEvent},
        market::KlineInterval,
    },
};

#[derive(Debug, Clone)]
pub struct BinanceWebSocket {
    base_url: Url,
}

impl BinanceWebSocket {
    #[must_use]
    pub fn new(testnet: bool) -> Self {
        Self {
            base_url: Url::parse(if testnet {
                "wss://testnet.binance.vision/ws/"
            } else {
                "wss://stream.binance.com/ws/"
            })
            .unwrap(),
        }
    }

    pub async fn stream_agg_trade(
        &self,
        symbol: &str,
        tx: mpsc::Sender<AggTradeEvent>,
    ) -> Result<(), BinanceError> {
        let suffix = format!("{}@aggTrade", symbol.to_lowercase());
        let ws_url = self.base_url.join(&suffix)?;

        loop {
            let (mut ws_stream, _) = match connect_async(&ws_url).await {
                Ok(stream) => stream,
                Err(e) => {
                    log::error!("WebSocket connection failed: {e}");
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    continue;
                }
            };

            while let Some(message) = ws_stream.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        let event: AggTradeEvent = match serde_json::from_str(&text) {
                            Ok(event) => event,
                            Err(e) => {
                                log::error!("Failed to parse agg trade event: {e}");
                                continue;
                            }
                        };
                        if tx.send(event).await.is_err() {
                            log::info!("Receiver dropped. Stopping processing.");
                            return Ok(());
                        }
                    }
                    Ok(Message::Ping(ping)) => {
                        if ws_stream.send(Message::Pong(ping)).await.is_err() {
                            log::error!("Failed to send pong response");
                            break;
                        }
                    }
                    Ok(Message::Close(_)) => {
                        log::info!("WebSocket connection closed by server");
                        break;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("WebSocket message error: {e}");
                        break;
                    }
                }
            }

            log::info!("Reconnecting in 5 seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    pub async fn stream_trades(
        &self,
        symbol: &str,
        tx: mpsc::Sender<TradeEvent>,
    ) -> Result<(), BinanceError> {
        let suffix = format!("{}@trade", symbol.to_lowercase());
        let ws_url = self.base_url.join(&suffix)?;

        loop {
            let (mut ws_stream, _) = match connect_async(&ws_url).await {
                Ok(stream) => stream,
                Err(e) => {
                    log::error!("WebSocket connection failed: {e}");
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    continue;
                }
            };

            while let Some(message) = ws_stream.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        let event: TradeEvent = match serde_json::from_str(&text) {
                            Ok(event) => event,
                            Err(e) => {
                                log::error!("Failed to parse trade event: {e}");
                                continue;
                            }
                        };
                        if tx.send(event).await.is_err() {
                            log::info!("Receiver dropped. Stopping processing.");
                            return Ok(());
                        }
                    }
                    Ok(Message::Ping(ping)) => {
                        if ws_stream.send(Message::Pong(ping)).await.is_err() {
                            log::error!("Failed to send pong response");
                            break;
                        }
                    }
                    Ok(Message::Close(_)) => {
                        log::info!("WebSocket connection closed by server");
                        break;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("WebSocket message error: {e}");
                        break;
                    }
                }
            }

            log::info!("Reconnecting in 5 seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    pub async fn stream_kline(
        &self,
        symbol: &str,
        interval: &KlineInterval,
        tx: mpsc::Sender<KlineEvent>,
    ) -> Result<(), BinanceError> {
        let suffix = format!("{}@kline_{}", symbol.to_lowercase(), interval);
        let ws_url = self.base_url.join(&suffix)?;

        loop {
            let (mut ws_stream, _) = match connect_async(&ws_url).await {
                Ok(stream) => stream,
                Err(e) => {
                    log::error!("WebSocket connection failed: {e}");
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    continue;
                }
            };

            while let Some(message) = ws_stream.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        let event: KlineEvent = match serde_json::from_str(&text) {
                            Ok(event) => event,
                            Err(e) => {
                                log::error!("Failed to parse kline event: {e}");
                                continue;
                            }
                        };
                        if tx.send(event).await.is_err() {
                            log::info!("Receiver dropped. Stopping processing.");
                            return Ok(());
                        }
                    }
                    Ok(Message::Ping(ping)) => {
                        if ws_stream.send(Message::Pong(ping)).await.is_err() {
                            log::error!("Failed to send pong response");
                            break;
                        }
                    }
                    Ok(Message::Close(_)) => {
                        log::info!("WebSocket connection closed by server");
                        break;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("WebSocket message error: {e}");
                        break;
                    }
                }
            }

            log::info!("Reconnecting in 5 seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    pub async fn stream_mini_ticker(
        &self,
        symbol: &str,
        tx: mpsc::Sender<MiniTickerEvent>,
    ) -> Result<(), BinanceError> {
        let suffix = format!("{}@miniTicker", symbol.to_lowercase());
        let ws_url = self.base_url.join(&suffix)?;

        loop {
            let (mut ws_stream, _) = match connect_async(&ws_url).await {
                Ok(stream) => stream,
                Err(e) => {
                    log::error!("WebSocket connection failed: {e}");
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    continue;
                }
            };

            while let Some(message) = ws_stream.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        let event: MiniTickerEvent = match serde_json::from_str(&text) {
                            Ok(event) => event,
                            Err(e) => {
                                log::error!("Failed to parse mini ticker event: {e}");
                                continue;
                            }
                        };
                        if tx.send(event).await.is_err() {
                            log::info!("Receiver dropped. Stopping processing.");
                            return Ok(());
                        }
                    }
                    Ok(Message::Ping(ping)) => {
                        if ws_stream.send(Message::Pong(ping)).await.is_err() {
                            log::error!("Failed to send pong response");
                            break;
                        }
                    }
                    Ok(Message::Close(_)) => {
                        log::info!("WebSocket connection closed by server");
                        break;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("WebSocket message error: {e}");
                        break;
                    }
                }
            }

            log::info!("Reconnecting in 5 seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    pub async fn stream_ticker(
        &self,
        symbol: &str,
        tx: mpsc::Sender<TickerEvent>,
    ) -> Result<(), BinanceError> {
        let suffix = format!("{}@ticker", symbol.to_lowercase());
        let ws_url = self.base_url.join(&suffix)?;

        loop {
            let (mut ws_stream, _) = match connect_async(&ws_url).await {
                Ok(stream) => stream,
                Err(e) => {
                    log::error!("WebSocket connection failed: {e}");
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    continue;
                }
            };

            while let Some(message) = ws_stream.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        let event: TickerEvent = match serde_json::from_str(&text) {
                            Ok(event) => event,
                            Err(e) => {
                                log::error!("Failed to parse ticker event: {e}");
                                continue;
                            }
                        };
                        if tx.send(event).await.is_err() {
                            log::info!("Receiver dropped. Stopping processing.");
                            return Ok(());
                        }
                    }
                    Ok(Message::Ping(ping)) => {
                        if ws_stream.send(Message::Pong(ping)).await.is_err() {
                            log::error!("Failed to send pong response");
                            break;
                        }
                    }
                    Ok(Message::Close(_)) => {
                        log::info!("WebSocket connection closed by server");
                        break;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("WebSocket message error: {e}");
                        break;
                    }
                }
            }

            log::info!("Reconnecting in 5 seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rust_decimal::Decimal;

    #[tokio::test]
    async fn test_stream_agg_trade() {
        let (tx, mut rx) = mpsc::channel(1);
        let ws = BinanceWebSocket::new(true);

        tokio::spawn(async move {
            ws.stream_agg_trade("btcusdt", tx).await.unwrap();
        });

        if let Some(event) = rx.recv().await {
            assert_eq!(event.symbol, "BTCUSDT");
            assert!(event.price > Decimal::ZERO);
            assert!(event.quantity > Decimal::ZERO);
        }
    }

    #[tokio::test]
    async fn test_stream_trades() {
        let (tx, mut rx) = mpsc::channel(1);
        let ws = BinanceWebSocket::new(true);

        tokio::spawn(async move {
            ws.stream_trades("btcusdt", tx).await.unwrap();
        });

        if let Some(event) = rx.recv().await {
            assert_eq!(event.symbol, "BTCUSDT");
            assert!(event.price > Decimal::ZERO);
            assert!(event.quantity > Decimal::ZERO);
        }
    }

    #[tokio::test]
    async fn test_stream_kline() {
        let (tx, mut rx) = mpsc::channel(1);
        let ws = BinanceWebSocket::new(true);

        tokio::spawn(async move {
            ws.stream_kline("btcusdt", &KlineInterval::OneMinute, tx)
                .await
                .unwrap();
        });

        if let Some(event) = rx.recv().await {
            assert_eq!(event.symbol, "BTCUSDT");
            assert!(event.kline.high_price > Decimal::ZERO);
            assert!(event.kline.quote_volume > Decimal::ZERO);
        }
    }

    #[tokio::test]
    async fn test_stream_mini_ticker() {
        let (tx, mut rx) = mpsc::channel(1);
        let ws = BinanceWebSocket::new(true);

        tokio::spawn(async move {
            ws.stream_mini_ticker("btcusdt", tx).await.unwrap();
        });

        if let Some(event) = rx.recv().await {
            assert_eq!(event.symbol, "BTCUSDT");
        }
    }

    #[tokio::test]
    async fn test_stream_ticker() {
        let (tx, mut rx) = mpsc::channel(1);
        let ws = BinanceWebSocket::new(true);

        tokio::spawn(async move {
            ws.stream_ticker("btcusdt", tx).await.unwrap();
        });

        if let Some(event) = rx.recv().await {
            assert_eq!(event.symbol, "BTCUSDT");
        }
    }
}
