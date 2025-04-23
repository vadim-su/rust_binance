use std::collections::HashMap;

use chrono::{DateTime, Utc};
use reqwest::{Client, Method};
use url::Url;

use crate::errors::BinanceError;
use crate::request::{make_request, sign_request};
use crate::types::general::Error;
use crate::types::trading::{Order, OrderCreationData};

use super::get_base_url;

#[derive(Debug, Clone)]
pub struct BinanceTradingClient {
    client: Client,
    api_key: String,
    secret: String,
    base_url: Url,
    recv_window: Option<u32>,
}

impl BinanceTradingClient {
    pub fn new(
        client: Client,
        api_key: String,
        secret: String,
        recv_window: Option<u32>,
        testnet: bool,
    ) -> Self {
        return Self {
            client,
            api_key,
            secret,
            base_url: get_base_url(testnet),
            recv_window,
        };
    }

    pub async fn create_order(&self, order_data: OrderCreationData) -> Result<Order, BinanceError> {
        let url = self.base_url.join("order")?;
        let data = &order_data;
        let method = Method::POST;

        let request = make_request(&self.client, method, &url, data)?;
        let signed_request = sign_request(request, &self.api_key, &self.secret).unwrap();

        let response = self.client.execute(signed_request).await?;

        if !response.status().is_success() {
            let status_code: u16 = response.status().as_u16();
            let error: Error = response.json().await?;
            return Err(BinanceError::Api(status_code, error));
        }

        let order: Order = response.json().await?;

        Ok(order)
    }

    pub async fn get_order(
        &self,
        symbol: &str,
        order_id: Option<u64>,
        orig_client_order_id: Option<&str>,
        recv_window: Option<u32>,
    ) -> Result<Order, BinanceError> {
        let url = self.base_url.join("order")?;
        let method = Method::GET;
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();

        let mut params = HashMap::new();

        params.insert("symbol", symbol.to_string());
        params.insert("timestamp", timestamp);

        if order_id.is_none() && orig_client_order_id.is_none() {
            return Err(BinanceError::MissingParameter(
                "Either orderId or origClientOrderId must be provided".to_string(),
            ));
        }

        if let Some(order_id) = order_id {
            params.insert("orderId", order_id.to_string());
        }
        if let Some(orig_client_order_id) = orig_client_order_id {
            params.insert("origClientOrderId", orig_client_order_id.to_string());
        }
        if let Some(recv_window) = recv_window {
            params.insert("recvWindow", recv_window.to_string());
        }

        let request = make_request(&self.client, method, &url, &params)?;
        let signed_request = sign_request(request, &self.api_key, &self.secret).unwrap();

        let response = self.client.execute(signed_request).await?;

        if !response.status().is_success() {
            let status_code: u16 = response.status().as_u16();
            let error: Error = response.json().await?;
            return Err(BinanceError::Api(status_code, error));
        }

        let order: Order = response.json().await?;

        Ok(order)
    }

    pub async fn get_open_orders(
        &self,
        symbol: Option<&str>,
        recv_window: Option<u32>,
    ) -> Result<Vec<Order>, BinanceError> {
        let url = self.base_url.join("openOrders")?;
        let method = Method::GET;
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();

        let mut params = HashMap::new();
        params.insert("timestamp", timestamp);

        if let Some(symbol) = symbol {
            params.insert("symbol", symbol.to_string());
        }
        if let Some(recv_window) = recv_window {
            params.insert("recvWindow", recv_window.to_string());
        }

        let request = make_request(&self.client, method, &url, &params)?;
        let signed_request = sign_request(request, &self.api_key, &self.secret).unwrap();

        let response = self.client.execute(signed_request).await?;

        if !response.status().is_success() {
            let status_code: u16 = response.status().as_u16();
            let error: Error = response.json().await?;
            return Err(BinanceError::Api(status_code, error));
        }

        let orders: Vec<Order> = response.json().await?;

        Ok(orders)
    }

