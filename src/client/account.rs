use std::collections::HashMap;

use reqwest::{Client, Method};
use url::Url;

use crate::errors::BinanceError;
use crate::request::{make_request, sign_request};
use crate::types::account::Account;
use crate::types::general::Error;

use super::get_base_url;

#[derive(Debug, Clone)]
pub struct BinanceAccountClient {
    client: Client,
    api_key: String,
    secret: String,
    base_url: Url,
}

impl BinanceAccountClient {
    pub fn new(client: Client, api_key: String, secret: String, testnet: bool) -> Self {
        return Self {
            client,
            api_key,
            secret,
            base_url: get_base_url(testnet),
        };
    }

    pub async fn get_account(
        &self,
        omit_zero_balances: Option<bool>,
        recv_window: Option<i64>,
    ) -> Result<Account, BinanceError> {
        let url = self.base_url.join("account")?;
        let method = Method::GET;
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();

        let mut params = HashMap::new();
        params.insert("timestamp", timestamp);

        if let Some(omit) = omit_zero_balances {
            params.insert("omitZeroBalances", omit.to_string());
        }

        if let Some(window) = recv_window {
            params.insert("recvWindow", window.to_string());
        }

        let request = make_request(&self.client, method, &url, &params)?;
        let signed_request = sign_request(request, &self.api_key, &self.secret).unwrap();

        let response = self.client.execute(signed_request).await?;

        if !response.status().is_success() {
            let status_code: u16 = response.status().as_u16();
            let error: Error = response.json().await?;
            return Err(BinanceError::Api(status_code, error));
        }

        let account: Account = response.json().await?;

        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_new_client() {
        let api_key = "test_api_key";
        let secret = "test_secret";

        let client =
            BinanceAccountClient::new(Client::new(), api_key.to_string(), secret.to_string(), true);
        assert_eq!(client.api_key, api_key);
        assert_eq!(client.secret, secret);
        assert_eq!(
            client.base_url.as_str(),
            "https://testnet.binance.vision/api/v3/"
        );

        let client = BinanceAccountClient::new(
            Client::new(),
            api_key.to_string(),
            secret.to_string(),
            false,
        );
        assert_eq!(client.api_key, api_key);
        assert_eq!(client.secret, secret);
        assert_eq!(client.base_url.as_str(), "https://api.binance.com/api/v3/");
    }

    #[tokio::test]
    async fn test_get_account() {
        let api_key = env::var("BINANCE_TEST_API_KEY").unwrap();
        let secret = env::var("BINANCE_TEST_SECRET").unwrap();

        let client =
            BinanceAccountClient::new(Client::new(), api_key.clone(), secret.clone(), true);

        let account = client.get_account(Some(true), None).await.unwrap();
        assert!(!account.balances.is_empty());
    }
}
