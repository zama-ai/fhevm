use std::{sync::Arc, time::Duration};

use aws_sdk_s3::{error::ProvideErrorMetadata, primitives::ByteStream, Client};
use ciphertext_attestation::manifest::SignedManifest;
use fhevm_engine_common::{
    pg_pool::{PostgresPoolManager, ServiceError},
    types::CoproSigner,
    versioning::StackMode,
};
use sqlx::PgPool;
use tokio::{task::JoinHandle, time::MissedTickBehavior};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::{aws_upload::COPROCESSOR_CONTEXT_ID_1, Config, ConsensusConfig, ExecutionError};

use super::manifest::{
    discover_block_children, discover_known_children, is_block_manifest_ready,
    load_manifest_descriptors, lock_next_block_to_progress, mark_manifest_published,
    pending_chain_ids, prepare_manifest, seal_block_content, ManifestProgressCursor, PendingBlock,
};
use super::manifest_archive::{manifest_object_key, store_authenticated_manifest};
use super::peer_downloader::schedule_manifest_verification;

const MANIFEST_POLL_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Progress {
    Advanced,
    Waiting,
    Busy,
}

pub(crate) async fn spawn_manifest_publisher(
    pool_mngr: &PostgresPoolManager,
    conf: Config,
    client: Arc<Client>,
    signer: CoproSigner,
    stack_mode: Arc<StackMode>,
) -> Result<JoinHandle<Result<(), ServiceError>>, ExecutionError> {
    let op = move |pool, token| {
        let conf = conf.clone();
        let client = Arc::clone(&client);
        let signer = Arc::clone(&signer);
        let stack_mode = Arc::clone(&stack_mode);
        async move {
            run_manifest_publisher(pool, token, conf, client, signer, stack_mode)
                .await
                .map_err(ServiceError::from)
        }
    };

    Ok(pool_mngr
        .spawn_with_db_retry(op, "consensus_manifest_publisher")
        .await)
}

async fn run_manifest_publisher(
    pool: PgPool,
    token: CancellationToken,
    conf: Config,
    client: Arc<Client>,
    signer: CoproSigner,
    stack_mode: Arc<StackMode>,
) -> Result<(), ExecutionError> {
    while stack_mode.gcs_mode() {
        tokio::select! {
            _ = token.cancelled() => return Ok(()),
            _ = tokio::time::sleep(Duration::from_secs(2)) => {}
        }
    }
    info!("Manifest publication enabled on the live stack");

    let cadence = i64::try_from(conf.consensus.publication_cadence).map_err(|_| {
        ExecutionError::InternalError("manifest publication cadence exceeds BIGINT".into())
    })?;
    let mut ticker = tokio::time::interval(MANIFEST_POLL_INTERVAL);
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        if stack_mode.is_paused() {
            info!("Retired stack stopped consensus manifest publication");
            return Ok(());
        }
        tokio::select! {
            _ = token.cancelled() => return Ok(()),
            _ = ticker.tick() => {}
        }
        if stack_mode.is_paused() {
            info!("Retired stack stopped consensus manifest publication");
            return Ok(());
        }

        let discovered = discover_known_children(&pool).await?;
        if discovered > 0 {
            debug!(discovered, "Discovered manifest lineage blocks");
        }

        loop {
            let chains = pending_chain_ids(&pool, cadence).await?;
            if chains.is_empty() {
                break;
            }

            let mut advanced = false;
            for host_chain_id in chains {
                match progress_chain(
                    &pool,
                    &client,
                    &conf.s3.bucket_ct128,
                    cadence,
                    host_chain_id,
                    &signer,
                    &conf.consensus,
                )
                .await
                {
                    Ok(Progress::Advanced) => advanced = true,
                    Ok(Progress::Waiting | Progress::Busy) => {}
                    Err(ExecutionError::DbError(err)) => {
                        return Err(ExecutionError::DbError(err));
                    }
                    Err(err) => {
                        error!(host_chain_id, error = %err, "Manifest publication attempt failed");
                    }
                }
            }

            if !advanced {
                break;
            }
        }
    }
}

