//! Tests for the synthetic Gateway input the GCS gw-listener injects so the
//! input-verification consensus track can anchor without user traffic.
//!
//! The determinism test is the important one: the whole approach rests on every operator
//! producing byte-identical proof bytes from the same on-chain context.

use std::time::Duration;

use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::synthetic_input::{
    build_synthetic_input, load_input_proving_material, synthetic_aux_data, synthetic_input_seed,
    synthetic_zk_proof_id, SyntheticInputContext, SYNTHETIC_GW_BLOCK_OFFSET,
    SYNTHETIC_ZK_PROOF_ID_BASE,
};
use gw_listener::synthetic_input::maybe_inject_synthetic_input;
use serial_test::serial;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use test_harness::instance::ImportMode;

/// Chain seeded into `host_chains` by the test harness.
const TEST_CHAIN_ID: i64 = 12345;
const GW_START_BLOCK: i64 = 500;
const TRIGGER_BLOCK: i64 = GW_START_BLOCK + SYNTHETIC_GW_BLOCK_OFFSET;
const NOTIFY_CHANNEL: &str = "event_zkpok_new_work";

struct TestDb {
    pool: Pool<Postgres>,
    _instance: test_harness::instance::DBInstance,
}

async fn setup() -> anyhow::Result<TestDb> {
    let instance = test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
        .await
        .expect("valid db instance");
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .acquire_timeout(Duration::from_secs(5))
        .connect(instance.db_url.as_str())
        .await?;
    sqlx::query("TRUNCATE verify_proofs").execute(&pool).await?;
    sqlx::query("DELETE FROM upgrade_state")
        .execute(&pool)
        .await?;
    Ok(TestDb {
        pool,
        _instance: instance,
    })
}

/// Seed the GCS row the injector reads its plan from.
async fn seed_gcs_upgrade(pool: &Pool<Postgres>, state: &str) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO upgrade_state (
             stack_role, state, status, proposal_id, version,
             start_block, end_block, gw_start_block, host_chain_id, proposal_block, updated_at
         )
         VALUES ('GCS', $1, 'in_progress', $2, '0.15.0',
                 100, 200, $3, $4, 100, NOW())",
    )
    .bind(state)
    .bind(vec![7u8; 32])
    .bind(GW_START_BLOCK)
    .bind(TEST_CHAIN_ID)
    .execute(pool)
    .await?;
    Ok(())
}

async fn synthetic_row_count(pool: &Pool<Postgres>) -> anyhow::Result<i64> {
    Ok(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM verify_proofs WHERE zk_proof_id >= $1")
            .bind(SYNTHETIC_ZK_PROOF_ID_BASE)
            .fetch_one(pool)
            .await?,
    )
}

fn test_context() -> SyntheticInputContext<'static> {
    SyntheticInputContext {
        proposal_id: &[7u8; 32],
        target_version: "0.15.0",
        host_chain_id: ChainId::try_from(TEST_CHAIN_ID as u64).unwrap(),
        gw_block_number: TRIGGER_BLOCK,
    }
}

/// Blue must never inject: it would be writing dry-run probes into live production tables.
#[tokio::test]
#[serial(db)]
async fn bcs_mode_never_injects() -> anyhow::Result<()> {
    let db = setup().await?;
    seed_gcs_upgrade(&db.pool, "UpgradeActivated").await?;

    let injected =
        maybe_inject_synthetic_input(&db.pool, false, TRIGGER_BLOCK as u64, NOTIFY_CHANNEL).await?;

    assert!(!injected, "BCS injected a synthetic input");
    assert_eq!(synthetic_row_count(&db.pool).await?, 0);
    Ok(())
}

