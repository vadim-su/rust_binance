use core::fmt;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Clone)]
pub struct Kline {
    pub open_time: DateTime<Utc>,
    pub open_price: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
    pub close_price: Decimal,
    pub volume: Decimal,
    pub close_time: DateTime<Utc>,
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
            open_time: chrono::DateTime::<Utc>::from_timestamp_millis({
                let open_time_u64 = vec[0]
                    .as_u64()
                    .ok_or_else(|| serde::de::Error::custom("Invalid open_time"))?;
                i64::try_from(open_time_u64)
                    .map_err(|_| serde::de::Error::custom("open_time out of range for i64"))?
            })
            .ok_or_else(|| serde::de::Error::custom("Invalid open_time millis"))?,

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

            close_time: chrono::DateTime::<Utc>::from_timestamp_millis({
                let close_time_u64 = vec[6]
                    .as_u64()
                    .ok_or_else(|| serde::de::Error::custom("Invalid close_time"))?;
                i64::try_from(close_time_u64)
                    .map_err(|_| serde::de::Error::custom("close_time out of range for i64"))?
            })
            .ok_or_else(|| serde::de::Error::custom("Invalid open_time millis"))?,

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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
            Self::OneSecond => "1s",
            Self::OneMinute => "1m",
            Self::ThreeMinutes => "3m",
            Self::FiveMinutes => "5m",
            Self::FifteenMinutes => "15m",
            Self::ThirtyMinutes => "30m",
            Self::OneHour => "1h",
            Self::TwoHours => "2h",
            Self::FourHours => "4h",
            Self::SixHours => "6h",
            Self::EightHours => "8h",
            Self::TwelveHours => "12h",
            Self::OneDay => "1d",
            Self::ThreeDays => "3d",
            Self::OneWeek => "1w",
            Self::OneMonth => "1M",
        };
        write!(f, "{s}")
    }
}
