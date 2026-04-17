use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::modules::users::{
    CreateRefreshSessionParams, CreateUserParams, RefreshTokenRepository, Role, UserRepository,
    UserStatus,
};
use crate::AppState;

use super::admin_user_error::AdminUserError;
use super::auth_context::AuthContext;
use super::dto::{
    LoginRequest, LoginResponse, MeResponse, RegisterClientRequest, RegisterClientResponse,
    RegisterPlumberRequest, RegisterPlumberResponse, PlumberProfileResponse, UserResponse,
    VerifyEmailRequest, VerifyEmailResponse,
};
use super::error::AuthError;
use super::login_error::LoginError;
use super::me_error::MeError;
use super::passwords::{hash_password, normalize_email, verify_password};
use super::logout_error::LogoutError;
use super::refresh_error::RefreshError;
use super::refresh_token_hash::{
    hash_refresh_jwt_for_storage, refresh_token_hash_hex_eq_constant_time,
};
use super::register_error::RegisterError;
use super::verify_email_error::VerifyEmailError;
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

fn validate_verification_token_hex(raw: &str) -> Result<&str, VerifyEmailError> {
    let t = raw.trim();
    if t.is_empty() {
        return Err(VerifyEmailError::Validation {
            message: "token is required".to_string(),
        });
    }
    if t.len() != 64 || !t.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(VerifyEmailError::Validation {
            message: "invalid verification token".to_string(),
        });
    }
    Ok(t)
}

pub async fn verify_email(
    state: &AppState,
    body: VerifyEmailRequest,
) -> Result<VerifyEmailResponse, VerifyEmailError> {
    let token = validate_verification_token_hex(&body.token)?;
    let token_hash = state
        .email_verification
        .hash_raw_token_hex(token)
        .map_err(|_| VerifyEmailError::Validation {
            message: "invalid verification token".to_string(),
        })?;

    let user = state
        .users
        .find_by_email_verification_token_hash(&token_hash)
        .await
        .map_err(|_| VerifyEmailError::Internal)?
        .ok_or(VerifyEmailError::InvalidToken)?;

    if user.is_email_verified {
        return Ok(VerifyEmailResponse {
            verified: false,
            already_verified: true,
        });
    }

    let Some(expires_at) = user.email_verification_expires_at else {
        return Err(VerifyEmailError::InvalidToken);
    };

    if Utc::now() > expires_at {
        return Err(VerifyEmailError::TokenExpired);
    }

    state
        .users
        .mark_email_verified_clear_verification(user.id)
        .await
        .map_err(|_| VerifyEmailError::Internal)?;

    Ok(VerifyEmailResponse {
        verified: true,
        already_verified: false,
    })
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
            user_status: UserStatus::Active,
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
            user_status: UserStatus::Active,
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
        profile: PlumberProfileResponse::from(profile),
    })
}

pub struct LoginSuccess {
    pub response: LoginResponse,
    pub refresh_jwt: String,
}