/// Outside a dry-run window there is no proposal to anchor, so nothing should be written.
#[tokio::test]
#[serial(db)]
async fn no_active_upgrade_is_a_noop() -> anyhow::Result<()> {
    let db = setup().await?;

    let injected =
        maybe_inject_synthetic_input(&db.pool, true, TRIGGER_BLOCK as u64, NOTIFY_CHANNEL).await?;

    assert!(!injected);
    assert_eq!(synthetic_row_count(&db.pool).await?, 0);
    Ok(())
}

/// Before the trigger block nothing is written; from the trigger block onward it is. The
/// trigger is a floor rather than an exact match so a transient failure at that exact block
/// does not lose the window's only chance to anchor.
#[tokio::test]
#[serial(db)]
async fn injects_once_the_trigger_block_is_reached() -> anyhow::Result<()> {
    let db = setup().await?;
    seed_gcs_upgrade(&db.pool, "DryRunStarted").await?;

    let before =
        maybe_inject_synthetic_input(&db.pool, true, (TRIGGER_BLOCK - 1) as u64, NOTIFY_CHANNEL)
            .await?;
    assert!(!before, "injected before the trigger block");
    assert_eq!(synthetic_row_count(&db.pool).await?, 0);

    // A later tick still injects, and records the deterministic trigger block rather than the
    // block it happened to run on - otherwise operators would anchor on different blocks.
    let late =
        maybe_inject_synthetic_input(&db.pool, true, (TRIGGER_BLOCK + 30) as u64, NOTIFY_CHANNEL)
            .await?;
    assert!(late, "a tick past the trigger block did not inject");
    assert_eq!(synthetic_row_count(&db.pool).await?, 1);

    let block_number: Option<i64> =
        sqlx::query_scalar("SELECT block_number FROM verify_proofs WHERE zk_proof_id >= $1")
            .bind(SYNTHETIC_ZK_PROOF_ID_BASE)
            .fetch_one(&db.pool)
            .await?;
    assert_eq!(
        block_number,
        Some(TRIGGER_BLOCK),
        "a late injection must still record the deterministic trigger block"
    );
    Ok(())
}

/// The row must land in the shape the zkproof-worker expects to pick up: unverified, on a
/// configured host chain, with the trigger block recorded.
#[tokio::test]
#[serial(db)]
async fn injected_row_is_shaped_for_the_zkproof_worker() -> anyhow::Result<()> {
    let db = setup().await?;
    seed_gcs_upgrade(&db.pool, "UpgradeActivated").await?;

    assert!(
        maybe_inject_synthetic_input(&db.pool, true, TRIGGER_BLOCK as u64, NOTIFY_CHANNEL).await?
    );

    let row = sqlx::query(
        "SELECT zk_proof_id, chain_id, block_number, verified, retry_count,
                octet_length(input) AS input_len, handles
           FROM verify_proofs
          WHERE zk_proof_id >= $1",
    )
    .bind(SYNTHETIC_ZK_PROOF_ID_BASE)
    .fetch_one(&db.pool)
    .await?;

    assert_eq!(
        row.try_get::<i64, _>("zk_proof_id")?,
        synthetic_zk_proof_id(&test_context()),
        "id is not the deterministic one every operator derives"
    );
    assert_eq!(row.try_get::<i64, _>("chain_id")?, TEST_CHAIN_ID);
    assert_eq!(
        row.try_get::<Option<i64>, _>("block_number")?,
        Some(TRIGGER_BLOCK)
    );
    // `verified IS NULL` is exactly what the zkproof-worker's acquisition query selects on.
    assert_eq!(row.try_get::<Option<bool>, _>("verified")?, None);
    assert_eq!(row.try_get::<i32, _>("retry_count")?, 0);
    // `octet_length` is INT4 in Postgres, so this decodes as i32, not i64.
    assert!(row.try_get::<Option<i32>, _>("input_len")?.unwrap_or(0) > 0);
    assert!(row.try_get::<Option<Vec<u8>>, _>("handles")?.is_none());
    Ok(())
}

