use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::modules::users::{
    CreateRefreshSessionParams, CreateUserParams, RefreshTokenRepository, Role, UserRepository,
};
use crate::AppState;

use super::auth_context::AuthContext;
use super::dto::{
    LoginRequest, LoginResponse, RegisterClientRequest, RegisterClientResponse,
    RegisterPlumberRequest, RegisterPlumberResponse, PlumberProfileResponse, UserResponse,
};
use super::error::AuthError;
use super::login_error::LoginError;
use super::passwords::{hash_password, normalize_email, verify_password};
use super::refresh_error::RefreshError;
use super::refresh_token_hash::{
    hash_refresh_jwt_for_storage, refresh_token_hash_hex_eq_constant_time,
};
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

pub struct LoginSuccess {
    pub response: LoginResponse,
    pub refresh_jwt: String,
}

/// Default policy: allow login when `!is_email_verified`; reject `!is_active` with [`LoginError::AccountInactive`].
pub async fn login(state: &AppState, body: LoginRequest) -> Result<LoginSuccess, LoginError> {
    let email = match normalize_email(&body.email) {
        Ok(e) => e,
        Err(AuthError::InvalidEmail) => {
            return Err(LoginError::Validation {
                message: "invalid email".to_string(),
            });
        }
        Err(_) => return Err(LoginError::Internal),
    };

    let user = state
        .users
        .find_by_email(&email)
        .await
        .map_err(|_| LoginError::Internal)?;
    let Some(user) = user else {
        return Err(LoginError::InvalidCredentials);
    };

    // Same 401 body for wrong password and verify failures (per Step 6 spec / plan).
    match verify_password(&body.password, user.password_hash()) {
        Ok(true) => {}
        Ok(false) | Err(_) => return Err(LoginError::InvalidCredentials),
    }

    if !user.is_active {
        return Err(LoginError::AccountInactive);
    }

    let jti = Uuid::new_v4().to_string();
    let refresh_jwt = state
        .jwt_config
        .create_refresh_token(user.id, user.role, &jti)
        .map_err(|_| LoginError::Internal)?;

    let token_hash = hash_refresh_jwt_for_storage(state.jwt_config.refresh_secret(), &refresh_jwt)
        .map_err(|_| LoginError::Internal)?;

    let expires_at = Utc::now() + Duration::seconds(state.jwt_config.refresh_ttl_secs());

    state
        .refresh_tokens
        .create_refresh_session(CreateRefreshSessionParams {
            user_id: user.id,
            jti: &jti,
            token_hash: &token_hash,
            expires_at,
        })
        .await
        .map_err(|_| LoginError::Internal)?;

    let access_token = state
        .jwt_config
        .create_access_token(user.id, user.role)
        .map_err(|_| LoginError::Internal)?;

    let expires_in = u64::try_from(state.jwt_config.access_ttl_secs()).unwrap_or(u64::MAX);

    Ok(LoginSuccess {
        response: LoginResponse {
            access_token,
            token_type: "Bearer",
            expires_in,
        },
        refresh_jwt,
    })
}

