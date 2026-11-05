//! Database regressions for repeated and fork-scoped producer observations.
mod ingest_fixtures;
use alloy::primitives::FixedBytes;
use fhevm_engine_common::database::{GCS_SCHEMA, GCS_SCHEMA_QUOTED};
use ingest_fixtures::*;

#[tokio::test]
async fn repeated_identical_operations_share_one_computation(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let b = block();
    let tx = FixedBytes::repeat_byte(19);
    let handle = computed_handle(51);
    let op = trivial_encrypt(handle);
    let result = ingest(
        &mut db,
        &b,
        vec![
            rpc_log(&op, &b, tx, 0),
            rpc_log(&op, &b, tx, 1),
            rpc_log(&allowed(handle), &b, tx, 2),
        ],
        false,
    )
    .await;
    let halts: Vec<String> = sqlx::query_scalar("SELECT reason FROM public.chain_recovery_halts WHERE resolved_at IS NULL")
        .fetch_all(&pool).await?;
    assert!(
        result.is_ok(),
        "a repeated constant is valid: {result:?}; halts={halts:?}"
    );
    Ok(())
}

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
    Ok(())
}

#[tokio::test]
async fn finalized_fallback_replay_preserves_deferred_synthesis(
) -> anyhow::Result<()> {
    use alloy::primitives::{Address, U256};
    use host_listener::contracts::BridgeContract;
    use host_listener::database::ingest::{
        ingest_block_logs, synthesize_finalized_fallback_grants, BlockLogs,
    };
    let (mut db, pool, _instance) = setup().await?;
    let b = block();
    let bridge = Address::repeat_byte(0xbd);
    let handle = computed_handle(67);
    let event = BridgeContract::FallbackGrantedPlaintext {
        dstHandle: handle,
        plaintext: U256::from(7),
    };
    let mut log = rpc_log(&event, &b, FixedBytes::repeat_byte(21), 17);
    log.inner.address = bridge;
    let mut block_logs = BlockLogs {
        logs: vec![log],
        summary: b,
        catchup: false,
        finalized: false,
    };
    let mut options = OPTIONS;
    options.disable_synthetic_ops = false;
    let chain_id = db.chain_id;
    ingest_block_logs(
        chain_id,
        &mut db,
        &block_logs,
        &None,
        &None,
        &None,
        &None,
        &Some(bridge),
        options.clone(),
    )
    .await?
    .require_complete()?;
    let mut tx = db.new_transaction().await?.expect("live stack");
    synthesize_finalized_fallback_grants(
        &db,
        &mut tx,
        b.number as i64,
        &b.hash,
    )
    .await?;
    tx.commit().await?;
    let stored: i64 = sqlx::query_scalar(
        "SELECT log_index FROM computations WHERE output_handle=$1",
    )
    .bind(handle.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(
        stored, 0,
        "deferred path uses observation rank, not the on-chain index"
    );
    block_logs.finalized = true;
    let result = ingest_block_logs(
        chain_id,
        &mut db,
        &block_logs,
        &None,
        &None,
        &None,
        &None,
        &Some(bridge),
        options,
    )
    .await;
    let halts: Vec<String> = sqlx::query_scalar("SELECT reason FROM public.chain_recovery_halts WHERE resolved_at IS NULL")
        .fetch_all(&pool).await?;
    assert!(
        result.is_ok(),
        "unchanged finalized grant must replay: {result:?}; halts={halts:?}"
    );
    Ok(())
}

#[tokio::test]
async fn repeated_mint_preserves_earliest_origin_across_split_deliveries(
) -> anyhow::Result<()> {
    use alloy::primitives::{Address, U256};
    use host_listener::contracts::TfheContract;
    let (mut db, pool, _instance) = setup().await?;
    let b = block();
    let tx = FixedBytes::repeat_byte(28);
    let producer = computed_handle(71);
    let result = computed_handle(72);
    // Deliver the later occurrence first, then learn of the earlier mint.
    for index in [9, 1] {
        ingest(
            &mut db,
            &b,
            vec![rpc_log(&trivial_encrypt(producer), &b, tx, index)],
            false,
        )
        .await?;
    }
    let consumer = TfheContract::FheAdd {
        caller: Address::ZERO,
        lhs: producer,
        rhs: U256::from(1).to_be_bytes::<32>().into(),
        scalarByte: FixedBytes([1]),
        result,
    };
    // A later duplicate in this delivery must not hide the persisted prefix.
    ingest(
        &mut db,
        &b,
        vec![
            rpc_log(&consumer, &b, tx, 5),
            rpc_log(&trivial_encrypt(producer), &b, tx, 9),
        ],
        false,
    )
    .await?;
    let first: i64 = sqlx::query_scalar(
        "SELECT log_index FROM computations WHERE output_handle=$1",
    )
    .bind(producer.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(first, 1);
    let mask: Vec<u8> = sqlx::query_scalar(
        "SELECT operand_boundary_mask FROM computations WHERE output_handle=$1",
    )
    .bind(result.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(
        mask,
        vec![0; 32],
        "the input was minted before the consumer"
    );
    Ok(())
}

#[tokio::test]
async fn sibling_fork_indexes_do_not_become_transaction_local_provenance(
) -> anyhow::Result<()> {
    use alloy::primitives::{Address, U256};
    use host_listener::contracts::TfheContract;
    let (mut db, pool, _instance) = setup().await?;
    let a = block();
    let mut b = a;
    b.hash = FixedBytes::repeat_byte(0xc1);
    let tx = FixedBytes::repeat_byte(29);
    let producer = computed_handle(73);
    let result = computed_handle(74);
    ingest(
        &mut db,
        &a,
        vec![rpc_log(&trivial_encrypt(producer), &a, tx, 1)],
        false,
    )
    .await?;
    ingest(
        &mut db,
        &b,
        vec![rpc_log(&trivial_encrypt(producer), &b, tx, 20)],
        false,
    )
    .await?;
    let index: i64 = sqlx::query_scalar(
        "SELECT log_index FROM computations WHERE output_handle=$1",
    )
    .bind(producer.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(index, 20, "do not take the minimum across forks");
    // An old-fork replay can replace the observation hint, but cannot lend its
    // index to a consumer in the other fork.
    ingest(
        &mut db,
        &a,
        vec![rpc_log(&trivial_encrypt(producer), &a, tx, 1)],
        true,
    )
    .await?;
    let consumer = TfheContract::FheAdd {
        caller: Address::ZERO,
        lhs: producer,
        rhs: U256::from(1).to_be_bytes::<32>().into(),
        scalarByte: FixedBytes([1]),
        result,
    };
    ingest(&mut db, &b, vec![rpc_log(&consumer, &b, tx, 10)], false).await?;
    let mask: Vec<u8> = sqlx::query_scalar(
        "SELECT operand_boundary_mask FROM computations WHERE output_handle=$1",
    )
    .bind(result.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(mask[31], 1, "another fork does not prove a preceding mint");
    Ok(())
}
