use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::modules::auth::auth_user::AuthUser;
use crate::AppState;

use super::create_order_error::CreateOrderError;
use super::dto::{CreateOrderRequest, CreateOrderResponse};
use super::service;

pub async fn create_order(
    State(state): State<AppState>,
    AuthUser(auth): AuthUser,
    Json(body): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<CreateOrderResponse>), CreateOrderError> {
    let res = service::create_order(&state, auth.user_id, body).await?;
    Ok((StatusCode::CREATED, Json(res)))
}
