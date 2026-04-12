use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::modules::domain_enums::{OrderStatus, OrderUrgency};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Order {
    pub id: Uuid,
    pub client_id: Uuid,
    pub assigned_plumber_id: Option<Uuid>,
    pub service_category_id: Uuid,
    pub city_id: Uuid,
    pub area_id: Option<Uuid>,
    pub street_id: Option<Uuid>,
    pub address_line: String,
    pub lat: f64,
    pub lng: f64,
    pub description: Option<String>,
    pub urgency: OrderUrgency,
    pub status: OrderStatus,
    pub estimated_price_min: Option<Decimal>,
    pub estimated_price_max: Option<Decimal>,
    pub final_price: Option<Decimal>,
    pub requested_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancel_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
