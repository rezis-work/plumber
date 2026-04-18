use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::modules::dispatch_outbox::DispatchOutboxRepository;
use crate::modules::geography::{Area, City, Street};
use crate::modules::service_categories::ServiceCategory;
use crate::AppState;

use super::create_order_error::CreateOrderError;
use super::dto::{CreateOrderRequest, CreateOrderResponse};
use super::repository::{OrderMediaInsert, OrderRepository};

const MAX_MEDIA_ITEMS: usize = 10;
const MAX_STORAGE_KEY_LEN: usize = 1024;
const MAX_MEDIA_BYTES: i32 = 20 * 1024 * 1024;

const ALLOWED_CONTENT_TYPES: &[&str] = &["image/jpeg", "image/png", "image/webp"];

pub async fn create_order(
    state: &AppState,
    client_user_id: Uuid,
    body: CreateOrderRequest,
) -> Result<CreateOrderResponse, CreateOrderError> {
    validate_payload(&body)?;

    let mut tx = state.pool.begin().await?;

    validate_refs(
        &mut tx,
        body.service_category_id,
        body.city_id,
        body.area_id,
        body.street_id,
    )
    .await?;

    let description_trimmed = body.description.trim();
    let order = OrderRepository::insert_tx(
        &mut tx,
        client_user_id,
        body.service_category_id,
        body.city_id,
        body.area_id,
        body.street_id,
        body.address_line.trim(),
        body.lat,
        body.lng,
        Some(description_trimmed),
        body.urgency,
        body.estimated_price_min,
        body.estimated_price_max,
    )
    .await?;

    if !body.media.is_empty() {
        let inserts: Vec<OrderMediaInsert<'_>> = body
            .media
            .iter()
            .enumerate()
            .map(|(i, m)| OrderMediaInsert {
                storage_key: m.storage_key.as_str(),
                content_type: m.content_type.as_str(),
                byte_size: m.byte_size,
                sort_order: m.sort_order.unwrap_or(i as i16),
            })
            .collect();

        OrderRepository::insert_media_tx(&mut tx, order.id, &inserts).await?;
    }

    DispatchOutboxRepository::insert_pending_bootstrap_tx(&mut tx, order.id).await?;

    tx.commit().await?;

    crate::modules::observability::log_order_transition(
        order.id,
        "order_created",
        None,
        None,
    );

    if let Some(redis) = state.redis_dispatch.as_ref() {
        if let Err(e) = redis.rpush_dispatch_queue(order.id).await {
            crate::modules::observability::record_dispatch_queue_rpush_failure();
            tracing::warn!(
                target = "dispatch",
                error = %e,
                order_id = %order.id,
                "dispatch_queue_rpush_failed"
            );
        }
    }

    Ok(CreateOrderResponse {
        id: order.id,
        status: order.status,
        requested_at: order.requested_at,
    })
}

