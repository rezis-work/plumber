use sqlx::PgPool;
use uuid::Uuid;

use super::model::ServicePriceGuide;

#[derive(Clone)]
pub struct ServicePriceGuideRepository {
    pool: PgPool,
}

impl ServicePriceGuideRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<ServicePriceGuide>, sqlx::Error> {
        sqlx::query_as::<_, ServicePriceGuide>(
            r#"
            SELECT id, service_category_id, city_id, area_id, min_price, max_price, currency,
                   estimated_duration_minutes, is_emergency_supported, notes, created_at, updated_at
            FROM service_price_guides
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn list_by_service_category_id(
        &self,
        service_category_id: Uuid,
    ) -> Result<Vec<ServicePriceGuide>, sqlx::Error> {
        sqlx::query_as::<_, ServicePriceGuide>(
            r#"
            SELECT id, service_category_id, city_id, area_id, min_price, max_price, currency,
                   estimated_duration_minutes, is_emergency_supported, notes, created_at, updated_at
            FROM service_price_guides
            WHERE service_category_id = $1
            ORDER BY city_id NULLS FIRST, area_id NULLS FIRST, created_at
            "#,
        )
        .bind(service_category_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Exact scope match: `(None, None)` = global; `(Some(city), None)` = city default;
    /// `(Some(city), Some(area))` = area-specific. `area_id` without `city_id` is invalid per DB CHECK.
    pub async fn find_exact_scope(
        &self,
        service_category_id: Uuid,
        city_id: Option<Uuid>,
        area_id: Option<Uuid>,
    ) -> Result<Option<ServicePriceGuide>, sqlx::Error> {
        sqlx::query_as::<_, ServicePriceGuide>(
            r#"
            SELECT id, service_category_id, city_id, area_id, min_price, max_price, currency,
                   estimated_duration_minutes, is_emergency_supported, notes, created_at, updated_at
            FROM service_price_guides
            WHERE service_category_id = $1
              AND (
                  ($2::uuid IS NULL AND $3::uuid IS NULL AND city_id IS NULL AND area_id IS NULL)
                  OR ($2::uuid IS NOT NULL AND $3::uuid IS NULL AND city_id = $2 AND area_id IS NULL)
                  OR ($2::uuid IS NOT NULL AND $3::uuid IS NOT NULL AND city_id = $2 AND area_id = $3)
              )
            "#,
        )
        .bind(service_category_id)
        .bind(city_id)
        .bind(area_id)
        .fetch_optional(&self.pool)
        .await
    }
}
