//! Historical executor bytes must retain their low-bit meaning on replay.
use alloy::primitives::{Address, FixedBytes, LogData, U256};
use alloy::rpc::types::Log;
use alloy::sol_types::SolEvent;
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::chain_recovery::is_halted;
use host_listener::cmd::block_history::BlockSummary;
use host_listener::contracts::{AclContract::Allowed, TfheContract};
use host_listener::database::ingest::{
    ingest_block_logs, BlockLogs, IngestOptions,
};
use host_listener::database::tfhe_event_propagate::Database;
use serial_test::serial;
use test_harness::instance::{setup_test_db, ImportMode};

fn handle(id: u8) -> FixedBytes<32> {
    let mut bytes = [0; 32];
    bytes[0] = id;
    bytes[21] = 255;
    bytes[22..30].copy_from_slice(&42_u64.to_be_bytes());
    bytes[30] = 5;
    bytes.into()
}

fn block(logs: Vec<LogData>) -> BlockLogs<Log> {
    let summary = BlockSummary {
        number: 100,
        hash: FixedBytes::repeat_byte(100),
        parent_hash: FixedBytes::repeat_byte(99),
        timestamp: 100,
    };
    BlockLogs {
        logs: logs
            .into_iter()
            .enumerate()
            .map(|(index, data)| Log {
                inner: alloy::primitives::Log {
                    address: Address::ZERO,
                    data,
                },
                block_hash: Some(summary.hash),
                block_number: Some(summary.number),
                transaction_hash: Some(FixedBytes::repeat_byte(10)),
                transaction_index: Some(0),
                log_index: Some(index as u64),
                ..Default::default()
            })
            .collect(),
        summary,
        catchup: true,
        finalized: false,
    }
}

async fn ingest(db: &mut Database, logs: Vec<LogData>) -> anyhow::Result<()> {
    ingest_block_logs(
        ChainId::try_from(42_u64)?,
        db,
        &block(logs),
        &Some(Address::ZERO),
        &Some(Address::ZERO),
        &None,
        &None,
        &None,
        IngestOptions {
            dependence_by_connexity: false,
            dependence_cross_block: true,
            dependent_ops_max_per_chain: 0,
            is_protocol_config_listener: false,
            disable_synthetic_ops: true,
        },
    )
    .await?
    .require_complete()?;
    Ok(())
}

fn add(flag: u8, result: FixedBytes<32>) -> LogData {
    TfheContract::FheAdd {
        caller: Address::ZERO,
        lhs: handle(1),
        rhs: handle(2),
        scalarByte: FixedBytes([flag]),
        result,
    }
    .encode_log_data()
}

#[tokio::test]
#[serial(db)]
async fn legacy_flags_preserve_dependencies_and_execution_mode_on_replay(
) -> anyhow::Result<()> {
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut db =
        Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 64).await?;
    let pool = db.pool().await;
    let flags = [0, 1, 2, 3, 128, 129, 254, 255];
    for normalize in [false, true] {
        let mut logs = vec![];
        for (index, raw) in flags.iter().copied().enumerate() {
            let result = handle(10 + index as u8);
            logs.push(add(if normalize { raw & 1 } else { raw }, result));
            logs.push(
                Allowed {
                    caller: Address::ZERO,
                    account: Address::ZERO,
                    handle: result,
                }
                .encode_log_data(),
            );
        }
        // Div/rem historically accepted every odd byte. Shift amounts also
        // predate the later lhs-width and uint8 contract checks.
        let scalar_byte = FixedBytes([if normalize { 1 } else { 255 }]);
        logs.push(
            TfheContract::FheDiv {
                caller: Address::ZERO,
                lhs: handle(1),
                rhs: FixedBytes::with_last_byte(7),
                scalarByte: scalar_byte,
                result: handle(20),
            }
            .encode_log_data(),
        );
        logs.push(
            TfheContract::FheRem {
                caller: Address::ZERO,
                lhs: handle(1),
                rhs: FixedBytes::with_last_byte(7),
                scalarByte: scalar_byte,
                result: handle(21),
            }
            .encode_log_data(),
        );
        logs.push(
            TfheContract::FheShl {
                caller: Address::ZERO,
                lhs: handle(1),
                rhs: U256::MAX.to_be_bytes::<32>().into(),
                scalarByte: scalar_byte,
                result: handle(22),
            }
            .encode_log_data(),
        );
        ingest(&mut db, logs).await?;
    }
    for (index, raw) in flags.iter().copied().enumerate() {
        let (scalar, dependencies, mask): (bool, Vec<Vec<u8>>, Vec<u8>) = sqlx::query_as(
            "SELECT is_scalar, dependencies, operand_boundary_mask FROM computations WHERE output_handle=$1"
        ).bind(handle(10 + index as u8).to_vec()).fetch_one(&pool).await?;
        assert_eq!(scalar, raw & 1 != 0);
        assert_eq!(dependencies, vec![handle(1).to_vec(), handle(2).to_vec()]);
        assert_eq!(
            mask[31],
            if scalar { 1 } else { 3 },
            "encrypted RHS must remain a boundary dependency"
        );
    }
    for id in [20, 21, 22] {
        let scalar: bool = sqlx::query_scalar(
            "SELECT is_scalar FROM computations WHERE output_handle=$1",
        )
        .bind(handle(id).to_vec())
        .fetch_one(&pool)
        .await?;
        assert!(scalar);
    }
    let dependencies: Vec<Vec<u8>> = sqlx::query_scalar(
        "SELECT dependencies FROM computations WHERE output_handle=$1",
    )
    .bind(handle(22).to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(dependencies[1], U256::MAX.to_be_bytes::<32>());
    assert!(!is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn replay_does_not_silently_reinterpret_an_existing_legacy_row(
) -> anyhow::Result<()> {
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut db =
        Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 64).await?;
    let pool = db.pool().await;
    ingest(&mut db, vec![add(2, handle(10))]).await?;
    // Model the old listener's non-zero interpretation. Such work may already
    // have been consumed; compatibility must not overwrite its execution mode.
    sqlx::query("UPDATE computations SET is_scalar=true, operand_boundary_mask=NULL WHERE output_handle=$1")
        .bind(handle(10).to_vec()).execute(&pool).await?;
    assert!(ingest(&mut db, vec![add(2, handle(10))]).await.is_err());
    assert!(is_halted(&pool).await?);
    let scalar: bool = sqlx::query_scalar(
        "SELECT is_scalar FROM computations WHERE output_handle=$1",
    )
    .bind(handle(10).to_vec())
    .fetch_one(&pool)
    .await?;
    assert!(
        scalar,
        "conflicting persisted metadata is retained for audited recovery"
    );
    Ok(())
}
