//! Email verification token: random raw bytes (exposed once as hex), HMAC-SHA256 stored in DB.

use hmac::{Hmac, Mac};
use rand_core::{OsRng, RngCore};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct EmailVerificationConfig {
    /// HMAC key; set `AUTH_EMAIL_VERIFICATION_SECRET` in production.
    pub secret: String,
    pub ttl_hours: u64,
}

impl EmailVerificationConfig {
    pub fn from_env() -> Self {
        let secret = std::env::var("AUTH_EMAIL_VERIFICATION_SECRET").unwrap_or_else(|_| {
            "development-only-pepper-not-for-production".to_string()
        });
        let ttl_hours = std::env::var("AUTH_EMAIL_VERIFICATION_TTL_HOURS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(48);
        Self { secret, ttl_hours }
    }

    /// Hex-encoded raw 32-byte token for the client; store [`hash_raw_token`] in the database.
    pub fn generate_raw_token_hex() -> String {
        let mut raw = [0u8; 32];
        OsRng.fill_bytes(&mut raw);
        hex::encode(raw)
    }

    pub fn hash_raw_token_hex(&self, raw_token_hex: &str) -> Result<String, ()> {
        let raw = hex::decode(raw_token_hex).map_err(|_| ())?;
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes()).map_err(|_| ())?;
        mac.update(&raw);
        Ok(hex::encode(mac.finalize().into_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_token_is_64_hex_chars() {
        let t = EmailVerificationConfig::generate_raw_token_hex();
        assert_eq!(t.len(), 64);
        assert!(t.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn hmac_hash_stable_for_same_secret_and_token() {
        let cfg = EmailVerificationConfig {
            secret: "k".to_string(),
            ttl_hours: 1,
        };
        let raw = EmailVerificationConfig::generate_raw_token_hex();
        let a = cfg.hash_raw_token_hex(&raw).unwrap();
        let b = cfg.hash_raw_token_hex(&raw).unwrap();
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
    }
}
