use alloy::signers::local::PrivateKeySigner;
use alloy_primitives::{Address, B256, U256};
use ciphertext_attestation::manifest::{
    block_content_digest, detailed_range_digest, DetailedRange, ManifestBlockEntry,
    ManifestPayload, ManifestReference, ManifestVersion, SignedManifest,
};
use fhevm_engine_common::{drift_revert, types::CoproSigner};
use serial_test::serial;
use sqlx::{PgPool, Row};
use std::{sync::Arc, time::Duration};
use test_harness::instance::{setup_test_db, ImportMode};
use tokio::sync::Barrier;

use super::*;
use crate::consensus::{
    manifest::{
        is_block_manifest_ready, load_manifest_descriptors, prepare_manifest,
        queue_manifest_revision_after_replay, seal_block_content,
    },
    manifest_archive::{load_manifest_revision, AuthenticatedManifest},
};

fn payload(publisher: Address, context: U256) -> ManifestPayload {
    let block_number = U256::from(42);
    let block_hash = B256::repeat_byte(0xab);
    let content = block_content_digest(
        ManifestVersion::V1,
        context,
        U256::from(7),
        block_number,
        block_hash,
        &[],
    )
    .unwrap();
    ManifestPayload {
        version: ManifestVersion::V1,
        publisher,
        coprocessor_context_id: context,
        host_chain_id: U256::from(7),
        publication_block_number: block_number,
        publication_block_hash: block_hash,
        publication_parent_block_hash: B256::repeat_byte(0xaa),
        revision: 0,
        supersedes: None,
        detailed_range: DetailedRange {
            first_block_number: block_number,
            last_block_number: block_number,
            digest: detailed_range_digest(
                ManifestVersion::V1,
                context,
                U256::from(7),
                block_number,
                block_number,
                &[content],
            ),
            blocks: vec![ManifestBlockEntry {
                block_number,
                block_hash,
                parent_block_hash: B256::repeat_byte(0xaa),
                block_content_digest: content,
                ciphertexts: Vec::new(),
            }],
        },
        historical_ranges: Vec::new(),
        full_consensus_checkpoint: None,
        previous_manifest: None,
    }
}

