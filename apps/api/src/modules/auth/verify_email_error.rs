use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum VerifyEmailError {
    Validation { message: String },
    InvalidToken,
    TokenExpired,
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    error: &'static str,
    message: String,
}

impl IntoResponse for VerifyEmailError {
    fn into_response(self) -> Response {
        match self {
            VerifyEmailError::Validation { message } => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "validation_error",
                    message,
                }),
            )
                .into_response(),
            VerifyEmailError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorBody {
                    error: "invalid_token",
                    message: "invalid or expired verification link".to_string(),
                }),
            )
                .into_response(),
            VerifyEmailError::TokenExpired => (
                StatusCode::GONE,
                Json(ErrorBody {
                    error: "token_expired",
                    message: "this verification link has expired".to_string(),
                }),
            )
                .into_response(),
            VerifyEmailError::Internal => (
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
