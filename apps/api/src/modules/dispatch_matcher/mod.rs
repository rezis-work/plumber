//! Dispatch matcher (OD-2): eligible plumbers for an order — SQL hard filters + deterministic Rust ranking.
//! See `docs/implementation_003_domain_modeling/implementation_003_orders_dispatch_tokens_redis.md` §5.

mod candidate;
mod config;
mod input;
mod query;
mod rank;

pub use candidate::MatcherCandidate;
pub use config::MatcherConfig;
pub use input::MatcherOrderInput;
pub use rank::rank_and_take_top;

use sqlx::{Executor, Postgres};
use uuid::Uuid;

pub async fn match_plumbers<'c, E>(
    executor: E,
    input: &MatcherOrderInput,
    config: &MatcherConfig,
) -> Result<Vec<Uuid>, sqlx::Error>
where
    E: Executor<'c, Database = Postgres>,
{
    let candidates = query::fetch_candidates(executor, input, config).await?;
    Ok(rank_and_take_top(candidates, config))
}

#[cfg(test)]
mod integration_tests {
    use chrono::Utc;
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::modules::domain_enums::{OrderStatus, OrderUrgency};
    use crate::modules::order_dispatches::OrderDispatchRepository;
    use super::{match_plumbers, MatcherConfig, MatcherOrderInput};

    async fn seed_geography_and_category(pool: &PgPool) -> (Uuid, Uuid, Uuid) {
        let city_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO cities (name, slug, is_active)
            VALUES ('Tbilisi', 'dm-tbilisi', true)
            RETURNING id
            "#,
        )
        .fetch_one(pool)
        .await
        .unwrap();

