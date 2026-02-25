use hmac::{Hmac, Mac};
use sha2::Sha256;

fn get_timestamp() -> u128 {
    // We expect system time to always be after UNIX_EPOCH (1970-01-01).
    // If this fails, the machine's clock is misconfigured, which is a fatal
    // environment issue — not something the application should recover from.
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System clock is before UNIX_EPOCH — check system time configuration")
        .as_millis()
}

fn create_signature(query_string: &str, secret: &str) -> String {
    // HMAC accepts keys of any size. If this fails, something is
    // fundamentally wrong with the crypto configuration.
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .expect("Failed to create HMAC-SHA256 instance — invalid secret key");

    mac.update(query_string.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}

use reqwest::Method;

use crate::error::BinanceError;

pub async fn send_signed_request(
    client: &reqwest::Client,
    method: Method,
    base_url: &str,
    endpoint: &str,
    api_key: &str,
    api_secret: &str,
    mut query_string: String,
) -> Result<String,BinanceError> {
    let timestamp = get_timestamp();

    if !query_string.is_empty() {
        query_string.push('&');
    }

    query_string.push_str(&format!("timestamp={}", timestamp));

    let signature = create_signature(&query_string, api_secret);

    let url: String = format!("{}/{}", base_url, endpoint);

    let request = match method {
        Method::GET => client.request(
            method,
            format!("{}?{}&signature={}", url, query_string, signature),
        ),
        _ => client
            .request(method, url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!("{}&signature={}", query_string, signature)),
    };

    let response = request.header("X-MBX-APIKEY", api_key).send().await?;

    let text = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
        return Ok(serde_json::to_string_pretty(&json)?);
    }

    Ok(text)
}

#[cfg(test)]
mod tests_timestamp {
    use super::*;

    #[test]
    fn test_timestamp_is_positive() {
        let ts = get_timestamp();
        assert!(ts > 0);
    }

    #[test]
    fn test_timestamp_is_reasonable() {
        // Assert timestamp is after 2020-01-01 and before 2100-01-01
        let ts = get_timestamp();
        assert!(ts > 1_577_836_800_000); // 2020-01-01 in ms
        assert!(ts < 4_102_444_800_000); // 2100-01-01 in ms
    }

    #[test]
    fn test_timestamps_are_monotonic() {
        let ts1 = get_timestamp();
        let ts2 = get_timestamp();
        assert!(ts2 >= ts1);
    }

    #[test]
    fn test_timestamps_increase_over_time() {
        let ts1 = get_timestamp();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let ts2 = get_timestamp();
        assert!(ts2 > ts1);
    }
}

#[cfg(test)]
mod tests_signature {
    use super::*;

    #[test]
    fn test_signature_is_valid_hex() {
        let sig = create_signature("foo=bar&baz=qux", "secret");
        assert!(sig.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_signature_is_correct_length() {
        // HMAC-SHA256 produces 32 bytes = 64 hex characters
        let sig = create_signature("foo=bar", "secret");
        assert_eq!(sig.len(), 64);
    }

    #[test]
    fn test_signature_is_deterministic() {
        let sig1 = create_signature("foo=bar&baz=qux", "secret");
        let sig2 = create_signature("foo=bar&baz=qux", "secret");
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_different_query_strings_produce_different_signatures() {
        let sig1 = create_signature("foo=bar", "secret");
        let sig2 = create_signature("foo=baz", "secret");
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_different_secrets_produce_different_signatures() {
        let sig1 = create_signature("foo=bar", "secret1");
        let sig2 = create_signature("foo=bar", "secret2");
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_known_value() {
        // Pre-computed: echo -n "symbol=BTCUSDT&side=BUY" | openssl dgst -sha256 -hmac "mysecret"
        let sig = create_signature("symbol=BTCUSDT&side=BUY", "mysecret");
        assert_eq!(
            sig,
            "f0fe50c8f82b55b3da13325f82379ff550b523c5853d73595f2a848688bd3434"
        );
    }

    #[test]
    fn test_empty_query_string() {
        let sig = create_signature("", "secret");
        assert_eq!(sig.len(), 64);
    }

    #[test]
    fn test_empty_secret() {
        let sig = create_signature("foo=bar", "");
        assert_eq!(sig.len(), 64);
    }

    #[test]
    fn test_special_characters_in_query() {
        // URL-encoded values should be treated as plain strings
        let sig1 = create_signature("foo=hello%20world", "secret");
        let sig2 = create_signature("foo=hello%20world", "secret");
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_parameter_order_matters() {
        let sig1 = create_signature("a=1&b=2", "secret");
        let sig2 = create_signature("b=2&a=1", "secret");
        assert_ne!(sig1, sig2);
    }
}
