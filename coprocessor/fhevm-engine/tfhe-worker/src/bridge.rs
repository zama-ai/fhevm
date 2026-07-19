//! Confidential bridge (see RFC 008).
//!
//! The host-listener ingests and validates the two bridge events into
//! `bridge_handle_events` (source-chain `BridgeHandle` event) and
//! `handle_bridged_events` (destination-chain `HandleBridged` event). This
//! worker performs the step where once both events are present for a
//! `(src_handle, dst_chain_id)` pair and the source ciphertext is fully
//! materialized, it copies the source ciphertext onto the derived destination
//! handle.
//!
//! Bridging creates a *copy* of a ciphertext, not a link. Because the copy is
//! bit-for-bit identical, its ct64/ct128 digests are identical too, so we reuse
//! the source's already-uploaded S3 blobs by copying the `ciphertext_digest`
//! row (retargeted to the destination chain) instead of re-running SnS. The
//! copied digest row drives the transaction-sender to publish on the
//! destination chain. The copied `ciphertexts` row makes the handle usable in
//! destination-chain computations.
//!
//! Association is one-shot per destination handle: `associate_pair` sets the
//! `is_associated` flag on the `handle_bridged_events` row in the same
//! transaction as the copy, and the readiness query skips flagged rows.

use std::sync::LazyLock;
use std::time::Duration;

use fhevm_engine_common::database::{
    connect_pool_with_options, resolve_database_url_from_option, EVENT_CIPHERTEXTS_UPLOADED,
};
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use prometheus::{register_int_counter, register_int_gauge, IntCounter, IntGauge};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Postgres, Transaction};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

static BRIDGE_ASSOCIATED_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_tfhe_worker_bridge_associated_counter",
        "Number of bridged handle pairs associated by the confidential bridge worker"
    )
    .unwrap()
});

static BRIDGE_ERROR_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_tfhe_worker_bridge_errors_counter",
        "Number of failed association cycles in the confidential bridge worker"
    )
    .unwrap()
});

// Grace period before an unassociated handle is counted, so normal in-flight
// handles (briefly unassociated while their ciphertext materializes) are excluded.
const IN_FLIGHT_GRACE_SECS: i32 = 300;

static UNASSOCIATED_HANDLES: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_bridge_unassociated_handles",
        "Unassociated bridged handles past the in-flight grace period"
    )
    .unwrap()
});

struct BridgeSourceMaterial {
    ciphertext: Vec<u8>,
    ciphertext_version: i16,
    ciphertext_type: i16,
    ct64_digest: Vec<u8>,
    ct128_digest: Vec<u8>,
    ciphertext128_format: i16,
    key_id_gw: Vec<u8>,
    s3_format_version: Option<i16>,
}

pub async fn run_confidential_bridge(
    args: crate::daemon_cli::Args,
    cancel_token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db_url = resolve_database_url_from_option(args.database_url.clone())?;
    // A single connection suffices: the worker polls and associates sequentially.
    let (pool, _pool_refresh_handle) = connect_pool_with_options(
        &db_url,
        PgPoolOptions::new().max_connections(1),
        Some(&cancel_token),
    )
    .await?;

    let poll_interval = Duration::from_millis(args.bridge_polling_interval_ms);
    info!(
        target: "bridge",
        polling_interval_ms = args.bridge_polling_interval_ms,
        "Starting confidential bridge worker"
    );

    loop {
        if cancel_token.is_cancelled() {
            break;
        }
        match drain_associations_at_cutover(
            &pool,
            args.bridge_associate_batch_size,
            &cancel_token,
            args.branch_cutover_block,
        )
        .await
        {
            Ok(associated) if associated > 0 => {
                info!(target: "bridge", associated, "Associated bridged handles")
            }
            Ok(_) => {}
            Err(err) => {
                BRIDGE_ERROR_COUNTER.inc();
                error!(target: "bridge", error = %err, "Bridge association cycle failed, retrying")
            }
        }
        match count_unassociated_handles(&pool).await {
            Ok(count) => UNASSOCIATED_HANDLES.set(count),
            Err(err) => {
                error!(target: "bridge", error = %err, "Failed to refresh unassociated-handles gauge")
            }
        }
        tokio::select! {
            _ = tokio::time::sleep(poll_interval) => {}
            _ = cancel_token.cancelled() => break,
        }
    }

    Ok(())
}

// Associates ready pairs.
#[cfg(test)]
pub(crate) async fn drain_associations(
    pool: &PgPool,
    batch_size: i64,
    cancel_token: &CancellationToken,
) -> Result<u64, sqlx::Error> {
    drain_associations_at_cutover(pool, batch_size, cancel_token, i64::MAX).await
}

