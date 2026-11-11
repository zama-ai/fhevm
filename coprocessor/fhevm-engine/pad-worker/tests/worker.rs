use alloy::{
    primitives::{keccak256, B256},
    signers::local::PrivateKeySigner,
};
use ciphertext_attestation::{
    consensus::Attestation, prf::Version, PrfOutputAttestation, PrfOutputRef,
};
use fhevm_engine_common::prf_reference::{reference_digest, reference_outputs};
use pad_worker::{
    evaluator::{MockEvaluator, PadEvaluator, PadOutputs},
    rows,
    upload::{reference_key, s3_key, Uploader},
    Config, PadWorker,
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
use test_harness::{
    instance::{setup_test_db, DBInstance, ImportMode},
    localstack::{create_localstack_s3_client, start_localstack, LocalstackContainer},
};
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
    // The operators agreed on the reference digest of the mock, so the lane starts ready.
    sqlx::query("INSERT INTO prf_lanes (prf_instance, reference_digest) VALUES ($1, $2)")
        .bind(i32::from(LANE))
        .bind(agreed_digest().as_slice())
        .execute(&pool)
        .await
        .unwrap();
    (instance, pool)
}

/// The reference digest of the mock evaluator for `LANE`.
fn agreed_digest() -> B256 {
    reference_digest(reference_outputs().into_iter().map(|(label, pad_length)| {
        let outputs = MockEvaluator.evaluate(LANE, label, pad_length).unwrap();
        (keccak256(&outputs.ct64), keccak256(&outputs.ct128))
    }))
}

