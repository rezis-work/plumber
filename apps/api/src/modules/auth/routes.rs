use axum::middleware;
use axum::routing::{get, post};
use axum::Router;

use crate::modules::users::Role;
use crate::AppState;

use super::handler;
use super::middleware_access::require_access_token;

/// Roles allowed on [`handler::rbac_staff_check`] (Step 8 `require_any_role` demo).
const RBAC_STAFF_ROLES: &[Role] = &[Role::Plumber, Role::Admin];

pub fn auth_routes(state: AppState) -> Router<AppState> {
    let public = Router::new()
        .route("/register/client", post(handler::register_client))
        .route("/register/plumber", post(handler::register_plumber))
        .route("/verify-email", post(handler::verify_email))
        .route("/login", post(handler::login))
        .route("/refresh", post(handler::refresh))
        .route("/logout", post(handler::logout));

    let protected = Router::new()
        .route("/me", get(handler::me))
        .route("/logout-all", post(handler::logout_all))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_access_token,
        ));

    // Step 8 stubs: access middleware outer, role inner (`MethodRouter::layer` order).
    let rbac_plumber = Router::new().route(
        "/rbac/plumber-check",
        get(handler::rbac_plumber_check)
            .layer(crate::require_role!(Role::Plumber))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                require_access_token,
            )),
    );

    let rbac_admin = Router::new().route(
        "/rbac/admin-check",
        get(handler::rbac_admin_check)
            .layer(crate::require_role!(Role::Admin))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                require_access_token,
            )),
    );

    let admin_users = Router::new()
        .route(
            "/admin/users/{user_id}/block",
            post(handler::admin_block_user),
        )
        .route(
            "/admin/users/{user_id}/soft-delete",
            post(handler::admin_soft_delete_user),
        )
        .layer(crate::require_role!(Role::Admin))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_access_token,
        ));

    let rbac_staff = Router::new().route(
        "/rbac/staff-check",
        get(handler::rbac_staff_check)
            .layer(crate::require_any_role!(RBAC_STAFF_ROLES))
            .layer(middleware::from_fn_with_state(state, require_access_token)),
    );

    public
        .merge(protected)
        .merge(rbac_plumber)
        .merge(rbac_admin)
        .merge(admin_users)
        .merge(rbac_staff)
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

    fn assert_forbidden(body: &[u8]) {
        let v: serde_json::Value = serde_json::from_slice(body).expect("json body");
        assert_eq!(v["error"], json!("forbidden"));
        assert_eq!(v["message"], json!("insufficient permissions"));
    }

    fn bearer_get(uri: &str, token: &str) -> Request<Body> {
        Request::builder()
            .method("GET")
            .uri(uri)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap()
    }

    fn bearer_post(uri: &str, token: &str) -> Request<Body> {
        Request::builder()
            .method("POST")
            .uri(uri)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap()
    }

    fn test_app_state(pool: PgPool) -> AppState {
        AppState {
            pool: pool.clone(),
            users: UserRepository::new(pool.clone()),
            refresh_tokens: RefreshTokenRepository::new(pool.clone()),
            password_config: PasswordConfig::from_env(),
            email_verification: EmailVerificationConfig {
                secret: "integration-test-hmac-key".to_string(),
                ttl_hours: 48,
            },
            jwt_config: JwtConfig::from_env(),
            cookie_config: CookieConfig::from_env(),
        }
    }

    fn app_with_pool(state: AppState) -> Router {
        Router::new()
            .nest("/auth", auth_routes(state.clone()))
            .with_state(state)
    }

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test auth_me_ok_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn auth_me_ok_client_profile_null(pool: PgPool) -> sqlx::Result<()> {
        use crate::modules::auth::passwords::hash_password;
        use crate::modules::users::CreateUserParams;

        let state = test_app_state(pool);
        let ph = hash_password("password123", &state.password_config).unwrap();
        let user = state
            .users
            .create_user(CreateUserParams {
                email: "me-client@example.com",
                password_hash: &ph,
                role: Role::Client,
                user_status: crate::modules::users::UserStatus::Active,
                is_email_verified: true,
                email_verification_token_hash: None,
                email_verification_expires_at: None,
            })
            .await?;

        let token = state
            .jwt_config
            .create_access_token(user.id, Role::Client)
            .expect("access token");

        let res = app_with_pool(state)
            .oneshot(bearer_get("/auth/me", &token))
            .await
            .expect("oneshot");

        assert_eq!(res.status(), StatusCode::OK);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        let v: serde_json::Value = serde_json::from_slice(&body).expect("json");
        assert_eq!(v["id"], json!(user.id.to_string()));
        assert_eq!(v["email"], json!("me-client@example.com"));
        assert_eq!(v["role"], json!("client"));
        assert_eq!(v["status"], json!("active"));
        assert_eq!(v["is_active"], json!(true));
        assert_eq!(v["is_email_verified"], json!(true));
        assert_eq!(v["profile"], serde_json::Value::Null);

        Ok(())
    }

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test auth_me_ok_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn auth_me_ok_plumber_with_profile(pool: PgPool) -> sqlx::Result<()> {
        use crate::modules::auth::dto::RegisterPlumberRequest;
        use crate::modules::auth::service;

        let state = test_app_state(pool);
        let reg = service::register_plumber(
            &state,
            RegisterPlumberRequest {
                email: "me-plumber@example.com".to_string(),
                password: "password123".to_string(),
                full_name: "Pat Pipe".to_string(),
                phone: "+1 555 0100".to_string(),
                years_of_experience: 3,
            },
        )
        .await
        .expect("register_plumber");

        let token = state
            .jwt_config
            .create_access_token(reg.user.id, Role::Plumber)
            .expect("access token");

        let res = app_with_pool(state)
            .oneshot(bearer_get("/auth/me", &token))
            .await
            .expect("oneshot");

        assert_eq!(res.status(), StatusCode::OK);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        let v: serde_json::Value = serde_json::from_slice(&body).expect("json");
        assert_eq!(v["id"], json!(reg.user.id.to_string()));
        assert_eq!(v["email"], json!("me-plumber@example.com"));
        assert_eq!(v["role"], json!("plumber"));
        assert_eq!(v["profile"]["full_name"], json!("Pat Pipe"));
        assert_eq!(v["profile"]["phone"], json!("+15550100"));
        assert_eq!(v["profile"]["years_of_experience"], json!(3));

        Ok(())
    }

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test auth_me_404_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn auth_me_404_valid_token_user_missing(pool: PgPool) -> sqlx::Result<()> {
        let state = test_app_state(pool);
        let ghost = Uuid::new_v4();
        let token = state
            .jwt_config
            .create_access_token(ghost, Role::Client)
            .expect("access token");

        let res = app_with_pool(state)
            .oneshot(bearer_get("/auth/me", &token))
            .await
            .expect("oneshot");

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        let v: serde_json::Value = serde_json::from_slice(&body).expect("json");
        assert_eq!(v["error"], json!("not_found"));

        Ok(())
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
    async fn auth_logout_all_401_no_authorization_header() {
        let jwt = JwtConfig::test_config();
        let res = app(jwt)
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/auth/logout-all")
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

    #[tokio::test]
    async fn rbac_plumber_check_403_for_client_token() {
        let jwt = JwtConfig::test_config();
        let token = jwt
            .create_access_token(Uuid::new_v4(), Role::Client)
            .expect("token");
        let res = app(jwt)
            .oneshot(bearer_get("/auth/rbac/plumber-check", &token))
            .await
            .expect("oneshot");
        assert_eq!(res.status(), StatusCode::FORBIDDEN);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        assert_forbidden(&body);
    }

    #[tokio::test]
    async fn rbac_admin_check_403_for_client_token() {
        let jwt = JwtConfig::test_config();
        let token = jwt
            .create_access_token(Uuid::new_v4(), Role::Client)
            .expect("token");
        let res = app(jwt)
            .oneshot(bearer_get("/auth/rbac/admin-check", &token))
            .await
            .expect("oneshot");
        assert_eq!(res.status(), StatusCode::FORBIDDEN);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        assert_forbidden(&body);
    }

    #[tokio::test]
    async fn rbac_plumber_check_200_for_plumber() {
        let jwt = JwtConfig::test_config();
        let token = jwt
            .create_access_token(Uuid::new_v4(), Role::Plumber)
            .expect("token");
        let res = app(jwt)
            .oneshot(bearer_get("/auth/rbac/plumber-check", &token))
            .await
            .expect("oneshot");
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn rbac_admin_check_200_for_admin() {
        let jwt = JwtConfig::test_config();
        let token = jwt
            .create_access_token(Uuid::new_v4(), Role::Admin)
            .expect("token");
        let res = app(jwt)
            .oneshot(bearer_get("/auth/rbac/admin-check", &token))
            .await
            .expect("oneshot");
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn rbac_admin_check_403_for_plumber() {
        let jwt = JwtConfig::test_config();
        let token = jwt
            .create_access_token(Uuid::new_v4(), Role::Plumber)
            .expect("token");
        let res = app(jwt)
            .oneshot(bearer_get("/auth/rbac/admin-check", &token))
            .await
            .expect("oneshot");
        assert_eq!(res.status(), StatusCode::FORBIDDEN);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        assert_forbidden(&body);
    }

    #[tokio::test]
    async fn rbac_staff_check_403_for_client() {
        let jwt = JwtConfig::test_config();
        let token = jwt
            .create_access_token(Uuid::new_v4(), Role::Client)
            .expect("token");
        let res = app(jwt)
            .oneshot(bearer_get("/auth/rbac/staff-check", &token))
            .await
            .expect("oneshot");
        assert_eq!(res.status(), StatusCode::FORBIDDEN);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        assert_forbidden(&body);
    }

    #[tokio::test]
    async fn rbac_staff_check_200_for_plumber_and_admin() {
        let jwt = JwtConfig::test_config();
        for role in [Role::Plumber, Role::Admin] {
            let token = jwt
                .create_access_token(Uuid::new_v4(), role)
                .expect("token");
            let res = app(jwt.clone())
                .oneshot(bearer_get("/auth/rbac/staff-check", &token))
                .await
                .expect("oneshot");
            assert_eq!(res.status(), StatusCode::OK, "role {:?}", role);
        }
    }

    #[tokio::test]
    async fn rbac_plumber_check_401_without_bearer() {
        let jwt = JwtConfig::test_config();
        let res = app(jwt)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/auth/rbac/plumber-check")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("oneshot");
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        assert_unauthorized(&body);
    }

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test admin_block_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn admin_block_unknown_user_returns_404(pool: PgPool) -> sqlx::Result<()> {
        let state = test_app_state(pool);
        let token = state
            .jwt_config
            .create_access_token(Uuid::new_v4(), Role::Admin)
            .expect("admin access token");
        let unknown = Uuid::new_v4();
        let res = app_with_pool(state)
            .oneshot(bearer_post(
                &format!("/auth/admin/users/{unknown}/block"),
                &token,
            ))
            .await
            .expect("oneshot");
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        Ok(())
    }
}
