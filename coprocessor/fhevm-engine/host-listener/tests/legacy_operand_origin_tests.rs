//! Historical executor handles must survive a fresh replay across upgrades.
mod ingest_fixtures;
use ingest_fixtures::*;

use alloy::primitives::{keccak256, Address, FixedBytes, U256};
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::chain_recovery::is_halted;
use host_listener::contracts::TfheContract;
use host_listener::database::ingest::{ingest_block_logs, BlockLogs};
use host_listener::database::tfhe_event_propagate::Database;
use serial_test::serial;

#[derive(Clone, Copy, PartialEq)]
enum Executor {
    BeforeContext,
    BeforeOrigin,
    Current,
}

// Exact abi.encodePacked layouts from v0.14.3 FHEVMExecutor.sol:
// trivialEncrypt and _binaryOp. Neither includes caller, transaction hash,
// nor operand-origin bits. The block context is identical for both txs.
fn contract_handle(
    op: u8,
    operands: &[u8],
    boundary: Option<u8>,
    executor: Executor,
) -> FixedBytes<32> {
    let b = block();
    let mut preimage = if executor == Executor::BeforeContext {
        Vec::new()
    } else {
        b"FHE_comp".to_vec()
    };
    preimage.push(op);
    preimage.extend_from_slice(operands);
    if let Some(boundary) = boundary {
        preimage.extend_from_slice(&U256::from(boundary).to_be_bytes::<32>());
    }
    preimage.extend_from_slice(Address::with_last_byte(1).as_slice());
    preimage.extend_from_slice(&U256::from(42).to_be_bytes::<32>());
    if executor != Executor::BeforeContext {
        preimage.extend_from_slice(b.parent_hash.as_slice());
        preimage
            .extend_from_slice(&U256::from(b.timestamp).to_be_bytes::<32>());
    }
    let mut result = keccak256(preimage);
    result.0[21] = 255;
    result.0[22..30].copy_from_slice(&42_u64.to_be_bytes());
    result.0[30] = 5;
    result.0[31] = 0;
    result
}

fn events(
    remint: bool,
    executor: Executor,
    scalar: u8,
) -> Vec<alloy::rpc::types::Log> {
    let modern = executor == Executor::Current;
    let b = block();
    let tx_a = computed_handle(201);
    let tx_b = computed_handle(202);
    let mut trivial_args = U256::from(7).to_be_bytes::<32>().to_vec();
    trivial_args.push(5);
    let a = contract_handle(24, &trivial_args, None, executor);
    let mut add_args = a.to_vec();
    add_args.extend_from_slice(&U256::from(1).to_be_bytes::<32>());
    add_args.push(1);
    let output = contract_handle(0, &add_args, modern.then_some(1), executor);
    let later_output = contract_handle(
        0,
        &add_args,
        modern.then_some(u8::from(!remint)),
        executor,
    );
    let add = TfheContract::FheAdd {
        caller: Address::with_last_byte(3),
        lhs: a,
        rhs: U256::from(1).to_be_bytes::<32>().into(),
        scalarByte: FixedBytes([scalar]),
        result: output,
    };
    // One wrapper persists A in tx A, then uses it in tx B before and after
    // re-minting the same trivial value. Both adds legitimately emit output.
    let mut mint = trivial_encrypt(a);
    mint.caller = Address::with_last_byte(3);
    let mut allow_a = allowed(a);
    allow_a.caller = mint.caller;
    allow_a.account = mint.caller;
    let mut allow_b = allowed(output);
    allow_b.caller = mint.caller;
    allow_b.account = mint.caller;
    let mut logs = vec![
        rpc_log(&mint, &b, tx_a, 0),
        rpc_log(&allow_a, &b, tx_a, 1),
        rpc_log(&add, &b, tx_b, 2),
    ];
    if remint {
        logs.push(rpc_log(&mint, &b, tx_b, 3));
    }
    logs.push(rpc_log(
        &TfheContract::FheAdd {
            result: later_output,
            ..add
        },
        &b,
        tx_b,
        4,
    ));
    logs.push(rpc_log(&allow_b, &b, tx_b, 5));
    if later_output != output {
        allow_b.handle = later_output;
        logs.push(rpc_log(&allow_b, &b, tx_b, 6));
    }
    for log in &mut logs {
        log.inner.address = if matches!(log.log_index, Some(1 | 5 | 6)) {
            Address::with_last_byte(1)
        } else {
            Address::with_last_byte(2)
        };
        log.transaction_index =
            Some(u64::from(log.transaction_hash == Some(tx_b)));
    }
    logs
}

