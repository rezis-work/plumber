//! Implementation 004 §12.6 / §7 — reclaim stuck outbox leases and nudge orphan bootstrap orders.

use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::dispatch_matcher::MatcherConfig;
use crate::modules::dispatch_outbox::DispatchOutboxRepository;

use super::redis::RedisDispatchHelper;
use super::service::{advance_dispatch_round, AdvanceDispatchError};

const DEFAULT_ORPHAN_MIN_AGE_SECS: i64 = 60;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReconcileSummary {
    /// Outbox rows returned to `pending` after an expired lease (§12.6).
    pub requeued_leases: u64,
    /// Outbox rows marked `failed` due to `DISPATCH_OUTBOX_MAX_ATTEMPTS` (§12.7).
    pub failed_max_attempts: u64,
    pub orphans_found: usize,
    pub rpush_ok: usize,
    pub advance_direct: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum ReconcileError {
    #[error(transparent)]
    Sql(#[from] sqlx::Error),
    #[error(transparent)]
    Advance(#[from] AdvanceDispatchError),
    #[error(transparent)]
    Redis(#[from] super::redis::DispatchRedisError),
}

pub(crate) fn orphan_min_age_secs_from_env() -> i64 {
    std::env::var("DISPATCH_RECONCILE_ORPHAN_MIN_AGE_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_ORPHAN_MIN_AGE_SECS)
        .max(0)
}

/// Unset or **`0`** = no cap on lease-reclaim retries.
pub(crate) fn dispatch_outbox_max_attempts_from_env() -> Option<i32> {
    std::env::var("DISPATCH_OUTBOX_MAX_ATTEMPTS")
        .ok()
        .and_then(|s| s.parse::<i32>().ok())
        .filter(|&n| n > 0)
}

/// Orders per §7: `searching`, no dispatches, pending bootstrap outbox at least `min_age_secs` old.
pub(crate) async fn list_orphan_bootstrap_order_ids(
    pool: &PgPool,
    min_age_secs: i64,
) -> Result<Vec<Uuid>, sqlx::Error> {
    let rows: Vec<(Uuid,)> = sqlx::query_as(
        r#"
        SELECT o.id
        FROM orders o
        WHERE o.status = 'searching'
          AND NOT EXISTS (SELECT 1 FROM order_dispatches d WHERE d.order_id = o.id)
          AND EXISTS (
              SELECT 1
              FROM dispatch_outbox x
              WHERE x.order_id = o.id
                AND x.job_kind = 'bootstrap_first_round'
                AND x.status = 'pending'
                AND x.created_at + ($1::bigint * INTERVAL '1 second') <= NOW()
          )
        "#,
    )
    .bind(min_age_secs)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

/// Requeue expired `processing` leases, then wake or advance orphan bootstrap orders (§7).
pub async fn reconcile_stale_outbox(
    pool: &PgPool,
    redis: Option<&RedisDispatchHelper>,
    matcher_config: &MatcherConfig,
) -> Result<ReconcileSummary, ReconcileError> {
    let repo = DispatchOutboxRepository::new(pool.clone());
    let max_attempts = dispatch_outbox_max_attempts_from_env();
    let lease = repo.requeue_expired_leases(max_attempts).await?;

    let min_age = orphan_min_age_secs_from_env();
    let orphan_ids = list_orphan_bootstrap_order_ids(pool, min_age).await?;
    let orphans_found = orphan_ids.len();

    let mut summary = ReconcileSummary {
        requeued_leases: lease.requeued_pending,
        failed_max_attempts: lease.failed_max_attempts,
        orphans_found,
        ..Default::default()
    };

    if let Some(r) = redis {
        for order_id in orphan_ids {
            match r.rpush_dispatch_queue(order_id).await {
                Ok(()) => summary.rpush_ok += 1,
                Err(e) => {
                    tracing::warn!(
                        target = "dispatch",
                        error = %e,
                        %order_id,
                        "reconcile_stale_outbox: RPUSH failed"
                    );
                }
            }
        }
    } else {
        for order_id in orphan_ids {
            advance_dispatch_round(pool, order_id, matcher_config, None).await?;
            summary.advance_direct += 1;
        }
    }

    let pending: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM dispatch_outbox WHERE status = 'pending'",
    )
    .fetch_one(pool)
    .await?;
    crate::modules::observability::set_dispatch_outbox_pending(pending as u64);

    Ok(summary)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::modules::dispatch_matcher::MatcherConfig;
    use crate::modules::dispatch_outbox::DispatchOutboxRepository;
    use crate::modules::domain_enums::{DispatchOutboxStatus, OrderStatus, OrderUrgency};

    use super::{list_orphan_bootstrap_order_ids, reconcile_stale_outbox};

    async fn seed_order_with_outbox(pool: &PgPool) -> Uuid {
        let slug = format!("rec-city-{}", Uuid::new_v4());
        let city_id: Uuid = sqlx::query_scalar(
            r#"INSERT INTO cities (name, slug, is_active) VALUES ('R', $1, true) RETURNING id"#,
        )
        .bind(&slug)
        .fetch_one(pool)
        .await
        .unwrap();

        let area_slug = format!("rec-area-{}", Uuid::new_v4());
        let area_id: Uuid = sqlx::query_scalar(
            r#"INSERT INTO areas (city_id, name, slug, is_active) VALUES ($1, 'A', $2, true) RETURNING id"#,
        )
        .bind(city_id)
        .bind(&area_slug)
        .fetch_one(pool)
        .await
        .unwrap();

        let cat_slug = format!("rec-cat-{}", Uuid::new_v4());
        let cat_id: Uuid = sqlx::query_scalar(
            r#"INSERT INTO service_categories (name, slug, is_active, sort_order) VALUES ('C', $1, true, 0) RETURNING id"#,
        )
        .bind(&cat_slug)
        .fetch_one(pool)
        .await
        .unwrap();

        let email = format!("rec-client-{}@test.local", Uuid::new_v4());
        let client_id: Uuid = sqlx::query_scalar(
            r#"INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
               VALUES ($1::citext, 'x', 'client', 'active', true) RETURNING id"#,
        )
        .bind(&email)
        .fetch_one(pool)
        .await
        .unwrap();

        let order_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO orders (
                client_id, service_category_id, city_id, area_id,
                address_line, lat, lng, description, urgency, status
            )
            VALUES ($1, $2, $3, $4, 'a', 41.7, 44.8, 'd', $5, $6)
            RETURNING id
            "#,
        )
        .bind(client_id)
        .bind(cat_id)
        .bind(city_id)
        .bind(Some(area_id))
        .bind(OrderUrgency::Normal)
        .bind(OrderStatus::Searching)
        .fetch_one(pool)
        .await
        .unwrap();

        let mut tx = pool.begin().await.unwrap();
        DispatchOutboxRepository::insert_pending_bootstrap_tx(&mut tx, order_id)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        order_id
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn reconcile_requeues_expired_lease(pool: PgPool) {
        let _order_id = seed_order_with_outbox(&pool).await;

        let claimed = DispatchOutboxRepository::try_claim_next_pending(&pool, 1)
            .await
            .unwrap()
            .unwrap();

        sqlx::query(
            r#"UPDATE dispatch_outbox SET lease_expires_at = NOW() - INTERVAL '1 minute' WHERE id = $1"#,
        )
        .bind(claimed.id)
        .execute(&pool)
        .await
        .unwrap();

        let s = reconcile_stale_outbox(&pool, None, &MatcherConfig::default())
            .await
            .unwrap();
        assert_eq!(s.requeued_leases, 1);
        assert_eq!(s.failed_max_attempts, 0);

        let (status, attempts): (DispatchOutboxStatus, i32) = sqlx::query_as(
            r#"SELECT status, attempt_count FROM dispatch_outbox WHERE id = $1"#,
        )
        .bind(claimed.id)
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(status, DispatchOutboxStatus::Pending);
        assert_eq!(attempts, 1);
    }

    async fn insert_plumber_rec(pool: &PgPool, email: &str, city_id: Uuid, cat_id: Uuid, area_id: Uuid) -> Uuid {
        use chrono::Utc;
        let user_id: Uuid = sqlx::query_scalar(
            r#"INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
               VALUES ($1::citext, 'x', 'plumber', 'active', true) RETURNING id"#,
        )
        .bind(email)
        .fetch_one(pool)
        .await
        .unwrap();

        let now = Utc::now();
        let plumber_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO plumber_profiles (
                user_id, full_name, phone, experience_years,
                is_approved, is_online, is_available,
                current_lat, current_lng, last_location_updated_at,
                service_radius_km, token_balance, rating_avg
            )
            VALUES ($1, 'P', '1', 1, true, true, true, 41.7, 44.8, $2, 50, 20, 4.0)
            RETURNING id
            "#,
        )
        .bind(user_id)
        .bind(now)
        .fetch_one(pool)
        .await
        .unwrap();

        sqlx::query(r#"INSERT INTO plumber_services (plumber_id, service_category_id) VALUES ($1, $2)"#)
            .bind(plumber_id)
            .bind(cat_id)
            .execute(pool)
            .await
            .unwrap();

        sqlx::query(
            r#"INSERT INTO plumber_service_areas (plumber_id, city_id, area_id) VALUES ($1, $2, $3)"#,
        )
        .bind(plumber_id)
        .bind(city_id)
        .bind(Some(area_id))
        .execute(pool)
        .await
        .unwrap();

        plumber_id
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn reconcile_orphan_db_only_advances(pool: PgPool) {
        let slug = format!("rec2-city-{}", Uuid::new_v4());
        let city_id: Uuid = sqlx::query_scalar(
            r#"INSERT INTO cities (name, slug, is_active) VALUES ('R2', $1, true) RETURNING id"#,
        )
        .bind(&slug)
        .fetch_one(&pool)
        .await
        .unwrap();

        let area_slug = format!("rec2-area-{}", Uuid::new_v4());
        let area_id: Uuid = sqlx::query_scalar(
            r#"INSERT INTO areas (city_id, name, slug, is_active) VALUES ($1, 'A', $2, true) RETURNING id"#,
        )
        .bind(city_id)
        .bind(&area_slug)
        .fetch_one(&pool)
        .await
        .unwrap();

        let cat_slug = format!("rec2-cat-{}", Uuid::new_v4());
        let cat_id: Uuid = sqlx::query_scalar(
            r#"INSERT INTO service_categories (name, slug, is_active, sort_order) VALUES ('C', $1, true, 0) RETURNING id"#,
        )
        .bind(&cat_slug)
        .fetch_one(&pool)
        .await
        .unwrap();

        for i in 1..=3 {
            insert_plumber_rec(
                &pool,
                &format!("rec2-p{i}@test.local"),
                city_id,
                cat_id,
                area_id,
            )
            .await;
        }

        let email = format!("rec2-client-{}@test.local", Uuid::new_v4());
        let client_id: Uuid = sqlx::query_scalar(
            r#"INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
               VALUES ($1::citext, 'x', 'client', 'active', true) RETURNING id"#,
        )
        .bind(&email)
        .fetch_one(&pool)
        .await
        .unwrap();

        let order_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO orders (
                client_id, service_category_id, city_id, area_id,
                address_line, lat, lng, description, urgency, status
            )
            VALUES ($1, $2, $3, $4, 'a', 41.7, 44.8, 'd', $5, $6)
            RETURNING id
            "#,
        )
        .bind(client_id)
        .bind(cat_id)
        .bind(city_id)
        .bind(Some(area_id))
        .bind(OrderUrgency::Normal)
        .bind(OrderStatus::Searching)
        .fetch_one(&pool)
        .await
        .unwrap();

        let mut tx = pool.begin().await.unwrap();
        DispatchOutboxRepository::insert_pending_bootstrap_tx(&mut tx, order_id)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        sqlx::query(
            r#"UPDATE dispatch_outbox SET created_at = NOW() - INTERVAL '5 minutes' WHERE order_id = $1 AND status = 'pending'"#,
        )
        .bind(order_id)
        .execute(&pool)
        .await
        .unwrap();

        let min_age = super::orphan_min_age_secs_from_env();
        let orphans = list_orphan_bootstrap_order_ids(&pool, min_age).await.unwrap();
        assert!(orphans.contains(&order_id));

        let s = reconcile_stale_outbox(&pool, None, &MatcherConfig::default())
            .await
            .unwrap();
        assert_eq!(s.orphans_found, 1);
        assert_eq!(s.advance_direct, 1);

        let n: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*)::bigint FROM order_dispatches WHERE order_id = $1"#,
        )
        .bind(order_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(n, 3);
    }
}
