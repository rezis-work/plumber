use sqlx::{Executor, PgPool, Postgres};
use uuid::Uuid;

use super::model::{CreateRefreshSessionParams, RefreshTokenRecord};

const RT_COLUMNS: &str = r#"
    id, user_id, jti, token_hash, expires_at, revoked_at, created_at
"#;

#[derive(Clone)]
pub struct RefreshTokenRepository {
    pool: PgPool,
}

impl RefreshTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn create_refresh_session(
        &self,
        params: CreateRefreshSessionParams<'_>,
    ) -> Result<RefreshTokenRecord, sqlx::Error> {
        Self::create_refresh_session_with(&self.pool, params).await
    }

    pub async fn create_refresh_session_with<'e, E>(
        executor: E,
        params: CreateRefreshSessionParams<'_>,
    ) -> Result<RefreshTokenRecord, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let q = format!(
            r#"
            INSERT INTO refresh_tokens (user_id, jti, token_hash, expires_at)
            VALUES ($1, $2, $3, $4)
            RETURNING {RT_COLUMNS}
            "#
        );
        sqlx::query_as::<_, RefreshTokenRecord>(&q)
            .bind(params.user_id)
            .bind(params.jti)
            .bind(params.token_hash)
            .bind(params.expires_at)
            .fetch_one(executor)
            .await
    }

    pub async fn find_active_by_jti(
        &self,
        jti: &str,
    ) -> Result<Option<RefreshTokenRecord>, sqlx::Error> {
        let q = format!(
            r#"
            SELECT {RT_COLUMNS}
            FROM refresh_tokens
            WHERE jti = $1
              AND revoked_at IS NULL
              AND expires_at > now()
            "#
        );
        sqlx::query_as::<_, RefreshTokenRecord>(&q)
            .bind(jti)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn revoke_by_jti(&self, jti: &str) -> Result<bool, sqlx::Error> {
        Self::revoke_by_jti_with(&self.pool, jti).await
    }

    pub async fn revoke_by_jti_with<'e, E>(executor: E, jti: &str) -> Result<bool, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let r = sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = now()
            WHERE jti = $1 AND revoked_at IS NULL
            "#,
        )
        .bind(jti)
        .execute(executor)
        .await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn revoke_all_for_user(&self, user_id: Uuid) -> Result<u64, sqlx::Error> {
        let r = sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = now()
            WHERE user_id = $1 AND revoked_at IS NULL
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        Ok(r.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::modules::auth::refresh_token_hash::hash_refresh_jwt_for_storage;
    use crate::modules::users::repository::{CreateUserParams, UserRepository};

    use super::super::model::{Role, UserStatus};

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test refresh_token -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn create_find_revoke_revoke_all_flow(pool: PgPool) -> sqlx::Result<()> {
        const SECRET: &str = "test-refresh-hmac-secret";

        let users = UserRepository::new(pool.clone());
        let repo = RefreshTokenRepository::new(pool);

        let user = users
            .create_user(CreateUserParams {
                email: "refresh-test@example.com",
                password_hash: "ph",
                role: Role::Client,
                user_status: UserStatus::Active,
                is_email_verified: false,
                email_verification_token_hash: None,
                email_verification_expires_at: None,
            })
            .await?;

        let jti = Uuid::new_v4().to_string();
        let raw_jwt = "eyJhbGci.test.signature";
        let token_hash = hash_refresh_jwt_for_storage(SECRET, raw_jwt).expect("hash");
        let expires_at = Utc::now() + chrono::Duration::hours(24);

        let row = repo
            .create_refresh_session(CreateRefreshSessionParams {
                user_id: user.id,
                jti: &jti,
                token_hash: &token_hash,
                expires_at,
            })
            .await?;

        assert_eq!(row.jti, jti);
        assert_eq!(row.token_hash, token_hash);

        let found = repo.find_active_by_jti(&jti).await?.expect("active session");
        assert_eq!(found.id, row.id);

        assert!(repo.revoke_by_jti(&jti).await?);
        assert!(repo.find_active_by_jti(&jti).await?.is_none());
        assert!(!repo.revoke_by_jti(&jti).await?);

        let jti_a = Uuid::new_v4().to_string();
        let jti_b = Uuid::new_v4().to_string();
        let h_a = hash_refresh_jwt_for_storage(SECRET, "jwt.a").unwrap();
        let h_b = hash_refresh_jwt_for_storage(SECRET, "jwt.b").unwrap();
        let exp = Utc::now() + chrono::Duration::hours(1);

        repo.create_refresh_session(CreateRefreshSessionParams {
            user_id: user.id,
            jti: &jti_a,
            token_hash: &h_a,
            expires_at: exp,
        })
        .await?;
        repo.create_refresh_session(CreateRefreshSessionParams {
            user_id: user.id,
            jti: &jti_b,
            token_hash: &h_b,
            expires_at: exp,
        })
        .await?;

        assert!(repo.find_active_by_jti(&jti_a).await?.is_some());
        assert!(repo.find_active_by_jti(&jti_b).await?.is_some());

        assert_eq!(repo.revoke_all_for_user(user.id).await?, 2);
        assert!(repo.find_active_by_jti(&jti_a).await?.is_none());
        assert!(repo.find_active_by_jti(&jti_b).await?.is_none());

        Ok(())
    }
}