pub(crate) async fn drain_associations_at_cutover(
    pool: &PgPool,
    batch_size: i64,
    cancel_token: &CancellationToken,
    branch_cutover_block: i64,
) -> Result<u64, sqlx::Error> {
    let mut total = 0;
    loop {
        if cancel_token.is_cancelled() {
            break;
        }
        let associated = associate_batch(pool, batch_size, branch_cutover_block).await?;
        total += associated;
        if associated < batch_size as u64 {
            break;
        }
    }
    Ok(total)
}

async fn count_unassociated_handles(pool: &PgPool) -> Result<i64, sqlx::Error> {
    // Skip handles whose ciphertext already exists (e.g. via grantFallbackPlaintext): they are
    // recovered but never get is_associated set, so counting them would inflate the gauge forever.
    sqlx::query_scalar!(
        r#"
        SELECT count(*) AS "count!"
        FROM handle_bridged_events
        WHERE NOT is_associated
          AND NOT EXISTS (SELECT 1 FROM ciphertexts WHERE handle = handle_bridged_events.dst_handle)
          AND NOT EXISTS (
                SELECT 1 FROM ciphertexts_branch
                WHERE handle = handle_bridged_events.dst_handle)
          AND (
                block_hash = ''::bytea
                OR EXISTS (
                    SELECT 1 FROM host_chain_blocks_valid dst_block
                    WHERE dst_block.chain_id = handle_bridged_events.dst_chain_id
                      AND dst_block.block_hash = handle_bridged_events.block_hash
                      AND dst_block.block_status <> 'orphaned'
                )
          )
          AND created_at <= now() - make_interval(secs => $1::int)
        "#,
        IN_FLIGHT_GRACE_SECS,
    )
    .fetch_one(pool)
    .await
}

