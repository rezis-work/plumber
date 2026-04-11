//! Request-scoped identity after access JWT verification (Step 7).

use serde::Serialize;
use uuid::Uuid;

use crate::modules::users::Role;

use super::claims::AuthJwtClaims;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct AuthContext {
    pub user_id: Uuid,
    pub role: Role,
}

impl AuthContext {
    pub(crate) fn from_claims(claims: &AuthJwtClaims) -> Result<Self, ()> {
        let user_id = Uuid::parse_str(claims.sub.trim()).map_err(|_| ())?;
        let role = parse_role_claim(&claims.role).ok_or(())?;
        Ok(Self { user_id, role })
    }
}

fn parse_role_claim(s: &str) -> Option<Role> {
    match s.trim() {
        "client" => Some(Role::Client),
        "plumber" => Some(Role::Plumber),
        "admin" => Some(Role::Admin),
        _ => None,
    }
}
