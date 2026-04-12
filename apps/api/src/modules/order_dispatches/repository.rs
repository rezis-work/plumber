use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::domain_enums::DispatchStatus;

use super::model::OrderDispatch;

#[derive(Clone)]
pub struct OrderDispatchRepository {
    pool: PgPool,
}

impl OrderDispatchRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<OrderDispatch>, sqlx::Error> {
        sqlx::query_as::<_, OrderDispatch>(
            r#"
            SELECT id, order_id, plumber_id, dispatch_rank, status, sent_at, responded_at, created_at
            FROM order_dispatches
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_order_and_plumber(
        &self,
        order_id: Uuid,
        plumber_id: Uuid,
    ) -> Result<Option<OrderDispatch>, sqlx::Error> {
        sqlx::query_as::<_, OrderDispatch>(
            r#"
            SELECT id, order_id, plumber_id, dispatch_rank, status, sent_at, responded_at, created_at
            FROM order_dispatches
            WHERE order_id = $1 AND plumber_id = $2
            "#,
        )
        .bind(order_id)
        .bind(plumber_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn list_by_order_id(
        &self,
        order_id: Uuid,
    ) -> Result<Vec<OrderDispatch>, sqlx::Error> {
        sqlx::query_as::<_, OrderDispatch>(
            r#"
            SELECT id, order_id, plumber_id, dispatch_rank, status, sent_at, responded_at, created_at
            FROM order_dispatches
            WHERE order_id = $1
            ORDER BY dispatch_rank, sent_at
            "#,
        )
        .bind(order_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn insert(
        &self,
        order_id: Uuid,
        plumber_id: Uuid,
        dispatch_rank: i16,
        status: DispatchStatus,
    ) -> Result<OrderDispatch, sqlx::Error> {
        sqlx::query_as::<_, OrderDispatch>(
            r#"
            INSERT INTO order_dispatches (order_id, plumber_id, dispatch_rank, status)
            VALUES ($1, $2, $3, $4)
            RETURNING id, order_id, plumber_id, dispatch_rank, status, sent_at, responded_at, created_at
            "#,
        )
        .bind(order_id)
        .bind(plumber_id)
        .bind(dispatch_rank)
        .bind(status)
        .fetch_one(&self.pool)
        .await
    }
}
