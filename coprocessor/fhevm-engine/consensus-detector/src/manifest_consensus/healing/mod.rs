//! Demand-driven local ct64 repair.
//!
//! The worker is started with publication and verification. It LISTENs on
//! `event_healing_work` and also polls on `healing_poll_interval` so a missed
//! NOTIFY still runs. Each pass takes up to `healing_batch_size` due
//! `can_be_healed` rows with `FOR UPDATE OF` `drifted_handle` only for that
//! pick, ordered by `drifted_handle_demand`. The handle is the unit of repair:
//! a reorg may leave several findings for one handle, and one install heals
//! them all. Only active siblings (healable reason, unhealed, not abandoned)
//! take part. A sibling without a target adopts the single target of the
//! others, so every sibling shares one target. Two active siblings pinned to
//! different digests are a real conflict: the handle is not installed and is
//! reported, since a pinned target is never rewritten.
//! Demand lives on a separate table so TFHE EMA writes never lock these rows.
//! The pick lock is not held across S3. A matching GET installs the ct64 and
//! sets `healed_at` in one transaction, which holds the shared cutover lock and
//! resolves the epoch's schema again, so a cutover or rollback during the GET
//! drops the job instead of writing into a replaced schema. Every epoch with a
//! schema is healed, whichever stack runs the pass, and the ct64 is installed
//! in the schema of the finding's epoch: a live stack's own schema, or `public`
//! for earlier epochs merged at cutover. Failed epochs have no schema and are
//! skipped. After a pass that installs at least one handle, the worker NOTIFYs `work_available` once so idle TFHE does
//! not wait for its poll. Rows without a pin, and digest mismatches, HEAD
//! registry attestations: a live quorum pins the digest, evidence, and sources
//! on every active sibling without a target, then GETs. A pinned digest that
//! no longer matches the live quorum is counted, never rewritten.
//!
//! S3 attestations carry no epoch, so they set a target only for a block whose
//! objects the finding's epoch uploaded (`upload_start_block`): never during a
//! dry run, when the Green uploader is parked. Each vote must also be tagged
//! with the finding's epoch by its uploader (`S3_METADATA_CONSENSUS_EPOCH_KEY`),
//! which covers peers that cut over later; an untagged object is `legacy`. A finding superseded by a later
//! epoch's cutover (`drift_superseded`) describes bytes that are gone: it is
//! marked `superseded_at` and never installed, also when the cutover lands
//! during its download.

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

use super::containment::{epoch_schemas, lock_cutover, set_execution_schema};
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
/// Failed attempts before a finding is abandoned: about one hour at
/// `RETRY_DELAY`.
pub(crate) const DEFAULT_MAX_ATTEMPTS: i32 = 120;
const RETRY_DELAY: Duration = Duration::from_secs(30);

/// Healing worker tuning, from the `--manifest-healing-*` arguments.
#[derive(Clone, Copy, Debug)]
pub(crate) struct HealingSettings {
    pub batch_size: i64,
    pub poll_interval: Duration,
    pub containment_timeout: Duration,
    pub max_attempts: i32,
}

impl Default for HealingSettings {
    fn default() -> Self {
        Self {
            batch_size: DEFAULT_BATCH_SIZE,
            poll_interval: DEFAULT_POLL_INTERVAL,
            containment_timeout: DEFAULT_CONTAINMENT_TIMEOUT,
            max_attempts: DEFAULT_MAX_ATTEMPTS,
        }
    }
}

struct DueHandle {
    id: i64,
    consensus_epoch: String,
    host_chain_id: i64,
    /// Decides whether an S3 attestation can be this epoch's evidence.
    block_number: i64,
    /// Block the finding was verified in: a bridged destination is resolved
    /// to its source in this block.
    block_hash: Vec<u8>,
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
    max_attempts: i32,
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
    settings: HealingSettings,
) -> JoinHandle<Result<(), ExecutionError>> {
    tokio::spawn(run_healing_worker(
        pool,
        token,
        S3Ct64Source::new(client),
        work_gate,
        settings,
    ))
}

