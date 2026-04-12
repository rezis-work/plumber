//! Errors for admin-only user lifecycle routes (`§6.1` block / soft-delete).

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum AdminUserError {
    NotFound,
    Forbidden,
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    error: &'static str,
    message: String,
}

impl IntoResponse for AdminUserError {
    fn into_response(self) -> Response {
        match self {
            AdminUserError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorBody {
                    error: "not_found",
                    message: "user not found or not applicable".to_string(),
                }),
            )
                .into_response(),
            AdminUserError::Forbidden => (
                StatusCode::FORBIDDEN,
                Json(ErrorBody {
                    error: "forbidden",
                    message: "cannot apply this action to your own account".to_string(),
                }),
            )
                .into_response(),
            AdminUserError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody {
                    error: "internal_error",
                    message: "something went wrong".to_string(),
                }),
            )
                .into_response(),
        }
    }
}
