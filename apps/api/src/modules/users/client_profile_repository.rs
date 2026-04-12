use sqlx::PgPool;
use uuid::Uuid;

use super::model::ClientProfile;

#[derive(Debug, Clone)]
pub struct UpsertClientProfileParams<'a> {
    pub user_id: Uuid,
    pub full_name: &'a str,
    pub phone: &'a str,
    pub default_city_id: Option<Uuid>,
    pub default_area_id: Option<Uuid>,
    pub default_street_id: Option<Uuid>,
    pub address_line: Option<&'a str>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}

#[derive(Clone)]
pub struct ClientProfileRepository {
    pool: PgPool,
}

impl ClientProfileRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn find_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<ClientProfile>, sqlx::Error> {
        sqlx::query_as::<_, ClientProfile>(
            r#"
            SELECT
                id, user_id, full_name, phone,
                default_city_id, default_area_id, default_street_id,
                address_line, lat, lng, created_at, updated_at
            FROM client_profiles
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn upsert(
        &self,
        params: UpsertClientProfileParams<'_>,
    ) -> Result<ClientProfile, sqlx::Error> {
        sqlx::query_as::<_, ClientProfile>(
            r#"
            INSERT INTO client_profiles (
                user_id, full_name, phone,
                default_city_id, default_area_id, default_street_id,
                address_line, lat, lng
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (user_id) DO UPDATE SET
                full_name = EXCLUDED.full_name,
                phone = EXCLUDED.phone,
                default_city_id = EXCLUDED.default_city_id,
                default_area_id = EXCLUDED.default_area_id,
                default_street_id = EXCLUDED.default_street_id,
                address_line = EXCLUDED.address_line,
                lat = EXCLUDED.lat,
                lng = EXCLUDED.lng,
                updated_at = now()
            RETURNING
                id, user_id, full_name, phone,
                default_city_id, default_area_id, default_street_id,
                address_line, lat, lng, created_at, updated_at
            "#,
        )
        .bind(params.user_id)
        .bind(params.full_name)
        .bind(params.phone)
        .bind(params.default_city_id)
        .bind(params.default_area_id)
        .bind(params.default_street_id)
        .bind(params.address_line)
        .bind(params.lat)
        .bind(params.lng)
        .fetch_one(&self.pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::model::{Role, UserStatus};
    use super::super::repository::{CreateUserParams, UserRepository};

    use super::*;

    #[ignore = "requires DATABASE_URL (Neon or Postgres); run: cargo test client_profile -- --ignored"]
    #[sqlx::test(migrations = "./migrations")]
    async fn upsert_round_trip_and_geography_fks(pool: PgPool) -> sqlx::Result<()> {
        let users = UserRepository::new(pool.clone());
        let profiles = ClientProfileRepository::new(pool);

        let city_id: Uuid = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO cities (name, slug, is_active)
            VALUES ('Test City', 'test-city', true)
            RETURNING id
            "#,
        )
        .fetch_one(profiles.pool())
        .await?;

        let user = users
            .create_user(CreateUserParams {
                email: "client-prof@example.com",
                password_hash: "ph",
                role: Role::Client,
                user_status: UserStatus::Active,
                is_email_verified: true,
                email_verification_token_hash: None,
                email_verification_expires_at: None,
            })
            .await?;

        let row = profiles
            .upsert(UpsertClientProfileParams {
                user_id: user.id,
                full_name: "Ada",
                phone: "+10000000000",
                default_city_id: Some(city_id),
                default_area_id: None,
                default_street_id: None,
                address_line: Some("Unit 1"),
                lat: Some(41.7),
                lng: Some(44.8),
            })
            .await?;

        assert_eq!(row.user_id, user.id);
        assert_eq!(row.full_name, "Ada");
        assert_eq!(row.default_city_id, Some(city_id));

        let again = profiles
            .upsert(UpsertClientProfileParams {
                user_id: user.id,
                full_name: "Ada Lovelace",
                phone: "+10000000000",
                default_city_id: None,
                default_area_id: None,
                default_street_id: None,
                address_line: None,
                lat: None,
                lng: None,
            })
            .await?;
        assert_eq!(again.full_name, "Ada Lovelace");
        assert!(again.default_city_id.is_none());

        let found = profiles.find_by_user_id(user.id).await?.expect("profile");
        assert_eq!(found.id, row.id);

        Ok(())
    }
}
