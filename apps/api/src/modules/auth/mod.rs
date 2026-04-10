pub mod dto;
pub mod error;
pub mod handler;
pub mod passwords;
pub mod register_error;
pub mod registration;
pub mod routes;
pub mod service;
pub mod verification;

pub use error::AuthError;
pub use passwords::{
    hash_password, normalize_email, validate_password_policy, verify_password, PasswordConfig,
};
pub use routes::auth_routes;
pub use verification::EmailVerificationConfig;
