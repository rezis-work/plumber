use chrono::{DateTime, Utc};
use sqlx::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
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
    #[allow(dead_code)] // read via password_hash(); used by auth after Step 1
    password_hash: String,
    pub role: Role,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    #[allow(dead_code)]
    pub(crate) fn password_hash(&self) -> &str {
        &self.password_hash
    }
}
