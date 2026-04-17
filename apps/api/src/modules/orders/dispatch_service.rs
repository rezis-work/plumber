use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::dispatch_matcher::MatcherConfig;
use crate::modules::observability;
use crate::modules::dispatch_writer::{advance_dispatch_round, AdvanceDispatchOutcome};
use crate::modules::domain_enums::{DispatchStatus, OrderStatus};
use crate::modules::order_dispatches::OrderDispatch;
use crate::modules::orders::model::Order;
use crate::modules::users::UserRepository;
use crate::AppState;

use super::dispatch_error::DispatchActionError;

/// Accept offer: transactional updates per OD-4 / spec §6.
pub async fn accept_dispatch(
    pool: &PgPool,
    user_id: Uuid,
    order_id: Uuid,
    dispatch_id: Uuid,
    users: &UserRepository,
) -> Result<(), DispatchActionError> {
    let Some(profile) = users
        .find_plumber_profile_by_user_id(user_id)
        .await
        .map_err(|_| DispatchActionError::Internal)?
    else {
        return Err(DispatchActionError::PlumberProfileMissing);
    };
    let plumber_profile_id = profile.id;

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
        return Err(DispatchActionError::OrderNotFound);
    };

    if !matches!(
        order.status,
        OrderStatus::Searching | OrderStatus::Dispatched
    ) {
        return Err(DispatchActionError::OrderNotOpen);
    }

    let dispatch: Option<OrderDispatch> = sqlx::query_as::<_, OrderDispatch>(
        r#"
        SELECT id, order_id, plumber_id, dispatch_rank, offer_round, status, sent_at, responded_at, created_at
        FROM order_dispatches
        WHERE id = $1 AND order_id = $2
        FOR UPDATE
        "#,
    )
    .bind(dispatch_id)
    .bind(order_id)
    .fetch_optional(&mut *tx)
    .await?;

    let Some(dispatch) = dispatch else {
        return Err(DispatchActionError::DispatchNotFound);
    };

    if dispatch.plumber_id != plumber_profile_id {
        return Err(DispatchActionError::Forbidden);
    }

    if dispatch.status != DispatchStatus::Sent {
        return Err(DispatchActionError::DispatchNotSent);
    }

    sqlx::query(
        r#"
        UPDATE order_dispatches
        SET status = 'lost_race', responded_at = NOW()
        WHERE order_id = $1
          AND id != $2
          AND status IN ('sent', 'viewed')
        "#,
    )
    .bind(order_id)
    .bind(dispatch_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        UPDATE order_dispatches
        SET status = 'accepted', responded_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(dispatch_id)
    .execute(&mut *tx)
    .await?;

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
    .bind(plumber_profile_id)
    .bind(order_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let secs = (Utc::now() - order.requested_at).num_milliseconds() as f64 / 1000.0;
    observability::record_time_to_accept_seconds(secs);
    observability::log_order_transition(
        order_id,
        "dispatch_accepted",
        Some(dispatch.offer_round),
        Some(plumber_profile_id),
    );

    Ok(())
}

/// Reject offer; after commit may call [`advance_dispatch_round`] if no `sent` left in this round.
pub async fn reject_dispatch(
    state: &AppState,
    user_id: Uuid,
    order_id: Uuid,
    dispatch_id: Uuid,
) -> Result<(), DispatchActionError> {
    let Some(profile) = state
        .users
        .find_plumber_profile_by_user_id(user_id)
        .await
        .map_err(|_| DispatchActionError::Internal)?
    else {
        return Err(DispatchActionError::PlumberProfileMissing);
    };
    let plumber_profile_id = profile.id;

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
        return Err(DispatchActionError::OrderNotFound);
    };

    if !matches!(
        order.status,
        OrderStatus::Searching | OrderStatus::Dispatched
    ) {
        return Err(DispatchActionError::OrderNotOpen);
    }

    let dispatch: Option<OrderDispatch> = sqlx::query_as::<_, OrderDispatch>(
        r#"
        SELECT id, order_id, plumber_id, dispatch_rank, offer_round, status, sent_at, responded_at, created_at
        FROM order_dispatches
        WHERE id = $1 AND order_id = $2
        FOR UPDATE
        "#,
    )
    .bind(dispatch_id)
    .bind(order_id)
    .fetch_optional(&mut *tx)
    .await?;

    let Some(dispatch) = dispatch else {
        return Err(DispatchActionError::DispatchNotFound);
    };

    if dispatch.plumber_id != plumber_profile_id {
        return Err(DispatchActionError::Forbidden);
    }

    if dispatch.status != DispatchStatus::Sent {
        return Err(DispatchActionError::DispatchNotSent);
    }

    let offer_round = dispatch.offer_round;

    sqlx::query(
        r#"
        UPDATE order_dispatches
        SET status = 'rejected', responded_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(dispatch_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    observability::log_order_transition(
        order_id,
        "dispatch_rejected",
        Some(offer_round),
        Some(plumber_profile_id),
    );

    let remaining_sent: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)::bigint
        FROM order_dispatches
        WHERE order_id = $1 AND offer_round = $2 AND status = 'sent'
        "#,
    )
    .bind(order_id)
    .bind(offer_round)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| DispatchActionError::Internal)?;

    if remaining_sent == 0
        && matches!(
            order.status,
            OrderStatus::Searching | OrderStatus::Dispatched
        )
    {
        let matcher_config = MatcherConfig::default();
        let redis = state.redis_dispatch.as_ref();
        match advance_dispatch_round(&state.pool, order_id, &matcher_config, redis).await {
            Ok(AdvanceDispatchOutcome::Success { .. }) => {
                observability::log_order_transition(
                    order_id,
                    "advance_after_reject",
                    Some(offer_round),
                    None,
                );
            }
            Ok(AdvanceDispatchOutcome::SkippedNoPlumbers) => {
                observability::log_order_transition(
                    order_id,
                    "advance_after_reject_no_plumbers",
                    Some(offer_round),
                    None,
                );
            }
            Ok(AdvanceDispatchOutcome::SkippedLockNotAcquired) => {
                tracing::warn!(target = "dispatch", %order_id, "advance_after_reject_lock_busy");
            }
            Ok(_) => {}
            Err(e) => {
                tracing::warn!(target = "dispatch", error = %e, %order_id, "advance_after_reject_failed");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::modules::auth::{CookieConfig, EmailVerificationConfig, JwtConfig, PasswordConfig};
    use crate::modules::dispatch_matcher::MatcherConfig;
    use crate::modules::dispatch_writer::{advance_dispatch_round, AdvanceDispatchOutcome};
    use crate::modules::domain_enums::{DispatchStatus, OrderStatus, OrderUrgency};
    use crate::modules::geography::GeographyRepository;
    use crate::modules::order_dispatches::OrderDispatchRepository;
    use crate::modules::orders::dispatch_error::DispatchActionError;
    use crate::modules::orders::OrderRepository;
    use crate::modules::service_categories::ServiceCategoryRepository;
    use crate::modules::users::{RefreshTokenRepository, UserRepository};
    use crate::AppState;

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
            VALUES ('Tbilisi', 'od4-tbilisi', true)
            RETURNING id
            "#,
        )
        .fetch_one(pool)
        .await
        .unwrap();

        let area_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO areas (city_id, name, slug, is_active)
            VALUES ($1, 'Vake', 'od4-vake', true)
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
            VALUES ('Leak', 'od4-leak', true, 0)
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

    async fn insert_searching_order(
        pool: &PgPool,
        city_id: Uuid,
        area_id: Uuid,
        cat_id: Uuid,
    ) -> Uuid {
        let client_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
            VALUES ('od4-client@test.local', 'x', 'client', 'active', true)
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
        .bind(cat_id)
        .bind(city_id)
        .bind(Some(area_id))
        .bind(OrderUrgency::Normal)
        .bind(OrderStatus::Searching)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn accept_happy_path(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_city_area_category(&pool).await;
        for i in 1..=3 {
            insert_plumber(
                &pool,
                &format!("od4-acc-p{i}@test.local"),
                city_id,
                cat_id,
                area_id,
            )
            .await;
        }
        let order_id = insert_searching_order(&pool, city_id, area_id, cat_id).await;
        let config = MatcherConfig::default();
        advance_dispatch_round(&pool, order_id, &config, None)
            .await
            .expect("advance");

        let rows = OrderDispatchRepository::new(pool.clone())
            .list_by_order_id(order_id)
            .await
            .unwrap();
        let winner_dispatch = rows.iter().find(|r| r.status == DispatchStatus::Sent).unwrap();
        let users = UserRepository::new(pool.clone());
        let winner_user = user_id_for_profile(&pool, winner_dispatch.plumber_id).await;

        accept_dispatch(&pool, winner_user, order_id, winner_dispatch.id, &users)
            .await
            .expect("accept");

        let order = OrderRepository::new(pool.clone())
            .find_by_id(order_id)
            .await
            .unwrap()
            .expect("order");
        assert_eq!(order.status, OrderStatus::Accepted);
        assert_eq!(order.assigned_plumber_id, Some(winner_dispatch.plumber_id));

        let refreshed = OrderDispatchRepository::new(pool.clone())
            .list_by_order_id(order_id)
            .await
            .unwrap();
        let mine = refreshed.iter().find(|r| r.id == winner_dispatch.id).unwrap();
        assert_eq!(mine.status, DispatchStatus::Accepted);
        assert!(refreshed.iter().any(|r| r.status == DispatchStatus::LostRace));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn accept_second_plumber_conflict(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_city_area_category(&pool).await;
        for i in 1..=3 {
            insert_plumber(
                &pool,
                &format!("od4-race-p{i}@test.local"),
                city_id,
                cat_id,
                area_id,
            )
            .await;
        }
        let order_id = insert_searching_order(&pool, city_id, area_id, cat_id).await;
        let config = MatcherConfig::default();
        advance_dispatch_round(&pool, order_id, &config, None)
            .await
            .expect("advance");

        let rows = OrderDispatchRepository::new(pool.clone())
            .list_by_order_id(order_id)
            .await
            .unwrap();
        let mut sent: Vec<_> = rows
            .iter()
            .filter(|r| r.status == DispatchStatus::Sent)
            .collect();
        sent.sort_by_key(|r| r.dispatch_rank);
        assert!(sent.len() >= 2);
        let d1 = sent[0];
        let d2 = sent[1];

        let users = UserRepository::new(pool.clone());
        let u1 = user_id_for_profile(&pool, d1.plumber_id).await;
        let u2 = user_id_for_profile(&pool, d2.plumber_id).await;

        accept_dispatch(&pool, u1, order_id, d1.id, &users)
            .await
            .expect("first accept");

        let err = accept_dispatch(&pool, u2, order_id, d2.id, &users)
            .await
            .unwrap_err();
        assert!(matches!(err, DispatchActionError::OrderNotOpen));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn reject_last_sent_in_round_triggers_advance(pool: PgPool) {
        let (city_id, area_id, cat_id) = seed_city_area_category(&pool).await;
        for i in 1..=6 {
            insert_plumber(
                &pool,
                &format!("od4-rej-p{i}@test.local"),
                city_id,
                cat_id,
                area_id,
            )
            .await;
        }
        let order_id = insert_searching_order(&pool, city_id, area_id, cat_id).await;
        let config = MatcherConfig::default();
        let first = advance_dispatch_round(&pool, order_id, &config, None)
            .await
            .expect("advance1");
        assert!(matches!(first, AdvanceDispatchOutcome::Success { .. }));

        let rows = OrderDispatchRepository::new(pool.clone())
            .list_by_order_id(order_id)
            .await
            .unwrap();
        let round1: Vec<_> = rows
            .iter()
            .filter(|r| r.offer_round == 1 && r.status == DispatchStatus::Sent)
            .collect();
        assert_eq!(round1.len(), 3);

        let state = app_state(pool.clone());
        for d in round1.iter().take(2) {
            let uid = user_id_for_profile(&pool, d.plumber_id).await;
            reject_dispatch(&state, uid, order_id, d.id)
                .await
                .expect("reject partial");
        }

        let mid = OrderDispatchRepository::new(pool.clone())
            .list_by_order_id(order_id)
            .await
            .unwrap();
        let sent_after_two = mid
            .iter()
            .filter(|r| r.offer_round == 1 && r.status == DispatchStatus::Sent)
            .count();
        assert_eq!(sent_after_two, 1);

        let last = round1[2];
        let uid_last = user_id_for_profile(&pool, last.plumber_id).await;
        reject_dispatch(&state, uid_last, order_id, last.id)
            .await
            .expect("reject last in round");

        let final_rows = OrderDispatchRepository::new(pool.clone())
            .list_by_order_id(order_id)
            .await
            .unwrap();
        let round2_sent = final_rows
            .iter()
            .filter(|r| r.offer_round == 2 && r.status == DispatchStatus::Sent)
            .count();
        assert_eq!(round2_sent, 3);

        let order = OrderRepository::new(pool.clone())
            .find_by_id(order_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(order.status, OrderStatus::Dispatched);
    }
}
