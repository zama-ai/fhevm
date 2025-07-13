use sqlx::{Pool, Postgres};
use testcontainers::{ContainerAsync, GenericImage, ImageExt, core::WaitFor, runners::AsyncRunner};
use tracing::info;

pub struct DbInstance {
    /// Use to keep the database container running during the tests.
    _db_container: ContainerAsync<GenericImage>,
    pub db: Pool<Postgres>,
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
        let cont_port = container.get_host_port_ipv4(5432).await?;
        let admin_db_url =
            format!("postgresql://postgres:postgres@{cont_host}:{cont_port}/postgres");
        let db_url =
            format!("postgresql://postgres:postgres@{cont_host}:{cont_port}/kms-connector");

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

        Ok(DbInstance {
            _db_container: container,
            db: pool,
        })
    }
}
