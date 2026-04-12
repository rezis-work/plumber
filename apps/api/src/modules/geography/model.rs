use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct City {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Area {
    pub id: Uuid,
    pub city_id: Uuid,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Street {
    pub id: Uuid,
    pub city_id: Uuid,
    pub area_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
