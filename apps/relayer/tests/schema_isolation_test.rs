mod common;

use common::utils::TestSetup;
use sqlx::postgres::PgPoolOptions;

/// Demonstrates that each test gets its own isolated schema
#[tokio::test]
async fn test_schema_isolation_basic() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Connect directly to verify schema isolation
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&setup.settings.storage.sql_database_url)
        .await
        .expect("Failed to connect");

    // Verify tables exist and are empty
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM input_proof_req")
        .fetch_one(&pool)
        .await
        .expect("Query should succeed");

    assert_eq!(count, 0, "Schema should start empty");

    pool.close().await;
    setup.shutdown().await;
}

/// Demonstrates that concurrent tests don't interfere with each other
#[tokio::test]
async fn test_concurrent_schema_isolation_1() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&setup.settings.storage.sql_database_url)
        .await
        .expect("Failed to connect");

    // Insert test data - this should not affect other tests
    sqlx::query!(
        r#"
        INSERT INTO input_proof_req (ext_job_id, int_job_id, req, req_status)
        VALUES ($1, $2, '{}', 'processing')
        "#,
        uuid::Uuid::new_v4(),
        vec![0u8; 32] as Vec<u8>
    )
    .execute(&pool)
    .await
    .expect("Insert should succeed");

    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM input_proof_req")
        .fetch_one(&pool)
        .await
        .expect("Query should succeed");

    assert_eq!(count, 1);

    pool.close().await;
    setup.shutdown().await;
}

/// Run concurrently with test_concurrent_schema_isolation_1
/// If schemas are isolated, this test should still see count=0
#[tokio::test]
async fn test_concurrent_schema_isolation_2() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&setup.settings.storage.sql_database_url)
        .await
        .expect("Failed to connect");

    // This should be 0 even if test_concurrent_schema_isolation_1 runs at the same time
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM input_proof_req")
        .fetch_one(&pool)
        .await
        .expect("Query should succeed");

    assert_eq!(count, 0, "Schema should be isolated from other tests");

    pool.close().await;
    setup.shutdown().await;
}

/// Verify schema is cleaned up after test
#[tokio::test]
async fn test_schema_cleanup() {
    let schema_name: String;
    let base_url: String;

    {
        let setup = TestSetup::new().await.expect("Failed to create test setup");
        schema_name = setup
            .settings
            .storage
            .sql_database_url
            .split("search_path=")
            .nth(1)
            .expect("Should have search_path")
            .to_string();

        // Extract base URL from the full URL (before ? or & parameters)
        base_url = setup
            .settings
            .storage
            .sql_database_url
            .split('?')
            .next()
            .expect("Should have base URL")
            .to_string();

        setup.shutdown().await;
    }

    // Verify schema no longer exists
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&base_url)
        .await
        .expect("Failed to connect");

    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM information_schema.schemata WHERE schema_name = $1)",
    )
    .bind(&schema_name)
    .fetch_one(&pool)
    .await
    .expect("Query should succeed");

    assert!(!exists, "Schema should be cleaned up after test");

    pool.close().await;
}
