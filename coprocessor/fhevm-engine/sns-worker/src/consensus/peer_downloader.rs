use std::{collections::HashSet, sync::Arc, time::Duration};

use alloy_primitives::{Address, B256, U256};
use aws_sdk_s3::Client;
use ciphertext_attestation::manifest::{ManifestReference, ManifestVersion, SignedManifest};
use fhevm_engine_common::{
    pg_pool::{PostgresPoolManager, ServiceError},
    versioning::StackMode,
};
use sqlx::{PgPool, Postgres, Transaction};
use tokio::{task::JoinHandle, time::MissedTickBehavior};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};
use url::Url;

use crate::ExecutionError;

use super::consensus_analysis::{evaluate_quorum, VerificationOutcome};
use super::drift_findings::apply_evaluation_to_drift_handles;
use super::manifest_archive::{
    load_manifest_by_reference, load_tip_eligible_manifest, store_authenticated_manifest,
    AuthenticatedManifest,
};

const DOWNLOAD_POLL_INTERVAL: Duration = Duration::from_secs(1);
const DOWNLOAD_LEASE: Duration = Duration::from_secs(5 * 60);
const PEER_FETCH_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_LIST_PAGES_PER_ATTEMPT: usize = 10;

#[derive(Clone, Debug)]
pub(crate) struct PeerManifestObject {
    pub object_key: String,
    pub signed_bytes: Vec<u8>,
}

#[derive(Clone, Debug)]
pub(crate) struct PeerDownloadRequest {
    pub publisher: Address,
    pub s3_bucket_url: String,
    pub version: ManifestVersion,
    pub coprocessor_context_id: U256,
    pub host_chain_id: i64,
    pub publication_block_number: i64,
    pub publication_block_hash: B256,
    pub known_revisions: HashSet<u64>,
}

pub(crate) trait PeerManifestSource: Send + Sync {
    async fn fetch_manifests(
        &self,
        request: &PeerDownloadRequest,
    ) -> Result<Vec<PeerManifestObject>, ExecutionError>;
}

#[derive(Clone)]
pub(crate) struct S3PeerManifestSource {
    client: Arc<Client>,
}

