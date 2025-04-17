use hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn sign_query(query: &str, secret: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("Invalid key");
    mac.update(query.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_query() {
        let secret = sign_query("test_query", "test_secret");
        assert_eq!(
            secret,
            "7540b9f8f77656ba870356a6f7d58e591857830316119d775828c98756814301"
        );
    }
}