fn config() -> Config {
    Config {
        prf_instance: LANE,
        key_id: vec![1],
        max_concurrent: NonZeroUsize::new(4).unwrap(),
        claim_timeout: Duration::from_secs(60),
        poll_interval: Duration::from_millis(100),
        pending_max_age: Duration::from_secs(600),
        output_max_age: None,
        gc_interval: Duration::from_secs(3600),
        ct64_max_age: Duration::from_secs(3600),
        start_check_interval: Duration::from_millis(100),
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

type Started = (
    PrivateKeySigner,
    CancellationToken,
    JoinHandle<anyhow::Result<()>>,
);

fn start(pool: &PgPool, config: Config, evaluator: Arc<dyn PadEvaluator>) -> Started {
    start_with(pool, config, evaluator, None)
}

fn start_with(
    pool: &PgPool,
    config: Config,
    evaluator: Arc<dyn PadEvaluator>,
    uploader: Option<Uploader>,
) -> Started {
    let signer = PrivateKeySigner::random();
    let cancel = CancellationToken::new();
    let mut worker = PadWorker::new(
        config,
        pool.clone(),
        evaluator,
        Arc::new(signer.clone()),
        cancel.clone(),
    );
    if let Some(uploader) = uploader {
        worker = worker.with_uploader(uploader);
    }
    (signer, cancel, tokio::spawn(worker.run()))
}

const BUCKET: &str = "pads";

/// LocalStack with no bucket, and an uploader that finds failed uploads every 100 ms.
async fn s3() -> (LocalstackContainer, aws_sdk_s3::Client, Uploader) {
    let localstack = start_localstack().await.expect("localstack");
    let client = create_localstack_s3_client(localstack.host_port).await;
    let uploader = Uploader::new(
        client.clone(),
        BUCKET.to_string(),
        NonZeroUsize::new(4).unwrap(),
        Duration::from_millis(100),
    );
    (localstack, client, uploader)
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

/// Fails all outputs except the reference outputs, so that the lane can start.
impl PadEvaluator for Failing {
    fn evaluate(&self, prf: u16, label: B256, pad_length: u16) -> anyhow::Result<PadOutputs> {
        if reference_outputs().contains(&(label, pad_length)) {
            return MockEvaluator.evaluate(prf, label, pad_length);
        }
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
    let ct64: Vec<u8> = sqlx::query_scalar("SELECT ciphertext64 FROM pads")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(ct64, expected.ct64);
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
    // Each pod also computes the reference outputs at start.
    let references = 2 * reference_outputs().len();
    assert_eq!(evaluations.0.load(Ordering::SeqCst), 10 + references);

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

#[tokio::test]
async fn uploads_the_output_with_its_attestation() {
    let (_instance, pool) = db().await;
    let (_localstack, client, uploader) = s3().await;
    client.create_bucket().bucket(BUCKET).send().await.unwrap();
    add_row(&pool, LANE, 0xab, 1, 0.0).await;
    let (signer, cancel, run) =
        start_with(&pool, config(), Arc::new(MockEvaluator), Some(uploader));

    until(&pool, "SELECT s3_verified_at IS NOT NULL FROM pads").await;

    let label = B256::repeat_byte(0xab);
    let key = format!("pads/{LANE}/{}/64/ct128", hex::encode(label));
    assert_eq!(
        s3_key(&rows::Claim {
            prf_instance: LANE,
            label,
            pad_length: 64
        }),
        key
    );
    let object = client
        .get_object()
        .bucket(BUCKET)
        .key(&key)
        .send()
        .await
        .unwrap();
    let attestation: PrfOutputAttestation = serde_json::from_str(
        &object.metadata().unwrap()[ciphertext_attestation::S3_METADATA_ATTESTATION_KEY],
    )
    .unwrap();
    attestation
        .verify(&PrfOutputRef::new(LANE, label, 64), signer.address())
        .unwrap();
    let body = object.body.collect().await.unwrap().into_bytes();
    assert_eq!(
        body.as_ref(),
        MockEvaluator.evaluate(LANE, label, 64).unwrap().ct128
    );

    cancel.cancel();
    run.await.unwrap().unwrap();
}

#[tokio::test]
async fn a_failed_upload_is_tried_again() {
    let (_instance, pool) = db().await;
    let (_localstack, client, uploader) = s3().await;
    add_row(&pool, LANE, 1, 1, 0.0).await;
    let config = Config {
        claim_timeout: Duration::from_secs(1),
        ..config()
    };
    let (_signer, cancel, run) = start_with(&pool, config, Arc::new(MockEvaluator), Some(uploader));

    // No bucket: the row is complete, but the upload fails.
    until(&pool, "SELECT signature IS NOT NULL FROM pads").await;
    sleep(Duration::from_millis(300)).await;
    let uploaded: bool = sqlx::query_scalar("SELECT s3_verified_at IS NOT NULL FROM pads")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!uploaded);

    client.create_bucket().bucket(BUCKET).send().await.unwrap();
    until(&pool, "SELECT s3_verified_at IS NOT NULL FROM pads").await;

    cancel.cancel();
    run.await.unwrap().unwrap();
}

#[tokio::test]
async fn writes_the_output_of_a_pending_row_only() {
    let (_instance, pool) = db().await;
    add_row(&pool, LANE, 1, 1, 0.0).await;
    let claim = rows::claim(
        &pool,
        LANE,
        Duration::from_secs(600),
        Duration::from_secs(60),
    )
    .await
    .unwrap()
    .unwrap();
    let attestation = PrfOutputAttestation {
        version: Version::V1,
        ciphertext_digest: B256::ZERO,
        sns_ciphertext_digest: B256::ZERO,
        signer: alloy::primitives::Address::ZERO,
        signature: vec![1; 65],
    };

    // The second write is from a pod whose claim went stale: the row keeps the first output.
    assert!(rows::write_output(&pool, &claim, &[9], &[1], &attestation)
        .await
        .unwrap());
    assert!(!rows::write_output(&pool, &claim, &[9], &[2], &attestation)
        .await
        .unwrap());
    let ct128: Vec<u8> = sqlx::query_scalar("SELECT ciphertext128 FROM pads")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(ct128, vec![1]);
}

/// Sets `state` on the row of `label`: `uploaded`, `not_uploaded`, `failed` or `claimed` (live).
async fn set_state(pool: &PgPool, prf: u16, label: u8, state: &str) {
    let update = match state {
        "uploaded" | "not_uploaded" => {
            "UPDATE pads SET ciphertext128 = '\\x01', ciphertext_digest = $3,
                             ciphertext128_digest = $3, signer = $4, signature = $5,
                             attestation_version = 1,
                             s3_verified_at = CASE WHEN $6 THEN NOW() END
              WHERE prf_instance = $1 AND label = $2"
        }
        "failed" => "UPDATE pads SET is_error = TRUE WHERE prf_instance = $1 AND label = $2",
        "claimed" => "UPDATE pads SET claimed_at = NOW() WHERE prf_instance = $1 AND label = $2",
        _ => panic!("unknown state {state}"),
    };
    let mut query = sqlx::query(update)
        .bind(i32::from(prf))
        .bind(B256::repeat_byte(label).to_vec());
    if state.ends_with("uploaded") {
        query = query
            .bind(vec![0u8; 32])
            .bind(vec![0u8; 20])
            .bind(vec![1u8; 65])
            .bind(state == "uploaded");
    }
    query.execute(pool).await.unwrap();
}

/// The labels of the rows of `prf`, sorted.
async fn labels(pool: &PgPool, prf: u16) -> Vec<u8> {
    let labels: Vec<Vec<u8>> =
        sqlx::query_scalar("SELECT label FROM pads WHERE prf_instance = $1 ORDER BY label")
            .bind(i32::from(prf))
            .fetch_all(pool)
            .await
            .unwrap();
    labels.iter().map(|l| l[0]).collect()
}

#[tokio::test]
async fn deletes_old_rows_of_all_instances_that_are_uploaded_or_failed() {
    let (_instance, pool) = db().await;
    for (label, age, state) in [
        (1, 3600.0, "uploaded"),
        (2, 3600.0, "failed"),
        (3, 3600.0, "not_uploaded"),
        (4, 60.0, "uploaded"),
    ] {
        add_row(&pool, LANE, label, 1, age).await;
        set_state(&pool, LANE, label, state).await;
    }
    add_row(&pool, LANE + 1, 5, 1, 3600.0).await;
    set_state(&pool, LANE + 1, 5, "uploaded").await;

    let deleted = rows::delete_old_outputs(&pool, Duration::from_secs(1800))
        .await
        .unwrap();

    assert_eq!(deleted, 3);
    assert_eq!(labels(&pool, LANE).await, vec![3, 4]);
    assert!(labels(&pool, LANE + 1).await.is_empty());
}

#[tokio::test]
async fn deletes_expired_pending_rows_of_all_instances_without_a_live_claim() {
    let (_instance, pool) = db().await;
    add_row(&pool, LANE, 1, 1, 3600.0).await;
    add_row(&pool, LANE, 2, 1, 3600.0).await;
    set_state(&pool, LANE, 2, "claimed").await;
    add_row(&pool, LANE, 3, 1, 3600.0).await;
    set_state(&pool, LANE, 3, "not_uploaded").await;
    // Another instance: one expired row, one new row.
    add_row(&pool, LANE + 1, 4, 1, 3600.0).await;
    add_row(&pool, LANE + 1, 5, 1, 60.0).await;

    let deleted =
        rows::delete_expired_pending(&pool, Duration::from_secs(600), Duration::from_secs(60))
            .await
            .unwrap();

    assert_eq!(deleted, 2);
    assert_eq!(labels(&pool, LANE).await, vec![2, 3]);
    assert_eq!(labels(&pool, LANE + 1).await, vec![5]);
}

#[tokio::test]
async fn gives_the_age_of_the_oldest_rows_of_each_instance() {
    let (_instance, pool) = db().await;
    add_row(&pool, LANE, 1, 1, 100.0).await;
    add_row(&pool, LANE, 2, 1, 200.0).await;
    set_state(&pool, LANE, 2, "not_uploaded").await;
    add_row(&pool, LANE + 1, 3, 1, 300.0).await;
    set_state(&pool, LANE + 1, 3, "uploaded").await;

    let mut oldest = rows::oldest_rows(&pool).await.unwrap();
    oldest.sort_by_key(|o| o.prf_instance);

    let near = |age: Option<f64>, expected: f64| age.is_some_and(|a| (a - expected).abs() < 5.0);
    assert_eq!(oldest.len(), 2);
    assert!(near(oldest[0].pending, 100.0));
    assert!(near(oldest[0].not_uploaded, 200.0));
    assert_eq!((oldest[1].pending, oldest[1].not_uploaded), (None, None));
}

#[tokio::test]
async fn the_gc_runs_in_the_worker() {
    let (_instance, pool) = db().await;
    add_row(&pool, LANE + 1, 1, 1, 3600.0).await;
    let config = Config {
        gc_interval: Duration::from_millis(100),
        ..config()
    };
    let (_signer, cancel, run) = start(&pool, config, Arc::new(MockEvaluator));

    until(&pool, "SELECT COUNT(*) = 0 FROM pads").await;

    cancel.cancel();
    run.await.unwrap().unwrap();
}

async fn object_exists(client: &aws_sdk_s3::Client, key: &str) -> bool {
    client
        .head_object()
        .bucket(BUCKET)
        .key(key)
        .send()
        .await
        .is_ok()
}

#[tokio::test]
async fn waits_for_the_agreed_reference_digest() {
    let (_instance, pool) = db().await;
    let (_localstack, client, uploader) = s3().await;
    client.create_bucket().bucket(BUCKET).send().await.unwrap();
    sqlx::query("UPDATE prf_lanes SET reference_digest = NULL")
        .execute(&pool)
        .await
        .unwrap();
    add_row(&pool, LANE, 1, 1, 0.0).await;
    let (_signer, cancel, run) =
        start_with(&pool, config(), Arc::new(MockEvaluator), Some(uploader));

    // A new instance: the lane uploads the reference objects, and does not claim.
    let keys: Vec<String> = reference_outputs()
        .into_iter()
        .map(|(label, pad_length)| {
            reference_key(&rows::Claim {
                prf_instance: LANE,
                label,
                pad_length,
            })
        })
        .collect();
    let deadline = tokio::time::Instant::now() + DEADLINE;
    for key in &keys {
        while !object_exists(&client, key).await {
            assert!(tokio::time::Instant::now() < deadline, "no object {key}");
            sleep(Duration::from_millis(50)).await;
        }
    }
    // The operators do not agree: the lane still waits.
    sqlx::query("UPDATE prf_lanes SET reference_mismatch_at = NOW()")
        .execute(&pool)
        .await
        .unwrap();
    sleep(Duration::from_millis(300)).await;
    assert!(!run.is_finished());
    assert!(pending(&pool, LANE, 1).await);
    let ready: bool = sqlx::query_scalar("SELECT ready_at IS NOT NULL FROM prf_lanes")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!ready);

    // The operators agree on the digest of the lane: it is ready, and claims.
    sqlx::query("UPDATE prf_lanes SET reference_digest = $1, reference_mismatch_at = NULL")
        .bind(agreed_digest().as_slice())
        .execute(&pool)
        .await
        .unwrap();
    until(&pool, "SELECT ready_at IS NOT NULL FROM prf_lanes").await;
    until(&pool, "SELECT signature IS NOT NULL FROM pads").await;

    cancel.cancel();
    run.await.unwrap().unwrap();
}

#[tokio::test]
async fn exits_if_the_agreed_reference_digest_is_another() {
    let (_instance, pool) = db().await;
    sqlx::query("UPDATE prf_lanes SET reference_digest = $1")
        .bind(B256::repeat_byte(9).as_slice())
        .execute(&pool)
        .await
        .unwrap();
    add_row(&pool, LANE, 1, 1, 0.0).await;
    let (_signer, _cancel, run) = start(&pool, config(), Arc::new(MockEvaluator));

    let error = run.await.unwrap().unwrap_err();
    assert!(error.to_string().contains("not the agreed"), "{error}");
    assert!(pending(&pool, LANE, 1).await);
}

#[tokio::test]
async fn clears_the_old_ct64_and_keeps_the_digest() {
    let (_instance, pool) = db().await;
    for (label, age) in [(1, 3600.0), (2, 60.0)] {
        add_row(&pool, LANE, label, 1, age).await;
        set_state(&pool, LANE, label, "uploaded").await;
    }
    sqlx::query("UPDATE pads SET ciphertext64 = '\\x0102'")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(rows::stored_ct64_bytes(&pool).await.unwrap(), 4);

    let cleared = rows::clear_old_ct64(&pool, Duration::from_secs(1800))
        .await
        .unwrap();

    assert_eq!(cleared, 1);
    let rows: Vec<(Vec<u8>, bool, bool)> = sqlx::query_as(
        "SELECT label, ciphertext64 IS NULL, ciphertext_digest IS NOT NULL FROM pads ORDER BY label",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    let states: Vec<(u8, bool, bool)> = rows.into_iter().map(|(l, c, d)| (l[0], c, d)).collect();
    assert_eq!(states, vec![(1, true, true), (2, false, true)]);
    assert_eq!(rows::stored_ct64_bytes(&pool).await.unwrap(), 2);
}
