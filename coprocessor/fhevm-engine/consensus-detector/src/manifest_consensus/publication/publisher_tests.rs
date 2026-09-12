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
        is_block_manifest_ready, load_manifest_descriptors, missing_handles_are_uncomputed,
        prepare_manifest, seal_block_content,
    },
    Config as ManifestConsensusConfig, ExecutionError,
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
        consensus_epoch: block_manifest::LEGACY_CONSENSUS_EPOCH.to_owned(),
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
            "manifests/v_1/context_1/chain_7/block_42/hash_{}/consensus_epoch/{}/revision/0",
            "ab".repeat(32),
            block_manifest::LEGACY_CONSENSUS_EPOCH,
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
async fn readiness_includes_errored_handles_without_waiting_for_digests() {
    const CHAIN_ID: i64 = 137;
    let block_hash = B256::repeat_byte(0x42);
    let good_handle = B256::repeat_byte(0x51);
    let error_handle = B256::repeat_byte(0x61);
    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create errored-handle database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect(instance.db_url())
        .await
        .expect("connect errored-handle database");
    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        42,
        block_hash,
        B256::repeat_byte(0x41),
        good_handle,
        B256::repeat_byte(0x52),
        B256::repeat_byte(0x53),
        B256::repeat_byte(0x54),
        B256::repeat_byte(0x55),
        B256::repeat_byte(0x56),
        B256::repeat_byte(0x57),
    )
    .await;
    sqlx::query(
        "INSERT INTO handle_producer_block (
             host_chain_id, producer_block_number, producer_block_hash, handle
         ) VALUES ($1, $2, $3, $4)",
    )
    .bind(CHAIN_ID)
    .bind(42_i64)
    .bind(block_hash.as_slice())
    .bind(error_handle.as_slice())
    .execute(&pool)
    .await
    .expect("insert errored producer handle");

    let block = load_seeded_block(&pool, CHAIN_ID, block_hash).await;
    let mut trx = pool.begin().await.expect("begin missing-digest check");
    assert!(!is_block_manifest_ready(&mut trx, &block)
        .await
        .expect("a handle without a digest or error still waits"));
    trx.rollback().await.expect("rollback missing-digest check");

    sqlx::query(
        "INSERT INTO computations (
             output_handle, dependencies, fhe_operation, is_scalar,
             transaction_id, host_chain_id, block_number, is_error, is_completed,
             error_message
         ) VALUES ($1, $2, $3, FALSE, $4, $5, $6, TRUE, FALSE, $7)",
    )
    .bind(error_handle.as_slice())
    .bind(Vec::<Vec<u8>>::new())
    .bind(1_i16)
    .bind(error_handle.as_slice())
    .bind(CHAIN_ID)
    .bind(42_i64)
    .bind("Unknown fhe operation")
    .execute(&pool)
    .await
    .expect("mark handle computation as error");

    let mut trx = pool.begin().await.expect("begin errored-handle check");
    assert!(is_block_manifest_ready(&mut trx, &block)
        .await
        .expect("an errored handle must not keep the block unready"));
    let descriptors = load_manifest_descriptors(&mut trx, &block, false)
        .await
        .expect("load descriptors including the errored handle");
    assert_eq!(descriptors.len(), 2);
    assert_eq!(descriptors[0].handle, good_handle);
    assert!(!descriptors[0].is_error());
    assert_eq!(descriptors[1].handle, error_handle);
    assert!(descriptors[1].is_error());
    assert_eq!(
        descriptors[1].error_message(),
        Some("Unknown fhe operation")
    );
    assert!(descriptors[0].error_message().is_none());
    trx.rollback().await.expect("rollback errored-handle check");
}