async fn associate_batch(
    pool: &PgPool,
    batch_size: i64,
    branch_cutover_block: i64,
) -> Result<u64, sqlx::Error> {
    let mut txn = pool.begin().await?;

    // A pair is ready to associate when:
    // - the destination handle is not already materialized by another path
    // - both validated events are present: the destination `HandleBridged`
    //   and the matching source `BridgeHandle` one
    // - the source approval's block is finalized; the destination event is
    //   consumed as observed (no finality wait), skipped only when its block
    //   is already known orphaned
    // - the source ciphertext is fully materialized: its ct64 blob exists and
    //   both digests (ct64 and ct128) are computed
    // - it has not been associated yet
    let ready = sqlx::query!(
        r#"
        SELECT dst_event.id,
               dst_event.src_handle,
               dst_event.dst_handle,
               dst_event.dst_chain_id,
               dst_event.block_number,
               dst_event.transaction_id,
               src_event.src_chain_id AS "src_chain_id!"
        FROM handle_bridged_events dst_event
        JOIN LATERAL (
            SELECT candidate.src_chain_id
            FROM bridge_handle_events candidate
            WHERE candidate.src_handle = dst_event.src_handle
              AND candidate.dst_chain_id = dst_event.dst_chain_id
              AND (
                    candidate.block_hash = ''::bytea
                    OR EXISTS (
                        SELECT 1 FROM host_chain_blocks_valid src_block
                        WHERE src_block.chain_id = candidate.src_chain_id
                          AND src_block.block_hash = candidate.block_hash
                          AND src_block.block_status = 'finalized'
                    )
              )
            ORDER BY candidate.id
            LIMIT 1
        ) src_event ON TRUE
        WHERE NOT dst_event.is_associated
          AND (
                (
                    dst_event.block_number < $2
                    AND NOT EXISTS (
                        SELECT 1 FROM ciphertexts dst_ct
                        WHERE dst_ct.handle = dst_event.dst_handle
                    )
                )
                OR (
                    dst_event.block_number >= $2
                    AND NOT EXISTS (
                        SELECT 1 FROM ciphertexts_branch dst_ct
                        WHERE dst_ct.handle = dst_event.dst_handle
                    )
                )
          )
          AND (
                dst_event.block_hash = ''::bytea
                OR EXISTS (
                    SELECT 1 FROM host_chain_blocks_valid dst_block
                    WHERE dst_block.chain_id = dst_event.dst_chain_id
                      AND dst_block.block_hash = dst_event.block_hash
                      AND dst_block.block_status <> 'orphaned'
                )
          )
          AND (
                EXISTS (
                    SELECT 1
                    FROM ciphertexts_branch src_ct
                    JOIN ciphertext_digest_branch src_digest
                      ON src_digest.handle = src_ct.handle
                     AND src_digest.producer_block_hash = src_ct.producer_block_hash
                    WHERE src_ct.handle = dst_event.src_handle
                      AND src_ct.ciphertext IS NOT NULL
                      AND src_ct.ciphertext_version = $3
                      AND src_digest.host_chain_id = src_event.src_chain_id
                      AND src_digest.ciphertext IS NOT NULL
                      AND src_digest.ciphertext128 IS NOT NULL
                      AND src_digest.ciphertext128_format IS NOT NULL
                      AND (
                            src_digest.producer_block_hash = ''::bytea
                            OR NOT EXISTS (
                                SELECT 1 FROM host_chain_blocks_valid producer_block
                                WHERE producer_block.chain_id = src_digest.host_chain_id
                                  AND producer_block.block_hash = src_digest.producer_block_hash
                                  AND producer_block.block_status = 'orphaned'
                            )
                      )
                      AND (
                            src_digest.block_hash = ''::bytea
                            OR NOT EXISTS (
                                SELECT 1 FROM host_chain_blocks_valid event_block
                                WHERE event_block.chain_id = src_digest.host_chain_id
                                  AND event_block.block_hash = src_digest.block_hash
                                  AND event_block.block_status = 'orphaned'
                            )
                      )
                )
                OR (
                    EXISTS (
                        SELECT 1 FROM ciphertexts src_ct
                        WHERE src_ct.handle = dst_event.src_handle
                          AND src_ct.ciphertext IS NOT NULL
                          AND src_ct.ciphertext_version = $3
                    )
                    AND EXISTS (
                        SELECT 1 FROM ciphertext_digest src_digest
                        WHERE src_digest.handle = dst_event.src_handle
                          AND src_digest.host_chain_id = src_event.src_chain_id
                          AND src_digest.ciphertext IS NOT NULL
                          AND src_digest.ciphertext128 IS NOT NULL
                    )
                )
          )
        ORDER BY dst_event.id
        FOR UPDATE OF dst_event SKIP LOCKED
        LIMIT $1
        "#,
        batch_size,
        branch_cutover_block,
        current_ciphertext_version(),
    )
    .fetch_all(txn.as_mut())
    .await?;

    // Only pairs whose writer fully materialized the destination (ciphertext
    // and digest placed, event marked) count as associated: the count feeds
    // the metric, the transaction-sender wakeup, and the drain loop's
    // continuation condition, so a pair the writer had to skip must not
    // register progress (a full batch of skips would otherwise spin the drain
    // loop hot while re-selecting the same rows).
    let mut associated: u64 = 0;
    for pair in ready {
        let pair_associated = if pair.block_number >= branch_cutover_block {
            associate_pair_branch(
                &mut txn,
                pair.id,
                &pair.src_handle,
                &pair.dst_handle,
                pair.src_chain_id,
                pair.dst_chain_id,
                pair.transaction_id.as_deref(),
            )
            .await?
        } else {
            associate_pair(
                &mut txn,
                pair.id,
                &pair.src_handle,
                &pair.dst_handle,
                pair.src_chain_id,
                pair.dst_chain_id,
            )
            .await?
        };
        if pair_associated {
            associated += 1;
        } else {
            // The pair stays unassociated and is retried on the next poll.
            // Reachable only when the source material or the destination slot
            // changed between the readiness check and the copy (concurrent
            // retraction or another materialization path).
            warn!(
                target: "bridge",
                id = pair.id,
                dst_handle = %hex::encode(&pair.dst_handle),
                "Ready bridge pair could not be associated; leaving it for the next cycle"
            );
        }
    }

    // Wake the transaction-sender.
    if associated > 0 {
        sqlx::query!("SELECT pg_notify($1, '')", EVENT_CIPHERTEXTS_UPLOADED)
            .execute(txn.as_mut())
            .await?;
    }

    txn.commit().await?;
    BRIDGE_ASSOCIATED_COUNTER.inc_by(associated);
    Ok(associated)
}

