//! In-process dispatch queue consumer (Implementation 004 §12.5 Option A).
//! Upstash REST cannot use blocking `BRPOP`; we use `LPOP` + idle sleep instead.

use std::time::Duration;

use sqlx::PgPool;
use tokio::sync::watch;

use crate::modules::dispatch_matcher::MatcherConfig;
use crate::modules::dispatch_outbox::{DispatchOutbox, DispatchOutboxRepository};
use crate::modules::dispatch_writer::redis::RedisDispatchHelper;
use crate::modules::dispatch_writer::service::{
    advance_dispatch_outcome_label, advance_dispatch_round, AdvanceDispatchOutcome,
};

const IDLE_SLEEP_SECS: u64 = 5;
const DEFAULT_LEASE_SECS: i64 = 120;

/// `DISPATCH_QUEUE_WORKER_ENABLED`: `1`, `true`, `yes` (case-insensitive).
pub fn dispatch_queue_worker_enabled() -> bool {
    std::env::var("DISPATCH_QUEUE_WORKER_ENABLED")
        .ok()
        .map(|s| {
            let s = s.trim();
            s == "1"
                || s.eq_ignore_ascii_case("true")
                || s.eq_ignore_ascii_case("yes")
        })
        .unwrap_or(false)
}

/// §12.7: in-process worker task count (default **1**, clamped **1..=8**).
pub fn dispatch_queue_worker_concurrency() -> usize {
    std::env::var("DISPATCH_QUEUE_WORKER_CONCURRENCY")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1)
        .clamp(1, 8)
}

/// §12.7: **`DISPATCH_OUTBOX_LEASE_SECS`** first, then legacy **`DISPATCH_WORKER_LEASE_SECS`**, default 120.
pub(crate) fn dispatch_outbox_lease_secs_from_env() -> i64 {
    let from_outbox = std::env::var("DISPATCH_OUTBOX_LEASE_SECS")
        .ok()
        .and_then(|s| s.parse::<i64>().ok());
    let from_worker = std::env::var("DISPATCH_WORKER_LEASE_SECS")
        .ok()
        .and_then(|s| s.parse::<i64>().ok());
    from_outbox
        .or(from_worker)
        .unwrap_or(DEFAULT_LEASE_SECS)
        .max(1)
}

pub async fn run_dispatch_queue_worker(
    pool: PgPool,
    redis: Option<RedisDispatchHelper>,
    matcher_config: MatcherConfig,
    shutdown: watch::Receiver<bool>,
) {
    let lease_secs = dispatch_outbox_lease_secs_from_env();
    tracing::info!(
        target = "dispatch",
        lease_secs,
        redis_enabled = redis.is_some(),
        "dispatch_queue_worker started"
    );

    loop {
        if *shutdown.borrow() {
            break;
        }

        let mut claimed: Option<DispatchOutbox> = None;

        if let Some(ref r) = redis {
            match r.lpop_dispatch_queue().await {
                Ok(Some(order_id)) => {
                    match DispatchOutboxRepository::try_claim_pending_for_order(
                        &pool, order_id, lease_secs,
                    )
                    .await
                    {
                        Ok(row) => {
                            if row.is_none() {
                                tracing::warn!(
                                    target = "dispatch",
                                    %order_id,
                                    "dispatch_worker: queue wake but no pending outbox row"
                                );
                                tokio::time::sleep(Duration::from_millis(200)).await;
                            }
                            claimed = row;
                        }
                        Err(e) => {
                            tracing::error!(
                                target = "dispatch",
                                error = %e,
                                %order_id,
                                "dispatch_worker: try_claim_pending_for_order failed"
                            );
                        }
                    }
                }
                Ok(None) => {}
                Err(e) => {
                    tracing::warn!(
                        target = "dispatch",
                        error = %e,
                        "dispatch_worker: LPOP dispatch:queue failed"
                    );
                }
            }
        }

        if claimed.is_none() && !*shutdown.borrow() {
            match DispatchOutboxRepository::try_claim_next_pending(&pool, lease_secs).await {
                Ok(row) => claimed = row,
                Err(e) => tracing::error!(
                    target = "dispatch",
                    error = %e,
                    "dispatch_worker: try_claim_next_pending failed"
                ),
            }
        }

        if let Some(row) = claimed {
            process_outbox_job(&pool, redis.as_ref(), &matcher_config, row).await;
        } else if !*shutdown.borrow() {
            tokio::time::sleep(Duration::from_secs(IDLE_SLEEP_SECS)).await;
        }
    }

    tracing::info!(target = "dispatch", "dispatch_queue_worker stopped");
}