#[tokio::test]
#[serial(db)]
async fn readiness_seals_missing_handles_as_uncomputed_after_lag_and_timeout() {
    const CHAIN_ID: i64 = 137;
    const BLOCK_NUMBER: i64 = 42;
    const CADENCE: i64 = 5;
    const MAX_LAG_MANIFESTS: u32 = 2;
    const TIMEOUT: Duration = Duration::from_secs(60);
    let policy = ManifestConsensusConfig {
        incomplete_block_timeout: TIMEOUT,
        incomplete_manifest_max_lag: MAX_LAG_MANIFESTS,
        ..ManifestConsensusConfig::default()
    };
    let block_hash = B256::repeat_byte(0x42);
    let good_handle = B256::repeat_byte(0x51);
    let missing_handle = B256::repeat_byte(0x71);
    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create uncomputed-handle database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect(instance.db_url())
        .await
        .expect("connect uncomputed-handle database");
    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        BLOCK_NUMBER,
        block_hash,
        B256::repeat_byte(0x41),
        good_handle,
        B256::repeat_byte(0x52),
        B256::repeat_byte(0x53),
        B256::repeat_byte(0x54),
        B256::repeat_byte(0x55),
        B256::repeat_byte(0x56),
        B256::repeat_byte(0x57),
    )
    .await;
    sqlx::query(
        "UPDATE block_manifest_state
            SET publication_cadence = $3
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(block_hash.as_slice())
    .bind(CADENCE)
    .execute(&pool)
    .await
    .expect("set publication cadence");
    sqlx::query(
        "INSERT INTO handle_producer_block (
             host_chain_id, producer_block_number, producer_block_hash, handle
         ) VALUES ($1, $2, $3, $4)",
    )
    .bind(CHAIN_ID)
    .bind(BLOCK_NUMBER)
    .bind(block_hash.as_slice())
    .bind(missing_handle.as_slice())
    .execute(&pool)
    .await
    .expect("insert missing producer handle");
    sqlx::query(
        "INSERT INTO host_chain_blocks_valid
             (chain_id, block_hash, parent_hash, block_number, block_status)
         VALUES ($1, $2, $3, $4, 'pending')",
    )
    .bind(CHAIN_ID)
    .bind(block_hash.as_slice())
    .bind(B256::repeat_byte(0x41).as_slice())
    .bind(BLOCK_NUMBER)
    .execute(&pool)
    .await
    .expect("insert producer host block");

    let lag = policy
        .incomplete_seal_lag_blocks(CADENCE)
        .expect("cadence and lag are positive");
    let stall_secs = policy
        .incomplete_block_timeout_secs()
        .expect("timeout is positive");
    assert_eq!(lag, CADENCE * i64::from(MAX_LAG_MANIFESTS));
    assert_eq!(stall_secs, TIMEOUT.as_secs() as i64);
    let block = load_seeded_block(&pool, CHAIN_ID, block_hash).await;
    let mut trx = pool.begin().await.expect("begin missing-handle wait");
    assert!(!is_block_manifest_ready(&mut trx, &block)
        .await
        .expect("missing digest without lag is not ready"));
    assert!(!missing_handles_are_uncomputed(&mut trx, &block, &policy)
        .await
        .expect("manifest lag has not elapsed"));
    trx.rollback().await.expect("rollback missing-handle wait");

    sqlx::query(
        "INSERT INTO host_chain_blocks_valid
             (chain_id, block_hash, parent_hash, block_number, block_status)
         VALUES ($1, $2, $3, $4, 'pending')",
    )
    .bind(CHAIN_ID)
    .bind(B256::repeat_byte(0x68).as_slice())
    .bind(block_hash.as_slice())
    .bind(BLOCK_NUMBER + lag)
    .execute(&pool)
    .await
    .expect("insert host tip at exactly the configured lag");
    let mut trx = pool.begin().await.expect("begin exact-lag check");
    assert!(!missing_handles_are_uncomputed(&mut trx, &block, &policy)
        .await
        .expect("exactly max-lag manifests is not more than max-lag"));
    trx.rollback().await.expect("rollback exact-lag check");

    sqlx::query(
        "INSERT INTO host_chain_blocks_valid
             (chain_id, block_hash, parent_hash, block_number, block_status)
         VALUES ($1, $2, $3, $4, 'pending')",
    )
    .bind(CHAIN_ID)
    .bind(B256::repeat_byte(0x69).as_slice())
    .bind(B256::repeat_byte(0x68).as_slice())
    .bind(BLOCK_NUMBER + lag + 1)
    .execute(&pool)
    .await
    .expect("insert host tip past the configured lag");
    let mut trx = pool.begin().await.expect("begin recent-progress check");
    assert!(!missing_handles_are_uncomputed(&mut trx, &block, &policy)
        .await
        .expect("a recently computed handle is still progress"));
    trx.rollback()
        .await
        .expect("rollback recent-progress check");

    sqlx::query(
        "UPDATE ciphertext_digest
            SET created_at = NOW() - ($3::BIGINT * INTERVAL '1 second')
          WHERE host_chain_id = $1 AND handle = $2",
    )
    .bind(CHAIN_ID)
    .bind(good_handle.as_slice())
    .bind(stall_secs + 1)
    .execute(&pool)
    .await
    .expect("age the last computed digest past the stall");
    sqlx::query(
        "UPDATE handle_producer_block
            SET created_at = NOW() - ($4::BIGINT * INTERVAL '1 second')
          WHERE host_chain_id = $1
            AND producer_block_number = $2
            AND producer_block_hash = $3",
    )
    .bind(CHAIN_ID)
    .bind(BLOCK_NUMBER)
    .bind(block_hash.as_slice())
    .bind(stall_secs + 1)
    .execute(&pool)
    .await
    .expect("age producer inventory past the stall");

    let mut trx = pool.begin().await.expect("begin uncomputed seal check");
    assert!(!is_block_manifest_ready(&mut trx, &block)
        .await
        .expect("missing digest is still not fully computed"));
    assert!(missing_handles_are_uncomputed(&mut trx, &block, &policy)
        .await
        .expect("lag and timeout both elapsed with no computed handle"));
    let descriptors = load_manifest_descriptors(&mut trx, &block, true)
        .await
        .expect("load descriptors with uncomputed missing handle");
    assert_eq!(descriptors.len(), 2);
    assert_eq!(descriptors[0].handle, good_handle);
    assert!(!descriptors[0].is_uncomputed());
    assert!(!descriptors[0].is_error());
    assert_eq!(descriptors[1].handle, missing_handle);
    assert!(descriptors[1].is_uncomputed());
    assert!(!descriptors[1].is_error());
    trx.rollback()
        .await
        .expect("rollback uncomputed seal check");
}