async fn run_healing_worker<S: Ct64Source>(
    pool: PgPool,
    token: CancellationToken,
    source: S,
    work_gate: Arc<ManifestWorkGate>,
    settings: HealingSettings,
) -> Result<(), ExecutionError> {
    let HealingSettings {
        batch_size,
        poll_interval,
        containment_timeout,
        max_attempts,
    } = settings;
    info!("Healing worker enabled for this stack consensus_epoch");
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen(EVENT_HEALING_WORK).await?;
    if let Err(err) = run_healing_pass(
        &pool,
        &source,
        &work_gate,
        batch_size,
        containment_timeout,
        max_attempts,
    )
    .await
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
        match run_healing_pass(
            &pool,
            &source,
            &work_gate,
            batch_size,
            containment_timeout,
            max_attempts,
        )
        .await
        {
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
    max_attempts: i32,
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
    report_conflicting_targets(pool, &schemas).await?;
    let due = lock_due(
        pool,
        &schemas,
        batch_size,
        containment_timeout,
        max_attempts,
    )
    .await?;
    if due.is_empty() {
        return Ok(());
    }
    let jobs = due.into_iter().map(|job| {
        let source = source.clone();
        let pool = pool.clone();
        let consensus_epoch = job.consensus_epoch.clone();
        async move {
            heal_one(&pool, &source, job).await.inspect_err(|_| {
                metrics::ATTEMPTS
                    .with_label_values(&[&consensus_epoch, metrics::TRANSIENT_FAILURE])
                    .inc();
            })
        }
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
    max_attempts: i32,
) -> Result<Vec<DueHandle>, ExecutionError> {
    let epochs: Vec<String> = schemas.keys().cloned().collect();
    let containment_timeout_secs = i64::try_from(containment_timeout.as_secs())
        .map_err(|_| ExecutionError::InternalError("containment timeout exceeds BIGINT".into()))?;
    let mut trx = pool.begin().await?;
    mark_superseded(&mut trx, &epochs, None).await?;
    align_sibling_targets(&mut trx, &epochs).await?;
    let rows = sqlx::query!(
        r#"
        SELECT dh.id,
               dh.consensus_epoch,
               dh.handle,
               dh.host_chain_id,
               dh.block_number,
               dh.block_hash,
               dh.coprocessor_context_id,
               dh.quorum_ct64_digest,
               dh.peer_sources,
               dh.target_evidence,
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
                   AND cand.heal_abandoned_at IS NULL
                   AND cand.superseded_at IS NULL
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
                   -- Only an active sibling pinned to another digest blocks the
                   -- handle: after alignment, a sibling without a target has
                   -- none to disagree with, and abandoned or unhealable rows
                   -- no longer decide.
                   AND NOT EXISTS (
                        SELECT 1
                          FROM drifted_handle other
                         WHERE other.consensus_epoch = cand.consensus_epoch
                           AND other.coprocessor_context_id = cand.coprocessor_context_id
                           AND other.host_chain_id = cand.host_chain_id
                           AND other.handle = cand.handle
                           AND other.healed_at IS NULL
                           AND other.heal_abandoned_at IS NULL
                           AND other.superseded_at IS NULL
                           AND other.reason IN (
                                'ct64_mismatch', 'missing_here', 'error_here', 'uncomputed_here'
                           )
                           AND other.quorum_ct64_digest IS NOT NULL
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
                block_number: row.block_number,
                block_hash: row.block_hash,
                handle: row.handle,
                coprocessor_context_id: row.coprocessor_context_id,
                quorum_ct64_digest: row.quorum_ct64_digest,
                peer_sources: row.peer_sources,
                target_evidence: row.target_evidence.unwrap_or(serde_json::Value::Null),
                uncontained: row.uncontained,
                containment_timeout_secs,
                max_attempts,
                schema,
            })
        })
        .collect()
}

/// Marks unhealed findings superseded by a later epoch's cutover, for every
/// sibling of a handle at once: one sibling in a replaced window means the
/// stored ct64 is no longer the one these findings describe. `handle` limits
/// the sweep to one handle. Returns the rows marked.
async fn mark_superseded(
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    epochs: &[String],
    handle: Option<&[u8]>,
) -> Result<u64, ExecutionError> {
    let marked = sqlx::query!(
        r#"
        UPDATE drifted_handle dh
           SET superseded_at = NOW(),
               claimed_by = NULL,
               claim_expires_at = NULL,
               next_retry_at = NULL
         WHERE dh.consensus_epoch = ANY($1)
           AND ($2::BYTEA IS NULL OR dh.handle = $2)
           AND dh.healed_at IS NULL
           AND dh.superseded_at IS NULL
           AND EXISTS (
                SELECT 1
                  FROM drifted_handle sibling
                 WHERE sibling.consensus_epoch = dh.consensus_epoch
                   AND sibling.coprocessor_context_id = dh.coprocessor_context_id
                   AND sibling.host_chain_id = dh.host_chain_id
                   AND sibling.handle = dh.handle
                   AND sibling.healed_at IS NULL
                   AND public.drift_superseded(
                        sibling.consensus_epoch, sibling.host_chain_id, sibling.block_number
                   )
           )
        "#,
        epochs,
        handle,
    )
    .execute(trx.as_mut())
    .await?;
    Ok(marked.rows_affected())
}

/// Gives every active sibling without a target the single target of the other
/// active siblings of its handle, so one install heals them all. Abandoned
/// siblings are filled too, so the install also marks them. A handle whose
/// siblings are pinned to several digests is left alone: that conflict is
/// reported, never resolved by rewriting a pin.
async fn align_sibling_targets(
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    epochs: &[String],
) -> Result<(), ExecutionError> {
    sqlx::query!(
        r#"
        WITH active AS (
            SELECT id, consensus_epoch, coprocessor_context_id, host_chain_id, handle,
                   block_number, quorum_ct64_digest, target_evidence, peer_sources
              FROM drifted_handle
             WHERE consensus_epoch = ANY($1)
               AND healed_at IS NULL
               AND heal_abandoned_at IS NULL
               AND superseded_at IS NULL
               AND reason IN ('ct64_mismatch', 'missing_here', 'error_here', 'uncomputed_here')
               AND quorum_ct64_digest IS NOT NULL
        ),
        single_target AS (
            SELECT consensus_epoch, coprocessor_context_id, host_chain_id, handle
              FROM active
             GROUP BY consensus_epoch, coprocessor_context_id, host_chain_id, handle
            HAVING COUNT(DISTINCT quorum_ct64_digest) = 1
        ),
        pinned AS (
            SELECT DISTINCT ON (a.consensus_epoch, a.coprocessor_context_id, a.host_chain_id, a.handle)
                   a.consensus_epoch, a.coprocessor_context_id, a.host_chain_id, a.handle,
                   a.quorum_ct64_digest, a.target_evidence, a.peer_sources
              FROM active a
              JOIN single_target t
                ON t.consensus_epoch = a.consensus_epoch
               AND t.coprocessor_context_id = a.coprocessor_context_id
               AND t.host_chain_id = a.host_chain_id
               AND t.handle = a.handle
             ORDER BY a.consensus_epoch, a.coprocessor_context_id, a.host_chain_id, a.handle,
                      a.block_number ASC, a.id ASC
        )
        UPDATE drifted_handle dh
           SET quorum_ct64_digest = pinned.quorum_ct64_digest,
               target_evidence = COALESCE(dh.target_evidence, pinned.target_evidence),
               peer_sources = CASE
                 WHEN dh.peer_sources = '[]'::jsonb THEN pinned.peer_sources
                 ELSE dh.peer_sources
               END
          FROM pinned
         WHERE dh.consensus_epoch = pinned.consensus_epoch
           AND dh.coprocessor_context_id = pinned.coprocessor_context_id
           AND dh.host_chain_id = pinned.host_chain_id
           AND dh.handle = pinned.handle
           AND dh.healed_at IS NULL
           AND dh.superseded_at IS NULL
           AND dh.reason IN ('ct64_mismatch', 'missing_here', 'error_here', 'uncomputed_here')
           AND dh.quorum_ct64_digest IS NULL
        "#,
        epochs,
    )
    .execute(trx.as_mut())
    .await?;
    Ok(())
}

/// Handles whose active siblings are pinned to different digests cannot be
/// healed without choosing a target against a pin; they stay frozen until an
/// operator resolves them.
async fn report_conflicting_targets(
    pool: &PgPool,
    schemas: &HashMap<String, String>,
) -> Result<(), ExecutionError> {
    let epochs: Vec<String> = schemas.keys().cloned().collect();
    let rows = sqlx::query!(
        r#"
        SELECT consensus_epoch, host_chain_id, handle
          FROM drifted_handle
         WHERE consensus_epoch = ANY($1)
           AND healed_at IS NULL
           AND heal_abandoned_at IS NULL
           AND superseded_at IS NULL
           AND reason IN ('ct64_mismatch', 'missing_here', 'error_here', 'uncomputed_here')
           AND quorum_ct64_digest IS NOT NULL
         GROUP BY consensus_epoch, coprocessor_context_id, host_chain_id, handle
        HAVING COUNT(DISTINCT quorum_ct64_digest) > 1
        "#,
        &epochs,
    )
    .fetch_all(pool)
    .await?;
    let mut per_epoch: HashMap<&str, i64> =
        epochs.iter().map(|epoch| (epoch.as_str(), 0)).collect();
    for row in &rows {
        *per_epoch.entry(row.consensus_epoch.as_str()).or_default() += 1;
    }
    for (epoch, count) in per_epoch {
        metrics::CONFLICTING_TARGETS
            .with_label_values(&[epoch])
            .set(count);
    }
    if let Some(first) = rows.first() {
        error!(
            conflicts = rows.len(),
            consensus_epoch = first.consensus_epoch,
            host_chain_id = first.host_chain_id,
            handle = hex::encode(&first.handle),
            "Unhealed findings of one handle are pinned to different ct64 targets; not healing them"
        );
    }
    Ok(())
}

async fn heal_one<S: Ct64Source>(
    pool: &PgPool,
    source: &S,
    job: DueHandle,
) -> Result<bool, ExecutionError> {
    let Ok(context_id) = u256_from_bytes(&job.coprocessor_context_id) else {
        error!(
            finding_id = job.id,
            "Finding has an invalid coprocessor context id"
        );
        return schedule_retry(pool, &job, Failure::Terminal).await;
    };
    if let Some(source_handle) = bridged_source(pool, &job).await? {
        if let Some((bytes, digest)) =
            fetch_from_bridged_source(pool, source, &job, context_id, &source_handle).await?
        {
            info!(
                finding_id = job.id,
                handle = hex::encode(&job.handle),
                source_handle = hex::encode(&source_handle),
                "Downloaded bridged ct64 from its source objects"
            );
            return install_matching_ct64(pool, &job, &bytes, &digest).await;
        }
    }
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
                Err(err @ ExecutionError::DbError(_)) => return Err(err),
                Err(err) => {
                    warn!(
                        finding_id = job.id,
                        bucket_url,
                        error = %err,
                        "Skipping peer bucket that did not serve the ct64"
                    );
                    skip_buckets.push(bucket_url);
                }
            }
        }
    }
    recover_from_attestations(pool, source, &job, context_id, &skip_buckets).await
}

