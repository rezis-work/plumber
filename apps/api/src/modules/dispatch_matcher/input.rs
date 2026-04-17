use uuid::Uuid;

use crate::modules::domain_enums::OrderUrgency;

#[derive(Debug, Clone)]
pub struct MatcherOrderInput {
    pub order_id: Uuid,
    pub service_category_id: Uuid,
    pub city_id: Uuid,
    pub area_id: Option<Uuid>,
    pub lat: f64,
    pub lng: f64,
    pub urgency: OrderUrgency,
}