    pub async fn get_orders(
        &self,
        symbol: &str,
        order_id: Option<u64>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: Option<u32>,
    ) -> Result<Vec<Order>, BinanceError> {
        let url = self.base_url.join("allOrders")?;
        let method = Method::GET;
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();

        let mut params = HashMap::new();
        params.insert("symbol", symbol.to_string());
        params.insert("timestamp", timestamp);

        if let Some(order_id) = order_id {
            params.insert("orderId", order_id.to_string());
        }
        if let Some(start_time) = start_time {
            params.insert("startTime", start_time.timestamp_millis().to_string());
        }
        if let Some(end_time) = end_time {
            params.insert("endTime", end_time.timestamp_millis().to_string());
        }
        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(recv_window) = self.recv_window {
            params.insert("recvWindow", recv_window.to_string());
        }

        let request = make_request(&self.client, method, &url, &params)?;
        let signed_request = sign_request(request, &self.api_key, &self.secret).unwrap();

        let response = self.client.execute(signed_request).await?;

        if !response.status().is_success() {
            let status_code: u16 = response.status().as_u16();
            let error: Error = response.json().await?;
            return Err(BinanceError::Api(status_code, error));
        }

        let orders: Vec<Order> = response.json().await?;

        Ok(orders)
    }

    pub async fn cancel_order(
        &self,
        symbol: &str,
        order_id: Option<u64>,
        orig_client_order_id: Option<&str>,
        new_client_order_id: Option<&str>,
        cancel_restriction: Option<&str>,
        recv_window: Option<u32>,
    ) -> Result<Order, BinanceError> {
        let url = self.base_url.join("order")?;
        let method = Method::DELETE;
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();

        let mut params = HashMap::new();
        params.insert("symbol", symbol.to_string());
        params.insert("timestamp", timestamp);

        if order_id.is_none() && orig_client_order_id.is_none() {
            return Err(BinanceError::MissingParameter(
                "Either orderId or origClientOrderId must be provided".to_string(),
            ));
        }

        if let Some(order_id) = order_id {
            params.insert("orderId", order_id.to_string());
        }
        if let Some(orig_client_order_id) = orig_client_order_id {
            params.insert("origClientOrderId", orig_client_order_id.to_string());
        }
        if let Some(new_client_order_id) = new_client_order_id {
            params.insert("newClientOrderId", new_client_order_id.to_string());
        }
        if let Some(cancel_restrictions) = cancel_restriction {
            params.insert("cancelRestrictions", cancel_restrictions.to_string());
        }
        if let Some(recv_window) = recv_window {
            params.insert("recvWindow", recv_window.to_string());
        }

        let request = make_request(&self.client, method, &url, &params)?;
        let signed_request = sign_request(request, &self.api_key, &self.secret).unwrap();

        let response = self.client.execute(signed_request).await?;

        if !response.status().is_success() {
            let status_code: u16 = response.status().as_u16();
            let error: Error = response.json().await?;
            return Err(BinanceError::Api(status_code, error));
        }

        let order: Order = response.json().await?;

        Ok(order)
    }

