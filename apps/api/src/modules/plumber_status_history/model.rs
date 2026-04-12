use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::types::Json;
use uuid::Uuid;

use crate::modules::domain_enums::PlumberStatusType;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PlumberStatusHistory {
    pub id: Uuid,
    pub plumber_id: Uuid,
    pub status_type: PlumberStatusType,
    pub meta: Option<Json<Value>>,
    pub created_at: DateTime<Utc>,
}
