use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AveragePrice {
    pub mins: u32,
    pub price: Decimal,
    pub close_time: u64,
}