    pub async fn cancel_open_orders(
        &self,
        symbol: &str,
        recv_window: Option<u32>,
    ) -> Result<Vec<Order>, BinanceError> {
        let url = self.base_url.join("openOrders")?;
        let method = Method::DELETE;
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();

        let mut params = HashMap::new();
        params.insert("symbol", symbol.to_string());
        params.insert("timestamp", timestamp);

        if let Some(recv_window) = recv_window {
            params.insert("recvWindow", recv_window.to_string());
        }

        let request = make_request(&self.client, method, &url, &params)?;
        let signed_request = sign_request(request, &self.api_key, &self.secret).unwrap();

        let response = self.client.execute(signed_request).await?;

        if !response.status().is_success() {
            let status_code: u16 = response.status().as_u16();
            let error: Error = response.json().await?;
            return Err(BinanceError::Api(status_code, error));
        }

        let orders: Vec<Order> = response.json().await?;

        Ok(orders)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use rust_decimal::Decimal;
    use serial_test::serial;

    use crate::types::trading::{CommonOrderCreateData, OrderSide, OrderStatus, TimeInForce};

    use super::*;

    #[test]
    fn test_new_client() {
        let api_key = "test_api_key";
        let secret = "test_secret";

        let client = BinanceTradingClient::new(
            Client::new(),
            api_key.to_string(),
            secret.to_string(),
            None,
            true,
        );
        assert_eq!(client.api_key, api_key);
        assert_eq!(client.secret, secret);
        assert_eq!(
            client.base_url.as_str(),
            "https://testnet.binance.vision/api/v3/"
        );

        let client = BinanceTradingClient::new(
            Client::new(),
            api_key.to_string(),
            secret.to_string(),
            None,
            false,
        );
        assert_eq!(client.api_key, api_key);
        assert_eq!(client.secret, secret);
        assert_eq!(client.base_url.as_str(), "https://api.binance.com/api/v3/");
    }

    #[tokio::test]
    async fn test_create_order() {
        let api_key = env::var("BINANCE_TEST_API_KEY").unwrap();
        let secret = env::var("BINANCE_TEST_SECRET").unwrap();

        let order_data = OrderCreationData::Limit {
            common: CommonOrderCreateData {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                timestamp: chrono::Utc::now().timestamp_millis().try_into().unwrap(),
                ..Default::default()
            },
            time_in_force: TimeInForce::Fok,
            quantity: Decimal::from_str_exact("0.0001").unwrap(),
            price: Decimal::from_str_exact("80000").unwrap(),
            iceberg_qty: None,
        };

        let client = BinanceTradingClient::new(
            Client::new(),
            api_key.to_string(),
            secret.to_string(),
            None,
            true,
        );
        let result = client.create_order(order_data).await.unwrap();
        assert_eq!(result.symbol, "BTCUSDT");
        assert_eq!(result.price, Decimal::from_str_exact("80000").unwrap());
        assert_eq!(result.time_in_force, TimeInForce::Fok);
        assert_eq!(result.side, OrderSide::Buy);
        assert_eq!(result.status, OrderStatus::Expired);
    }

    #[tokio::test]
    async fn test_get_open_orders() {
        let api_key = env::var("BINANCE_TEST_API_KEY").unwrap();
        let secret = env::var("BINANCE_TEST_SECRET").unwrap();

        let client =
            BinanceTradingClient::new(Client::new(), api_key.clone(), secret.clone(), None, true);

        client.get_open_orders(None, None).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_orders() {
        let api_key = env::var("BINANCE_TEST_API_KEY").unwrap();
        let secret = env::var("BINANCE_TEST_SECRET").unwrap();

        let client =
            BinanceTradingClient::new(Client::new(), api_key.clone(), secret.clone(), None, true);

        client
            .get_orders("BTCUSDT", None, None, None, None)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn test_order_pipeline() {
        let api_key = env::var("BINANCE_TEST_API_KEY").unwrap();
        let secret = env::var("BINANCE_TEST_SECRET").unwrap();

        let client = BinanceTradingClient::new(Client::new(), api_key, secret, None, true);

        let order_data = OrderCreationData::Limit {
            common: CommonOrderCreateData {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                timestamp: chrono::Utc::now().timestamp_millis().try_into().unwrap(),
                ..Default::default()
            },
            time_in_force: TimeInForce::Gtc,
            quantity: Decimal::from_str_exact("0.0001").unwrap(),
            price: Decimal::from_str_exact("80000").unwrap(),
            iceberg_qty: None,
        };

        let created_order = client.create_order(order_data).await.unwrap();
        assert_eq!(created_order.symbol, "BTCUSDT");
        assert_eq!(created_order.time_in_force, TimeInForce::Gtc);
        assert_eq!(created_order.side, OrderSide::Buy);

        let order = client
            .get_order("BTCUSDT", Some(created_order.order_id), None, None)
            .await
            .unwrap();

        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.order_id, created_order.order_id);
        assert_eq!(order.status, OrderStatus::New);

        let order = client
            .cancel_order(
                "BTCUSDT",
                Some(created_order.order_id),
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();

        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.order_id, created_order.order_id);
        assert_eq!(order.status, OrderStatus::Canceled);
    }

    #[tokio::test]
    #[serial]
    async fn test_cancel_open_orders() {
        let api_key = env::var("BINANCE_TEST_API_KEY").unwrap();
        let secret = env::var("BINANCE_TEST_SECRET").unwrap();

        let client = BinanceTradingClient::new(Client::new(), api_key, secret, None, true);

        let order_data = OrderCreationData::Limit {
            common: CommonOrderCreateData {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                timestamp: chrono::Utc::now().timestamp_millis().try_into().unwrap(),
                ..Default::default()
            },
            time_in_force: TimeInForce::Gtc,
            quantity: Decimal::from_str_exact("0.0001").unwrap(),
            price: Decimal::from_str_exact("80000").unwrap(),
            iceberg_qty: None,
        };

        let created_order = client.create_order(order_data).await.unwrap();

        let orders = client.cancel_open_orders("BTCUSDT", None).await.unwrap();

        assert!(!orders.is_empty());
        assert_eq!(orders[0].symbol, "BTCUSDT");
        assert_eq!(orders[0].order_id, created_order.order_id);
    }
}