/// Source handle when the finding is a `HandleBridged` destination of its
/// block. The copy reuses the source's S3 objects, so peers serve its bytes
/// under the source key, not under its own.
async fn bridged_source(pool: &PgPool, job: &DueHandle) -> Result<Option<Vec<u8>>, ExecutionError> {
    let mut trx = pool.begin().await?;
    // Bridge events are per stack: read the finding's epoch.
    set_execution_schema(&mut trx, &job.schema)
        .await
        .map_err(from_containment)?;
    let source_handle = sqlx::query_scalar!(
        r#"SELECT src_handle
             FROM handle_bridged_events
            WHERE dst_handle = $1
              AND dst_chain_id = $2
              AND block_hash = $3"#,
        &job.handle,
        job.host_chain_id,
        &job.block_hash,
    )
    .fetch_optional(trx.as_mut())
    .await?;
    trx.rollback().await?;
    Ok(source_handle)
}

/// Bytes for a bridged destination from the peers' source objects: the pinned
/// sources first, then a live attestation quorum on the source handle. `None`
/// sends the finding down the normal path, where a destination that a fallback
/// grant materialized is served under its own key. Failures here charge no
/// attempt; the normal path charges one.
async fn fetch_from_bridged_source<S: Ct64Source>(
    pool: &PgPool,
    source: &S,
    job: &DueHandle,
    context_id: U256,
    source_handle: &[u8],
) -> Result<Option<(Vec<u8>, Vec<u8>)>, ExecutionError> {
    let pinned = job.quorum_ct64_digest.as_deref();
    let mut tried = Vec::new();
    if let Some(target) = pinned {
        for bucket_url in peer_bucket_urls(&job.peer_sources) {
            match source
                .get_ct64(&bucket_url, source_handle, context_id)
                .await
            {
                Ok(bytes) if keccak256(&bytes).as_slice() == target => {
                    return Ok(Some((bytes, target.to_vec())));
                }
                Ok(_) => {}
                Err(err @ ExecutionError::DbError(_)) => return Err(err),
                Err(err) => warn!(
                    finding_id = job.id,
                    bucket_url,
                    error = %err,
                    "Skipping peer bucket that did not serve the bridged source ct64"
                ),
            }
            tried.push(bucket_url);
        }
    }
    // Without a pin this quorum sets the target, so it must be the finding's
    // epoch's evidence, judged on the destination's window; the normal path
    // then applies the same rule.
    if pinned.is_none()
        && !matches!(
            attestation_scope(pool, job).await?,
            AttestationScope::Uploaded
        )
    {
        return Ok(None);
    }
    let peers = load_registry_peers(pool).await?;
    let Some(first) = peers.first() else {
        return Ok(None);
    };
    let threshold = pinned_threshold(&job.target_evidence).unwrap_or(first.threshold);
    if threshold == 0 {
        return Ok(None);
    }
    let heads = peers.iter().map(|peer| {
        let source = source.clone();
        let bucket_url = peer.bucket_url.clone();
        let source_handle = source_handle.to_vec();
        let signer = peer.signer;
        async move {
            let attested = source
                .head_ct64_digest(&bucket_url, &source_handle, context_id, Some(signer))
                .await;
            (bucket_url, attested)
        }
    });
    let votes: Vec<(String, Vec<u8>)> = join_all(heads)
        .await
        .into_iter()
        .filter_map(|(bucket_url, attested)| {
            let attested = attested.ok()?;
            (attested.consensus_epoch == job.consensus_epoch)
                .then(|| (bucket_url, attested.digest.to_vec()))
        })
        .collect();
    let Some((digest, buckets)) = majority_digest(&votes, threshold) else {
        return Ok(None);
    };
    if pinned.is_some_and(|target| target != digest.as_slice()) {
        return Ok(None);
    }
    for bucket_url in buckets.iter().filter(|bucket| !tried.contains(bucket)) {
        match source.get_ct64(bucket_url, source_handle, context_id).await {
            Ok(bytes) if keccak256(&bytes).as_slice() == digest.as_slice() => {
                if pinned.is_none() {
                    persist_live_quorum(pool, job, &digest, &buckets, &peers, threshold).await?;
                }
                return Ok(Some((bytes, digest)));
            }
            Ok(_) => {}
            Err(err @ ExecutionError::DbError(_)) => return Err(err),
            Err(err) => warn!(
                finding_id = job.id,
                bucket_url,
                error = %err,
                "Skipping quorum peer that did not serve the bridged source ct64"
            ),
        }
    }
    Ok(None)
}

