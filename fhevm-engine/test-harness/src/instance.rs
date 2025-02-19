use crate::db_utils::setup_test_user;
use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};

pub struct DBInstance {
    _container: Option<testcontainers::ContainerAsync<testcontainers::GenericImage>>,
    db_url: String,
}

impl DBInstance {
    pub fn db_url(&self) -> &str {
        self.db_url.as_str()
    }
}

/// Sets up a test database instance.
///
/// If `COPROCESSOR_TEST_LOCALHOST` is set, it sets up a test database using an existing local PostgreSQL instance.
/// Otherwise, it sets up a test database using a custom Docker container running PostgreSQL 15.7.
///
/// # Returns
///
/// A `Result` containing a `DBInstance` on success. Dropping this instance terminates the database container.
///
///
/// # Examples
///
/// ```ignore
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let db_instance = setup_test_db().await?;
///     println!("Test DB URL: {}", db_instance.db_url());
///     Ok(())
/// }
/// ```
pub async fn setup_test_db() -> Result<DBInstance, Box<dyn std::error::Error>> {
    let is_localhost = std::env::var("COPROCESSOR_TEST_LOCALHOST").is_ok();

    // Drop and recreate the database in localhost mode
    // This is useful for running tests locally with applying latest migrations
    let is_localhost_with_reset = std::env::var("COPROCESSOR_TEST_LOCALHOST_RESET").is_ok();

    if is_localhost || is_localhost_with_reset {
        setup_test_app_existing_localhost(is_localhost_with_reset).await
    } else {
        setup_test_app_custom_docker().await
    }
}

async fn setup_test_app_existing_localhost(
    with_reset: bool,
) -> Result<DBInstance, Box<dyn std::error::Error>> {
    const LOCALHOST: &str = "127.0.0.1";
    const LOCAL_PORT: i32 = 5432;
    let db_url = format!("postgresql://postgres:postgres@{LOCALHOST}:{LOCAL_PORT}/coprocessor");

    if with_reset {
        let admin_db_url =
            format!("postgresql://postgres:postgres@{LOCALHOST}:{LOCAL_PORT}/postgres");
        create_database(&admin_db_url, &db_url).await?;
    }

    Ok(DBInstance {
        _container: None,
        db_url: db_url.to_string(),
    })
}

async fn setup_test_app_custom_docker() -> Result<DBInstance, Box<dyn std::error::Error>> {
    let container = GenericImage::new("postgres", "15.7")
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .start()
        .await
        .expect("postgres started");

    println!("Postgres started...");

    let cont_host = container.get_host().await?;
    let cont_port = container.get_host_port_ipv4(5432).await?;

    let admin_db_url = format!("postgresql://postgres:postgres@{cont_host}:{cont_port}/postgres");
    let db_url = format!("postgresql://postgres:postgres@{cont_host}:{cont_port}/coprocessor");
    create_database(&admin_db_url, &db_url).await?;

    Ok(DBInstance {
        _container: Some(container),
        db_url,
    })
}

async fn create_database(
    admin_db_url: &str,
    db_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating coprocessor db...");
    let admin_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(admin_db_url)
        .await?;

    sqlx::query!("DROP DATABASE IF EXISTS coprocessor;")
        .execute(&admin_pool)
        .await?;

    sqlx::query!("CREATE DATABASE coprocessor;")
        .execute(&admin_pool)
        .await?;

    println!("database url: {db_url}");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await?;

    println!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    println!("Creating test user with all keys...");
    setup_test_user(&pool).await?;
    println!("DB prepared");

    Ok(())
}
