//! Demand-driven local ct64 repair.
//!
//! The worker is started with publication and verification. It LISTENs on
//! `event_healing_work` and also polls every 30s so a missed NOTIFY still
//! runs. Each pass takes due `can_be_healed` rows with `FOR UPDATE OF`
//! `drifted_handle` only for that pick, ordered by `drifted_handle_demand`.
//! Demand lives on a separate table so TFHE EMA writes never lock these rows.
//! The pick lock is not held across S3. Installation is not in this pass.

use std::sync::Arc;
use std::time::Duration;

use alloy_primitives::{keccak256, U256};
use aws_sdk_s3::Client;
use futures::future::join_all;
use sqlx::{postgres::PgListener, PgPool};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use fhevm_engine_common::pg_pool::is_fatal_connection_error;

use super::{ExecutionError, ManifestWorkGate};

mod download;

use download::{Ct64Source, S3Ct64Source};

/// pg_notify channel emitted on `drifted_handle` insert/update.
pub(crate) const EVENT_HEALING_WORK: &str = "event_healing_work";

const POLL_INTERVAL: Duration = Duration::from_secs(30);
const RETRY_DELAY: Duration = Duration::from_secs(30);
const MAX_DOWNLOADS: i64 = 8;

struct DueHandle {
    id: i64,
    handle: Vec<u8>,
    coprocessor_context_id: Vec<u8>,
    target_ct64_digest: Vec<u8>,
    peer_sources: serde_json::Value,
}

pub(crate) fn spawn_healing_worker(
    pool: PgPool,
    token: CancellationToken,
    client: Arc<Client>,
    work_gate: Arc<ManifestWorkGate>,
) -> JoinHandle<Result<(), ExecutionError>> {
    tokio::spawn(run_healing_worker(
        pool,
        token,
        S3Ct64Source::new(client),
        work_gate,
    ))
}

async fn run_healing_worker<S: Ct64Source>(
    pool: PgPool,
    token: CancellationToken,
    source: S,
    work_gate: Arc<ManifestWorkGate>,
) -> Result<(), ExecutionError> {
    info!("Healing worker enabled for this stack consensus_epoch");
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen(EVENT_HEALING_WORK).await?;
    if let Err(err) = run_healing_pass(&pool, &source, &work_gate).await {
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
            _ = tokio::time::sleep(POLL_INTERVAL) => {}
        }
        match run_healing_pass(&pool, &source, &work_gate).await {
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
) -> Result<(), ExecutionError> {
    let Some(epoch) = work_gate.pinned_consensus_epoch() else {
        return Ok(());
    };
    let due = lock_due(pool, &epoch).await?;
    if due.is_empty() {
        return Ok(());
    }
    let jobs = due.into_iter().map(|job| {
        let source = source.clone();
        let pool = pool.clone();
        async move { heal_one(&pool, &source, job).await }
    });
    for result in join_all(jobs).await {
        result?;
    }
    Ok(())
}

async fn lock_due(pool: &PgPool, consensus_epoch: &str) -> Result<Vec<DueHandle>, ExecutionError> {
    let mut trx = pool.begin().await?;
    let rows = sqlx::query!(
        r#"
        SELECT dh.id,
               dh.handle,
               dh.coprocessor_context_id,
               dh.target_ct64_digest AS "target_ct64_digest!",
               dh.peer_sources::text AS "peer_sources!"
          FROM drifted_handle dh
          LEFT JOIN drifted_handle_demand demand
            ON demand.handle = dh.handle
         WHERE dh.can_be_healed
           AND dh.consensus_epoch = $1
           AND (dh.next_retry_at IS NULL OR dh.next_retry_at <= NOW())
         ORDER BY COALESCE(demand.tx_unlock_potential, 0) DESC,
                  dh.block_number ASC,
                  dh.id ASC
         LIMIT $2
         FOR UPDATE OF dh
        "#,
        consensus_epoch,
        MAX_DOWNLOADS,
    )
    .fetch_all(trx.as_mut())
    .await?;
    trx.commit().await?;
    Ok(rows
        .into_iter()
        .map(|row| DueHandle {
            id: row.id,
            handle: row.handle,
            coprocessor_context_id: row.coprocessor_context_id,
            target_ct64_digest: row.target_ct64_digest,
            peer_sources: serde_json::from_str(&row.peer_sources)
                .unwrap_or(serde_json::Value::Array(vec![])),
        })
        .collect())
}

async fn heal_one<S: Ct64Source>(
    pool: &PgPool,
    source: &S,
    job: DueHandle,
) -> Result<(), ExecutionError> {
    let Ok(context_id) = u256_from_bytes(&job.coprocessor_context_id) else {
        return schedule_retry(pool, job.id).await;
    };
    let buckets = peer_bucket_urls(&job.peer_sources);
    for bucket_url in &buckets {
        match source.get_ct64(bucket_url, &job.handle, context_id).await {
            Ok(bytes) if keccak256(&bytes).as_slice() == job.target_ct64_digest => {
                info!(
                    finding_id = job.id,
                    handle = hex::encode(&job.handle),
                    "Downloaded ct64 matching the pinned target"
                );
                return Ok(());
            }
            Ok(_) => {
                warn!(
                    finding_id = job.id,
                    bucket_url, "Peer ct64 digest does not match the pinned target"
                );
            }
            Err(ExecutionError::S3ObjectNotFound(_)) | Err(ExecutionError::S3TransientError(_)) => {
            }
            Err(err) => return Err(err),
        }
    }
    schedule_retry(pool, job.id).await
}

async fn schedule_retry(pool: &PgPool, id: i64) -> Result<(), ExecutionError> {
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
    Ok(())
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
