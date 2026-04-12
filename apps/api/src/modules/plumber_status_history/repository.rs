use serde_json::Value;
use sqlx::types::Json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::domain_enums::PlumberStatusType;

use super::model::PlumberStatusHistory;

#[derive(Clone)]
pub struct PlumberStatusHistoryRepository {
    pool: PgPool,
}

impl PlumberStatusHistoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn list_by_plumber_id(
        &self,
        plumber_id: Uuid,
    ) -> Result<Vec<PlumberStatusHistory>, sqlx::Error> {
        sqlx::query_as::<_, PlumberStatusHistory>(
            r#"
            SELECT id, plumber_id, status_type, meta, created_at
            FROM plumber_status_history
            WHERE plumber_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(plumber_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn insert(
        &self,
        plumber_id: Uuid,
        status_type: PlumberStatusType,
        meta: Option<Value>,
    ) -> Result<PlumberStatusHistory, sqlx::Error> {
        let meta_json = meta.map(Json);
        sqlx::query_as::<_, PlumberStatusHistory>(
            r#"
            INSERT INTO plumber_status_history (plumber_id, status_type, meta)
            VALUES ($1, $2, $3)
            RETURNING id, plumber_id, status_type, meta, created_at
            "#,
        )
        .bind(plumber_id)
        .bind(status_type)
        .bind(meta_json)
        .fetch_one(&self.pool)
        .await
    }
}