#[tokio::test]
#[serial(db)]
async fn late_ciphertext_discards_unpublished_uncomputed_seal() {
    const CHAIN_ID: i64 = 137;
    const BLOCK_NUMBER: i64 = 42;
    let block_hash = B256::repeat_byte(0x42);
    let good_handle = B256::repeat_byte(0x51);
    let missing_handle = B256::repeat_byte(0x71);
    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create late-ciphertext database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect(instance.db_url())
        .await
        .expect("connect late-ciphertext database");
    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        BLOCK_NUMBER,
        block_hash,
        B256::repeat_byte(0x41),
        good_handle,
        B256::repeat_byte(0x52),
        B256::repeat_byte(0x53),
        B256::repeat_byte(0x54),
        B256::repeat_byte(0x55),
        B256::repeat_byte(0x56),
        B256::repeat_byte(0x57),
    )
    .await;
    sqlx::query(
        "INSERT INTO handle_producer_block (
             host_chain_id, producer_block_number, producer_block_hash, handle
         ) VALUES ($1, $2, $3, $4)",
    )
    .bind(CHAIN_ID)
    .bind(BLOCK_NUMBER)
    .bind(block_hash.as_slice())
    .bind(missing_handle.as_slice())
    .execute(&pool)
    .await
    .expect("insert missing producer handle");

    let block = load_seeded_block(&pool, CHAIN_ID, block_hash).await;
    let mut trx = pool.begin().await.expect("begin uncomputed seal");
    let descriptors = load_manifest_descriptors(&mut trx, &block, true)
        .await
        .expect("load uncomputed descriptors");
    assert!(descriptors
        .iter()
        .any(|descriptor| descriptor.is_uncomputed()));
    seal_block_content(&mut trx, &block, U256::ONE, &descriptors)
        .await
        .expect("seal uncomputed block");
    trx.commit().await.expect("commit uncomputed seal");

    sqlx::query(
        "INSERT INTO ciphertext_digest (
             host_chain_id, key_id_gw, handle, ciphertext, ciphertext128,
             ciphertext128_format
         ) VALUES ($1, $2, $3, $4, $5, 11)",
    )
    .bind(CHAIN_ID)
    .bind(B256::repeat_byte(0x54).as_slice())
    .bind(missing_handle.as_slice())
    .bind(B256::repeat_byte(0x76).as_slice())
    .bind(B256::repeat_byte(0x77).as_slice())
    .execute(&pool)
    .await
    .expect("late ciphertext arrives after seal");

    let sealed = load_seeded_block(&pool, CHAIN_ID, block_hash).await;
    assert!(sealed.block_content_digest.is_some());
    let signer = PrivateKeySigner::random();
    let mut trx = pool.begin().await.expect("begin stale-seal preparation");
    let err = prepare_manifest(&mut trx, &sealed, U256::ONE, signer.address())
        .await
        .expect_err("live ciphertext must discard the uncomputed seal");
    assert!(
        matches!(err, ExecutionError::StaleBlockSeal { .. }),
        "unexpected error: {err}"
    );
    trx.commit().await.expect("commit discarded seal");

    let unsealed = load_seeded_block(&pool, CHAIN_ID, block_hash).await;
    assert!(unsealed.block_content_digest.is_none());
    assert!(unsealed.block_handle_count.is_none());
    let error_count: i64 = sqlx::query_scalar(
        "SELECT publication_error_count FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(block_hash.as_slice())
    .fetch_one(&pool)
    .await
    .expect("load publication error count");
    assert_eq!(error_count, 0);

    let mut trx = pool
        .begin()
        .await
        .expect("begin reseal after late ciphertext");
    assert!(is_block_manifest_ready(&mut trx, &unsealed)
        .await
        .expect("late ciphertext makes the block ready"));
    let descriptors = load_manifest_descriptors(&mut trx, &unsealed, false)
        .await
        .expect("load complete descriptors");
    assert!(descriptors
        .iter()
        .all(|descriptor| !descriptor.is_uncomputed()));
    let digest = seal_block_content(&mut trx, &unsealed, U256::ONE, &descriptors)
        .await
        .expect("reseal with live ciphertext");
    let mut resealed = unsealed;
    resealed.block_content_digest = Some(digest.as_slice().to_vec());
    resealed.block_handle_count =
        Some(i64::try_from(descriptors.len()).expect("descriptor count fits BIGINT"));
    prepare_manifest(&mut trx, &resealed, U256::ONE, signer.address())
        .await
        .expect("publish from the resealed digest");
    trx.commit().await.expect("commit reseal");
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
    let err = load_manifest_descriptors(&mut trx, &block, false)
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
    assert!(
        matches!(err, ExecutionError::StaleBlockSeal { .. }),
        "unexpected error: {err}"
    );
    trx.commit().await.expect("commit discarded corrupted seal");
    let unsealed = load_seeded_block(&pool, CHAIN_ID, block_hash).await;
    assert!(unsealed.block_content_digest.is_none());
    assert!(unsealed.block_handle_count.is_none());

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
    sqlx::query("UPDATE block_manifest_state SET generation = '1' WHERE block_hash IN ($1, $2)")
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

    assert_eq!(prepared.payload.consensus_epoch, "1");
    assert_eq!(prepared.payload.detailed_range.blocks.len(), 1);
    assert_eq!(
        prepared.payload.detailed_range.blocks[0].block_hash,
        current_hash
    );
    assert!(prepared.payload.historical_ranges.is_empty());

    let current_manifest_digest = B256::repeat_byte(0xd1);
    sqlx::query(
        "UPDATE block_manifest_state
            SET manifest_publisher = $2,
                manifest_digest = $3,
                manifest_published = TRUE,
                manifest_published_at = NOW()
          WHERE generation = '1' AND block_hash = $1",
    )
    .bind(current_hash.as_slice())
    .bind(signer.address().as_slice())
    .bind(current_manifest_digest.as_slice())
    .execute(&pool)
    .await
    .expect("mark the first current-generation manifest published");

    let next = load_seeded_block(&pool, CHAIN_ID, next_hash).await;
    let mut trx = pool.begin().await.expect("begin later lineage load");
    let (lineage, last_published_manifest) =
        crate::manifest_consensus::publication::manifest_history::load_detailed_lineage(
            &mut trx,
            &next,
            U256::ONE,
        )
        .await
        .expect("load later current-generation lineage");
    trx.rollback().await.expect("rollback later lineage load");
    assert_eq!(lineage.len(), 1);
    assert_eq!(lineage[0].generation, "1");
    let last_published_manifest = last_published_manifest.expect("find last local publication");
    assert_eq!(last_published_manifest.generation, "1");
    assert_eq!(last_published_manifest.block_hash, current_hash);
    assert_eq!(
        last_published_manifest.manifest_digest,
        current_manifest_digest
    );
}