impl S3PeerManifestSource {
    pub(crate) fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

impl PeerManifestSource for S3PeerManifestSource {
    async fn fetch_manifests(
        &self,
        request: &PeerDownloadRequest,
    ) -> Result<Vec<PeerManifestObject>, ExecutionError> {
        debug!(
            publisher = %request.publisher,
            bucket_url = request.s3_bucket_url,
            "Listing numbered peer manifest revisions"
        );
        let location = s3_bucket_location(&request.s3_bucket_url)?;
        let bucket = location.bucket.clone();
        let registered_prefix = location.key_prefix.clone();
        let prefix = location.object_key(manifest_prefix(request));
        let mut continuation_token = None;
        let mut keys = Vec::new();
        let mut fully_listed = false;

        for _ in 0..MAX_LIST_PAGES_PER_ATTEMPT {
            let mut list = self
                .client
                .list_objects_v2()
                .bucket(&bucket)
                .prefix(&prefix);
            if let Some(token) = continuation_token.as_deref() {
                list = list.continuation_token(token);
            }
            let response = list.send().await.map_err(|err| {
                ExecutionError::S3TransientError(format!(
                    "failed to list peer manifest prefix {prefix} in {bucket}: {err}"
                ))
            })?;
            for object in response.contents() {
                let Some(key) = object.key() else {
                    continue;
                };
                let Some(revision) = revision_below_prefix(key, &prefix) else {
                    continue;
                };
                if !request.known_revisions.contains(&revision) {
                    let canonical_key = if registered_prefix.is_empty() {
                        key.to_owned()
                    } else {
                        key.strip_prefix(&format!("{registered_prefix}/"))
                            .unwrap_or(key)
                            .to_owned()
                    };
                    keys.push((revision, key.to_owned(), canonical_key));
                }
            }
            if response.is_truncated() != Some(true) {
                fully_listed = true;
                break;
            }
            continuation_token = response.next_continuation_token().map(ToOwned::to_owned);
            if continuation_token.is_none() {
                break;
            }
        }
        if !fully_listed {
            return Err(ExecutionError::S3TransientError(format!(
                "peer manifest prefix {prefix} in {bucket} exceeded the bounded listing budget"
            )));
        }

        keys.sort_unstable_by_key(|(revision, _, _)| *revision);
        keys.dedup_by_key(|(revision, _, _)| *revision);
        let mut manifests = Vec::with_capacity(keys.len());
        for (_, s3_object_key, canonical_object_key) in keys {
            let body = self
                .client
                .get_object()
                .bucket(&bucket)
                .key(&s3_object_key)
                .send()
                .await
                .map_err(|err| {
                    ExecutionError::S3TransientError(format!(
                        "failed to download peer manifest {s3_object_key} from {bucket}: {err}"
                    ))
                })?
                .body
                .collect()
                .await
                .map_err(|err| {
                    ExecutionError::S3TransientError(format!(
                        "failed to read peer manifest {s3_object_key} from {bucket}: {err}"
                    ))
                })?
                .into_bytes()
                .to_vec();
            manifests.push(PeerManifestObject {
                object_key: canonical_object_key,
                signed_bytes: body,
            });
        }
        Ok(manifests)
    }
}

#[derive(Clone, Debug)]
struct VerificationScope {
    local_publisher: Address,
    version: ManifestVersion,
    coprocessor_context_id: U256,
    host_chain_id: i64,
    publication_block_number: i64,
    publication_block_hash: B256,
    revision: u64,
    local_manifest_digest: B256,
}

#[derive(Clone, Debug)]
struct ClaimedPeer {
    publisher: Address,
    s3_bucket_url: String,
    known_revisions: HashSet<u64>,
}

#[derive(Clone, Debug)]
pub(crate) struct VerificationClaim {
    target_id: i64,
    worker_id: String,
    attempt: i32,
    required_quorum: usize,
    scope: VerificationScope,
    peers: Vec<ClaimedPeer>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct VerificationRunResult {
    pub target_id: i64,
    pub attempt: i32,
    pub outcome: VerificationOutcome,
}

pub(crate) async fn spawn_peer_manifest_downloader(
    pool_mngr: &PostgresPoolManager,
    client: Arc<Client>,
    stack_mode: Arc<StackMode>,
) -> Result<JoinHandle<Result<(), ServiceError>>, ExecutionError> {
    let source = S3PeerManifestSource::new(client);
    let op = move |pool, token| {
        let source = source.clone();
        let stack_mode = Arc::clone(&stack_mode);
        async move {
            run_peer_manifest_downloader(pool, token, source, stack_mode)
                .await
                .map_err(ServiceError::from)
        }
    };
    Ok(pool_mngr
        .spawn_with_db_retry(op, "consensus_peer_manifest_downloader")
        .await)
}

async fn run_peer_manifest_downloader<S: PeerManifestSource>(
    pool: PgPool,
    token: CancellationToken,
    source: S,
    stack_mode: Arc<StackMode>,
) -> Result<(), ExecutionError> {
    while stack_mode.gcs_mode() {
        tokio::select! {
            _ = token.cancelled() => return Ok(()),
            _ = tokio::time::sleep(Duration::from_secs(2)) => {}
        }
    }
    info!("Peer manifest verification enabled on the live stack");
    let worker_id = downloader_worker_id();
    let mut ticker = tokio::time::interval(DOWNLOAD_POLL_INTERVAL);
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        if stack_mode.is_paused() {
            info!("Retired stack stopped peer manifest verification");
            return Ok(());
        }
        tokio::select! {
            _ = token.cancelled() => return Ok(()),
            _ = ticker.tick() => {}
        }

        loop {
            match run_peer_manifest_download_once(&pool, &source, &worker_id, DOWNLOAD_LEASE).await
            {
                Ok(Some(result)) => {
                    debug!(
                        target_id = result.target_id,
                        attempt = result.attempt,
                        outcome = ?result.outcome,
                        "Completed peer manifest download attempt"
                    );
                }
                Ok(None) => break,
                Err(ExecutionError::DbError(err)) => return Err(ExecutionError::DbError(err)),
                Err(err) => {
                    error!(error = %err, "Peer manifest verification attempt failed");
                    break;
                }
            }
        }
    }
}

pub(crate) async fn schedule_manifest_verification(
    trx: &mut Transaction<'_, Postgres>,
    local: &AuthenticatedManifest,
    verification_delay: Duration,
    retry_delay: Duration,
    retry_count: u32,
) -> Result<i64, ExecutionError> {
    let payload = &local.signed.payload;
    let host_chain_id = i64_from_u256("manifest host chain id", payload.host_chain_id)?;
    let publication_block_number = i64_from_u256(
        "manifest publication block number",
        payload.publication_block_number,
    )?;
    let revision = i64::try_from(payload.revision)
        .map_err(|_| internal("manifest revision exceeds BIGINT"))?;
    let delay_micros = duration_micros("verification delay", verification_delay)?;
    let retry_delay_micros = duration_micros("verification retry delay", retry_delay)?;
    let max_attempts = retry_count
        .checked_add(1)
        .and_then(|attempts| i32::try_from(attempts).ok())
        .ok_or_else(|| internal("verification retry count exceeds INTEGER"))?;
    let context = payload.coprocessor_context_id.to_be_bytes::<32>();

    let inserted = sqlx::query!(
        r#"
        INSERT INTO block_consensus_verification_target (
            local_publisher,
            version,
            coprocessor_context_id,
            host_chain_id,
            publication_block_number,
            publication_block_hash,
            revision,
            local_manifest_digest,
            eligible_at,
            next_attempt_at,
            retry_delay_micros,
            max_attempts
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8,
            NOW() + $9::BIGINT * INTERVAL '1 microsecond',
            NOW() + $9::BIGINT * INTERVAL '1 microsecond',
            $10, $11
        )
        ON CONFLICT (
            local_publisher,
            version,
            coprocessor_context_id,
            host_chain_id,
            publication_block_number,
            publication_block_hash,
            revision
        ) DO NOTHING
        RETURNING id
        "#,
        payload.publisher.as_slice(),
        i16::from(u8::from(payload.version)),
        context.as_slice(),
        host_chain_id,
        publication_block_number,
        payload.publication_block_hash.as_slice(),
        revision,
        local.digest.as_slice(),
        delay_micros,
        retry_delay_micros,
        max_attempts,
    )
    .fetch_optional(trx.as_mut())
    .await?;

    let target_id = if let Some(row) = inserted {
        row.id
    } else {
        let row = sqlx::query!(
            r#"
            SELECT id, local_manifest_digest
              FROM block_consensus_verification_target
             WHERE local_publisher = $1
               AND version = $2
               AND coprocessor_context_id = $3
               AND host_chain_id = $4
               AND publication_block_number = $5
               AND publication_block_hash = $6
               AND revision = $7
            "#,
            payload.publisher.as_slice(),
            i16::from(u8::from(payload.version)),
            context.as_slice(),
            host_chain_id,
            publication_block_number,
            payload.publication_block_hash.as_slice(),
            revision,
        )
        .fetch_one(trx.as_mut())
        .await?;
        let stored_digest = b256(
            "stored verification target manifest digest",
            &row.local_manifest_digest,
        )?;
        if stored_digest != local.digest {
            return Err(internal(format!(
                "verification target for publisher {} revision {} has conflicting digest",
                payload.publisher, payload.revision,
            )));
        }
        row.id
    };

    bind_target_to_current_registry(trx, target_id).await?;
    Ok(target_id)
}

pub(crate) async fn run_peer_manifest_download_once<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    worker_id: &str,
    lease_duration: Duration,
) -> Result<Option<VerificationRunResult>, ExecutionError> {
    bind_one_waiting_target(pool).await?;
    let Some(claim) = claim_due_target(pool, worker_id, lease_duration).await? else {
        return Ok(None);
    };

    for peer in &claim.peers {
        download_claimed_peer(pool, source, &claim, peer).await?;
    }
    finish_claim(pool, &claim).await.map(Some)
}

