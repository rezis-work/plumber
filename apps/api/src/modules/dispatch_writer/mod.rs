//! OD-3 dispatch writer: transactional round advance + optional Redis (Upstash).
//! See `docs/implementation_003_domain_modeling/implementation_003_orders_dispatch_tokens_redis.md` §10.

mod expire_job;
mod handler;
mod reconcile;
mod redis;
pub mod service;
mod worker;

pub use expire_job::{run_dispatch_expiry_tick, ExpireTickError, ExpireTickSummary};
pub use handler::{
    post_advance, post_expire_due, post_reconcile_outbox, AdvanceRequest, AdvanceResponse,
    ExpireDueResponse, ReconcileOutboxResponse,
};
pub use reconcile::{reconcile_stale_outbox, ReconcileError, ReconcileSummary};
pub use redis::{DispatchRedisError, RedisDispatchHelper};
pub use service::{
    advance_dispatch_outcome_label, advance_dispatch_round, AdvanceDispatchError,
    AdvanceDispatchOutcome,
};
pub use worker::{
    dispatch_queue_worker_concurrency, dispatch_queue_worker_enabled, run_dispatch_queue_worker,
};

use axum::routing::post;
use axum::Router;

use crate::AppState;

pub fn dispatch_writer_routes() -> Router<AppState> {
    Router::new()
        .route("/advance", post(post_advance))
        .route("/expire-due", post(post_expire_due))
        .route("/reconcile-outbox", post(post_reconcile_outbox))
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::modules::dispatch_matcher::MatcherConfig;
    use crate::modules::domain_enums::{OrderStatus, OrderUrgency};
    use crate::modules::order_dispatches::OrderDispatchRepository;
    use crate::modules::orders::OrderRepository;

    use super::service::{advance_dispatch_round, AdvanceDispatchOutcome};

    async fn seed_city_area_category(pool: &PgPool) -> (Uuid, Uuid, Uuid) {
        let city_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO cities (name, slug, is_active)
            VALUES ('Tbilisi', 'dw3-tbilisi', true)
            RETURNING id
            "#,
        )
        .fetch_one(pool)
        .await
        .unwrap();

        let area_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO areas (city_id, name, slug, is_active)
            VALUES ($1, 'Vake', 'dw3-vake', true)
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
            VALUES ('Leak', 'dw3-leak', true, 0)
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
    ) -> Uuid {
        let client_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
            VALUES ('dw3-client@test.local', 'x', 'client', 'active', true)
            RETURNING id
            "#,
        )
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
        .bind(OrderUrgency::Normal)
        .bind(OrderStatus::Searching)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn advance_first_round_dispatched_status_and_offer_round(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_city_area_category(&pool).await;
        for i in 1..=3 {
            insert_plumber(
                &pool,
                &format!("dw3-p{i}@test.local"),
                city_id,
                cat_id,
                area_id,
            )
            .await;
        }
        let order_id = insert_searching_order(&pool, city_id, area_id, cat_id).await;
        let config = MatcherConfig::default();

        let out = advance_dispatch_round(&pool, order_id, &config, None)
            .await
            .expect("advance");

        let AdvanceDispatchOutcome::Success {
            offer_round,
            inserted_plumber_ids,
            ..
        } = out
        else {
            panic!("expected success: {out:?}");
        };
        assert_eq!(offer_round, 1);
        assert_eq!(inserted_plumber_ids.len(), 3);

        let order = OrderRepository::new(pool.clone())
            .find_by_id(order_id)
            .await
            .unwrap()
            .expect("order");
        assert_eq!(order.status, OrderStatus::Dispatched);

        let rows = OrderDispatchRepository::new(pool.clone())
            .list_by_order_id(order_id)
            .await
            .unwrap();
        assert_eq!(rows.len(), 3);
        assert!(rows.iter().all(|r| r.offer_round == 1));
        assert!(rows.iter().all(|r| r.dispatch_rank >= 1 && r.dispatch_rank <= 3));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn advance_second_round_two_more_plumbers(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_city_area_category(&pool).await;
        for i in 1..=5 {
            insert_plumber(
                &pool,
                &format!("dw3b-p{i}@test.local"),
                city_id,
                cat_id,
                area_id,
            )
            .await;
        }
        let order_id = insert_searching_order(&pool, city_id, area_id, cat_id).await;
        let config = MatcherConfig::default();

        let first = advance_dispatch_round(&pool, order_id, &config, None)
            .await
            .expect("advance1");
        assert!(matches!(first, AdvanceDispatchOutcome::Success { .. }));

        let second = advance_dispatch_round(&pool, order_id, &config, None)
            .await
            .expect("advance2");
        let AdvanceDispatchOutcome::Success { offer_round, .. } = second else {
            panic!("expected success round2");
        };
        assert_eq!(offer_round, 2);

        let rows = OrderDispatchRepository::new(pool.clone())
            .list_by_order_id(order_id)
            .await
            .unwrap();
        assert_eq!(rows.len(), 5);
        let r1 = rows.iter().filter(|r| r.offer_round == 1).count();
        let r2 = rows.iter().filter(|r| r.offer_round == 2).count();
        assert_eq!(r1, 3);
        assert_eq!(r2, 2);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn advance_skips_non_dispatchable_completed(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_city_area_category(&pool).await;
        insert_plumber(&pool, "dw3c-p1@test.local", city_id, cat_id, area_id).await;
        let order_id = insert_searching_order(&pool, city_id, area_id, cat_id).await;

        sqlx::query(r#"UPDATE orders SET status = 'completed' WHERE id = $1"#)
            .bind(order_id)
            .execute(&pool)
            .await
            .unwrap();

        let out = advance_dispatch_round(&pool, order_id, &MatcherConfig::default(), None)
            .await
            .expect("advance");
        assert_eq!(out, AdvanceDispatchOutcome::SkippedNotDispatchable);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn advance_skips_when_no_eligible_plumbers(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_city_area_category(&pool).await;
        let order_id = insert_searching_order(&pool, city_id, area_id, cat_id).await;

        let out = advance_dispatch_round(&pool, order_id, &MatcherConfig::default(), None)
            .await
            .expect("advance");
        assert_eq!(out, AdvanceDispatchOutcome::SkippedNoPlumbers);

        let order = OrderRepository::new(pool.clone())
            .find_by_id(order_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(order.status, OrderStatus::Searching);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn advance_order_not_found(pool: PgPool) {
        let out = advance_dispatch_round(&pool, Uuid::new_v4(), &MatcherConfig::default(), None)
            .await
            .expect("advance");
        assert_eq!(out, AdvanceDispatchOutcome::SkippedOrderNotFound);
    }

    /// §12.1: `dispatch_outbox` partial unique + FK cascade from `orders`.
    #[sqlx::test(migrations = "./migrations")]
    async fn dispatch_outbox_partial_unique_and_order_cascade(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_city_area_category(&pool).await;
        let order_id = insert_searching_order(&pool, city_id, area_id, cat_id).await;

        sqlx::query(
            r#"INSERT INTO dispatch_outbox (order_id, job_kind, status)
               VALUES ($1, 'bootstrap_first_round', 'pending')"#,
        )
        .bind(order_id)
        .execute(&pool)
        .await
        .expect("first pending insert");

        let err = sqlx::query(
            r#"INSERT INTO dispatch_outbox (order_id, job_kind, status)
               VALUES ($1, 'bootstrap_first_round', 'pending')"#,
        )
        .bind(order_id)
        .execute(&pool)
        .await
        .expect_err("second pending same order+kind should violate unique index");

        let sqlx::Error::Database(db) = &err else {
            panic!("expected database error: {err:?}");
        };
        assert_eq!(db.code().as_deref(), Some("23505"), "{db:?}");

        sqlx::query(
            r#"UPDATE dispatch_outbox SET status = 'done' WHERE order_id = $1 AND status = 'pending'"#,
        )
        .bind(order_id)
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"INSERT INTO dispatch_outbox (order_id, job_kind, status)
               VALUES ($1, 'bootstrap_first_round', 'pending')"#,
        )
        .bind(order_id)
        .execute(&pool)
        .await
        .expect("pending again after done should succeed");

        let before: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM dispatch_outbox WHERE order_id = $1"#)
            .bind(order_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(before, 2);

        sqlx::query(r#"DELETE FROM orders WHERE id = $1"#)
            .bind(order_id)
            .execute(&pool)
            .await
            .unwrap();

        let after: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM dispatch_outbox WHERE order_id = $1"#)
            .bind(order_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(after, 0);
    }
}