fn validate_payload(body: &CreateOrderRequest) -> Result<(), CreateOrderError> {
    if body.address_line.trim().is_empty() {
        return Err(CreateOrderError::Validation {
            message: "address_line is required".to_string(),
        });
    }
    if body.address_line.len() > 500 {
        return Err(CreateOrderError::Validation {
            message: "address_line is too long".to_string(),
        });
    }
    if body.description.trim().is_empty() {
        return Err(CreateOrderError::Validation {
            message: "description is required".to_string(),
        });
    }
    if body.description.len() > 8000 {
        return Err(CreateOrderError::Validation {
            message: "description is too long".to_string(),
        });
    }
    if !body.lat.is_finite() || !body.lng.is_finite() {
        return Err(CreateOrderError::Validation {
            message: "lat and lng must be finite numbers".to_string(),
        });
    }

    if let (Some(min), Some(max)) = (body.estimated_price_min, body.estimated_price_max) {
        if min > max {
            return Err(CreateOrderError::Validation {
                message: "estimated_price_min must be <= estimated_price_max".to_string(),
            });
        }
    }

    if body.media.len() > MAX_MEDIA_ITEMS {
        return Err(CreateOrderError::Validation {
            message: format!("at most {MAX_MEDIA_ITEMS} media items allowed"),
        });
    }

    for (i, m) in body.media.iter().enumerate() {
        let key = m.storage_key.trim();
        if key.is_empty() {
            return Err(CreateOrderError::Validation {
                message: format!("media[{i}].storage_key is required"),
            });
        }
        if key.len() > MAX_STORAGE_KEY_LEN {
            return Err(CreateOrderError::Validation {
                message: format!("media[{i}].storage_key is too long"),
            });
        }
        if !ALLOWED_CONTENT_TYPES.contains(&m.content_type.as_str()) {
            return Err(CreateOrderError::Validation {
                message: format!(
                    "media[{i}].content_type must be one of: {}",
                    ALLOWED_CONTENT_TYPES.join(", ")
                ),
            });
        }
        if m.byte_size <= 0 || m.byte_size > MAX_MEDIA_BYTES {
            return Err(CreateOrderError::Validation {
                message: format!(
                    "media[{i}].byte_size must be between 1 and {MAX_MEDIA_BYTES}"
                ),
            });
        }
    }

    Ok(())
}

