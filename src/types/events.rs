use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AggTradeEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: i64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "a")]
    pub aggregate_trade_id: u64,

    #[serde(rename = "p")]
    pub price: Decimal,

    #[serde(rename = "q")]
    pub quantity: Decimal,

    #[serde(rename = "f")]
    pub first_trade_id: u64,

    #[serde(rename = "l")]
    pub last_trade_id: u64,

    #[serde(rename = "T")]
    pub trade_time: u64,

    #[serde(rename = "m")]
    pub is_buyer_market_maker: bool,
}

#[derive(Debug, Deserialize)]
pub struct TradeEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "t")]
    pub trade_id: u64,

    #[serde(rename = "p")]
    pub price: Decimal,

    #[serde(rename = "q")]
    pub quantity: Decimal,

    #[serde(rename = "T")]
    pub trade_time: i64,

    #[serde(rename = "m")]
    pub is_buyer_market_maker: bool,
}

#[derive(Debug, Deserialize)]
pub struct KlineEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "k")]
    pub kline: KlineData,
}

#[derive(Debug, Deserialize)]
pub struct KlineData {
    #[serde(rename = "t")]
    pub start_time: u64,

    #[serde(rename = "T")]
    pub close_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "f")]
    pub first_trade_id: i64,

    #[serde(rename = "L")]
    pub last_trade_id: i64,

    #[serde(rename = "o")]
    pub open_price: Decimal,

    #[serde(rename = "c")]
    pub close_price: Decimal,

    #[serde(rename = "h")]
    pub high_price: Decimal,

    #[serde(rename = "l")]
    pub low_price: Decimal,

    #[serde(rename = "v")]
    pub base_volume: Decimal,

    #[serde(rename = "n")]
    pub number_of_trades: i32,

    #[serde(rename = "x")]
    pub is_closed: bool,

    #[serde(rename = "q")]
    pub quote_volume: Decimal,

    #[serde(rename = "V")]
    pub taker_buy_base_volume: Decimal,

    #[serde(rename = "Q")]
    pub taker_buy_quote_volume: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct MiniTickerEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "c")]
    pub close_price: Decimal,

    #[serde(rename = "o")]
    pub open_price: Decimal,

    #[serde(rename = "h")]
    pub high_price: Decimal,

    #[serde(rename = "l")]
    pub low_price: Decimal,

    #[serde(rename = "v")]
    pub base_volume: Decimal,

    #[serde(rename = "q")]
    pub quote_volume: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct TickerEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "p")]
    pub price_change: Decimal,

    #[serde(rename = "P")]
    pub price_change_percent: Decimal,

    #[serde(rename = "w")]
    pub weighted_avg_price: Decimal,

    #[serde(rename = "x")]
    pub first_trade_price: Decimal,

    #[serde(rename = "c")]
    pub last_price: Decimal,

    #[serde(rename = "Q")]
    pub last_quantity: Decimal,

    #[serde(rename = "b")]
    pub best_bid_price: Decimal,

    #[serde(rename = "B")]
    pub best_bid_quantity: Decimal,

    #[serde(rename = "a")]
    pub best_ask_price: Decimal,

    #[serde(rename = "A")]
    pub best_ask_quantity: Decimal,

    #[serde(rename = "o")]
    pub open_price: Decimal,

    #[serde(rename = "h")]
    pub high_price: Decimal,

    #[serde(rename = "l")]
    pub low_price: Decimal,

    #[serde(rename = "v")]
    pub base_volume: Decimal,

    #[serde(rename = "q")]
    pub quote_volume: Decimal,

    #[serde(rename = "O")]
    pub open_time: u64,

    #[serde(rename = "C")]
    pub close_time: u64,

    #[serde(rename = "F")]
    pub first_trade_id: u64,

    #[serde(rename = "L")]
    pub last_trade_id: u64,

    #[serde(rename = "n")]
    pub total_trades: u64,
}
