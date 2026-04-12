use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PlumberServiceArea {
    pub id: Uuid,
    pub plumber_id: Uuid,
    pub city_id: Uuid,
    pub area_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}