async fn bind_one_waiting_target(pool: &PgPool) -> Result<(), ExecutionError> {
    let mut trx = pool.begin().await?;
    let target_id = sqlx::query_scalar!(
        r#"
        SELECT id
          FROM block_consensus_verification_target
         WHERE state = 'waiting_registry'
           AND next_attempt_at <= NOW()
         ORDER BY next_attempt_at, id
         FOR UPDATE SKIP LOCKED
         LIMIT 1
        "#,
    )
    .fetch_optional(trx.as_mut())
    .await?;
    if let Some(target_id) = target_id {
        bind_target_to_current_registry(&mut trx, target_id).await?;
    }
    trx.commit().await?;
    Ok(())
}

async fn bind_target_to_current_registry(
    trx: &mut Transaction<'_, Postgres>,
    target_id: i64,
) -> Result<bool, ExecutionError> {
    let target = sqlx::query!(
        r#"
        SELECT local_publisher,
               version,
               coprocessor_context_id,
               host_chain_id,
               publication_block_number,
               publication_block_hash,
               state
          FROM block_consensus_verification_target
         WHERE id = $1
         FOR UPDATE
        "#,
        target_id,
    )
    .fetch_one(trx.as_mut())
    .await?;
    if target.state != "waiting_registry" {
        return Ok(false);
    }

    let registry = sqlx::query!(
        r#"
        SELECT signer_address,
               s3_bucket_url,
               coprocessor_threshold,
               gateway_chain_id,
               gateway_config_address,
               snapshot_block_number,
               snapshot_block_hash
          FROM gateway_config_coprocessors
         ORDER BY signer_address
        "#,
    )
    .fetch_all(trx.as_mut())
    .await?;
    if registry.is_empty() {
        postpone_registry_binding(trx, target_id, "GatewayConfig registry snapshot is empty")
            .await?;
        return Ok(false);
    }

    let local_publisher = address(
        "verification target local publisher",
        &target.local_publisher,
    )?;
    let first = &registry[0];
    let threshold = first.coprocessor_threshold;
    let coprocessor_count = i32::try_from(registry.len())
        .map_err(|_| internal("registered coprocessor count exceeds INTEGER"))?;
    let required_quorum = i32::try_from(threshold)
        .map_err(|_| internal("GatewayConfig threshold exceeds INTEGER"))?;
    if required_quorum <= 0 || required_quorum > coprocessor_count {
        postpone_registry_binding(
            trx,
            target_id,
            &format!(
                "invalid GatewayConfig threshold {required_quorum} for {coprocessor_count} coprocessors"
            ),
        )
        .await?;
        return Ok(false);
    }

    let gateway_chain_id = first.gateway_chain_id;
    let gateway_config_address = first.gateway_config_address.clone();
    let snapshot_block_number = first.snapshot_block_number;
    let snapshot_block_hash = first.snapshot_block_hash.clone();
    let mut local_registered = false;
    let mut peers = Vec::with_capacity(registry.len().saturating_sub(1));
    for row in registry {
        if row.coprocessor_threshold != threshold
            || row.gateway_chain_id != gateway_chain_id
            || row.gateway_config_address != gateway_config_address
            || row.snapshot_block_number != snapshot_block_number
            || row.snapshot_block_hash != snapshot_block_hash
        {
            return Err(internal(
                "GatewayConfig registry rows do not form one consistent snapshot",
            ));
        }
        let publisher = address("GatewayConfig signer address", &row.signer_address)?;
        if publisher == local_publisher {
            local_registered = true;
        } else {
            peers.push((publisher, row.s3_bucket_url));
        }
    }
    if !local_registered {
        postpone_registry_binding(
            trx,
            target_id,
            "local manifest publisher is absent from GatewayConfig registry",
        )
        .await?;
        return Ok(false);
    }

    sqlx::query!(
        r#"
        UPDATE block_consensus_verification_target
           SET state = 'pending',
               gateway_chain_id = $2,
               gateway_config_address = $3,
               registry_block_number = $4,
               registry_block_hash = $5,
               registered_coprocessor_count = $6,
               required_quorum = $7,
               last_error = NULL,
               updated_at = NOW()
         WHERE id = $1
        "#,
        target_id,
        gateway_chain_id,
        &gateway_config_address,
        snapshot_block_number,
        &snapshot_block_hash,
        coprocessor_count,
        required_quorum,
    )
    .execute(trx.as_mut())
    .await?;
    for (publisher, s3_bucket_url) in peers {
        sqlx::query!(
            r#"
            INSERT INTO block_consensus_peer_download (
                target_id,
                publisher,
                s3_bucket_url
            )
            VALUES ($1, $2, $3)
            ON CONFLICT (target_id, publisher) DO NOTHING
            "#,
            target_id,
            publisher.as_slice(),
            s3_bucket_url,
        )
        .execute(trx.as_mut())
        .await?;
    }
    Ok(true)
}

