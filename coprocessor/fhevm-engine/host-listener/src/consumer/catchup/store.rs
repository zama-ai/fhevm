//! Shared intent and replay admission, including commands interrupted by a crash.

use anyhow::{Context, Result};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::ManualCatchupArgs;

#[derive(Debug)]
pub struct Request {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub block_start: i64,
    pub block_end: i64,
    pub finalized: bool,
    pub request_pending: bool,
    pub cancel_pending: bool,
}

/// Select the request for the original flags, or create it when a live
/// reference is available. None means that a new request still needs that
/// reference. Every replica serializes changes to this consumer's intent.
/// A supplied owner leases a new row in the same transaction. Reusing an
/// existing request never takes its current owner's lease.
pub async fn ensure_requested(
    pool: &PgPool,
    chain_id: i64,
    consumer_id: &str,
    args: &ManualCatchupArgs,
    reference: Option<u64>,
    owner: Option<Uuid>,
    gcs_mode: bool,
) -> Result<Option<Uuid>> {
    args.validate()?;
    let (requested_from, requested_to) = args.requested()?;
    let Some(mut tx) = write_transaction(pool, gcs_mode).await? else {
        return Ok(None);
    };
    sqlx::query_scalar!(
        "WITH locked AS (SELECT pg_advisory_xact_lock(hashtextextended($1, $2)))
         SELECT 1 AS \"locked!\" FROM locked",
        consumer_id,
        chain_id,
    )
    .fetch_one(&mut *tx)
    .await?;
    let existing = sqlx::query_scalar!(
        "SELECT id FROM host_consumer_catchups
         WHERE chain_id = $1 AND consumer_id = $2
           AND requested_from = $3 AND requested_to = $4 AND finalized = $5
           AND status = 'ONGOING'",
        chain_id,
        consumer_id,
        requested_from,
        requested_to,
        !args.catchup_unfinalized,
    )
    .fetch_optional(&mut *tx)
    .await?;

    let id = if let Some(id) = existing {
        id
    } else {
        let Some(reference) = reference else {
            return Ok(None);
        };
        let (start, end) = args.resolve(reference)?;
        let start =
            i64::try_from(start).context("catchup start exceeds BIGINT")?;
        let end = i64::try_from(end).context("catchup end exceeds BIGINT")?;
        let id = Uuid::now_v7();
        sqlx::query!(
            "INSERT INTO host_consumer_catchups
             (id, chain_id, consumer_id, requested_from, requested_to, block_start, block_end,
              owner_id, lease_until, finalized)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8,
                     CASE WHEN $8::uuid IS NULL THEN NULL
                          ELSE clock_timestamp() + interval '60 seconds' END, $9)",
            id,
            chain_id,
            consumer_id,
            requested_from,
            requested_to,
            start,
            end,
            owner,
            !args.catchup_unfinalized,
        )
        .execute(&mut *tx)
        .await?;
        id
    };
    if !args.no_override_catchup {
        suppress_others(&mut tx, chain_id, consumer_id, id).await?;
    }
    tx.commit().await?;
    Ok(Some(id))
}

async fn suppress_others(
    tx: &mut Transaction<'_, Postgres>,
    chain_id: i64,
    consumer_id: &str,
    retained: Uuid,
) -> Result<()> {
    sqlx::query!(
        "UPDATE host_consumer_catchups
         SET status = 'SUPPRESSED', request_pending = FALSE, cancel_pending = TRUE
         WHERE chain_id = $1 AND consumer_id = $2
           AND status = 'ONGOING' AND id <> $3",
        chain_id,
        consumer_id,
        retained,
    )
    .execute(&mut **tx)
    .await?;
    Ok(())
}

/// Claim abandoned requests without disturbing an active owner. Every claim
/// batch uses a fresh token, including reacquisition by the same process.
pub async fn take_request_ownership(
    pool: &PgPool,
    chain_id: i64,
    consumer_id: &str,
    gcs_mode: bool,
) -> Result<Vec<Request>> {
    let Some(mut tx) = write_transaction(pool, gcs_mode).await? else {
        return Ok(Vec::new());
    };
    let owner = Uuid::new_v4();
    let requests = sqlx::query_as!(
        Request,
        "WITH available AS (
           SELECT id FROM host_consumer_catchups
           WHERE chain_id = $1 AND consumer_id = $2
             AND (status = 'ONGOING' OR cancel_pending)
             AND (owner_id IS NULL OR lease_until <= clock_timestamp())
           ORDER BY cancel_pending DESC, created_at, id
           FOR UPDATE SKIP LOCKED
         )
         UPDATE host_consumer_catchups AS r
         SET owner_id = $3, lease_until = clock_timestamp() + interval '60 seconds',
             request_pending = (r.status = 'ONGOING')
         FROM available WHERE r.id = available.id
         RETURNING r.id, r.owner_id AS \"owner_id!\", r.block_start,
                   r.block_end, r.finalized, r.request_pending, r.cancel_pending",
        chain_id,
        consumer_id,
        owner,
    )
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(requests)
}

