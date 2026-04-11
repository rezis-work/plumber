use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::users::Role;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in: u64,
}

/// Step 12 `GET /auth/me`: user from DB + optional plumber profile (no secrets).
#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub id: Uuid,
    pub email: String,
    pub role: Role,
    pub is_active: bool,
    pub is_email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub profile: Option<PlumberProfileResponse>,
}

/// Step 11 `POST /auth/logout-all` success body.
#[derive(Debug, Serialize)]
pub struct LogoutAllResponse {
    pub sessions_revoked: u64,
}

#[derive(Debug, Deserialize)]
pub struct RegisterClientRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterPlumberRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub phone: String,
    pub years_of_experience: i32,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: Role,
    pub is_active: bool,
    pub is_email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct RegisterClientResponse {
    #[serde(flatten)]
    pub user: UserResponse,
    pub email_verification_token: String,
    pub email_verification_expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PlumberProfileResponse {
    pub full_name: String,
    pub phone: String,
    pub years_of_experience: i32,
}

#[derive(Debug, Serialize)]
pub struct RegisterPlumberResponse {
    #[serde(flatten)]
    pub user: UserResponse,
    pub profile: PlumberProfileResponse,
}

impl From<crate::modules::users::User> for UserResponse {
    fn from(u: crate::modules::users::User) -> Self {
        Self {
            id: u.id,
            email: u.email,
            role: u.role,
            is_active: u.is_active,
            is_email_verified: u.is_email_verified,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

impl From<crate::modules::users::PlumberProfile> for PlumberProfileResponse {
    fn from(p: crate::modules::users::PlumberProfile) -> Self {
        Self {
            full_name: p.full_name,
            phone: p.phone,
            years_of_experience: p.years_of_experience,
        }
    }
}