async fn postpone_registry_binding(
    trx: &mut Transaction<'_, Postgres>,
    target_id: i64,
    error: &str,
) -> Result<(), ExecutionError> {
    sqlx::query!(
        r#"
        UPDATE block_consensus_verification_target
           SET next_attempt_at = GREATEST(
                   eligible_at,
                   NOW() + retry_delay_micros * INTERVAL '1 microsecond'
               ),
               last_error = $2,
               updated_at = NOW()
         WHERE id = $1
        "#,
        target_id,
        error,
    )
    .execute(trx.as_mut())
    .await?;
    Ok(())
}

pub(crate) async fn claim_due_target(
    pool: &PgPool,
    worker_id: &str,
    lease_duration: Duration,
) -> Result<Option<VerificationClaim>, ExecutionError> {
    let lease_micros = duration_micros("verification lease", lease_duration)?;
    let mut trx = pool.begin().await?;
    let row = sqlx::query!(
        r#"
        WITH selected AS (
            SELECT id
              FROM block_consensus_verification_target
             WHERE required_quorum IS NOT NULL
               AND next_attempt_at <= NOW()
               AND (
                    state = 'pending'
                    OR (state = 'leased' AND lease_expires_at <= NOW())
               )
             ORDER BY next_attempt_at, id
             FOR UPDATE SKIP LOCKED
             LIMIT 1
        )
        UPDATE block_consensus_verification_target target
           SET state = 'leased',
               lease_owner = $1,
               lease_expires_at = NOW() + $2::BIGINT * INTERVAL '1 microsecond',
               updated_at = NOW()
          FROM selected
         WHERE target.id = selected.id
        RETURNING target.id,
                  target.attempt_count + 1 AS "attempt!",
                  target.required_quorum AS "required_quorum!",
                  target.local_publisher,
                  target.version,
                  target.coprocessor_context_id,
                  target.host_chain_id,
                  target.publication_block_number,
                  target.publication_block_hash,
                  target.revision,
                  target.local_manifest_digest
        "#,
        worker_id,
        lease_micros,
    )
    .fetch_optional(trx.as_mut())
    .await?;
    let Some(row) = row else {
        trx.commit().await?;
        return Ok(None);
    };
    let target_id = row.id;
    let attempt = row.attempt;
    let context = u256("verification target context", &row.coprocessor_context_id)?;
    let version = manifest_version(row.version)?;
    let scope = VerificationScope {
        local_publisher: address("verification target local publisher", &row.local_publisher)?,
        version,
        coprocessor_context_id: context,
        host_chain_id: row.host_chain_id,
        publication_block_number: row.publication_block_number,
        publication_block_hash: b256(
            "verification target publication block hash",
            &row.publication_block_hash,
        )?,
        revision: u64::try_from(row.revision)
            .map_err(|_| internal("verification target revision is negative"))?,
        local_manifest_digest: b256(
            "verification target local manifest digest",
            &row.local_manifest_digest,
        )?,
    };

    let peer_rows = sqlx::query!(
        r#"
        SELECT publisher, s3_bucket_url
          FROM block_consensus_peer_download
         WHERE target_id = $1
           AND completed_attempt < $2
         ORDER BY publisher
        "#,
        target_id,
        attempt,
    )
    .fetch_all(trx.as_mut())
    .await?;
    let mut peers = Vec::with_capacity(peer_rows.len());
    for peer in peer_rows {
        let publisher = address("peer download publisher", &peer.publisher)?;
        let scope_context = scope.coprocessor_context_id.to_be_bytes::<32>();
        let known_rows = sqlx::query_scalar!(
            r#"
            SELECT revision
              FROM block_consensus_manifest
             WHERE publisher = $1
               AND version = $2
               AND coprocessor_context_id = $3
               AND host_chain_id = $4
               AND publication_block_number = $5
               AND publication_block_hash = $6
            "#,
            publisher.as_slice(),
            i16::from(u8::from(scope.version)),
            scope_context.as_slice(),
            scope.host_chain_id,
            scope.publication_block_number,
            scope.publication_block_hash.as_slice(),
        )
        .fetch_all(trx.as_mut())
        .await?;
        let known_revisions = known_rows
            .into_iter()
            .map(|revision| {
                u64::try_from(revision)
                    .map_err(|_| internal("archived peer manifest revision is negative"))
            })
            .collect::<Result<HashSet<_>, _>>()?;
        peers.push(ClaimedPeer {
            publisher,
            s3_bucket_url: peer.s3_bucket_url,
            known_revisions,
        });
    }
    trx.commit().await?;
    Ok(Some(VerificationClaim {
        target_id,
        worker_id: worker_id.to_owned(),
        attempt,
        required_quorum: usize::try_from(row.required_quorum)
            .map_err(|_| internal("required quorum is negative"))?,
        scope,
        peers,
    }))
}

