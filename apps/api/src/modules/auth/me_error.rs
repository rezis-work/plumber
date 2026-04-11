//! Errors for `GET /auth/me` (Step 12).

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum MeError {
    NotFound,
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    error: &'static str,
    message: String,
}

impl IntoResponse for MeError {
    fn into_response(self) -> Response {
        match self {
            MeError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorBody {
                    error: "not_found",
                    message: "user not found".to_string(),
                }),
            )
                .into_response(),
            MeError::Internal => (
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
