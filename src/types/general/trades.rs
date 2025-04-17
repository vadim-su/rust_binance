use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub price: Decimal,
    pub qty: Decimal,
    pub quote_qty: Decimal,
    pub time: u64,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

#[derive(Debug, Deserialize)]
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

    #[serde(rename = "T")]
    pub timestamp: u64,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(rename = "M")]
    pub is_best_match: bool,
}
