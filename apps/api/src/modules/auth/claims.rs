//! JWT claims for access and refresh tokens (Step 4).

use serde::{Deserialize, Serialize};

/// Serialized in JWT as `token_type`: `"access"` | `"refresh"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
}

/// Custom + registered claims shared by access and refresh tokens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthJwtClaims {
    pub sub: String,
    pub role: String,
    #[serde(rename = "token_type")]
    pub token_type: TokenType,
    pub jti: String,
    pub exp: i64,
    pub iat: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
}
