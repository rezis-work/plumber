use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum RegisterError {
    Validation { message: String },
    Conflict,
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    error: &'static str,
    message: String,
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        match self {
            RegisterError::Validation { message } => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "validation_error",
                    message,
                }),
            )
                .into_response(),
            RegisterError::Conflict => (
                StatusCode::CONFLICT,
                Json(ErrorBody {
                    error: "conflict",
                    message: "email already registered".to_string(),
                }),
            )
                .into_response(),
            RegisterError::Internal => (
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
