use axum::{routing::get, Router};

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("failed to bind");

    println!("API running on http://0.0.0.0:3001");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}
