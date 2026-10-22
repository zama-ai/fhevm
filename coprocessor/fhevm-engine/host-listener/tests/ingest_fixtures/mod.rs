//! Small database-only fixtures shared by ingestion regression suites.
#![allow(dead_code)]

use alloy::primitives::{Address, FixedBytes, U256};
use alloy::rpc::types::Log;
use alloy::sol_types::SolEvent;
use fhevm_engine_common::chain_id::ChainId;
use host_listener::cmd::block_history::BlockSummary;
use host_listener::contracts::{AclContract, TfheContract};
use host_listener::database::ingest::{
    ingest_block_logs, BlockLogs, IngestOptions,
};
use host_listener::database::tfhe_event_propagate::Database;
use sqlx::postgres::PgPoolOptions;
use test_harness::instance::ImportMode;

pub const OPTIONS: IngestOptions = IngestOptions {
    dependence_by_connexity: false,
    dependence_cross_block: true,
    dependent_ops_max_per_chain: 0,
    is_protocol_config_listener: false,
    disable_synthetic_ops: true,
};

pub fn rpc_log<E: SolEvent>(
    event: &E,
    block: &BlockSummary,
    tx_hash: FixedBytes<32>,
    log_index: u64,
) -> Log {
    Log {
        inner: alloy::primitives::Log {
            address: Address::ZERO,
            data: event.encode_log_data(),
        },
        block_hash: Some(block.hash),
        block_number: Some(block.number),
        block_timestamp: None,
        transaction_hash: Some(tx_hash),
        transaction_index: Some(0),
        log_index: Some(log_index),
        removed: false,
    }
}

pub fn trivial_encrypt(result: FixedBytes<32>) -> TfheContract::TrivialEncrypt {
    TfheContract::TrivialEncrypt {
        caller: Address::ZERO,
        pt: U256::from(7),
        toType: 5,
        result,
    }
}

pub fn allowed(handle: FixedBytes<32>) -> AclContract::Allowed {
    AclContract::Allowed {
        caller: Address::ZERO,
        account: Address::with_last_byte(1),
        handle,
    }
}

pub async fn computation_flags(
    pool: &sqlx::PgPool,
    handle: FixedBytes<32>,
) -> anyhow::Result<(bool, bool)> {
    let row: (bool, bool) = sqlx::query_as(
        "SELECT is_allowed, is_completed FROM computations WHERE output_handle = $1",
    )
    .bind(handle.to_vec())
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn chain_status(
    pool: &sqlx::PgPool,
    dcid: FixedBytes<32>,
) -> anyhow::Result<String> {
    Ok(sqlx::query_scalar(
        "SELECT status FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(dcid.to_vec())
    .fetch_one(pool)
    .await?)
}

pub async fn schedulable(
    pool: &sqlx::PgPool,
    dcid: FixedBytes<32>,
) -> anyhow::Result<i64> {
    Ok(sqlx::query_scalar(
        "SELECT count(*) FROM computations
         WHERE is_completed = FALSE AND is_error = FALSE AND is_allowed = TRUE
           AND dependence_chain_id = $1",
    )
    .bind(dcid.to_vec())
    .fetch_one(pool)
    .await?)
}

pub async fn setup(
) -> anyhow::Result<(Database, sqlx::PgPool, test_harness::instance::DBInstance)>
{
    let db_instance = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    let db =
        Database::new(&db_instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(db_instance.db_url())
        .await?;
    Ok((db, pool, db_instance))
}

pub fn computed_handle(id: u8) -> FixedBytes<32> {
    let mut bytes = [0u8; 32];
    bytes[0] = id;
    bytes[21] = 0xff;
    bytes[22..30].copy_from_slice(&42u64.to_be_bytes());
    bytes[30] = 5;
    FixedBytes::from(bytes)
}

pub async fn ingest(
    db: &mut Database,
    block: &BlockSummary,
    logs: Vec<Log>,
    catchup: bool,
) -> anyhow::Result<()> {
    ingest_with_options(db, block, logs, catchup, OPTIONS).await
}

pub async fn ingest_with_options(
    db: &mut Database,
    block: &BlockSummary,
    logs: Vec<Log>,
    catchup: bool,
    options: IngestOptions,
) -> anyhow::Result<()> {
    ingest_filtered(db, block, logs, catchup, options, None).await
}

pub fn block() -> BlockSummary {
    BlockSummary {
        number: 100,
        hash: computed_handle(0xb1),
        parent_hash: computed_handle(0xb0),
        timestamp: 1_790_000_000,
    }
}

pub async fn ingest_filtered(
    db: &mut Database,
    block: &BlockSummary,
    logs: Vec<Log>,
    catchup: bool,
    options: IngestOptions,
    contract_address: Option<Address>,
) -> anyhow::Result<()> {
    let block_logs = BlockLogs {
        logs,
        summary: *block,
        catchup,
        finalized: false,
    };
    ingest_block_logs(
        ChainId::try_from(42_u64)?,
        db,
        &block_logs,
        &contract_address,
        &contract_address,
        &None,
        &None,
        &None,
        options,
    )
    .await?
    .require_complete()?;
    Ok(())
}
