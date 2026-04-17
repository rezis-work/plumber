use axum::extract::{Path, State};
use axum::Json;
use serde::Serialize;
use uuid::Uuid;

use crate::modules::auth::auth_user::AuthUser;
use crate::AppState;

use super::dispatch_error::DispatchActionError;
use super::dispatch_service;

#[derive(Serialize)]
pub struct AcceptDispatchResponse {
    pub order_id: Uuid,
    pub status: &'static str,
}

#[derive(Serialize)]
pub struct RejectDispatchResponse {
    pub dispatch_id: Uuid,
    pub status: &'static str,
}

pub async fn accept_dispatch(
    State(state): State<AppState>,
    AuthUser(auth): AuthUser,
    Path((order_id, dispatch_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<AcceptDispatchResponse>, DispatchActionError> {
    dispatch_service::accept_dispatch(
        &state.pool,
        auth.user_id,
        order_id,
        dispatch_id,
        &state.users,
    )
    .await?;

    Ok(Json(AcceptDispatchResponse {
        order_id,
        status: "accepted",
    }))
}

pub async fn reject_dispatch(
    State(state): State<AppState>,
    AuthUser(auth): AuthUser,
    Path((order_id, dispatch_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<RejectDispatchResponse>, DispatchActionError> {
    dispatch_service::reject_dispatch(&state, auth.user_id, order_id, dispatch_id).await?;

    Ok(Json(RejectDispatchResponse {
        dispatch_id,
        status: "rejected",
    }))
}