/// Refresh owned request state on every poll, extending a valid lease only
/// once it has at most 30 seconds left. Missing rows mean ownership expired,
/// was taken over, or the request became terminal. Never revive an old token.
pub async fn renew(
    pool: &PgPool,
    ids: &[Uuid],
    owners: &[Uuid],
    gcs_mode: bool,
) -> Result<Vec<Request>> {
    anyhow::ensure!(ids.len() == owners.len(), "unpaired catchup leases");
    let Some(mut tx) = write_transaction(pool, gcs_mode).await? else {
        return Ok(Vec::new());
    };
    sqlx::query!(
        "UPDATE host_consumer_catchups AS r
         SET lease_until = clock_timestamp() + interval '60 seconds'
         FROM UNNEST($1::uuid[], $2::uuid[]) AS owned(id, owner_id)
         WHERE r.id = owned.id AND r.owner_id = owned.owner_id
           AND r.lease_until > clock_timestamp()
           AND r.lease_until <= clock_timestamp() + interval '30 seconds'
           AND (r.status = 'ONGOING' OR r.cancel_pending)",
        ids,
        owners,
    )
    .execute(&mut *tx)
    .await?;
    let requests = sqlx::query_as!(
        Request,
        "SELECT r.id, r.owner_id AS \"owner_id!\", r.block_start,
                r.block_end, r.finalized, r.request_pending, r.cancel_pending
         FROM host_consumer_catchups AS r
         JOIN UNNEST($1::uuid[], $2::uuid[]) AS owned(id, owner_id)
           ON r.id = owned.id AND r.owner_id = owned.owner_id
         WHERE r.lease_until > clock_timestamp()
           AND (r.status = 'ONGOING' OR r.cancel_pending)",
        ids,
        owners,
    )
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(requests)
}

/// Guard against ownership/status changes since the coordinator's snapshot.
pub async fn owns_command(pool: &PgPool, request: &Request) -> Result<bool> {
    Ok(sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM host_consumer_catchups
         WHERE id = $1 AND owner_id = $2 AND lease_until > clock_timestamp()
           AND CASE WHEN $3 THEN cancel_pending
                    ELSE status = 'ONGOING' AND request_pending END) AS \"owned!\"",
        request.id,
        request.owner_id,
        request.cancel_pending,
    )
    .fetch_one(pool)
    .await?)
}

