//! Password hashing (Argon2id PHC strings) and credential validation.
//!
//! Verification uses parameters embedded in each stored hash; [`PasswordConfig`] env tuning
//! applies only to **new** hashes from [`hash_password`].

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
use rand_core::OsRng;

use super::error::AuthError;

const DEFAULT_MIN_LENGTH: usize = 8;
const DEFAULT_MAX_LENGTH: usize = 256;
/// Memory cost (KiB). OWASP suggests ~19 MiB for Argon2id.
const DEFAULT_ARGON2_M_COST: u32 = 19_456;
const DEFAULT_ARGON2_T_COST: u32 = 3;
const DEFAULT_ARGON2_P_COST: u32 = 1;

#[derive(Debug, Clone)]
pub struct PasswordConfig {
    pub min_length: usize,
    pub max_length: usize,
    m_cost: u32,
    t_cost: u32,
    p_cost: u32,
}

impl PasswordConfig {
    pub fn from_env() -> Self {
        Self {
            min_length: parse_usize_env("AUTH_PASSWORD_MIN_LENGTH", DEFAULT_MIN_LENGTH),
            max_length: parse_usize_env("AUTH_PASSWORD_MAX_LENGTH", DEFAULT_MAX_LENGTH),
            m_cost: parse_u32_env("AUTH_ARGON2_M_COST", DEFAULT_ARGON2_M_COST),
            t_cost: parse_u32_env("AUTH_ARGON2_T_COST", DEFAULT_ARGON2_T_COST),
            p_cost: parse_u32_env("AUTH_ARGON2_P_COST", DEFAULT_ARGON2_P_COST),
        }
    }

    fn argon2(&self) -> Result<Argon2<'static>, AuthError> {
        let params = Params::new(self.m_cost, self.t_cost, self.p_cost, None)
            .map_err(|_| AuthError::HashingFailed)?;
        Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
    }
}

fn parse_u32_env(key: &str, default: u32) -> u32 {
    std::env::var(key)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

fn parse_usize_env(key: &str, default: usize) -> usize {
    std::env::var(key)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

/// Trim, Unicode lowercase, then validate a minimal email shape (no full RFC 5322).
pub fn normalize_email(raw: &str) -> Result<String, AuthError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(AuthError::InvalidEmail);
    }
    let normalized = trimmed.to_lowercase();
    if !email_format_ok(&normalized) {
        return Err(AuthError::InvalidEmail);
    }
    Ok(normalized)
}

/// Deliberately small dependency-free check: length bounds, single `@`, non-empty parts.
fn email_format_ok(s: &str) -> bool {
    if s.len() > 254 {
        return false;
    }
    let Some((local, domain)) = s.split_once('@') else {
        return false;
    };
    if local.is_empty() || domain.is_empty() {
        return false;
    }
    if local.contains('@') || domain.contains('@') {
        return false;
    }
    if domain.starts_with('.') || domain.ends_with('.') || domain.contains("..") {
        return false;
    }
    true
}

pub fn validate_password_policy(plain: &str, config: &PasswordConfig) -> Result<(), AuthError> {
    let len = plain.len();
    if plain.trim().is_empty() {
        return Err(AuthError::WeakPassword);
    }
    if len < config.min_length {
        return Err(AuthError::InvalidPassword);
    }
    if len > config.max_length {
        return Err(AuthError::InvalidPassword);
    }
    Ok(())
}

pub fn hash_password(plain: &str, config: &PasswordConfig) -> Result<String, AuthError> {
    validate_password_policy(plain, config)?;
    let argon2 = config.argon2()?;
    let salt = SaltString::generate(&mut OsRng);
    argon2
        .hash_password(plain.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|_| AuthError::HashingFailed)
}

/// Returns `Ok(true)` if the password matches, `Ok(false)` if it does not.
/// `Err` means the stored hash could not be parsed or verification failed for non-password reasons.
pub fn verify_password(plain: &str, stored_hash: &str) -> Result<bool, AuthError> {
    let parsed = PasswordHash::new(stored_hash).map_err(|_| AuthError::InvalidPasswordHash)?;
    let argon2 = Argon2::default();
    match argon2.verify_password(plain.as_bytes(), &parsed) {
        Ok(()) => Ok(true),
        Err(password_hash::Error::Password) => Ok(false),
        Err(_) => Err(AuthError::InvalidPasswordHash),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> PasswordConfig {
        PasswordConfig {
            min_length: 8,
            max_length: 256,
            m_cost: 8 * 1024,
            t_cost: 2,
            p_cost: 1,
        }
    }

    #[test]
    fn hash_then_verify_ok() {
        let config = test_config();
        let hash = hash_password("correct-horse-battery-staple", &config).unwrap();
        assert!(verify_password("correct-horse-battery-staple", &hash).unwrap());
    }

    #[test]
    fn verify_wrong_password_false() {
        let config = test_config();
        let hash = hash_password("secret-password", &config).unwrap();
        assert!(!verify_password("wrong-password", &hash).unwrap());
    }

    #[test]
    fn password_too_short() {
        let config = test_config();
        let err = hash_password("short", &config).unwrap_err();
        assert_eq!(err, AuthError::InvalidPassword);
    }

    #[test]
    fn password_too_long() {
        let config = PasswordConfig {
            max_length: 10,
            ..test_config()
        };
        let err = hash_password("12345678901", &config).unwrap_err();
        assert_eq!(err, AuthError::InvalidPassword);
    }

    #[test]
    fn empty_password_weak() {
        let config = test_config();
        let err = validate_password_policy("   ", &config).unwrap_err();
        assert_eq!(err, AuthError::WeakPassword);
    }

    #[test]
    fn normalize_email_trims_and_lowercases() {
        assert_eq!(
            normalize_email("  Test@Example.COM  ").unwrap(),
            "test@example.com"
        );
    }

    #[test]
    fn invalid_email_no_at() {
        assert_eq!(
            normalize_email("not-an-email"),
            Err(AuthError::InvalidEmail)
        );
    }

    #[test]
    fn invalid_email_empty() {
        assert_eq!(normalize_email("   "), Err(AuthError::InvalidEmail));
    }

    #[test]
    fn verify_invalid_hash_string() {
        let r = verify_password("x", "not-a-phc-string");
        assert_eq!(r, Err(AuthError::InvalidPasswordHash));
    }
}
