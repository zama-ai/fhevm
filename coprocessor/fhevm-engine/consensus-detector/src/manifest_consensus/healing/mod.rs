//! Demand-driven local ct64 repair.
//!
//! The worker is started with publication and verification. It LISTENs on
//! `event_healing_work` and also polls on `healing_poll_interval` so a missed
//! NOTIFY still runs. Each pass takes up to `healing_batch_size` due
//! `can_be_healed` rows with `FOR UPDATE OF` `drifted_handle` only for that
//! pick, ordered by `drifted_handle_demand`. One row per handle: a reorg may
//! leave several findings, and they are healed together only when every
//! unhealed sibling has the same `quorum_ct64_digest`. Disagreeing targets
//! stay in the inventory and are not installed.
//! Demand lives on a separate table so TFHE EMA writes never lock these rows.
//! The pick lock is not held across S3. A matching GET installs the ct64 and
//! sets `healed_at` in one transaction. Every epoch with a schema is healed,
//! whichever stack runs the pass, and the ct64 is installed in the schema of
//! the finding's epoch: a live stack's own schema, or `public` for earlier
//! epochs merged at cutover. Failed epochs have no schema and are skipped. After a pass that installs at least
//! one handle, the worker NOTIFYs `work_available` once so idle TFHE does
//! not wait for its poll. Rows without a pin, and digest mismatches, HEAD
//! registry attestations: a live quorum pins the digest, evidence, and sources,
//! then GETs. A pinned digest that no longer matches the live quorum is
//! counted, never rewritten.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::time::Duration;

use alloy_primitives::{keccak256, Address, U256};
use aws_sdk_s3::Client;
use futures::future::join_all;
use sqlx::{postgres::PgListener, PgPool};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use fhevm_engine_common::gcs_activation::WORK_AVAILABLE_CHANNEL;
use fhevm_engine_common::pg_pool::is_fatal_connection_error;
use fhevm_engine_common::types::get_ct_type;
use fhevm_engine_common::CIPHERTEXT_VERSION;

use super::containment::{epoch_schemas, set_execution_schema};
use super::{ExecutionError, ManifestWorkGate};

mod download;
mod metrics;

use download::{Ct64Source, S3Ct64Source};

/// pg_notify channel emitted on `drifted_handle` insert/update.
pub(crate) const EVENT_HEALING_WORK: &str = "event_healing_work";

pub(crate) const DEFAULT_BATCH_SIZE: i64 = 8;
pub(crate) const DEFAULT_POLL_INTERVAL: Duration = Duration::from_secs(30);
/// Containment reduces propagation; verification still detects what it misses.
/// After this delay an uncontained ct64 finding is healed anyway.
pub(crate) const DEFAULT_CONTAINMENT_TIMEOUT: Duration = Duration::from_secs(5 * 60);
const RETRY_DELAY: Duration = Duration::from_secs(30);

struct DueHandle {
    id: i64,
    consensus_epoch: String,
    host_chain_id: i64,
    handle: Vec<u8>,
    coprocessor_context_id: Vec<u8>,
    quorum_ct64_digest: Option<Vec<u8>>,
    peer_sources: serde_json::Value,
    target_evidence: serde_json::Value,
    /// A ct64 finding past the containment timeout without being contained.
    uncontained: bool,
    /// Schema of the finding's epoch: the ct64 is installed there, not in the
    /// healer's own stack.
    schema: String,
    containment_timeout_secs: i64,
}

struct RegistryPeer {
    signer: Address,
    bucket_url: String,
    threshold: usize,
}

pub(crate) fn spawn_healing_worker(
    pool: PgPool,
    token: CancellationToken,
    client: Arc<Client>,
    work_gate: Arc<ManifestWorkGate>,
    batch_size: i64,
    poll_interval: Duration,
    containment_timeout: Duration,
) -> JoinHandle<Result<(), ExecutionError>> {
    tokio::spawn(run_healing_worker(
        pool,
        token,
        S3Ct64Source::new(client),
        work_gate,
        batch_size,
        poll_interval,
        containment_timeout,
    ))
}

