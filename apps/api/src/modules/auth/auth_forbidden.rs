//! Stable 403 JSON when the access token is valid but the role is not allowed (Step 8).

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct AuthForbiddenBody {
    pub error: &'static str,
    pub message: &'static str,
}

/// Returned when [`super::auth_context::AuthContext`] is present but its role fails RBAC.
#[derive(Debug, Clone, Copy)]
pub struct AuthForbidden;

impl IntoResponse for AuthForbidden {
    fn into_response(self) -> Response {
        (
            StatusCode::FORBIDDEN,
            Json(AuthForbiddenBody {
                error: "forbidden",
                message: "insufficient permissions",
            }),
        )
            .into_response()
    }
}