async fn download_claimed_peer<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    claim: &VerificationClaim,
    peer: &ClaimedPeer,
) -> Result<(), ExecutionError> {
    let request = PeerDownloadRequest {
        publisher: peer.publisher,
        s3_bucket_url: peer.s3_bucket_url.clone(),
        version: claim.scope.version,
        coprocessor_context_id: claim.scope.coprocessor_context_id,
        host_chain_id: claim.scope.host_chain_id,
        publication_block_number: claim.scope.publication_block_number,
        publication_block_hash: claim.scope.publication_block_hash,
        known_revisions: peer.known_revisions.clone(),
    };
    let fetch = tokio::time::timeout(PEER_FETCH_TIMEOUT, source.fetch_manifests(&request)).await;
    let objects = match fetch {
        Err(_) => {
            let err = ExecutionError::S3TransientError(format!(
                "peer manifest fetch timed out after {PEER_FETCH_TIMEOUT:?}"
            ));
            record_peer_failure(pool, claim, peer.publisher, &err.to_string()).await?;
            warn!(
                target_id = claim.target_id,
                publisher = %peer.publisher,
                error = %err,
                "Peer manifest download failed"
            );
            return Ok(());
        }
        Ok(Ok(objects)) => objects,
        Ok(Err(err)) => {
            record_peer_failure(pool, claim, peer.publisher, &err.to_string()).await?;
            warn!(
                target_id = claim.target_id,
                publisher = %peer.publisher,
                error = %err,
                "Peer manifest download failed"
            );
            return Ok(());
        }
    };

    for object in objects {
        let mut trx = pool.begin().await?;
        require_active_lease(&mut trx, claim).await?;
        let stored = match store_authenticated_manifest(
            &mut trx,
            peer.publisher,
            &object.object_key,
            &object.signed_bytes,
        )
        .await
        {
            Ok(stored) => stored,
            Err(err) => {
                trx.rollback().await?;
                record_peer_failure(pool, claim, peer.publisher, &err.to_string()).await?;
                warn!(
                    target_id = claim.target_id,
                    publisher = %peer.publisher,
                    error = %err,
                    "Rejected downloaded peer manifest"
                );
                return Ok(());
            }
        };
        if !manifest_matches_scope(&stored.manifest.signed, &claim.scope) {
            let error = format!(
                "peer manifest {} does not match verification target {}",
                object.object_key, claim.target_id,
            );
            trx.rollback().await?;
            record_peer_failure(pool, claim, peer.publisher, &error).await?;
            return Ok(());
        }
        let revision = i64::try_from(stored.manifest.signed.payload.revision)
            .map_err(|_| internal("downloaded manifest revision exceeds BIGINT"))?;
        sqlx::query!(
            r#"
            UPDATE block_consensus_peer_download
               SET latest_revision = GREATEST(COALESCE(latest_revision, -1), $3),
                   last_attempt_at = NOW(),
                   updated_at = NOW()
             WHERE target_id = $1
               AND publisher = $2
            "#,
            claim.target_id,
            peer.publisher.as_slice(),
            revision,
        )
        .execute(trx.as_mut())
        .await?;
        trx.commit().await?;
    }

    let mut trx = pool.begin().await?;
    require_active_lease(&mut trx, claim).await?;
    sqlx::query!(
        r#"
        UPDATE block_consensus_peer_download
           SET completed_attempt = $3,
               last_attempt_at = NOW(),
               last_error = NULL,
               updated_at = NOW()
         WHERE target_id = $1
           AND publisher = $2
        "#,
        claim.target_id,
        peer.publisher.as_slice(),
        claim.attempt,
    )
    .execute(trx.as_mut())
    .await?;
    trx.commit().await?;
    Ok(())
}

