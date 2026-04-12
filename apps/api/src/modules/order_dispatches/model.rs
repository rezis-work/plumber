use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::modules::domain_enums::DispatchStatus;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct OrderDispatch {
    pub id: Uuid,
    pub order_id: Uuid,
    pub plumber_id: Uuid,
    pub dispatch_rank: i16,
    pub status: DispatchStatus,
    pub sent_at: DateTime<Utc>,
    pub responded_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