async fn recover_from_attestations<S: Ct64Source>(
    pool: &PgPool,
    source: &S,
    job: &DueHandle,
    context_id: U256,
    skip_buckets: &[String],
) -> Result<bool, ExecutionError> {
    match attestation_scope(pool, job).await? {
        AttestationScope::Uploaded => {}
        AttestationScope::Parked => {
            info!(
                finding_id = job.id,
                consensus_epoch = job.consensus_epoch,
                "Epoch's stack uploads nothing before cutover; its S3 attestations are another epoch's"
            );
            return defer_uncharged(pool, job).await;
        }
        AttestationScope::BeforeUpload(upload_start_block) => {
            warn!(
                finding_id = job.id,
                consensus_epoch = job.consensus_epoch,
                block_number = job.block_number,
                upload_start_block,
                "Block's S3 objects were uploaded by the previous epoch; no attestation target"
            );
            return schedule_retry(pool, job, Failure::Transient).await;
        }
    }
    let peers = load_registry_peers(pool).await?;
    if peers.is_empty() {
        warn!(
            finding_id = job.id,
            "Coprocessor registry is empty; healing waits for gw-listener"
        );
        return schedule_retry(pool, job, Failure::Transient).await;
    }
    let threshold = pinned_threshold(&job.target_evidence).unwrap_or(peers[0].threshold);
    if threshold == 0 {
        warn!(
            finding_id = job.id,
            "Coprocessor threshold is 0; no attestation quorum is possible"
        );
        return schedule_retry(pool, job, Failure::Transient).await;
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
            // The S3 key and attestation carry no epoch: another epoch's
            // object is no evidence for this finding.
            Ok(attested) if attested.consensus_epoch != job.consensus_epoch => {
                warn!(
                    finding_id = job.id,
                    bucket_url,
                    uploaded_by = attested.consensus_epoch,
                    "Skipping peer whose ct64 was uploaded by another epoch"
                );
            }
            Ok(attested) => votes.push((bucket_url, attested.digest.to_vec())),
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
        return schedule_retry(pool, job, Failure::Transient).await;
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
            return schedule_retry(pool, job, Failure::Transient).await;
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
            Err(err @ ExecutionError::DbError(_)) => return Err(err),
            Err(err) => {
                warn!(
                    finding_id = job.id,
                    bucket_url,
                    error = %err,
                    "Skipping quorum peer that did not serve the ct64"
                );
            }
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
    schedule_retry(pool, job, Failure::Transient).await
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
    // Pin the whole handle, not only this row: a sibling left without a target
    // would neither share this attempt's accounting nor be marked by the
    // install.
    sqlx::query!(
        r#"
        UPDATE drifted_handle
           SET quorum_ct64_digest = COALESCE(quorum_ct64_digest, $1),
               target_evidence = COALESCE(target_evidence, $2),
               peer_sources = CASE
                 WHEN peer_sources = '[]'::jsonb THEN $3
                 ELSE peer_sources
               END
         WHERE consensus_epoch = $4
           AND coprocessor_context_id = $5
           AND host_chain_id = $6
           AND handle = $7
           AND healed_at IS NULL
           AND superseded_at IS NULL
           AND reason IN ('ct64_mismatch', 'missing_here', 'error_here', 'uncomputed_here')
           AND (quorum_ct64_digest IS NULL OR quorum_ct64_digest = $1)
        "#,
        digest,
        evidence,
        sources,
        job.consensus_epoch,
        &job.coprocessor_context_id,
        job.host_chain_id,
        &job.handle,
    )
    .execute(pool)
    .await?;
    Ok(())
}

fn live_peer_sources(peers: &[RegistryPeer], buckets: &[String]) -> serde_json::Value {
    buckets
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
        .collect()
}

fn live_target_evidence(
    peers: &[RegistryPeer],
    buckets: &[String],
    digest: &[u8],
    threshold: usize,
) -> serde_json::Value {
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
    serde_json::json!({
        "required_quorum": threshold,
        "registered_coprocessor_count": peers.len(),
        "source": "attestation",
        "statements": statements,
    })
}

fn pinned_threshold(evidence: &serde_json::Value) -> Option<usize> {
    evidence
        .get("required_quorum")
        .and_then(serde_json::Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .filter(|value| *value > 0)
}

enum AttestationScope {
    /// The epoch's stack uploaded this block's S3 objects.
    Uploaded,
    /// The epoch's uploader is parked until cutover: every object is another
    /// epoch's.
    Parked,
    /// Cut over from `upload_start_block`: this block's objects were uploaded
    /// by the previous epoch and are only replaced one by one.
    BeforeUpload(i64),
}

/// S3 attestations carry no epoch, so they are evidence for a finding only
/// where the finding's own epoch uploaded the objects. `legacy` has no window
/// and uploads from block 0.
async fn attestation_scope(
    pool: &PgPool,
    job: &DueHandle,
) -> Result<AttestationScope, ExecutionError> {
    let window = sqlx::query_scalar!(
        r#"
        SELECT upload_start_block
          FROM consensus_epoch_block_window
         WHERE consensus_epoch = $1
           AND host_chain_id = $2
        "#,
        job.consensus_epoch,
        job.host_chain_id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(match window {
        None => AttestationScope::Uploaded,
        Some(None) => AttestationScope::Parked,
        Some(Some(start)) if job.block_number < start => AttestationScope::BeforeUpload(start),
        Some(Some(_)) => AttestationScope::Uploaded,
    })
}

/// Waits the retry delay without counting an attempt: the evidence can still
/// appear, so the budget is kept for real failures.
async fn defer_uncharged(pool: &PgPool, job: &DueHandle) -> Result<bool, ExecutionError> {
    let retry_secs = i64::try_from(RETRY_DELAY.as_secs()).unwrap_or(30);
    sqlx::query!(
        r#"
        UPDATE drifted_handle
           SET next_retry_at = NOW() + ($5::BIGINT * INTERVAL '1 second')
         WHERE consensus_epoch = $1
           AND coprocessor_context_id = $2
           AND host_chain_id = $3
           AND handle = $4
           AND healed_at IS NULL
        "#,
        job.consensus_epoch,
        &job.coprocessor_context_id,
        job.host_chain_id,
        &job.handle,
        retry_secs,
    )
    .execute(pool)
    .await?;
    Ok(false)
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
        error!(
            finding_id = job.id,
            handle = hex::encode(&job.handle),
            "Finding handle has no ciphertext type"
        );
        return schedule_retry(pool, job, Failure::Terminal).await;
    };
    let mut trx = pool.begin().await?;
    // The schema was resolved before the download. Cutover or rollback may have
    // replaced it since, so resolve it again under the shared cutover lock,
    // which holds it until commit.
    lock_cutover(&mut trx).await.map_err(from_containment)?;
    let schemas = epoch_schemas(&mut trx).await.map_err(from_containment)?;
    let Some(schema) = schemas.get(&job.consensus_epoch) else {
        trx.rollback().await?;
        info!(
            finding_id = job.id,
            consensus_epoch = job.consensus_epoch,
            "Finding's epoch lost its schema during the download; not installed"
        );
        return Ok(false);
    };
    if *schema != job.schema {
        trx.rollback().await?;
        info!(
            finding_id = job.id,
            consensus_epoch = job.consensus_epoch,
            from = job.schema,
            to = schema,
            "Finding's epoch moved schema during the download; retried on the next pass"
        );
        return Ok(false);
    }
    // A later epoch's cutover may have replaced the handle's bytes during the
    // download: the finding then describes bytes that are gone, and installing
    // it would overwrite the successor's copy.
    if mark_superseded(
        &mut trx,
        std::slice::from_ref(&job.consensus_epoch),
        Some(&job.handle),
    )
    .await?
        > 0
    {
        trx.commit().await?;
        info!(
            finding_id = job.id,
            consensus_epoch = job.consensus_epoch,
            "Finding superseded by a later epoch's cutover; not installed"
        );
        return Ok(false);
    }
    // `ciphertexts` and `computations` below resolve to the finding's stack;
    // `drifted_handle` exists only in public.
    set_execution_schema(&mut trx, schema)
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
           AND superseded_at IS NULL
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
    // TODO(follow-up PR): stamp the real consensus_version carried with the
    // ct64 bytes downloaded from S3, instead of NULL.
    sqlx::query!(
        r#"
        INSERT INTO ciphertexts (
            handle, ciphertext, ciphertext_version, ciphertext_type, consensus_version
        ) VALUES ($1, $2, $3, $4, NULL)
        ON CONFLICT (handle, ciphertext_version) DO UPDATE
        SET ciphertext = EXCLUDED.ciphertext,
            consensus_version = NULL
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
    metrics::ATTEMPTS
        .with_label_values(&[&job.consensus_epoch, metrics::SUCCESS])
        .inc();
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

/// A transient failure is retried until `max_attempts`; a terminal one can
/// never succeed and abandons the finding at once.
#[derive(Clone, Copy)]
enum Failure {
    Transient,
    Terminal,
}

/// Delays and counts the attempt on the finding and on its unhealed siblings
/// with the same target, so a sibling does not restart the count. The finding
/// is abandoned (`heal_abandoned_at`) when the attempt is terminal or the last
/// allowed one.
async fn schedule_retry(
    pool: &PgPool,
    job: &DueHandle,
    failure: Failure,
) -> Result<bool, ExecutionError> {
    let retry_secs = i64::try_from(RETRY_DELAY.as_secs()).unwrap_or(30);
    let rows = sqlx::query!(
        r#"
        WITH target AS (
            SELECT consensus_epoch, coprocessor_context_id, host_chain_id,
                   handle, quorum_ct64_digest
              FROM drifted_handle
             WHERE id = $1
        )
        UPDATE drifted_handle dh
           SET heal_attempts = dh.heal_attempts + 1,
               next_retry_at = NOW() + ($2::BIGINT * INTERVAL '1 second'),
               heal_abandoned_at = CASE
                   WHEN $3 OR dh.heal_attempts + 1 >= $4 THEN NOW()
               END
          FROM target t
         WHERE dh.consensus_epoch = t.consensus_epoch
           AND dh.coprocessor_context_id = t.coprocessor_context_id
           AND dh.host_chain_id = t.host_chain_id
           AND dh.handle = t.handle
           AND dh.quorum_ct64_digest IS NOT DISTINCT FROM t.quorum_ct64_digest
           AND dh.healed_at IS NULL
           AND dh.heal_abandoned_at IS NULL
           AND dh.superseded_at IS NULL
        RETURNING dh.id, dh.heal_attempts, dh.heal_abandoned_at IS NOT NULL AS "abandoned!"
        "#,
        job.id,
        retry_secs,
        matches!(failure, Failure::Terminal),
        job.max_attempts,
    )
    .fetch_all(pool)
    .await?;
    let abandoned = rows.iter().find(|row| row.id == job.id).is_some_and(|row| {
        if row.abandoned {
            error!(
                finding_id = job.id,
                handle = hex::encode(&job.handle),
                attempts = row.heal_attempts,
                max_attempts = job.max_attempts,
                "Abandoning ct64 healing; reset heal_attempts and heal_abandoned_at to retry"
            );
        }
        row.abandoned
    });
    let outcome = if abandoned {
        metrics::TERMINAL_FAILURE
    } else {
        metrics::TRANSIENT_FAILURE
    };
    metrics::ATTEMPTS
        .with_label_values(&[&job.consensus_epoch, outcome])
        .inc();
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
