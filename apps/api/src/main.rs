use std::time::Duration;

use api::modules::auth::auth_routes;
use api::modules::auth::{CookieConfig, EmailVerificationConfig, JwtConfig, PasswordConfig};
use api::modules::geography::GeographyRepository;
use api::modules::orders::orders_routes;
use api::modules::orders::OrderRepository;
use api::modules::service_categories::ServiceCategoryRepository;
use api::modules::users::{RefreshTokenRepository, UserRepository};
use api::AppState;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{AllowOrigin, CorsLayer};

async fn health() -> &'static str {
    "ok"
}

/// When `CORS_ALLOWED_ORIGINS` is set (comma-separated), enable credentialed CORS for browser clients.
fn cors_layer_from_env() -> Option<CorsLayer> {
    let raw = std::env::var("CORS_ALLOWED_ORIGINS").ok()?;
    let origins: Vec<HeaderValue> = raw
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|s| HeaderValue::from_str(s).ok())
        .collect();
    if origins.is_empty() {
        return None;
    }
    Some(
        CorsLayer::new()
            .allow_origin(AllowOrigin::list(origins))
            .allow_credentials(true)
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers([AUTHORIZATION, CONTENT_TYPE]),
    )
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");

    let state = AppState {
        pool: pool.clone(),
        users: UserRepository::new(pool.clone()),
        orders: OrderRepository::new(pool.clone()),
        geography: GeographyRepository::new(pool.clone()),
        service_categories: ServiceCategoryRepository::new(pool.clone()),
        refresh_tokens: RefreshTokenRepository::new(pool.clone()),
        password_config: PasswordConfig::from_env(),
        email_verification: EmailVerificationConfig::from_env(),
        jwt_config: JwtConfig::from_env(),
        cookie_config: CookieConfig::from_env(),
    };

    let mut app = Router::new()
        .route("/health", get(health))
        .nest("/auth", auth_routes(state.clone()))
        .nest("/orders", orders_routes(state.clone()))
        .with_state(state);

    if let Some(cors) = cors_layer_from_env() {
        app = app.layer(cors);
    }

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("failed to bind");

    println!("API running on http://0.0.0.0:3001");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}
