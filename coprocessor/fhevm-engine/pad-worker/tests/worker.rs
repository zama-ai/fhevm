use alloy::{primitives::B256, signers::local::PrivateKeySigner};
use ciphertext_attestation::{
    consensus::Attestation, prf::Version, PrfOutputAttestation, PrfOutputRef,
};
use pad_worker::{
    evaluator::{MockEvaluator, PadEvaluator, PadOutputs},
    rows, Config, PadWorker,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{
    num::NonZeroUsize,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};
use tokio::{task::JoinHandle, time::sleep};
use tokio_util::sync::CancellationToken;

const LANE: u16 = 1;
const DEADLINE: Duration = Duration::from_secs(20);

async fn db() -> (DBInstance, PgPool) {
    let instance = setup_test_db(ImportMode::None).await.expect("test db");
    let pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(instance.db_url())
        .await
        .unwrap();
    (instance, pool)
}

fn config() -> Config {
    Config {
        prf_instance: LANE,
        key_id: vec![1],
        max_concurrent: NonZeroUsize::new(4).unwrap(),
        claim_timeout: Duration::from_secs(60),
        poll_interval: Duration::from_millis(100),
        pending_max_age: Duration::from_secs(600),
    }
}

/// A pending row of `prf`, `label`, triggered by `connector`, created `age_secs` ago.
async fn add_row(pool: &PgPool, prf: u16, label: u8, connector: u8, age_secs: f64) {
    sqlx::query(
        "INSERT INTO pads (prf_instance, label, pad_length, triggered_by, created_at)
         VALUES ($1, $2, 64, $3, NOW() - make_interval(secs => $4))",
    )
    .bind(i32::from(prf))
    .bind(B256::repeat_byte(label).as_slice())
    .bind(vec![connector; 20])
    .bind(age_secs)
    .execute(pool)
    .await
    .unwrap();
}

fn start(
    pool: &PgPool,
    config: Config,
    evaluator: Arc<dyn PadEvaluator>,
) -> (
    PrivateKeySigner,
    CancellationToken,
    JoinHandle<anyhow::Result<()>>,
) {
    let signer = PrivateKeySigner::random();
    let cancel = CancellationToken::new();
    let worker = PadWorker::new(
        config,
        pool.clone(),
        evaluator,
        Arc::new(signer.clone()),
        cancel.clone(),
    );
    (signer, cancel, tokio::spawn(worker.run()))
}

/// Waits until `query` returns true.
async fn until(pool: &PgPool, query: &str) {
    let deadline = tokio::time::Instant::now() + DEADLINE;
    while !sqlx::query_scalar::<_, bool>(query)
        .fetch_one(pool)
        .await
        .unwrap()
    {
        assert!(tokio::time::Instant::now() < deadline, "timeout: {query}");
        sleep(Duration::from_millis(50)).await;
    }
}

async fn pending(pool: &PgPool, prf: u16, label: u8) -> bool {
    sqlx::query_scalar(
        "SELECT signature IS NULL AND NOT is_error FROM pads
          WHERE prf_instance = $1 AND label = $2",
    )
    .bind(i32::from(prf))
    .bind(B256::repeat_byte(label).as_slice())
    .fetch_one(pool)
    .await
    .unwrap()
}

/// `ciphertext128`, both digests, `signer`, `signature`, `attestation_version`.
type OutputRow = (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, i16);

/// Counts the evaluations.
#[derive(Default)]
struct Counting(AtomicUsize);

impl PadEvaluator for Counting {
    fn evaluate(&self, prf: u16, label: B256, pad_length: u16) -> anyhow::Result<PadOutputs> {
        self.0.fetch_add(1, Ordering::SeqCst);
        MockEvaluator.evaluate(prf, label, pad_length)
    }
}

struct Failing;

impl PadEvaluator for Failing {
    fn evaluate(&self, _: u16, _: B256, _: u16) -> anyhow::Result<PadOutputs> {
        anyhow::bail!("bad parameters")
    }
}

#[tokio::test]
async fn computes_signs_and_writes_a_row() {
    let (_instance, pool) = db().await;
    add_row(&pool, LANE, 0xab, 1, 0.0).await;
    let (signer, cancel, run) = start(&pool, config(), Arc::new(MockEvaluator));

    until(&pool, "SELECT signature IS NOT NULL FROM pads").await;

    let row: OutputRow = sqlx::query_as(
        "SELECT ciphertext128, ciphertext_digest, ciphertext128_digest, signer, signature,
                attestation_version FROM pads",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let label = B256::repeat_byte(0xab);
    let expected = MockEvaluator.evaluate(LANE, label, 64).unwrap();
    assert_eq!(row.0, expected.ct128);
    let attestation = PrfOutputAttestation {
        version: Version::try_from(u8::try_from(row.5).unwrap()).unwrap(),
        ciphertext_digest: B256::try_from(row.1.as_slice()).unwrap(),
        sns_ciphertext_digest: B256::try_from(row.2.as_slice()).unwrap(),
        signer: alloy::primitives::Address::try_from(row.3.as_slice()).unwrap(),
        signature: row.4,
    };
    attestation
        .verify(&PrfOutputRef::new(LANE, label, 64), signer.address())
        .unwrap();
    assert_eq!(
        attestation.ciphertext_digest,
        alloy::primitives::keccak256(&expected.ct64)
    );
    until(&pool, "SELECT heartbeat_at IS NOT NULL FROM prf_lanes").await;

    cancel.cancel();
    run.await.unwrap().unwrap();
}

#[tokio::test]
async fn computes_only_the_rows_of_its_instance() {
    let (_instance, pool) = db().await;
    add_row(&pool, LANE, 1, 1, 0.0).await;
    add_row(&pool, LANE + 1, 2, 1, 0.0).await;
    let (_signer, cancel, run) = start(&pool, config(), Arc::new(MockEvaluator));

    until(
        &pool,
        "SELECT COUNT(*) = 1 FROM pads WHERE signature IS NOT NULL",
    )
    .await;
    sleep(Duration::from_millis(300)).await;
    assert!(pending(&pool, LANE + 1, 2).await);

    cancel.cancel();
    run.await.unwrap().unwrap();
}

#[tokio::test]
async fn two_pods_compute_each_row_once() {
    let (_instance, pool) = db().await;
    for label in 0..10 {
        add_row(&pool, LANE, label, 1, 0.0).await;
    }
    let evaluations = Arc::new(Counting::default());
    let (_, cancel_a, run_a) = start(&pool, config(), evaluations.clone());
    let (_, cancel_b, run_b) = start(&pool, config(), evaluations.clone());

    until(&pool, "SELECT bool_and(signature IS NOT NULL) FROM pads").await;
    assert_eq!(evaluations.0.load(Ordering::SeqCst), 10);

    cancel_a.cancel();
    cancel_b.cancel();
    run_a.await.unwrap().unwrap();
    run_b.await.unwrap().unwrap();
}

#[tokio::test]
async fn takes_a_stale_claim_again_but_not_a_live_one() {
    let (_instance, pool) = db().await;
    add_row(&pool, LANE, 1, 1, 0.0).await;
    add_row(&pool, LANE, 2, 1, 0.0).await;
    // Row 1: claimed by a pod that stopped. Row 2: claimed by a pod that still works on it.
    sqlx::query(
        "UPDATE pads SET claimed_at = CASE WHEN label = $1 THEN NOW() - INTERVAL '1 hour'
                                          ELSE NOW() END",
    )
    .bind(B256::repeat_byte(1).as_slice())
    .execute(&pool)
    .await
    .unwrap();
    let (_signer, cancel, run) = start(&pool, config(), Arc::new(MockEvaluator));

    until(
        &pool,
        "SELECT COUNT(*) = 1 FROM pads WHERE signature IS NOT NULL",
    )
    .await;
    sleep(Duration::from_millis(300)).await;
    assert!(!pending(&pool, LANE, 1).await);
    assert!(pending(&pool, LANE, 2).await);

    cancel.cancel();
    run.await.unwrap().unwrap();
}

#[tokio::test]
async fn skips_expired_rows() {
    let (_instance, pool) = db().await;
    add_row(&pool, LANE, 1, 1, 3600.0).await;
    add_row(&pool, LANE, 2, 1, 0.0).await;
    let (_signer, cancel, run) = start(&pool, config(), Arc::new(MockEvaluator));

    until(
        &pool,
        "SELECT COUNT(*) = 1 FROM pads WHERE signature IS NOT NULL",
    )
    .await;
    sleep(Duration::from_millis(300)).await;
    assert!(pending(&pool, LANE, 1).await);

    cancel.cancel();
    run.await.unwrap().unwrap();
}

#[tokio::test]
async fn a_failed_evaluation_fails_the_row() {
    let (_instance, pool) = db().await;
    add_row(&pool, LANE, 1, 1, 0.0).await;
    let (_signer, cancel, run) = start(&pool, config(), Arc::new(Failing));

    until(
        &pool,
        "SELECT is_error AND error_message LIKE '%bad parameters%' FROM pads",
    )
    .await;

    cancel.cancel();
    run.await.unwrap().unwrap();
}

#[tokio::test]
async fn a_temporary_error_retries_the_row_after_a_backoff() {
    let (_instance, pool) = db().await;
    add_row(&pool, LANE, 1, 1, 0.0).await;
    let claim_timeout = Duration::from_secs(60);
    let claim = || rows::claim(&pool, LANE, Duration::from_secs(600), claim_timeout);
    let first = claim().await.unwrap().unwrap();

    for _ in 0..5 {
        rows::retry(&pool, &first, "signer down", claim_timeout)
            .await
            .unwrap();
    }
    let (attempts, failed, message): (i32, bool, String) =
        sqlx::query_as("SELECT attempts, is_error, error_message FROM pads")
            .fetch_one(&pool)
            .await
            .unwrap();
    // Never a final failure: the row stays pending, and keeps the last error.
    assert_eq!(
        (attempts, failed, message.as_str()),
        (5, false, "signer down")
    );
    // The backoff holds the row.
    assert!(claim().await.unwrap().is_none());

    // After the backoff, the row can be claimed again.
    sqlx::query("UPDATE pads SET claimed_at = claimed_at - INTERVAL '1 minute'")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(claim().await.unwrap(), Some(first));
}

#[tokio::test]
async fn the_connector_with_fewer_rows_in_progress_goes_first() {
    let (_instance, pool) = db().await;
    // Connector 1 asked first, for two rows. Connector 2 asked later, for one row.
    add_row(&pool, LANE, 1, 1, 30.0).await;
    add_row(&pool, LANE, 2, 1, 20.0).await;
    add_row(&pool, LANE, 3, 2, 10.0).await;
    let claim = || {
        rows::claim(
            &pool,
            LANE,
            Duration::from_secs(600),
            Duration::from_secs(60),
        )
    };

    let labels: Vec<u8> = [
        claim().await.unwrap().unwrap(),
        claim().await.unwrap().unwrap(),
        claim().await.unwrap().unwrap(),
    ]
    .iter()
    .map(|c| c.label[0])
    .collect();
    assert_eq!(labels, vec![1, 3, 2]);
    assert!(claim().await.unwrap().is_none());
}
