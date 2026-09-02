use super::*;
use alloy::signers::local::PrivateKeySigner;
use block_manifest::{
    block_content_digest, detailed_range_digest, BlockCiphertextDescriptor, CiphertextFormat,
    DetailedRange, ManifestBlockEntry, ManifestPayload,
};
use serial_test::serial;
use sqlx::PgPool;
use std::sync::Arc;
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};
use tokio::sync::Barrier;

const TEST_CONTEXT_ID: U256 = U256::ONE;
const TEST_CHAIN_ID: i64 = 7;
const TEST_BLOCK_NUMBER: i64 = 42;
const TEST_WORKERS: usize = 8;

#[tokio::test]
async fn authentication_binds_publisher_and_numbered_key() {
    let signer = PrivateKeySigner::random();
    let manifest = sign_payload(&signer, payload(signer.address(), 1, 0)).await;
    let body = serde_json::to_vec(&manifest).expect("serialize manifest");
    let key = manifest_object_key(&manifest);

    authenticate_manifest_object(signer.address(), &key, &body).expect("authenticate manifest");

    let wrong_publisher = PrivateKeySigner::random().address();
    assert!(authenticate_manifest_object(wrong_publisher, &key, &body)
        .unwrap_err()
        .to_string()
        .contains("does not match expected publisher"),);

    let wrong_numbered_key = key.strip_suffix("revision/0").unwrap().to_owned() + "revision/1";
    assert!(
        authenticate_manifest_object(signer.address(), &wrong_numbered_key, &body)
            .unwrap_err()
            .to_string()
            .contains("does not match signed identity"),
    );

    let mut invalid_signature = manifest;
    invalid_signature.signature[0] ^= 1;
    let invalid_body = serde_json::to_vec(&invalid_signature).expect("serialize invalid signature");
    assert!(
        authenticate_manifest_object(signer.address(), &key, &invalid_body)
            .unwrap_err()
            .to_string()
            .contains("signature or payload is invalid"),
    );
}

#[tokio::test]
async fn v1_manifest_object_key_is_generation_namespaced() {
    let signer = PrivateKeySigner::random();
    let mut payload = payload(signer.address(), 1, 0);
    payload.version = ManifestVersion::V1;
    payload.consensus_epoch = "9".to_owned();
    let block_digest = block_content_digest(
        ManifestVersion::V1,
        payload.coprocessor_context_id,
        payload.host_chain_id,
        payload.publication_block_number,
        payload.publication_block_hash,
        &payload.detailed_range.blocks[0].ciphertexts,
    )
    .expect("compute v1 block digest");
    payload.detailed_range.blocks[0].block_content_digest = block_digest;
    payload.detailed_range.digest = detailed_range_digest(
        ManifestVersion::V1,
        payload.coprocessor_context_id,
        payload.host_chain_id,
        payload.detailed_range.first_block_number,
        payload.detailed_range.last_block_number,
        &[block_digest],
    );
    let manifest = sign_payload(&signer, payload).await;

    assert_eq!(
        manifest_object_key(&manifest),
        format!(
            "manifests/v_1/context_1/chain_7/block_42/hash_{}/consensus_epoch/9/revision/0",
            hex::encode(test_block_hash())
        ),
    );
}

