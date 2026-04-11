//! Mint and verify access/refresh JWTs (HS256). Step 5/6 will persist refresh sessions and expose login.

use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use thiserror::Error;
use uuid::Uuid;

use crate::modules::users::Role;

use super::claims::{AuthJwtClaims, TokenType};

const DEFAULT_DEV_ACCESS_SECRET: &str = "development-only-jwt-access-not-for-production";
const DEFAULT_DEV_REFRESH_SECRET: &str = "development-only-jwt-refresh-not-for-production";
const DEFAULT_ACCESS_TTL_MINUTES: i64 = 15;
const DEFAULT_REFRESH_TTL_DAYS: i64 = 7;

#[derive(Debug, Clone)]
pub struct JwtConfig {
    access_secret: String,
    refresh_secret: String,
    access_ttl_secs: i64,
    refresh_ttl_secs: i64,
    issuer: Option<String>,
    audience: Option<String>,
}

impl JwtConfig {
    pub fn from_env() -> Self {
        let access_secret = std::env::var("AUTH_JWT_ACCESS_SECRET").unwrap_or_else(|_| {
            DEFAULT_DEV_ACCESS_SECRET.to_string()
        });
        let refresh_secret = std::env::var("AUTH_JWT_REFRESH_SECRET").unwrap_or_else(|_| {
            DEFAULT_DEV_REFRESH_SECRET.to_string()
        });
        let access_ttl_minutes = parse_i64_env("AUTH_JWT_ACCESS_TTL_MINUTES", DEFAULT_ACCESS_TTL_MINUTES);
        let refresh_ttl_days = parse_i64_env("AUTH_JWT_REFRESH_TTL_DAYS", DEFAULT_REFRESH_TTL_DAYS);
        let issuer = std::env::var("AUTH_JWT_ISSUER").ok().filter(|s| !s.is_empty());
        let audience = std::env::var("AUTH_JWT_AUDIENCE").ok().filter(|s| !s.is_empty());
        Self {
            access_secret,
            refresh_secret,
            access_ttl_secs: access_ttl_minutes.saturating_mul(60),
            refresh_ttl_secs: refresh_ttl_days.saturating_mul(86_400),
            issuer,
            audience,
        }
    }

    pub fn access_ttl_secs(&self) -> i64 {
        self.access_ttl_secs
    }

    pub fn refresh_ttl_secs(&self) -> i64 {
        self.refresh_ttl_secs
    }

    /// Same secret as refresh JWT signing; used for [`super::refresh_token_hash::hash_refresh_jwt_for_storage`].
    pub fn refresh_secret(&self) -> &str {
        self.refresh_secret.as_str()
    }

    fn validation(&self) -> Validation {
        let mut v = Validation::new(Algorithm::HS256);
        if let Some(ref iss) = self.issuer {
            v.set_issuer(&[iss.as_str()]);
        }
        if let Some(ref aud) = self.audience {
            v.set_audience(&[aud.as_str()]);
            v.validate_aud = true;
        }
        v
    }

    /// Generate `jti`, sign with access secret.
    pub fn create_access_token(&self, user_id: Uuid, role: Role) -> Result<String, JwtError> {
        let jti = Uuid::new_v4().to_string();
        self.sign_access_with_jti(user_id, role, &jti)
    }

    fn sign_access_with_jti(
        &self,
        user_id: Uuid,
        role: Role,
        jti: &str,
    ) -> Result<String, JwtError> {
        let now = Utc::now().timestamp();
        let exp = now + self.access_ttl_secs;
        let claims = AuthJwtClaims {
            sub: user_id.to_string(),
            role: role_to_claim_string(role),
            token_type: TokenType::Access,
            jti: jti.to_string(),
            exp,
            iat: now,
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
        };
        let header = Header::new(Algorithm::HS256);
        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.access_secret.as_bytes()),
        )
        .map_err(JwtError::from)
    }

    /// Caller supplies `jti` (aligned with DB row in Step 5+). Signs with refresh secret.
    pub fn create_refresh_token(
        &self,
        user_id: Uuid,
        role: Role,
        jti: &str,
    ) -> Result<String, JwtError> {
        let now = Utc::now().timestamp();
        let exp = now + self.refresh_ttl_secs;
        let claims = AuthJwtClaims {
            sub: user_id.to_string(),
            role: role_to_claim_string(role),
            token_type: TokenType::Refresh,
            jti: jti.to_string(),
            exp,
            iat: now,
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
        };
        let header = Header::new(Algorithm::HS256);
        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.refresh_secret.as_bytes()),
        )
        .map_err(JwtError::from)
    }

    pub fn verify_access_token(&self, token: &str) -> Result<AuthJwtClaims, JwtError> {
        let data = decode::<AuthJwtClaims>(
            token,
            &DecodingKey::from_secret(self.access_secret.as_bytes()),
            &self.validation(),
        )
        .map_err(JwtError::from)?;
        if data.claims.token_type != TokenType::Access {
            return Err(JwtError::WrongTokenType);
        }
        Ok(data.claims)
    }

    pub fn verify_refresh_token(&self, token: &str) -> Result<AuthJwtClaims, JwtError> {
        let data = decode::<AuthJwtClaims>(
            token,
            &DecodingKey::from_secret(self.refresh_secret.as_bytes()),
            &self.validation(),
        )
        .map_err(JwtError::from)?;
        if data.claims.token_type != TokenType::Refresh {
            return Err(JwtError::WrongTokenType);
        }
        Ok(data.claims)
    }

    #[cfg(test)]
    pub(crate) fn test_config() -> Self {
        Self {
            access_secret: "test-access-secret-key-bytes".to_string(),
            refresh_secret: "test-refresh-secret-other".to_string(),
            access_ttl_secs: 900,
            refresh_ttl_secs: 604_800,
            issuer: None,
            audience: None,
        }
    }

    #[cfg(test)]
    fn test_config_with_iss_aud() -> Self {
        Self {
            access_secret: "test-access-secret-key-bytes".to_string(),
            refresh_secret: "test-refresh-secret-other".to_string(),
            access_ttl_secs: 900,
            refresh_ttl_secs: 604_800,
            issuer: Some("plumber-api".to_string()),
            audience: Some("plumber-clients".to_string()),
        }
    }
}

