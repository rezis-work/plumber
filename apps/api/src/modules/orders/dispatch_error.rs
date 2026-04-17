use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum DispatchActionError {
    OrderNotFound,
    DispatchNotFound,
    Forbidden,
    OrderNotOpen,
    DispatchNotSent,
    PlumberProfileMissing,
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    error: &'static str,
    message: String,
}

impl IntoResponse for DispatchActionError {
    fn into_response(self) -> Response {
        match self {
            DispatchActionError::OrderNotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorBody {
                    error: "order_not_found",
                    message: "order not found".to_string(),
                }),
            )
                .into_response(),
            DispatchActionError::DispatchNotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorBody {
                    error: "dispatch_not_found",
                    message: "dispatch not found for this order".to_string(),
                }),
            )
                .into_response(),
            DispatchActionError::Forbidden => (
                StatusCode::FORBIDDEN,
                Json(ErrorBody {
                    error: "forbidden",
                    message: "not your dispatch".to_string(),
                }),
            )
                .into_response(),
            DispatchActionError::OrderNotOpen => (
                StatusCode::CONFLICT,
                Json(ErrorBody {
                    error: "order_not_open",
                    message: "order is not open for acceptance".to_string(),
                }),
            )
                .into_response(),
            DispatchActionError::DispatchNotSent => (
                StatusCode::CONFLICT,
                Json(ErrorBody {
                    error: "dispatch_not_sent",
                    message: "dispatch is not in sent status".to_string(),
                }),
            )
                .into_response(),
            DispatchActionError::PlumberProfileMissing => (
                StatusCode::FORBIDDEN,
                Json(ErrorBody {
                    error: "plumber_profile_missing",
                    message: "plumber profile not found".to_string(),
                }),
            )
                .into_response(),
            DispatchActionError::Internal => (
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

impl From<sqlx::Error> for DispatchActionError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("dispatch action db error: {e}");
        DispatchActionError::Internal
    }
}
