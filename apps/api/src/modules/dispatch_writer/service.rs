use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::dispatch_matcher::{match_plumbers, MatcherConfig, MatcherOrderInput};
use crate::modules::observability;
use crate::modules::domain_enums::{DispatchStatus, OrderStatus, OrderUrgency};
use crate::modules::order_dispatches::OrderDispatchRepository;
use crate::modules::orders::OrderRepository;

use super::redis::RedisDispatchHelper;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdvanceDispatchOutcome {
    Success {
        offer_round: i16,
        inserted_plumber_ids: Vec<Uuid>,
        new_dispatch_ids: Vec<Uuid>,
    },
    SkippedOrderNotFound,
    SkippedNotDispatchable,
    SkippedNoPlumbers,
    SkippedLockNotAcquired,
}

#[derive(Debug, thiserror::Error)]
pub enum AdvanceDispatchError {
    #[error(transparent)]
    Sql(#[from] sqlx::Error),
    #[error(transparent)]
    Redis(#[from] super::redis::DispatchRedisError),
}

fn is_dispatchable(status: OrderStatus) -> bool {
    matches!(
        status,
        OrderStatus::Searching | OrderStatus::Dispatched
    )
}

fn offer_ttl_secs(urgency: OrderUrgency, config: &MatcherConfig) -> u64 {
    let mins = match urgency {
        OrderUrgency::Emergency => config.dispatch_offer_ttl_minutes_emergency,
        OrderUrgency::Normal | OrderUrgency::Urgent => {
            config.dispatch_offer_ttl_minutes_normal
        }
    };
    mins.saturating_mul(60)
}

/// OD-3: lock order row, next `offer_round`, matcher on same txn, insert `order_dispatches`, optional Redis.
pub async fn advance_dispatch_round(
    pool: &PgPool,
    order_id: Uuid,
    matcher_config: &MatcherConfig,
    redis: Option<&RedisDispatchHelper>,
) -> Result<AdvanceDispatchOutcome, AdvanceDispatchError> {
    let mut tx = pool.begin().await?;

    let Some(order) = OrderRepository::find_by_id_for_update_tx(&mut tx, order_id).await? else {
        tx.commit().await?;
        return Ok(AdvanceDispatchOutcome::SkippedOrderNotFound);
    };

    if !is_dispatchable(order.status) {
        tx.commit().await?;
        return Ok(AdvanceDispatchOutcome::SkippedNotDispatchable);
    }

    let prev_max = OrderDispatchRepository::max_offer_round_tx(&mut tx, order_id).await?;
    let next_round = prev_max.saturating_add(1);

    let mut lock_held = false;
    if let Some(r) = redis {
        if !r.try_acquire_order_lock(order_id).await? {
            tx.rollback().await?;
            return Ok(AdvanceDispatchOutcome::SkippedLockNotAcquired);
        }
        lock_held = true;
    }

    let input = MatcherOrderInput {
        order_id,
        service_category_id: order.service_category_id,
        city_id: order.city_id,
        area_id: order.area_id,
        lat: order.lat,
        lng: order.lng,
        urgency: order.urgency,
    };

    let plumbers = match match_plumbers(&mut *tx, &input, matcher_config).await {
        Ok(p) => p,
        Err(e) => {
            if lock_held {
                if let Some(r) = redis {
                    let _ = r.release_order_lock(order_id).await;
                }
            }
            tx.rollback().await?;
            return Err(AdvanceDispatchError::Sql(e));
        }
    };

    if plumbers.is_empty() {
        if lock_held {
            if let Some(r) = redis {
                let _ = r.release_order_lock(order_id).await;
            }
        }
        tx.commit().await?;
        return Ok(AdvanceDispatchOutcome::SkippedNoPlumbers);
    }

    let k = matcher_config.batch_size.min(plumbers.len());
    let mut new_dispatch_ids = Vec::with_capacity(k);

    for (i, plumber_id) in plumbers.iter().take(k).enumerate() {
        let rank = (i + 1) as i16;
        let row = match OrderDispatchRepository::insert_tx(
            &mut tx,
            order_id,
            *plumber_id,
            rank,
            next_round,
            DispatchStatus::Sent,
        )
        .await
        {
            Ok(row) => row,
            Err(e) => {
                if lock_held {
                    if let Some(r) = redis {
                        let _ = r.release_order_lock(order_id).await;
                    }
                }
                tx.rollback().await?;
                return Err(AdvanceDispatchError::Sql(e));
            }
        };
        new_dispatch_ids.push(row.id);
    }

    if order.status == OrderStatus::Searching {
        OrderRepository::set_status_dispatched_if_searching_tx(&mut tx, order_id).await?;
    }

    tx.commit().await?;

    if let Some(r) = redis {
        let ttl = offer_ttl_secs(order.urgency, matcher_config);
        for dispatch_id in &new_dispatch_ids {
            if let Err(e) = r.set_dispatch_deadline(*dispatch_id, ttl).await {
                tracing::warn!(
                    error = %e,
                    %dispatch_id,
                    "redis set_dispatch_deadline failed (DB already committed)"
                );
            }
        }
        if let Err(e) = r.release_order_lock(order_id).await {
            tracing::warn!(error = %e, %order_id, "redis release_order_lock failed");
        }
    }

    let first_plumber = plumbers.iter().take(k).next().copied();
    if next_round == 1 {
        let secs = (Utc::now() - order.requested_at).num_milliseconds() as f64 / 1000.0;
        observability::record_time_to_first_offer_seconds(secs);
    }
    observability::log_order_transition(
        order_id,
        "dispatch_round_advanced",
        Some(next_round),
        first_plumber,
    );

    Ok(AdvanceDispatchOutcome::Success {
        offer_round: next_round,
        inserted_plumber_ids: plumbers.into_iter().take(k).collect(),
        new_dispatch_ids,
    })
}
