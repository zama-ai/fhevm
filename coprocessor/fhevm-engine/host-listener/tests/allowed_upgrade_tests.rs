//! A computation first ingested without its ACL allow event (e.g. a partial
//! get_logs while recovering from a reorg) must become schedulable once the
//! same block is re-ingested with the allow event.

mod ingest_fixtures;
use ingest_fixtures::*;

use alloy::primitives::{Address, FixedBytes, U256};
use alloy::rpc::types::Log;
use alloy::sol_types::SolEvent;
use fhevm_engine_common::chain_id::ChainId;
use host_listener::cmd::block_history::BlockSummary;
use host_listener::contracts::{AclContract, TfheContract};
use host_listener::database::ingest::{ingest_block_logs, BlockLogs};
use host_listener::database::tfhe_event_propagate::Database;
use serial_test::serial;

alloy::sol! {
    interface OtherContract {
        // The selector is identical to Ownable's event, but indexing differs.
        event OwnershipTransferred(address previousOwner, address newOwner);
    }
}

async fn ingest(
    db: &mut Database,
    block: &BlockSummary,
    logs: Vec<Log>,
    catchup: bool,
) -> anyhow::Result<()> {
    ingest_filtered(db, block, logs, catchup, OPTIONS, Some(Address::ZERO))
        .await
}

fn block() -> BlockSummary {
    BlockSummary {
        number: 100,
        hash: FixedBytes::with_last_byte(0xb1),
        parent_hash: FixedBytes::with_last_byte(0xb0),
        timestamp: 1_790_000_000,
    }
}

