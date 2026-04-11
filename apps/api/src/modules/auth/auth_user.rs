//! Extractor for [`AuthContext`] after [`super::middleware_access::require_access_token`] (Step 7).

use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use super::auth_context::AuthContext;
use super::auth_unauthorized::AuthUnauthorized;

#[derive(Debug, Clone, Copy)]
pub struct AuthUser(pub AuthContext);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthUnauthorized;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthContext>()
            .copied()
            .map(AuthUser)
            .ok_or(AuthUnauthorized)
    }
}