#[tokio::test]
#[serial(db)]
async fn missing_in_generation_parent_is_materialized_as_empty() {
    const CHAIN_ID: i64 = 137;
    let start_hash = B256::repeat_byte(0x60);
    let missing_hash = B256::repeat_byte(0x61);
    let target_hash = B256::repeat_byte(0x62);
    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create empty-gap lineage database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect(instance.db_url())
        .await
        .expect("connect empty-gap lineage database");

    sqlx::query(
        "INSERT INTO generation_history (
             generation, proposal_id, proposal_block, stack_version, outcome
         ) VALUES ('1', $1, 50, 'test-green', 'pending')",
    )
    .bind(vec![0x91_u8; 32])
    .execute(&pool)
    .await
    .expect("allocate generation");
    sqlx::query(
        "INSERT INTO generation_block_window (
             generation, host_chain_id, start_block, consensus_deadline_block
         ) VALUES ('1', $1, 60, 70)",
    )
    .bind(CHAIN_ID)
    .execute(&pool)
    .await
    .expect("store generation window");
    sqlx::query(
        "UPDATE blue_green_generation SET generation = '1', updated_at = NOW()
          WHERE singleton = TRUE",
    )
    .execute(&pool)
    .await
    .expect("select generation");

    sqlx::query(
        "INSERT INTO host_chain_blocks_valid
             (chain_id, block_hash, parent_hash, block_number, block_status)
         VALUES ($1, $2, $3, 60, 'pending'), ($1, $4, $2, 61, 'pending')",
    )
    .bind(CHAIN_ID)
    .bind(start_hash.as_slice())
    .bind(B256::repeat_byte(0x59).as_slice())
    .bind(missing_hash.as_slice())
    .execute(&pool)
    .await
    .expect("insert host start and empty parent");

    sqlx::query(
        "INSERT INTO block_manifest_state (
             generation, host_chain_id, block_number, block_hash, parent_block_hash,
             publication_cadence
         ) VALUES ('1', $1, 60, $2, $3, 1)",
    )
    .bind(CHAIN_ID)
    .bind(start_hash.as_slice())
    .bind(B256::repeat_byte(0x59).as_slice())
    .execute(&pool)
    .await
    .expect("track generation start");
    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        62,
        target_hash,
        missing_hash,
        B256::repeat_byte(0x51),
        B256::repeat_byte(0x52),
        B256::repeat_byte(0x53),
        B256::repeat_byte(0x54),
        B256::repeat_byte(0x55),
        B256::repeat_byte(0x56),
        B256::repeat_byte(0x57),
    )
    .await;
    sqlx::query("UPDATE block_manifest_state SET generation = '1' WHERE block_hash = $1")
        .bind(target_hash.as_slice())
        .execute(&pool)
        .await
        .expect("assign target generation");

    let mut trx = pool.begin().await.expect("begin start seal");
    let start = load_seeded_block(&pool, CHAIN_ID, start_hash).await;
    seal_block_content(&mut trx, &start, U256::ONE, &[])
        .await
        .expect("seal empty generation start");
    trx.commit().await.expect("commit start seal");
    seal_seeded_block(&pool, CHAIN_ID, target_hash, U256::ONE).await;

    let target = load_seeded_block(&pool, CHAIN_ID, target_hash).await;
    let signer = PrivateKeySigner::random();
    let mut trx = pool.begin().await.expect("begin gap-filling prepare");
    let prepared = prepare_manifest(&mut trx, &target, U256::ONE, signer.address())
        .await
        .expect("prepare manifest across an empty host gap");
    trx.commit().await.expect("commit gap-filling prepare");

    assert_eq!(prepared.payload.detailed_range.blocks.len(), 3);
    assert_eq!(
        prepared.payload.detailed_range.blocks[0].block_hash,
        start_hash
    );
    assert!(prepared.payload.detailed_range.blocks[0]
        .ciphertexts
        .is_empty());
    assert_eq!(
        prepared.payload.detailed_range.blocks[1].block_hash,
        missing_hash
    );
    assert!(prepared.payload.detailed_range.blocks[1]
        .ciphertexts
        .is_empty());
    assert_eq!(
        prepared.payload.detailed_range.blocks[2].block_hash,
        target_hash
    );
    let filled = load_seeded_block(&pool, CHAIN_ID, missing_hash).await;
    assert_eq!(filled.block_handle_count, Some(0));
    assert!(filled.block_content_digest.is_some());
}

