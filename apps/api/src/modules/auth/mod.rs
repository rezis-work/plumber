//! Step 8 RBAC: [`crate::require_role!`] and [`crate::require_any_role!`] (from [`middleware_rbac`]) compose with [`require_access_token`] / [`require_authenticated`].

pub mod auth_context;
pub mod auth_forbidden;
pub mod auth_unauthorized;
pub mod auth_user;
pub mod claims;
pub mod cookie_config;
pub mod dto;
pub mod error;
pub mod handler;
pub mod login_error;
pub mod logout_error;
pub mod middleware_access;
pub mod middleware_rbac;
pub mod passwords;
pub mod refresh_token_hash;
pub mod register_error;
pub mod registration;
pub mod refresh_error;
pub mod routes;
pub mod service;
pub mod service_token;
pub mod verification;

pub use auth_context::AuthContext;
pub use auth_forbidden::AuthForbidden;
pub use logout_error::LogoutError;
pub use refresh_error::RefreshError;
pub use claims::{AuthJwtClaims, TokenType};
pub use cookie_config::CookieConfig;
pub use error::AuthError;
pub use passwords::{
    hash_password, normalize_email, validate_password_policy, verify_password, PasswordConfig,
};
pub use refresh_token_hash::hash_refresh_jwt_for_storage;
pub use middleware_access::require_access_token;
pub use routes::auth_routes;

/// Same as [`require_access_token`]: Bearer access JWT + [`AuthContext`] (Step 7).
pub use middleware_access::require_access_token as require_authenticated;
pub use service_token::{JwtConfig, JwtError};
pub use verification::EmailVerificationConfig;
