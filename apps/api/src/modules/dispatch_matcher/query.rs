use chrono::Utc;
use sqlx::{Executor, Postgres};

use super::candidate::MatcherCandidate;
use super::config::MatcherConfig;
use super::input::MatcherOrderInput;

/// Hard filters per implementation_003_orders_dispatch_tokens_redis.md §5 (SQL); returns distance in km.
pub async fn fetch_candidates<'c, E>(
    executor: E,
    input: &MatcherOrderInput,
    config: &MatcherConfig,
) -> Result<Vec<MatcherCandidate>, sqlx::Error>
where
    E: Executor<'c, Database = Postgres>,
{
    let location_cutoff = Utc::now() - config.location_max_age;

    sqlx::query_as::<_, MatcherCandidate>(
        r#"
        SELECT
            plumber_id,
            token_balance,
            rating_avg,
            distance_km
        FROM (
            SELECT
                pp.id AS plumber_id,
                pp.token_balance,
                pp.rating_avg,
                pp.service_radius_km::double precision AS radius_km,
                (
                    6371.0 * acos(
                        GREATEST(
                            -1.0::double precision,
                            LEAST(
                                1.0::double precision,
                                sin(radians($5::double precision))
                                    * sin(radians(pp.current_lat::double precision))
                                    + cos(radians($5::double precision))
                                        * cos(radians(pp.current_lat::double precision))
                                        * cos(
                                            radians(pp.current_lng::double precision)
                                            - radians($6::double precision)
                                        )
                            )
                        )
                    )
                ) AS distance_km
            FROM plumber_profiles pp
            INNER JOIN plumber_services ps
                ON ps.plumber_id = pp.id
                AND ps.service_category_id = $4
            WHERE pp.is_approved = true
              AND pp.is_online = true
              AND pp.is_available = true
              AND pp.current_lat IS NOT NULL
              AND pp.current_lng IS NOT NULL
              AND pp.last_location_updated_at IS NOT NULL
              AND pp.last_location_updated_at >= $7
              AND (
                  $8::order_urgency IS DISTINCT FROM 'emergency'::order_urgency
                  OR pp.token_balance >= $9
              )
              AND pp.id NOT IN (
                  SELECT plumber_id FROM order_dispatches WHERE order_id = $1
              )
              AND EXISTS (
                  SELECT 1
                  FROM plumber_service_areas psa
                  WHERE psa.plumber_id = pp.id
                    AND psa.city_id = $2
                    AND (
                        $3::uuid IS NULL
                        OR psa.area_id IS NULL
                        OR psa.area_id = $3
                    )
              )
        ) sub
        WHERE sub.distance_km <= sub.radius_km
        "#,
    )
    .bind(input.order_id)
    .bind(input.city_id)
    .bind(input.area_id)
    .bind(input.service_category_id)
    .bind(input.lat)
    .bind(input.lng)
    .bind(location_cutoff)
    .bind(input.urgency)
    .bind(config.emergency_min_token_balance)
    .fetch_all(executor)
    .await
}
