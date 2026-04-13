use axum::middleware;
use axum::routing::post;
use axum::Router;

use crate::modules::users::Role;
use crate::AppState;

use crate::modules::auth::middleware_access::require_access_token;

use super::handler;

pub fn orders_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create_order))
        .layer(crate::require_role!(Role::Client))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_access_token,
        ))
}
