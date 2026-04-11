//! Stable 401 JSON for missing/invalid Bearer access token (Step 7).

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct AuthUnauthorizedBody {
    pub error: &'static str,
    pub message: &'static str,
}

/// Returned when `Authorization` is missing, not Bearer, or access JWT verification fails.
#[derive(Debug, Clone, Copy)]
pub struct AuthUnauthorized;

impl IntoResponse for AuthUnauthorized {
    fn into_response(self) -> Response {
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthUnauthorizedBody {
                error: "unauthorized",
                message: "authentication required",
            }),
        )
            .into_response()
    }
}