async fn process_outbox_job(
    pool: &PgPool,
    redis: Option<&RedisDispatchHelper>,
    matcher_config: &MatcherConfig,
    row: DispatchOutbox,
) {
    let outbox_id = row.id;
    let order_id = row.order_id;

    let count: Result<i64, sqlx::Error> = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM order_dispatches WHERE order_id = $1",
    )
    .bind(order_id)
    .fetch_one(pool)
    .await;

    let count = match count {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(
                target = "dispatch",
                error = %e,
                %order_id,
                "dispatch_worker: count order_dispatches failed"
            );
            if let Ok(mut tx) = pool.begin().await {
                let _ = DispatchOutboxRepository::mark_failed_tx(
                    &mut tx,
                    outbox_id,
                    &format!("count order_dispatches: {e}"),
                )
                .await;
                let _ = tx.commit().await;
            }
            tracing::info!(
                target = "dispatch",
                %order_id,
                %outbox_id,
                job_kind = ?row.job_kind,
                outcome = "error_count_dispatches",
                "dispatch_worker_job"
            );
            return;
        }
    };

    if count > 0 {
        // Idempotent duplicate queue delivery after dispatches already exist (§12.5 guard).
        let Ok(mut tx) = pool.begin().await else {
            tracing::error!(target = "dispatch", %outbox_id, "dispatch_worker: begin tx for mark_done (dup guard)");
            return;
        };
        let _ = DispatchOutboxRepository::mark_done_tx(&mut tx, outbox_id).await;
        let _ = tx.commit().await;
        tracing::info!(
            target = "dispatch",
            %order_id,
            %outbox_id,
            job_kind = ?row.job_kind,
            outcome = "idempotent_duplicate_dispatches",
            "dispatch_worker_job"
        );
        return;
    }

    // v1: only bootstrap job kind exists; future kinds can branch here.
    let advance_result =
        advance_dispatch_round(pool, order_id, matcher_config, redis).await;

    let Ok(mut tx) = pool.begin().await else {
        tracing::error!(target = "dispatch", %outbox_id, "dispatch_worker: begin tx after advance");
        return;
    };

    match &advance_result {
        Ok(
            outcome @ (AdvanceDispatchOutcome::Success { .. }
            | AdvanceDispatchOutcome::SkippedNoPlumbers
            | AdvanceDispatchOutcome::SkippedNotDispatchable
            | AdvanceDispatchOutcome::SkippedOrderNotFound),
        ) => {
            let _ = DispatchOutboxRepository::mark_done_tx(&mut tx, outbox_id).await;
            let _ = tx.commit().await;
            tracing::info!(
                target = "dispatch",
                %order_id,
                %outbox_id,
                job_kind = ?row.job_kind,
                outcome = advance_dispatch_outcome_label(outcome),
                "dispatch_worker_job"
            );
        }
        Ok(outcome @ AdvanceDispatchOutcome::SkippedLockNotAcquired) => {
            let _ = DispatchOutboxRepository::release_claim_to_pending_tx(&mut tx, outbox_id).await;
            let _ = tx.commit().await;
            tracing::info!(
                target = "dispatch",
                %order_id,
                %outbox_id,
                job_kind = ?row.job_kind,
                outcome = advance_dispatch_outcome_label(outcome),
                "dispatch_worker_job"
            );
        }
        Err(e) => {
            // v1: surface operator-visible failure (no automatic requeue with backoff).
            let _ = DispatchOutboxRepository::mark_failed_tx(&mut tx, outbox_id, &e.to_string()).await;
            let _ = tx.commit().await;
            tracing::info!(
                target = "dispatch",
                %order_id,
                %outbox_id,
                job_kind = ?row.job_kind,
                outcome = "error_advance_dispatch",
                error = %e,
                "dispatch_worker_job"
            );
        }
    }
}
