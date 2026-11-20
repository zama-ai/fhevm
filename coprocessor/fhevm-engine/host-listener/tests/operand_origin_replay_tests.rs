//! Database regressions for repeated and fork-scoped producer observations.
mod ingest_fixtures;
use fhevm_engine_common::database::{GCS_SCHEMA, GCS_SCHEMA_QUOTED};
use ingest_fixtures::*;

#[tokio::test]
async fn executor_version_migration_updates_existing_green_tables(
) -> anyhow::Result<()> {
    let (_db, pool, _instance) = setup().await?;
    sqlx::raw_sql(&format!(
        "ALTER TABLE public.host_chain_blocks_valid DROP COLUMN tfhe_executor_address, DROP COLUMN tfhe_executor_version;
         CREATE SCHEMA {GCS_SCHEMA_QUOTED};
         CREATE TABLE {GCS_SCHEMA_QUOTED}.host_chain_blocks_valid (LIKE public.host_chain_blocks_valid INCLUDING ALL);"
    )).execute(&pool).await?;
    sqlx::raw_sql(include_str!("../../db-migration/migrations/20261024120000_host_executor_version_state.sql"))
        .execute(&pool).await?;
    let columns: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM information_schema.columns WHERE table_schema=$1 AND table_name='host_chain_blocks_valid' AND column_name IN ('tfhe_executor_address','tfhe_executor_version')")
        .bind(GCS_SCHEMA).fetch_one(&pool).await?;
    assert_eq!(
        columns, 2,
        "migration left the active Green listener's table incompatible"
    );
    let unvalidated: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM pg_constraint c JOIN pg_namespace n ON n.oid = c.connamespace
         WHERE n.nspname IN ('public', $1) AND c.conname IN ('host_chain_blocks_valid_executor_address_length', 'host_chain_blocks_valid_executor_version_format')
           AND NOT c.convalidated"
    ).bind(GCS_SCHEMA).fetch_one(&pool).await?;
    assert_eq!(
        unvalidated, 4,
        "rollout must not scan retained per-block history"
    );
    Ok(())
}
