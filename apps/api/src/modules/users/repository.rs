use sqlx::PgPool;
use uuid::Uuid;

use super::model::{Role, User};

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, password_hash, role, is_active, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, password_hash, role, is_active, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create_user(
        &self,
        email: &str,
        password_hash: &str,
        role: Role,
        is_active: bool,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, password_hash, role, is_active)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, password_hash, role, is_active, created_at, updated_at
            "#,
        )
        .bind(email)
        .bind(password_hash)
        .bind(role)
        .bind(is_active)
        .fetch_one(&self.pool)
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
            .create_user(
                "test@example.com",
                "dummy_hash_not_real",
                Role::Client,
                true,
            )
            .await?;

        assert_eq!(created.email, "test@example.com");
        assert_eq!(created.password_hash(), "dummy_hash_not_real");
        assert_eq!(created.role, Role::Client);
        assert!(created.is_active);

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