async fn record_peer_failure(
    pool: &PgPool,
    claim: &VerificationClaim,
    publisher: Address,
    error: &str,
) -> Result<(), ExecutionError> {
    let mut trx = pool.begin().await?;
    require_active_lease(&mut trx, claim).await?;
    sqlx::query!(
        r#"
        UPDATE block_consensus_peer_download
           SET completed_attempt = $3,
               last_attempt_at = NOW(),
               last_error = $4,
               updated_at = NOW()
         WHERE target_id = $1
           AND publisher = $2
        "#,
        claim.target_id,
        publisher.as_slice(),
        claim.attempt,
        error,
    )
    .execute(trx.as_mut())
    .await?;
    trx.commit().await?;
    Ok(())
}

async fn require_active_lease(
    trx: &mut Transaction<'_, Postgres>,
    claim: &VerificationClaim,
) -> Result<(), ExecutionError> {
    let active = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1
              FROM block_consensus_verification_target
             WHERE id = $1
               AND state = 'leased'
               AND lease_owner = $2
               AND lease_expires_at > NOW()
             FOR UPDATE
        ) AS "active!"
        "#,
        claim.target_id,
        &claim.worker_id,
    )
    .fetch_one(trx.as_mut())
    .await?;
    if !active {
        return Err(internal(format!(
            "verification lease for target {} is no longer owned by {}",
            claim.target_id, claim.worker_id,
        )));
    }
    Ok(())
}

