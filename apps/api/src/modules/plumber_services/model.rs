use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PlumberService {
    pub id: Uuid,
    pub plumber_id: Uuid,
    pub service_category_id: Uuid,
    pub created_at: DateTime<Utc>,
}
