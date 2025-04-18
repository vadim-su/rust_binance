use crate::types::general::Error;

#[derive(Debug, thiserror::Error)]
pub enum BinanceError {
    #[error("URL construction error: {0}")]
    Url(#[from] url::ParseError),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error: {0} - {1:?}")]
    Api(u16, Error),
}