async fn progress_chain(
    pool: &PgPool,
    client: &Client,
    bucket: &str,
    cadence: i64,
    host_chain_id: i64,
    signer: &CoproSigner,
    consensus: &ConsensusConfig,
) -> Result<Progress, ExecutionError> {
    let mut cursor = ManifestProgressCursor::start();
    let mut attempted_candidate = false;

    loop {
        let mut trx = pool.begin().await?;
        let Some(block) =
            lock_next_block_to_progress(&mut trx, host_chain_id, cadence, &cursor).await?
        else {
            trx.rollback().await?;
            return Ok(if attempted_candidate {
                Progress::Waiting
            } else {
                Progress::Busy
            });
        };
        attempted_candidate = true;
        cursor.advance_to(&block);

        let result = progress_locked_block(
            &mut trx,
            client,
            bucket,
            host_chain_id,
            &block,
            signer,
            consensus,
        )
        .await;
        match result {
            Ok(Progress::Advanced) => {
                trx.commit().await?;
                return Ok(Progress::Advanced);
            }
            Ok(Progress::Waiting) => {
                trx.commit().await?;
            }
            Ok(Progress::Busy) => unreachable!("a locked block cannot be busy"),
            Err(err @ ExecutionError::DbError(_)) => return Err(err),
            Err(err) => {
                trx.rollback().await?;
                error!(
                    host_chain_id,
                    block_number = block.block_number,
                    block_hash = %hex::encode(&block.block_hash),
                    error = %err,
                    "Manifest candidate failed; continuing with another lineage"
                );
            }
        }
    }
}

async fn progress_locked_block(
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    client: &Client,
    bucket: &str,
    host_chain_id: i64,
    block: &PendingBlock,
    signer: &CoproSigner,
    consensus: &ConsensusConfig,
) -> Result<Progress, ExecutionError> {
    if block.block_content_digest.is_none() {
        if !is_block_manifest_ready(trx, block).await? {
            return Ok(Progress::Waiting);
        }
        let descriptors = load_manifest_descriptors(trx, block).await?;
        seal_block_content(trx, block, COPROCESSOR_CONTEXT_ID_1, &descriptors).await?;
        let discovered = discover_block_children(trx, block).await?;
        debug!(
            host_chain_id,
            block_number = block.block_number,
            block_hash = %hex::encode(&block.block_hash),
            descriptor_count = descriptors.len(),
            discovered_children = discovered,
            "Sealed block manifest content"
        );
        return Ok(Progress::Advanced);
    }

    publish_block_manifest(trx, client, bucket, block, signer, consensus).await?;
    Ok(Progress::Advanced)
}

pub(crate) async fn publish_block_manifest(
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    client: &Client,
    bucket: &str,
    block: &PendingBlock,
    signer: &CoproSigner,
    consensus: &ConsensusConfig,
) -> Result<(), ExecutionError> {
    let prepared = prepare_manifest(trx, block, COPROCESSOR_CONTEXT_ID_1, signer.address()).await?;
    let detailed_range_start = i64::try_from(prepared.payload.detailed_range.first_block_number)
        .map_err(|_| ExecutionError::InternalError("detailed range start exceeds BIGINT".into()))?;
    let detailed_range_digest = prepared.payload.detailed_range.digest;
    let frontier_range_count = prepared.next_frontier.as_slice().len();
    let signed = prepared
        .payload
        .sign(signer.as_ref())
        .await
        .map_err(|err| ExecutionError::InternalError(err.to_string()))?;
    signed
        .verify()
        .map_err(|err| ExecutionError::InternalError(err.to_string()))?;
    let manifest_digest = signed
        .digest()
        .map_err(|err| ExecutionError::InternalError(err.to_string()))?;
    let body = serde_json::to_vec(&signed)
        .map_err(|err| ExecutionError::SerializationError(err.to_string()))?;
    let key = manifest_object_key(&signed);

    let stored_body = put_immutable_manifest(client, bucket, &key, &signed, &body).await?;
    let archived =
        store_authenticated_manifest(trx, signed.payload.publisher, &key, &stored_body).await?;
    if archived.manifest.digest != manifest_digest {
        return Err(ExecutionError::InternalError(format!(
            "stored manifest digest {} does not match published digest {manifest_digest}",
            archived.manifest.digest,
        )));
    }
    mark_manifest_published(
        trx,
        block,
        detailed_range_start,
        detailed_range_digest,
        manifest_digest,
    )
    .await?;
    if consensus.verify_others_party_manifests {
        schedule_manifest_verification(
            trx,
            &archived.manifest,
            consensus.verification_delay,
            consensus.verification_retry_delay,
            consensus.verification_retry_count,
        )
        .await?;
    }
    info!(
        host_chain_id = block.host_chain_id,
        block_number = block.block_number,
        block_hash = %hex::encode(&block.block_hash),
        revision = block.manifest_revision,
        manifest_digest = %manifest_digest,
        frontier_range_count,
        bucket,
        key,
        archive_outcome = ?archived.outcome,
        "Published immutable consensus manifest"
    );
    Ok(())
}

