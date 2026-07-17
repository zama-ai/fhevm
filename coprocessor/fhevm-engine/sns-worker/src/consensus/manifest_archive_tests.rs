use super::*;
use alloy::signers::local::PrivateKeySigner;
use ciphertext_attestation::{
    manifest::{
        block_content_digest, detailed_range_digest, BlockCiphertextDescriptor, DetailedRange,
        ManifestBlockEntry, ManifestPayload,
    },
    CiphertextFormat,
};
use serial_test::serial;
use sqlx::PgPool;
use std::{collections::BTreeMap, sync::Arc};
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};
use tokio::sync::Barrier;

const TEST_CONTEXT_ID: U256 = U256::ONE;
const TEST_CHAIN_ID: i64 = 7;
const TEST_BLOCK_NUMBER: i64 = 42;
const TEST_WORKERS: usize = 8;

#[tokio::test]
async fn peer_manifest_authentication_binds_publisher_and_numbered_key() {
    let signer = PrivateKeySigner::random();
    let manifest = sign_payload(&signer, payload(signer.address(), 1, 0, None)).await;
    let body = serde_json::to_vec(&manifest).expect("serialize peer manifest");
    let key = manifest_object_key(&manifest);

    authenticate_manifest_object(signer.address(), &key, &body)
        .expect("authenticate peer manifest");

    let wrong_publisher = PrivateKeySigner::random().address();
    assert!(authenticate_manifest_object(wrong_publisher, &key, &body)
        .unwrap_err()
        .to_string()
        .contains("does not match expected publisher"),);

    let wrong_numbered_key = key.strip_suffix("/0").unwrap().to_owned() + "/1";
    assert!(
        authenticate_manifest_object(signer.address(), &wrong_numbered_key, &body)
            .unwrap_err()
            .to_string()
            .contains("does not match signed identity"),
    );

    let mut invalid_signature = manifest;
    invalid_signature.signature[0] ^= 1;
    let invalid_body =
        serde_json::to_vec(&invalid_signature).expect("serialize invalid peer manifest");
    assert!(
        authenticate_manifest_object(signer.address(), &key, &invalid_body)
            .unwrap_err()
            .to_string()
            .contains("signature or payload is invalid"),
    );
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "database-backed concurrent multi-coprocessor manifest archive simulation"]
#[serial]
async fn concurrent_peer_archive_supports_db_only_multi_coprocessor_quorum() {
    let (_instance, pool) = setup_archive_db().await;
    let signers = [
        PrivateKeySigner::random(),
        PrivateKeySigner::random(),
        PrivateKeySigner::random(),
    ];
    let manifests = [
        sign_payload(&signers[0], payload(signers[0].address(), 1, 0, None)).await,
        sign_payload(&signers[1], payload(signers[1].address(), 1, 0, None)).await,
        sign_payload(&signers[2], payload(signers[2].address(), 9, 0, None)).await,
    ];

    for manifest in &manifests {
        let outcomes = concurrent_store(&pool, manifest, TEST_WORKERS).await;
        assert_eq!(
            outcomes
                .iter()
                .filter(|outcome| matches!(outcome, Ok(StoreOutcome::Inserted)))
                .count(),
            1,
            "exactly one worker must insert each peer manifest",
        );
        assert_eq!(
            outcomes
                .iter()
                .filter(|outcome| matches!(outcome, Ok(StoreOutcome::AlreadyPresent)))
                .count(),
            TEST_WORKERS - 1,
        );
    }

    let mut commitment_groups = BTreeMap::<B256, Vec<Address>>::new();
    for signer in &signers {
        let mut trx = pool.begin().await.expect("begin archived tip read");
        let tip = load_tip_eligible_manifest(
            &mut trx,
            signer.address(),
            ManifestVersion::V1,
            TEST_CONTEXT_ID,
            TEST_CHAIN_ID,
            TEST_BLOCK_NUMBER,
            test_block_hash(),
        )
        .await
        .expect("load archived peer tip")
        .expect("peer revision zero is tip eligible");
        trx.commit().await.expect("commit archived tip read");
        commitment_groups
            .entry(tip.signed.payload.detailed_range.digest)
            .or_default()
            .push(signer.address());
    }

    let group_sizes = commitment_groups.values().map(Vec::len).collect::<Vec<_>>();
    assert_eq!(group_sizes.iter().copied().max(), Some(2));
    assert_eq!(group_sizes.iter().sum::<usize>(), signers.len());
    assert_eq!(archive_row_count(&pool).await, 3);
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "database-backed concurrent numbered peer manifest revision simulation"]
#[serial]
async fn concurrent_peer_archive_retains_revisions_and_does_not_select_a_gap() {
    let (_instance, pool) = setup_archive_db().await;
    let signer = PrivateKeySigner::random();
    let revision_zero = sign_payload(&signer, payload(signer.address(), 1, 0, None)).await;
    let revision_one = sign_payload(
        &signer,
        payload(
            signer.address(),
            2,
            1,
            Some(manifest_reference(&revision_zero)),
        ),
    )
    .await;
    let revision_two = sign_payload(
        &signer,
        payload(
            signer.address(),
            3,
            2,
            Some(manifest_reference(&revision_one)),
        ),
    )
    .await;

    assert_single_insert(concurrent_store(&pool, &revision_zero, TEST_WORKERS).await);
    assert_single_insert(concurrent_store(&pool, &revision_two, TEST_WORKERS).await);
    assert_eq!(
        load_tip_revision(&pool, signer.address()).await,
        Some(0),
        "revision two must not become authoritative while revision one is absent",
    );

    assert_single_insert(concurrent_store(&pool, &revision_one, TEST_WORKERS).await);
    assert_eq!(load_tip_revision(&pool, signer.address()).await, Some(2));
    assert_eq!(archive_row_count(&pool).await, 3);
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "database-backed concurrent peer manifest equivocation simulation"]
#[serial]
async fn concurrent_peer_archive_rejects_equivocation_at_one_numbered_key() {
    let (_instance, pool) = setup_archive_db().await;
    let signer = PrivateKeySigner::random();
    let first = sign_payload(&signer, payload(signer.address(), 1, 0, None)).await;
    let conflicting = sign_payload(&signer, payload(signer.address(), 9, 0, None)).await;
    let barrier = Arc::new(Barrier::new(TEST_WORKERS));
    let mut workers = Vec::with_capacity(TEST_WORKERS);

    for worker_id in 0..TEST_WORKERS {
        let pool = pool.clone();
        let barrier = Arc::clone(&barrier);
        let manifest = if worker_id % 2 == 0 {
            first.clone()
        } else {
            conflicting.clone()
        };
        workers.push(tokio::spawn(async move {
            let body = serde_json::to_vec(&manifest).expect("serialize peer manifest");
            let key = manifest_object_key(&manifest);
            barrier.wait().await;
            let mut trx = pool.begin().await.expect("begin equivocation insert");
            match store_authenticated_manifest(&mut trx, manifest.payload.publisher, &key, &body)
                .await
            {
                Ok(stored) => {
                    trx.commit().await.expect("commit equivocation winner");
                    Ok(stored.outcome)
                }
                Err(err) => {
                    trx.rollback().await.expect("rollback equivocation loser");
                    Err(err.to_string())
                }
            }
        }));
    }

    let mut outcomes = Vec::with_capacity(TEST_WORKERS);
    for worker in workers {
        outcomes.push(worker.await.expect("equivocation worker panicked"));
    }
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| matches!(outcome, Ok(StoreOutcome::Inserted)))
            .count(),
        1,
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| outcome
                .as_ref()
                .is_err_and(|err| err.contains("equivocation")))
            .count(),
        TEST_WORKERS / 2,
    );
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
    let body = Arc::new(serde_json::to_vec(manifest).expect("serialize peer manifest"));
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
            let mut trx = pool.begin().await.expect("begin peer archive insert");
            match store_authenticated_manifest(&mut trx, publisher, &key, &body).await {
                Ok(stored) => {
                    trx.commit().await.expect("commit peer archive insert");
                    Ok(stored.outcome)
                }
                Err(err) => {
                    trx.rollback().await.expect("rollback peer archive insert");
                    Err(err.to_string())
                }
            }
        }));
    }

    let mut outcomes = Vec::with_capacity(worker_count);
    for worker in workers {
        outcomes.push(worker.await.expect("peer archive worker panicked"));
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

async fn load_tip_revision(pool: &PgPool, publisher: Address) -> Option<u64> {
    let mut trx = pool.begin().await.expect("begin tip revision read");
    let revision = load_tip_eligible_manifest(
        &mut trx,
        publisher,
        ManifestVersion::V1,
        TEST_CONTEXT_ID,
        TEST_CHAIN_ID,
        TEST_BLOCK_NUMBER,
        test_block_hash(),
    )
    .await
    .expect("load tip-eligible revision")
    .map(|manifest| manifest.signed.payload.revision);
    trx.commit().await.expect("commit tip revision read");
    revision
}

async fn archive_row_count(pool: &PgPool) -> i64 {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM block_consensus_manifest")
        .fetch_one(pool)
        .await
        .expect("count archived manifests")
}

async fn sign_payload(signer: &PrivateKeySigner, payload: ManifestPayload) -> SignedManifest {
    payload.sign(signer).await.expect("sign peer manifest")
}

fn manifest_reference(manifest: &SignedManifest) -> ManifestReference {
    ManifestReference {
        block_number: manifest.payload.publication_block_number,
        block_hash: manifest.payload.publication_block_hash,
        revision: manifest.payload.revision,
        manifest_digest: manifest.digest().expect("digest peer manifest"),
    }
}

fn payload(
    publisher: Address,
    material: u8,
    revision: u64,
    supersedes: Option<ManifestReference>,
) -> ManifestPayload {
    let block_number = U256::from(TEST_BLOCK_NUMBER);
    let block_hash = test_block_hash();
    let parent_block_hash = B256::repeat_byte(0xa9);
    let descriptors = vec![BlockCiphertextDescriptor {
        handle: B256::repeat_byte(1),
        keyset_id: U256::from(17),
        gateway_key_id: Some(U256::from(17)),
        ct64_digest: B256::repeat_byte(material),
        ct128_digest: B256::repeat_byte(material.wrapping_add(1)),
        ct128_format: CiphertextFormat::CompressedOnCpu,
    }];
    let block_digest = block_content_digest(
        ManifestVersion::V1,
        TEST_CONTEXT_ID,
        U256::from(TEST_CHAIN_ID),
        block_number,
        block_hash,
        &descriptors,
    )
    .expect("compute peer block digest");
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
        publisher,
        coprocessor_context_id: TEST_CONTEXT_ID,
        host_chain_id: U256::from(TEST_CHAIN_ID),
        publication_block_number: block_number,
        publication_block_hash: block_hash,
        publication_parent_block_hash: parent_block_hash,
        revision,
        supersedes,
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
        full_consensus_checkpoint: None,
        previous_manifest: None,
    }
}

fn test_block_hash() -> B256 {
    B256::repeat_byte(0xaa)
}
