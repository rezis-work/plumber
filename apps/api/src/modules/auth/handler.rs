use axum::http::header::{self, HeaderValue};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;

use crate::AppState;

use super::auth_user::AuthUser;
use super::dto::{
    LoginRequest, MeResponse, RegisterClientRequest, RegisterClientResponse, RegisterPlumberRequest,
    RegisterPlumberResponse,
};
use super::login_error::LoginError;
use super::refresh_error::RefreshError;
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

pub async fn refresh(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Response, RefreshError> {
    let Some(cookie_raw) = headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
    else {
        return Err(RefreshError::Unauthorized);
    };
    let Some(raw_refresh_jwt) = state.cookie_config.refresh_from_cookie_header(cookie_raw) else {
        return Err(RefreshError::Unauthorized);
    };

    let service::LoginSuccess {
        response,
        refresh_jwt,
    } = service::refresh(&state, &raw_refresh_jwt).await?;

    let cookie_str = state
        .cookie_config
        .refresh_set_cookie_string(&refresh_jwt, state.jwt_config.refresh_ttl_secs())
        .map_err(|_| RefreshError::Internal)?;

    let cookie_header = HeaderValue::from_str(&cookie_str).map_err(|_| RefreshError::Internal)?;

    let mut res = (StatusCode::OK, Json(response)).into_response();
    res.headers_mut().append(header::SET_COOKIE, cookie_header);
    Ok(res)
}

pub async fn me(AuthUser(ctx): AuthUser) -> Json<MeResponse> {
    Json(MeResponse {
        user_id: ctx.user_id,
        role: ctx.role,
    })
}

/// Step 8 RBAC verification stub; replace with a real plumber-only route when domains land.
pub async fn rbac_plumber_check() -> Json<serde_json::Value> {
    Json(json!({ "ok": true }))
}

/// Step 8 RBAC verification stub.
pub async fn rbac_admin_check() -> Json<serde_json::Value> {
    Json(json!({ "ok": true }))
}

/// Step 8 `require_any_role` verification stub.
pub async fn rbac_staff_check() -> Json<serde_json::Value> {
    Json(json!({ "ok": true }))
}
