use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct MatcherCandidate {
    pub plumber_id: Uuid,
    pub token_balance: i32,
    pub rating_avg: Decimal,
    pub distance_km: f64,
}