async fn run_healing_worker<S: Ct64Source>(
    pool: PgPool,
    token: CancellationToken,
    source: S,
    work_gate: Arc<ManifestWorkGate>,
    batch_size: i64,
    poll_interval: Duration,
    containment_timeout: Duration,
) -> Result<(), ExecutionError> {
    info!("Healing worker enabled for this stack consensus_epoch");
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen(EVENT_HEALING_WORK).await?;
    if let Err(err) =
        run_healing_pass(&pool, &source, &work_gate, batch_size, containment_timeout).await
    {
        if let ExecutionError::DbError(db) = &err {
            if is_fatal_connection_error(db) {
                return Err(err);
            }
        }
        error!(error = %err, "Healing pass failed; retrying on the next wake");
    }
    loop {
        tokio::select! {
            _ = token.cancelled() => return Ok(()),
            recv = listener.recv() => {
                if let Err(err) = recv {
                    warn!(error = %err, "healing LISTEN recv error");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
            _ = tokio::time::sleep(poll_interval) => {}
        }
        match run_healing_pass(&pool, &source, &work_gate, batch_size, containment_timeout).await {
            Ok(()) => {}
            Err(ExecutionError::DbError(err)) if is_fatal_connection_error(&err) => {
                return Err(ExecutionError::DbError(err));
            }
            Err(err) => {
                error!(error = %err, "Healing pass failed; retrying on the next wake");
            }
        }
    }
}

async fn run_healing_pass<S: Ct64Source>(
    pool: &PgPool,
    source: &S,
    work_gate: &ManifestWorkGate,
    batch_size: i64,
    containment_timeout: Duration,
) -> Result<(), ExecutionError> {
    if work_gate.pinned_consensus_epoch().is_none() {
        return Ok(());
    }
    // Every epoch with a schema is healed, whichever stack runs this pass, so a
    // cutover does not strand the previous epoch's findings. Row locks and the
    // `healed_at IS NULL` guard coordinate healers of both stacks.
    let schemas = load_epoch_schemas(pool).await?;
    if schemas.is_empty() {
        return Ok(());
    }
    let due = lock_due(pool, &schemas, batch_size, containment_timeout).await?;
    if due.is_empty() {
        return Ok(());
    }
    let jobs = due.into_iter().map(|job| {
        let source = source.clone();
        let pool = pool.clone();
        async move { heal_one(&pool, &source, job).await }
    });
    let mut installed = false;
    for result in join_all(jobs).await {
        if result? {
            installed = true;
        }
    }
    if installed {
        notify_work_available(pool).await?;
    }
    Ok(())
}

async fn notify_work_available(pool: &PgPool) -> Result<(), ExecutionError> {
    sqlx::query!("SELECT pg_notify($1, '')", WORK_AVAILABLE_CHANNEL)
        .execute(pool)
        .await?;
    Ok(())
}

async fn load_epoch_schemas(pool: &PgPool) -> Result<HashMap<String, String>, ExecutionError> {
    let mut trx = pool.begin().await?;
    let schemas = epoch_schemas(&mut trx).await.map_err(from_containment)?;
    trx.rollback().await?;
    Ok(schemas)
}

/// Keeps database errors typed so fatal connection loss still stops the worker.
fn from_containment(error: anyhow::Error) -> ExecutionError {
    match error.downcast::<sqlx::Error>() {
        Ok(error) => ExecutionError::DbError(error),
        Err(error) => ExecutionError::InternalError(error.to_string()),
    }
}

async fn lock_due(
    pool: &PgPool,
    schemas: &HashMap<String, String>,
    batch_size: i64,
    containment_timeout: Duration,
) -> Result<Vec<DueHandle>, ExecutionError> {
    let epochs: Vec<String> = schemas.keys().cloned().collect();
    let containment_timeout_secs = i64::try_from(containment_timeout.as_secs())
        .map_err(|_| ExecutionError::InternalError("containment timeout exceeds BIGINT".into()))?;
    let mut trx = pool.begin().await?;
    let rows = sqlx::query!(
        r#"
        SELECT dh.id,
               dh.consensus_epoch,
               dh.handle,
               dh.host_chain_id,
               dh.coprocessor_context_id,
               dh.quorum_ct64_digest,
               dh.peer_sources::text AS "peer_sources!",
               dh.target_evidence::text AS "target_evidence?",
               (dh.reason = 'ct64_mismatch' AND NOT dh.is_contained) AS "uncontained!"
          FROM drifted_handle dh
          JOIN (
                SELECT DISTINCT ON (cand.host_chain_id, cand.coprocessor_context_id, cand.handle)
                       cand.id,
                       COALESCE(demand.tx_unlock_potential, 0) AS unlock
                  FROM drifted_handle cand
                  LEFT JOIN drifted_handle_demand demand
                    ON demand.handle = cand.handle
                 WHERE cand.healed_at IS NULL
                   AND cand.reason IN (
                        'ct64_mismatch', 'missing_here', 'error_here', 'uncomputed_here'
                   )
                   AND cand.consensus_epoch = ANY($1)
                   -- A healed ct64 root leaves containment's scan: wait until its
                   -- already-computed descendants are marked. Other reasons have
                   -- no wrong local ct64 for consumers to have read. Past the
                   -- timeout, heal anyway: verification detects what was missed.
                   AND (cand.reason <> 'ct64_mismatch' OR cand.is_contained
                        OR cand.detected_at <= NOW() - $3::BIGINT * INTERVAL '1 second')
                   AND (cand.next_retry_at IS NULL OR cand.next_retry_at <= NOW())
                   AND NOT EXISTS (
                        SELECT 1
                          FROM drifted_handle other
                         WHERE other.consensus_epoch = cand.consensus_epoch
                           AND other.coprocessor_context_id = cand.coprocessor_context_id
                           AND other.host_chain_id = cand.host_chain_id
                           AND other.handle = cand.handle
                           AND other.healed_at IS NULL
                           AND other.quorum_ct64_digest IS DISTINCT FROM cand.quorum_ct64_digest
                   )
                 ORDER BY cand.host_chain_id,
                          cand.coprocessor_context_id,
                          cand.handle,
                          cand.block_number ASC,
                          cand.id ASC
               ) picked ON picked.id = dh.id
         ORDER BY picked.unlock DESC, dh.block_number ASC, dh.id ASC
         LIMIT $2
         FOR UPDATE OF dh
        "#,
        &epochs,
        batch_size,
        containment_timeout_secs,
    )
    .fetch_all(trx.as_mut())
    .await?;
    trx.commit().await?;
    rows.into_iter()
        .map(|row| {
            let schema = schemas.get(&row.consensus_epoch).cloned().ok_or_else(|| {
                ExecutionError::InternalError(format!(
                    "no schema for healing epoch {}",
                    row.consensus_epoch
                ))
            })?;
            Ok(DueHandle {
                id: row.id,
                consensus_epoch: row.consensus_epoch,
                host_chain_id: row.host_chain_id,
                handle: row.handle,
                coprocessor_context_id: row.coprocessor_context_id,
                quorum_ct64_digest: row.quorum_ct64_digest,
                peer_sources: serde_json::from_str(&row.peer_sources)
                    .unwrap_or(serde_json::Value::Array(vec![])),
                target_evidence: row
                    .target_evidence
                    .as_deref()
                    .and_then(|json| serde_json::from_str(json).ok())
                    .unwrap_or(serde_json::Value::Null),
                uncontained: row.uncontained,
                containment_timeout_secs,
                schema,
            })
        })
        .collect()
}

async fn heal_one<S: Ct64Source>(
    pool: &PgPool,
    source: &S,
    job: DueHandle,
) -> Result<bool, ExecutionError> {
    let Ok(context_id) = u256_from_bytes(&job.coprocessor_context_id) else {
        return schedule_retry(pool, job.id).await;
    };
    let mut skip_buckets = Vec::new();
    if let Some(target) = job.quorum_ct64_digest.as_deref() {
        for bucket_url in peer_bucket_urls(&job.peer_sources) {
            match source.get_ct64(&bucket_url, &job.handle, context_id).await {
                Ok(bytes) if keccak256(&bytes).as_slice() == target => {
                    info!(
                        finding_id = job.id,
                        handle = hex::encode(&job.handle),
                        "Downloaded ct64 matching the pinned target"
                    );
                    return install_matching_ct64(pool, &job, &bytes, target).await;
                }
                Ok(_) => {
                    error!(
                        finding_id = job.id,
                        bucket_url,
                        handle = hex::encode(&job.handle),
                        "Peer ct64 digest does not match the pinned target; falling back to attestation quorum"
                    );
                    skip_buckets.push(bucket_url);
                    return recover_from_attestations(
                        pool,
                        source,
                        &job,
                        context_id,
                        &skip_buckets,
                    )
                    .await;
                }
                Err(ExecutionError::S3ObjectNotFound(_))
                | Err(ExecutionError::S3TransientError(_)) => {
                    skip_buckets.push(bucket_url);
                }
                Err(err) => return Err(err),
            }
        }
    }
    recover_from_attestations(pool, source, &job, context_id, &skip_buckets).await
}

async fn recover_from_attestations<S: Ct64Source>(
    pool: &PgPool,
    source: &S,
    job: &DueHandle,
    context_id: U256,
    skip_buckets: &[String],
) -> Result<bool, ExecutionError> {
    let peers = load_registry_peers(pool).await?;
    if peers.is_empty() {
        return schedule_retry(pool, job.id).await;
    }
    let threshold = pinned_threshold(&job.target_evidence).unwrap_or(peers[0].threshold);
    if threshold == 0 {
        return schedule_retry(pool, job.id).await;
    }
    let heads = peers.iter().map(|peer| {
        let source = source.clone();
        let bucket_url = peer.bucket_url.clone();
        let handle = job.handle.clone();
        let signer = peer.signer;
        async move {
            (
                bucket_url.clone(),
                source
                    .head_ct64_digest(&bucket_url, &handle, context_id, Some(signer))
                    .await,
            )
        }
    });
    let mut votes = Vec::new();
    for (bucket_url, result) in join_all(heads).await {
        match result {
            Ok(digest) => votes.push((bucket_url, digest.to_vec())),
            Err(err) => {
                warn!(
                    finding_id = job.id,
                    bucket_url,
                    error = %err,
                    "Skipping peer without a usable ct64 attestation"
                );
            }
        }
    }
    let Some((digest, buckets)) = majority_digest(&votes, threshold) else {
        warn!(
            finding_id = job.id,
            handle = hex::encode(&job.handle),
            votes = votes.len(),
            threshold,
            "No attestation quorum for this handle yet"
        );
        return schedule_retry(pool, job.id).await;
    };
    if let Some(pinned) = job.quorum_ct64_digest.as_deref() {
        if digest != pinned {
            error!(
                finding_id = job.id,
                handle = hex::encode(&job.handle),
                "Attestation quorum digest differs from the pinned healing target"
            );
            metrics::QUORUM_CHANGED
                .with_label_values(&[&job.consensus_epoch])
                .inc();
            return schedule_retry(pool, job.id).await;
        }
    } else {
        persist_live_quorum(pool, job, &digest, &buckets, &peers, threshold).await?;
    }
    for bucket_url in buckets
        .iter()
        .filter(|bucket| !skip_buckets.iter().any(|skipped| skipped == *bucket))
    {
        match source.get_ct64(bucket_url, &job.handle, context_id).await {
            Ok(bytes) if keccak256(&bytes).as_slice() == digest => {
                info!(
                    finding_id = job.id,
                    handle = hex::encode(&job.handle),
                    bucket_url,
                    "Downloaded ct64 matching the pinned target from attestation quorum"
                );
                return install_matching_ct64(pool, job, &bytes, &digest).await;
            }
            Ok(_) => {
                error!(
                    finding_id = job.id,
                    bucket_url, "Quorum peer ct64 body does not match its attested digest"
                );
            }
            Err(ExecutionError::S3ObjectNotFound(_)) | Err(ExecutionError::S3TransientError(_)) => {
            }
            Err(err) => return Err(err),
        }
    }
    error!(
        finding_id = job.id,
        handle = hex::encode(&job.handle),
        "Pinned ct64 target still has quorum but no peer supplied matching bytes"
    );
    metrics::BAD_TARGET_DIGEST
        .with_label_values(&[&job.consensus_epoch])
        .inc();
    schedule_retry(pool, job.id).await
}

async fn load_registry_peers(pool: &PgPool) -> Result<Vec<RegistryPeer>, ExecutionError> {
    let rows = sqlx::query!(
        r#"
        SELECT signer_address,
               s3_bucket_url,
               coprocessor_threshold
          FROM gateway_config_coprocessors
         WHERE s3_bucket_url <> ''
         ORDER BY signer_address
        "#,
    )
    .fetch_all(pool)
    .await?;
    let mut peers = Vec::with_capacity(rows.len());
    for row in rows {
        let Ok(signer) = <[u8; 20]>::try_from(row.signer_address.as_slice()) else {
            continue;
        };
        let threshold = usize::try_from(row.coprocessor_threshold).unwrap_or(0);
        peers.push(RegistryPeer {
            signer: Address::from(signer),
            bucket_url: row.s3_bucket_url,
            threshold,
        });
    }
    Ok(peers)
}

async fn persist_live_quorum(
    pool: &PgPool,
    job: &DueHandle,
    digest: &[u8],
    buckets: &[String],
    peers: &[RegistryPeer],
    threshold: usize,
) -> Result<(), ExecutionError> {
    let sources = live_peer_sources(peers, buckets);
    let evidence = live_target_evidence(peers, buckets, digest, threshold);
    sqlx::query!(
        r#"
        UPDATE drifted_handle
           SET quorum_ct64_digest = COALESCE(quorum_ct64_digest, $2),
               target_evidence = COALESCE(target_evidence, $3::jsonb),
               peer_sources = CASE
                 WHEN peer_sources = '[]'::jsonb THEN $4::jsonb
                 ELSE peer_sources
               END
         WHERE id = $1
           AND healed_at IS NULL
           AND (quorum_ct64_digest IS NULL OR quorum_ct64_digest = $2)
        "#,
        job.id,
        digest,
        evidence.as_str(),
        sources.as_str(),
    )
    .execute(pool)
    .await?;
    Ok(())
}

fn live_peer_sources(peers: &[RegistryPeer], buckets: &[String]) -> String {
    let entries: Vec<serde_json::Value> = buckets
        .iter()
        .filter_map(|bucket| {
            peers
                .iter()
                .find(|peer| peer.bucket_url == *bucket)
                .map(|peer| {
                    serde_json::json!({
                        "publisher": peer.signer.to_string(),
                        "s3_bucket_url": bucket,
                    })
                })
        })
        .collect();
    serde_json::to_string(&entries).expect("peer_sources is a JSON array")
}

fn live_target_evidence(
    peers: &[RegistryPeer],
    buckets: &[String],
    digest: &[u8],
    threshold: usize,
) -> String {
    let digest_hex = alloy_primitives::B256::try_from(digest)
        .map(|digest| digest.to_string())
        .unwrap_or_default();
    let statements: Vec<serde_json::Value> = buckets
        .iter()
        .filter_map(|bucket| {
            peers
                .iter()
                .find(|peer| peer.bucket_url == *bucket)
                .map(|peer| {
                    serde_json::json!({
                        "publisher": peer.signer.to_string(),
                        "ct64_digest": digest_hex,
                    })
                })
        })
        .collect();
    serde_json::to_string(&serde_json::json!({
        "required_quorum": threshold,
        "registered_coprocessor_count": peers.len(),
        "source": "attestation",
        "statements": statements,
    }))
    .expect("target_evidence is a JSON object")
}

fn pinned_threshold(evidence: &serde_json::Value) -> Option<usize> {
    evidence
        .get("required_quorum")
        .and_then(serde_json::Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .filter(|value| *value > 0)
}

fn majority_digest(
    votes: &[(String, Vec<u8>)],
    threshold: usize,
) -> Option<(Vec<u8>, Vec<String>)> {
    let mut groups: BTreeMap<&[u8], Vec<String>> = BTreeMap::new();
    for (bucket, digest) in votes {
        groups
            .entry(digest.as_slice())
            .or_default()
            .push(bucket.clone());
    }
    groups
        .into_iter()
        .max_by(|(left_digest, left), (right_digest, right)| {
            left.len()
                .cmp(&right.len())
                .then_with(|| right_digest.cmp(left_digest))
        })
        .and_then(|(digest, buckets)| {
            (buckets.len() >= threshold).then(|| (digest.to_vec(), buckets))
        })
}

async fn install_matching_ct64(
    pool: &PgPool,
    job: &DueHandle,
    bytes: &[u8],
    digest: &[u8],
) -> Result<bool, ExecutionError> {
    let Ok(ciphertext_type) = get_ct_type(&job.handle) else {
        return schedule_retry(pool, job.id).await;
    };
    let mut trx = pool.begin().await?;
    // `ciphertexts` and `computations` below resolve to the finding's stack;
    // `drifted_handle` exists only in public.
    set_execution_schema(&mut trx, &job.schema)
        .await
        .map_err(from_containment)?;
    let marked = sqlx::query!(
        r#"
        UPDATE drifted_handle
           SET healed_at = NOW(),
               claimed_by = NULL,
               claim_expires_at = NULL,
               next_retry_at = NULL
         WHERE consensus_epoch = $1
           AND coprocessor_context_id = $2
           AND host_chain_id = $3
           AND handle = $4
           AND healed_at IS NULL
           AND can_be_healed
           AND (reason <> 'ct64_mismatch' OR is_contained
                OR detected_at <= NOW() - $6::BIGINT * INTERVAL '1 second')
           AND quorum_ct64_digest = $5
        "#,
        job.consensus_epoch,
        &job.coprocessor_context_id,
        job.host_chain_id,
        &job.handle,
        digest,
        job.containment_timeout_secs,
    )
    .execute(trx.as_mut())
    .await?;
    if marked.rows_affected() == 0 {
        trx.rollback().await?;
        return Ok(false);
    }
    sqlx::query!(
        r#"
        INSERT INTO ciphertexts (
            handle, ciphertext, ciphertext_version, ciphertext_type
        ) VALUES ($1, $2, $3, $4)
        ON CONFLICT (handle, ciphertext_version) DO UPDATE
        SET ciphertext = EXCLUDED.ciphertext
        WHERE ciphertexts.ciphertext IS DISTINCT FROM EXCLUDED.ciphertext
        "#,
        &job.handle,
        bytes,
        CIPHERTEXT_VERSION,
        ciphertext_type,
    )
    .execute(trx.as_mut())
    .await?;
    // Same rule as the TFHE upload path: stored bytes are the ground truth of
    // success, so a pending or errored row for this handle is now completed.
    sqlx::query!(
        r#"
        UPDATE computations
           SET is_completed = true,
               completed_at = CURRENT_TIMESTAMP,
               is_error = false,
               error_message = NULL,
               error_retry_count = 0
         WHERE output_handle = $1
           AND is_completed = false
        "#,
        &job.handle,
    )
    .execute(trx.as_mut())
    .await?;
    trx.commit().await?;
    if job.uncontained {
        metrics::HEALED_UNCONTAINED
            .with_label_values(&[&job.consensus_epoch])
            .inc();
        warn!(
            finding_id = job.id,
            handle = hex::encode(&job.handle),
            "Healed uncontained ct64 drift after the containment timeout; verification must catch missed descendants"
        );
    }
    Ok(true)
}

async fn schedule_retry(pool: &PgPool, id: i64) -> Result<bool, ExecutionError> {
    let retry_secs = i64::try_from(RETRY_DELAY.as_secs()).unwrap_or(30);
    sqlx::query!(
        r#"
        UPDATE drifted_handle
           SET next_retry_at = NOW() + ($2::BIGINT * INTERVAL '1 second')
         WHERE id = $1
        "#,
        id,
        retry_secs,
    )
    .execute(pool)
    .await?;
    Ok(false)
}

fn peer_bucket_urls(peer_sources: &serde_json::Value) -> Vec<String> {
    peer_sources
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            entry
                .get("s3_bucket_url")
                .and_then(serde_json::Value::as_str)
                .filter(|url| !url.is_empty())
                .map(str::to_owned)
        })
        .collect()
}

fn u256_from_bytes(value: &[u8]) -> Result<U256, ExecutionError> {
    let bytes: [u8; 32] = value.try_into().map_err(|_| {
        ExecutionError::InternalError(format!(
            "coprocessor_context_id must be 32 bytes, got {}",
            value.len()
        ))
    })?;
    Ok(U256::from_be_bytes(bytes))
}

#[cfg(test)]
#[path = "scheduler_tests.rs"]
mod scheduler_tests;
