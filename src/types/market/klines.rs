use core::fmt;

use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Kline {
    pub open_time: u64,
    pub open_price: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
    pub close_price: Decimal,
    pub volume: Decimal,
    pub close_time: u64,
    pub quote_asset_volume: Decimal,
    pub number_of_trades: u64,
    pub taker_buy_base_asset_volume: Decimal,
    pub taker_buy_quote_asset_volume: Decimal,
}

// Custom deserialization for Kline to handle array format
impl<'de> Deserialize<'de> for Kline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec: [Value; 12] = Deserialize::deserialize(deserializer)?;
        return Ok(Self {
            open_time: vec[0]
                .as_u64()
                .ok_or_else(|| serde::de::Error::custom("Invalid open_time"))?,

            open_price: vec[1]
                .as_str()
                .ok_or_else(|| serde::de::Error::custom("Invalid open_price"))?
                .parse()
                .map_err(serde::de::Error::custom)?,

            high_price: vec[2]
                .as_str()
                .ok_or_else(|| serde::de::Error::custom("Invalid high_price"))?
                .parse()
                .map_err(serde::de::Error::custom)?,

            low_price: vec[3]
                .as_str()
                .ok_or_else(|| serde::de::Error::custom("Invalid low_price"))?
                .parse()
                .map_err(serde::de::Error::custom)?,

            close_price: vec[4]
                .as_str()
                .ok_or_else(|| serde::de::Error::custom("Invalid close_price"))?
                .parse()
                .map_err(serde::de::Error::custom)?,

            volume: vec[5]
                .as_str()
                .ok_or_else(|| serde::de::Error::custom("Invalid volume"))?
                .parse()
                .map_err(serde::de::Error::custom)?,

            close_time: vec[6]
                .as_u64()
                .ok_or_else(|| serde::de::Error::custom("Invalid close_time"))?,

            quote_asset_volume: vec[7]
                .as_str()
                .ok_or_else(|| serde::de::Error::custom("Invalid quote_asset_volume"))?
                .parse()
                .map_err(serde::de::Error::custom)?,

            number_of_trades: vec[8]
                .as_u64()
                .ok_or_else(|| serde::de::Error::custom("Invalid number_of_trades"))?,

            taker_buy_base_asset_volume: vec[9]
                .as_str()
                .ok_or_else(|| serde::de::Error::custom("Invalid taker_buy_base_asset_volume"))?
                .parse()
                .map_err(serde::de::Error::custom)?,

            taker_buy_quote_asset_volume: vec[10]
                .as_str()
                .ok_or_else(|| serde::de::Error::custom("Invalid taker_buy_quote_asset_volume"))?
                .parse()
                .map_err(serde::de::Error::custom)?,
        });
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum KlineInterval {
    OneSecond,
    OneMinute,
    ThreeMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    TwoHours,
    FourHours,
    SixHours,
    EightHours,
    TwelveHours,
    OneDay,
    ThreeDays,
    OneWeek,
    OneMonth,
}

impl fmt::Display for KlineInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            KlineInterval::OneSecond => "1s",
            KlineInterval::OneMinute => "1m",
            KlineInterval::ThreeMinutes => "3m",
            KlineInterval::FiveMinutes => "5m",
            KlineInterval::FifteenMinutes => "15m",
            KlineInterval::ThirtyMinutes => "30m",
            KlineInterval::OneHour => "1h",
            KlineInterval::TwoHours => "2h",
            KlineInterval::FourHours => "4h",
            KlineInterval::SixHours => "6h",
            KlineInterval::EightHours => "8h",
            KlineInterval::TwelveHours => "12h",
            KlineInterval::OneDay => "1d",
            KlineInterval::ThreeDays => "3d",
            KlineInterval::OneWeek => "1w",
            KlineInterval::OneMonth => "1M",
        };
        write!(f, "{}", s)
    }
}
