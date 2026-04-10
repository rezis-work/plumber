pub mod error;
pub mod passwords;

pub use error::AuthError;
pub use passwords::{
    hash_password, normalize_email, validate_password_policy, verify_password, PasswordConfig,
};
