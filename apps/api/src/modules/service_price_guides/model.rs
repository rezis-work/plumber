use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ServicePriceGuide {
    pub id: Uuid,
    pub service_category_id: Uuid,
    pub city_id: Option<Uuid>,
    pub area_id: Option<Uuid>,
    pub min_price: Decimal,
    pub max_price: Decimal,
    pub currency: String,
    pub estimated_duration_minutes: i32,
    pub is_emergency_supported: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
