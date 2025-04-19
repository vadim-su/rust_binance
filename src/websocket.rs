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

        let (mut ws_stream, _) = connect_async(ws_url).await?;

        while let Some(message) = ws_stream.next().await {
            let message = message?;

            match message {
                Message::Text(text) => {
                    let event: TradeEvent = serde_json::from_str(&text)?;
                    if let Err(_) = tx.send(event).await {
                        // If the receiver has been dropped, we can stop processing messages
                        break;
                    }
                }
                Message::Ping(ping) => {
                    ws_stream.send(Message::Pong(ping)).await?;
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }

        Ok(())
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
        // If the receiver has been dropped, we can stop processing messages
        drop(rx);
    }
}
