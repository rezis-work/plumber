use chrono::{DateTime, Utc};
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

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[allow(dead_code)]
    password_hash: String,
    pub role: Role,
    pub is_active: bool,
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
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PlumberProfile {
    pub user_id: Uuid,
    pub full_name: String,
    pub phone: String,
    pub years_of_experience: i32,
}
