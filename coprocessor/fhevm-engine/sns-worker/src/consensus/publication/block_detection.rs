use crate::{ExecutionError, HandleItem};
use fhevm_engine_common::utils::to_hex;
use sqlx::{Postgres, Transaction};
use tracing::debug;

use super::cadence::manifest_publication_cadence;

pub(crate) async fn ensure_block_manifest_state_row(
    trx: &mut Transaction<'_, Postgres>,
    task: &HandleItem,
    consensus_enabled: bool,
) -> Result<(), ExecutionError> {
    if !consensus_enabled || task.producer_block_hash.is_empty() {
        return Ok(());
    }

    let block_number = task.block_number.ok_or_else(|| {
        ExecutionError::InternalError(format!(
            "cannot create block manifest state without block number for producer {}",
            to_hex(&task.producer_block_hash),
        ))
    })?;

    let row = sqlx::query!(
        r#"
        WITH source AS (
            SELECT b.chain_id,
                   b.block_number,
                   b.block_hash,
                   b.parent_hash,
                   $4::BIGINT AS publication_cadence
              FROM host_chain_blocks_valid b
             WHERE b.chain_id = $1
               AND b.block_number = $2
               AND b.block_hash = $3
               AND b.parent_hash IS NOT NULL
               AND OCTET_LENGTH(b.parent_hash) = 32
               AND b.block_status <> 'orphaned'
        ),
        inserted AS (
            INSERT INTO block_manifest_state (
                host_chain_id,
                block_number,
                block_hash,
                parent_block_hash,
                publication_cadence
            )
            SELECT chain_id,
                   block_number,
                   block_hash,
                   parent_hash,
                   publication_cadence
              FROM source
            ON CONFLICT (host_chain_id, block_hash) DO NOTHING
            RETURNING 1
        )
        SELECT EXISTS(SELECT 1 FROM source) AS "source_exists!",
               EXISTS(SELECT 1 FROM inserted) AS "inserted!"
        "#,
        task.host_chain_id.as_i64(),
        block_number,
        &task.producer_block_hash,
        manifest_publication_cadence(task.host_chain_id.as_i64()),
    )
    .fetch_one(trx.as_mut())
    .await?;

    if !row.source_exists {
        return Err(ExecutionError::InternalError(format!(
            "cannot create block manifest state without live parent metadata for chain {} block {} producer {}",
            task.host_chain_id.as_i64(),
            block_number,
            to_hex(&task.producer_block_hash),
        )));
    }

    if row.inserted {
        debug!(
            host_chain_id = task.host_chain_id.as_i64(),
            block_number,
            block_hash = %to_hex(&task.producer_block_hash),
            "Created block manifest state"
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BigCiphertext, HandleItem};
    use fhevm_engine_common::chain_id::ChainId;
    use serial_test::serial;
    use sqlx::Row;
    use std::sync::Arc;
    use test_harness::instance::{setup_test_db, DBInstance, ImportMode};

    const CHAIN_ID: i64 = 137;

    fn task(block_number: Option<i64>, producer_block_hash: Vec<u8>) -> HandleItem {
        HandleItem {
            host_chain_id: ChainId::try_from(CHAIN_ID).expect("valid chain ID"),
            key_id_gw: vec![0x11; 32],
            handle: vec![0x22; 32],
            producer_block_hash,
            block_hash: vec![0x33; 32],
            block_number,
            ct64_compressed: Arc::new(vec![0x44]),
            ct128: Arc::new(BigCiphertext::default()),
            ct64_digest: None,
            ct128_digest: None,
            s3_format_version: None,
            span: tracing::Span::none(),
            transaction_id: None,
        }
    }

    async fn setup_pool() -> (DBInstance, sqlx::PgPool) {
        let instance = setup_test_db(ImportMode::None)
            .await
            .expect("create manifest state database");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect(instance.db_url())
            .await
            .expect("connect manifest state database");
        (instance, pool)
    }

    async fn assert_task_is_ignored(
        pool: &sqlx::PgPool,
        task: &HandleItem,
        consensus_enabled: bool,
    ) {
        let mut trx = pool
            .begin()
            .await
            .expect("begin ignored manifest state transaction");
        ensure_block_manifest_state_row(&mut trx, task, consensus_enabled)
            .await
            .expect("task is ignored");
        trx.commit()
            .await
            .expect("commit ignored manifest state transaction");
    }

    async fn manifest_state_count(pool: &sqlx::PgPool) -> i64 {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM block_manifest_state")
            .fetch_one(pool)
            .await
            .expect("count manifest state rows")
    }

    #[tokio::test]
    #[serial(db)]
    async fn creates_manifest_state_from_live_block_once() {
        let (_instance, pool) = setup_pool().await;
        let block_hash = vec![0x42; 32];
        let parent_hash = vec![0x41; 32];
        sqlx::query(
            "INSERT INTO host_chain_blocks_valid \
             (chain_id, block_hash, parent_hash, block_number, block_status) \
             VALUES ($1, $2, $3, 42, 'pending')",
        )
        .bind(CHAIN_ID)
        .bind(&block_hash)
        .bind(&parent_hash)
        .execute(&pool)
        .await
        .expect("seed live host block");

        let task = task(Some(42), block_hash.clone());
        for _ in 0..2 {
            let mut trx = pool
                .begin()
                .await
                .expect("begin manifest state transaction");
            ensure_block_manifest_state_row(&mut trx, &task, true)
                .await
                .expect("create manifest state row");
            trx.commit()
                .await
                .expect("commit manifest state transaction");
        }

        let row = sqlx::query(
            "SELECT block_number, parent_block_hash, publication_cadence \
               FROM block_manifest_state \
              WHERE host_chain_id = $1 AND block_hash = $2",
        )
        .bind(CHAIN_ID)
        .bind(&block_hash)
        .fetch_one(&pool)
        .await
        .expect("load created manifest state row");
        assert_eq!(row.get::<i64, _>("block_number"), 42);
        assert_eq!(row.get::<Vec<u8>, _>("parent_block_hash"), parent_hash);
        assert_eq!(row.get::<i64, _>("publication_cadence"), 30);
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM block_manifest_state \
              WHERE host_chain_id = $1 AND block_hash = $2",
        )
        .bind(CHAIN_ID)
        .bind(&block_hash)
        .fetch_one(&pool)
        .await
        .expect("count manifest state rows");
        assert_eq!(count, 1, "replaying an upload must not duplicate state");
    }

    #[tokio::test]
    #[serial(db)]
    async fn rejects_orphaned_block_and_skips_disabled_or_hashless_tasks() {
        let (_instance, pool) = setup_pool().await;
        let orphaned_hash = vec![0x52; 32];
        sqlx::query(
            "INSERT INTO host_chain_blocks_valid \
             (chain_id, block_hash, parent_hash, block_number, block_status) \
             VALUES ($1, $2, $3, 52, 'orphaned')",
        )
        .bind(CHAIN_ID)
        .bind(&orphaned_hash)
        .bind(vec![0x51u8; 32])
        .execute(&pool)
        .await
        .expect("seed orphaned host block");

        let mut trx = pool
            .begin()
            .await
            .expect("begin orphaned manifest state transaction");
        let err = ensure_block_manifest_state_row(&mut trx, &task(Some(52), orphaned_hash), true)
            .await
            .expect_err("orphaned source must not create manifest state");
        assert!(err.to_string().contains("without live parent metadata"));
        trx.rollback().await.expect("rollback orphaned transaction");

        assert_task_is_ignored(&pool, &task(Some(53), Vec::new()), true).await;
        assert_task_is_ignored(&pool, &task(Some(54), vec![0x54; 32]), false).await;

        assert_eq!(manifest_state_count(&pool).await, 0);
    }
}
