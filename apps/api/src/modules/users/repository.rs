use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::model::{PlumberProfile, Role, User};

const USER_COLUMNS: &str = r#"
    id, email, password_hash, role, is_active, is_email_verified,
    email_verification_token_hash, email_verification_expires_at, created_at, updated_at
"#;

#[derive(Debug, Clone)]
pub struct CreateUserParams<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,
    pub role: Role,
    pub is_active: bool,
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
                email, password_hash, role, is_active,
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
            .bind(params.is_active)
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
                email, password_hash, role, is_active,
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
            .bind(params.is_active)
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
        sqlx::query_as::<_, PlumberProfile>(
            r#"
            INSERT INTO plumber_profiles (user_id, full_name, phone, years_of_experience)
            VALUES ($1, $2, $3, $4)
            RETURNING user_id, full_name, phone, years_of_experience
            "#,
        )
        .bind(user_id)
        .bind(full_name)
        .bind(phone)
        .bind(years_of_experience)
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn find_plumber_profile_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<PlumberProfile>, sqlx::Error> {
        sqlx::query_as::<_, PlumberProfile>(
            r#"
            SELECT user_id, full_name, phone, years_of_experience
            FROM plumber_profiles
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }
}

#[cfg(test)]
mod tests {
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
                is_active: true,
                is_email_verified: false,
                email_verification_token_hash: None,
                email_verification_expires_at: None,
            })
            .await?;

        assert_eq!(created.email, "test@example.com");
        assert_eq!(created.password_hash(), "dummy_hash_not_real");
        assert_eq!(created.role, Role::Client);
        assert!(created.is_active);
        assert!(!created.is_email_verified);

        let by_email = repo
            .find_by_email("test@example.com")
            .await?
            .expect("user by email");
        assert_eq!(by_email.id, created.id);

        let by_id = repo.find_by_id(created.id).await?.expect("user by id");
        assert_eq!(by_id.email, "test@example.com");

        Ok(())
    }
}
