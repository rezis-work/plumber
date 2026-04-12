use sqlx::PgPool;
use uuid::Uuid;

use super::model::PlumberServiceArea;

#[derive(Clone)]
pub struct PlumberServiceAreaRepository {
    pool: PgPool,
}

impl PlumberServiceAreaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn list_by_plumber_id(
        &self,
        plumber_id: Uuid,
    ) -> Result<Vec<PlumberServiceArea>, sqlx::Error> {
        sqlx::query_as::<_, PlumberServiceArea>(
            r#"
            SELECT id, plumber_id, city_id, area_id, created_at
            FROM plumber_service_areas
            WHERE plumber_id = $1
            ORDER BY city_id, area_id NULLS FIRST, created_at
            "#,
        )
        .bind(plumber_id)
        .fetch_all(&self.pool)
        .await
    }

    /// `area_id` `None` matches a whole-city row (`area_id IS NULL` in the database).
    pub async fn find_by_plumber_city_and_area(
        &self,
        plumber_id: Uuid,
        city_id: Uuid,
        area_id: Option<Uuid>,
    ) -> Result<Option<PlumberServiceArea>, sqlx::Error> {
        sqlx::query_as::<_, PlumberServiceArea>(
            r#"
            SELECT id, plumber_id, city_id, area_id, created_at
            FROM plumber_service_areas
            WHERE plumber_id = $1
              AND city_id = $2
              AND (
                  ($3::uuid IS NULL AND area_id IS NULL)
                  OR area_id = $3
              )
            "#,
        )
        .bind(plumber_id)
        .bind(city_id)
        .bind(area_id)
        .fetch_optional(&self.pool)
        .await
    }
}
