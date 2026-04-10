pub mod modules;

pub use modules::auth::{
    hash_password, normalize_email, validate_password_policy, verify_password, AuthError,
    PasswordConfig,
};
pub use modules::users::{Role, User, UserRepository};

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub users: UserRepository,
}
