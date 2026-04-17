use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use serde::Serialize;
use uuid::Uuid;

use crate::modules::dispatch_matcher::MatcherConfig;
use crate::AppState;

use super::expire_job::run_dispatch_expiry_tick;
use super::service::{advance_dispatch_round, AdvanceDispatchOutcome};

fn constant_time_secret_eq(got: &str, expected: &str) -> bool {
    let a = got.as_bytes();
    let b = expected.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    subtle::ConstantTimeEq::ct_eq(a, b).into()
}

#[derive(serde::Deserialize)]
pub struct AdvanceRequest {
    pub order_id: Uuid,
}

#[derive(Serialize)]
pub struct AdvanceResponse {
    pub outcome: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_round: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inserted_plumber_ids: Option<Vec<Uuid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_dispatch_ids: Option<Vec<Uuid>>,
}

pub async fn post_advance(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<AdvanceRequest>,
) -> Result<(StatusCode, Json<AdvanceResponse>), StatusCode> {
    let Some(expected) = state.dispatch_advance_secret.as_ref() else {
        return Err(StatusCode::NOT_FOUND);
    };
    let ok = headers
        .get(axum::http::header::HeaderName::from_static("x-internal-secret"))
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| constant_time_secret_eq(v, expected));
    if !ok {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let matcher_config = MatcherConfig::default();
    let outcome = advance_dispatch_round(
        &state.pool,
        body.order_id,
        &matcher_config,
        state.redis_dispatch.as_ref(),
    )
    .await
    .map_err(|e| {
        tracing::error!("advance_dispatch_round: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let (status, body) = match outcome {
        AdvanceDispatchOutcome::Success {
            offer_round,
            inserted_plumber_ids,
            new_dispatch_ids,
        } => (
            StatusCode::OK,
            AdvanceResponse {
                outcome: "success",
                offer_round: Some(offer_round),
                inserted_plumber_ids: Some(inserted_plumber_ids),
                new_dispatch_ids: Some(new_dispatch_ids),
            },
        ),
        AdvanceDispatchOutcome::SkippedOrderNotFound => (
            StatusCode::NOT_FOUND,
            AdvanceResponse {
                outcome: "skipped_order_not_found",
                offer_round: None,
                inserted_plumber_ids: None,
                new_dispatch_ids: None,
            },
        ),
        AdvanceDispatchOutcome::SkippedNotDispatchable => (
            StatusCode::CONFLICT,
            AdvanceResponse {
                outcome: "skipped_not_dispatchable",
                offer_round: None,
                inserted_plumber_ids: None,
                new_dispatch_ids: None,
            },
        ),
        AdvanceDispatchOutcome::SkippedNoPlumbers => (
            StatusCode::OK,
            AdvanceResponse {
                outcome: "skipped_no_plumbers",
                offer_round: None,
                inserted_plumber_ids: None,
                new_dispatch_ids: None,
            },
        ),
        AdvanceDispatchOutcome::SkippedLockNotAcquired => (
            StatusCode::SERVICE_UNAVAILABLE,
            AdvanceResponse {
                outcome: "skipped_lock_not_acquired",
                offer_round: None,
                inserted_plumber_ids: None,
                new_dispatch_ids: None,
            },
        ),
    };

    Ok((status, Json(body)))
}

#[derive(Serialize)]
pub struct ExpireDueResponse {
    pub expired_count: usize,
    pub rounds_checked: usize,
    pub orders_advanced: usize,
    pub orders_expired: usize,
}

/// OD-5: expire overdue offers, advance rounds, expire orders when no plumbers remain.
/// Same auth as [`post_advance`]: `X-Internal-Secret` + `DISPATCH_INTERNAL_SECRET`.
pub async fn post_expire_due(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<ExpireDueResponse>), StatusCode> {
    let Some(expected) = state.dispatch_advance_secret.as_ref() else {
        return Err(StatusCode::NOT_FOUND);
    };
    let ok = headers
        .get(axum::http::header::HeaderName::from_static("x-internal-secret"))
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| constant_time_secret_eq(v, expected));
    if !ok {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let matcher_config = MatcherConfig::default();
    let summary = run_dispatch_expiry_tick(
        &state.pool,
        &matcher_config,
        state.redis_dispatch.as_ref(),
    )
    .await
    .map_err(|e| {
        tracing::error!("run_dispatch_expiry_tick: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((
        StatusCode::OK,
        Json(ExpireDueResponse {
            expired_count: summary.expired_count,
            rounds_checked: summary.rounds_checked,
            orders_advanced: summary.orders_advanced,
            orders_expired: summary.orders_expired,
        }),
    ))
}