async fn validate_refs(
    tx: &mut Transaction<'_, Postgres>,
    service_category_id: Uuid,
    city_id: Uuid,
    area_id: Option<Uuid>,
    street_id: Option<Uuid>,
) -> Result<(), CreateOrderError> {
    let cat: Option<ServiceCategory> = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, icon, is_active, sort_order, created_at, updated_at
        FROM service_categories
        WHERE id = $1
        "#,
    )
    .bind(service_category_id)
    .fetch_optional(&mut **tx)
    .await?;

    let Some(cat) = cat else {
        return Err(CreateOrderError::InvalidCategory);
    };
    if !cat.is_active {
        return Err(CreateOrderError::CategoryInactive);
    }

    let city: Option<City> = sqlx::query_as(
        r#"
        SELECT id, name, slug, is_active, created_at, updated_at
        FROM cities
        WHERE id = $1
        "#,
    )
    .bind(city_id)
    .fetch_optional(&mut **tx)
    .await?;

    let Some(city) = city else {
        return Err(CreateOrderError::InvalidCity);
    };
    if !city.is_active {
        return Err(CreateOrderError::CityInactive);
    }

    if let Some(aid) = area_id {
        let area: Option<Area> = sqlx::query_as(
            r#"
            SELECT id, city_id, name, slug, is_active, created_at, updated_at
            FROM areas
            WHERE id = $1
            "#,
        )
        .bind(aid)
        .fetch_optional(&mut **tx)
        .await?;

        let Some(area) = area else {
            return Err(CreateOrderError::InvalidArea);
        };
        if area.city_id != city_id {
            return Err(CreateOrderError::AreaNotInCity);
        }
        if !area.is_active {
            return Err(CreateOrderError::AreaInactive);
        }
    }

    if let Some(sid) = street_id {
        let street: Option<Street> = sqlx::query_as(
            r#"
            SELECT id, city_id, area_id, name, slug, is_active, created_at, updated_at
            FROM streets
            WHERE id = $1
            "#,
        )
        .bind(sid)
        .fetch_optional(&mut **tx)
        .await?;

        let Some(street) = street else {
            return Err(CreateOrderError::InvalidStreet);
        };
        if street.city_id != city_id {
            return Err(CreateOrderError::StreetNotInCity);
        }
        if !street.is_active {
            return Err(CreateOrderError::StreetInactive);
        }
        if let Some(aid) = area_id {
            if let Some(s_area) = street.area_id {
                if s_area != aid {
                    return Err(CreateOrderError::StreetAreaMismatch);
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::modules::auth::cookie_config::CookieConfig;
    use crate::modules::auth::passwords::PasswordConfig;
    use crate::modules::auth::service_token::JwtConfig;
    use crate::modules::auth::verification::EmailVerificationConfig;
    use crate::modules::domain_enums::OrderUrgency;
    use crate::modules::geography::GeographyRepository;
    use crate::modules::orders::OrderRepository;
    use crate::modules::service_categories::ServiceCategoryRepository;
    use crate::modules::users::{RefreshTokenRepository, UserRepository};
    use crate::AppState;

    use super::super::create_order_error::CreateOrderError;
    use super::super::dto::{CreateOrderMediaItem, CreateOrderRequest};
    use super::create_order;

    fn test_app_state(pool: PgPool) -> AppState {
        AppState {
            pool: pool.clone(),
            users: UserRepository::new(pool.clone()),
            orders: OrderRepository::new(pool.clone()),
            geography: GeographyRepository::new(pool.clone()),
            service_categories: ServiceCategoryRepository::new(pool.clone()),
            refresh_tokens: RefreshTokenRepository::new(pool.clone()),
            password_config: PasswordConfig::from_env(),
            email_verification: EmailVerificationConfig {
                secret: "od1-test-hmac".to_string(),
                ttl_hours: 48,
            },
            jwt_config: JwtConfig::from_env(),
            cookie_config: CookieConfig::from_env(),
            redis_dispatch: None,
            dispatch_advance_secret: None,
        }
    }

    async fn seed_client_category_city_area_street(
        pool: &PgPool,
    ) -> sqlx::Result<(Uuid, Uuid, Uuid, Uuid, Uuid)> {
        let client_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
            VALUES ('od1-client@test.local', 'x', 'client', 'active', true)
            RETURNING id
            "#,
        )
        .fetch_one(pool)
        .await?;

        let cat_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO service_categories (name, slug, is_active, sort_order)
            VALUES ('Plumbing', 'plumbing-od1', true, 0)
            RETURNING id
            "#,
        )
        .fetch_one(pool)
        .await?;

        let city_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO cities (name, slug, is_active)
            VALUES ('Tbilisi', 'tbilisi-od1', true)
            RETURNING id
            "#,
        )
        .fetch_one(pool)
        .await?;

        let area_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO areas (city_id, name, slug, is_active)
            VALUES ($1, 'Vake', 'vake-od1', true)
            RETURNING id
            "#,
        )
        .bind(city_id)
        .fetch_one(pool)
        .await?;

        let street_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO streets (city_id, area_id, name, slug, is_active)
            VALUES ($1, $2, 'Chavchavadze', 'chavch-od1', true)
            RETURNING id
            "#,
        )
        .bind(city_id)
        .bind(area_id)
        .fetch_one(pool)
        .await?;

        Ok((client_id, cat_id, city_id, area_id, street_id))
    }

    fn base_request(
        service_category_id: Uuid,
        city_id: Uuid,
        area_id: Option<Uuid>,
        street_id: Option<Uuid>,
    ) -> CreateOrderRequest {
        CreateOrderRequest {
            service_category_id,
            city_id,
            area_id,
            street_id,
            address_line: "123 Main St".into(),
            lat: 41.7,
            lng: 44.8,
            description: "Leaky pipe under sink".into(),
            urgency: OrderUrgency::Normal,
            estimated_price_min: None,
            estimated_price_max: None,
            media: vec![],
        }
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn create_order_ok_without_media(pool: PgPool) -> sqlx::Result<()> {
        let state = test_app_state(pool.clone());
        let (client_id, cat_id, city_id, area_id, street_id) =
            seed_client_category_city_area_street(&pool).await?;

        let res = create_order(
            &state,
            client_id,
            base_request(cat_id, city_id, Some(area_id), Some(street_id)),
        )
        .await
        .expect("create_order");

        let n: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM orders WHERE id = $1"#)
            .bind(res.id)
            .fetch_one(&pool)
            .await?;
        assert_eq!(n, 1);

        let m: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM order_media WHERE order_id = $1"#)
            .bind(res.id)
            .fetch_one(&pool)
            .await?;
        assert_eq!(m, 0);

        let o: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM dispatch_outbox WHERE order_id = $1 AND status = 'pending'"#,
        )
        .bind(res.id)
        .fetch_one(&pool)
        .await?;
        assert_eq!(o, 1);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn create_order_ok_with_media(pool: PgPool) -> sqlx::Result<()> {
        let state = test_app_state(pool.clone());
        let (client_id, cat_id, city_id, area_id, street_id) =
            seed_client_category_city_area_street(&pool).await?;

        let mut req = base_request(cat_id, city_id, Some(area_id), Some(street_id));
        req.media = vec![CreateOrderMediaItem {
            storage_key: "dev/photo1.jpg".into(),
            content_type: "image/jpeg".into(),
            byte_size: 1024,
            sort_order: None,
        }];

        let res = create_order(&state, client_id, req).await.expect("create_order");

        let m: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM order_media WHERE order_id = $1"#)
            .bind(res.id)
            .fetch_one(&pool)
            .await?;
        assert_eq!(m, 1);

        let o: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM dispatch_outbox WHERE order_id = $1 AND status = 'pending'"#,
        )
        .bind(res.id)
        .fetch_one(&pool)
        .await?;
        assert_eq!(o, 1);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn create_order_rejects_empty_description(pool: PgPool) -> sqlx::Result<()> {
        let state = test_app_state(pool.clone());
        let (client_id, cat_id, city_id, area_id, street_id) =
            seed_client_category_city_area_street(&pool).await?;

        let mut req = base_request(cat_id, city_id, Some(area_id), Some(street_id));
        req.description = "   ".into();

        let err = create_order(&state, client_id, req)
            .await
            .expect_err("validation");
        assert!(matches!(
            err,
            CreateOrderError::Validation { ref message } if message.contains("description")
        ));

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn create_order_rejects_unknown_category(pool: PgPool) -> sqlx::Result<()> {
        let state = test_app_state(pool.clone());
        let (client_id, _cat_id, city_id, area_id, street_id) =
            seed_client_category_city_area_street(&pool).await?;

        let req = base_request(Uuid::new_v4(), city_id, Some(area_id), Some(street_id));
        let err = create_order(&state, client_id, req)
            .await
            .expect_err("invalid category");
        assert!(matches!(err, CreateOrderError::InvalidCategory));

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn create_order_rejects_price_range(pool: PgPool) -> sqlx::Result<()> {
        let state = test_app_state(pool.clone());
        let (client_id, cat_id, city_id, area_id, street_id) =
            seed_client_category_city_area_street(&pool).await?;

        let mut req = base_request(cat_id, city_id, Some(area_id), Some(street_id));
        req.estimated_price_min = Some(Decimal::new(200, 0));
        req.estimated_price_max = Some(Decimal::new(50, 0));

        let err = create_order(&state, client_id, req)
            .await
            .expect_err("price range");
        assert!(matches!(
            err,
            CreateOrderError::Validation { ref message } if message.contains("estimated_price")
        ));

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn create_order_rejects_street_area_mismatch(pool: PgPool) -> sqlx::Result<()> {
        let state = test_app_state(pool.clone());
        let (client_id, cat_id, city_id, _area_id, street_id) =
            seed_client_category_city_area_street(&pool).await?;

        let other_area_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO areas (city_id, name, slug, is_active)
            VALUES ($1, 'Saburtalo', 'saburtalo-od1', true)
            RETURNING id
            "#,
        )
        .bind(city_id)
        .fetch_one(&pool)
        .await?;

        let req = base_request(cat_id, city_id, Some(other_area_id), Some(street_id));
        let err = create_order(&state, client_id, req)
            .await
            .expect_err("street area mismatch");
        assert!(matches!(err, CreateOrderError::StreetAreaMismatch));

        Ok(())
    }
}
