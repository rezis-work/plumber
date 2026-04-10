use std::time::Duration;

use api::modules::auth::auth_routes;
use api::modules::auth::{EmailVerificationConfig, PasswordConfig};
use api::AppState;
use api::modules::users::UserRepository;
use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;

async fn health() -> &'static str {
    "ok"
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
        password_config: PasswordConfig::from_env(),
        email_verification: EmailVerificationConfig::from_env(),
    };

    let app = Router::new()
        .route("/health", get(health))
        .nest("/auth", auth_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("failed to bind");

    println!("API running on http://0.0.0.0:3001");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}
