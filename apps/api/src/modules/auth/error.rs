use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AuthError {
    #[error("invalid email")]
    InvalidEmail,
    #[error("invalid password")]
    InvalidPassword,
    #[error("password does not meet policy")]
    WeakPassword,
    #[error("password hashing failed")]
    HashingFailed,
    #[error("invalid stored password hash")]
    InvalidPasswordHash,
}
