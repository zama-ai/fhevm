use super::*;
use crate::manifest_consensus::ManifestWorkGate;
use alloy_primitives::{keccak256, U256};
use serial_test::serial;
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
    let sources = format!(r#"[{{"s3_bucket_url":"{bucket_url}"}}]"#);
    let id: i64 = sqlx::query_scalar(
        "INSERT INTO drifted_handle (
            consensus_epoch, coprocessor_context_id, host_chain_id,
            block_number, block_hash, handle, detection_kind, reason,
            local_present, observed_present, target_ct64_digest, peer_sources
         ) SELECT consensus_epoch, $1, 1, $2, $3, $3, 'inferred', 'ct64_mismatch',
                  TRUE, FALSE, $4, $5::jsonb
             FROM blue_green_consensus_epoch
         RETURNING id",
    )
    .bind(bytes(1))
    .bind(i64::from(handle))
    .bind(bytes(handle))
    .bind(keccak256(body).as_slice())
    .bind(sources)
    .fetch_one(pool)
    .await
    .unwrap();
    id
}

type FakeStore = HashMap<(String, Vec<u8>), Vec<u8>>;

#[derive(Clone, Default)]
struct FakeCt64 {
    objects: Arc<Mutex<FakeStore>>,
    gets: Arc<AtomicUsize>,
    inflight: Arc<AtomicUsize>,
    max_inflight: Arc<AtomicUsize>,
}

impl FakeCt64 {
    fn put(&self, bucket_url: &str, handle: u8, body: Vec<u8>) {
        self.objects
            .lock()
            .unwrap()
            .insert((bucket_url.to_owned(), bytes(handle)), body);
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
            .cloned()
            .ok_or_else(|| ExecutionError::S3ObjectNotFound(bucket_url.to_owned()))
    }
}

#[tokio::test]
#[serial(db)]
async fn pass_downloads_matching_ct64_from_peer_bucket() {
    let (_db, pool) = setup().await;
    let body = vec![7u8; 8];
    insert_healable(&pool, 1, "s3://peer-a", &body).await;
    let source = FakeCt64::default();
    source.put("s3://peer-a", 1, body);
    run_healing_pass(&pool, &source, &ManifestWorkGate::always_enabled())
        .await
        .unwrap();
    assert_eq!(source.gets.load(Ordering::SeqCst), 1);
}

#[tokio::test]
#[serial(db)]
async fn pass_downloads_due_handles_concurrently() {
    let (_db, pool) = setup().await;
    let body_a = vec![1u8; 8];
    let body_b = vec![2u8; 8];
    insert_healable(&pool, 1, "s3://peer-a", &body_a).await;
    insert_healable(&pool, 2, "s3://peer-a", &body_b).await;
    let source = FakeCt64::default();
    source.put("s3://peer-a", 1, body_a);
    source.put("s3://peer-a", 2, body_b);
    run_healing_pass(&pool, &source, &ManifestWorkGate::always_enabled())
        .await
        .unwrap();
    assert_eq!(source.gets.load(Ordering::SeqCst), 2);
    assert!(source.max_inflight.load(Ordering::SeqCst) >= 2);
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
    ));
    tokio::time::sleep(Duration::from_millis(200)).await;
    insert_healable(&pool, 3, "s3://peer-a", &body).await;
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            if source.gets.load(Ordering::SeqCst) >= 1 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("healing worker should run on NOTIFY, not wait for the 30s poll");
    token.cancel();
    worker.await.unwrap().unwrap();
}
