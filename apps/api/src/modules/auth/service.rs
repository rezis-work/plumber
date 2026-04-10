use chrono::{Duration, Utc};

use crate::modules::users::{CreateUserParams, Role, UserRepository};
use crate::AppState;

use super::dto::{
    RegisterClientRequest, RegisterClientResponse, RegisterPlumberRequest, RegisterPlumberResponse,
    PlumberProfileResponse, UserResponse,
};
use super::error::AuthError;
use super::passwords::{hash_password, normalize_email};
use super::register_error::RegisterError;
use super::registration::{normalize_and_validate_phone, validate_full_name, validate_years_of_experience};
use super::verification::EmailVerificationConfig;

fn map_auth_error(e: AuthError) -> RegisterError {
    match e {
        AuthError::InvalidEmail => RegisterError::Validation {
            message: "invalid email".to_string(),
        },
        AuthError::InvalidPassword | AuthError::WeakPassword => RegisterError::Validation {
            message: "invalid password".to_string(),
        },
        AuthError::HashingFailed | AuthError::InvalidPasswordHash => RegisterError::Internal,
    }
}

fn is_unique_violation(e: &sqlx::Error) -> bool {
    if let Some(db) = e.as_database_error() {
        return db.code().as_deref() == Some("23505");
    }
    false
}

pub async fn register_client(
    state: &AppState,
    body: RegisterClientRequest,
) -> Result<RegisterClientResponse, RegisterError> {
    let email = normalize_email(&body.email).map_err(map_auth_error)?;
    let password_hash = hash_password(&body.password, &state.password_config).map_err(map_auth_error)?;

    if state
        .users
        .find_by_email(&email)
        .await
        .map_err(|_| RegisterError::Internal)?
        .is_some()
    {
        return Err(RegisterError::Conflict);
    }

    let raw_hex = EmailVerificationConfig::generate_raw_token_hex();
    let token_hash = state
        .email_verification
        .hash_raw_token_hex(&raw_hex)
        .map_err(|_| RegisterError::Internal)?;
    let expires_at =
        Utc::now() + Duration::hours(state.email_verification.ttl_hours as i64);

    let user = match state
        .users
        .create_user(CreateUserParams {
            email: &email,
            password_hash: &password_hash,
            role: Role::Client,
            is_active: true,
            is_email_verified: false,
            email_verification_token_hash: Some(&token_hash),
            email_verification_expires_at: Some(expires_at),
        })
        .await
    {
        Ok(u) => u,
        Err(e) => {
            if is_unique_violation(&e) {
                return Err(RegisterError::Conflict);
            }
            return Err(RegisterError::Internal);
        }
    };

    let expires_at = user
        .email_verification_expires_at
        .ok_or(RegisterError::Internal)?;

    Ok(RegisterClientResponse {
        user: UserResponse::from(user),
        email_verification_token: raw_hex,
        email_verification_expires_at: expires_at,
    })
}

