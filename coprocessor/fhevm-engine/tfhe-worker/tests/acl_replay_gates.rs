//! Exercise CFI-0020 gate repair through the real worker lease/release API.
//! Ciphertexts are opaque fixtures; no TFHE operation is evaluated.

// The shared fixture is formatted by host-listener with its own rustfmt config.
#[rustfmt::skip]
#[path = "../../host-listener/tests/ingest_fixtures/mod.rs"]
mod ingest_fixtures;
use ingest_fixtures::*;

use alloy::primitives::{Address, FixedBytes};
use fhevm_engine_common::chain_id::ChainId;
use host_listener::contracts::TfheContract;
use host_listener::database::tfhe_event_propagate::Database;
use serial_test::serial;
use tfhe_worker::dependence_chain::LockMngr;
use uuid::Uuid;

#[tokio::test]
#[serial(db)]
async fn repaired_gate_preserves_sibling_debt_and_discharges_once() -> anyhow::Result<()> {
    let (mut db, pool, instance) = setup().await?;
    let block = block();
    let parent_a = FixedBytes::<32>::with_last_byte(0xa0);
    let parent_b = FixedBytes::<32>::with_last_byte(0xa1);
    let child = FixedBytes::<32>::with_last_byte(0xa2);
    let fork = FixedBytes::<32>::with_last_byte(0xa3);
    let input_a = computed_handle(0x10);
    let input_b = computed_handle(0x11);
    let output = computed_handle(0x12);
    let fork_output = computed_handle(0x13);
    let logs = |complete| {
        let mut logs = vec![
            rpc_log(&trivial_encrypt(input_a), &block, parent_a, 0),
            rpc_log(&trivial_encrypt(input_b), &block, parent_b, 2),
            rpc_log(&allowed(input_b), &block, parent_b, 3),
            rpc_log(
                &TfheContract::FheAdd {
                    caller: Address::ZERO,
                    lhs: input_a,
                    rhs: input_b,
                    scalarByte: FixedBytes::ZERO,
                    result: output,
                },
                &block,
                child,
                4,
            ),
            rpc_log(&allowed(output), &block, child, 5),
            rpc_log(
                &TfheContract::FheNeg {
                    caller: Address::ZERO,
                    ct: input_b,
                    result: fork_output,
                },
                &block,
                fork,
                6,
            ),
            rpc_log(&allowed(fork_output), &block, fork, 7),
        ];
        if complete {
            logs.push(rpc_log(&allowed(input_a), &block, parent_a, 1));
        }
        logs
    };
    ingest(&mut db, &block, logs(false), false).await?;
    let count = || async {
        sqlx::query_scalar::<_, i32>(
            "SELECT dependency_count FROM dependence_chain WHERE dependence_chain_id = $1",
        )
        .bind(child.as_slice())
        .fetch_one(&pool)
        .await
    };
    assert_eq!(count().await?, 1, "B already gates the child");
    let mut replay = Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128).await?;
    for _ in 0..3 {
        ingest(&mut replay, &block, logs(true), true).await?;
        assert_eq!(count().await?, 2, "add A once and retain the debt to B");
    }

    let mut worker = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
    let mut acquired = worker
        .acquire_next_locks(10)
        .await?
        .into_iter()
        .filter_map(|(id, _)| id)
        .collect::<Vec<_>>();
    acquired.sort();
    assert_eq!(acquired, vec![parent_a.to_vec(), parent_b.to_vec()]);

    for (parent, handle, remaining) in [(parent_a, input_a, 1), (parent_b, input_b, 0)] {
        sqlx::query("UPDATE computations SET is_completed = TRUE WHERE output_handle = $1")
            .bind(handle.as_slice())
            .execute(&pool)
            .await?;
        sqlx::query(
            "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
             VALUES ($1, '\\x00', 0, 5)",
        )
        .bind(handle.as_slice())
        .execute(&pool)
        .await?;
        assert_eq!(
            worker.release_locks(&[parent.to_vec()], true, None).await?,
            1
        );
        assert_eq!(count().await?, remaining);
        // A repeated release and a complete replay must not discharge another
        // parent's debt, or re-arm a gate for a materialized input.
        assert_eq!(
            worker.release_locks(&[parent.to_vec()], true, None).await?,
            0
        );
        ingest(&mut replay, &block, logs(true), true).await?;
        assert_eq!(count().await?, remaining);
    }
    let mut consumer = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
    let ready = consumer.acquire_next_locks(10).await?;
    assert!(ready
        .iter()
        .any(|(id, _)| id.as_deref() == Some(child.as_slice())));
    let retained: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM dependence_chain WHERE dependence_chain_id = ANY($1::bytea[])
         AND cardinality(dependents) <> 0",
    )
    .bind(&[parent_a.to_vec(), parent_b.to_vec()])
    .fetch_one(&pool)
    .await?;
    assert_eq!(retained, 0, "retirement prunes the repaired edges");
    Ok(())
}
