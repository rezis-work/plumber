pub mod modules;

pub use modules::auth::{
    hash_password, hash_refresh_jwt_for_storage, normalize_email, validate_password_policy,
    verify_password, AuthError, AuthJwtClaims, CookieConfig, EmailVerificationConfig, JwtConfig,
    JwtError, PasswordConfig, TokenType,
};
pub use modules::users::{
    CreateRefreshSessionParams, RefreshTokenRecord, RefreshTokenRepository, Role, User,
    UserRepository,
};

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub users: UserRepository,
    pub password_config: PasswordConfig,
    pub email_verification: EmailVerificationConfig,
    pub jwt_config: JwtConfig,
    pub refresh_tokens: RefreshTokenRepository,
    pub cookie_config: CookieConfig,
}