/// Option B: plumber accounts are created with `is_email_verified = true`; no verification token in response.
pub async fn register_plumber(
    state: &AppState,
    body: RegisterPlumberRequest,
) -> Result<RegisterPlumberResponse, RegisterError> {
    let email = normalize_email(&body.email).map_err(map_auth_error)?;
    let full_name = validate_full_name(&body.full_name)?;
    let phone = normalize_and_validate_phone(&body.phone)?;
    validate_years_of_experience(body.years_of_experience)?;
    let password_hash = hash_password(&body.password, &state.password_config).map_err(map_auth_error)?;

    if state
        .users
        .find_by_email(&email)
        .await
        .map_err(|_| RegisterError::Internal)?
        .is_some()
    {
        return Err(RegisterError::Conflict);
    }

    let mut tx = state
        .users
        .pool()
        .begin()
        .await
        .map_err(|_| RegisterError::Internal)?;

    let user = match UserRepository::create_user_tx(
        &mut tx,
        CreateUserParams {
            email: &email,
            password_hash: &password_hash,
            role: Role::Plumber,
            is_active: true,
            is_email_verified: true,
            email_verification_token_hash: None,
            email_verification_expires_at: None,
        },
    )
    .await
    {
        Ok(u) => u,
        Err(e) => {
            let _ = tx.rollback().await;
            if is_unique_violation(&e) {
                return Err(RegisterError::Conflict);
            }
            return Err(RegisterError::Internal);
        }
    };

    let profile = match UserRepository::create_plumber_profile_tx(
        &mut tx,
        user.id,
        &full_name,
        &phone,
        body.years_of_experience,
    )
    .await
    {
        Ok(p) => p,
        Err(_) => {
            let _ = tx.rollback().await;
            return Err(RegisterError::Internal);
        }
    };

    tx.commit()
        .await
        .map_err(|_| RegisterError::Internal)?;

    Ok(RegisterPlumberResponse {
        user: UserResponse::from(user),
        profile: PlumberProfileResponse {
            full_name: profile.full_name,
            phone: profile.phone,
            years_of_experience: profile.years_of_experience,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::auth::dto::{RegisterClientRequest, RegisterPlumberRequest};
    use crate::modules::auth::passwords::PasswordConfig;
    use crate::modules::auth::verification::EmailVerificationConfig;
    use crate::modules::users::UserRepository;
    use sqlx::PgPool;

    fn test_app_state(pool: PgPool) -> AppState {
        AppState {
            pool: pool.clone(),
            users: UserRepository::new(pool),
            password_config: PasswordConfig::from_env(),
            email_verification: EmailVerificationConfig {
                secret: "integration-test-hmac-key".to_string(),
                ttl_hours: 48,
            },
        }
    }

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test register_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn register_client_persists_hashed_token(pool: PgPool) -> sqlx::Result<()> {
        let state = test_app_state(pool);
        let res = register_client(
            &state,
            RegisterClientRequest {
                email: "client-reg@example.com".to_string(),
                password: "password123".to_string(),
            },
        )
        .await
        .expect("register_client");

        assert!(!res.user.is_email_verified);
        assert_eq!(res.email_verification_token.len(), 64);

        let stored = state
            .users
            .find_by_email("client-reg@example.com")
            .await?
            .expect("user row");
        assert!(!stored.is_email_verified);
        let db_hash = stored
            .email_verification_token_hash
            .as_ref()
            .expect("verification hash set");
        assert_eq!(db_hash.len(), 64);
        assert_ne!(db_hash.as_str(), res.email_verification_token.as_str());
        let expected = state
            .email_verification
            .hash_raw_token_hex(&res.email_verification_token)
            .expect("hash");
        assert_eq!(db_hash.as_str(), expected.as_str());

        let second = register_client(
            &state,
            RegisterClientRequest {
                email: "client-reg@example.com".to_string(),
                password: "password123".to_string(),
            },
        )
        .await;
        assert!(matches!(second, Err(RegisterError::Conflict)));

        Ok(())
    }

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test register_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn register_plumber_option_b_verified_and_profile(pool: PgPool) -> sqlx::Result<()> {
        let state = test_app_state(pool);
        let body = RegisterPlumberRequest {
            email: "plumber-reg@example.com".to_string(),
            password: "password123".to_string(),
            full_name: "  Jane Doe  ".to_string(),
            phone: "+1 234 567 8901".to_string(),
            years_of_experience: 5,
        };
        let res = register_plumber(&state, body).await.expect("register_plumber");

        assert!(res.user.is_email_verified);
        assert_eq!(res.profile.full_name, "Jane Doe");
        assert_eq!(res.profile.phone, "+12345678901");
        assert_eq!(res.profile.years_of_experience, 5);

        let stored = state
            .users
            .find_by_email("plumber-reg@example.com")
            .await?
            .expect("user");
        assert!(stored.is_email_verified);
        assert!(stored.email_verification_token_hash.is_none());

        let row: (String, String, i32) = sqlx::query_as(
            "SELECT full_name, phone, years_of_experience FROM plumber_profiles WHERE user_id = $1",
        )
        .bind(res.user.id)
        .fetch_one(state.users.pool())
        .await?;
        assert_eq!(row.0, "Jane Doe");
        assert_eq!(row.1, "+12345678901");
        assert_eq!(row.2, 5);

        Ok(())
    }
}
