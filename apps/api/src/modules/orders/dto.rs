use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::domain_enums::{OrderStatus, OrderUrgency};

#[derive(Debug, Deserialize)]
pub struct CreateOrderMediaItem {
    pub storage_key: String,
    pub content_type: String,
    pub byte_size: i32,
    #[serde(default)]
    pub sort_order: Option<i16>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub service_category_id: Uuid,
    pub city_id: Uuid,
    pub area_id: Option<Uuid>,
    pub street_id: Option<Uuid>,
    pub address_line: String,
    pub lat: f64,
    pub lng: f64,
    pub description: String,
    pub urgency: OrderUrgency,
    pub estimated_price_min: Option<Decimal>,
    pub estimated_price_max: Option<Decimal>,
    #[serde(default)]
    pub media: Vec<CreateOrderMediaItem>,
}

#[derive(Debug, Serialize)]
pub struct CreateOrderResponse {
    pub id: Uuid,
    pub status: OrderStatus,
    pub requested_at: DateTime<Utc>,
}
