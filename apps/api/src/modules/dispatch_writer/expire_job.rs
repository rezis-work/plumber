//! OD-5: cron-style tick — expire overdue `sent`/`viewed` dispatches (TTL matches
//! [`MatcherConfig`] and Redis in [`super::service::offer_ttl_secs`]), then advance rounds
//! when a round has no active offers, and expire the parent order when the matcher has no one left.
//!
//! Open offers in a round are **`sent` or `viewed`**; advance only runs when neither remains
//! (stricter than spec §9’s `sent`-only wording, avoids starting round N+1 while a plumber still
//! has a live offer).

use std::collections::HashSet;

use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::dispatch_matcher::MatcherConfig;
use crate::modules::domain_enums::OrderStatus;
use crate::modules::observability;
use crate::modules::orders::OrderRepository;

use super::redis::RedisDispatchHelper;
use super::service::{advance_dispatch_round, AdvanceDispatchError, AdvanceDispatchOutcome};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExpireTickSummary {
    pub expired_count: usize,
    pub rounds_checked: usize,
    pub orders_advanced: usize,
    pub orders_expired: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum ExpireTickError {
    #[error(transparent)]
    Sql(#[from] sqlx::Error),
    #[error(transparent)]
    Advance(#[from] AdvanceDispatchError),
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct ExpiredRoundHint {
    order_id: Uuid,
    offer_round: i16,
}

/// Expire overdue dispatches, then for each affected `(order_id, offer_round)` try
/// [`advance_dispatch_round`] when the round has no `sent`/`viewed` rows left.
pub async fn run_dispatch_expiry_tick(
    pool: &PgPool,
    matcher_config: &MatcherConfig,
    redis: Option<&RedisDispatchHelper>,
) -> Result<ExpireTickSummary, ExpireTickError> {
    let normal_mins = i32::try_from(matcher_config.dispatch_offer_ttl_minutes_normal)
        .unwrap_or(i32::MAX);
    let emergency_mins = i32::try_from(matcher_config.dispatch_offer_ttl_minutes_emergency)
        .unwrap_or(i32::MAX);

    let expired_rows: Vec<ExpiredRoundHint> = sqlx::query_as(
        r#"
        UPDATE order_dispatches od
        SET
            status = 'expired',
            responded_at = COALESCE(od.responded_at, NOW())
        FROM orders o
        WHERE od.order_id = o.id
          AND od.status IN ('sent', 'viewed')
          AND od.sent_at
              + (
                  INTERVAL '1 minute'
                  * (
                      CASE o.urgency
                          WHEN 'emergency' THEN $2::integer
                          ELSE $1::integer
                      END
                  )
              )
              < NOW()
        RETURNING od.order_id, od.offer_round
        "#,
    )
    .bind(normal_mins)
    .bind(emergency_mins)
    .fetch_all(pool)
    .await?;

    let expired_count = expired_rows.len();
    observability::log_expire_tick(expired_count);
    let mut pairs: HashSet<(Uuid, i16)> = HashSet::new();
    for r in &expired_rows {
        pairs.insert((r.order_id, r.offer_round));
    }

    let orders_repo = OrderRepository::new(pool.clone());
    let mut summary = ExpireTickSummary {
        expired_count,
        ..Default::default()
    };

    for (order_id, offer_round) in pairs {
        summary.rounds_checked += 1;

        let Some(order) = orders_repo.find_by_id(order_id).await? else {
            continue;
        };
        if !matches!(
            order.status,
            OrderStatus::Searching | OrderStatus::Dispatched
        ) {
            continue;
        }

        let active_in_round: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)::bigint
            FROM order_dispatches
            WHERE order_id = $1
              AND offer_round = $2
              AND status IN ('sent', 'viewed')
            "#,
        )
        .bind(order_id)
        .bind(offer_round)
        .fetch_one(pool)
        .await?;

        if active_in_round > 0 {
            continue;
        }

        let outcome = advance_dispatch_round(pool, order_id, matcher_config, redis).await?;

        match outcome {
            AdvanceDispatchOutcome::Success { .. } => {
                summary.orders_advanced += 1;
                observability::log_order_transition(
                    order_id,
                    "advance_after_expiry",
                    Some(offer_round),
                    None,
                );
            }
            AdvanceDispatchOutcome::SkippedNoPlumbers => {
                let any_open: i64 = sqlx::query_scalar(
                    r#"
                    SELECT COUNT(*)::bigint
                    FROM order_dispatches
                    WHERE order_id = $1
                      AND status IN ('sent', 'viewed')
                    "#,
                )
                .bind(order_id)
                .fetch_one(pool)
                .await?;

                if any_open == 0 {
                    let res = sqlx::query(
                        r#"
                        UPDATE orders
                        SET status = 'expired', updated_at = NOW()
                        WHERE id = $1
                          AND status IN ('searching', 'dispatched')
                        "#,
                    )
                    .bind(order_id)
                    .execute(pool)
                    .await?;

                    if res.rows_affected() > 0 {
                        summary.orders_expired += 1;
                        observability::log_order_transition(order_id, "order_expired", None, None);
                    }
                }
            }
            AdvanceDispatchOutcome::SkippedLockNotAcquired => {
                tracing::warn!(
                    target = "dispatch",
                    %order_id,
                    %offer_round,
                    "expire_tick_advance_lock_busy"
                );
            }
            AdvanceDispatchOutcome::SkippedOrderNotFound
            | AdvanceDispatchOutcome::SkippedNotDispatchable => {}
        }
    }

    Ok(summary)
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::modules::dispatch_matcher::MatcherConfig;
    use crate::modules::domain_enums::{DispatchStatus, OrderStatus, OrderUrgency};
    use crate::modules::order_dispatches::OrderDispatchRepository;
    use crate::modules::orders::OrderRepository;

    use super::super::service::{advance_dispatch_round, AdvanceDispatchOutcome};
    use super::run_dispatch_expiry_tick;

    async fn seed_city_area_category(pool: &PgPool) -> (Uuid, Uuid, Uuid) {
        let city_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO cities (name, slug, is_active)
            VALUES ('Tbilisi', 'od5-tbilisi', true)
            RETURNING id
            "#,
        )
        .fetch_one(pool)
        .await
        .unwrap();

        let area_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO areas (city_id, name, slug, is_active)
            VALUES ($1, 'Vake', 'od5-vake', true)
            RETURNING id
            "#,
        )
        .bind(city_id)
        .fetch_one(pool)
        .await
        .unwrap();

        let cat_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO service_categories (name, slug, is_active, sort_order)
            VALUES ('Leak', 'od5-leak', true, 0)
            RETURNING id
            "#,
        )
        .fetch_one(pool)
        .await
        .unwrap();

        (city_id, area_id, cat_id)
    }

    async fn insert_plumber(pool: &PgPool, email: &str, city_id: Uuid, cat_id: Uuid, area_id: Uuid) -> Uuid {
        let user_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
            VALUES ($1::citext, 'x', 'plumber', 'active', true)
            RETURNING id
            "#,
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

        sqlx::query(
            r#"INSERT INTO plumber_services (plumber_id, service_category_id) VALUES ($1, $2)"#,
        )
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

    async fn insert_searching_order(
        pool: &PgPool,
        city_id: Uuid,
        area_id: Uuid,
        cat_id: Uuid,
        urgency: OrderUrgency,
    ) -> Uuid {
        let client_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
            VALUES ($1::citext, 'x', 'client', 'active', true)
            RETURNING id
            "#,
        )
        .bind(format!("od5-cl-{}@test.local", Uuid::new_v4()))
        .fetch_one(pool)
        .await
        .unwrap();

        sqlx::query_scalar(
            r#"
            INSERT INTO orders (
                client_id, service_category_id, city_id, area_id,
                address_line, lat, lng, description, urgency, status
            )
            VALUES ($1, $2, $3, $4, 'addr', 41.7, 44.8, 'leak', $5, $6)
            RETURNING id
            "#,
        )
        .bind(client_id)
        .bind(cat_id)
        .bind(city_id)
        .bind(Some(area_id))
        .bind(urgency)
        .bind(OrderStatus::Searching)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    async fn backdate_dispatches(pool: &PgPool, order_id: Uuid, minutes_ago: i64) {
        let t = Utc::now() - Duration::minutes(minutes_ago);
        sqlx::query(
            r#"
            UPDATE order_dispatches
            SET sent_at = $1
            WHERE order_id = $2
              AND status IN ('sent', 'viewed')
            "#,
        )
        .bind(t)
        .bind(order_id)
        .execute(pool)
        .await
        .unwrap();
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn expire_partial_round_does_not_advance(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_city_area_category(&pool).await;
        for i in 1..=3 {
            insert_plumber(
                &pool,
                &format!("od5-partial-p{i}@test.local"),
                city_id,
                cat_id,
                area_id,
            )
            .await;
        }
        let order_id = insert_searching_order(&pool, city_id, area_id, cat_id, OrderUrgency::Normal).await;
        let config = MatcherConfig::default();
        advance_dispatch_round(&pool, order_id, &config, None)
            .await
            .unwrap();

        let rows = OrderDispatchRepository::new(pool.clone())
            .list_by_order_id(order_id)
            .await
            .unwrap();
        assert_eq!(rows.len(), 3);
        let one_id = rows[0].id;
        sqlx::query(
            r#"UPDATE order_dispatches SET sent_at = $1 WHERE id = $2"#,
        )
        .bind(Utc::now() - Duration::minutes(120))
        .bind(one_id)
        .execute(&pool)
        .await
        .unwrap();

        let s = run_dispatch_expiry_tick(&pool, &config, None).await.unwrap();
        assert_eq!(s.expired_count, 1);
        assert_eq!(s.orders_advanced, 0);
        assert_eq!(s.orders_expired, 0);

        let after = OrderDispatchRepository::new(pool.clone())
            .list_by_order_id(order_id)
            .await
            .unwrap();
        assert!(after.iter().any(|r| r.id == one_id && r.status == DispatchStatus::Expired));
        assert_eq!(
            after
                .iter()
                .filter(|r| r.status == DispatchStatus::Sent)
                .count(),
            2
        );

        let order = OrderRepository::new(pool.clone())
            .find_by_id(order_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(order.status, OrderStatus::Dispatched);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn expire_full_round_advances_next(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_city_area_category(&pool).await;
        for i in 1..=6 {
            insert_plumber(
                &pool,
                &format!("od5-full-p{i}@test.local"),
                city_id,
                cat_id,
                area_id,
            )
            .await;
        }
        let order_id = insert_searching_order(&pool, city_id, area_id, cat_id, OrderUrgency::Normal).await;
        let config = MatcherConfig::default();
        advance_dispatch_round(&pool, order_id, &config, None)
            .await
            .unwrap();

        backdate_dispatches(&pool, order_id, 120).await;

        let s = run_dispatch_expiry_tick(&pool, &config, None).await.unwrap();
        assert_eq!(s.expired_count, 3);
        assert_eq!(s.orders_advanced, 1);
        assert_eq!(s.orders_expired, 0);

        let rows = OrderDispatchRepository::new(pool.clone())
            .list_by_order_id(order_id)
            .await
            .unwrap();
        assert_eq!(rows.len(), 6);
        assert_eq!(
            rows.iter()
                .filter(|r| r.offer_round == 2 && r.status == DispatchStatus::Sent)
                .count(),
            3
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn expire_then_no_plumbers_marks_order_expired(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_city_area_category(&pool).await;
        insert_plumber(&pool, "od5-one@test.local", city_id, cat_id, area_id).await;
        let order_id = insert_searching_order(&pool, city_id, area_id, cat_id, OrderUrgency::Normal).await;
        let config = MatcherConfig::default();
        let out = advance_dispatch_round(&pool, order_id, &config, None)
            .await
            .unwrap();
        assert!(matches!(out, AdvanceDispatchOutcome::Success { .. }));

        backdate_dispatches(&pool, order_id, 120).await;

        let s = run_dispatch_expiry_tick(&pool, &config, None).await.unwrap();
        assert_eq!(s.expired_count, 1);
        assert_eq!(s.orders_advanced, 0);
        assert_eq!(s.orders_expired, 1);

        let order = OrderRepository::new(pool.clone())
            .find_by_id(order_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(order.status, OrderStatus::Expired);
    }
}