/// Copies the source ciphertext and its digest onto the destination handle in
/// the legacy tables (destination events below the branch cutover). The digest
/// is retargeted to the destination chain so the transaction-sender publishes
/// it there. The ct64/ct128 digests (and therefore the S3 blobs they
/// reference) are unchanged because the ciphertext is identical.
///
/// Returns whether the pair was fully associated: ciphertext and digest
/// materialized and the event marked, all in the caller's transaction.
pub(crate) async fn associate_pair(
    txn: &mut Transaction<'_, Postgres>,
    id: i64,
    src_handle: &[u8],
    dst_handle: &[u8],
    src_chain_id: i64,
    dst_chain_id: i64,
) -> Result<bool, sqlx::Error> {
    // Source reads are branch-aware even though the destination is written to
    // the legacy tables: a destination event below the cutover can reference a
    // source handle produced by a wave2 binary, whose bytes and digests exist
    // only in the branch tables.
    let Some(source) = fetch_bridge_source_material(txn, src_handle, src_chain_id).await? else {
        return Ok(false);
    };

    // Write-once: copy the source ciphertext only if the destination handle has no
    // ciphertext yet.
    let ciphertext_copied = sqlx::query!(
        r#"
        INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
        SELECT $1, $2, $3, $4
        WHERE NOT EXISTS (SELECT 1 FROM ciphertexts WHERE handle = $1)
        ON CONFLICT (handle, ciphertext_version) DO NOTHING
        "#,
        dst_handle,
        &source.ciphertext,
        source.ciphertext_version,
        source.ciphertext_type,
    )
    .execute(txn.as_mut())
    .await?
    .rows_affected()
        > 0;

    // Copy the digest and mark the event associated only when we actually placed
    // the ciphertext. If the destination was already materialized by another path
    // (e.g. a grantFallbackPlaintext recovery), the copy above is a no-op.
    //
    // Contract: `is_associated` is set in the SAME transaction as the copy —
    // the host-listener's reorg cleanup uses a flagged observation in an
    // orphaned block as proof that this association produced the
    // materialization, and retracts it.
    if !ciphertext_copied {
        return Ok(false);
    }

    // `s3_format_version` travels with the digests: the copied row points at
    // the source's S3 objects, so it must carry the version those objects
    // were written with (NULL means "not uploaded" and would make the row
    // self-contradictory — digests present but officially absent from S3).
    sqlx::query!(
        r#"
        INSERT INTO ciphertext_digest
            (handle, ciphertext, ciphertext128, ciphertext128_format, host_chain_id, key_id_gw, s3_format_version)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (handle) DO NOTHING
        "#,
        dst_handle,
        &source.ct64_digest,
        &source.ct128_digest,
        source.ciphertext128_format,
        dst_chain_id,
        &source.key_id_gw,
        source.s3_format_version,
    )
    .execute(txn.as_mut())
    .await?;

    sqlx::query!(
        "UPDATE handle_bridged_events SET is_associated = true WHERE id = $1",
        id
    )
    .execute(txn.as_mut())
    .await?;

    Ok(true)
}

/// Wave-2 bridge association. Source material prefers a live branch row but
/// falls back to the legacy tables so pre-cutover handles remain bridgeable.
/// The destination is branchless because its lifetime is governed explicitly
/// by the destination `HandleBridged` observation and its reorg cleanup.
///
/// Returns whether the pair was fully associated (see [`associate_pair`]).
#[allow(clippy::too_many_arguments)]
async fn associate_pair_branch(
    txn: &mut Transaction<'_, Postgres>,
    id: i64,
    src_handle: &[u8],
    dst_handle: &[u8],
    src_chain_id: i64,
    dst_chain_id: i64,
    transaction_id: Option<&[u8]>,
) -> Result<bool, sqlx::Error> {
    let Some(source) = fetch_bridge_source_material(txn, src_handle, src_chain_id).await? else {
        return Ok(false);
    };

    let ciphertext_copied = sqlx::query!(
        r#"
        INSERT INTO ciphertexts_branch (
            handle,
            ciphertext,
            ciphertext_version,
            ciphertext_type,
            producer_block_hash,
            block_number
        )
        VALUES ($1, $2, $3, $4, ''::bytea, NULL)
        ON CONFLICT (handle, ciphertext_version, producer_block_hash) DO NOTHING
        "#,
        dst_handle,
        &source.ciphertext,
        source.ciphertext_version,
        source.ciphertext_type,
    )
    .execute(txn.as_mut())
    .await?
    .rows_affected()
        > 0;

    if !ciphertext_copied {
        return Ok(false);
    }

    sqlx::query!(
        r#"
        INSERT INTO ciphertext_digest_branch (
            handle,
            ciphertext,
            ciphertext128,
            ciphertext128_format,
            host_chain_id,
            key_id_gw,
            s3_format_version,
            producer_block_hash,
            block_number,
            block_hash,
            transaction_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, ''::bytea, NULL, ''::bytea, $8)
        ON CONFLICT (handle, producer_block_hash, block_hash) DO NOTHING
        "#,
        dst_handle,
        &source.ct64_digest,
        &source.ct128_digest,
        source.ciphertext128_format,
        dst_chain_id,
        &source.key_id_gw,
        source.s3_format_version,
        transaction_id,
    )
    .execute(txn.as_mut())
    .await?;

    sqlx::query!(
        "UPDATE handle_bridged_events SET is_associated = TRUE WHERE id = $1",
        id
    )
    .execute(txn.as_mut())
    .await?;

    Ok(true)
}

