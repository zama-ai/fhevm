use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Default test database URL - can be overridden via TEST_DATABASE_URL env var
const DEFAULT_TEST_DB_URL: &str = "postgresql://postgres:postgres@localhost:5433/relayer_db";

/// Manages isolated PostgreSQL schema per test
///
/// Each test gets its own schema with:
/// - Unique schema name (test_<uuid>)
/// - Full migrations applied
/// - Automatic cleanup on drop
pub struct TestSchema {
    schema_name: String,
    base_url: String,
    /// Flag to track if cleanup has been performed
    /// We don't keep a pool open - reduces connection pressure in CI
    cleanup_done: bool,
}

impl TestSchema {
    /// Create a new test schema with migrations
    pub async fn new() -> Result<Self> {
        Self::new_with_base_url(None).await
    }

    /// Create a new test schema with custom base URL
    pub async fn new_with_base_url(base_url: Option<String>) -> Result<Self> {
        let schema_name = format!("test_{}", Uuid::new_v4().simple());
        let base_url = base_url.unwrap_or_else(|| {
            std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| DEFAULT_TEST_DB_URL.to_string())
        });

        info!("Creating test schema: {}", schema_name);

        // Create temporary admin connection for schema management
        // Uses single connection to minimize connection usage in CI
        let admin_pool = PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_secs(5))
            .connect(&base_url)
            .await?;

        // Create schema
        Self::create_schema(&admin_pool, &schema_name).await?;

        // Close admin pool immediately after schema creation
        // to avoid holding connections during test execution
        admin_pool.close().await;

        // Run migrations in this schema (uses its own temporary pool)
        Self::run_migrations(&base_url, &schema_name).await?;

        debug!("Test schema {} ready", schema_name);

        Ok(TestSchema {
            schema_name,
            base_url,
            cleanup_done: false,
        })
    }

    /// Get database URL configured for this test schema
    pub fn database_url(&self) -> String {
        Self::build_schema_url(&self.base_url, &self.schema_name)
    }

    /// Get base database URL (without schema)
    #[allow(dead_code)]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get schema name
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Create the schema in the database
    async fn create_schema(pool: &PgPool, schema_name: &str) -> Result<()> {
        let query = format!("CREATE SCHEMA IF NOT EXISTS {}", schema_name);

        sqlx::query(&query)
            .execute(pool)
            .await
            .with_context(|| format!("Failed to create schema {}", schema_name))?;

        debug!("Schema {} created", schema_name);
        Ok(())
    }

    /// Run migrations in the test schema
    async fn run_migrations(base_url: &str, schema_name: &str) -> Result<()> {
        debug!("Running migrations in schema {}", schema_name);

        // Create a connection with search_path set to our test schema
        let database_url_with_schema = Self::build_schema_url(base_url, schema_name);

        let migration_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url_with_schema)
            .await
            .context("Failed to connect for migrations")?;

        // Run migrations (they will be created in the active schema due to search_path)
        sqlx::migrate!("./relayer-migrate/migrations")
            .run(&migration_pool)
            .await
            .with_context(|| format!("Failed to run migrations in schema {}", schema_name))?;

        migration_pool.close().await;

        debug!("Migrations completed in schema {}", schema_name);
        Ok(())
    }

    /// Build database URL with schema-specific search_path
    fn build_schema_url(base_url: &str, schema_name: &str) -> String {
        let separator = if base_url.contains('?') { "&" } else { "?" };
        format!(
            "{}{}options=-c search_path={}",
            base_url, separator, schema_name
        )
    }

    /// Drop the schema (called automatically on drop, but can be called manually)
    pub async fn cleanup(&mut self) -> Result<()> {
        if self.cleanup_done {
            return Ok(());
        }

        info!("Dropping test schema: {}", self.schema_name);

        // Create a temporary connection for cleanup only
        let cleanup_pool = PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_secs(2))
            .connect(&self.base_url)
            .await;

        match cleanup_pool {
            Ok(pool) => {
                // Drop schema with CASCADE to remove all objects
                let drop_query = format!("DROP SCHEMA IF EXISTS {} CASCADE", self.schema_name);

                match sqlx::query(&drop_query).execute(&pool).await {
                    Ok(_) => {
                        debug!("Schema {} dropped successfully", self.schema_name);
                    }
                    Err(e) => {
                        // Log but don't fail - cleanup is best-effort
                        warn!("Failed to drop schema {}: {}", self.schema_name, e);
                    }
                }

                pool.close().await;
            }
            Err(e) => {
                warn!(
                    "Failed to connect for cleanup of schema {}: {}",
                    self.schema_name, e
                );
            }
        }

        self.cleanup_done = true;
        Ok(())
    }
}

impl Drop for TestSchema {
    fn drop(&mut self) {
        if !self.cleanup_done {
            // Spawn blocking cleanup task
            // Note: This is best-effort since we can't await in Drop
            let schema_name = self.schema_name.clone();
            let base_url = self.base_url.clone();

            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    if let Ok(pool) = PgPoolOptions::new()
                        .max_connections(1)
                        .acquire_timeout(Duration::from_secs(2))
                        .connect(&base_url)
                        .await
                    {
                        let drop_query = format!("DROP SCHEMA IF EXISTS {} CASCADE", schema_name);
                        if let Err(e) = sqlx::query(&drop_query).execute(&pool).await {
                            error!("Failed to cleanup schema {} on drop: {}", schema_name, e);
                        } else {
                            debug!("Schema {} cleaned up on drop", schema_name);
                        }
                        pool.close().await;
                    }
                });
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_schema_creation_and_cleanup() {
        let schema = TestSchema::new()
            .await
            .expect("Failed to create test schema");

        // Verify we can connect with the schema URL
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&schema.database_url())
            .await
            .expect("Failed to connect to test schema");

        // Verify tables exist in schema
        let result = sqlx::query("SELECT COUNT(*) FROM input_proof_req")
            .fetch_one(&pool)
            .await;

        assert!(result.is_ok(), "Table should exist in test schema");

        pool.close().await;
        drop(schema);
    }

    #[tokio::test]
    async fn test_multiple_schemas_isolated() {
        let schema1 = TestSchema::new().await.expect("Failed to create schema 1");
        let schema2 = TestSchema::new().await.expect("Failed to create schema 2");

        assert_ne!(
            schema1.schema_name(),
            schema2.schema_name(),
            "Schemas should have unique names"
        );

        // Both should have independent data
        let pool1 = PgPoolOptions::new()
            .max_connections(1)
            .connect(&schema1.database_url())
            .await
            .unwrap();

        let pool2 = PgPoolOptions::new()
            .max_connections(1)
            .connect(&schema2.database_url())
            .await
            .unwrap();

        // Both should work independently
        let count1 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM input_proof_req")
            .fetch_one(&pool1)
            .await
            .unwrap();

        let count2 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM input_proof_req")
            .fetch_one(&pool2)
            .await
            .unwrap();

        assert_eq!(count1, 0);
        assert_eq!(count2, 0);

        pool1.close().await;
        pool2.close().await;
    }
}
