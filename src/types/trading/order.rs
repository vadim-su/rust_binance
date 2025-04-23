use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

const ORDER_RESULT_TYPE: &str = "RESULT";

fn get_order_result_type() -> String {
    ORDER_RESULT_TYPE.to_string()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CommonOrderCreateData {
    pub symbol: String,
    pub side: OrderSide,
    pub timestamp: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    #[serde(default = "get_order_result_type")]
    pub new_order_resp_type: String,
}

impl Default for CommonOrderCreateData {
    fn default() -> Self {
        Self {
            symbol: String::new(),
            side: OrderSide::default(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
            new_order_resp_type: get_order_result_type(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(
    rename_all = "SCREAMING_SNAKE_CASE",
    rename_all_fields = "camelCase",
    tag = "type"
)]
pub enum OrderCreationData {
    Limit {
        #[serde(flatten)]
        common: CommonOrderCreateData,
        time_in_force: TimeInForce,
        quantity: Decimal,
        price: Decimal,
        #[serde(skip_serializing_if = "Option::is_none")]
        iceberg_qty: Option<Decimal>,
    },

    Market {
        #[serde(flatten)]
        common: CommonOrderCreateData,
        #[serde(skip_serializing_if = "Option::is_none")]
        quantity: Option<Decimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        quote_order_qty: Option<Decimal>,
    },

    StopLoss {
        #[serde(flatten)]
        common: CommonOrderCreateData,
        quantity: Decimal,
        #[serde(skip_serializing_if = "Option::is_none")]
        stop_price: Option<Decimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        trailing_delta: Option<i64>,
    },

    StopLossLimit {
        #[serde(flatten)]
        common: CommonOrderCreateData,
        time_in_force: TimeInForce,
        quantity: Decimal,
        price: Decimal,
        #[serde(skip_serializing_if = "Option::is_none")]
        stop_price: Option<Decimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        trailing_delta: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        iceberg_qty: Option<Decimal>,
    },

    TakeProfit {
        #[serde(flatten)]
        common: CommonOrderCreateData,
        quantity: Decimal,
        #[serde(skip_serializing_if = "Option::is_none")]
        stop_price: Option<Decimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        trailing_delta: Option<i64>,
    },

    TakeProfitLimit {
        #[serde(flatten)]
        common: CommonOrderCreateData,
        time_in_force: TimeInForce,
        quantity: Decimal,
        price: Decimal,
        #[serde(skip_serializing_if = "Option::is_none")]
        stop_price: Option<Decimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        trailing_delta: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        iceberg_qty: Option<Decimal>,
    },

    LimitMaker {
        #[serde(flatten)]
        common: CommonOrderCreateData,
        quantity: Decimal,
        price: Decimal,
    },
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    #[default]
    Buy,
    Sell,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    #[default]
    Gtc,
    Ioc,
    Fok,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SelfTradePreventionMode {
    #[default]
    None,
    ExpireMaker,
    ExpireTaker,
    ExpireBoth,
    Decrement,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PendingNew,
    PartiallyFilled,
    Filled,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,
    ExpiredInMatch,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_field_names)]
pub struct Order {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: Option<i64>,
    pub price: Decimal,
    pub orig_qty: Decimal,
    pub executed_qty: Decimal,
    pub orig_quote_order_qty: Decimal,
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub type_: String,
    pub side: OrderSide,
    pub working_time: Option<i64>,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}