async fn finish_claim(
    pool: &PgPool,
    claim: &VerificationClaim,
) -> Result<VerificationRunResult, ExecutionError> {
    let mut trx = pool.begin().await?;
    require_active_lease(&mut trx, claim).await?;
    let local_reference = ManifestReference {
        block_number: U256::from(
            u64::try_from(claim.scope.publication_block_number)
                .map_err(|_| internal("publication block number is negative"))?,
        ),
        block_hash: claim.scope.publication_block_hash,
        revision: claim.scope.revision,
        manifest_digest: claim.scope.local_manifest_digest,
    };
    let local = load_manifest_by_reference(
        &mut trx,
        claim.scope.local_publisher,
        claim.scope.version,
        claim.scope.coprocessor_context_id,
        claim.scope.host_chain_id,
        &local_reference,
    )
    .await?
    .ok_or_else(|| internal("verification target local manifest is absent from archive"))?;
    let peer_rows = sqlx::query_scalar!(
        "SELECT publisher FROM block_consensus_peer_download WHERE target_id = $1",
        claim.target_id,
    )
    .fetch_all(trx.as_mut())
    .await?;
    let mut manifests = vec![local];
    for publisher in peer_rows {
        let publisher = address("peer download publisher", &publisher)?;
        if let Some(manifest) = load_tip_eligible_manifest(
            &mut trx,
            publisher,
            claim.scope.version,
            claim.scope.coprocessor_context_id,
            claim.scope.host_chain_id,
            claim.scope.publication_block_number,
            claim.scope.publication_block_hash,
        )
        .await?
        {
            manifests.push(manifest);
        }
    }
    let evaluation = evaluate_quorum(
        &manifests,
        claim.scope.local_publisher,
        claim.required_quorum,
    );
    apply_evaluation_to_drift_handles(
        &mut trx,
        claim.target_id,
        &manifests,
        claim.scope.local_publisher,
        &evaluation,
    )
    .await?;
    let attempt = claim.attempt;
    let target = sqlx::query!(
        "SELECT max_attempts, retry_delay_micros FROM block_consensus_verification_target WHERE id = $1",
        claim.target_id,
    )
    .fetch_one(trx.as_mut())
    .await?;
    let max_attempts = target.max_attempts;
    let retry_delay_micros = target.retry_delay_micros;
    // A drift result is immediately persisted and reportable, but the target
    // keeps its bounded retry budget so a newly published peer revision can
    // demonstrate remission. Only full consensus ends polling early.
    let terminal = evaluation.outcome == VerificationOutcome::Consensus;
    let state = if terminal {
        "complete"
    } else if attempt >= max_attempts {
        "exhausted"
    } else {
        "pending"
    };
    sqlx::query!(
        r#"
        UPDATE block_consensus_verification_target
           SET attempt_count = $3,
               state = $4,
               latest_outcome = $5,
               quorum_scope_count = $6,
               local_drift_scope_count = $7,
               last_attempt_at = NOW(),
               next_attempt_at = CASE
                   WHEN $4 = 'pending'
                   THEN NOW() + $8::BIGINT * INTERVAL '1 microsecond'
                   ELSE NULL
               END,
               lease_owner = NULL,
               lease_expires_at = NULL,
               last_error = NULL,
               updated_at = NOW()
         WHERE id = $1
           AND lease_owner = $2
        "#,
        claim.target_id,
        &claim.worker_id,
        attempt,
        state,
        evaluation.outcome.as_db_str(),
        evaluation.quorum_scope_count,
        evaluation.local_drift_scope_count,
        retry_delay_micros,
    )
    .execute(trx.as_mut())
    .await?;
    trx.commit().await?;
    Ok(VerificationRunResult {
        target_id: claim.target_id,
        attempt,
        outcome: evaluation.outcome,
    })
}

