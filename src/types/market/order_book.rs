use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
}

// Define a single struct for both bids and asks
#[derive(Debug, Serialize)]
pub struct Order {
    pub price: Decimal,
    pub quantity: Decimal,
}

impl<'de> Deserialize<'de> for Order {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec: [String; 2] = Deserialize::deserialize(deserializer)?;
        return Ok(Self {
            price: vec[0].parse().map_err(serde::de::Error::custom)?,
            quantity: vec[1].parse().map_err(serde::de::Error::custom)?,
        });
    }
}
