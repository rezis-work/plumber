use std::time::Duration;

use api::modules::auth::auth_routes;
use api::modules::dispatch_matcher::MatcherConfig;
use api::modules::auth::{CookieConfig, EmailVerificationConfig, JwtConfig, PasswordConfig};
use api::modules::dispatch_writer::{
    dispatch_queue_worker_concurrency, dispatch_queue_worker_enabled, dispatch_writer_routes,
    run_dispatch_queue_worker, RedisDispatchHelper,
};
use tokio::sync::watch;
use api::modules::geography::GeographyRepository;
use api::modules::orders::orders_routes;
use api::modules::orders::OrderRepository;
use api::modules::service_categories::ServiceCategoryRepository;
use api::modules::users::{RefreshTokenRepository, UserRepository};
use api::AppState;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use axum::response::IntoResponse;
use axum::{routing::get, Router};

use api::modules::observability;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{AllowOrigin, CorsLayer};

async fn health() -> &'static str {
    "ok"
}

/// Prometheus scrape endpoint; restrict exposure in production (firewall / internal network).
async fn metrics() -> impl IntoResponse {
    (
        [(
            axum::http::header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )],
        observability::metrics_render(),
    )
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

/// When `DISPATCH_INTERNAL_SECRET` is unset, `cargo run` (debug) uses this so `/internal/dispatch/*` works locally.
/// Release builds never use this; omit the env var in production to keep those routes disabled (404).
#[cfg(debug_assertions)]
const DEBUG_DISPATCH_INTERNAL_SECRET: &str = "your-local-dev-secret";

fn dispatch_advance_secret_from_env() -> Option<String> {
    if let Some(s) = std::env::var("DISPATCH_INTERNAL_SECRET")
        .ok()
        .filter(|s| !s.trim().is_empty())
    {
        return Some(s);
    }
    #[cfg(debug_assertions)]
    {
        tracing::warn!(
            target: "api",
            "DISPATCH_INTERNAL_SECRET unset; using debug-build default for /internal/dispatch/* (header X-Internal-Secret)"
        );
        Some(DEBUG_DISPATCH_INTERNAL_SECRET.to_string())
    }
    #[cfg(not(debug_assertions))]
    {
        None
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    observability::init_tracing();
    observability::init_metrics();

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

    let redis_dispatch = RedisDispatchHelper::from_env();
    let dispatch_advance_secret = dispatch_advance_secret_from_env();

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
        redis_dispatch,
        dispatch_advance_secret,
    };

    let (worker_joins, shutdown_tx) = if dispatch_queue_worker_enabled() {
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        let n = dispatch_queue_worker_concurrency();
        tracing::info!(
            target = "dispatch",
            worker_concurrency = n,
            "dispatch_queue_workers_spawn"
        );
        let mut joins = Vec::with_capacity(n);
        for _ in 0..n {
            let pool_worker = pool.clone();
            let redis_worker = state.redis_dispatch.clone();
            let rx = shutdown_rx.clone();
            joins.push(tokio::spawn(async move {
                run_dispatch_queue_worker(
                    pool_worker,
                    redis_worker,
                    MatcherConfig::default(),
                    rx,
                )
                .await;
            }));
        }
        (Some(joins), Some(shutdown_tx))
    } else {
        (None, None)
    };

    let mut app = Router::new()
        .route("/health", get(health))
        .route("/metrics", get(metrics))
        .nest("/auth", auth_routes(state.clone()))
        .nest("/orders", orders_routes(state.clone()))
        .nest("/internal/dispatch", dispatch_writer_routes())
        .with_state(state);

    if let Some(cors) = cors_layer_from_env() {
        app = app.layer(cors);
    }

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("failed to bind");

    println!("API running on http://0.0.0.0:3001");

    let shutdown_tx_for_signal = shutdown_tx.clone();
    let server = axum::serve(listener, app).with_graceful_shutdown(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for ctrl+c");
        if let Some(tx) = shutdown_tx_for_signal {
            tx.send_replace(true);
        }
    });

    let server_result = server.await;

    if let Some(joins) = worker_joins {
        for h in joins {
            let _ = tokio::time::timeout(Duration::from_secs(30), h).await;
        }
    }

    server_result.expect("server failed");
}
