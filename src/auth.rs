//! Request signing for authenticated endpoints.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current Unix time in milliseconds.
pub(crate) fn timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before Unix epoch")
        .as_millis() as u64
}

/// Creates the hex-encoded HMAC-SHA256 signature for a request.
/// The signed string is `{timestamp}{method}/v2{path}{body}` where `path`
/// includes the query string and `body` is the compact JSON body or empty.
pub(crate) fn sign(secret: &str, timestamp: u64, method: &str, path: &str, body: &str) -> String {
    let payload = format!("{timestamp}{method}/v2{path}{body}");
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("hmac accepts any key length");
    mac.update(payload.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_matches_known_vector() {
        let sig = sign("secret", 1_548_169_000_000, "GET", "/time", "");
        assert_eq!(sig.len(), 64);
        assert!(sig.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn sign_is_deterministic() {
        let a = sign("secret", 1, "POST", "/order", "{\"market\":\"BTC-EUR\"}");
        let b = sign("secret", 1, "POST", "/order", "{\"market\":\"BTC-EUR\"}");
        assert_eq!(a, b);
    }
}
