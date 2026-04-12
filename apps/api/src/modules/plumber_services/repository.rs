use sqlx::PgPool;
use uuid::Uuid;

use super::model::PlumberService;

#[derive(Clone)]
pub struct PlumberServiceRepository {
    pool: PgPool,
}

impl PlumberServiceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn list_by_plumber_id(
        &self,
        plumber_id: Uuid,
    ) -> Result<Vec<PlumberService>, sqlx::Error> {
        sqlx::query_as::<_, PlumberService>(
            r#"
            SELECT id, plumber_id, service_category_id, created_at
            FROM plumber_services
            WHERE plumber_id = $1
            ORDER BY created_at
            "#,
        )
        .bind(plumber_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn list_plumber_ids_by_service_category_id(
        &self,
        service_category_id: Uuid,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        sqlx::query_scalar::<_, Uuid>(
            r#"
            SELECT plumber_id
            FROM plumber_services
            WHERE service_category_id = $1
            ORDER BY plumber_id
            "#,
        )
        .bind(service_category_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_by_plumber_and_category(
        &self,
        plumber_id: Uuid,
        service_category_id: Uuid,
    ) -> Result<Option<PlumberService>, sqlx::Error> {
        sqlx::query_as::<_, PlumberService>(
            r#"
            SELECT id, plumber_id, service_category_id, created_at
            FROM plumber_services
            WHERE plumber_id = $1 AND service_category_id = $2
            "#,
        )
        .bind(plumber_id)
        .bind(service_category_id)
        .fetch_optional(&self.pool)
        .await
    }
}
