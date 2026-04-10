//! HMAC-SHA256 of the **raw** refresh JWT for DB storage (Step 5).
//!
//! Use the same secret as refresh JWT signing ([`AUTH_JWT_REFRESH_SECRET`](super::service_token::JwtConfig))
//! so one rotation updates both verification and lookup binding. For stricter separation, introduce
//! a dedicated `AUTH_REFRESH_TOKEN_HMAC_SECRET` later and pass it here instead.

use hmac::{Hmac, Mac};
use sha2::Sha256;

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
}