fn manifest_matches_scope(manifest: &SignedManifest, scope: &VerificationScope) -> bool {
    let payload = &manifest.payload;
    payload.version == scope.version
        && payload.coprocessor_context_id == scope.coprocessor_context_id
        && payload.host_chain_id
            == u64::try_from(scope.host_chain_id)
                .map(U256::from)
                .unwrap_or(U256::MAX)
        && payload.publication_block_number
            == u64::try_from(scope.publication_block_number)
                .map(U256::from)
                .unwrap_or(U256::MAX)
        && payload.publication_block_hash == scope.publication_block_hash
}

fn manifest_prefix(request: &PeerDownloadRequest) -> String {
    format!(
        "manifests/v{}/{}/{}/{}/{}/",
        u8::from(request.version),
        request.coprocessor_context_id,
        request.host_chain_id,
        request.publication_block_number,
        hex::encode(request.publication_block_hash),
    )
}

fn revision_below_prefix(key: &str, prefix: &str) -> Option<u64> {
    key.strip_prefix(prefix)?.parse().ok()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct S3BucketLocation {
    bucket: String,
    key_prefix: String,
}

impl S3BucketLocation {
    fn object_key(&self, suffix: String) -> String {
        if self.key_prefix.is_empty() {
            suffix
        } else {
            format!("{}/{suffix}", self.key_prefix)
        }
    }
}

fn s3_bucket_location(bucket_url: &str) -> Result<S3BucketLocation, ExecutionError> {
    let url = Url::parse(bucket_url)
        .map_err(|err| internal(format!("invalid peer S3 bucket URL {bucket_url}: {err}")))?;
    let host = url
        .host_str()
        .ok_or_else(|| internal(format!("peer S3 bucket URL {bucket_url} has no host")))?;
    let segments = url
        .path_segments()
        .map(|segments| {
            segments
                .filter(|segment| !segment.is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let first_host_label = host.split('.').next().unwrap_or_default();
    let virtual_hosted = url.scheme() == "s3"
        || host.contains(".s3.")
        || host.contains(".s3-")
        || host.ends_with(".s3.amazonaws.com");
    let (bucket, key_segments) = if virtual_hosted {
        (first_host_label, segments.as_slice())
    } else {
        let Some((bucket, key_segments)) = segments.split_first() else {
            return Err(internal(format!(
                "cannot determine bucket name from peer S3 URL {bucket_url}"
            )));
        };
        (*bucket, key_segments)
    };
    if bucket.is_empty() || bucket == "s3" {
        return Err(internal(format!(
            "cannot determine bucket name from peer S3 URL {bucket_url}"
        )));
    }
    Ok(S3BucketLocation {
        bucket: bucket.to_owned(),
        key_prefix: key_segments.join("/"),
    })
}

fn downloader_worker_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    format!("{}-{nanos}", std::process::id())
}

fn duration_micros(field: &str, duration: Duration) -> Result<i64, ExecutionError> {
    i64::try_from(duration.as_micros())
        .map_err(|_| internal(format!("{field} exceeds PostgreSQL interval precision")))
}

fn manifest_version(value: i16) -> Result<ManifestVersion, ExecutionError> {
    let value = u8::try_from(value).map_err(|_| internal("manifest version is outside uint8"))?;
    ManifestVersion::try_from(value)
        .map_err(|err| internal(format!("stored manifest version is invalid: {err}")))
}

fn i64_from_u256(field: &str, value: U256) -> Result<i64, ExecutionError> {
    i64::try_from(value).map_err(|_| internal(format!("{field} exceeds BIGINT")))
}

fn u256(field: &str, value: &[u8]) -> Result<U256, ExecutionError> {
    let bytes: [u8; 32] = value
        .try_into()
        .map_err(|_| internal(format!("{field} must be 32 bytes, got {}", value.len())))?;
    Ok(U256::from_be_bytes(bytes))
}

fn b256(field: &str, value: &[u8]) -> Result<B256, ExecutionError> {
    let bytes: [u8; 32] = value
        .try_into()
        .map_err(|_| internal(format!("{field} must be 32 bytes, got {}", value.len())))?;
    Ok(B256::from(bytes))
}

fn address(field: &str, value: &[u8]) -> Result<Address, ExecutionError> {
    let bytes: [u8; 20] = value
        .try_into()
        .map_err(|_| internal(format!("{field} must be 20 bytes, got {}", value.len())))?;
    Ok(Address::from(bytes))
}

fn internal(message: impl Into<String>) -> ExecutionError {
    ExecutionError::InternalError(message.into())
}

#[cfg(test)]
#[path = "peer_downloader_tests.rs"]
mod tests;
