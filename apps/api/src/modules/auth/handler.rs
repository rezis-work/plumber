use axum::{extract::State, http::StatusCode, Json};

use crate::AppState;

use super::dto::{
    RegisterClientRequest, RegisterClientResponse, RegisterPlumberRequest, RegisterPlumberResponse,
};
use super::register_error::RegisterError;
use super::service;

pub async fn register_client(
    State(state): State<AppState>,
    Json(body): Json<RegisterClientRequest>,
) -> Result<(StatusCode, Json<RegisterClientResponse>), RegisterError> {
    let res = service::register_client(&state, body).await?;
    Ok((StatusCode::CREATED, Json(res)))
}

pub async fn register_plumber(
    State(state): State<AppState>,
    Json(body): Json<RegisterPlumberRequest>,
) -> Result<(StatusCode, Json<RegisterPlumberResponse>), RegisterError> {
    let res = service::register_plumber(&state, body).await?;
    Ok((StatusCode::CREATED, Json(res)))
}
