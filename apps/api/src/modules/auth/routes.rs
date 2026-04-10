use axum::routing::post;
use axum::Router;

use crate::AppState;

use super::handler;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register/client", post(handler::register_client))
        .route("/register/plumber", post(handler::register_plumber))
}