#[tokio::test]
#[serial]
async fn local_archive_stores_and_loads_by_publication_identity() {
    let (_instance, pool) = setup_archive_db().await;
    let signer = PrivateKeySigner::random();
    let manifest = sign_payload(&signer, payload(signer.address(), 1, 0)).await;
    let body = serde_json::to_vec(&manifest).expect("serialize manifest");
    let key = manifest_object_key(&manifest);

    let mut trx = pool.begin().await.expect("begin local archive insert");
    let stored = store_authenticated_manifest(
        &mut trx,
        signer.address(),
        &key,
        &body,
        ManifestSource::Local,
    )
    .await
    .expect("store local manifest");
    assert_eq!(stored.outcome, StoreOutcome::Inserted);
    trx.commit().await.expect("commit local archive insert");

    let mut trx = pool.begin().await.expect("begin local archive load");
    let loaded = load_manifest_revision(
        &mut trx,
        signer.address(),
        ManifestVersion::V1,
        TEST_CONTEXT_ID,
        TEST_CHAIN_ID,
        block_manifest::LEGACY_CONSENSUS_EPOCH,
        TEST_BLOCK_NUMBER,
        test_block_hash(),
        0,
    )
    .await
    .expect("load by publication identity")
    .expect("stored revision exists");
    trx.commit().await.expect("commit local archive load");
    assert_eq!(loaded.digest, stored.manifest.digest);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn concurrent_local_archive_inserts_once() {
    let (_instance, pool) = setup_archive_db().await;
    let signer = PrivateKeySigner::random();
    let manifest = sign_payload(&signer, payload(signer.address(), 1, 0)).await;
    let outcomes = concurrent_store(&pool, &manifest, TEST_WORKERS).await;
    assert_single_insert(outcomes);
    assert_eq!(archive_row_count(&pool).await, 1);
}

async fn setup_archive_db() -> (DBInstance, PgPool) {
    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create manifest archive database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(16)
        .connect(instance.db_url())
        .await
        .expect("connect manifest archive database");
    (instance, pool)
}

async fn concurrent_store(
    pool: &PgPool,
    manifest: &SignedManifest,
    worker_count: usize,
) -> Vec<Result<StoreOutcome, String>> {
    let barrier = Arc::new(Barrier::new(worker_count));
    let body = Arc::new(serde_json::to_vec(manifest).expect("serialize manifest"));
    let key = Arc::new(manifest_object_key(manifest));
    let publisher = manifest.payload.publisher;
    let mut workers = Vec::with_capacity(worker_count);
    for _ in 0..worker_count {
        let pool = pool.clone();
        let barrier = Arc::clone(&barrier);
        let body = Arc::clone(&body);
        let key = Arc::clone(&key);
        workers.push(tokio::spawn(async move {
            barrier.wait().await;
            let mut trx = pool.begin().await.expect("begin local archive insert");
            match store_authenticated_manifest(
                &mut trx,
                publisher,
                &key,
                &body,
                ManifestSource::Local,
            )
            .await
            {
                Ok(stored) => {
                    trx.commit().await.expect("commit local archive insert");
                    Ok(stored.outcome)
                }
                Err(err) => {
                    trx.rollback().await.expect("rollback local archive insert");
                    Err(err.to_string())
                }
            }
        }));
    }

    let mut outcomes = Vec::with_capacity(worker_count);
    for worker in workers {
        outcomes.push(worker.await.expect("local archive worker panicked"));
    }
    outcomes
}

fn assert_single_insert(outcomes: Vec<Result<StoreOutcome, String>>) {
    assert!(outcomes.iter().all(Result::is_ok), "{outcomes:?}");
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| matches!(outcome, Ok(StoreOutcome::Inserted)))
            .count(),
        1,
    );
}

async fn archive_row_count(pool: &PgPool) -> i64 {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM block_manifest")
        .fetch_one(pool)
        .await
        .expect("count archived manifests")
}

async fn sign_payload(signer: &PrivateKeySigner, payload: ManifestPayload) -> SignedManifest {
    payload.sign(signer).await.expect("sign manifest")
}

fn payload(publisher: Address, material: u8, revision: u64) -> ManifestPayload {
    let block_number = U256::from(TEST_BLOCK_NUMBER);
    let block_hash = test_block_hash();
    let parent_block_hash = B256::repeat_byte(0xa9);
    let descriptors = vec![BlockCiphertextDescriptor::computed(
        B256::repeat_byte(1),
        U256::from(17),
        Some(U256::from(17)),
        B256::repeat_byte(material),
        B256::repeat_byte(material.wrapping_add(1)),
        CiphertextFormat::CompressedOnCpu,
    )];
    let block_digest = block_content_digest(
        ManifestVersion::V1,
        TEST_CONTEXT_ID,
        U256::from(TEST_CHAIN_ID),
        block_number,
        block_hash,
        &descriptors,
    )
    .expect("compute block digest");
    let detailed_digest = detailed_range_digest(
        ManifestVersion::V1,
        TEST_CONTEXT_ID,
        U256::from(TEST_CHAIN_ID),
        block_number,
        block_number,
        &[block_digest],
    );
    ManifestPayload {
        version: ManifestVersion::V1,
        consensus_epoch: block_manifest::LEGACY_CONSENSUS_EPOCH.to_owned(),
        publisher,
        coprocessor_context_id: TEST_CONTEXT_ID,
        host_chain_id: U256::from(TEST_CHAIN_ID),
        publication_block_number: block_number,
        publication_block_hash: block_hash,
        publication_parent_block_hash: parent_block_hash,
        revision,
        detailed_range: DetailedRange {
            first_block_number: block_number,
            last_block_number: block_number,
            digest: detailed_digest,
            blocks: vec![ManifestBlockEntry {
                block_number,
                block_hash,
                parent_block_hash,
                block_content_digest: block_digest,
                ciphertexts: descriptors,
            }],
        },
        historical_ranges: vec![],
    }
}

fn test_block_hash() -> B256 {
    B256::repeat_byte(0xaa)
}
