//! Bearer access JWT middleware (Step 7).

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum::http::header::AUTHORIZATION;

use crate::AppState;

use super::auth_context::AuthContext;
use super::auth_unauthorized::AuthUnauthorized;

/// Extract token from `Authorization: Bearer <token>` (scheme case-insensitive).
fn bearer_token(header_value: &str) -> Option<&str> {
    let s = header_value.trim();
    let split = s.find(|c: char| c.is_whitespace())?;
    let scheme = &s[..split];
    if !scheme.eq_ignore_ascii_case("bearer") {
        return None;
    }
    let t = s[split..].trim();
    if t.is_empty() {
        return None;
    }
    Some(t)
}

pub async fn require_access_token(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AuthUnauthorized> {
    let Some(authz) = req.headers().get(AUTHORIZATION).and_then(|v| v.to_str().ok()) else {
        return Err(AuthUnauthorized);
    };
    let Some(raw) = bearer_token(authz) else {
        return Err(AuthUnauthorized);
    };

    let claims = state
        .jwt_config
        .verify_access_token(raw)
        .map_err(|_| AuthUnauthorized)?;

    let ctx = AuthContext::from_claims(&claims).map_err(|_| AuthUnauthorized)?;
    req.extensions_mut().insert(ctx);
    Ok(next.run(req).await)
}