async fn replay(
    db: &mut Database,
    logs: Vec<alloy::rpc::types::Log>,
) -> anyhow::Result<()> {
    ingest_block_logs(
        ChainId::try_from(42u64)?,
        db,
        &BlockLogs {
            logs,
            summary: block(),
            catchup: true,
            finalized: true,
        },
        &Some(Address::with_last_byte(1)),
        &Some(Address::with_last_byte(2)),
        &None,
        &None,
        &None,
        OPTIONS,
    )
    .await?
    .require_complete()?;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn historical_repeated_output_after_remint_must_not_halt_fresh_sync(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    replay(&mut db, events(true, Executor::BeforeOrigin, 1)).await?;
    replay(&mut db, events(true, Executor::BeforeOrigin, 1)).await?;
    assert!(!is_halted(&pool).await?);
    let masks: Vec<Option<Vec<u8>>> = sqlx::query_scalar(
        "SELECT operand_boundary_mask FROM computations WHERE fhe_operation = 0"
    ).fetch_all(&pool).await?;
    assert_eq!(
        masks,
        vec![None],
        "legacy duplicate has no committed origin"
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn historical_repeated_output_without_remint_is_accepted(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    replay(&mut db, events(false, Executor::BeforeOrigin, 1)).await?;
    assert!(!is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn historical_nonboolean_scalar_uses_the_normalized_handle_preimage(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    replay(&mut db, events(true, Executor::BeforeContext, 3)).await?;
    assert!(!is_halted(&pool).await?);
    let mask: Option<Vec<u8>> = sqlx::query_scalar(
        "SELECT operand_boundary_mask FROM computations WHERE fhe_operation = 0"
    ).fetch_one(&pool).await?;
    assert!(mask.is_none());
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn historical_replay_preserves_a_mask_recorded_before_this_fix(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    replay(&mut db, events(false, Executor::BeforeOrigin, 1)).await?;
    let mask = U256::from(1).to_be_bytes::<32>().to_vec();
    sqlx::query("UPDATE computations SET operand_boundary_mask = $1 WHERE fhe_operation = 0")
        .bind(&mask).execute(&pool).await?;
    replay(&mut db, events(true, Executor::BeforeOrigin, 1)).await?;
    let retained: Vec<u8> = sqlx::query_scalar(
        "SELECT operand_boundary_mask FROM computations WHERE fhe_operation = 0"
    ).fetch_one(&pool).await?;
    assert_eq!(retained, mask, "never rewrite existing execution metadata");
    assert!(!is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn modern_handles_keep_distinct_authoritative_origins(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    replay(&mut db, events(true, Executor::Current, 1)).await?;
    replay(&mut db, events(true, Executor::Current, 1)).await?;
    let masks: Vec<Vec<u8>> = sqlx::query_scalar(
        "SELECT operand_boundary_mask FROM computations WHERE fhe_operation = 0 ORDER BY operand_boundary_mask"
    ).fetch_all(&pool).await?;
    assert_eq!(
        masks,
        vec![
            U256::ZERO.to_be_bytes::<32>().to_vec(),
            U256::from(1).to_be_bytes::<32>().to_vec(),
        ]
    );
    assert!(!is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn modern_origin_changes_still_halt_replay() -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    // The incomplete delivery omits the remint, so the last modern operation
    // is initially assigned a boundary input. A complete refetch must contain
    // that mismatch rather than silently changing the worker's sourcing rule.
    let complete = events(true, Executor::Current, 1);
    let partial = complete
        .iter()
        .filter(|log| log.log_index != Some(3))
        .cloned()
        .collect();
    replay(&mut db, partial).await?;
    assert!(replay(&mut db, complete).await.is_err());
    assert!(is_halted(&pool).await?);
    Ok(())
}
