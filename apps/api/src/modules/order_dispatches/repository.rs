use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::modules::domain_enums::DispatchStatus;

use super::model::OrderDispatch;

const DISPATCH_COLUMNS: &str = r#"
    id, order_id, plumber_id, dispatch_rank, offer_round, status, sent_at, responded_at, created_at
"#;

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
        let q = format!(
            "SELECT {DISPATCH_COLUMNS} FROM order_dispatches WHERE id = $1"
        );
        sqlx::query_as::<_, OrderDispatch>(&q)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn find_by_order_and_plumber(
        &self,
        order_id: Uuid,
        plumber_id: Uuid,
    ) -> Result<Option<OrderDispatch>, sqlx::Error> {
        let q = format!(
            "SELECT {DISPATCH_COLUMNS} FROM order_dispatches WHERE order_id = $1 AND plumber_id = $2"
        );
        sqlx::query_as::<_, OrderDispatch>(&q)
            .bind(order_id)
            .bind(plumber_id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn list_by_order_id(
        &self,
        order_id: Uuid,
    ) -> Result<Vec<OrderDispatch>, sqlx::Error> {
        let q = format!(
            r#"
            SELECT {DISPATCH_COLUMNS}
            FROM order_dispatches
            WHERE order_id = $1
            ORDER BY offer_round, dispatch_rank, sent_at
            "#
        );
        sqlx::query_as::<_, OrderDispatch>(&q)
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
        let q = format!(
            r#"
            INSERT INTO order_dispatches (order_id, plumber_id, dispatch_rank, status)
            VALUES ($1, $2, $3, $4)
            RETURNING {DISPATCH_COLUMNS}
            "#
        );
        sqlx::query_as::<_, OrderDispatch>(&q)
            .bind(order_id)
            .bind(plumber_id)
            .bind(dispatch_rank)
            .bind(status)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn insert_tx(
        tx: &mut Transaction<'_, Postgres>,
        order_id: Uuid,
        plumber_id: Uuid,
        dispatch_rank: i16,
        offer_round: i16,
        status: DispatchStatus,
    ) -> Result<OrderDispatch, sqlx::Error> {
        let q = format!(
            r#"
            INSERT INTO order_dispatches (order_id, plumber_id, dispatch_rank, offer_round, status)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING {DISPATCH_COLUMNS}
            "#
        );
        sqlx::query_as::<_, OrderDispatch>(&q)
            .bind(order_id)
            .bind(plumber_id)
            .bind(dispatch_rank)
            .bind(offer_round)
            .bind(status)
            .fetch_one(&mut **tx)
            .await
    }

    pub async fn max_offer_round_tx(
        tx: &mut Transaction<'_, Postgres>,
        order_id: Uuid,
    ) -> Result<i16, sqlx::Error> {
        let v: Option<i16> = sqlx::query_scalar(
            r#"
            SELECT COALESCE(MAX(offer_round), 0)::smallint
            FROM order_dispatches
            WHERE order_id = $1
            "#,
        )
        .bind(order_id)
        .fetch_one(&mut **tx)
        .await?;
        Ok(v.unwrap_or(0))
    }
}
