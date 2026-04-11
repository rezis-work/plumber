use axum::http::header::{self, HeaderValue};
use axum::response::{IntoResponse, Response};
use axum::{extract::State, http::StatusCode, Json};

use crate::AppState;

use super::dto::{
    LoginRequest, RegisterClientRequest, RegisterClientResponse, RegisterPlumberRequest,
    RegisterPlumberResponse,
};
use super::login_error::LoginError;
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

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Response, LoginError> {
    let service::LoginSuccess {
        response,
        refresh_jwt,
    } = service::login(&state, body).await?;

    let cookie_str = state
        .cookie_config
        .refresh_set_cookie_string(&refresh_jwt, state.jwt_config.refresh_ttl_secs())
        .map_err(|_| LoginError::Internal)?;

    let cookie_header = HeaderValue::from_str(&cookie_str).map_err(|_| LoginError::Internal)?;

    let mut res = (StatusCode::OK, Json(response)).into_response();
    res.headers_mut().append(header::SET_COOKIE, cookie_header);
    Ok(res)
}
