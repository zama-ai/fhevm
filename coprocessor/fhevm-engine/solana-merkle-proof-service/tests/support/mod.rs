use solana_merkle_proof_service::MIGRATOR;
use sqlx::postgres::PgPoolOptions;
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};

/// A fresh database holding only the record's schema. Keep the instance alive while the pool is
/// in use: dropping it stops the container.
pub async fn record_db() -> (DBInstance, sqlx::PgPool) {
    let instance = setup_test_db(ImportMode::SkipMigrations)
        .await
        .expect("test database");
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(instance.db_url())
        .await
        .expect("connect test database");
    MIGRATOR.run(&pool).await.expect("migrate the record");
    (instance, pool)
}
