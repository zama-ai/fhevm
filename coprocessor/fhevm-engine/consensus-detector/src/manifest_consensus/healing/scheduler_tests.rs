use super::*;
use crate::manifest_consensus::ManifestWorkGate;
use alloy_primitives::{keccak256, Address, B256, U256};
use block_manifest::LEGACY_CONSENSUS_EPOCH;
use fhevm_engine_common::gcs_activation::WORK_AVAILABLE_CHANNEL;
use serial_test::serial;
use sqlx::postgres::PgListener;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};

fn bytes(n: u8) -> Vec<u8> {
    vec![n; 32]
}

async fn setup() -> (DBInstance, PgPool) {
    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let pool = PgPool::connect(instance.db_url()).await.unwrap();
    (instance, pool)
}

async fn insert_healable(pool: &PgPool, handle: u8, bucket_url: &str, body: &[u8]) -> i64 {
    insert_healable_from(pool, handle, Some(bucket_url), body, true).await
}

async fn insert_sibling(
    pool: &PgPool,
    handle: u8,
    block: u8,
    bucket_url: &str,
    body: &[u8],
) -> i64 {
    let sources = format!(r#"[{{"s3_bucket_url":"{bucket_url}"}}]"#);
    sqlx::query_scalar(
        "INSERT INTO drifted_handle (
            consensus_epoch, coprocessor_context_id, host_chain_id,
            block_number, block_hash, handle, detection_kind, reason,
            local_present, quorum_present, quorum_ct64_digest, peer_sources,
            is_contained
         ) SELECT consensus_epoch, $1, 1, $2, $3, $4, 'inferred', 'ct64_mismatch',
                  TRUE, FALSE, $5, $6::jsonb, TRUE
             FROM blue_green_consensus_epoch
         RETURNING id",
    )
    .bind(bytes(1))
    .bind(i64::from(block))
    .bind(bytes(block))
    .bind(bytes(handle))
    .bind(keccak256(body).as_slice().to_vec())
    .bind(sources)
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn insert_healable_from(
    pool: &PgPool,
    handle: u8,
    bucket_url: Option<&str>,
    body: &[u8],
    pin: bool,
) -> i64 {
    let sources = bucket_url
        .map(|url| format!(r#"[{{"s3_bucket_url":"{url}"}}]"#))
        .unwrap_or_else(|| "[]".into());
    let id: i64 = sqlx::query_scalar(
        "INSERT INTO drifted_handle (
            consensus_epoch, coprocessor_context_id, host_chain_id,
            block_number, block_hash, handle, detection_kind, reason,
            local_present, quorum_present, quorum_ct64_digest, peer_sources,
            is_contained
         ) SELECT consensus_epoch, $1, 1, $2, $3, $3, 'inferred', 'ct64_mismatch',
                  TRUE, FALSE, $4, $5::jsonb, TRUE
             FROM blue_green_consensus_epoch
         RETURNING id",
    )
    .bind(bytes(1))
    .bind(i64::from(handle))
    .bind(bytes(handle))
    .bind(pin.then(|| keccak256(body).as_slice().to_vec()))
    .bind(sources)
    .fetch_one(pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
         VALUES ($1, $2, 0, 0)
         ON CONFLICT (handle, ciphertext_version) DO NOTHING",
    )
    .bind(bytes(handle))
    .bind(vec![0xffu8; 4])
    .execute(pool)
    .await
    .unwrap();
    id
}

async fn stored_ct64(pool: &PgPool, handle: u8) -> Vec<u8> {
    sqlx::query_scalar(
        "SELECT ciphertext FROM ciphertexts WHERE handle = $1 AND ciphertext_version = 0",
    )
    .bind(bytes(handle))
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn pass(pool: &PgPool, source: &FakeCt64) {
    run_healing_pass(
        pool,
        source,
        &ManifestWorkGate::always_enabled(),
        DEFAULT_BATCH_SIZE,
        DEFAULT_CONTAINMENT_TIMEOUT,
    )
    .await
    .unwrap();
}

async fn healed(pool: &PgPool, id: i64) -> (bool, bool) {
    let row: (bool, bool) = sqlx::query_as(
        "SELECT can_be_healed, healed_at IS NOT NULL FROM drifted_handle WHERE id = $1",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .unwrap();
    row
}

struct FakeObject {
    digest: B256,
    body: Option<Vec<u8>>,
}

type FakeStore = HashMap<(String, Vec<u8>), FakeObject>;

#[derive(Clone, Default)]
struct FakeCt64 {
    objects: Arc<Mutex<FakeStore>>,
    gets: Arc<AtomicUsize>,
    heads: Arc<AtomicUsize>,
    inflight: Arc<AtomicUsize>,
    max_inflight: Arc<AtomicUsize>,
}

impl FakeCt64 {
    fn put(&self, bucket_url: &str, handle: u8, body: Vec<u8>) {
        let digest = keccak256(&body);
        self.objects.lock().unwrap().insert(
            (bucket_url.to_owned(), bytes(handle)),
            FakeObject {
                digest,
                body: Some(body),
            },
        );
    }

    fn attest(&self, bucket_url: &str, handle: u8, digest: B256) {
        self.objects.lock().unwrap().insert(
            (bucket_url.to_owned(), bytes(handle)),
            FakeObject { digest, body: None },
        );
    }
}

impl Ct64Source for FakeCt64 {
    async fn get_ct64(
        &self,
        bucket_url: &str,
        handle: &[u8],
        _coprocessor_context_id: U256,
    ) -> Result<Vec<u8>, ExecutionError> {
        self.gets.fetch_add(1, Ordering::SeqCst);
        let n = self.inflight.fetch_add(1, Ordering::SeqCst) + 1;
        self.max_inflight.fetch_max(n, Ordering::SeqCst);
        tokio::time::sleep(Duration::from_millis(50)).await;
        self.inflight.fetch_sub(1, Ordering::SeqCst);
        self.objects
            .lock()
            .unwrap()
            .get(&(bucket_url.to_owned(), handle.to_vec()))
            .and_then(|object| object.body.clone())
            .ok_or_else(|| ExecutionError::S3ObjectNotFound(bucket_url.to_owned()))
    }

    async fn head_ct64_digest(
        &self,
        bucket_url: &str,
        handle: &[u8],
        _coprocessor_context_id: U256,
        _expected_signer: Option<Address>,
    ) -> Result<B256, ExecutionError> {
        self.heads.fetch_add(1, Ordering::SeqCst);
        self.objects
            .lock()
            .unwrap()
            .get(&(bucket_url.to_owned(), handle.to_vec()))
            .map(|object| object.digest)
            .ok_or_else(|| ExecutionError::S3ObjectNotFound(bucket_url.to_owned()))
    }
}

async fn seed_registry(pool: &PgPool, buckets: &[&str], threshold: i64) {
    for (index, bucket) in buckets.iter().enumerate() {
        sqlx::query(
            r#"
            INSERT INTO gateway_config_coprocessors (
                tx_sender_address,
                signer_address,
                s3_bucket_url,
                coprocessor_threshold,
                gateway_chain_id,
                gateway_config_address,
                snapshot_block_number,
                snapshot_block_hash
            )
            VALUES ($1, $2, $3, $4, 54321, $5, 100, $6)
            "#,
        )
        .bind(Address::repeat_byte(0x30 + index as u8).as_slice())
        .bind(Address::repeat_byte(0x40 + index as u8).as_slice())
        .bind(*bucket)
        .bind(threshold)
        .bind(Address::repeat_byte(0x10).as_slice())
        .bind(B256::repeat_byte(0x20).as_slice())
        .execute(pool)
        .await
        .unwrap();
    }
}

#[tokio::test]
#[serial(db)]
async fn pass_downloads_matching_ct64_from_peer_bucket() {
    let (_db, pool) = setup().await;
    let body = vec![7u8; 8];
    let id = insert_healable(&pool, 1, "s3://peer-a", &body).await;
    let source = FakeCt64::default();
    source.put("s3://peer-a", 1, body.clone());
    pass(&pool, &source).await;
    assert_eq!(source.gets.load(Ordering::SeqCst), 1);
    assert_eq!(stored_ct64(&pool, 1).await, body);
    assert_eq!(healed(&pool, id).await, (false, true));
}

#[tokio::test]
#[serial(db)]
async fn pass_downloads_due_handles_concurrently() {
    let (_db, pool) = setup().await;
    let body_a = vec![1u8; 8];
    let body_b = vec![2u8; 8];
    let id_a = insert_healable(&pool, 1, "s3://peer-a", &body_a).await;
    let id_b = insert_healable(&pool, 2, "s3://peer-a", &body_b).await;
    let source = FakeCt64::default();
    source.put("s3://peer-a", 1, body_a.clone());
    source.put("s3://peer-a", 2, body_b.clone());
    pass(&pool, &source).await;
    assert_eq!(source.gets.load(Ordering::SeqCst), 2);
    assert!(source.max_inflight.load(Ordering::SeqCst) >= 2);
    assert_eq!(stored_ct64(&pool, 1).await, body_a);
    assert_eq!(stored_ct64(&pool, 2).await, body_b);
    assert_eq!(healed(&pool, id_a).await, (false, true));
    assert_eq!(healed(&pool, id_b).await, (false, true));
}

#[tokio::test]
#[serial(db)]
async fn notify_wakes_the_worker_before_the_poll() {
    let (_db, pool) = setup().await;
    let body = vec![9u8; 8];
    let source = FakeCt64::default();
    source.put("s3://peer-a", 3, body.clone());
    let token = CancellationToken::new();
    let worker = tokio::spawn(run_healing_worker(
        pool.clone(),
        token.clone(),
        source.clone(),
        ManifestWorkGate::always_enabled(),
        DEFAULT_BATCH_SIZE,
        DEFAULT_POLL_INTERVAL,
        DEFAULT_CONTAINMENT_TIMEOUT,
    ));
    tokio::time::sleep(Duration::from_millis(200)).await;
    let id = insert_healable(&pool, 3, "s3://peer-a", &body).await;
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            if healed(&pool, id).await == (false, true) {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("healing worker should run on NOTIFY, not wait for the 30s poll");
    assert_eq!(stored_ct64(&pool, 3).await, body);
    token.cancel();
    worker.await.unwrap().unwrap();
}

#[tokio::test]
#[serial(db)]
async fn digest_mismatch_does_not_install() {
    let (_db, pool) = setup().await;
    let target = vec![7u8; 8];
    let id = insert_healable(&pool, 4, "s3://peer-a", &target).await;
    let source = FakeCt64::default();
    source.put("s3://peer-a", 4, vec![8u8; 8]);
    pass(&pool, &source).await;
    assert_eq!(healed(&pool, id).await, (true, false));
    assert_eq!(stored_ct64(&pool, 4).await, vec![0xffu8; 4]);
}

#[tokio::test]
#[serial(db)]
async fn mismatch_falls_back_to_attestation_quorum_peer() {
    let (_db, pool) = setup().await;
    seed_registry(&pool, &["s3://peer-a", "s3://peer-b", "s3://peer-c"], 2).await;
    let body = vec![7u8; 8];
    let id = insert_healable(&pool, 5, "s3://peer-a", &body).await;
    let source = FakeCt64::default();
    source.put("s3://peer-a", 5, vec![8u8; 8]);
    source.put("s3://peer-b", 5, body.clone());
    source.put("s3://peer-c", 5, body.clone());
    pass(&pool, &source).await;
    assert_eq!(stored_ct64(&pool, 5).await, body);
    assert_eq!(healed(&pool, id).await, (false, true));
    assert!(source.heads.load(Ordering::SeqCst) >= 2);
}

#[tokio::test]
#[serial(db)]
async fn inferred_without_sources_uses_attestation_quorum() {
    let (_db, pool) = setup().await;
    seed_registry(&pool, &["s3://peer-a", "s3://peer-b"], 2).await;
    let body = vec![3u8; 8];
    let id = insert_healable_from(&pool, 6, None, &body, false).await;
    let source = FakeCt64::default();
    source.put("s3://peer-a", 6, body.clone());
    source.put("s3://peer-b", 6, body.clone());
    pass(&pool, &source).await;
    assert_eq!(stored_ct64(&pool, 6).await, body);
    assert_eq!(healed(&pool, id).await, (false, true));
    let pinned: Vec<u8> =
        sqlx::query_scalar("SELECT quorum_ct64_digest FROM drifted_handle WHERE id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(pinned, keccak256(&body).as_slice());
    let sources: serde_json::Value =
        sqlx::query_scalar("SELECT peer_sources FROM drifted_handle WHERE id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(sources.as_array().map(Vec::len), Some(2));
    let evidence: serde_json::Value =
        sqlx::query_scalar("SELECT target_evidence FROM drifted_handle WHERE id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(evidence["source"], "attestation");
}

#[tokio::test]
#[serial(db)]
async fn attestation_quorum_on_another_digest_is_quorum_changed() {
    let (_db, pool) = setup().await;
    seed_registry(&pool, &["s3://peer-a", "s3://peer-b"], 2).await;
    let target = vec![7u8; 8];
    let other = vec![9u8; 8];
    let id = insert_healable_from(&pool, 7, None, &target, true).await;
    let source = FakeCt64::default();
    source.put("s3://peer-a", 7, other.clone());
    source.put("s3://peer-b", 7, other);
    let before = metrics::QUORUM_CHANGED
        .with_label_values(&[LEGACY_CONSENSUS_EPOCH])
        .get();
    pass(&pool, &source).await;
    assert_eq!(healed(&pool, id).await, (true, false));
    assert!(
        metrics::QUORUM_CHANGED
            .with_label_values(&[LEGACY_CONSENSUS_EPOCH])
            .get()
            > before
    );
}

#[tokio::test]
#[serial(db)]
async fn matching_attestations_without_bytes_are_a_bad_target() {
    let (_db, pool) = setup().await;
    seed_registry(&pool, &["s3://peer-a", "s3://peer-b"], 2).await;
    let body = vec![7u8; 8];
    let digest = keccak256(&body);
    let id = insert_healable_from(&pool, 8, None, &body, false).await;
    let source = FakeCt64::default();
    source.attest("s3://peer-a", 8, digest);
    source.attest("s3://peer-b", 8, digest);
    let before = metrics::BAD_TARGET_DIGEST
        .with_label_values(&[LEGACY_CONSENSUS_EPOCH])
        .get();
    pass(&pool, &source).await;
    assert_eq!(healed(&pool, id).await, (true, false));
    assert!(
        metrics::BAD_TARGET_DIGEST
            .with_label_values(&[LEGACY_CONSENSUS_EPOCH])
            .get()
            > before
    );
}

#[tokio::test]
#[serial(db)]
async fn pass_heals_at_most_batch_size_handles() {
    let (_db, pool) = setup().await;
    let source = FakeCt64::default();
    let mut ids = Vec::new();
    for handle in 1u8..=3 {
        let body = vec![handle; 8];
        source.put("s3://peer-a", handle, body.clone());
        ids.push(insert_healable(&pool, handle, "s3://peer-a", &body).await);
    }
    run_healing_pass(
        &pool,
        &source,
        &ManifestWorkGate::always_enabled(),
        2,
        DEFAULT_CONTAINMENT_TIMEOUT,
    )
    .await
    .unwrap();
    let mut healed_count = 0;
    for id in ids {
        if healed(&pool, id).await.1 {
            healed_count += 1;
        }
    }
    assert_eq!(healed_count, 2);
    assert_eq!(source.gets.load(Ordering::SeqCst), 2);
}

#[tokio::test]
#[serial(db)]
async fn same_target_siblings_heal_once() {
    let (_db, pool) = setup().await;
    let body = vec![4u8; 8];
    let source = FakeCt64::default();
    source.put("s3://peer-a", 9, body.clone());
    let first = insert_sibling(&pool, 9, 9, "s3://peer-a", &body).await;
    let second = insert_sibling(&pool, 9, 19, "s3://peer-a", &body).await;
    sqlx::query(
        "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
         VALUES ($1, $2, 0, 0)
         ON CONFLICT (handle, ciphertext_version) DO NOTHING",
    )
    .bind(bytes(9))
    .bind(vec![0xffu8; 4])
    .execute(&pool)
    .await
    .unwrap();

    pass(&pool, &source).await;

    assert_eq!(healed(&pool, first).await, (false, true));
    assert_eq!(healed(&pool, second).await, (false, true));
    assert_eq!(stored_ct64(&pool, 9).await, body);
    assert_eq!(source.gets.load(Ordering::SeqCst), 1);
}

#[tokio::test]
#[serial(db)]
async fn disagreeing_targets_are_left_unhealed() {
    let (_db, pool) = setup().await;
    let body = vec![4u8; 8];
    let other = vec![5u8; 8];
    let source = FakeCt64::default();
    source.put("s3://peer-a", 10, body.clone());
    let first = insert_sibling(&pool, 10, 10, "s3://peer-a", &body).await;
    let second = insert_sibling(&pool, 10, 20, "s3://peer-a", &other).await;

    pass(&pool, &source).await;

    assert_eq!(healed(&pool, first).await, (true, false));
    assert_eq!(healed(&pool, second).await, (true, false));
    assert_eq!(source.gets.load(Ordering::SeqCst), 0);
}

async fn listen(pool: &PgPool, channel: &str) -> PgListener {
    let mut listener = PgListener::connect_with(pool).await.unwrap();
    listener.listen(channel).await.unwrap();
    listener
}

#[tokio::test]
#[serial(db)]
async fn pass_notifies_tfhe_after_a_successful_batch() {
    let (_db, pool) = setup().await;
    let body = vec![7u8; 8];
    let source = FakeCt64::default();
    source.put("s3://peer-a", 1, body.clone());
    insert_healable(&pool, 1, "s3://peer-a", &body).await;
    let mut listener = listen(&pool, WORK_AVAILABLE_CHANNEL).await;
    pass(&pool, &source).await;
    tokio::time::timeout(Duration::from_secs(2), listener.recv())
        .await
        .expect("idle TFHE should wake after a healing batch that installed ct64")
        .unwrap();
}

#[tokio::test]
#[serial(db)]
async fn pass_does_not_notify_tfhe_when_nothing_installed() {
    let (_db, pool) = setup().await;
    let target = vec![7u8; 8];
    insert_healable(&pool, 4, "s3://peer-a", &target).await;
    let source = FakeCt64::default();
    source.put("s3://peer-a", 4, vec![8u8; 8]);
    let mut listener = listen(&pool, WORK_AVAILABLE_CHANNEL).await;
    pass(&pool, &source).await;
    assert!(
        tokio::time::timeout(Duration::from_millis(300), listener.recv())
            .await
            .is_err(),
        "a batch that only retries must not wake TFHE"
    );
}

async fn set_contained(pool: &PgPool, id: i64, contained: bool) {
    sqlx::query("UPDATE drifted_handle SET is_contained = $2 WHERE id = $1")
        .bind(id)
        .bind(contained)
        .execute(pool)
        .await
        .unwrap();
}

async fn insert_computation(pool: &PgPool, handle: u8, is_error: bool) {
    sqlx::query(
        "INSERT INTO computations (
            output_handle, dependencies, fhe_operation, is_scalar, transaction_id,
            is_completed, is_error, error_message, error_retry_count
         ) VALUES ($1, '{}', 0, FALSE, $2, FALSE, $3, $4, $5)",
    )
    .bind(bytes(handle))
    .bind(bytes(0xee))
    .bind(is_error)
    .bind(is_error.then_some("local computation failed"))
    .bind(if is_error { 3i16 } else { 0 })
    .execute(pool)
    .await
    .unwrap();
}

async fn computation_state(pool: &PgPool, handle: u8) -> (bool, bool, Option<String>, i16) {
    sqlx::query_as(
        "SELECT is_completed, is_error, error_message, error_retry_count
           FROM computations WHERE output_handle = $1",
    )
    .bind(bytes(handle))
    .fetch_one(pool)
    .await
    .unwrap()
}

#[tokio::test]
#[serial(db)]
async fn uncontained_ct64_mismatch_waits_for_containment() {
    let (_db, pool) = setup().await;
    let body = vec![5u8; 8];
    let id = insert_healable(&pool, 20, "s3://peer-a", &body).await;
    set_contained(&pool, id, false).await;
    let source = FakeCt64::default();
    source.put("s3://peer-a", 20, body.clone());

    pass(&pool, &source).await;
    assert_eq!(source.gets.load(Ordering::SeqCst), 0);
    assert_eq!(healed(&pool, id).await, (true, false));
    assert_eq!(stored_ct64(&pool, 20).await, vec![0xffu8; 4]);

    set_contained(&pool, id, true).await;
    pass(&pool, &source).await;
    assert_eq!(healed(&pool, id).await, (false, true));
    assert_eq!(stored_ct64(&pool, 20).await, body);
}

#[tokio::test]
#[serial(db)]
async fn uncontained_sibling_is_not_marked_healed() {
    let (_db, pool) = setup().await;
    let body = vec![6u8; 8];
    let first = insert_sibling(&pool, 21, 1, "s3://peer-a", &body).await;
    let second = insert_sibling(&pool, 21, 2, "s3://peer-a", &body).await;
    set_contained(&pool, second, false).await;
    let source = FakeCt64::default();
    source.put("s3://peer-a", 21, body.clone());

    pass(&pool, &source).await;
    assert_eq!(healed(&pool, first).await, (false, true));
    assert_eq!(healed(&pool, second).await, (true, false));
}

#[tokio::test]
#[serial(db)]
async fn other_reasons_heal_without_containment_and_complete_the_computation() {
    for (handle, reason, errored) in [
        (30u8, "missing_here", false),
        (31, "error_here", true),
        (32, "uncomputed_here", false),
    ] {
        let (_db, pool) = setup().await;
        let body = vec![handle; 8];
        let id = crate::manifest_consensus::containment::propagation_tests::direct_root(
            &pool, handle, reason,
        )
        .await;
        sqlx::query(
            "UPDATE drifted_handle
                SET quorum_ct64_digest = $2,
                    peer_sources = '[{\"s3_bucket_url\":\"s3://peer-a\"}]'::jsonb,
                    is_contained = FALSE
              WHERE id = $1",
        )
        .bind(id)
        .bind(keccak256(&body).as_slice())
        .execute(&pool)
        .await
        .unwrap();
        insert_computation(&pool, handle, errored).await;
        let source = FakeCt64::default();
        source.put("s3://peer-a", handle, body.clone());

        pass(&pool, &source).await;
        assert_eq!(healed(&pool, id).await, (false, true), "{reason}");
        assert_eq!(stored_ct64(&pool, handle).await, body, "{reason}");
        assert_eq!(
            computation_state(&pool, handle).await,
            (true, false, None, 0),
            "{reason}: installed bytes complete the computation"
        );
    }
}

#[tokio::test]
#[serial(db)]
async fn containment_wakes_healing() {
    let (_db, pool) = setup().await;
    let body = vec![7u8; 8];
    let id = insert_healable(&pool, 22, "s3://peer-a", &body).await;
    set_contained(&pool, id, false).await;
    let mut listener = listen(&pool, "event_healing_work").await;
    set_contained(&pool, id, true).await;
    tokio::time::timeout(Duration::from_secs(2), listener.recv())
        .await
        .expect("marking a finding contained must wake healing")
        .unwrap();
}

#[tokio::test]
#[serial(db)]
async fn uncontained_ct64_mismatch_heals_after_the_containment_timeout() {
    let (_db, pool) = setup().await;
    let body = vec![8u8; 8];
    let id = insert_healable(&pool, 23, "s3://peer-a", &body).await;
    set_contained(&pool, id, false).await;
    sqlx::query(
        "UPDATE drifted_handle SET detected_at = NOW() - $2 * INTERVAL '1 second' WHERE id = $1",
    )
    .bind(id)
    .bind(i64::try_from(DEFAULT_CONTAINMENT_TIMEOUT.as_secs()).unwrap() + 1)
    .execute(&pool)
    .await
    .unwrap();
    let source = FakeCt64::default();
    source.put("s3://peer-a", 23, body.clone());
    let before = metrics::HEALED_UNCONTAINED
        .with_label_values(&[LEGACY_CONSENSUS_EPOCH])
        .get();

    pass(&pool, &source).await;
    assert_eq!(healed(&pool, id).await, (false, true));
    assert_eq!(stored_ct64(&pool, 23).await, body);
    assert_eq!(
        metrics::HEALED_UNCONTAINED
            .with_label_values(&[LEGACY_CONSENSUS_EPOCH])
            .get(),
        before + 1
    );
}
