mod general;
mod market;
mod root;

use url::Url;

pub use root::BinanceClient;

/// The API version to use for the Binance API.
pub const API_VERSION: &str = "v3";

/// Returns the base URL for the Binance API depending on whether the testnet is used or not.
fn get_base_url(testnet: bool) -> Url {
    if testnet {
        return format!("https://testnet.binance.vision/api/{API_VERSION}/")
            .parse()
            .unwrap();
    }
    return format!("https://api.binance.com/api/{API_VERSION}/")
        .parse()
        .unwrap();
}
