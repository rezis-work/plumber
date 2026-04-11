use axum::middleware;
use axum::routing::{get, post};
use axum::Router;

use crate::AppState;

use super::handler;
use super::middleware_access::require_access_token;

pub fn auth_routes(state: AppState) -> Router<AppState> {
    let public = Router::new()
        .route("/register/client", post(handler::register_client))
        .route("/register/plumber", post(handler::register_plumber))
        .route("/login", post(handler::login));

    let protected = Router::new()
        .route("/me", get(handler::me))
        .layer(middleware::from_fn_with_state(
            state,
            require_access_token,
        ));

    public.merge(protected)
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::header::AUTHORIZATION;
    use axum::http::{Request, StatusCode};
    use axum::Router;
    use http_body_util::BodyExt;
    use serde_json::json;
    use sqlx::PgPool;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::modules::auth::cookie_config::CookieConfig;
    use crate::modules::auth::passwords::PasswordConfig;
    use crate::modules::auth::service_token::JwtConfig;
    use crate::modules::auth::verification::EmailVerificationConfig;
    use crate::modules::users::{RefreshTokenRepository, Role, UserRepository};
    use crate::AppState;

    use super::auth_routes;

    fn test_state(jwt: JwtConfig) -> AppState {
        let pool =
            PgPool::connect_lazy("postgres://127.0.0.1:1/plumber_auth_me_router_tests").unwrap();
        AppState {
            pool: pool.clone(),
            users: UserRepository::new(pool.clone()),
            refresh_tokens: RefreshTokenRepository::new(pool.clone()),
            password_config: PasswordConfig::from_env(),
            email_verification: EmailVerificationConfig::from_env(),
            jwt_config: jwt,
            cookie_config: CookieConfig::from_env(),
        }
    }

    fn app(jwt: JwtConfig) -> Router {
        let state = test_state(jwt);
        Router::new()
            .nest("/auth", auth_routes(state.clone()))
            .with_state(state)
    }

    fn assert_unauthorized(body: &[u8]) {
        let v: serde_json::Value = serde_json::from_slice(body).expect("json body");
        assert_eq!(v["error"], json!("unauthorized"));
        assert_eq!(v["message"], json!("authentication required"));
    }

    #[tokio::test]
    async fn auth_me_ok_with_valid_access_token() {
        let jwt = JwtConfig::test_config();
        let uid = Uuid::new_v4();
        let token = jwt
            .create_access_token(uid, Role::Plumber)
            .expect("access token");

        let res = app(jwt)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/auth/me")
                    .header(AUTHORIZATION, format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("oneshot");

        assert_eq!(res.status(), StatusCode::OK);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(v["user_id"], json!(uid.to_string()));
        assert_eq!(v["role"], json!("plumber"));
    }

    #[tokio::test]
    async fn auth_me_401_no_authorization_header() {
        let jwt = JwtConfig::test_config();
        let res = app(jwt)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/auth/me")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("oneshot");

        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        assert_unauthorized(&body);
    }

    #[tokio::test]
    async fn auth_me_401_wrong_scheme() {
        let jwt = JwtConfig::test_config();
        let res = app(jwt)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/auth/me")
                    .header(AUTHORIZATION, "Basic abc")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("oneshot");

        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        assert_unauthorized(&body);
    }

    #[tokio::test]
    async fn auth_me_401_bearer_empty_token() {
        let jwt = JwtConfig::test_config();
        let res = app(jwt)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/auth/me")
                    .header(AUTHORIZATION, "Bearer ")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("oneshot");

        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        assert_unauthorized(&body);
    }

    #[tokio::test]
    async fn auth_me_401_garbage_token() {
        let jwt = JwtConfig::test_config();
        let res = app(jwt)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/auth/me")
                    .header(AUTHORIZATION, "Bearer not.a.valid.jwt")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("oneshot");

        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        assert_unauthorized(&body);
    }

    #[tokio::test]
    async fn auth_me_401_refresh_jwt_as_bearer() {
        let jwt = JwtConfig::test_config();
        let refresh = jwt
            .create_refresh_token(
                Uuid::new_v4(),
                Role::Client,
                &Uuid::new_v4().to_string(),
            )
            .expect("refresh");

        let res = app(jwt)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/auth/me")
                    .header(AUTHORIZATION, format!("Bearer {}", refresh))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("oneshot");

        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        assert_unauthorized(&body);
    }
}