async fn wait_for_manifest_publication(pool: &PgPool, chain_id: i64, block_hash: B256) {
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            let published = sqlx::query_scalar::<_, bool>(
                "SELECT manifest_published
                   FROM block_manifest_state
                  WHERE host_chain_id = $1 AND block_hash = $2",
            )
            .bind(chain_id)
            .bind(block_hash.as_slice())
            .fetch_one(pool)
            .await
            .expect("read publication state");
            if published {
                return;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("publisher loop publishes within fast cadence");
}

#[test]
fn object_key_matches_the_v1_layout() {
    let manifest = SignedManifest {
        payload: payload(Address::ZERO, U256::ONE),
        signature: Vec::new(),
    };
    assert_eq!(
        manifest_object_key(&manifest),
        format!("manifests/v1/1/7/42/{}/0", "ab".repeat(32)),
    );
}

#[tokio::test]
async fn immutable_retry_accepts_the_existing_signature_for_the_same_payload() {
    let signer = PrivateKeySigner::random();
    let intended = payload(signer.address(), U256::ONE)
        .sign(&signer)
        .await
        .unwrap();
    let existing = payload(signer.address(), U256::ONE)
        .sign(&signer)
        .await
        .unwrap();
    let existing = serde_json::to_vec(&existing).unwrap();

    validate_existing_manifest(&existing, &intended, "bucket", "key").unwrap();
}

#[tokio::test]
async fn immutable_retry_rejects_a_different_signed_payload() {
    let signer = PrivateKeySigner::random();
    let intended = payload(signer.address(), U256::ONE)
        .sign(&signer)
        .await
        .unwrap();
    let existing = payload(signer.address(), U256::from(2))
        .sign(&signer)
        .await
        .unwrap();
    let existing = serde_json::to_vec(&existing).unwrap();

    assert!(
        validate_existing_manifest(&existing, &intended, "bucket", "key")
            .unwrap_err()
            .to_string()
            .contains("different payload")
    );
}

#[tokio::test]
#[serial(s3)]
async fn immutable_s3_retry_reads_the_existing_body_and_rejects_a_conflict() {
    let localstack = test_harness::localstack::start_localstack()
        .await
        .expect("start LocalStack for immutable retry");
    let client = test_harness::localstack::create_localstack_s3_client(localstack.host_port).await;
    let bucket = "immutable-manifest-retry";
    client
        .create_bucket()
        .bucket(bucket)
        .send()
        .await
        .expect("create immutable retry bucket");

    let signer = PrivateKeySigner::random();
    let intended = payload(signer.address(), U256::ONE)
        .sign(&signer)
        .await
        .expect("sign intended manifest");
    let intended_body = serde_json::to_vec(&intended).expect("encode intended manifest");
    let existing_key = manifest_object_key(&intended);
    client
        .put_object()
        .bucket(bucket)
        .key(&existing_key)
        .body(intended_body.clone().into())
        .send()
        .await
        .expect("seed immutable manifest object");
    assert_eq!(
        put_immutable_manifest(&client, bucket, &existing_key, &intended, &intended_body)
            .await
            .expect("matching immutable retry succeeds"),
        intended_body
    );

    let conflicting = payload(signer.address(), U256::from(2))
        .sign(&signer)
        .await
        .expect("sign conflicting manifest");
    let conflicting_body = serde_json::to_vec(&conflicting).expect("encode conflicting manifest");
    let conflicting_key = format!("{existing_key}-conflict");
    client
        .put_object()
        .bucket(bucket)
        .key(&conflicting_key)
        .body(conflicting_body.into())
        .send()
        .await
        .expect("seed conflicting immutable manifest object");
    let err = put_immutable_manifest(&client, bucket, &conflicting_key, &intended, &intended_body)
        .await
        .expect_err("different immutable body must be rejected");
    assert!(err.to_string().contains("different payload"));
}

#[tokio::test]
#[serial(db)]
async fn readiness_and_preparation_reject_incomplete_or_corrupted_block_content() {
    const CHAIN_ID: i64 = 137;
    let block_hash = B256::repeat_byte(0x42);
    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create manifest integrity database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect(instance.db_url())
        .await
        .expect("connect manifest integrity database");
    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        42,
        block_hash,
        B256::repeat_byte(0x41),
        B256::repeat_byte(0x51),
        B256::repeat_byte(0x52),
        B256::repeat_byte(0x53),
        B256::repeat_byte(0x54),
        B256::repeat_byte(0x55),
        B256::repeat_byte(0x56),
        B256::repeat_byte(0x57),
    )
    .await;

    sqlx::query(
        "UPDATE pbs_computations_branch
            SET is_completed = FALSE
          WHERE host_chain_id = $1 AND producer_block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(block_hash.as_slice())
    .execute(&pool)
    .await
    .expect("make manifest handle incomplete");
    let block = load_seeded_block(&pool, CHAIN_ID, block_hash).await;
    let mut trx = pool
        .begin()
        .await
        .expect("begin incomplete readiness check");
    assert!(!is_block_manifest_ready(&mut trx, &block)
        .await
        .expect("incomplete handle is not a manifest error"));
    trx.rollback()
        .await
        .expect("rollback incomplete readiness check");

    sqlx::query(
        "UPDATE pbs_computations_branch
            SET is_completed = FALSE,
                is_error = TRUE,
                error_message = 'test failure',
                error_at = NOW()
          WHERE host_chain_id = $1 AND producer_block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(block_hash.as_slice())
    .execute(&pool)
    .await
    .expect("make manifest handle fail");
    let mut trx = pool.begin().await.expect("begin failed readiness check");
    let err = is_block_manifest_ready(&mut trx, &block)
        .await
        .expect_err("failed handle must abort manifest publication");
    assert!(err.to_string().contains("failed allowed handle"));
    trx.rollback()
        .await
        .expect("rollback failed readiness check");

    sqlx::query(
        "UPDATE pbs_computations_branch
            SET is_completed = TRUE,
                is_error = FALSE,
                error_message = NULL,
                error_at = NULL
          WHERE host_chain_id = $1 AND producer_block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(block_hash.as_slice())
    .execute(&pool)
    .await
    .expect("restore manifest handle");
    sqlx::query(
        "UPDATE ciphertext_digest_branch
            SET ciphertext128_format = 99
          WHERE host_chain_id = $1 AND producer_block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(block_hash.as_slice())
    .execute(&pool)
    .await
    .expect("corrupt manifest ciphertext format");
    let mut trx = pool.begin().await.expect("begin invalid descriptor check");
    let err = load_manifest_descriptors(&mut trx, &block)
        .await
        .expect_err("invalid ciphertext format must not enter a manifest");
    assert!(err.to_string().contains("invalid ct128 format"));
    trx.rollback()
        .await
        .expect("rollback invalid descriptor check");
    sqlx::query(
        "UPDATE ciphertext_digest_branch
            SET ciphertext128_format = 11
          WHERE host_chain_id = $1 AND producer_block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(block_hash.as_slice())
    .execute(&pool)
    .await
    .expect("restore manifest ciphertext format");

    seal_seeded_block(&pool, CHAIN_ID, block_hash, U256::ONE).await;
    sqlx::query(
        "UPDATE block_manifest_state
            SET block_handle_count = 2
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(block_hash.as_slice())
    .execute(&pool)
    .await
    .expect("corrupt stored descriptor count");

    let signer = PrivateKeySigner::random();
    let block = load_seeded_block(&pool, CHAIN_ID, block_hash).await;
    let mut trx = pool
        .begin()
        .await
        .expect("begin corrupted manifest preparation");
    let err = prepare_manifest(&mut trx, &block, U256::ONE, signer.address())
        .await
        .expect_err("corrupted sealed state must not be published");
    assert!(err.to_string().contains("descriptor count changed"));
    trx.rollback()
        .await
        .expect("rollback corrupted manifest preparation");

    let state = sqlx::query(
        "SELECT manifest_published,
                (SELECT COUNT(*) FROM block_manifest WHERE host_chain_id = $1) AS archive_count
           FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(block_hash.as_slice())
    .fetch_one(&pool)
    .await
    .expect("load corrupted manifest state");
    assert!(!state.get::<bool, _>("manifest_published"));
    assert_eq!(state.get::<i64, _>("archive_count"), 0);
}

#[tokio::test]
#[serial(db)]
async fn fast_publisher_loop_seals_archives_and_uploads_a_manifest() {
    const CHAIN_ID: i64 = 137;
    let block_hash = B256::repeat_byte(0x62);
    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create fast publisher database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .connect(instance.db_url())
        .await
        .expect("connect fast publisher database");
    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        62,
        block_hash,
        B256::repeat_byte(0x61),
        B256::repeat_byte(0x63),
        B256::repeat_byte(0x64),
        B256::repeat_byte(0x65),
        B256::repeat_byte(0x66),
        B256::repeat_byte(0x67),
        B256::repeat_byte(0x68),
        B256::repeat_byte(0x69),
    )
    .await;

    let localstack = test_harness::localstack::start_localstack()
        .await
        .expect("start LocalStack for fast publisher loop");
    let client =
        Arc::new(test_harness::localstack::create_localstack_s3_client(localstack.host_port).await);
    let bucket = "fast-manifest-publisher".to_owned();
    client
        .create_bucket()
        .bucket(&bucket)
        .send()
        .await
        .expect("create fast publisher bucket");

    let signer: CoproSigner = Arc::new(PrivateKeySigner::random());
    let token = tokio_util::sync::CancellationToken::new();
    let publisher = tokio::spawn(run_manifest_publisher_with_poll_interval(
        pool.clone(),
        token.clone(),
        ManifestPublisherContext {
            bucket: bucket.clone(),
            client: Arc::clone(&client),
            signer: Arc::clone(&signer),
            consensus: ConsensusConfig::default(),
            stack_mode: fhevm_engine_common::versioning::StackMode::new(false),
        },
        Duration::from_millis(10),
    ));

    wait_for_manifest_publication(&pool, CHAIN_ID, block_hash).await;
    token.cancel();
    tokio::time::timeout(Duration::from_secs(1), publisher)
        .await
        .expect("publisher loop exits after cancellation")
        .expect("publisher task joins")
        .expect("publisher loop succeeds");

    let manifest = load_local_revision(
        &pool,
        signer.address(),
        U256::ONE,
        CHAIN_ID,
        62,
        block_hash,
        0,
    )
    .await;
    let key = manifest_object_key(&manifest.signed);
    let body = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .expect("load published manifest object")
        .body
        .collect()
        .await
        .expect("read published manifest object")
        .into_bytes();
    let stored: SignedManifest = serde_json::from_slice(&body).expect("decode published manifest");
    assert_eq!(stored, manifest.signed);
}

#[tokio::test(flavor = "multi_thread")]
#[serial(db)]
#[ignore = "database-backed concurrent replay revision queue"]
async fn concurrent_replay_revision_requests_queue_exactly_once() {
    const CHAIN_ID: i64 = 7;
    const WORKER_COUNT: usize = 8;
    let block_hash = B256::repeat_byte(0x42);
    let parent_hash = B256::repeat_byte(0x41);
    let sealed_digest = B256::repeat_byte(0x51);
    let range_digest = B256::repeat_byte(0x52);
    let manifest_digest = B256::repeat_byte(0x53);

    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create concurrent revision queue database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(WORKER_COUNT as u32 + 2)
        .connect(instance.db_url())
        .await
        .expect("connect concurrent revision queue database");
    sqlx::query(
        r#"
        INSERT INTO block_manifest_state (
            host_chain_id,
            block_number,
            block_hash,
            parent_block_hash,
            publication_cadence,
            block_content_digest,
            block_handle_count,
            manifest_range_start,
            manifest_range_digest,
            manifest_revision,
            manifest_publisher,
            manifest_digest,
            manifest_published,
            manifest_published_at
        ) VALUES ($1, 42, $2, $3, 1, $4, 1, 42, $5, 0, $7, $6, TRUE, NOW())
        "#,
    )
    .bind(CHAIN_ID)
    .bind(block_hash.as_slice())
    .bind(parent_hash.as_slice())
    .bind(sealed_digest.as_slice())
    .bind(range_digest.as_slice())
    .bind(manifest_digest.as_slice())
    .bind(Address::repeat_byte(0x54).as_slice())
    .execute(&pool)
    .await
    .expect("seed published manifest state");

    let barrier = Arc::new(Barrier::new(WORKER_COUNT));
    let mut workers = Vec::with_capacity(WORKER_COUNT);
    for _ in 0..WORKER_COUNT {
        let pool = pool.clone();
        let barrier = barrier.clone();
        workers.push(tokio::spawn(async move {
            let mut trx = pool.begin().await.map_err(|err| err.to_string())?;
            barrier.wait().await;
            let result = queue_manifest_revision_after_replay(&mut trx, CHAIN_ID, block_hash, 0)
                .await
                .map_err(|err| err.to_string());
            match result {
                Ok(revision) => {
                    trx.commit().await.map_err(|err| err.to_string())?;
                    Ok(revision)
                }
                Err(err) => {
                    trx.rollback()
                        .await
                        .map_err(|rollback| rollback.to_string())?;
                    Err(err)
                }
            }
        }));
    }

    let mut outcomes = Vec::with_capacity(WORKER_COUNT);
    for worker in workers {
        outcomes.push(worker.await.expect("revision queue worker joined"));
    }
    assert_eq!(
        outcomes.iter().filter(|outcome| outcome == &&Ok(1)).count(),
        1,
        "exactly one worker must queue revision 1: {outcomes:?}",
    );
    assert!(
        outcomes
            .iter()
            .filter_map(|outcome| outcome.as_ref().err())
            .all(|err| err.contains("manifest replay could not queue revision 1")),
        "losing workers must observe the stale revision guard: {outcomes:?}",
    );

    let state = sqlx::query(
        r#"
        SELECT manifest_revision,
               manifest_published,
               block_content_digest,
               block_handle_count,
               manifest_range_start,
               manifest_range_digest,
               manifest_digest,
               manifest_published_at IS NULL AS publication_time_missing
          FROM block_manifest_state
         WHERE host_chain_id = $1 AND block_hash = $2
        "#,
    )
    .bind(CHAIN_ID)
    .bind(block_hash.as_slice())
    .fetch_one(&pool)
    .await
    .expect("load queued revision state");
    assert_eq!(state.get::<i64, _>("manifest_revision"), 1);
    assert!(!state.get::<bool, _>("manifest_published"));
    for field in [
        "block_content_digest",
        "manifest_range_digest",
        "manifest_digest",
    ] {
        assert!(state.get::<Option<Vec<u8>>, _>(field).is_none(), "{field}");
    }
    assert!(state.get::<Option<i64>, _>("block_handle_count").is_none());
    assert!(state
        .get::<Option<i64>, _>("manifest_range_start")
        .is_none());
    assert!(state.get::<bool, _>("publication_time_missing"));
}

#[tokio::test]
#[serial(db)]
#[ignore = "database-backed manifest evidence preservation during production revert"]
async fn production_revert_preserves_immutable_manifest_archive_evidence() {
    const CHAIN_ID: i64 = 100;
    let publisher = [0x11u8; 20];
    let context = [0x22u8; 32];
    let block_hash = [0x33u8; 32];
    let manifest_digest = [0x44u8; 32];
    let signed_manifest = br#"{"signed":"historical-evidence"}"#;

    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create manifest evidence database");
    let pool = PgPool::connect(instance.db_url())
        .await
        .expect("connect manifest evidence database");
    sqlx::query(
        "INSERT INTO host_chains (chain_id, name, acl_contract_address) \
         VALUES ($1, 'manifest-evidence', '0x1')",
    )
    .bind(CHAIN_ID)
    .execute(&pool)
    .await
    .expect("insert host chain");
    sqlx::query(
        r#"
        INSERT INTO block_manifest (
            publisher,
            version,
            coprocessor_context_id,
            host_chain_id,
            publication_block_number,
            publication_block_hash,
            revision,
            manifest_digest,
            object_key,
            signed_manifest,
            manifest_source
        ) VALUES ($1, 1, $2, $3, 10, $4, 0, $5, $6, $7, 'local')
        "#,
    )
    .bind(publisher.as_slice())
    .bind(context.as_slice())
    .bind(CHAIN_ID)
    .bind(block_hash.as_slice())
    .bind(manifest_digest.as_slice())
    .bind("manifests/v1/1/100/10/evidence/0")
    .bind(signed_manifest.as_slice())
    .execute(&pool)
    .await
    .expect("insert immutable manifest evidence");

    drift_revert::execute_revert(&pool, CHAIN_ID, 6)
        .await
        .expect("execute production revert");

    let retained = sqlx::query(
        r#"
        SELECT manifest_digest, signed_manifest
          FROM block_manifest
         WHERE publisher = $1
           AND host_chain_id = $2
           AND publication_block_hash = $3
           AND revision = 0
        "#,
    )
    .bind(publisher.as_slice())
    .bind(CHAIN_ID)
    .bind(block_hash.as_slice())
    .fetch_one(&pool)
    .await
    .expect("immutable manifest evidence survives revert");
    assert_eq!(
        retained.get::<Vec<u8>, _>("manifest_digest"),
        manifest_digest.to_vec(),
    );
    assert_eq!(
        retained.get::<Vec<u8>, _>("signed_manifest"),
        signed_manifest.to_vec(),
    );
}

#[tokio::test]
#[serial(db)]
#[ignore = "PostgreSQL and LocalStack-backed production revision publication"]
async fn production_publisher_creates_and_uploads_superseding_revision() {
    const CHAIN_ID: i64 = 7;
    const BLOCK_NUMBER: i64 = 42;
    const CHILD_BLOCK_NUMBER: i64 = 43;
    let context = U256::ONE;
    let block_hash = B256::repeat_byte(0x42);
    let parent_hash = B256::repeat_byte(0x41);
    let handle = B256::repeat_byte(0x51);
    let transaction_id = B256::repeat_byte(0x52);
    let dependence_chain_id = B256::repeat_byte(0x53);
    let gateway_key_id = B256::repeat_byte(0x77);
    let drifted_keyset_id = B256::repeat_byte(0x99);
    let repaired_keyset_id = B256::repeat_byte(0x17);
    let ct64_digest = B256::repeat_byte(0x61);
    let ct128_digest = B256::repeat_byte(0x62);
    let child_block_hash = B256::repeat_byte(0x43);
    let child_gateway_key_id = B256::repeat_byte(0x78);

    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create revision publication database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(8)
        .connect(instance.db_url())
        .await
        .expect("connect revision publication database");
    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        BLOCK_NUMBER,
        block_hash,
        parent_hash,
        handle,
        transaction_id,
        dependence_chain_id,
        gateway_key_id,
        drifted_keyset_id,
        ct64_digest,
        ct128_digest,
    )
    .await;
    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        CHILD_BLOCK_NUMBER,
        child_block_hash,
        block_hash,
        B256::repeat_byte(0x54),
        B256::repeat_byte(0x55),
        B256::repeat_byte(0x56),
        child_gateway_key_id,
        B256::repeat_byte(0x18),
        B256::repeat_byte(0x63),
        B256::repeat_byte(0x64),
    )
    .await;

    let localstack = test_harness::localstack::start_localstack()
        .await
        .expect("start LocalStack for manifest revision publication");
    let client = test_harness::localstack::create_localstack_s3_client(localstack.host_port).await;
    let bucket = "manifest-revision-publication";
    client
        .create_bucket()
        .bucket(bucket)
        .send()
        .await
        .expect("create manifest revision bucket");
    let signer: CoproSigner = Arc::new(PrivateKeySigner::random());
    let consensus = ConsensusConfig::default();

    publish_pending_revision(
        &pool, &client, bucket, CHAIN_ID, context, &signer, &consensus,
    )
    .await;
    publish_pending_revision(
        &pool, &client, bucket, CHAIN_ID, context, &signer, &consensus,
    )
    .await;
    let revision_zero = load_local_revision(
        &pool,
        signer.address(),
        context,
        CHAIN_ID,
        BLOCK_NUMBER,
        block_hash,
        0,
    )
    .await;
    assert_eq!(revision_zero.signed.payload.revision, 0);
    assert!(revision_zero.signed.payload.supersedes.is_none());
    let child_revision_zero = load_local_revision(
        &pool,
        signer.address(),
        context,
        CHAIN_ID,
        CHILD_BLOCK_NUMBER,
        child_block_hash,
        0,
    )
    .await;
    assert_eq!(
        child_revision_zero.signed.payload.previous_manifest,
        Some(ManifestReference {
            publisher: revision_zero.signed.payload.publisher,
            block_number: revision_zero.signed.payload.publication_block_number,
            block_hash,
            revision: 0,
            manifest_digest: revision_zero.digest,
        })
    );

    let mut trx = pool.begin().await.expect("begin repaired revision request");
    sqlx::query("UPDATE keys SET key_id = $1 WHERE chain_id = $2 AND key_id_gw = $3")
        .bind(repaired_keyset_id.as_slice())
        .bind(CHAIN_ID)
        .bind(gateway_key_id.as_slice())
        .execute(trx.as_mut())
        .await
        .expect("apply replayed keyset material");
    assert_eq!(
        queue_manifest_revision_after_replay(&mut trx, CHAIN_ID, block_hash, 0)
            .await
            .expect("queue superseding manifest"),
        1,
    );
    assert_eq!(
        queue_manifest_revision_after_replay(&mut trx, CHAIN_ID, child_block_hash, 0)
            .await
            .expect("queue dependent superseding manifest"),
        1,
    );
    trx.commit()
        .await
        .expect("commit repaired revision request");

    let rotated_signer: CoproSigner = Arc::new(PrivateKeySigner::random());
    publish_pending_revision(
        &pool,
        &client,
        bucket,
        CHAIN_ID,
        context,
        &rotated_signer,
        &consensus,
    )
    .await;
    publish_pending_revision(
        &pool,
        &client,
        bucket,
        CHAIN_ID,
        context,
        &rotated_signer,
        &consensus,
    )
    .await;
    let revision_one = load_local_revision(
        &pool,
        rotated_signer.address(),
        context,
        CHAIN_ID,
        BLOCK_NUMBER,
        block_hash,
        1,
    )
    .await;
    assert_eq!(revision_one.signed.payload.revision, 1);
    assert_eq!(
        revision_one.signed.payload.supersedes,
        Some(ManifestReference {
            publisher: revision_zero.signed.payload.publisher,
            block_number: revision_zero.signed.payload.publication_block_number,
            block_hash,
            revision: 0,
            manifest_digest: revision_zero.digest,
        })
    );
    assert_ne!(
        revision_zero.signed.payload.detailed_range.digest,
        revision_one.signed.payload.detailed_range.digest,
    );
    assert_eq!(
        revision_one.signed.payload.detailed_range.blocks[0].ciphertexts[0].keyset_id,
        U256::from_be_slice(repaired_keyset_id.as_slice()),
    );
    let child_revision_one = load_local_revision(
        &pool,
        rotated_signer.address(),
        context,
        CHAIN_ID,
        CHILD_BLOCK_NUMBER,
        child_block_hash,
        1,
    )
    .await;
    assert_eq!(
        child_revision_one.signed.payload.supersedes,
        Some(ManifestReference {
            publisher: child_revision_zero.signed.payload.publisher,
            block_number: child_revision_zero.signed.payload.publication_block_number,
            block_hash: child_block_hash,
            revision: 0,
            manifest_digest: child_revision_zero.digest,
        })
    );
    assert_eq!(
        child_revision_one.signed.payload.previous_manifest,
        Some(ManifestReference {
            publisher: revision_one.signed.payload.publisher,
            block_number: revision_one.signed.payload.publication_block_number,
            block_hash,
            revision: 1,
            manifest_digest: revision_one.digest,
        })
    );
    assert_ne!(
        child_revision_zero.signed.payload.historical_ranges,
        child_revision_one.signed.payload.historical_ranges,
    );

    for manifest in [
        &revision_zero.signed,
        &child_revision_zero.signed,
        &revision_one.signed,
        &child_revision_one.signed,
    ] {
        let key = manifest_object_key(manifest);
        let body = client
            .get_object()
            .bucket(bucket)
            .key(&key)
            .send()
            .await
            .expect("load numbered manifest object")
            .body
            .collect()
            .await
            .expect("read numbered manifest object")
            .into_bytes();
        let stored: SignedManifest =
            serde_json::from_slice(&body).expect("decode numbered manifest object");
        assert_eq!(stored, *manifest);
        assert_eq!(
            key.rsplit('/').next().unwrap(),
            manifest.payload.revision.to_string()
        );
    }

    let state = sqlx::query(
        "SELECT manifest_revision, manifest_published, manifest_digest,
                (SELECT COUNT(*) FROM block_manifest
                  WHERE host_chain_id = $1
                    AND publication_block_hash = $2) AS archive_count
           FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(block_hash.as_slice())
    .fetch_one(&pool)
    .await
    .expect("load final revision publication state");
    assert_eq!(state.get::<i64, _>("manifest_revision"), 1);
    assert!(state.get::<bool, _>("manifest_published"));
    assert_eq!(
        state.get::<Vec<u8>, _>("manifest_digest"),
        revision_one.digest.to_vec(),
    );
    assert_eq!(state.get::<i64, _>("archive_count"), 2);
    let total_archive_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM block_manifest
          WHERE host_chain_id = $1",
    )
    .bind(CHAIN_ID)
    .fetch_one(&pool)
    .await
    .expect("count all uploaded manifest revisions");
    assert_eq!(total_archive_count, 4);
    for publisher in [signer.address(), rotated_signer.address()] {
        let publisher_archive_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM block_manifest
              WHERE publisher = $1 AND host_chain_id = $2",
        )
        .bind(publisher.as_slice())
        .bind(CHAIN_ID)
        .fetch_one(&pool)
        .await
        .expect("count manifest revisions for one signer");
        assert_eq!(publisher_archive_count, 2);
    }
}

#[tokio::test]
#[serial(db)]
#[ignore = "PostgreSQL and LocalStack-backed competing-lineage publication"]
async fn failed_creation_or_upload_does_not_block_competing_lineage_with_multiple_workers() {
    const CHAIN_ID: i64 = 9;
    const BLOCK_NUMBER: i64 = 42;
    const WORKER_COUNT: usize = 4;
    const ATTEMPTS_PER_WORKER: usize = 8;
    let context = U256::ONE;
    let blocked_hash = B256::repeat_byte(0x20);
    let creation_failed_hash = B256::repeat_byte(0x25);
    let ready_hash = B256::repeat_byte(0x30);

    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create competing-lineage publication database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(8)
        .connect(instance.db_url())
        .await
        .expect("connect competing-lineage publication database");
    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        BLOCK_NUMBER,
        blocked_hash,
        B256::repeat_byte(0x10),
        B256::repeat_byte(0x41),
        B256::repeat_byte(0x42),
        B256::repeat_byte(0x43),
        B256::repeat_byte(0x44),
        B256::repeat_byte(0x45),
        B256::repeat_byte(0x46),
        B256::repeat_byte(0x47),
    )
    .await;
    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        BLOCK_NUMBER,
        ready_hash,
        B256::repeat_byte(0x11),
        B256::repeat_byte(0x51),
        B256::repeat_byte(0x52),
        B256::repeat_byte(0x53),
        B256::repeat_byte(0x54),
        B256::repeat_byte(0x55),
        B256::repeat_byte(0x56),
        B256::repeat_byte(0x57),
    )
    .await;
    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        BLOCK_NUMBER,
        creation_failed_hash,
        B256::repeat_byte(0x12),
        B256::repeat_byte(0x71),
        B256::repeat_byte(0x72),
        B256::repeat_byte(0x73),
        B256::repeat_byte(0x74),
        B256::repeat_byte(0x75),
        B256::repeat_byte(0x76),
        B256::repeat_byte(0x77),
    )
    .await;
    sqlx::query(
        "UPDATE ciphertext_digest_branch
            SET ciphertext128_format = 99
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(creation_failed_hash.as_slice())
    .execute(&pool)
    .await
    .expect("make one sibling fail manifest creation");
    let blocked_child_hash = B256::repeat_byte(0x21);
    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        BLOCK_NUMBER + 1,
        blocked_child_hash,
        blocked_hash,
        B256::repeat_byte(0x61),
        B256::repeat_byte(0x62),
        B256::repeat_byte(0x63),
        B256::repeat_byte(0x64),
        B256::repeat_byte(0x65),
        B256::repeat_byte(0x66),
        B256::repeat_byte(0x67),
    )
    .await;
    seal_seeded_block(&pool, CHAIN_ID, blocked_hash, context).await;

    let localstack = test_harness::localstack::start_localstack()
        .await
        .expect("start LocalStack for competing-lineage publication");
    let client = test_harness::localstack::create_localstack_s3_client(localstack.host_port).await;
    let bucket = "manifest-competing-lineages";
    client
        .create_bucket()
        .bucket(bucket)
        .send()
        .await
        .expect("create competing-lineage manifest bucket");
    let signer: CoproSigner = Arc::new(PrivateKeySigner::random());

    let (blocked_key, conflicting_body) = {
        let mut trx = pool
            .begin()
            .await
            .expect("begin blocked manifest preparation");
        let blocked = load_seeded_block(&pool, CHAIN_ID, blocked_hash).await;
        let prepared = prepare_manifest(&mut trx, &blocked, context, signer.address())
            .await
            .expect("prepare manifest used to derive blocked object key");
        trx.rollback()
            .await
            .expect("rollback blocked manifest preparation");
        let intended = prepared
            .payload
            .clone()
            .sign(signer.as_ref())
            .await
            .expect("sign intended blocked manifest");
        let mut conflicting_payload = prepared.payload;
        conflicting_payload.detailed_range.blocks[0].ciphertexts[0].gateway_key_id = None;
        let conflicting = conflicting_payload
            .sign(signer.as_ref())
            .await
            .expect("sign conflicting immutable manifest");
        conflicting
            .verify()
            .expect("conflicting immutable manifest is valid");
        (
            manifest_object_key(&intended),
            serde_json::to_vec(&conflicting).expect("serialize conflicting manifest"),
        )
    };
    client
        .put_object()
        .bucket(bucket)
        .key(&blocked_key)
        .body(ByteStream::from(conflicting_body))
        .send()
        .await
        .expect("seed conflicting immutable manifest object");

    let consensus = ConsensusConfig::default();
    let mut workers = Vec::with_capacity(WORKER_COUNT);
    for _ in 0..WORKER_COUNT {
        let pool = pool.clone();
        let client = client.clone();
        let signer = Arc::clone(&signer);
        let consensus = consensus.clone();
        workers.push(tokio::spawn(async move {
            let mut outcomes = Vec::new();
            // Lock contention is an expected outcome: all workers may race on
            // the same first candidate. Keep polling long enough for a later
            // wave to reach the ready sibling after the failing siblings.
            for _ in 0..ATTEMPTS_PER_WORKER {
                outcomes.push(
                    progress_chain(&pool, &client, bucket, CHAIN_ID, &signer, &consensus)
                        .await
                        .expect("progress competing manifest lineages"),
                );
            }
            outcomes
        }));
    }
    let mut advanced = 0;
    for worker in workers {
        advanced += worker
            .await
            .expect("join competing-lineage publisher")
            .into_iter()
            .filter(|outcome| *outcome == PublicationProgress::Advanced)
            .count();
    }
    assert!(
        advanced >= 2,
        "one worker must seal and publish the ready fork"
    );

    let states = sqlx::query(
        "SELECT block_hash, manifest_published
           FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_number = $2",
    )
    .bind(CHAIN_ID)
    .bind(BLOCK_NUMBER)
    .fetch_all(&pool)
    .await
    .expect("load competing-lineage publication states");
    assert_eq!(states.len(), 3);
    for state in states {
        let hash = B256::from_slice(&state.get::<Vec<u8>, _>("block_hash"));
        let published = state.get::<bool, _>("manifest_published");
        assert_eq!(published, hash == ready_hash);
    }
    let ready_archive_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM block_manifest
          WHERE publisher = $1 AND host_chain_id = $2
            AND publication_block_hash = $3",
    )
    .bind(signer.address().as_slice())
    .bind(CHAIN_ID)
    .bind(ready_hash.as_slice())
    .fetch_one(&pool)
    .await
    .expect("count ready-lineage manifest archive rows");
    assert_eq!(
        ready_archive_count, 1,
        "only one worker publishes the manifest"
    );
    let blocked_child = load_seeded_block(&pool, CHAIN_ID, blocked_child_hash).await;
    assert!(blocked_child.block_content_digest.is_none());
    assert!(!blocked_child.manifest_published);

    for _ in 0..3 {
        assert_eq!(
            progress_chain(&pool, &client, bucket, CHAIN_ID, &signer, &consensus,)
                .await
                .expect("retry permanently conflicting manifest"),
            PublicationProgress::Waiting,
        );
    }
}

#[path = "publisher_test_support.rs"]
mod support;

use support::*;
