use chrono::{DateTime, Utc, serde::ts_milliseconds};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub price: Decimal,
    pub qty: Decimal,
    pub quote_qty: Decimal,
    #[serde(with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressedTrade {
    #[serde(rename = "a")]
    pub id: u64,

    #[serde(rename = "p")]
    pub price: Decimal,

    #[serde(rename = "q")]
    pub qty: Decimal,

    #[serde(rename = "f")]
    pub first_trade_id: u64,

    #[serde(rename = "l")]
    pub last_trade_id: u64,

    #[serde(rename = "T", with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(rename = "M")]
    pub is_best_match: bool,
}