fn parse_i64_env(key: &str, default: i64) -> i64 {
    std::env::var(key)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

fn role_to_claim_string(role: Role) -> String {
    match role {
        Role::Client => "client".to_string(),
        Role::Plumber => "plumber".to_string(),
        Role::Admin => "admin".to_string(),
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum JwtError {
    #[error("invalid or malformed token")]
    InvalidToken,
    #[error("token signature invalid")]
    InvalidSignature,
    #[error("token expired")]
    Expired,
    #[error("wrong token type")]
    WrongTokenType,
    #[error("jwt error: {0}")]
    Other(String),
}

impl From<jsonwebtoken::errors::Error> for JwtError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        match e.kind() {
            ErrorKind::ExpiredSignature => JwtError::Expired,
            ErrorKind::InvalidSignature => JwtError::InvalidSignature,
            ErrorKind::InvalidToken
            | ErrorKind::InvalidAlgorithm
            | ErrorKind::InvalidAlgorithmName
            | ErrorKind::Json(_)
            | ErrorKind::Utf8(_) => JwtError::InvalidToken,
            _ => JwtError::Other(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access_round_trip() {
        let cfg = JwtConfig::test_config();
        let uid = Uuid::new_v4();
        let token = cfg
            .create_access_token(uid, Role::Client)
            .expect("create access");
        let claims = cfg.verify_access_token(&token).expect("verify access");
        assert_eq!(claims.sub, uid.to_string());
        assert_eq!(claims.role, "client");
        assert_eq!(claims.token_type, TokenType::Access);
        assert!(!claims.jti.is_empty());
    }

    #[test]
    fn refresh_round_trip_with_fixed_jti() {
        let cfg = JwtConfig::test_config();
        let uid = Uuid::new_v4();
        let jti = Uuid::new_v4().to_string();
        let token = cfg
            .create_refresh_token(uid, Role::Plumber, &jti)
            .expect("create refresh");
        let claims = cfg.verify_refresh_token(&token).expect("verify refresh");
        assert_eq!(claims.sub, uid.to_string());
        assert_eq!(claims.role, "plumber");
        assert_eq!(claims.token_type, TokenType::Refresh);
        assert_eq!(claims.jti, jti);
    }

    #[test]
    fn access_token_fails_refresh_verifier() {
        let cfg = JwtConfig::test_config();
        let token = cfg
            .create_access_token(Uuid::new_v4(), Role::Client)
            .unwrap();
        let err = cfg.verify_refresh_token(&token).unwrap_err();
        assert_eq!(err, JwtError::InvalidSignature);
    }

    #[test]
    fn refresh_token_fails_access_verifier() {
        let cfg = JwtConfig::test_config();
        let token = cfg
            .create_refresh_token(Uuid::new_v4(), Role::Client, &Uuid::new_v4().to_string())
            .unwrap();
        let err = cfg.verify_access_token(&token).unwrap_err();
        assert_eq!(err, JwtError::InvalidSignature);
    }

    #[test]
    fn wrong_token_type_with_same_secret_still_rejected() {
        // Misconfiguration: same secret — decode succeeds but token_type guard applies.
        let cfg = JwtConfig {
            access_secret: "same".to_string(),
            refresh_secret: "same".to_string(),
            access_ttl_secs: 60,
            refresh_ttl_secs: 60,
            issuer: None,
            audience: None,
        };
        let access = cfg
            .create_access_token(Uuid::new_v4(), Role::Admin)
            .unwrap();
        assert_eq!(
            cfg.verify_refresh_token(&access).unwrap_err(),
            JwtError::WrongTokenType
        );
        let refresh = cfg
            .create_refresh_token(Uuid::new_v4(), Role::Admin, &Uuid::new_v4().to_string())
            .unwrap();
        assert_eq!(
            cfg.verify_access_token(&refresh).unwrap_err(),
            JwtError::WrongTokenType
        );
    }

    #[test]
    fn expired_access_token_rejected() {
        let cfg = JwtConfig::test_config();
        let uid = Uuid::new_v4();
        let jti = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();
        let claims = AuthJwtClaims {
            sub: uid.to_string(),
            role: "client".to_string(),
            token_type: TokenType::Access,
            jti,
            exp: now - 3600,
            iat: now - 7200,
            iss: None,
            aud: None,
        };
        let header = Header::new(Algorithm::HS256);
        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret(cfg.access_secret.as_bytes()),
        )
        .unwrap();
        assert_eq!(
            cfg.verify_access_token(&token).unwrap_err(),
            JwtError::Expired
        );
    }

    #[test]
    fn iss_aud_round_trip() {
        let cfg = JwtConfig::test_config_with_iss_aud();
        let uid = Uuid::new_v4();
        let token = cfg.create_access_token(uid, Role::Client).unwrap();
        let c = cfg.verify_access_token(&token).unwrap();
        assert_eq!(c.iss.as_deref(), Some("plumber-api"));
        assert_eq!(c.aud.as_deref(), Some("plumber-clients"));
    }
}