/// Selects the freshest live source material for `src_handle`, preferring a
/// branch row over the legacy tables. Mirrors the source arms of the readiness
/// query in `associate_batch`, so any pair selected as ready resolves material
/// here unless it raced a concurrent retraction.
async fn fetch_bridge_source_material(
    txn: &mut Transaction<'_, Postgres>,
    src_handle: &[u8],
    src_chain_id: i64,
) -> Result<Option<BridgeSourceMaterial>, sqlx::Error> {
    sqlx::query_as!(
        BridgeSourceMaterial,
        r#"
        SELECT ciphertext AS "ciphertext!",
               ciphertext_version AS "ciphertext_version!",
               ciphertext_type AS "ciphertext_type!",
               ct64_digest AS "ct64_digest!",
               ct128_digest AS "ct128_digest!",
               ciphertext128_format AS "ciphertext128_format!",
               key_id_gw AS "key_id_gw!",
               s3_format_version AS "s3_format_version?"
        FROM (
            SELECT src_ct.ciphertext,
                   src_ct.ciphertext_version,
                   src_ct.ciphertext_type,
                   src_digest.ciphertext AS ct64_digest,
                   src_digest.ciphertext128 AS ct128_digest,
                   src_digest.ciphertext128_format,
                   src_digest.key_id_gw,
                   src_digest.s3_format_version,
                   0 AS source_priority,
                   COALESCE(src_digest.block_number, -1) AS source_block_number,
                   src_digest.producer_block_hash AS source_producer_block_hash
            FROM ciphertexts_branch src_ct
            JOIN ciphertext_digest_branch src_digest
              ON src_digest.handle = src_ct.handle
             AND src_digest.producer_block_hash = src_ct.producer_block_hash
            WHERE src_ct.handle = $1
              AND src_ct.ciphertext IS NOT NULL
              AND src_ct.ciphertext_version = $2
              AND src_digest.host_chain_id = $3
              AND src_digest.ciphertext IS NOT NULL
              AND src_digest.ciphertext128 IS NOT NULL
              AND src_digest.ciphertext128_format IS NOT NULL
              AND (
                    src_digest.producer_block_hash = ''::bytea
                    OR NOT EXISTS (
                        SELECT 1 FROM host_chain_blocks_valid producer_block
                        WHERE producer_block.chain_id = src_digest.host_chain_id
                          AND producer_block.block_hash = src_digest.producer_block_hash
                          AND producer_block.block_status = 'orphaned'
                    )
              )
              AND (
                    src_digest.block_hash = ''::bytea
                    OR NOT EXISTS (
                        SELECT 1 FROM host_chain_blocks_valid event_block
                        WHERE event_block.chain_id = src_digest.host_chain_id
                          AND event_block.block_hash = src_digest.block_hash
                          AND event_block.block_status = 'orphaned'
                    )
              )

            UNION ALL

            SELECT src_ct.ciphertext,
                   src_ct.ciphertext_version,
                   src_ct.ciphertext_type,
                   src_digest.ciphertext AS ct64_digest,
                   src_digest.ciphertext128 AS ct128_digest,
                   src_digest.ciphertext128_format,
                   src_digest.key_id_gw,
                   src_digest.s3_format_version,
                   1 AS source_priority,
                   -1 AS source_block_number,
                   ''::bytea AS source_producer_block_hash
            FROM ciphertexts src_ct
            JOIN ciphertext_digest src_digest ON src_digest.handle = src_ct.handle
            WHERE src_ct.handle = $1
              AND src_ct.ciphertext IS NOT NULL
              AND src_ct.ciphertext_version = $2
              AND src_digest.host_chain_id = $3
              AND src_digest.ciphertext IS NOT NULL
              AND src_digest.ciphertext128 IS NOT NULL
        ) source
        -- Deterministic across coprocessors: never tie-break on a local column
        -- like created_at. Under per-block re-randomization two live sibling
        -- forks of the same source handle hold different bytes, so an
        -- ingestion-order tie-break would copy divergent bytes/digests onto the
        -- (write-once) destination on different coprocessors. producer_block_hash
        -- is unique per fork at a given height and identical fleet-wide.
        ORDER BY source_priority, source_block_number DESC, source_producer_block_hash
        LIMIT 1
        "#,
        src_handle,
        current_ciphertext_version(),
        src_chain_id,
    )
    .fetch_optional(txn.as_mut())
    .await
}
