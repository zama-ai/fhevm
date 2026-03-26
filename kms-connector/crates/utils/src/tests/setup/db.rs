use alloy::primitives::U256;
use sqlx::{Pool, Postgres, types::chrono::Utc};
use testcontainers::{ContainerAsync, GenericImage, ImageExt, core::WaitFor, runners::AsyncRunner};
use tracing::info;

const POSTGRES_PORT: u16 = 5432;

pub struct DbInstance {
    /// Use to keep the database container running during the tests.
    pub db_container: ContainerAsync<GenericImage>,
    pub db: Pool<Postgres>,
    pub url: String,
}

impl DbInstance {
    pub async fn setup() -> anyhow::Result<Self> {
        info!("Starting Postgres container...");
        let container = GenericImage::new("postgres", "17.5")
            .with_wait_for(WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_PASSWORD", "postgres")
            .start()
            .await?;
        info!("Postgres container ready!");

        let cont_host = container.get_host().await?;
        let host_port = container.get_host_port_ipv4(POSTGRES_PORT).await?;
        let admin_db_url =
            format!("postgresql://postgres:postgres@{cont_host}:{host_port}/postgres");
        let db_url =
            format!("postgresql://postgres:postgres@{cont_host}:{host_port}/kms-connector");

        info!("Creating KMS Connector db...");
        let admin_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(&admin_db_url)
            .await?;
        sqlx::query("CREATE DATABASE \"kms-connector\";")
            .execute(&admin_pool)
            .await?;
        info!("KMS Connector DB url: {db_url}");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await?;

        info!("Running migrations...");
        sqlx::migrate!("../../connector-db/migrations")
            .run(&pool)
            .await?;
        info!("KMS Connector DB ready!");

        info!("Inserting context #{TESTING_KMS_CONTEXT} for tests...");
        let now = Utc::now();
        sqlx::query!(
            "INSERT INTO kms_context(id, is_valid, created_at, updated_at) \
            VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
            TESTING_KMS_CONTEXT.as_le_slice(),
            true,
            now,
            now,
        )
        .execute(&pool)
        .await?;
        info!("Context #{TESTING_KMS_CONTEXT} is ready for tests!");

        Ok(DbInstance {
            db_container: container,
            db: pool,
            url: db_url,
        })
    }
}

pub const TESTING_KMS_CONTEXT: U256 = U256::ONE;
