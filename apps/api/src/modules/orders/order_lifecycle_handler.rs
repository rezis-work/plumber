use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use uuid::Uuid;

use crate::modules::auth::auth_user::AuthUser;
use crate::AppState;

use super::completion_error::CompletionError;
use super::completion_service::{self, CompleteOutcome};

#[derive(Serialize)]
pub struct StartOrderResponse {
    pub order_id: Uuid,
    pub status: &'static str,
}

#[derive(Serialize)]
pub struct CompleteOrderResponse {
    pub order_id: Uuid,
    pub status: &'static str,
    pub tokens_credited: i32,
}

pub async fn start_order(
    State(state): State<AppState>,
    AuthUser(auth): AuthUser,
    Path(order_id): Path<Uuid>,
) -> Result<(StatusCode, Json<StartOrderResponse>), CompletionError> {
    completion_service::start_order(&state.pool, auth.user_id, order_id, &state.users).await?;
    Ok((
        StatusCode::OK,
        Json(StartOrderResponse {
            order_id,
            status: "in_progress",
        }),
    ))
}

pub async fn complete_order(
    State(state): State<AppState>,
    AuthUser(auth): AuthUser,
    Path(order_id): Path<Uuid>,
) -> Result<Json<CompleteOrderResponse>, CompletionError> {
    match completion_service::complete_order(&state, auth.user_id, order_id).await? {
        CompleteOutcome::Completed {
            order_id,
            tokens_credited,
            ..
        } => Ok(Json(CompleteOrderResponse {
            order_id,
            status: "completed",
            tokens_credited,
        })),
        CompleteOutcome::AlreadyCompleted { order_id } => Ok(Json(CompleteOrderResponse {
            order_id,
            status: "already_completed",
            tokens_credited: 0,
        })),
    }
}
