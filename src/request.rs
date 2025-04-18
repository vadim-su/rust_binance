use hmac::{Hmac, Mac};
use reqwest::{Request, header::InvalidHeaderValue};
use serde::Serialize;
use sha2::Sha256;
use url::Url;

const API_KEY_HEADER: &str = "X-MBX-APIKEY";

pub fn make_request<T: Serialize>(
    client: &reqwest::Client,
    method: reqwest::Method,
    url: &Url,
    data: &T,
) -> Result<Request, reqwest::Error> {
    let mut url = url.clone();
    let query = serde_qs::to_string(data).unwrap();
    url.set_query(Some(&query));

    println!("Request URL: {url}");

    let request = client.request(method, url).build()?;
    return Ok(request);
}

pub fn sign_request(
    mut request: Request,
    api_key: &str,
    secret: &str,
) -> Result<Request, InvalidHeaderValue> {
    let query = request.url().query().unwrap_or("");
    let signature = sign_query(query, secret);
    {
        let mut pairs = request.url_mut().query_pairs_mut();
        pairs.append_pair("signature", &signature);
    }

    request.headers_mut().insert(
        API_KEY_HEADER,
        reqwest::header::HeaderValue::from_str(api_key)?,
    );

    Ok(request)
}

pub fn sign_query(query: &str, secret: &str) -> String {
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(query.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use super::*;

    #[test]
    fn test_sign_query() {
        let secret = sign_query("test_query", "test_secret");
        assert_eq!(
            secret,
            "7540b9f8f77656ba870356a6f7d58e591857830316119d775828c98756814301"
        );
    }

    #[test]
    fn test_sign_request() {
        let request = Client::new()
            .get("https://api.binance.com/api/v3/order")
            .query(&[("symbol", "BTCUSDT"), ("side", "BUY")])
            .build()
            .unwrap();

        let signed_request = sign_request(request, "test_apikey", "test_secret").unwrap();

        assert_eq!(
            signed_request
                .headers()
                .get(API_KEY_HEADER)
                .unwrap()
                .to_str()
                .unwrap(),
            "test_apikey"
        );

        assert!(
            signed_request
                .url()
                .query_pairs()
                .any(|(k, v)| k == "signature"
                    && v == sign_query("symbol=BTCUSDT&side=BUY", "test_secret"))
        );
    }

    #[test]
    fn test_sign_request_with_empty_query() {
        let request = Client::new()
            .get("https://api.binance.com/api/v3/order")
            .build()
            .unwrap();

        let signed_request = sign_request(request, "test_apikey", "test_secret").unwrap();

        assert!(
            signed_request
                .headers()
                .get(API_KEY_HEADER)
                .unwrap()
                .to_str()
                .unwrap()
                == "test_apikey"
        );

        assert!(
            signed_request
                .url()
                .query_pairs()
                .any(|(k, v)| k == "signature" && v == sign_query("", "test_secret"))
        );
    }
}