#[tokio::test]
#[serial(db)]
async fn computation_upgraded_when_allow_event_arrives_later(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let block = block();
    let tx_hash = FixedBytes::<32>::with_last_byte(0x9f);
    let handle = computed_handle(0x01);

    // First ingestion misses the ACL allow event.
    ingest(
        &mut db,
        &block,
        vec![rpc_log(&trivial_encrypt(handle), &block, tx_hash, 0)],
        false,
    )
    .await?;
    assert_eq!(computation_flags(&pool, handle).await?, (false, true));
    // The worker finds no work and marks the chain processed.
    sqlx::query("UPDATE dependence_chain SET status = 'processed'")
        .execute(&pool)
        .await?;
    assert_eq!(schedulable(&pool, tx_hash).await?, 0);

    // Re-ingestion of the same block, now with the allow event.
    ingest(
        &mut db,
        &block,
        vec![
            rpc_log(&trivial_encrypt(handle), &block, tx_hash, 0),
            rpc_log(&allowed(handle), &block, tx_hash, 1),
        ],
        true,
    )
    .await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    assert_eq!(chain_status(&pool, tx_hash).await?, "updated");
    assert_eq!(schedulable(&pool, tx_hash).await?, 1);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn computation_not_upgraded_when_ciphertext_exists() -> anyhow::Result<()>
{
    let (mut db, pool, _instance) = setup().await?;
    let block = block();
    let tx_hash = FixedBytes::<32>::with_last_byte(0x9e);
    let handle = computed_handle(0x02);

    ingest(
        &mut db,
        &block,
        vec![rpc_log(&trivial_encrypt(handle), &block, tx_hash, 0)],
        false,
    )
    .await?;
    sqlx::query(
        "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
         VALUES ($1, '\\x00', 0, 5)",
    )
    .bind(handle.to_vec())
    .execute(&pool)
    .await?;

    ingest(
        &mut db,
        &block,
        vec![
            rpc_log(&trivial_encrypt(handle), &block, tx_hash, 0),
            rpc_log(&allowed(handle), &block, tx_hash, 1),
        ],
        true,
    )
    .await?;
    assert_eq!(computation_flags(&pool, handle).await?, (false, true));
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn allowed_first_insert_unchanged() -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let block = block();
    let tx_hash = FixedBytes::<32>::with_last_byte(0x9d);
    let handle = computed_handle(0x03);
    let logs = || {
        vec![
            rpc_log(&trivial_encrypt(handle), &block, tx_hash, 0),
            rpc_log(&allowed(handle), &block, tx_hash, 1),
        ]
    };

    ingest(&mut db, &block, logs(), false).await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    // A duplicate ingestion is still a no-op.
    ingest(&mut db, &block, logs(), true).await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    assert_eq!(schedulable(&pool, tx_hash).await?, 1);
    Ok(())
}

async fn replay_with_different_chain(cleanup: bool) -> anyhow::Result<()> {
    let (mut db, pool, instance) = setup().await?;
    let parent = block();
    let parent_tx = FixedBytes::<32>::with_last_byte(0xa0);
    let child_tx = FixedBytes::<32>::with_last_byte(0xa1);
    let input = computed_handle(0x10);
    let output = computed_handle(0x11);
    ingest(
        &mut db,
        &parent,
        vec![
            rpc_log(&trivial_encrypt(input), &parent, parent_tx, 0),
            rpc_log(&allowed(input), &parent, parent_tx, 1),
        ],
        false,
    )
    .await?;
    let child = BlockSummary {
        number: 101,
        hash: FixedBytes::with_last_byte(0xb2),
        parent_hash: parent.hash,
        timestamp: parent.timestamp + 1,
    };
    let op = TfheContract::FheAdd {
        caller: Address::ZERO,
        lhs: input,
        rhs: input,
        scalarByte: FixedBytes::ZERO,
        result: output,
    };
    ingest(
        &mut db,
        &child,
        vec![rpc_log(&op, &child, child_tx, 0)],
        false,
    )
    .await?;
    let original: Vec<u8> = sqlx::query_scalar(
        "SELECT dependence_chain_id FROM computations WHERE output_handle = $1",
    )
    .bind(output.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(original, parent_tx.to_vec());
    sqlx::query(
        "UPDATE computations SET is_completed = TRUE WHERE output_handle = $1",
    )
    .bind(input.to_vec())
    .execute(&pool)
    .await?;
    assert_eq!(computation_flags(&pool, output).await?, (false, true));
    sqlx::query("UPDATE dependence_chain SET status = 'processed'")
        .execute(&pool)
        .await?;
    // Simulate normal worker cleanup of an old processed chain.
    if cleanup {
        sqlx::query(
            "DELETE FROM dependence_chain WHERE dependence_chain_id = $1",
        )
        .bind(original)
        .execute(&pool)
        .await?;
    }
    // A fresh catch-up process no longer has the original in-memory chain cache.
    let mut replay_db =
        Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    ingest(
        &mut replay_db,
        &child,
        vec![
            rpc_log(&op, &child, child_tx, 0),
            rpc_log(&allowed(output), &child, child_tx, 1),
        ],
        true,
    )
    .await?;
    assert_eq!(computation_flags(&pool, output).await?, (true, false));
    let persisted_chain: Vec<u8> = sqlx::query_scalar(
        "SELECT dependence_chain_id FROM computations WHERE output_handle = $1",
    )
    .bind(output.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(persisted_chain, parent_tx.to_vec());
    assert_eq!(chain_status(&pool, parent_tx).await?, "updated");
    // The cold replay inferred the child's transaction as a separate chain.
    assert_eq!(chain_status(&pool, child_tx).await?, "updated");
    let reachable: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM computations c JOIN dependence_chain dc USING (dependence_chain_id)
         WHERE c.output_handle = $1 AND c.is_allowed AND NOT c.is_completed
         AND NOT c.is_error AND dc.status = 'updated'
         AND dc.dependency_count = 0 AND dc.worker_id IS NULL"
    ).bind(output.to_vec()).fetch_one(&pool).await?;
    assert_eq!(reachable, 1, "upgraded work must have a schedulable chain");
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn computation_upgrade_requeues_original_chain_with_different_replay_chain(
) -> anyhow::Result<()> {
    replay_with_different_chain(false).await
}

#[tokio::test]
#[serial(db)]
async fn computation_upgrade_recreates_cleaned_up_original_chain(
) -> anyhow::Result<()> {
    replay_with_different_chain(true).await
}

#[tokio::test]
#[serial(db)]
async fn replay_repairs_counts_without_duplicate_or_finalization_regression(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let block = block();
    let tx_hash = FixedBytes::<32>::with_last_byte(0x9c);
    let handle = computed_handle(0x04);
    let incomplete =
        || vec![rpc_log(&trivial_encrypt(handle), &block, tx_hash, 0)];
    let complete = || {
        vec![
            rpc_log(&trivial_encrypt(handle), &block, tx_hash, 0),
            rpc_log(&allowed(handle), &block, tx_hash, 1),
        ]
    };
    ingest(&mut db, &block, incomplete(), false).await?;
    ingest(&mut db, &block, complete(), true).await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    for logs in [complete(), incomplete(), complete()] {
        ingest(&mut db, &block, logs, true).await?;
        let counts: (i32, i32) = sqlx::query_as(
            "SELECT fhe_event_count, allow_event_count FROM host_chain_blocks_valid WHERE chain_id=42 AND block_hash=$1"
        ).bind(block.hash.to_vec()).fetch_one(&pool).await?;
        assert_eq!(counts, (1, 1));
    }
    let mut tx = db.new_transaction().await?.expect("active stack");
    db.mark_block_as_valid(&mut tx, &block, true, 0, 0).await?;
    tx.commit().await?;
    let counts: (i32, i32) = sqlx::query_as(
        "SELECT fhe_event_count, allow_event_count FROM host_chain_blocks_valid WHERE chain_id=42 AND block_hash=$1"
    ).bind(block.hash.to_vec()).fetch_one(&pool).await?;
    assert_eq!(counts, (1, 1));
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn wildcard_envelope_collision_does_not_reject_an_unrelated_event(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let block = block();
    let event = OtherContract::OwnershipTransferred {
        previousOwner: Address::with_last_byte(1),
        newOwner: Address::with_last_byte(2),
    };
    assert_eq!(
        OtherContract::OwnershipTransferred::SIGNATURE_HASH,
        AclContract::OwnershipTransferred::SIGNATURE_HASH,
    );
    let log = rpc_log(&event, &block, FixedBytes::with_last_byte(0x90), 0);

    // With an explicitly matched ACL address, this remains a malformed known
    // event and must stop ingestion. Wildcard mode cannot claim that ownership.
    assert!(ingest(&mut db, &block, vec![log.clone()], false)
        .await
        .unwrap_err()
        .to_string()
        .contains("malformed known ACL"));
    ingest_block_logs(
        ChainId::try_from(42_u64)?,
        &mut db,
        &BlockLogs {
            logs: vec![log],
            summary: block,
            catchup: false,
            finalized: false,
        },
        &None,
        &None,
        &None,
        &None,
        &None,
        OPTIONS,
    )
    .await?;
    let counts: (i64, i64) = sqlx::query_as(
        "SELECT fhe_event_count::bigint, allow_event_count::bigint
         FROM host_chain_blocks_valid WHERE chain_id = 42 AND block_hash = $1",
    )
    .bind(block.hash.as_slice())
    .fetch_one(&pool)
    .await?;
    assert_eq!(counts, (0, 0));
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn malformed_known_envelopes_rollback_and_allow_valid_retry(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let block = block();
    let tx_hash = FixedBytes::with_last_byte(0x91);
    let handle = computed_handle(0x21);
    db.poller_set_last_caught_up_block(ChainId::try_from(42_u64)?, 99)
        .await?;
    let valid = || {
        vec![
            rpc_log(&trivial_encrypt(handle), &block, tx_hash, 0),
            rpc_log(&allowed(handle), &block, tx_hash, 1),
        ]
    };
    for index in [0, 1] {
        let mut logs = valid();
        // Retain a recognized selector and topics but omit its required data.
        logs[index].inner.data = alloy::primitives::LogData::new_unchecked(
            logs[index].inner.data.topics().to_vec(),
            Default::default(),
        );
        let err = ingest(&mut db, &block, logs, false).await.unwrap_err();
        assert!(err.to_string().contains("malformed known"), "{err}");
        let counts: (i64, i64, i64) = sqlx::query_as(
            "SELECT (SELECT COUNT(*) FROM computations),
                    (SELECT COUNT(*) FROM allowed_handles),
                    (SELECT COUNT(*) FROM host_chain_blocks_valid WHERE chain_id=42)",
        ).fetch_one(&pool).await?;
        assert_eq!(counts, (0, 0, 0), "rejected block must be atomic");
        let cursor: i64 = sqlx::query_scalar(
            "SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id=42",
        ).fetch_one(&pool).await?;
        assert_eq!(cursor, 99);
    }
    ingest(&mut db, &block, valid(), false).await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn unknown_envelopes_do_not_prevent_valid_events() -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let block = block();
    let tx_hash = FixedBytes::with_last_byte(0x92);
    let handle = computed_handle(0x22);
    let mut unknown = rpc_log(&allowed(handle), &block, tx_hash, 0);
    unknown.inner.data = alloy::primitives::LogData::new_unchecked(
        vec![FixedBytes::with_last_byte(0xee)],
        Default::default(),
    );
    ingest(
        &mut db,
        &block,
        vec![
            unknown,
            rpc_log(&trivial_encrypt(handle), &block, tx_hash, 1),
            rpc_log(&allowed(handle), &block, tx_hash, 2),
        ],
        false,
    )
    .await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn shift_replay_preserves_legacy_and_current_contract_amounts(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let block = block();
    let tx_hash = FixedBytes::with_last_byte(0x94);
    let input = computed_handle(0x30); // Uint64: legacy scalar range exceeds uint8.
    let current_output = computed_handle(0x31);
    let legacy_output = computed_handle(0x32);
    let shift = |amount: u64, result| TfheContract::FheShl {
        caller: Address::ZERO,
        lhs: input,
        rhs: FixedBytes::from(U256::from(amount).to_be_bytes::<32>()),
        scalarByte: FixedBytes::with_last_byte(1),
        result,
    };
    let logs = || {
        vec![
            rpc_log(&trivial_encrypt(input), &block, tx_hash, 0),
            rpc_log(&shift(255, current_output), &block, tx_hash, 1),
            rpc_log(&shift(256, legacy_output), &block, tx_hash, 2),
            rpc_log(&allowed(current_output), &block, tx_hash, 3),
            rpc_log(&allowed(legacy_output), &block, tx_hash, 4),
        ]
    };
    ingest(&mut db, &block, logs(), false).await?;
    ingest(&mut db, &block, logs(), true).await?;
    for output in [current_output, legacy_output] {
        assert_eq!(computation_flags(&pool, output).await?, (true, false));
    }
    let scalar: Vec<Vec<u8>> = sqlx::query_scalar(
        "SELECT dependencies FROM computations WHERE output_handle=$1",
    )
    .bind(legacy_output.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(
        scalar[1],
        U256::from(256).to_be_bytes::<32>().to_vec(),
        "historical replay must preserve the original amount, not truncate it"
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn restart_replay_preserves_migrated_legacy_masks_and_ciphertexts(
) -> anyhow::Result<()> {
    use fhevm_engine_common::chain_recovery::{
        is_halted, prepare_host_replay, readiness_blocked, HostReplayStartup,
    };
    for materialized in [false, true] {
        let (mut db, pool, _instance) = setup().await?;
        let block = block();
        let output = computed_handle(0x25);
        let tx_hash = FixedBytes::with_last_byte(0x94);
        let event = trivial_encrypt(output);
        let logs = vec![
            rpc_log(&event, &block, tx_hash, 0),
            rpc_log(&allowed(output), &block, tx_hash, 1),
        ];
        ingest(&mut db, &block, logs.clone(), false).await?;
        // This is the schema migration's retained pre-upgrade row shape:
        // no semantic backfill of the legacy boundary mask.
        sqlx::query("UPDATE computations SET operand_boundary_mask=NULL, is_completed=$1")
            .bind(materialized).execute(&pool).await?;
        if materialized {
            sqlx::query("INSERT INTO ciphertexts (handle,ciphertext,ciphertext_version,ciphertext_type) VALUES ($1,decode('cafe','hex'),0,5)")
                .bind(output.to_vec()).execute(&pool).await?;
        }
        db.poller_set_last_caught_up_block(ChainId::try_from(42_u64)?, 100)
            .await?;
        let HostReplayStartup::Ready {
            replay: Some(replay),
            ..
        } = prepare_host_replay(&pool, false, 42, 1, None).await?
        else {
            panic!("restart must replay the retained boundary");
        };
        ingest(&mut db, &block, logs, true).await?;
        assert!(!replay.complete(&pool, 100).await?);
        assert!(!readiness_blocked(&pool).await?);
        assert!(!is_halted(&pool).await?);
        let mask: Option<Vec<u8>> = sqlx::query_scalar(
            "SELECT operand_boundary_mask FROM computations WHERE output_handle=$1",
        ).bind(output.to_vec()).fetch_one(&pool).await?;
        assert!(
            mask.is_none(),
            "legacy execution semantics must not be backfilled"
        );
        assert_eq!(
            computation_flags(&pool, output).await?,
            (true, materialized)
        );
        let ciphertext: Option<Vec<u8>> = sqlx::query_scalar(
            "SELECT ciphertext FROM ciphertexts WHERE handle=$1",
        )
        .bind(output.to_vec())
        .fetch_optional(&pool)
        .await?;
        assert_eq!(ciphertext, materialized.then_some(vec![0xca, 0xfe]));

        // Missing legacy metadata is not permission to change recorded operands.
        let mut changed = event;
        changed.pt = U256::from(8);
        assert!(ingest(
            &mut db,
            &block,
            vec![rpc_log(&changed, &block, tx_hash, 0)],
            true
        )
        .await
        .is_err());
        assert!(is_halted(&pool).await?);
        db.pool().await.close().await;
        pool.close().await;
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn replay_contains_changed_operand_origin_before_releasing_work(
) -> anyhow::Result<()> {
    use fhevm_engine_common::chain_recovery::{
        begin_replay, is_halted, readiness_blocked, ListenerSource,
    };
    for materialized in [false, true] {
        let (mut db, pool, _instance) = setup().await?;
        let block = block();
        let tx_hash = FixedBytes::with_last_byte(0x93);
        let input = computed_handle(0x23);
        let output = computed_handle(0x24);
        let consumer = TfheContract::FheAdd {
            caller: Address::ZERO,
            lhs: input,
            rhs: input,
            scalarByte: FixedBytes::ZERO,
            result: output,
        };
        // Initial observation missed the transaction-local producer.
        ingest(
            &mut db,
            &block,
            vec![
                rpc_log(&consumer, &block, tx_hash, 1),
                rpc_log(&allowed(output), &block, tx_hash, 2),
            ],
            false,
        )
        .await?;
        let mask: Vec<u8> = sqlx::query_scalar(
            "SELECT operand_boundary_mask FROM computations WHERE output_handle=$1",
        ).bind(output.to_vec()).fetch_one(&pool).await?;
        assert!(mask.iter().any(|byte| *byte != 0));
        if materialized {
            sqlx::query("UPDATE computations SET is_completed=TRUE")
                .execute(&pool)
                .await?;
            sqlx::query("INSERT INTO ciphertexts (handle,ciphertext,ciphertext_version,ciphertext_type) VALUES ($1,decode('01','hex'),0,5)")
                .bind(output.to_vec()).execute(&pool).await?;
        }
        begin_replay(&pool, ListenerSource::Host, false, 42, 100, 100).await?;
        let err = ingest(
            &mut db,
            &block,
            vec![
                rpc_log(&trivial_encrypt(input), &block, tx_hash, 0),
                rpc_log(&consumer, &block, tx_hash, 1),
                rpc_log(&allowed(output), &block, tx_hash, 2),
            ],
            true,
        )
        .await
        .unwrap_err();
        assert!(
            err.to_string().contains("execution metadata differs"),
            "{err}"
        );
        assert!(is_halted(&pool).await?);
        assert!(readiness_blocked(&pool).await?);
        let retained: Vec<u8> = sqlx::query_scalar(
            "SELECT operand_boundary_mask FROM computations WHERE output_handle=$1",
        ).bind(output.to_vec()).fetch_one(&pool).await?;
        assert_eq!(retained, mask, "preserve original execution evidence");
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM computations")
                .fetch_one(&pool)
                .await?;
        assert_eq!(count, 1, "the newly seen producer must roll back too");
        let pending: bool = sqlx::query_scalar("SELECT pending FROM public.listener_replay_state WHERE source='host' AND chain_id=42")
            .fetch_one(&pool).await?;
        assert!(pending, "failed replay must not release derived work");
        db.pool().await.close().await;
        pool.close().await;
    }
    Ok(())
}

async fn set_chain_status(
    pool: &sqlx::PgPool,
    dcid: FixedBytes<32>,
    status: &str,
) -> anyhow::Result<()> {
    sqlx::query(
        "UPDATE dependence_chain SET status = $2 WHERE dependence_chain_id = $1",
    )
    .bind(dcid.to_vec())
    .bind(status)
    .execute(pool)
    .await?;
    Ok(())
}

/// Ingest one allowed computation, then put its chain in `prepare`'s state and
/// re-ingest the identical block. Returns the chain status afterwards (None if
/// the chain row does not exist) and the number of schedulable rows.
async fn reingest_after(
    prepare: &str,
    tx_byte: u8,
    handle_byte: u8,
) -> anyhow::Result<(Option<String>, i64, bool)> {
    let (mut db, pool, _instance) = setup().await?;
    let block = block();
    let tx_hash = computed_handle(tx_byte);
    let handle = computed_handle(handle_byte);
    let logs = || {
        vec![
            rpc_log(&trivial_encrypt(handle), &block, tx_hash, 0),
            rpc_log(&allowed(handle), &block, tx_hash, 1),
        ]
    };
    ingest(&mut db, &block, logs(), false).await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    match prepare {
        "processed" => set_chain_status(&pool, tx_hash, "processed").await?,
        "missing" => {
            sqlx::query(
                "DELETE FROM dependence_chain WHERE dependence_chain_id = $1",
            )
            .bind(tx_hash.to_vec())
            .execute(&pool)
            .await?;
        }
        "processing" => {
            sqlx::query(
                "UPDATE dependence_chain SET status = 'processing',
                   worker_id = gen_random_uuid(), lock_acquired_at = NOW(),
                   lock_expires_at = NOW() + INTERVAL '2 minutes'
                 WHERE dependence_chain_id = $1",
            )
            .bind(tx_hash.to_vec())
            .execute(&pool)
            .await?;
        }
        "computed" => {
            sqlx::query(
                "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
                 VALUES ($1, '\\x00', 0, 5)",
            )
            .bind(handle.to_vec())
            .execute(&pool)
            .await?;
            sqlx::query("UPDATE computations SET is_completed = TRUE WHERE output_handle = $1")
                .bind(handle.to_vec())
                .execute(&pool)
                .await?;
            set_chain_status(&pool, tx_hash, "processed").await?;
        }
        other => anyhow::bail!("unknown preparation {other}"),
    }
    // A fresh process re-ingests the same block (e.g. catch-up from an earlier
    // block after the fix is deployed).
    let mut replay_db =
        Database::new(&_instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    ingest(&mut replay_db, &block, logs(), true).await?;
    let status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(tx_hash.to_vec())
    .fetch_optional(&pool)
    .await?;
    let locked: Option<bool> = sqlx::query_scalar(
        "SELECT worker_id IS NOT NULL FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(tx_hash.to_vec())
    .fetch_optional(&pool)
    .await?;
    Ok((
        status,
        schedulable(&pool, tx_hash).await?,
        locked.unwrap_or(false),
    ))
}

#[tokio::test]
#[serial(db)]
async fn pending_computation_rearms_processed_chain() -> anyhow::Result<()> {
    let (status, pending, _) = reingest_after("processed", 0xc0, 0x20).await?;
    assert_eq!(status.as_deref(), Some("updated"));
    assert_eq!(pending, 1);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn pending_computation_recreates_missing_chain() -> anyhow::Result<()> {
    let (status, pending, _) = reingest_after("missing", 0xc1, 0x21).await?;
    assert_eq!(status.as_deref(), Some("updated"));
    assert_eq!(pending, 1);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn pending_computation_leaves_processing_chain_alone(
) -> anyhow::Result<()> {
    let (status, pending, locked) =
        reingest_after("processing", 0xc2, 0x22).await?;
    // The re-arm path only touches processed or missing chains: the owning
    // worker keeps its lock (the block-level upsert may still flag the chain
    // as updated, which the worker's release preserves).
    assert!(locked, "the processing worker must keep its lock");
    assert!(status.is_some());
    assert_eq!(pending, 1);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn computed_computation_does_not_rearm() -> anyhow::Result<()> {
    let (status, pending, _) = reingest_after("computed", 0xc3, 0x23).await?;
    // The block-level upsert may mark the chain updated, but there is no
    // schedulable work: nothing is recomputed.
    assert!(status.is_some());
    assert_eq!(pending, 0);
    Ok(())
}

/// 2026-10-14 testnet (artifact): a 1-block reorg replaced block N. The worker
/// ran the producer transaction's chain for the orphan version and marked it
/// processed; the canonical version of the same transaction then arrived with
/// different (block-dependent) output handles. Those must be computed.
#[tokio::test]
#[serial(db)]
async fn canonical_version_after_orphan_chain_processed() -> anyhow::Result<()>
{
    let (mut db, pool, instance) = setup().await?;
    let orphan = block();
    let canonical = BlockSummary {
        hash: computed_handle(0xbf),
        ..orphan
    };
    let producer_tx = computed_handle(0xd0);
    let orphan_out = computed_handle(0x30);
    let canonical_out = computed_handle(0x31);

    ingest(
        &mut db,
        &orphan,
        vec![
            rpc_log(&trivial_encrypt(orphan_out), &orphan, producer_tx, 0),
            rpc_log(&allowed(orphan_out), &orphan, producer_tx, 1),
        ],
        false,
    )
    .await?;
    // The worker computes the orphan version and marks the chain processed.
    sqlx::query(
        "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
         VALUES ($1, '\\x00', 0, 5)",
    )
    .bind(orphan_out.to_vec())
    .execute(&pool)
    .await?;
    sqlx::query(
        "UPDATE computations SET is_completed = TRUE WHERE output_handle = $1",
    )
    .bind(orphan_out.to_vec())
    .execute(&pool)
    .await?;
    set_chain_status(&pool, producer_tx, "processed").await?;

    // The canonical version of the block arrives.
    ingest(
        &mut db,
        &canonical,
        vec![
            rpc_log(
                &trivial_encrypt(canonical_out),
                &canonical,
                producer_tx,
                0,
            ),
            rpc_log(&allowed(canonical_out), &canonical, producer_tx, 1),
        ],
        false,
    )
    .await?;
    assert_eq!(
        computation_flags(&pool, canonical_out).await?,
        (true, false)
    );
    let reachable = |out: FixedBytes<32>| {
        let pool = pool.clone();
        async move {
            sqlx::query_scalar::<_, i64>(
                "SELECT count(*) FROM computations c JOIN dependence_chain dc USING (dependence_chain_id)
                 WHERE c.output_handle = $1 AND c.is_allowed AND NOT c.is_completed
                   AND NOT c.is_error AND dc.status = 'updated' AND dc.worker_id IS NULL",
            )
            .bind(out.to_vec())
            .fetch_one(&pool)
            .await
        }
    };
    assert_eq!(reachable(canonical_out).await?, 1);

    // A stale processed status must also be repaired when the canonical
    // block is replayed; this models recovery, not the origin of that status.
    set_chain_status(&pool, producer_tx, "processed").await?;
    assert_eq!(reachable(canonical_out).await?, 0);
    let mut replay_db =
        Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    ingest(
        &mut replay_db,
        &canonical,
        vec![
            rpc_log(
                &trivial_encrypt(canonical_out),
                &canonical,
                producer_tx,
                0,
            ),
            rpc_log(&allowed(canonical_out), &canonical, producer_tx, 1),
        ],
        true,
    )
    .await?;
    assert_eq!(reachable(canonical_out).await?, 1);
    Ok(())
}
