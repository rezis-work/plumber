//! HMAC-SHA256 of the **raw** refresh JWT for DB storage (Step 5).
//!
//! Use the same secret as refresh JWT signing ([`AUTH_JWT_REFRESH_SECRET`](super::service_token::JwtConfig))
//! so one rotation updates both verification and lookup binding. For stricter separation, introduce
//! a dedicated `AUTH_REFRESH_TOKEN_HMAC_SECRET` later and pass it here instead.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use subtle::ConstantTimeEq;

type HmacSha256 = Hmac<Sha256>;

/// Returns hex-encoded HMAC-SHA256(`refresh_secret`, `raw_jwt` UTF-8 bytes). Never store the plaintext JWT.
pub fn hash_refresh_jwt_for_storage(refresh_secret: &str, raw_jwt: &str) -> Result<String, ()> {
    if refresh_secret.is_empty() {
        return Err(());
    }
    let mut mac = HmacSha256::new_from_slice(refresh_secret.as_bytes()).map_err(|_| ())?;
    mac.update(raw_jwt.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

/// Constant-time equality of two hex-encoded digests (e.g. stored `token_hash` vs recomputed HMAC).
pub fn refresh_token_hash_hex_eq_constant_time(a_hex: &str, b_hex: &str) -> bool {
    let Ok(a) = hex::decode(a_hex) else {
        return false;
    };
    let Ok(b) = hex::decode(b_hex) else {
        return false;
    };
    if a.len() != b.len() {
        return false;
    }
    bool::from(a.ct_eq(&b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_inputs_same_digest() {
        let a = hash_refresh_jwt_for_storage("secret", "header.payload.sig").unwrap();
        let b = hash_refresh_jwt_for_storage("secret", "header.payload.sig").unwrap();
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
    }

    #[test]
    fn different_jwt_different_digest() {
        let a = hash_refresh_jwt_for_storage("secret", "a.b.c").unwrap();
        let b = hash_refresh_jwt_for_storage("secret", "a.b.d").unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn empty_secret_rejected() {
        assert!(hash_refresh_jwt_for_storage("", "x").is_err());
    }

    #[test]
    fn hex_eq_constant_time_matches_hash_outputs() {
        let h = hash_refresh_jwt_for_storage("s", "jwt").unwrap();
        assert!(refresh_token_hash_hex_eq_constant_time(&h, &h));
        let h2 = hash_refresh_jwt_for_storage("s", "jwt2").unwrap();
        assert!(!refresh_token_hash_hex_eq_constant_time(&h, &h2));
    }

    #[test]
    fn hex_eq_rejects_bad_hex() {
        assert!(!refresh_token_hash_hex_eq_constant_time("gg", "gg"));
    }
}