/// Default policy: allow login when `!is_email_verified`; reject non-active or soft-deleted with [`LoginError::AccountInactive`].
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

    if !user.login_allowed() {
        return Err(LoginError::AccountInactive);
    }

    state
        .users
        .touch_last_login_at(user.id)
        .await
        .map_err(|_| LoginError::Internal)?;

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

    let user = state
        .users
        .find_by_id(ctx.user_id)
        .await
        .map_err(|_| RefreshError::Internal)?;
    let Some(user) = user else {
        return Err(RefreshError::Unauthorized);
    };
    if !user.login_allowed() {
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

/// Revoke refresh session when cookie JWT verifies (Step 10). Always ok for missing/invalid cookie (idempotent).
pub async fn logout(state: &AppState, raw_refresh_jwt: Option<&str>) -> Result<(), LogoutError> {
    if let Some(raw) = raw_refresh_jwt {
        if let Ok(claims) = state.jwt_config.verify_refresh_token(raw) {
            state
                .refresh_tokens
                .revoke_by_jti(&claims.jti)
                .await
                .map_err(|_| LogoutError::Internal)?;
        }
    }
    Ok(())
}

/// Revoke every refresh session for the user (Step 11). Returns count of rows updated.
pub async fn logout_all(state: &AppState, user_id: Uuid) -> Result<u64, LogoutError> {
    state
        .refresh_tokens
        .revoke_all_for_user(user_id)
        .await
        .map_err(|_| LogoutError::Internal)
}

/// Current user for `GET /auth/me` (Step 12): DB row + plumber profile when applicable.
pub async fn me_profile(state: &AppState, user_id: Uuid) -> Result<MeResponse, MeError> {
    let Some(user) = state
        .users
        .find_by_id(user_id)
        .await
        .map_err(|_| MeError::Internal)?
    else {
        return Err(MeError::NotFound);
    };

    let profile = if user.role == Role::Plumber {
        state
            .users
            .find_plumber_profile_by_user_id(user.id)
            .await
            .map_err(|_| MeError::Internal)?
            .map(PlumberProfileResponse::from)
    } else {
        None
    };

    let is_active = user.login_allowed();
    Ok(MeResponse {
        id: user.id,
        email: user.email,
        role: user.role,
        status: user.user_status,
        is_active,
        is_email_verified: user.is_email_verified,
        created_at: user.created_at,
        updated_at: user.updated_at,
        profile,
    })
}

pub async fn admin_block_user(state: &AppState, target_id: Uuid) -> Result<(), AdminUserError> {
    let ok = state
        .users
        .set_user_blocked(target_id)
        .await
        .map_err(|_| AdminUserError::Internal)?;
    if !ok {
        return Err(AdminUserError::NotFound);
    }
    Ok(())
}

pub async fn admin_soft_delete_user(state: &AppState, target_id: Uuid) -> Result<(), AdminUserError> {
    let ok = state
        .users
        .soft_delete_user(target_id)
        .await
        .map_err(|_| AdminUserError::Internal)?;
    if !ok {
        return Err(AdminUserError::NotFound);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::auth::dto::{RegisterClientRequest, RegisterPlumberRequest};
    use crate::modules::auth::passwords::PasswordConfig;
    use crate::modules::auth::cookie_config::CookieConfig;
    use crate::modules::auth::service_token::JwtConfig;
    use crate::modules::auth::verification::EmailVerificationConfig;
    use crate::modules::geography::GeographyRepository;
    use crate::modules::orders::OrderRepository;
    use crate::modules::service_categories::ServiceCategoryRepository;
    use crate::modules::users::{RefreshTokenRepository, UserRepository, UserStatus};
    use sqlx::PgPool;

    fn test_app_state(pool: PgPool) -> AppState {
        AppState {
            pool: pool.clone(),
            users: UserRepository::new(pool.clone()),
            orders: OrderRepository::new(pool.clone()),
            geography: GeographyRepository::new(pool.clone()),
            service_categories: ServiceCategoryRepository::new(pool.clone()),
            refresh_tokens: RefreshTokenRepository::new(pool.clone()),
            password_config: PasswordConfig::from_env(),
            email_verification: EmailVerificationConfig {
                secret: "integration-test-hmac-key".to_string(),
                ttl_hours: 48,
            },
            jwt_config: JwtConfig::from_env(),
            cookie_config: CookieConfig::from_env(),
            redis_dispatch: None,
            dispatch_advance_secret: None,
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
            "SELECT full_name, phone, experience_years FROM plumber_profiles WHERE user_id = $1",
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
                user_status: UserStatus::Active,
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

        let u = state
            .users
            .find_by_email("login-ok@example.com")
            .await?
            .expect("user after login");
        assert!(u.last_login_at.is_some());

        Ok(())
    }

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test refresh_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn refresh_rejected_when_user_blocked_after_login(pool: PgPool) -> sqlx::Result<()> {
        use crate::modules::auth::dto::LoginRequest;
        use crate::modules::auth::refresh_error::RefreshError;
        use crate::modules::users::Role;

        let state = test_app_state(pool);
        let ph = hash_password("password123", &state.password_config).unwrap();
        let user = state
            .users
            .create_user(CreateUserParams {
                email: "refresh-blocked@example.com",
                password_hash: &ph,
                role: Role::Client,
                user_status: UserStatus::Active,
                is_email_verified: true,
                email_verification_token_hash: None,
                email_verification_expires_at: None,
            })
            .await?;

        let login_out = login(
            &state,
            LoginRequest {
                email: "refresh-blocked@example.com".to_string(),
                password: "password123".to_string(),
            },
        )
        .await
        .expect("login");

        sqlx::query(
            "UPDATE users SET user_status = 'blocked'::user_status WHERE id = $1",
        )
        .bind(user.id)
        .execute(state.users.pool())
        .await?;

        let replay = refresh(&state, &login_out.refresh_jwt).await;
        assert!(matches!(replay, Err(RefreshError::Unauthorized)));

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
                user_status: UserStatus::Active,
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

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test logout_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn logout_revokes_refresh_second_logout_idempotent(pool: PgPool) -> sqlx::Result<()> {
        use crate::modules::auth::dto::LoginRequest;
        use crate::modules::auth::refresh_error::RefreshError;
        use crate::modules::users::Role;

        let state = test_app_state(pool);
        let ph = hash_password("password123", &state.password_config).unwrap();
        state
            .users
            .create_user(CreateUserParams {
                email: "logout-user@example.com",
                password_hash: &ph,
                role: Role::Client,
                user_status: UserStatus::Active,
                is_email_verified: true,
                email_verification_token_hash: None,
                email_verification_expires_at: None,
            })
            .await?;

        let login_out = login(
            &state,
            LoginRequest {
                email: "logout-user@example.com".to_string(),
                password: "password123".to_string(),
            },
        )
        .await
        .expect("login");

        let refresh_jwt = login_out.refresh_jwt.clone();
        logout(&state, Some(refresh_jwt.as_str()))
            .await
            .expect("logout with valid refresh");

        let after = refresh(&state, &refresh_jwt).await;
        assert!(matches!(after, Err(RefreshError::Unauthorized)));

        logout(&state, None).await.expect("logout without cookie jwt");
        logout(&state, Some("not-a-jwt"))
            .await
            .expect("logout with invalid jwt");

        Ok(())
    }

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test logout_all_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn logout_all_revokes_two_sessions(pool: PgPool) -> sqlx::Result<()> {
        use crate::modules::auth::dto::LoginRequest;
        use crate::modules::auth::refresh_error::RefreshError;
        use crate::modules::users::Role;

        let state = test_app_state(pool);
        let ph = hash_password("password123", &state.password_config).unwrap();
        state
            .users
            .create_user(CreateUserParams {
                email: "logout-all-user@example.com",
                password_hash: &ph,
                role: Role::Client,
                user_status: UserStatus::Active,
                is_email_verified: true,
                email_verification_token_hash: None,
                email_verification_expires_at: None,
            })
            .await?;

        let login_a = login(
            &state,
            LoginRequest {
                email: "logout-all-user@example.com".to_string(),
                password: "password123".to_string(),
            },
        )
        .await
        .expect("login a");
        let refresh_a = login_a.refresh_jwt.clone();

        let login_b = login(
            &state,
            LoginRequest {
                email: "logout-all-user@example.com".to_string(),
                password: "password123".to_string(),
            },
        )
        .await
        .expect("login b");
        let refresh_b = login_b.refresh_jwt;

        let user = state
            .users
            .find_by_email("logout-all-user@example.com")
            .await?
            .expect("user");

        let n = logout_all(&state, user.id).await.expect("logout_all");
        assert_eq!(n, 2);

        assert!(matches!(
            refresh(&state, &refresh_a).await,
            Err(RefreshError::Unauthorized)
        ));
        assert!(matches!(
            refresh(&state, &refresh_b).await,
            Err(RefreshError::Unauthorized)
        ));

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
                user_status: UserStatus::Active,
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
                user_status: UserStatus::Blocked,
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

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test verify_email_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn verify_email_success_clears_token(pool: PgPool) -> sqlx::Result<()> {
        use crate::modules::auth::dto::VerifyEmailRequest;

        let state = test_app_state(pool);
        let reg = register_client(
            &state,
            RegisterClientRequest {
                email: "verify-ok@example.com".to_string(),
                password: "password123".to_string(),
            },
        )
        .await
        .expect("register");

        let out = verify_email(
            &state,
            VerifyEmailRequest {
                token: reg.email_verification_token.clone(),
            },
        )
        .await
        .expect("verify");

        assert!(out.verified);
        assert!(!out.already_verified);

        let stored = state
            .users
            .find_by_email("verify-ok@example.com")
            .await?
            .unwrap();
        assert!(stored.is_email_verified);
        assert!(stored.email_verification_token_hash.is_none());

        Ok(())
    }

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test verify_email_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn verify_email_wrong_token_unauthorized(pool: PgPool) -> sqlx::Result<()> {
        use crate::modules::auth::dto::VerifyEmailRequest;
        use crate::modules::auth::verify_email_error::VerifyEmailError;

        let state = test_app_state(pool);
        let err = verify_email(
            &state,
            VerifyEmailRequest {
                token: "a".repeat(64),
            },
        )
        .await
        .err()
        .expect("wrong token");
        assert!(matches!(err, VerifyEmailError::InvalidToken));
        Ok(())
    }

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test verify_email_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn verify_email_expired_gone(pool: PgPool) -> sqlx::Result<()> {
        use crate::modules::auth::dto::VerifyEmailRequest;
        use crate::modules::auth::verify_email_error::VerifyEmailError;
        use crate::modules::users::Role;

        let state = test_app_state(pool);
        let raw_hex = EmailVerificationConfig::generate_raw_token_hex();
        let token_hash = state
            .email_verification
            .hash_raw_token_hex(&raw_hex)
            .expect("hash");
        let past = Utc::now() - Duration::hours(1);
        let ph = hash_password("password123", &state.password_config).unwrap();
        state
            .users
            .create_user(CreateUserParams {
                email: "verify-expired@example.com",
                password_hash: &ph,
                role: Role::Client,
                user_status: UserStatus::Active,
                is_email_verified: false,
                email_verification_token_hash: Some(&token_hash),
                email_verification_expires_at: Some(past),
            })
            .await?;

        let err = verify_email(
            &state,
            VerifyEmailRequest { token: raw_hex },
        )
        .await
        .err()
        .expect("expired");
        assert!(matches!(err, VerifyEmailError::TokenExpired));
        Ok(())
    }

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test verify_email_ -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn verify_email_idempotent_when_already_verified(pool: PgPool) -> sqlx::Result<()> {
        use crate::modules::auth::dto::VerifyEmailRequest;

        let state = test_app_state(pool);
        let reg = register_client(
            &state,
            RegisterClientRequest {
                email: "verify-idem@example.com".to_string(),
                password: "password123".to_string(),
            },
        )
        .await
        .expect("register");

        sqlx::query("UPDATE users SET is_email_verified = true WHERE email = $1")
            .bind("verify-idem@example.com")
            .execute(state.users.pool())
            .await?;

        let out = verify_email(
            &state,
            VerifyEmailRequest {
                token: reg.email_verification_token,
            },
        )
        .await
        .expect("idem");

        assert!(!out.verified);
        assert!(out.already_verified);

        Ok(())
    }
}
