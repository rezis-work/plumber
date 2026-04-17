use axum::middleware;
use axum::routing::post;
use axum::Router;

use crate::modules::users::Role;
use crate::AppState;

use crate::modules::auth::middleware_access::require_access_token;

use super::dispatch_handler;
use super::handler;
use super::order_lifecycle_handler;

pub fn orders_routes(state: AppState) -> Router<AppState> {
    let client = Router::new()
        .route("/", post(handler::create_order))
        .route(
            "/{order_id}/complete",
            post(order_lifecycle_handler::complete_order),
        )
        .layer(crate::require_role!(Role::Client))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_access_token,
        ));

    let plumber = Router::new()
        .route(
            "/{order_id}/start",
            post(order_lifecycle_handler::start_order),
        )
        .route(
            "/{order_id}/dispatches/{dispatch_id}/accept",
            post(dispatch_handler::accept_dispatch),
        )
        .route(
            "/{order_id}/dispatches/{dispatch_id}/reject",
            post(dispatch_handler::reject_dispatch),
        )
        .layer(crate::require_role!(Role::Plumber))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_access_token,
        ));

    Router::new().merge(client).merge(plumber)
}