/// Rotate refresh session and mint new access + refresh JWTs (Step 9).
pub async fn refresh(state: &AppState, raw_refresh_jwt: &str) -> Result<LoginSuccess, RefreshError> {
    let claims = state
        .jwt_config
        .verify_refresh_token(raw_refresh_jwt)
        .map_err(|_| RefreshError::Unauthorized)?;

    let ctx = AuthContext::from_claims(&claims).map_err(|_| RefreshError::Unauthorized)?;

    let row = state
        .refresh_tokens
        .find_active_by_jti(&claims.jti)
        .await
        .map_err(|_| RefreshError::Internal)?;
    let Some(row) = row else {
        return Err(RefreshError::Unauthorized);
    };

    if row.user_id != ctx.user_id {
        return Err(RefreshError::Unauthorized);
    }

    let recomputed = hash_refresh_jwt_for_storage(state.jwt_config.refresh_secret(), raw_refresh_jwt)
        .map_err(|_| RefreshError::Internal)?;
    if !refresh_token_hash_hex_eq_constant_time(&row.token_hash, &recomputed) {
        return Err(RefreshError::Unauthorized);
    }

    let mut tx = state.pool.begin().await.map_err(|_| RefreshError::Internal)?;
    let revoked = RefreshTokenRepository::revoke_by_jti_with(&mut *tx, &claims.jti)
        .await
        .map_err(|_| RefreshError::Internal)?;
    if !revoked {
        let _ = tx.rollback().await;
        return Err(RefreshError::Unauthorized);
    }

    let jti_new = Uuid::new_v4().to_string();
    let refresh_jwt_new = state
        .jwt_config
        .create_refresh_token(ctx.user_id, ctx.role, &jti_new)
        .map_err(|_| RefreshError::Internal)?;
    let token_hash_new =
        hash_refresh_jwt_for_storage(state.jwt_config.refresh_secret(), &refresh_jwt_new)
            .map_err(|_| RefreshError::Internal)?;
    let expires_at = Utc::now() + Duration::seconds(state.jwt_config.refresh_ttl_secs());

    RefreshTokenRepository::create_refresh_session_with(
        &mut *tx,
        CreateRefreshSessionParams {
            user_id: ctx.user_id,
            jti: &jti_new,
            token_hash: &token_hash_new,
            expires_at,
        },
    )
    .await
    .map_err(|_| RefreshError::Internal)?;

    tx.commit().await.map_err(|_| RefreshError::Internal)?;

    let access_token = state
        .jwt_config
        .create_access_token(ctx.user_id, ctx.role)
        .map_err(|_| RefreshError::Internal)?;

    let expires_in = u64::try_from(state.jwt_config.access_ttl_secs()).unwrap_or(u64::MAX);

    Ok(LoginSuccess {
        response: LoginResponse {
            access_token,
            token_type: "Bearer",
            expires_in,
        },
        refresh_jwt: refresh_jwt_new,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::auth::dto::{RegisterClientRequest, RegisterPlumberRequest};
    use crate::modules::auth::passwords::PasswordConfig;
    use crate::modules::auth::cookie_config::CookieConfig;
    use crate::modules::auth::service_token::JwtConfig;
    use crate::modules::auth::verification::EmailVerificationConfig;
    use crate::modules::users::{RefreshTokenRepository, UserRepository};
    use sqlx::PgPool;

    fn test_app_state(pool: PgPool) -> AppState {
        AppState {
            pool: pool.clone(),
            users: UserRepository::new(pool.clone()),
            refresh_tokens: RefreshTokenRepository::new(pool.clone()),
            password_config: PasswordConfig::from_env(),
            email_verification: EmailVerificationConfig {
                secret: "integration-test-hmac-key".to_string(),
                ttl_hours: 48,
            },
            jwt_config: JwtConfig::from_env(),
            cookie_config: CookieConfig::from_env(),
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

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test login_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn login_success_persists_refresh_session(pool: PgPool) -> sqlx::Result<()> {
        use crate::modules::auth::dto::LoginRequest;
        use crate::modules::users::Role;

        let state = test_app_state(pool);
        let ph = hash_password("password123", &state.password_config).unwrap();
        state
            .users
            .create_user(CreateUserParams {
                email: "login-ok@example.com",
                password_hash: &ph,
                role: Role::Client,
                is_active: true,
                is_email_verified: false,
                email_verification_token_hash: None,
                email_verification_expires_at: None,
            })
            .await?;

        let out = login(
            &state,
            LoginRequest {
                email: "login-ok@example.com".to_string(),
                password: "password123".to_string(),
            },
        )
        .await
        .expect("login");

        assert!(!out.response.access_token.is_empty());
        assert_eq!(out.response.token_type, "Bearer");
        assert!(out.response.expires_in > 0);

        let claims = state
            .jwt_config
            .verify_refresh_token(&out.refresh_jwt)
            .expect("refresh jwt");
        let session = state
            .refresh_tokens
            .find_active_by_jti(&claims.jti)
            .await?
            .expect("db session");
        assert_eq!(session.user_id.to_string(), claims.sub);

        let cookie_str = state
            .cookie_config
            .refresh_set_cookie_string(&out.refresh_jwt, state.jwt_config.refresh_ttl_secs())
            .expect("cookie");
        assert!(
            cookie_str.to_lowercase().contains("httponly"),
            "cookie: {cookie_str}"
        );
        assert!(cookie_str.contains(&state.cookie_config.refresh_cookie_name));

        Ok(())
    }

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test refresh_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn refresh_rotates_and_old_refresh_rejected(pool: PgPool) -> sqlx::Result<()> {
        use crate::modules::auth::dto::LoginRequest;
        use crate::modules::auth::refresh_error::RefreshError;
        use crate::modules::users::Role;

        let state = test_app_state(pool);
        let ph = hash_password("password123", &state.password_config).unwrap();
        state
            .users
            .create_user(CreateUserParams {
                email: "refresh-rotate@example.com",
                password_hash: &ph,
                role: Role::Client,
                is_active: true,
                is_email_verified: true,
                email_verification_token_hash: None,
                email_verification_expires_at: None,
            })
            .await?;

        let login_out = login(
            &state,
            LoginRequest {
                email: "refresh-rotate@example.com".to_string(),
                password: "password123".to_string(),
            },
        )
        .await
        .expect("login");

        let old_refresh = login_out.refresh_jwt.clone();
        let refresh_out = refresh(&state, &old_refresh).await.expect("refresh");
        assert_ne!(
            refresh_out.response.access_token,
            login_out.response.access_token
        );
        assert_ne!(refresh_out.refresh_jwt, old_refresh);

        let replay = refresh(&state, &old_refresh).await;
        assert!(matches!(replay, Err(RefreshError::Unauthorized)));

        let chain = refresh(&state, &refresh_out.refresh_jwt)
            .await
            .expect("chain refresh");
        assert!(!chain.response.access_token.is_empty());

        Ok(())
    }

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test login_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn login_wrong_password_matches_unknown_email_body(pool: PgPool) -> sqlx::Result<()> {
        use axum::response::IntoResponse;
        use crate::modules::auth::dto::LoginRequest;
        use crate::modules::users::Role;

        let state = test_app_state(pool);
        let ph = hash_password("password123", &state.password_config).unwrap();
        state
            .users
            .create_user(CreateUserParams {
                email: "login-user@example.com",
                password_hash: &ph,
                role: Role::Client,
                is_active: true,
                is_email_verified: true,
                email_verification_token_hash: None,
                email_verification_expires_at: None,
            })
            .await?;

        let e1 = login(
            &state,
            LoginRequest {
                email: "login-user@example.com".to_string(),
                password: "wrong-password-xx".to_string(),
            },
        )
        .await
        .err()
        .expect("wrong password");
        let e2 = login(
            &state,
            LoginRequest {
                email: "no-such-user@example.com".to_string(),
                password: "password123".to_string(),
            },
        )
        .await
        .err()
        .expect("unknown user");

        let r1 = e1.into_response();
        let r2 = e2.into_response();
        assert_eq!(r1.status(), r2.status());
        let b1 = axum::body::to_bytes(r1.into_body(), usize::MAX)
            .await
            .unwrap();
        let b2 = axum::body::to_bytes(r2.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(b1, b2);

        Ok(())
    }

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test login_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn login_inactive_user_forbidden(pool: PgPool) -> sqlx::Result<()> {
        use axum::http::StatusCode;
        use axum::response::IntoResponse;
        use crate::modules::auth::dto::LoginRequest;
        use crate::modules::users::Role;

        let state = test_app_state(pool);
        let ph = hash_password("password123", &state.password_config).unwrap();
        state
            .users
            .create_user(CreateUserParams {
                email: "inactive@example.com",
                password_hash: &ph,
                role: Role::Client,
                is_active: false,
                is_email_verified: true,
                email_verification_token_hash: None,
                email_verification_expires_at: None,
            })
            .await?;

        let err = login(
            &state,
            LoginRequest {
                email: "inactive@example.com".to_string(),
                password: "password123".to_string(),
            },
        )
        .await
        .err()
        .expect("inactive");

        assert_eq!(err.into_response().status(), StatusCode::FORBIDDEN);
        Ok(())
    }
}