/// Re-processing the same range after a restart must not re-prove (seconds of CPU) or
/// duplicate the row.
#[tokio::test]
#[serial(db)]
async fn injection_is_idempotent() -> anyhow::Result<()> {
    let db = setup().await?;
    seed_gcs_upgrade(&db.pool, "UpgradeActivated").await?;

    assert!(
        maybe_inject_synthetic_input(&db.pool, true, TRIGGER_BLOCK as u64, NOTIFY_CHANNEL).await?
    );
    let second =
        maybe_inject_synthetic_input(&db.pool, true, TRIGGER_BLOCK as u64, NOTIFY_CHANNEL).await?;

    assert!(!second, "second pass re-injected");
    assert_eq!(synthetic_row_count(&db.pool).await?, 1);
    Ok(())
}

/// The synthetic row must be invisible to the transaction-sender: no contract requested this
/// input, so publishing a `verifyProofResponse` would revert with `VerifyProofNotRequested`.
///
/// Mirrors the sender's selection predicate, with the row forced into the state it reaches
/// after successful local verification - the only state from which it could be sent.
#[tokio::test]
#[serial(db)]
async fn synthetic_row_is_never_selected_for_sending() -> anyhow::Result<()> {
    let db = setup().await?;
    seed_gcs_upgrade(&db.pool, "UpgradeActivated").await?;
    assert!(
        maybe_inject_synthetic_input(&db.pool, true, TRIGGER_BLOCK as u64, NOTIFY_CHANNEL).await?
    );

    sqlx::query("UPDATE verify_proofs SET verified = TRUE, handles = $1 WHERE zk_proof_id >= $2")
        .bind(vec![0u8; 32])
        .bind(SYNTHETIC_ZK_PROOF_ID_BASE)
        .execute(&db.pool)
        .await?;

    let sendable: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM verify_proofs
          WHERE (verified = TRUE OR (verified = FALSE AND handles IS NOT NULL))
            AND retry_count < $1
            AND zk_proof_id < $2",
    )
    .bind(6_i64)
    .bind(SYNTHETIC_ZK_PROOF_ID_BASE)
    .fetch_one(&db.pool)
    .await?;

    assert_eq!(
        sendable, 0,
        "a synthetic input would be sent to the Gateway"
    );
    Ok(())
}

/// The load-bearing property: two operators building from the same on-chain context produce
/// byte-identical proof bytes, so their blob hashes, handles and state hashes agree.
#[tokio::test]
#[serial(db)]
async fn proof_bytes_are_identical_across_operators() -> anyhow::Result<()> {
    let db = setup().await?;

    let acl: String =
        sqlx::query_scalar("SELECT acl_contract_address FROM host_chains WHERE chain_id = $1")
            .bind(TEST_CHAIN_ID)
            .fetch_one(&db.pool)
            .await?;

    let ctx = test_context();
    let aux = synthetic_aux_data(ctx.host_chain_id, &acl)?;
    let seed = synthetic_input_seed(&ctx);

    // Two independent loads stand in for two operators reading the same key material.
    let first = load_input_proving_material(&db.pool).await?;
    let second = load_input_proving_material(&db.pool).await?;

    let blob_a = build_synthetic_input(&first, &aux, &seed)?;
    let blob_b = build_synthetic_input(&second, &aux, &seed)?;

    assert!(!blob_a.is_empty());
    assert_eq!(
        blob_a, blob_b,
        "synthetic proof bytes differ between operators — consensus could never be reached"
    );

    // A different window must produce a different proof, or a replayed blob from an earlier
    // attempt would be accepted as this attempt's evidence.
    let other_seed = synthetic_input_seed(&SyntheticInputContext {
        gw_block_number: TRIGGER_BLOCK + 1,
        ..ctx
    });
    let blob_c = build_synthetic_input(&first, &aux, &other_seed)?;
    assert_ne!(blob_a, blob_c, "seed does not affect the proof bytes");

    Ok(())
}
