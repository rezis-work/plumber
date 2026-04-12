use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::types::Json;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AdminAuditLog {
    pub id: Uuid,
    pub admin_id: Option<Uuid>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: String,
    pub meta: Option<Json<Value>>,
    pub created_at: DateTime<Utc>,
}
