pub mod claims;
pub mod cookie_config;
pub mod dto;
pub mod error;
pub mod handler;
pub mod login_error;
pub mod passwords;
pub mod refresh_token_hash;
pub mod register_error;
pub mod registration;
pub mod routes;
pub mod service;
pub mod service_token;
pub mod verification;

pub use claims::{AuthJwtClaims, TokenType};
pub use cookie_config::CookieConfig;
pub use error::AuthError;
pub use passwords::{
    hash_password, normalize_email, validate_password_policy, verify_password, PasswordConfig,
};
pub use refresh_token_hash::hash_refresh_jwt_for_storage;
pub use routes::auth_routes;
pub use service_token::{JwtConfig, JwtError};
pub use verification::EmailVerificationConfig;
