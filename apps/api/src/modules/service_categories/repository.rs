use sqlx::PgPool;
use uuid::Uuid;

use super::model::ServiceCategory;

#[derive(Clone)]
pub struct ServiceCategoryRepository {
    pool: PgPool,
}

impl ServiceCategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<ServiceCategory>, sqlx::Error> {
        sqlx::query_as::<_, ServiceCategory>(
            r#"
            SELECT id, name, slug, description, icon, is_active, sort_order, created_at, updated_at
            FROM service_categories
            WHERE slug = $1
            "#,
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<ServiceCategory>, sqlx::Error> {
        sqlx::query_as::<_, ServiceCategory>(
            r#"
            SELECT id, name, slug, description, icon, is_active, sort_order, created_at, updated_at
            FROM service_categories
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// When `include_inactive` is false, only active rows are returned (public-style).
    pub async fn list(&self, include_inactive: bool) -> Result<Vec<ServiceCategory>, sqlx::Error> {
        sqlx::query_as::<_, ServiceCategory>(
            r#"
            SELECT id, name, slug, description, icon, is_active, sort_order, created_at, updated_at
            FROM service_categories
            WHERE ($1 OR is_active = true)
            ORDER BY sort_order, name
            "#,
        )
        .bind(include_inactive)
        .fetch_all(&self.pool)
        .await
    }
}