        let area_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO areas (city_id, name, slug, is_active)
            VALUES ($1, 'Vake', 'dm-vake', true)
            RETURNING id
            "#,
        )
        .bind(city_id)
        .fetch_one(pool)
        .await
        .unwrap();

        let _other_area_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO areas (city_id, name, slug, is_active)
            VALUES ($1, 'Saburtalo', 'dm-sab', true)
            RETURNING id
            "#,
        )
        .bind(city_id)
        .fetch_one(pool)
        .await
        .unwrap();

        let cat_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO service_categories (name, slug, is_active, sort_order)
            VALUES ('Leak', 'dm-leak', true, 0)
            RETURNING id
            "#,
        )
        .fetch_one(pool)
        .await
        .unwrap();

        (city_id, area_id, cat_id)
    }

    async fn insert_plumber_user(pool: &PgPool, email: &str) -> Uuid {
        sqlx::query_scalar(
            r#"
            INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
            VALUES ($1::citext, 'x', 'plumber', 'active', true)
            RETURNING id
            "#,
        )
        .bind(email)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    async fn insert_plumber_profile(
        pool: &PgPool,
        user_id: Uuid,
        token_balance: i32,
        lat: f64,
        lng: f64,
    ) -> Uuid {
        let now = Utc::now();
        sqlx::query_scalar(
            r#"
            INSERT INTO plumber_profiles (
                user_id, full_name, phone, experience_years,
                is_approved, is_online, is_available,
                current_lat, current_lng, last_location_updated_at,
                service_radius_km, token_balance, rating_avg
            )
            VALUES ($1, 'P', '1', 1, true, true, true, $2, $3, $4, 50, $5, 4.0)
            RETURNING id
            "#,
        )
        .bind(user_id)
        .bind(lat)
        .bind(lng)
        .bind(now)
        .bind(token_balance)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    async fn link_service_and_area(
        pool: &PgPool,
        plumber_id: Uuid,
        category_id: Uuid,
        city_id: Uuid,
        area_id: Option<Uuid>,
    ) {
        sqlx::query(
            r#"
            INSERT INTO plumber_services (plumber_id, service_category_id)
            VALUES ($1, $2)
            "#,
        )
        .bind(plumber_id)
        .bind(category_id)
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            INSERT INTO plumber_service_areas (plumber_id, city_id, area_id)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(plumber_id)
        .bind(city_id)
        .bind(area_id)
        .execute(pool)
        .await
        .unwrap();
    }

    async fn insert_client_and_order(
        pool: &PgPool,
        city_id: Uuid,
        area_id: Option<Uuid>,
        category_id: Uuid,
        urgency: OrderUrgency,
    ) -> Uuid {
        let client_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
            VALUES ('dm-client@test.local', 'x', 'client', 'active', true)
            RETURNING id
            "#,
        )
        .fetch_one(pool)
        .await
        .unwrap();

        sqlx::query_scalar(
            r#"
            INSERT INTO orders (
                client_id, service_category_id, city_id, area_id,
                address_line, lat, lng, description, urgency, status
            )
            VALUES ($1, $2, $3, $4, 'addr', 41.7, 44.8, 'leak', $5, $6)
            RETURNING id
            "#,
        )
        .bind(client_id)
        .bind(category_id)
        .bind(city_id)
        .bind(area_id)
        .bind(urgency)
        .bind(OrderStatus::Searching)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn matcher_excludes_already_dispatched_plumber(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_geography_and_category(&pool).await;
        let u_a = insert_plumber_user(&pool, "dm-pa@test.local").await;
        let u_b = insert_plumber_user(&pool, "dm-pb@test.local").await;
        let p_a = insert_plumber_profile(&pool, u_a, 20, 41.7, 44.8).await;
        let p_b = insert_plumber_profile(&pool, u_b, 20, 41.7, 44.8).await;
        link_service_and_area(&pool, p_a, cat_id, city_id, Some(area_id)).await;
        link_service_and_area(&pool, p_b, cat_id, city_id, Some(area_id)).await;

        let order_id = insert_client_and_order(&pool, city_id, Some(area_id), cat_id, OrderUrgency::Normal)
            .await;

        OrderDispatchRepository::new(pool.clone())
            .insert(order_id, p_a, 1, crate::modules::domain_enums::DispatchStatus::Sent)
            .await
            .unwrap();

        let input = MatcherOrderInput {
            order_id,
            service_category_id: cat_id,
            city_id,
            area_id: Some(area_id),
            lat: 41.7,
            lng: 44.8,
            urgency: OrderUrgency::Normal,
        };
        let config = MatcherConfig::default();
        let matched = match_plumbers(&pool, &input, &config).await.unwrap();
        assert_eq!(matched, vec![p_b]);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn matcher_emergency_requires_min_tokens(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_geography_and_category(&pool).await;
        let u_low = insert_plumber_user(&pool, "dm-low@test.local").await;
        let u_ok = insert_plumber_user(&pool, "dm-ok@test.local").await;
        let p_low = insert_plumber_profile(&pool, u_low, 5, 41.7, 44.8).await;
        let p_ok = insert_plumber_profile(&pool, u_ok, 15, 41.7, 44.8).await;
        link_service_and_area(&pool, p_low, cat_id, city_id, Some(area_id)).await;
        link_service_and_area(&pool, p_ok, cat_id, city_id, Some(area_id)).await;

        let order_id =
            insert_client_and_order(&pool, city_id, Some(area_id), cat_id, OrderUrgency::Emergency)
                .await;

        let input = MatcherOrderInput {
            order_id,
            service_category_id: cat_id,
            city_id,
            area_id: Some(area_id),
            lat: 41.7,
            lng: 44.8,
            urgency: OrderUrgency::Emergency,
        };
        let config = MatcherConfig::default();
        let matched = match_plumbers(&pool, &input, &config).await.unwrap();
        assert_eq!(matched, vec![p_ok]);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn matcher_area_rule_wrong_area_without_whole_city_excluded(pool: PgPool) {
        let (city_id, area_vake, cat_id) = seed_geography_and_category(&pool).await;
        let area_sab: Uuid = sqlx::query_scalar(
            r#"
            SELECT id FROM areas WHERE slug = 'dm-sab' LIMIT 1
            "#,
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let u = insert_plumber_user(&pool, "dm-wrong-area@test.local").await;
        let p = insert_plumber_profile(&pool, u, 20, 41.7, 44.8).await;
        // Only Saburtalo — order is Vake
        link_service_and_area(&pool, p, cat_id, city_id, Some(area_sab)).await;

        let order_id =
            insert_client_and_order(&pool, city_id, Some(area_vake), cat_id, OrderUrgency::Normal)
                .await;

        let input = MatcherOrderInput {
            order_id,
            service_category_id: cat_id,
            city_id,
            area_id: Some(area_vake),
            lat: 41.7,
            lng: 44.8,
            urgency: OrderUrgency::Normal,
        };
        let matched = match_plumbers(&pool, &input, &MatcherConfig::default())
            .await
            .unwrap();
        assert!(matched.is_empty());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn matcher_whole_city_covers_specific_area_order(pool: PgPool) {
        let (city_id, area_vake, cat_id) = seed_geography_and_category(&pool).await;
        let u = insert_plumber_user(&pool, "dm-whole-city@test.local").await;
        let p = insert_plumber_profile(&pool, u, 20, 41.7, 44.8).await;
        link_service_and_area(&pool, p, cat_id, city_id, None).await;

        let order_id =
            insert_client_and_order(&pool, city_id, Some(area_vake), cat_id, OrderUrgency::Normal)
                .await;

        let input = MatcherOrderInput {
            order_id,
            service_category_id: cat_id,
            city_id,
            area_id: Some(area_vake),
            lat: 41.7,
            lng: 44.8,
            urgency: OrderUrgency::Normal,
        };
        let matched = match_plumbers(&pool, &input, &MatcherConfig::default())
            .await
            .unwrap();
        assert_eq!(matched, vec![p]);
    }
}
