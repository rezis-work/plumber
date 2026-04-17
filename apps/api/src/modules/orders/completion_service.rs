//! OD-6: order `accepted` → `in_progress` (plumber) and `in_progress` → `completed` (client)
//! with `plumber_token_ledger` + `token_balance` in one transaction (spec §6.3).

use chrono::Duration;
use sqlx::PgPool;
use uuid::Uuid;
use crate::modules::domain_enums::OrderStatus;
use crate::modules::observability;
use crate::modules::orders::model::Order;
use crate::modules::order_dispatches::OrderDispatch;
use crate::modules::users::UserRepository;
use crate::AppState;

use super::completion_error::CompletionError;

/// Aligns with `platform_settings.speed_bonus_window_minutes` seed (30).
const SPEED_BONUS_WINDOW_MINUTES: i64 = 30;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompleteOutcome {
    Completed {
        order_id: Uuid,
        tokens_credited: i32,
        plumber_id: Uuid,
    },
    AlreadyCompleted {
        order_id: Uuid,
    },
}

pub async fn start_order(
    pool: &PgPool,
    user_id: Uuid,
    order_id: Uuid,
    users: &UserRepository,
) -> Result<(), CompletionError> {
    let Some(profile) = users
        .find_plumber_profile_by_user_id(user_id)
        .await
        .map_err(|_| CompletionError::Internal)?
    else {
        return Err(CompletionError::Forbidden);
    };

    let mut tx = pool.begin().await?;

    let order: Option<Order> = sqlx::query_as::<_, Order>(
        r#"
        SELECT id, client_id, assigned_plumber_id, service_category_id, city_id, area_id, street_id,
               address_line, lat, lng, description, urgency, status,
               estimated_price_min, estimated_price_max, final_price,
               requested_at, accepted_at, started_at, completed_at, cancelled_at, cancel_reason,
               created_at, updated_at
        FROM orders
        WHERE id = $1
        FOR UPDATE
        "#,
    )
    .bind(order_id)
    .fetch_optional(&mut *tx)
    .await?;

    let Some(order) = order else {
        return Err(CompletionError::OrderNotFound);
    };

    let Some(assigned) = order.assigned_plumber_id else {
        return Err(CompletionError::NoAssignedPlumber);
    };

    if assigned != profile.id {
        return Err(CompletionError::Forbidden);
    }

    if order.status != OrderStatus::Accepted {
        return Err(CompletionError::OrderNotAccepted);
    }

    let updated = sqlx::query(
        r#"
        UPDATE orders
        SET status = 'in_progress',
            started_at = NOW(),
            updated_at = NOW()
        WHERE id = $1
          AND status = 'accepted'
        "#,
    )
    .bind(order_id)
    .execute(&mut *tx)
    .await?;

    if updated.rows_affected() == 0 {
        return Err(CompletionError::OrderNotAccepted);
    }

    tx.commit().await?;

    observability::log_order_transition(order_id, "order_started", None, Some(profile.id));

    Ok(())
}

