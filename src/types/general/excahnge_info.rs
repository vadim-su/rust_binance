use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<serde_json::Value>,
    pub symbols: Vec<SymbolInfo>,
    pub sors: Option<Vec<Sor>>,
}

#[derive(Debug, Deserialize)]
pub struct RateLimit {
    #[serde(flatten)]
    pub details: serde_json::Value,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u8,
    pub quote_asset: String,
    pub quote_precision: u8,
    pub quote_asset_precision: u8,
    pub base_commission_precision: u8,
    pub quote_commission_precision: u8,
    pub order_types: Vec<String>,
    pub iceberg_allowed: bool,
    pub oco_allowed: bool,
    pub oto_allowed: bool,
    pub quote_order_qty_market_allowed: bool,
    pub allow_trailing_stop: bool,
    pub cancel_replace_allowed: bool,
    pub allow_amend: Option<bool>,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub filters: Vec<serde_json::Value>,
    pub permissions: Vec<String>,
    pub permission_sets: Vec<Vec<String>>,
    pub default_self_trade_prevention_mode: String,
    pub allowed_self_trade_prevention_modes: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sor {
    pub base_asset: String,
    pub symbols: Vec<String>,
}