async fn put_immutable_manifest(
    client: &Client,
    bucket: &str,
    key: &str,
    intended: &SignedManifest,
    body: &[u8],
) -> Result<Vec<u8>, ExecutionError> {
    let result = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .content_type("application/json")
        .if_none_match("*")
        .body(ByteStream::from(body.to_vec()))
        .send()
        .await;

    match result {
        Ok(_) => Ok(body.to_vec()),
        Err(err)
            if err.as_service_error().and_then(ProvideErrorMetadata::code)
                == Some("PreconditionFailed") =>
        {
            let existing = client
                .get_object()
                .bucket(bucket)
                .key(key)
                .send()
                .await
                .map_err(|get_err| ExecutionError::S3TransientError(get_err.to_string()))?;
            let existing = existing
                .body
                .collect()
                .await
                .map_err(|collect_err| ExecutionError::S3TransientError(collect_err.to_string()))?
                .into_bytes();
            validate_existing_manifest(&existing, intended, bucket, key)?;
            Ok(existing.to_vec())
        }
        Err(err) => Err(ExecutionError::S3TransientError(err.to_string())),
    }
}

fn validate_existing_manifest(
    existing: &[u8],
    intended: &SignedManifest,
    bucket: &str,
    key: &str,
) -> Result<(), ExecutionError> {
    let existing: SignedManifest = serde_json::from_slice(existing).map_err(|err| {
        ExecutionError::InternalError(format!(
            "immutable manifest object at s3://{bucket}/{key} is not valid JSON: {err}",
        ))
    })?;
    existing.verify().map_err(|err| {
        ExecutionError::InternalError(format!(
            "immutable manifest object at s3://{bucket}/{key} has an invalid signature: {err}",
        ))
    })?;
    if existing.payload != intended.payload {
        return Err(ExecutionError::InternalError(format!(
            "immutable manifest object already exists with a different payload at s3://{bucket}/{key}",
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use alloy::signers::local::PrivateKeySigner;
    use alloy_primitives::{Address, B256, U256};
    use ciphertext_attestation::manifest::{
        block_content_digest, detailed_range_digest, DetailedRange, ManifestBlockEntry,
        ManifestPayload, ManifestReference, ManifestVersion, SignedManifest,
    };
    use fhevm_engine_common::types::CoproSigner;
    use serial_test::serial;
    use sqlx::{PgPool, Row};
    use test_harness::instance::{setup_test_db, ImportMode};

    use super::*;
    use crate::consensus::{
        manifest::{
            is_block_manifest_ready, queue_manifest_revision_after_replay, seal_block_content,
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
        let client =
            test_harness::localstack::create_localstack_s3_client(localstack.host_port).await;
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

        publish_pending_revision(
            &pool, &client, bucket, CHAIN_ID, context, &signer, &consensus,
        )
        .await;
        publish_pending_revision(
            &pool, &client, bucket, CHAIN_ID, context, &signer, &consensus,
        )
        .await;
        let revision_one = load_local_revision(
            &pool,
            signer.address(),
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
            signer.address(),
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
                block_number: child_revision_zero.signed.payload.publication_block_number,
                block_hash: child_block_hash,
                revision: 0,
                manifest_digest: child_revision_zero.digest,
            })
        );
        assert_eq!(
            child_revision_one.signed.payload.previous_manifest,
            Some(ManifestReference {
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
                    (SELECT COUNT(*) FROM block_consensus_manifest
                      WHERE publisher = $1 AND host_chain_id = $2
                        AND publication_block_hash = $3) AS archive_count
               FROM block_consensus
              WHERE host_chain_id = $2 AND block_hash = $3",
        )
        .bind(signer.address().as_slice())
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
            "SELECT COUNT(*) FROM block_consensus_manifest
              WHERE publisher = $1 AND host_chain_id = $2",
        )
        .bind(signer.address().as_slice())
        .bind(CHAIN_ID)
        .fetch_one(&pool)
        .await
        .expect("count all uploaded manifest revisions");
        assert_eq!(total_archive_count, 4);
    }

    #[tokio::test]
    #[serial(db)]
    #[ignore = "PostgreSQL and LocalStack-backed competing-lineage publication"]
    async fn failed_creation_or_upload_does_not_block_competing_lineage_with_multiple_workers() {
        const CHAIN_ID: i64 = 9;
        const BLOCK_NUMBER: i64 = 42;
        const WORKER_COUNT: usize = 4;
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
        let client =
            test_harness::localstack::create_localstack_s3_client(localstack.host_port).await;
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
                for _ in 0..2 {
                    outcomes.push(
                        progress_chain(&pool, &client, bucket, 1, CHAIN_ID, &signer, &consensus)
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
                .filter(|outcome| *outcome == Progress::Advanced)
                .count();
        }
        assert!(
            advanced >= 2,
            "one worker must seal and publish the ready fork"
        );

        let states = sqlx::query(
            "SELECT block_hash, manifest_published
               FROM block_consensus
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
            "SELECT COUNT(*) FROM block_consensus_manifest
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
                progress_chain(&pool, &client, bucket, 1, CHAIN_ID, &signer, &consensus,)
                    .await
                    .expect("retry permanently conflicting manifest"),
                Progress::Waiting,
            );
        }
    }

    async fn publish_pending_revision(
        pool: &PgPool,
        client: &Client,
        bucket: &str,
        host_chain_id: i64,
        context: U256,
        signer: &CoproSigner,
        consensus: &ConsensusConfig,
    ) {
        let mut trx = pool.begin().await.expect("begin block seal");
        let cursor = ManifestProgressCursor::start();
        let block = lock_next_block_to_progress(&mut trx, host_chain_id, 1, &cursor)
            .await
            .expect("lock block for seal")
            .expect("pending block for seal");
        assert!(block.block_content_digest.is_none());
        assert!(is_block_manifest_ready(&mut trx, &block)
            .await
            .expect("check replayed block readiness"));
        let descriptors = load_manifest_descriptors(&mut trx, &block)
            .await
            .expect("load replayed descriptors");
        seal_block_content(&mut trx, &block, context, &descriptors)
            .await
            .expect("seal replayed block");
        trx.commit().await.expect("commit replayed block seal");

        let mut trx = pool.begin().await.expect("begin manifest publication");
        let cursor = ManifestProgressCursor::start();
        let block = lock_next_block_to_progress(&mut trx, host_chain_id, 1, &cursor)
            .await
            .expect("lock block for publication")
            .expect("pending block for publication");
        publish_block_manifest(&mut trx, client, bucket, &block, signer, consensus)
            .await
            .expect("publish numbered manifest revision");
        trx.commit().await.expect("commit manifest publication");
    }

    async fn load_seeded_block(
        pool: &PgPool,
        host_chain_id: i64,
        block_hash: B256,
    ) -> PendingBlock {
        let row = sqlx::query(
            "SELECT host_chain_id, block_number, block_hash, parent_block_hash,
                    block_content_digest, descriptor_count, manifest_revision,
                    manifest_digest, manifest_published
               FROM block_consensus
              WHERE host_chain_id = $1 AND block_hash = $2",
        )
        .bind(host_chain_id)
        .bind(block_hash.as_slice())
        .fetch_one(pool)
        .await
        .expect("load seeded manifest block");
        PendingBlock {
            host_chain_id: row.get("host_chain_id"),
            block_number: row.get("block_number"),
            block_hash: row.get("block_hash"),
            parent_block_hash: row.get("parent_block_hash"),
            block_content_digest: row.get("block_content_digest"),
            descriptor_count: row.get("descriptor_count"),
            manifest_revision: row.get("manifest_revision"),
            manifest_digest: row.get("manifest_digest"),
            manifest_published: row.get("manifest_published"),
        }
    }

    async fn seal_seeded_block(pool: &PgPool, host_chain_id: i64, block_hash: B256, context: U256) {
        let mut trx = pool.begin().await.expect("begin seeded block seal");
        let block = load_seeded_block(pool, host_chain_id, block_hash).await;
        assert!(is_block_manifest_ready(&mut trx, &block)
            .await
            .expect("check seeded block readiness"));
        let descriptors = load_manifest_descriptors(&mut trx, &block)
            .await
            .expect("load seeded block descriptors");
        seal_block_content(&mut trx, &block, context, &descriptors)
            .await
            .expect("seal seeded block");
        trx.commit().await.expect("commit seeded block seal");
    }

    #[allow(clippy::too_many_arguments)]
    async fn seed_revision_publication_block(
        pool: &PgPool,
        host_chain_id: i64,
        block_number: i64,
        block_hash: B256,
        parent_hash: B256,
        handle: B256,
        transaction_id: B256,
        dependence_chain_id: B256,
        gateway_key_id: B256,
        keyset_id: B256,
        ct64_digest: B256,
        ct128_digest: B256,
    ) {
        sqlx::query(
            "INSERT INTO block_consensus
                 (host_chain_id, block_number, block_hash, parent_block_hash)
             VALUES ($1, $2, $3, $4)",
        )
        .bind(host_chain_id)
        .bind(block_number)
        .bind(block_hash.as_slice())
        .bind(parent_hash.as_slice())
        .execute(pool)
        .await
        .expect("insert revision publication block");
        sqlx::query(
            "INSERT INTO computations_branch (
                 output_handle, dependencies, fhe_operation, is_scalar,
                 dependence_chain_id, transaction_id, is_allowed,
                 schedule_order, is_completed, is_error, host_chain_id,
                 block_number, producer_block_hash
             ) VALUES (
                 $1, ARRAY[]::BYTEA[], 0, FALSE, $2, $3, TRUE,
                 NOW(), TRUE, FALSE, $4, $5, $6
             )",
        )
        .bind(handle.as_slice())
        .bind(dependence_chain_id.as_slice())
        .bind(transaction_id.as_slice())
        .bind(host_chain_id)
        .bind(block_number)
        .bind(block_hash.as_slice())
        .execute(pool)
        .await
        .expect("insert revision computation");
        sqlx::query(
            "INSERT INTO pbs_computations_branch (
                 handle, host_chain_id, block_number, producer_block_hash,
                 block_hash, is_completed, is_error
             ) VALUES ($1, $2, $3, $4, $4, TRUE, FALSE)",
        )
        .bind(handle.as_slice())
        .bind(host_chain_id)
        .bind(block_number)
        .bind(block_hash.as_slice())
        .execute(pool)
        .await
        .expect("insert revision PBS witness");
        sqlx::query(
            "INSERT INTO keys (key_id_gw, key_id, pks_key, sks_key, chain_id, block_hash)
             VALUES ($1, $2, ''::BYTEA, ''::BYTEA, $3, $4)",
        )
        .bind(gateway_key_id.as_slice())
        .bind(keyset_id.as_slice())
        .bind(host_chain_id)
        .bind(block_hash.as_slice())
        .execute(pool)
        .await
        .expect("insert revision keyset");
        sqlx::query(
            "INSERT INTO ciphertext_digest_branch (
                 host_chain_id, key_id_gw, handle, producer_block_hash,
                 block_hash, block_number, ciphertext, ciphertext128,
                 ciphertext128_format
             ) VALUES ($1, $2, $3, $4, $4, $5, $6, $7, 11)",
        )
        .bind(host_chain_id)
        .bind(gateway_key_id.as_slice())
        .bind(handle.as_slice())
        .bind(block_hash.as_slice())
        .bind(block_number)
        .bind(ct64_digest.as_slice())
        .bind(ct128_digest.as_slice())
        .execute(pool)
        .await
        .expect("insert revision ciphertext digests");
    }

    async fn load_local_revision(
        pool: &PgPool,
        publisher: Address,
        context: U256,
        host_chain_id: i64,
        block_number: i64,
        block_hash: B256,
        revision: i64,
    ) -> AuthenticatedManifest {
        let mut trx = pool.begin().await.expect("begin local revision load");
        let manifest = load_manifest_revision(
            &mut trx,
            publisher,
            ManifestVersion::V1,
            context,
            host_chain_id,
            block_number,
            block_hash,
            revision,
        )
        .await
        .expect("load local manifest revision")
        .expect("local manifest revision exists");
        trx.commit().await.expect("commit local revision load");
        manifest
    }
}