/// Client completes order: ledger + balance in one transaction; idempotent if already completed.
pub async fn complete_order(
    state: &AppState,
    client_user_id: Uuid,
    order_id: Uuid,
) -> Result<CompleteOutcome, CompletionError> {
    let mut tx = state.pool.begin().await?;

    let order: Option<Order> = sqlx::query_as::<_, Order>(
        r#"
        SELECT id, client_id, assigned_plumber_id, service_category_id, city_id, area_id, street_id,
               address_line, lat, lng, description, urgency, status,
               estimated_price_min, estimated_price_max, final_price,
               requested_at, accepted_at, started_at, completed_at, cancelled_at, cancel_reason,
               created_at, updated_at
        FROM orders
        WHERE id = $1
        FOR UPDATE
        "#,
    )
    .bind(order_id)
    .fetch_optional(&mut *tx)
    .await?;

    let Some(order) = order else {
        return Err(CompletionError::OrderNotFound);
    };

    if order.client_id != client_user_id {
        return Err(CompletionError::Forbidden);
    }

    if order.status == OrderStatus::Completed {
        tx.rollback().await?;
        return Ok(CompleteOutcome::AlreadyCompleted { order_id });
    }

    if order.status != OrderStatus::InProgress {
        return Err(CompletionError::OrderNotInProgress);
    }

    let Some(plumber_id) = order.assigned_plumber_id else {
        return Err(CompletionError::NoAssignedPlumber);
    };

    let dispatch: Option<OrderDispatch> = sqlx::query_as::<_, OrderDispatch>(
        r#"
        SELECT id, order_id, plumber_id, dispatch_rank, offer_round, status, sent_at, responded_at, created_at
        FROM order_dispatches
        WHERE order_id = $1
          AND plumber_id = $2
          AND status = 'accepted'
        FOR UPDATE
        "#,
    )
    .bind(order_id)
    .bind(plumber_id)
    .fetch_optional(&mut *tx)
    .await?;

    let Some(dispatch) = dispatch else {
        return Err(CompletionError::NoWinningDispatch);
    };

    let speed_bonus = speed_bonus_applies(&dispatch, SPEED_BONUS_WINDOW_MINUTES);

    let key_completion = format!("order:{order_id}:completion");
    let key_speed = format!("order:{order_id}:speed:{plumber_id}");

    let updated = sqlx::query(
        r#"
        UPDATE orders
        SET status = 'completed',
            completed_at = NOW(),
            updated_at = NOW()
        WHERE id = $1
          AND status = 'in_progress'
        "#,
    )
    .bind(order_id)
    .execute(&mut *tx)
    .await?;

    if updated.rows_affected() == 0 {
        return Err(CompletionError::OrderNotInProgress);
    }

    let d_completion: Option<i32> = sqlx::query_scalar::<_, i32>(
        r#"
        INSERT INTO plumber_token_ledger (plumber_id, delta, reason, order_id, idempotency_key)
        VALUES ($1, 1, 'order_completed', $2, $3)
        ON CONFLICT (idempotency_key) DO NOTHING
        RETURNING delta
        "#,
    )
    .bind(plumber_id)
    .bind(order_id)
    .bind(&key_completion)
    .fetch_optional(&mut *tx)
    .await?;

    let d_speed: Option<i32> = if speed_bonus {
        sqlx::query_scalar::<_, i32>(
            r#"
            INSERT INTO plumber_token_ledger (plumber_id, delta, reason, order_id, idempotency_key)
            VALUES ($1, 2, 'speed_bonus', $2, $3)
            ON CONFLICT (idempotency_key) DO NOTHING
            RETURNING delta
            "#,
        )
        .bind(plumber_id)
        .bind(order_id)
        .bind(&key_speed)
        .fetch_optional(&mut *tx)
        .await?
    } else {
        None
    };

    let mut total = 0i32;
    if let Some(d) = d_completion {
        total += d;
    }
    if let Some(d) = d_speed {
        total += d;
    }

    if total > 0 {
        sqlx::query(
            r#"
            UPDATE plumber_profiles
            SET token_balance = token_balance + $1
            WHERE id = $2
            "#,
        )
        .bind(total)
        .bind(plumber_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    if total > 0 {
        if let Some(redis) = state.redis_dispatch.as_ref() {
            if let Err(e) = redis.invalidate_plumber_token_cache(plumber_id).await {
                tracing::warn!(
                    target = "dispatch",
                    error = %e,
                    %plumber_id,
                    "plumber_token_cache_invalidate_failed"
                );
            }
        }
    }

    let max_round: Option<i16> = sqlx::query_scalar(
        r#"SELECT MAX(offer_round) FROM order_dispatches WHERE order_id = $1"#,
    )
    .bind(order_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| CompletionError::Internal)?;

    if let Some(m) = max_round {
        observability::record_dispatch_rounds_on_complete(m);
    }
    observability::record_token_grants(total);
    observability::log_order_transition(order_id, "order_completed", None, Some(plumber_id));

    Ok(CompleteOutcome::Completed {
        order_id,
        tokens_credited: total,
        plumber_id,
    })
}

fn speed_bonus_applies(dispatch: &OrderDispatch, window_minutes: i64) -> bool {
    let Some(responded_at) = dispatch.responded_at else {
        return false;
    };
    let window = Duration::minutes(window_minutes);
    responded_at.signed_duration_since(dispatch.sent_at) <= window
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::modules::auth::{CookieConfig, EmailVerificationConfig, JwtConfig, PasswordConfig};
    use crate::modules::dispatch_matcher::MatcherConfig;
    use crate::modules::dispatch_writer::{advance_dispatch_round, AdvanceDispatchOutcome};
    use crate::modules::domain_enums::{OrderStatus, OrderUrgency};
    use crate::modules::geography::GeographyRepository;
    use crate::modules::order_dispatches::OrderDispatchRepository;
    use crate::modules::orders::OrderRepository;
    use crate::modules::service_categories::ServiceCategoryRepository;
    use crate::modules::users::{RefreshTokenRepository, UserRepository};
    use crate::AppState;

    use super::{complete_order, start_order, CompleteOutcome};

    fn app_state(pool: PgPool) -> AppState {
        AppState {
            pool: pool.clone(),
            users: UserRepository::new(pool.clone()),
            orders: OrderRepository::new(pool.clone()),
            geography: GeographyRepository::new(pool.clone()),
            service_categories: ServiceCategoryRepository::new(pool.clone()),
            refresh_tokens: RefreshTokenRepository::new(pool.clone()),
            password_config: PasswordConfig::from_env(),
            email_verification: EmailVerificationConfig::from_env(),
            jwt_config: JwtConfig::from_env(),
            cookie_config: CookieConfig::from_env(),
            redis_dispatch: None,
            dispatch_advance_secret: None,
        }
    }

    async fn seed_city_area_category(pool: &PgPool) -> (Uuid, Uuid, Uuid) {
        let city_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO cities (name, slug, is_active)
            VALUES ('Tbilisi', 'od6-tbilisi', true)
            RETURNING id
            "#,
        )
        .fetch_one(pool)
        .await
        .unwrap();

        let area_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO areas (city_id, name, slug, is_active)
            VALUES ($1, 'Vake', 'od6-vake', true)
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
            VALUES ('Leak', 'od6-leak', true, 0)
            RETURNING id
            "#,
        )
        .fetch_one(pool)
        .await
        .unwrap();

        (city_id, area_id, cat_id)
    }

    async fn insert_plumber(pool: &PgPool, email: &str, city_id: Uuid, cat_id: Uuid, area_id: Uuid) -> Uuid {
        let user_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
            VALUES ($1::citext, 'x', 'plumber', 'active', true)
            RETURNING id
            "#,
        )
        .bind(email)
        .fetch_one(pool)
        .await
        .unwrap();

        let now = Utc::now();
        let plumber_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO plumber_profiles (
                user_id, full_name, phone, experience_years,
                is_approved, is_online, is_available,
                current_lat, current_lng, last_location_updated_at,
                service_radius_km, token_balance, rating_avg
            )
            VALUES ($1, 'P', '1', 1, true, true, true, 41.7, 44.8, $2, 50, 20, 4.0)
            RETURNING id
            "#,
        )
        .bind(user_id)
        .bind(now)
        .fetch_one(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"INSERT INTO plumber_services (plumber_id, service_category_id) VALUES ($1, $2)"#,
        )
        .bind(plumber_id)
        .bind(cat_id)
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"INSERT INTO plumber_service_areas (plumber_id, city_id, area_id) VALUES ($1, $2, $3)"#,
        )
        .bind(plumber_id)
        .bind(city_id)
        .bind(Some(area_id))
        .execute(pool)
        .await
        .unwrap();

        plumber_id
    }

    async fn user_id_for_profile(pool: &PgPool, profile_id: Uuid) -> Uuid {
        sqlx::query_scalar::<_, Uuid>(
            r#"SELECT user_id FROM plumber_profiles WHERE id = $1"#,
        )
        .bind(profile_id)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    async fn insert_searching_order(pool: &PgPool, city_id: Uuid, area_id: Uuid, cat_id: Uuid) -> Uuid {
        let client_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
            VALUES ($1::citext, 'x', 'client', 'active', true)
            RETURNING id
            "#,
        )
        .bind(format!("od6-client-{}@test.local", Uuid::new_v4()))
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
        .bind(cat_id)
        .bind(city_id)
        .bind(Some(area_id))
        .bind(OrderUrgency::Normal)
        .bind(OrderStatus::Searching)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    /// Searching order → first dispatch round → single winner `accepted` + order `accepted`.
    async fn seed_accepted_order_with_dispatch(
        pool: &PgPool,
        city_id: Uuid,
        area_id: Uuid,
        cat_id: Uuid,
        sent_at: chrono::DateTime<Utc>,
        responded_at: chrono::DateTime<Utc>,
    ) -> (Uuid, Uuid, Uuid) {
        for i in 1..=3 {
            insert_plumber(
                pool,
                &format!("od6-p{i}-{}@test.local", Uuid::new_v4()),
                city_id,
                cat_id,
                area_id,
            )
            .await;
        }

        let order_id = insert_searching_order(pool, city_id, area_id, cat_id).await;
        let config = MatcherConfig::default();
        let out = advance_dispatch_round(pool, order_id, &config, None)
            .await
            .unwrap();
        assert!(matches!(out, AdvanceDispatchOutcome::Success { .. }));

        let mut rows = OrderDispatchRepository::new(pool.clone())
            .list_by_order_id(order_id)
            .await
            .unwrap();
        rows.sort_by_key(|r| r.dispatch_rank);
        let winner = &rows[0];
        let plumber_profile = winner.plumber_id;
        let plumber_user = user_id_for_profile(pool, plumber_profile).await;

        sqlx::query(
            r#"
            UPDATE order_dispatches
            SET status = 'lost_race', responded_at = NOW()
            WHERE order_id = $1 AND id != $2 AND status IN ('sent', 'viewed')
            "#,
        )
        .bind(order_id)
        .bind(winner.id)
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            UPDATE order_dispatches
            SET status = 'accepted', sent_at = $1, responded_at = $2
            WHERE id = $3
            "#,
        )
        .bind(sent_at)
        .bind(responded_at)
        .bind(winner.id)
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            UPDATE orders
            SET status = 'accepted',
                accepted_at = NOW(),
                assigned_plumber_id = $1,
                updated_at = NOW()
            WHERE id = $2
            "#,
        )
        .bind(plumber_profile)
        .bind(order_id)
        .execute(pool)
        .await
        .unwrap();

        let client_id: Uuid = sqlx::query_scalar(
            r#"SELECT client_id FROM orders WHERE id = $1"#,
        )
        .bind(order_id)
        .fetch_one(pool)
        .await
        .unwrap();

        (order_id, client_id, plumber_user)
    }

    async fn token_balance(pool: &PgPool, plumber_id: Uuid) -> i32 {
        sqlx::query_scalar(r#"SELECT token_balance FROM plumber_profiles WHERE id = $1"#)
            .bind(plumber_id)
            .fetch_one(pool)
            .await
            .unwrap()
    }

    async fn ledger_count_for_order(pool: &PgPool, order_id: Uuid) -> i64 {
        sqlx::query_scalar(
            r#"SELECT COUNT(*)::bigint FROM plumber_token_ledger WHERE order_id = $1"#,
        )
        .bind(order_id)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn complete_order_credits_completion_and_speed_bonus(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_city_area_category(&pool).await;
        let now = Utc::now();
        let sent = now - chrono::Duration::minutes(5);
        let responded = now - chrono::Duration::minutes(1);
        let (order_id, client_id, plumber_user) =
            seed_accepted_order_with_dispatch(&pool, city_id, area_id, cat_id, sent, responded)
                .await;

        let plumber_profile: Uuid = sqlx::query_scalar(
            r#"SELECT assigned_plumber_id FROM orders WHERE id = $1"#,
        )
        .bind(order_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let before = token_balance(&pool, plumber_profile).await;
        let users = UserRepository::new(pool.clone());

        start_order(&pool, plumber_user, order_id, &users)
            .await
            .expect("start");

        let state = app_state(pool.clone());
        let out = complete_order(&state, client_id, order_id)
            .await
            .expect("complete");

        let CompleteOutcome::Completed {
            tokens_credited,
            plumber_id,
            ..
        } = out
        else {
            panic!("expected completed");
        };
        assert_eq!(plumber_id, plumber_profile);
        assert_eq!(tokens_credited, 3);

        let after = token_balance(&pool, plumber_profile).await;
        assert_eq!(after - before, 3);
        assert_eq!(ledger_count_for_order(&pool, order_id).await, 2);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn complete_order_no_speed_bonus_when_slow(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_city_area_category(&pool).await;
        let now = Utc::now();
        let sent = now - chrono::Duration::minutes(90);
        let responded = now - chrono::Duration::minutes(30);
        let (order_id, client_id, plumber_user) =
            seed_accepted_order_with_dispatch(&pool, city_id, area_id, cat_id, sent, responded)
                .await;

        let plumber_profile: Uuid = sqlx::query_scalar(
            r#"SELECT assigned_plumber_id FROM orders WHERE id = $1"#,
        )
        .bind(order_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let before = token_balance(&pool, plumber_profile).await;
        let users = UserRepository::new(pool.clone());
        start_order(&pool, plumber_user, order_id, &users)
            .await
            .unwrap();

        let state = app_state(pool.clone());
        let out = complete_order(&state, client_id, order_id).await.unwrap();
        let CompleteOutcome::Completed { tokens_credited, .. } = out else {
            panic!("expected completed");
        };
        assert_eq!(tokens_credited, 1);

        let after = token_balance(&pool, plumber_profile).await;
        assert_eq!(after - before, 1);
        assert_eq!(ledger_count_for_order(&pool, order_id).await, 1);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn complete_order_idempotent_second_call(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_city_area_category(&pool).await;
        let now = Utc::now();
        let (order_id, client_id, plumber_user) =
            seed_accepted_order_with_dispatch(&pool, city_id, area_id, cat_id, now, now).await;

        let plumber_profile: Uuid = sqlx::query_scalar(
            r#"SELECT assigned_plumber_id FROM orders WHERE id = $1"#,
        )
        .bind(order_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let users = UserRepository::new(pool.clone());
        start_order(&pool, plumber_user, order_id, &users)
            .await
            .unwrap();

        let state = app_state(pool.clone());
        complete_order(&state, client_id, order_id).await.unwrap();
        let bal_mid = token_balance(&pool, plumber_profile).await;

        let second = complete_order(&state, client_id, order_id).await.unwrap();
        assert!(matches!(
            second,
            CompleteOutcome::AlreadyCompleted { .. }
        ));

        let bal_after = token_balance(&pool, plumber_profile).await;
        assert_eq!(bal_mid, bal_after);
        assert_eq!(ledger_count_for_order(&pool, order_id).await, 2);
    }
}
