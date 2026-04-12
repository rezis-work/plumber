use serde_json::Value;
use sqlx::types::Json;
use sqlx::PgPool;
use uuid::Uuid;

use super::model::AdminAuditLog;

const MAX_LIST_LIMIT: i64 = 100;

#[derive(Clone)]
pub struct AdminAuditLogRepository {
    pool: PgPool,
}

impl AdminAuditLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn insert(
        &self,
        admin_id: Option<Uuid>,
        action: &str,
        entity_type: &str,
        entity_id: &str,
        meta: Option<Value>,
    ) -> Result<AdminAuditLog, sqlx::Error> {
        let meta_json = meta.map(Json);
        sqlx::query_as::<_, AdminAuditLog>(
            r#"
            INSERT INTO admin_audit_logs (admin_id, action, entity_type, entity_id, meta)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, admin_id, action, entity_type, entity_id, meta, created_at
            "#,
        )
        .bind(admin_id)
        .bind(action)
        .bind(entity_type)
        .bind(entity_id)
        .bind(meta_json)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn list_by_entity(
        &self,
        entity_type: &str,
        entity_id: &str,
        limit: i64,
    ) -> Result<Vec<AdminAuditLog>, sqlx::Error> {
        let limit = limit.clamp(1, MAX_LIST_LIMIT);
        sqlx::query_as::<_, AdminAuditLog>(
            r#"
            SELECT id, admin_id, action, entity_type, entity_id, meta, created_at
            FROM admin_audit_logs
            WHERE entity_type = $1 AND entity_id = $2
            ORDER BY created_at DESC
            LIMIT $3
            "#,
        )
        .bind(entity_type)
        .bind(entity_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn list_recent(&self, limit: i64) -> Result<Vec<AdminAuditLog>, sqlx::Error> {
        let limit = limit.clamp(1, MAX_LIST_LIMIT);
        sqlx::query_as::<_, AdminAuditLog>(
            r#"
            SELECT id, admin_id, action, entity_type, entity_id, meta, created_at
            FROM admin_audit_logs
            ORDER BY created_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }
}
