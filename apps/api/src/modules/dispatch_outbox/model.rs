use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::modules::domain_enums::{DispatchOutboxJobKind, DispatchOutboxStatus};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DispatchOutbox {
    pub id: Uuid,
    pub order_id: Uuid,
    pub job_kind: DispatchOutboxJobKind,
    pub status: DispatchOutboxStatus,
    pub created_at: DateTime<Utc>,
    pub claimed_at: Option<DateTime<Utc>>,
    pub lease_expires_at: Option<DateTime<Utc>>,
    pub processed_at: Option<DateTime<Utc>>,
    pub attempt_count: i32,
    pub last_error: Option<String>,
}
