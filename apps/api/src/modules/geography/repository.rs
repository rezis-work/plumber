use sqlx::PgPool;
use uuid::Uuid;

use super::model::{Area, City, Street};

#[derive(Clone)]
pub struct GeographyRepository {
    pool: PgPool,
}

impl GeographyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn find_city_by_slug(&self, slug: &str) -> Result<Option<City>, sqlx::Error> {
        sqlx::query_as::<_, City>(
            r#"
            SELECT id, name, slug, is_active, created_at, updated_at
            FROM cities
            WHERE slug = $1
            "#,
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_city_by_id(&self, id: Uuid) -> Result<Option<City>, sqlx::Error> {
        sqlx::query_as::<_, City>(
            r#"
            SELECT id, name, slug, is_active, created_at, updated_at
            FROM cities
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// When `include_inactive` is false, only active cities are returned (public-style).
    pub async fn list_cities(&self, include_inactive: bool) -> Result<Vec<City>, sqlx::Error> {
        sqlx::query_as::<_, City>(
            r#"
            SELECT id, name, slug, is_active, created_at, updated_at
            FROM cities
            WHERE ($1 OR is_active = true)
            ORDER BY name
            "#,
        )
        .bind(include_inactive)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn list_areas_by_city_id(
        &self,
        city_id: Uuid,
        include_inactive: bool,
    ) -> Result<Vec<Area>, sqlx::Error> {
        sqlx::query_as::<_, Area>(
            r#"
            SELECT id, city_id, name, slug, is_active, created_at, updated_at
            FROM areas
            WHERE city_id = $1
              AND ($2 OR is_active = true)
            ORDER BY name
            "#,
        )
        .bind(city_id)
        .bind(include_inactive)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_area_by_city_and_slug(
        &self,
        city_id: Uuid,
        slug: &str,
        include_inactive: bool,
    ) -> Result<Option<Area>, sqlx::Error> {
        sqlx::query_as::<_, Area>(
            r#"
            SELECT id, city_id, name, slug, is_active, created_at, updated_at
            FROM areas
            WHERE city_id = $1
              AND slug = $2
              AND ($3 OR is_active = true)
            "#,
        )
        .bind(city_id)
        .bind(slug)
        .bind(include_inactive)
        .fetch_optional(&self.pool)
        .await
    }

    /// `area_id` `None` lists streets with no area in that city; `Some` scopes to that area.
    pub async fn list_streets_by_city_and_area(
        &self,
        city_id: Uuid,
        area_id: Option<Uuid>,
        include_inactive: bool,
    ) -> Result<Vec<Street>, sqlx::Error> {
        sqlx::query_as::<_, Street>(
            r#"
            SELECT id, city_id, area_id, name, slug, is_active, created_at, updated_at
            FROM streets
            WHERE city_id = $1
              AND (
                  ($2::uuid IS NULL AND area_id IS NULL)
                  OR area_id = $2
              )
              AND ($3 OR is_active = true)
            ORDER BY name
            "#,
        )
        .bind(city_id)
        .bind(area_id)
        .bind(include_inactive)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_street_by_city_area_and_slug(
        &self,
        city_id: Uuid,
        area_id: Option<Uuid>,
        slug: &str,
        include_inactive: bool,
    ) -> Result<Option<Street>, sqlx::Error> {
        sqlx::query_as::<_, Street>(
            r#"
            SELECT id, city_id, area_id, name, slug, is_active, created_at, updated_at
            FROM streets
            WHERE city_id = $1
              AND slug = $4
              AND (
                  ($2::uuid IS NULL AND area_id IS NULL)
                  OR area_id = $2
              )
              AND ($3 OR is_active = true)
            "#,
        )
        .bind(city_id)
        .bind(area_id)
        .bind(include_inactive)
        .bind(slug)
        .fetch_optional(&self.pool)
        .await
    }
}
