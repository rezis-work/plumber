//! Errors for `POST /auth/refresh` (Step 9): generic **401** for any client-visible auth failure.

use axum::response::{IntoResponse, Response};

use super::auth_unauthorized::AuthUnauthorized;
use super::login_error::LoginError;

#[derive(Debug)]
pub enum RefreshError {
    /// Same JSON as [`AuthUnauthorized`] — do not distinguish missing cookie vs bad JWT vs reuse.
    Unauthorized,
    Internal,
}

impl IntoResponse for RefreshError {
    fn into_response(self) -> Response {
        match self {
            RefreshError::Unauthorized => AuthUnauthorized.into_response(),
            RefreshError::Internal => LoginError::Internal.into_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RefreshError;
    use axum::response::IntoResponse;

    use super::AuthUnauthorized;

    #[tokio::test]
    async fn unauthorized_same_body_as_auth_unauthorized() {
        let a = RefreshError::Unauthorized.into_response();
        let b = AuthUnauthorized.into_response();
        assert_eq!(a.status(), b.status());
        let ba = axum::body::to_bytes(a.into_body(), usize::MAX)
            .await
            .unwrap();
        let bb = axum::body::to_bytes(b.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(ba, bb);
    }
}