/// Freeze the effective end before the first broker publication. A replay's
/// payloads can then cache that end safely. Reacquisition never re-clamps an ID.
/// None means not owned, waiting for a head, or terminal SKIPPED (no publication).
pub async fn bound_for_publication(
    pool: &PgPool,
    request: &Request,
    observed_head: Option<u64>,
    gcs_mode: bool,
) -> Result<Option<i64>> {
    let Some(mut tx) = write_transaction(pool, gcs_mode).await? else {
        return Ok(None);
    };
    let row = sqlx::query!(
        "SELECT block_start, block_end, observed_head FROM host_consumer_catchups
         WHERE id = $1 AND owner_id = $2 AND lease_until > clock_timestamp()
           AND status = 'ONGOING' AND request_pending FOR UPDATE",
        request.id,
        request.owner_id,
    )
    .fetch_optional(&mut *tx)
    .await?;
    let Some(row) = row else { return Ok(None) };
    if row.observed_head.is_some() {
        return Ok(Some(row.block_end));
    }
    let Some(head) = observed_head else {
        tracing::debug!(catchup_id = %request.id, finalized = request.finalized,
            "Waiting for an observed head before bounding catchup");
        return Ok(None);
    };
    let head =
        i64::try_from(head).context("observed catchup head exceeds BIGINT")?;
    let skipped = row.block_start > head;
    let end = if skipped {
        row.block_end
    } else {
        row.block_end.min(head)
    };
    let status = if skipped { "SKIPPED" } else { "ONGOING" };
    sqlx::query!(
        "UPDATE host_consumer_catchups
         SET block_end = $2, observed_head = $3, status = $4,
             request_pending = NOT $5,
             owner_id = CASE WHEN $5 THEN NULL ELSE owner_id END,
             lease_until = CASE WHEN $5 THEN NULL ELSE lease_until END
         WHERE id = $1",
        request.id,
        end,
        head,
        status,
        skipped,
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    if skipped {
        tracing::warn!(catchup_id = %request.id, finalized = request.finalized,
            start = row.block_start, requested_end = row.block_end, observed_head = head,
            "Skipping catchup: start is above the observed eligible head");
        Ok(None)
    } else {
        if end < row.block_end {
            tracing::warn!(catchup_id = %request.id, finalized = request.finalized,
                start = row.block_start, requested_end = row.block_end, end, observed_head = head,
                "Clamping catchup end to the observed eligible head");
        }
        Ok(Some(end))
    }
}

pub async fn requested(
    pool: &PgPool,
    id: Uuid,
    owner: Uuid,
    gcs_mode: bool,
) -> Result<bool> {
    let Some(mut tx) = write_transaction(pool, gcs_mode).await? else {
        return Ok(false);
    };
    let updated = sqlx::query!(
        "UPDATE host_consumer_catchups SET request_pending = FALSE
         WHERE id = $1 AND owner_id = $2 AND lease_until > clock_timestamp()
           AND status = 'ONGOING'",
        id,
        owner,
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(updated.rows_affected() == 1)
}

pub async fn cancelled(
    pool: &PgPool,
    id: Uuid,
    owner: Uuid,
    gcs_mode: bool,
) -> Result<bool> {
    let Some(mut tx) = write_transaction(pool, gcs_mode).await? else {
        return Ok(false);
    };
    let updated = sqlx::query!(
        "UPDATE host_consumer_catchups
         SET cancel_pending = FALSE, owner_id = NULL, lease_until = NULL
         WHERE id = $1 AND owner_id = $2 AND lease_until > clock_timestamp()
           AND cancel_pending",
        id,
        owner,
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(updated.rows_affected() == 1)
}

/// Bounds are frozen before first publication and can be cached by the handler.
pub async fn block_end(
    pool: &PgPool,
    chain_id: i64,
    consumer_id: &str,
    id: Uuid,
) -> Result<Option<u64>> {
    Ok(sqlx::query_scalar!(
        "SELECT block_end FROM host_consumer_catchups
         WHERE id = $1 AND chain_id = $2 AND consumer_id = $3",
        id,
        chain_id,
        consumer_id,
    )
    .fetch_optional(pool)
    .await?
    .map(|end| end as u64))
}

/// Record successful ingestion of the last block. Any replica can receive it,
/// independently of the command lease. Suppression must never be overwritten.
pub async fn completed(
    pool: &PgPool,
    chain_id: i64,
    consumer_id: &str,
    id: Uuid,
    gcs_mode: bool,
) -> Result<bool> {
    let Some(mut tx) = write_transaction(pool, gcs_mode).await? else {
        return Ok(false);
    };
    let updated = sqlx::query!(
        "UPDATE host_consumer_catchups
         SET status = 'DONE', request_pending = FALSE, owner_id = NULL, lease_until = NULL
         WHERE id = $1 AND chain_id = $2 AND consumer_id = $3
           AND status = 'ONGOING'",
        id,
        chain_id,
        consumer_id,
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(updated.rows_affected() == 1)
}

/// Consult shared state on every delivery, rather than a per-pod snapshot.
pub async fn accepts(
    pool: &PgPool,
    chain_id: i64,
    consumer_id: &str,
    id: Option<Uuid>,
    finalized: bool,
) -> Result<bool> {
    let Some(id) = id else { return Ok(false) };
    Ok(sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM host_consumer_catchups
         WHERE id = $1 AND chain_id = $2 AND consumer_id = $3
           AND finalized = $4 AND status IN ('ONGOING', 'DONE')) AS \"accepted!\"",
        id,
        chain_id,
        consumer_id,
        finalized,
    )
    .fetch_one(pool)
    .await?)
}

/// Keep suppression serialized with the actual block writes. A delivery
/// admitted before replacement must not commit after suppression commits.
/// DONE remains accepted while another replica drains earlier deliveries.
pub async fn lock_for_ingest(
    tx: &mut Transaction<'_, Postgres>,
    chain_id: i64,
    id: Uuid,
) -> Result<bool, sqlx::Error> {
    let status = sqlx::query_scalar!(
        "SELECT status FROM host_consumer_catchups
             WHERE id = $1 AND chain_id = $2 FOR SHARE",
        id,
        chain_id,
    )
    .fetch_optional(&mut **tx)
    .await?;
    Ok(matches!(status.as_deref(), Some("ONGOING" | "DONE")))
}

async fn write_transaction(
    pool: &PgPool,
    gcs_mode: bool,
) -> Result<Option<Transaction<'static, Postgres>>> {
    use fhevm_engine_common::versioning::{
        begin_write_guarded, GcsRollbackPolicy,
    };
    Ok(
        begin_write_guarded(pool, gcs_mode, GcsRollbackPolicy::Continue)
            .await?
            .into_tx(),
    )
}
