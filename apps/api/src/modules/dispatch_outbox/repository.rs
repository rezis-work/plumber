use sqlx::{PgPool, Postgres, Transaction};

use uuid::Uuid;

use crate::modules::domain_enums::{DispatchOutboxJobKind, DispatchOutboxStatus};

use super::model::DispatchOutbox;

const LAST_ERROR_MAX_CHARS: usize = 2000;

/// Rows touched by [`DispatchOutboxRepository::requeue_expired_leases`] (§12.7 max attempts).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LeaseReclaimSummary {
    pub requeued_pending: u64,
    pub failed_max_attempts: u64,
}

#[derive(Clone)]
pub struct DispatchOutboxRepository {
    pool: PgPool,
}

impl DispatchOutboxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Insert bootstrap outbox row inside the same transaction as `orders` (Implementation 004 §12.2).
    pub async fn insert_pending_bootstrap_tx(
        tx: &mut Transaction<'_, Postgres>,
        order_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO dispatch_outbox (order_id, job_kind, status)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(order_id)
        .bind(DispatchOutboxJobKind::BootstrapFirstRound)
        .bind(DispatchOutboxStatus::Pending)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    /// Claim the oldest `pending` row for workers (`FOR UPDATE SKIP LOCKED`). Uses its own transaction.
    pub async fn try_claim_next_pending(
        pool: &PgPool,
        lease_secs: i64,
    ) -> Result<Option<DispatchOutbox>, sqlx::Error> {
        let lease_secs = lease_secs.max(1);
        let mut tx = pool.begin().await?;
        let row = sqlx::query_as::<_, DispatchOutbox>(
            r#"
            WITH picked AS (
                SELECT id
                FROM dispatch_outbox
                WHERE status = 'pending'
                ORDER BY created_at
                FOR UPDATE SKIP LOCKED
                LIMIT 1
            )
            UPDATE dispatch_outbox AS d
            SET status = 'processing',
                claimed_at = NOW(),
                lease_expires_at = NOW() + ($1::bigint * INTERVAL '1 second')
            FROM picked
            WHERE d.id = picked.id
            RETURNING d.id, d.order_id, d.job_kind, d.status, d.created_at,
                      d.claimed_at, d.lease_expires_at, d.processed_at, d.attempt_count, d.last_error
            "#,
        )
        .bind(lease_secs)
        .fetch_optional(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(row)
    }

    /// Claim the oldest `pending` row for this `order_id` (Redis queue wake-up path, §12.5).
    pub async fn try_claim_pending_for_order(
        pool: &PgPool,
        order_id: Uuid,
        lease_secs: i64,
    ) -> Result<Option<DispatchOutbox>, sqlx::Error> {
        let lease_secs = lease_secs.max(1);
        let mut tx = pool.begin().await?;
        let row = sqlx::query_as::<_, DispatchOutbox>(
            r#"
            WITH picked AS (
                SELECT id
                FROM dispatch_outbox
                WHERE status = 'pending' AND order_id = $2
                ORDER BY created_at
                FOR UPDATE SKIP LOCKED
                LIMIT 1
            )
            UPDATE dispatch_outbox AS d
            SET status = 'processing',
                claimed_at = NOW(),
                lease_expires_at = NOW() + ($1::bigint * INTERVAL '1 second')
            FROM picked
            WHERE d.id = picked.id
            RETURNING d.id, d.order_id, d.job_kind, d.status, d.created_at,
                      d.claimed_at, d.lease_expires_at, d.processed_at, d.attempt_count, d.last_error
            "#,
        )
        .bind(lease_secs)
        .bind(order_id)
        .fetch_optional(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(row)
    }

    /// Return a `processing` row to `pending` without incrementing `attempt_count` (e.g. lock not acquired).
    pub async fn release_claim_to_pending_tx(
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let r = sqlx::query(
            r#"
            UPDATE dispatch_outbox
            SET status = 'pending',
                claimed_at = NULL,
                lease_expires_at = NULL
            WHERE id = $1 AND status = 'processing'
            "#,
        )
        .bind(id)
        .execute(&mut **tx)
        .await?;
        Ok(r.rows_affected())
    }

    pub async fn mark_done_tx(
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let r = sqlx::query(
            r#"
            UPDATE dispatch_outbox
            SET status = 'done', processed_at = NOW()
            WHERE id = $1 AND status = 'processing'
            "#,
        )
        .bind(id)
        .execute(&mut **tx)
        .await?;
        Ok(r.rows_affected())
    }

    pub async fn mark_failed_tx(
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        last_error: &str,
    ) -> Result<u64, sqlx::Error> {
        let err: String = last_error.chars().take(LAST_ERROR_MAX_CHARS).collect();
        let r = sqlx::query(
            r#"
            UPDATE dispatch_outbox
            SET status = 'failed', last_error = $2, processed_at = NOW()
            WHERE id = $1 AND status = 'processing'
            "#,
        )
        .bind(id)
        .bind(err)
        .execute(&mut **tx)
        .await?;
        Ok(r.rows_affected())
    }

    /// Reclaim expired `processing` leases. If `max_attempts` is **`Some(N)`** with **N ≥ 1**,
    /// rows where **`attempt_count + 1 >= N`** become **`failed`** instead of **`pending`** (§12.7).
    pub async fn requeue_expired_leases_tx(
        tx: &mut Transaction<'_, Postgres>,
        max_attempts: Option<i32>,
    ) -> Result<LeaseReclaimSummary, sqlx::Error> {
        let rows: Vec<(DispatchOutboxStatus,)> = sqlx::query_as(
            r#"
            UPDATE dispatch_outbox
            SET
                status = CASE
                    WHEN $1::integer IS NOT NULL
                         AND (attempt_count + 1) >= $1::integer THEN 'failed'::dispatch_outbox_status
                    ELSE 'pending'::dispatch_outbox_status
                END,
                claimed_at = NULL,
                lease_expires_at = NULL,
                attempt_count = attempt_count + 1,
                last_error = CASE
                    WHEN $1::integer IS NOT NULL
                         AND (attempt_count + 1) >= $1::integer THEN 'max_attempts_exceeded'
                    ELSE COALESCE(last_error, 'lease_expired')
                END,
                processed_at = CASE
                    WHEN $1::integer IS NOT NULL
                         AND (attempt_count + 1) >= $1::integer THEN NOW()
                    ELSE NULL
                END
            WHERE status = 'processing'
              AND lease_expires_at IS NOT NULL
              AND lease_expires_at < NOW()
            RETURNING status
            "#,
        )
        .bind(max_attempts)
        .fetch_all(&mut **tx)
        .await?;

        let requeued_pending = rows
            .iter()
            .filter(|(s,)| *s == DispatchOutboxStatus::Pending)
            .count() as u64;
        let failed_max_attempts = rows
            .iter()
            .filter(|(s,)| *s == DispatchOutboxStatus::Failed)
            .count() as u64;
        Ok(LeaseReclaimSummary {
            requeued_pending,
            failed_max_attempts,
        })
    }

    pub async fn requeue_expired_leases(
        &self,
        max_attempts: Option<i32>,
    ) -> Result<LeaseReclaimSummary, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let summary = Self::requeue_expired_leases_tx(&mut tx, max_attempts).await?;
        tx.commit().await?;
        Ok(summary)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::modules::domain_enums::{DispatchOutboxStatus, OrderStatus, OrderUrgency};
    use super::DispatchOutboxRepository;

    async fn seed_order(pool: &PgPool) -> uuid::Uuid {
        let client_email = format!("outbox-client-{}@test.local", Uuid::new_v4());
        let slug = format!("outbox-city-{}", Uuid::new_v4());
        let city_id: uuid::Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO cities (name, slug, is_active)
            VALUES ('OutboxCity', $1, true)
            RETURNING id
            "#,
        )
        .bind(&slug)
        .fetch_one(pool)
        .await
        .unwrap();

        let area_slug = format!("outbox-area-{}", Uuid::new_v4());
        let area_id: uuid::Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO areas (city_id, name, slug, is_active)
            VALUES ($1, 'A', $2, true)
            RETURNING id
            "#,
        )
        .bind(city_id)
        .bind(&area_slug)
        .fetch_one(pool)
        .await
        .unwrap();

        let cat_slug = format!("outbox-cat-{}", Uuid::new_v4());
        let cat_id: uuid::Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO service_categories (name, slug, is_active, sort_order)
            VALUES ('Cat', $1, true, 0)
            RETURNING id
            "#,
        )
        .bind(&cat_slug)
        .fetch_one(pool)
        .await
        .unwrap();

        let client_id: uuid::Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
            VALUES ($1::citext, 'x', 'client', 'active', true)
            RETURNING id
            "#,
        )
        .bind(&client_email)
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
    async fn insert_claim_mark_done_flow(pool: PgPool) {
        let order_id = seed_order(&pool).await;
        let mut tx = pool.begin().await.unwrap();
        DispatchOutboxRepository::insert_pending_bootstrap_tx(&mut tx, order_id)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        let claimed = DispatchOutboxRepository::try_claim_next_pending(&pool, 120)
            .await
            .unwrap()
            .expect("claimed row");
        assert_eq!(claimed.order_id, order_id);
        assert_eq!(claimed.status, DispatchOutboxStatus::Processing);
        assert!(claimed.claimed_at.is_some());
        assert!(claimed.lease_expires_at.is_some());

        let mut tx = pool.begin().await.unwrap();
        let n = DispatchOutboxRepository::mark_done_tx(&mut tx, claimed.id)
            .await
            .unwrap();
        assert_eq!(n, 1);
        tx.commit().await.unwrap();

        let st: DispatchOutboxStatus = sqlx::query_scalar(
            "SELECT status FROM dispatch_outbox WHERE id = $1",
        )
        .bind(claimed.id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(st, DispatchOutboxStatus::Done);
        let processed: Option<chrono::DateTime<chrono::Utc>> =
            sqlx::query_scalar("SELECT processed_at FROM dispatch_outbox WHERE id = $1")
                .bind(claimed.id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert!(processed.is_some());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn requeue_expired_lease(pool: PgPool) {
        let order_id = seed_order(&pool).await;
        let mut tx = pool.begin().await.unwrap();
        DispatchOutboxRepository::insert_pending_bootstrap_tx(&mut tx, order_id)
            .await
            .unwrap();
        tx.commit().await.unwrap();

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

        let repo = DispatchOutboxRepository::new(pool.clone());
        let s = repo.requeue_expired_leases(None).await.unwrap();
        assert_eq!(s.requeued_pending, 1);
        assert_eq!(s.failed_max_attempts, 0);

        let (status, attempts, last_err): (DispatchOutboxStatus, i32, Option<String>) = sqlx::query_as(
            r#"SELECT status, attempt_count, last_error FROM dispatch_outbox WHERE id = $1"#,
        )
        .bind(claimed.id)
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(status, DispatchOutboxStatus::Pending);
        assert_eq!(attempts, 1);
        assert_eq!(last_err.as_deref(), Some("lease_expired"));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn requeue_expired_lease_marks_failed_at_max_attempts(pool: PgPool) {
        let order_id = seed_order(&pool).await;
        let mut tx = pool.begin().await.unwrap();
        DispatchOutboxRepository::insert_pending_bootstrap_tx(&mut tx, order_id)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        let claimed = DispatchOutboxRepository::try_claim_next_pending(&pool, 1)
            .await
            .unwrap()
            .unwrap();

        sqlx::query(
            r#"UPDATE dispatch_outbox SET lease_expires_at = NOW() - INTERVAL '1 minute', attempt_count = 1 WHERE id = $1"#,
        )
        .bind(claimed.id)
        .execute(&pool)
        .await
        .unwrap();

        let repo = DispatchOutboxRepository::new(pool.clone());
        let s = repo.requeue_expired_leases(Some(2)).await.unwrap();
        assert_eq!(s.requeued_pending, 0);
        assert_eq!(s.failed_max_attempts, 1);

        let (status, attempts, last_err): (DispatchOutboxStatus, i32, Option<String>) = sqlx::query_as(
            r#"SELECT status, attempt_count, last_error FROM dispatch_outbox WHERE id = $1"#,
        )
        .bind(claimed.id)
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(status, DispatchOutboxStatus::Failed);
        assert_eq!(attempts, 2);
        assert_eq!(last_err.as_deref(), Some("max_attempts_exceeded"));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn mark_failed_truncates_error(pool: PgPool) {
        let order_id = seed_order(&pool).await;
        let mut tx = pool.begin().await.unwrap();
        DispatchOutboxRepository::insert_pending_bootstrap_tx(&mut tx, order_id)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        let claimed = DispatchOutboxRepository::try_claim_next_pending(&pool, 60)
            .await
            .unwrap()
            .unwrap();

        let long = "x".repeat(3000);
        let mut tx = pool.begin().await.unwrap();
        let n = DispatchOutboxRepository::mark_failed_tx(&mut tx, claimed.id, &long)
            .await
            .unwrap();
        assert_eq!(n, 1);
        tx.commit().await.unwrap();

        let err: String = sqlx::query_scalar("SELECT last_error FROM dispatch_outbox WHERE id = $1")
            .bind(claimed.id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(err.len(), super::LAST_ERROR_MAX_CHARS);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn claim_pending_for_order_only_matching_order(pool: PgPool) {
        let order_a = seed_order(&pool).await;
        let order_b = seed_order(&pool).await;

        let mut tx = pool.begin().await.unwrap();
        DispatchOutboxRepository::insert_pending_bootstrap_tx(&mut tx, order_a)
            .await
            .unwrap();
        DispatchOutboxRepository::insert_pending_bootstrap_tx(&mut tx, order_b)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        let claimed_a = DispatchOutboxRepository::try_claim_pending_for_order(&pool, order_a, 120)
            .await
            .unwrap()
            .expect("row a");
        assert_eq!(claimed_a.order_id, order_a);

        let claimed_b = DispatchOutboxRepository::try_claim_pending_for_order(&pool, order_b, 120)
            .await
            .unwrap()
            .expect("row b");
        assert_eq!(claimed_b.order_id, order_b);

        let none = DispatchOutboxRepository::try_claim_pending_for_order(&pool, order_a, 120)
            .await
            .unwrap();
        assert!(none.is_none(), "order_a outbox already claimed");

        let mut tx = pool.begin().await.unwrap();
        DispatchOutboxRepository::mark_done_tx(&mut tx, claimed_a.id).await.unwrap();
        DispatchOutboxRepository::mark_done_tx(&mut tx, claimed_b.id).await.unwrap();
        tx.commit().await.unwrap();
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn release_claim_to_pending_preserves_attempt_count(pool: PgPool) {
        let order_id = seed_order(&pool).await;
        let mut tx = pool.begin().await.unwrap();
        DispatchOutboxRepository::insert_pending_bootstrap_tx(&mut tx, order_id)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        let claimed = DispatchOutboxRepository::try_claim_next_pending(&pool, 120)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(claimed.attempt_count, 0);

        let mut tx = pool.begin().await.unwrap();
        let n = DispatchOutboxRepository::release_claim_to_pending_tx(&mut tx, claimed.id)
            .await
            .unwrap();
        assert_eq!(n, 1);
        tx.commit().await.unwrap();

        let (status, attempts): (DispatchOutboxStatus, i32) = sqlx::query_as(
            r#"SELECT status, attempt_count FROM dispatch_outbox WHERE id = $1"#,
        )
        .bind(claimed.id)
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(status, DispatchOutboxStatus::Pending);
        assert_eq!(attempts, 0);
    }
}
