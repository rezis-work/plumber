use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::model::{PlumberProfile, Role, User, UserStatus};

const PLUMBER_PROFILE_COLUMNS: &str = r#"
    id, user_id, full_name, phone, experience_years,
    bio, avatar_url, is_approved, approved_at, approved_by,
    is_online, is_available, current_city_id, current_area_id, current_street_id,
    current_lat, current_lng, service_radius_km, last_location_updated_at,
    rating_avg, rating_count, completed_orders_count, cancelled_orders_count,
    token_balance,
    created_at, updated_at
"#;

const USER_COLUMNS: &str = r#"
    id, email, password_hash, role, user_status, last_login_at, blocked_at, deleted_at,
    is_email_verified, email_verification_token_hash, email_verification_expires_at,
    created_at, updated_at
"#;

#[derive(Debug, Clone)]
pub struct CreateUserParams<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,
    pub role: Role,
    pub user_status: UserStatus,
    pub is_email_verified: bool,
    pub email_verification_token_hash: Option<&'a str>,
    pub email_verification_expires_at: Option<DateTime<Utc>>,
}

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let q = format!(
            "SELECT {USER_COLUMNS} FROM users WHERE email = $1"
        );
        sqlx::query_as::<_, User>(&q)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let q = format!("SELECT {USER_COLUMNS} FROM users WHERE id = $1");
        sqlx::query_as::<_, User>(&q)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn create_user(&self, params: CreateUserParams<'_>) -> Result<User, sqlx::Error> {
        let q = format!(
            r#"
            INSERT INTO users (
                email, password_hash, role, user_status,
                is_email_verified, email_verification_token_hash, email_verification_expires_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING {USER_COLUMNS}
            "#
        );
        sqlx::query_as::<_, User>(&q)
            .bind(params.email)
            .bind(params.password_hash)
            .bind(params.role)
            .bind(params.user_status)
            .bind(params.is_email_verified)
            .bind(params.email_verification_token_hash)
            .bind(params.email_verification_expires_at)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn create_user_tx<'e>(
        tx: &mut Transaction<'e, Postgres>,
        params: CreateUserParams<'_>,
    ) -> Result<User, sqlx::Error> {
        let q = format!(
            r#"
            INSERT INTO users (
                email, password_hash, role, user_status,
                is_email_verified, email_verification_token_hash, email_verification_expires_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING {USER_COLUMNS}
            "#
        );
        sqlx::query_as::<_, User>(&q)
            .bind(params.email)
            .bind(params.password_hash)
            .bind(params.role)
            .bind(params.user_status)
            .bind(params.is_email_verified)
            .bind(params.email_verification_token_hash)
            .bind(params.email_verification_expires_at)
            .fetch_one(&mut **tx)
            .await
    }

    pub async fn create_plumber_profile_tx<'e>(
        tx: &mut Transaction<'e, Postgres>,
        user_id: Uuid,
        full_name: &str,
        phone: &str,
        years_of_experience: i32,
    ) -> Result<PlumberProfile, sqlx::Error> {
        let q = format!(
            r#"
            INSERT INTO plumber_profiles (user_id, full_name, phone, experience_years)
            VALUES ($1, $2, $3, $4)
            RETURNING {PLUMBER_PROFILE_COLUMNS}
            "#
        );
        sqlx::query_as::<_, PlumberProfile>(&q)
        .bind(user_id)
        .bind(full_name)
        .bind(phone)
        .bind(years_of_experience)
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn find_by_email_verification_token_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let q = format!(
            "SELECT {USER_COLUMNS} FROM users WHERE email_verification_token_hash = $1"
        );
        sqlx::query_as::<_, User>(&q)
            .bind(token_hash)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn touch_last_login_at(&self, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE users SET last_login_at = NOW(), updated_at = NOW() WHERE id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Sets `user_status` to `blocked`. Sets `blocked_at` only when transitioning from a non-blocked status.
    pub async fn set_user_blocked(&self, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let r = sqlx::query(
            r#"
            UPDATE users SET
                user_status = 'blocked',
                blocked_at = CASE
                    WHEN user_status IS DISTINCT FROM 'blocked' THEN NOW()
                    ELSE blocked_at
                END,
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn soft_delete_user(&self, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let r = sqlx::query(
            r#"
            UPDATE users SET deleted_at = NOW(), updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn mark_email_verified_clear_verification(
        &self,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE users SET
                is_email_verified = true,
                email_verification_token_hash = NULL,
                email_verification_expires_at = NULL,
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_plumber_profile_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<PlumberProfile>, sqlx::Error> {
        let q = format!(
            r#"
            SELECT {PLUMBER_PROFILE_COLUMNS}
            FROM plumber_profiles
            WHERE user_id = $1
            "#
        );
        sqlx::query_as::<_, PlumberProfile>(&q)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::model::UserStatus;

    use super::*;

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn create_find_by_email_and_id(pool: PgPool) -> sqlx::Result<()> {
        let repo = UserRepository::new(pool);
        let created = repo
            .create_user(CreateUserParams {
                email: "test@example.com",
                password_hash: "dummy_hash_not_real",
                role: Role::Client,
                user_status: UserStatus::Active,
                is_email_verified: false,
                email_verification_token_hash: None,
                email_verification_expires_at: None,
            })
            .await?;

        assert_eq!(created.email, "test@example.com");
        assert_eq!(created.password_hash(), "dummy_hash_not_real");
        assert_eq!(created.role, Role::Client);
        assert_eq!(created.user_status, UserStatus::Active);
        assert!(created.login_allowed());
        assert!(!created.is_email_verified);

        let by_email = repo
            .find_by_email("test@example.com")
            .await?
            .expect("user by email");
        assert_eq!(by_email.id, created.id);

        let by_id = repo.find_by_id(created.id).await?.expect("user by id");
        assert_eq!(by_id.email, "test@example.com");

        assert!(by_id.last_login_at.is_none());
        repo.touch_last_login_at(created.id).await?;
        let touched = repo.find_by_id(created.id).await?.expect("user after touch");
        assert!(touched.last_login_at.is_some());

        Ok(())
    }
}
