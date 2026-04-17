use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use sqlx::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, Serialize)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum Role {
    Client,
    Plumber,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, Serialize)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Blocked,
    Pending,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[allow(dead_code)]
    password_hash: String,
    pub role: Role,
    pub user_status: UserStatus,
    pub last_login_at: Option<DateTime<Utc>>,
    pub blocked_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_email_verified: bool,
    pub email_verification_token_hash: Option<String>,
    pub email_verification_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    #[allow(dead_code)] // repository integration test
    pub(crate) fn password_hash(&self) -> &str {
        &self.password_hash
    }

    /// Login and refresh are allowed only for active, non–soft-deleted accounts.
    pub fn login_allowed(&self) -> bool {
        self.user_status == UserStatus::Active && self.deleted_at.is_none()
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PlumberProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub full_name: String,
    pub phone: String,
    pub experience_years: i32,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub is_approved: bool,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<Uuid>,
    pub is_online: bool,
    pub is_available: bool,
    pub current_city_id: Option<Uuid>,
    pub current_area_id: Option<Uuid>,
    pub current_street_id: Option<Uuid>,
    pub current_lat: Option<f64>,
    pub current_lng: Option<f64>,
    pub service_radius_km: Decimal,
    pub last_location_updated_at: Option<DateTime<Utc>>,
    pub rating_avg: Decimal,
    pub rating_count: i32,
    pub completed_orders_count: i32,
    pub cancelled_orders_count: i32,
    pub token_balance: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ClientProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub full_name: String,
    pub phone: String,
    pub default_city_id: Option<Uuid>,
    pub default_area_id: Option<Uuid>,
    pub default_street_id: Option<Uuid>,
    pub address_line: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RefreshTokenRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub jti: String,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct CreateRefreshSessionParams<'a> {
    pub user_id: Uuid,
    pub jti: &'a str,
    pub token_hash: &'a str,
    pub expires_at: DateTime<Utc>,
}
