use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

use crate::{errors::BinanceError, types::events::TradeEvent};

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
}

#[cfg(test)]
mod tests {
    use super::*;

    use rust_decimal::Decimal;

    #[tokio::test]
    async fn test_stream_trades() {
        let (tx, mut rx) = mpsc::channel(1);
        let ws = BinanceWebSocket::new(false);

        tokio::spawn(async move {
            ws.stream_trades("btcusdt", tx).await.unwrap();
        });

        if let Some(event) = rx.recv().await {
            assert_eq!(event.symbol, "BTCUSDT");
            assert!(event.price > Decimal::ZERO);
            assert!(event.quantity > Decimal::ZERO);
        }
    }
}
