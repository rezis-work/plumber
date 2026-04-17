use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::modules::domain_enums::OrderUrgency;

use super::model::Order;

const MAX_LIST_LIMIT: i64 = 100;

#[derive(Debug, Clone)]
pub struct OrderMediaInsert<'a> {
    pub storage_key: &'a str,
    pub content_type: &'a str,
    pub byte_size: i32,
    pub sort_order: i16,
}

#[derive(Clone)]
pub struct OrderRepository {
    pool: PgPool,
}

impl OrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn find_by_id_for_update_tx(
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> Result<Option<Order>, sqlx::Error> {
        sqlx::query_as::<_, Order>(
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
        .bind(id)
        .fetch_optional(&mut **tx)
        .await
    }

    pub async fn set_status_dispatched_if_searching_tx(
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let r = sqlx::query(
            r#"
            UPDATE orders
            SET status = 'dispatched', updated_at = NOW()
            WHERE id = $1 AND status = 'searching'
            "#,
        )
        .bind(id)
        .execute(&mut **tx)
        .await?;
        Ok(r.rows_affected())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Order>, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            r#"
            SELECT id, client_id, assigned_plumber_id, service_category_id, city_id, area_id, street_id,
                   address_line, lat, lng, description, urgency, status,
                   estimated_price_min, estimated_price_max, final_price,
                   requested_at, accepted_at, started_at, completed_at, cancelled_at, cancel_reason,
                   created_at, updated_at
            FROM orders
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn list_by_client_id(
        &self,
        client_id: Uuid,
        limit: i64,
    ) -> Result<Vec<Order>, sqlx::Error> {
        let limit = limit.clamp(1, MAX_LIST_LIMIT);
        sqlx::query_as::<_, Order>(
            r#"
            SELECT id, client_id, assigned_plumber_id, service_category_id, city_id, area_id, street_id,
                   address_line, lat, lng, description, urgency, status,
                   estimated_price_min, estimated_price_max, final_price,
                   requested_at, accepted_at, started_at, completed_at, cancelled_at, cancel_reason,
                   created_at, updated_at
            FROM orders
            WHERE client_id = $1
            ORDER BY requested_at DESC
            LIMIT $2
            "#,
        )
        .bind(client_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    /// Insert order inside an open transaction (e.g. with `order_media` in the same txn).
    pub async fn insert_tx(
        tx: &mut Transaction<'_, Postgres>,
        client_id: Uuid,
        service_category_id: Uuid,
        city_id: Uuid,
        area_id: Option<Uuid>,
        street_id: Option<Uuid>,
        address_line: &str,
        lat: f64,
        lng: f64,
        description: Option<&str>,
        urgency: OrderUrgency,
        estimated_price_min: Option<Decimal>,
        estimated_price_max: Option<Decimal>,
    ) -> Result<Order, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            r#"
            INSERT INTO orders (
                client_id, service_category_id, city_id, area_id, street_id,
                address_line, lat, lng, description, urgency,
                estimated_price_min, estimated_price_max
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id, client_id, assigned_plumber_id, service_category_id, city_id, area_id, street_id,
                      address_line, lat, lng, description, urgency, status,
                      estimated_price_min, estimated_price_max, final_price,
                      requested_at, accepted_at, started_at, completed_at, cancelled_at, cancel_reason,
                      created_at, updated_at
            "#,
        )
        .bind(client_id)
        .bind(service_category_id)
        .bind(city_id)
        .bind(area_id)
        .bind(street_id)
        .bind(address_line)
        .bind(lat)
        .bind(lng)
        .bind(description)
        .bind(urgency)
        .bind(estimated_price_min)
        .bind(estimated_price_max)
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn insert_media_tx(
        tx: &mut Transaction<'_, Postgres>,
        order_id: Uuid,
        items: &[OrderMediaInsert<'_>],
    ) -> Result<(), sqlx::Error> {
        for item in items {
            sqlx::query(
                r#"
                INSERT INTO order_media (order_id, storage_key, content_type, byte_size, sort_order)
                VALUES ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(order_id)
            .bind(item.storage_key)
            .bind(item.content_type)
            .bind(item.byte_size)
            .bind(item.sort_order)
            .execute(&mut **tx)
            .await?;
        }
        Ok(())
    }
}
