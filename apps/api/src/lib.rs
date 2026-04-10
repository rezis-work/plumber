pub mod modules;

pub use modules::users::{Role, User, UserRepository};

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub users: UserRepository,
}
