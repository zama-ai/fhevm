use alloy::signers::local::PrivateKeySigner;
use alloy_primitives::{Address, B256, U256};
use block_manifest::{
    block_content_digest, detailed_range_digest, DetailedRange, ManifestBlockEntry,
    ManifestPayload, ManifestVersion, SignedManifest,
};
use fhevm_engine_common::{drift_revert, types::CoproSigner};
use serial_test::serial;
use sqlx::{PgPool, Row};
use std::{sync::Arc, time::Duration};
use test_harness::instance::{setup_test_db, ImportMode};

use super::*;
use crate::manifest_consensus::{
    manifest_archive::{load_manifest_revision, AuthenticatedManifest},
    publication::manifest_builder::{
        is_block_manifest_ready, load_manifest_descriptors, prepare_manifest, seal_block_content,
    },
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
        generation: 0,
        publisher,
        coprocessor_context_id: context,
        host_chain_id: U256::from(7),
        publication_block_number: block_number,
        publication_block_hash: block_hash,
        publication_parent_block_hash: B256::repeat_byte(0xaa),
        revision: 0,
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
                generation: 0,
                block_number,
                block_hash,
                parent_block_hash: B256::repeat_byte(0xaa),
                block_content_digest: content,
                ciphertexts: Vec::new(),
            }],
        },
        historical_ranges: Vec::new(),
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
fn object_key_matches_the_generation_scoped_v1_layout() {
    let manifest = SignedManifest {
        payload: payload(Address::ZERO, U256::ONE),
        signature: Vec::new(),
    };
    assert_eq!(
        manifest_object_key(&manifest),
        format!(
            "manifests/v_1/context_1/chain_7/generation_0/block_42/hash_{}/revision_0",
            "ab".repeat(32)
        ),
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
async fn manifest_s3_timeout_is_a_transient_failure() {
    let err = await_manifest_s3_operation(
        Duration::from_millis(1),
        std::future::pending::<Result<(), ExecutionError>>(),
    )
    .await
    .expect_err("pending S3 operation times out");

    assert!(
        matches!(err, ExecutionError::S3TransientError(message) if message.contains("timed out"))
    );
}

#[test]
fn immutable_retry_rejects_an_oversized_existing_manifest() {
    let bucket = "bucket";
    let key = "key";
    assert!(ensure_manifest_length(Some(MAX_MANIFEST_BYTES as i64), bucket, key).is_ok());
    assert!(
        ensure_manifest_length(Some((MAX_MANIFEST_BYTES + 1) as i64), bucket, key)
            .unwrap_err()
            .to_string()
            .contains("exceeds")
    );
    assert!(ensure_manifest_size(MAX_MANIFEST_BYTES + 1, bucket, key)
        .unwrap_err()
        .to_string()
        .contains("exceeds"));
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
        "UPDATE ciphertext_digest
            SET ciphertext128 = NULL
          WHERE host_chain_id = $1 AND handle = $2",
    )
    .bind(CHAIN_ID)
    .bind(B256::repeat_byte(0x51).as_slice())
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
        "UPDATE ciphertext_digest
            SET ciphertext128 = $3
          WHERE host_chain_id = $1 AND handle = $2",
    )
    .bind(CHAIN_ID)
    .bind(B256::repeat_byte(0x51).as_slice())
    .bind(B256::repeat_byte(0x57).as_slice())
    .execute(&pool)
    .await
    .expect("restore manifest digest");
    sqlx::query(
        "UPDATE ciphertext_digest
            SET ciphertext128_format = 0
          WHERE host_chain_id = $1 AND handle = $2",
    )
    .bind(CHAIN_ID)
    .bind(B256::repeat_byte(0x51).as_slice())
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
        "UPDATE ciphertext_digest
            SET ciphertext128_format = 11
          WHERE host_chain_id = $1 AND handle = $2",
    )
    .bind(CHAIN_ID)
    .bind(B256::repeat_byte(0x51).as_slice())
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
async fn first_manifest_of_a_generation_starts_a_new_history_lineage() {
    const CHAIN_ID: i64 = 137;
    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create generation-boundary database");
    let pool = PgPool::connect(instance.db_url())
        .await
        .expect("connect generation-boundary database");
    let previous_hash = B256::repeat_byte(0x70);
    let current_hash = B256::repeat_byte(0x71);
    let next_hash = B256::repeat_byte(0x72);

    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        70,
        previous_hash,
        B256::repeat_byte(0x6f),
        B256::repeat_byte(0x10),
        B256::repeat_byte(0x11),
        B256::repeat_byte(0x12),
        B256::repeat_byte(0x13),
        B256::repeat_byte(0x14),
        B256::repeat_byte(0x15),
        B256::repeat_byte(0x16),
    )
    .await;
    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        71,
        current_hash,
        previous_hash,
        B256::repeat_byte(0x20),
        B256::repeat_byte(0x21),
        B256::repeat_byte(0x22),
        B256::repeat_byte(0x23),
        B256::repeat_byte(0x24),
        B256::repeat_byte(0x25),
        B256::repeat_byte(0x26),
    )
    .await;
    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        72,
        next_hash,
        current_hash,
        B256::repeat_byte(0x30),
        B256::repeat_byte(0x31),
        B256::repeat_byte(0x32),
        B256::repeat_byte(0x33),
        B256::repeat_byte(0x34),
        B256::repeat_byte(0x35),
        B256::repeat_byte(0x36),
    )
    .await;
    sqlx::query("UPDATE block_manifest_state SET generation = 1 WHERE block_hash IN ($1, $2)")
        .bind(current_hash.as_slice())
        .bind(next_hash.as_slice())
        .execute(&pool)
        .await
        .expect("assign current-generation blocks");

    seal_seeded_block(&pool, CHAIN_ID, previous_hash, U256::ONE).await;
    seal_seeded_block(&pool, CHAIN_ID, current_hash, U256::ONE).await;

    let target = load_seeded_block(&pool, CHAIN_ID, current_hash).await;
    let signer = PrivateKeySigner::random();
    let mut trx = pool.begin().await.expect("begin manifest preparation");
    let prepared = prepare_manifest(&mut trx, &target, U256::ONE, signer.address())
        .await
        .expect("prepare first manifest of the new generation");
    trx.rollback().await.expect("rollback manifest preparation");

    assert_eq!(prepared.payload.generation, 1);
    assert_eq!(prepared.payload.detailed_range.blocks.len(), 1);
    assert_eq!(prepared.payload.detailed_range.blocks[0].generation, 1);
    assert_eq!(
        prepared.payload.detailed_range.blocks[0].block_hash,
        current_hash
    );
    assert!(prepared.payload.historical_ranges.is_empty());

    let current_manifest_digest = B256::repeat_byte(0xd1);
    let current_range_digest = B256::repeat_byte(0xd2);
    sqlx::query(
        "UPDATE block_manifest_state
            SET manifest_range_start = block_number,
                manifest_range_digest = $2,
                manifest_publisher = $3,
                manifest_digest = $4,
                manifest_published = TRUE,
                manifest_published_at = NOW()
          WHERE generation = 1 AND block_hash = $1",
    )
    .bind(current_hash.as_slice())
    .bind(current_range_digest.as_slice())
    .bind(signer.address().as_slice())
    .bind(current_manifest_digest.as_slice())
    .execute(&pool)
    .await
    .expect("mark the first current-generation manifest published");

    let next = load_seeded_block(&pool, CHAIN_ID, next_hash).await;
    let mut trx = pool.begin().await.expect("begin later lineage load");
    let (lineage, last_published_manifest) =
        crate::manifest_consensus::publication::manifest_history::load_detailed_lineage(
            &mut trx, &next,
        )
        .await
        .expect("load later current-generation lineage");
    trx.rollback().await.expect("rollback later lineage load");
    assert_eq!(lineage.len(), 1);
    assert_eq!(lineage[0].generation, 1);
    let last_published_manifest = last_published_manifest.expect("find last local publication");
    assert_eq!(last_published_manifest.generation, 1);
    assert_eq!(last_published_manifest.block_hash, current_hash);
    assert_eq!(
        last_published_manifest.manifest_digest,
        current_manifest_digest
    );
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
            consensus: ManifestConsensusConfig::default(),
        },
        Duration::from_millis(10),
        ManifestWorkGate::always_enabled(),
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

#[tokio::test]
#[serial(db)]
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
    .bind("manifests/v_1/context_1/chain_100/generation_0/block_10/hash_evidence/revision_0")
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
        "UPDATE ciphertext_digest
            SET ciphertext128_format = 0
          WHERE host_chain_id = $1 AND handle = $2",
    )
    .bind(CHAIN_ID)
    .bind(B256::repeat_byte(0x71).as_slice())
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

    let consensus = ManifestConsensusConfig::default();
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
                    progress_chain(
                        &pool,
                        &client,
                        bucket,
                        CHAIN_ID,
                        &signer,
                        &consensus,
                        &ManifestWorkGate::always_enabled(),
                        0,
                    )
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
            progress_chain(
                &pool,
                &client,
                bucket,
                CHAIN_ID,
                &signer,
                &consensus,
                &ManifestWorkGate::always_enabled(),
                0,
            )
            .await
            .expect("retry permanently conflicting manifest"),
            PublicationProgress::Waiting,
        );
    }
}

#[path = "publisher_test_support.rs"]
mod support;

use support::*;
