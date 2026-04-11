use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum LoginError {
    /// Malformed / empty email after normalize rules.
    Validation { message: String },
    /// Unknown user or wrong password — identical JSON body.
    InvalidCredentials,
    AccountInactive,
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    error: &'static str,
    message: String,
}

impl IntoResponse for LoginError {
    fn into_response(self) -> Response {
        match self {
            LoginError::Validation { message } => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "validation_error",
                    message,
                }),
            )
                .into_response(),
            LoginError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorBody {
                    error: "invalid_credentials",
                    message: "invalid email or password".to_string(),
                }),
            )
                .into_response(),
            LoginError::AccountInactive => (
                StatusCode::FORBIDDEN,
                Json(ErrorBody {
                    error: "account_inactive",
                    message: "this account is disabled".to_string(),
                }),
            )
                .into_response(),
            LoginError::Internal => (
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn invalid_credentials_same_body() {
        let a = LoginError::InvalidCredentials.into_response();
        let b = LoginError::InvalidCredentials.into_response();
        assert_eq!(a.status(), b.status());
        let ba = axum::body::to_bytes(a.into_body(), usize::MAX).await.unwrap();
        let bb = axum::body::to_bytes(b.into_body(), usize::MAX).await.unwrap();
        assert_eq!(ba, bb);
        let v: serde_json::Value = serde_json::from_slice(&ba).unwrap();
        assert_eq!(v["error"], "invalid_credentials");
        assert_eq!(v["message"], "invalid email or password");
    }
}