#[tokio::test]
#[serial(db)]
async fn missing_in_generation_parent_with_producers_is_inserted_unsealed() {
    const CHAIN_ID: i64 = 137;
    let start_hash = B256::repeat_byte(0x60);
    let missing_hash = B256::repeat_byte(0x61);
    let target_hash = B256::repeat_byte(0x62);
    let parent_handle = B256::repeat_byte(0x71);
    let parent_gateway_key = B256::repeat_byte(0x72);
    let parent_keyset = B256::repeat_byte(0x73);
    let parent_ct64 = B256::repeat_byte(0x74);
    let parent_ct128 = B256::repeat_byte(0x75);
    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create producer-gap lineage database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect(instance.db_url())
        .await
        .expect("connect producer-gap lineage database");

    sqlx::query(
        "INSERT INTO generation_history (
             generation, proposal_id, proposal_block, stack_version, outcome
         ) VALUES ('1', $1, 50, 'test-green', 'pending')",
    )
    .bind(vec![0x91_u8; 32])
    .execute(&pool)
    .await
    .expect("allocate generation");
    sqlx::query(
        "INSERT INTO generation_block_window (
             generation, host_chain_id, start_block, consensus_deadline_block
         ) VALUES ('1', $1, 60, 70)",
    )
    .bind(CHAIN_ID)
    .execute(&pool)
    .await
    .expect("store generation window");
    sqlx::query(
        "UPDATE blue_green_generation SET generation = '1', updated_at = NOW()
          WHERE singleton = TRUE",
    )
    .execute(&pool)
    .await
    .expect("select generation");

    sqlx::query(
        "INSERT INTO host_chain_blocks_valid
             (chain_id, block_hash, parent_hash, block_number, block_status)
         VALUES ($1, $2, $3, 60, 'pending'), ($1, $4, $2, 61, 'pending')",
    )
    .bind(CHAIN_ID)
    .bind(start_hash.as_slice())
    .bind(B256::repeat_byte(0x59).as_slice())
    .bind(missing_hash.as_slice())
    .execute(&pool)
    .await
    .expect("insert host start and missing parent");

    sqlx::query(
        "INSERT INTO block_manifest_state (
             generation, host_chain_id, block_number, block_hash, parent_block_hash,
             publication_cadence
         ) VALUES ('1', $1, 60, $2, $3, 1)",
    )
    .bind(CHAIN_ID)
    .bind(start_hash.as_slice())
    .bind(B256::repeat_byte(0x59).as_slice())
    .execute(&pool)
    .await
    .expect("track generation start");
    sqlx::query(
        "INSERT INTO handle_producer_block (
             host_chain_id, producer_block_number, producer_block_hash, handle
         ) VALUES ($1, 61, $2, $3)",
    )
    .bind(CHAIN_ID)
    .bind(missing_hash.as_slice())
    .bind(parent_handle.as_slice())
    .execute(&pool)
    .await
    .expect("insert missing-parent producer inventory");
    sqlx::query(
        "INSERT INTO keys (key_id_gw, key_id, pks_key, sks_key, chain_id, block_hash)
         VALUES ($1, $2, ''::BYTEA, ''::BYTEA, $3, $4)",
    )
    .bind(parent_gateway_key.as_slice())
    .bind(parent_keyset.as_slice())
    .bind(CHAIN_ID)
    .bind(missing_hash.as_slice())
    .execute(&pool)
    .await
    .expect("insert missing-parent keyset");
    sqlx::query(
        "INSERT INTO ciphertext_digest (
             host_chain_id, key_id_gw, handle, ciphertext, ciphertext128,
             ciphertext128_format
         ) VALUES ($1, $2, $3, $4, $5, 11)",
    )
    .bind(CHAIN_ID)
    .bind(parent_gateway_key.as_slice())
    .bind(parent_handle.as_slice())
    .bind(parent_ct64.as_slice())
    .bind(parent_ct128.as_slice())
    .execute(&pool)
    .await
    .expect("insert missing-parent ciphertext digests");
    seed_revision_publication_block(
        &pool,
        CHAIN_ID,
        62,
        target_hash,
        missing_hash,
        B256::repeat_byte(0x51),
        B256::repeat_byte(0x52),
        B256::repeat_byte(0x53),
        B256::repeat_byte(0x54),
        B256::repeat_byte(0x55),
        B256::repeat_byte(0x56),
        B256::repeat_byte(0x57),
    )
    .await;
    sqlx::query("UPDATE block_manifest_state SET generation = '1' WHERE block_hash = $1")
        .bind(target_hash.as_slice())
        .execute(&pool)
        .await
        .expect("assign target generation");

    let mut trx = pool.begin().await.expect("begin start seal");
    let start = load_seeded_block(&pool, CHAIN_ID, start_hash).await;
    seal_block_content(&mut trx, &start, U256::ONE, &[])
        .await
        .expect("seal empty generation start");
    trx.commit().await.expect("commit start seal");
    seal_seeded_block(&pool, CHAIN_ID, target_hash, U256::ONE).await;

    let target = load_seeded_block(&pool, CHAIN_ID, target_hash).await;
    let child_digest = target
        .block_content_digest
        .clone()
        .expect("child is already sealed");
    let signer = PrivateKeySigner::random();
    let mut trx = pool.begin().await.expect("begin unsealed-parent prepare");
    let error = prepare_manifest(&mut trx, &target, U256::ONE, signer.address())
        .await
        .expect_err("producer inventory must not empty-seal the parent");
    assert!(
        matches!(
            error,
            ExecutionError::PredecessorUnsealed {
                host_chain_id: CHAIN_ID,
                block_number: 61,
            }
        ),
        "{error:?}"
    );
    trx.commit()
        .await
        .expect("commit unsealed parent like a Waiting publication tick");

    let parent = load_seeded_block(&pool, CHAIN_ID, missing_hash).await;
    assert!(parent.block_content_digest.is_none());
    assert!(parent.block_handle_count.is_none());
    assert!(!parent.manifest_published);
    let child = load_seeded_block(&pool, CHAIN_ID, target_hash).await;
    assert_eq!(
        child.block_content_digest.as_deref(),
        Some(child_digest.as_slice())
    );

    let mut trx = pool.begin().await.expect("begin second unsealed prepare");
    let error = prepare_manifest(&mut trx, &child, U256::ONE, signer.address())
        .await
        .expect_err("an already unsealed parent still blocks prepare");
    assert!(
        matches!(
            error,
            ExecutionError::PredecessorUnsealed {
                host_chain_id: CHAIN_ID,
                block_number: 61,
            }
        ),
        "{error:?}"
    );
    trx.rollback()
        .await
        .expect("rollback second unsealed prepare");

    seal_seeded_block(&pool, CHAIN_ID, missing_hash, U256::ONE).await;
    let target = load_seeded_block(&pool, CHAIN_ID, target_hash).await;
    let mut trx = pool.begin().await.expect("begin prepare after parent seal");
    let prepared = prepare_manifest(&mut trx, &target, U256::ONE, signer.address())
        .await
        .expect("prepare after the producer parent is sealed");
    trx.commit()
        .await
        .expect("commit prepare after parent seal");
    assert_eq!(prepared.payload.detailed_range.blocks.len(), 3);
    assert_eq!(
        prepared.payload.detailed_range.blocks[1].block_hash,
        missing_hash
    );
    assert_eq!(
        prepared.payload.detailed_range.blocks[1].ciphertexts.len(),
        1
    );
    assert_eq!(
        prepared.payload.detailed_range.blocks[1].ciphertexts[0].handle,
        parent_handle
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
    .bind("manifests/v_1/context_1/chain_100/block_10/hash_evidence/consensus_epoch/legacy/revision/0")
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
        if let block_manifest::CiphertextStatus::Computed { gateway_key_id, .. } =
            &mut conflicting_payload.detailed_range.blocks[0].ciphertexts[0].status
        {
            *gateway_key_id = None;
        }
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
                        block_manifest::LEGACY_CONSENSUS_EPOCH,
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
                block_manifest::LEGACY_CONSENSUS_EPOCH,
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
