use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum CompletionError {
    OrderNotFound,
    Forbidden,
    OrderNotInProgress,
    OrderNotAccepted,
    NoAssignedPlumber,
    NoWinningDispatch,
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    error: &'static str,
    message: String,
}

impl IntoResponse for CompletionError {
    fn into_response(self) -> Response {
        match self {
            CompletionError::OrderNotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorBody {
                    error: "order_not_found",
                    message: "order not found".to_string(),
                }),
            )
                .into_response(),
            CompletionError::Forbidden => (
                StatusCode::FORBIDDEN,
                Json(ErrorBody {
                    error: "forbidden",
                    message: "not allowed for this order".to_string(),
                }),
            )
                .into_response(),
            CompletionError::OrderNotInProgress => (
                StatusCode::CONFLICT,
                Json(ErrorBody {
                    error: "order_not_in_progress",
                    message: "order must be in progress to complete".to_string(),
                }),
            )
                .into_response(),
            CompletionError::OrderNotAccepted => (
                StatusCode::CONFLICT,
                Json(ErrorBody {
                    error: "order_not_accepted",
                    message: "order must be accepted to start".to_string(),
                }),
            )
                .into_response(),
            CompletionError::NoAssignedPlumber => (
                StatusCode::CONFLICT,
                Json(ErrorBody {
                    error: "no_assigned_plumber",
                    message: "order has no assigned plumber".to_string(),
                }),
            )
                .into_response(),
            CompletionError::NoWinningDispatch => (
                StatusCode::CONFLICT,
                Json(ErrorBody {
                    error: "no_winning_dispatch",
                    message: "no accepted dispatch for assigned plumber".to_string(),
                }),
            )
                .into_response(),
            CompletionError::Internal => (
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

impl From<sqlx::Error> for CompletionError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("completion db error: {e}");
        CompletionError::Internal
    }
}
