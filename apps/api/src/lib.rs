pub mod modules;

pub use modules::auth::{
    hash_password, normalize_email, validate_password_policy, verify_password, AuthError,
    EmailVerificationConfig, PasswordConfig,
};
pub use modules::users::{Role, User, UserRepository};

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub users: UserRepository,
    pub password_config: PasswordConfig,
    pub email_verification: EmailVerificationConfig,
}
