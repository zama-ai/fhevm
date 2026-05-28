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

use std::time::Duration;

use fhevm_engine_common::database::{connect_pool_with_options, resolve_database_url_from_option};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Postgres, Transaction};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

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
        match drain_associations(&pool, args.bridge_associate_batch_size, &cancel_token).await {
            Ok(associated) if associated > 0 => {
                info!(target: "bridge", associated, "Associated bridged handles")
            }
            Ok(_) => {}
            Err(err) => {
                error!(target: "bridge", error = %err, "Bridge association cycle failed, retrying")
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
pub(crate) async fn drain_associations(
    pool: &PgPool,
    batch_size: i64,
    cancel_token: &CancellationToken,
) -> Result<u64, sqlx::Error> {
    let mut total = 0;
    loop {
        if cancel_token.is_cancelled() {
            break;
        }
        let associated = associate_batch(pool, batch_size).await?;
        total += associated;
        if associated < batch_size as u64 {
            break;
        }
    }
    Ok(total)
}

async fn associate_batch(pool: &PgPool, batch_size: i64) -> Result<u64, sqlx::Error> {
    let mut txn = pool.begin().await?;

    // A pair is ready to associate when:
    // - both validated events are present: the destination `HandleBridged`
    //   and the matching source `BridgeHandle` one
    // - the source ciphertext is fully materialized: its ct64 blob exists and
    //   both digests (ct64 and ct128) are computed
    // - it has not been associated yet
    let ready = sqlx::query!(
        r#"
        SELECT dst_event.id, dst_event.src_handle, dst_event.dst_handle, dst_event.dst_chain_id
        FROM handle_bridged_events dst_event
        WHERE NOT dst_event.is_associated
          AND EXISTS (
                SELECT 1 FROM bridge_handle_events src_event
                WHERE src_event.src_handle = dst_event.src_handle
                  AND src_event.dst_chain_id = dst_event.dst_chain_id)
          AND EXISTS (
                SELECT 1 FROM ciphertexts src_ct
                WHERE src_ct.handle = dst_event.src_handle)
          AND EXISTS (
                SELECT 1 FROM ciphertext_digest src_digest
                WHERE src_digest.handle = dst_event.src_handle
                  AND src_digest.ciphertext IS NOT NULL
                  AND src_digest.ciphertext128 IS NOT NULL)
        ORDER BY dst_event.id
        FOR UPDATE OF dst_event SKIP LOCKED
        LIMIT $1
        "#,
        batch_size,
    )
    .fetch_all(txn.as_mut())
    .await?;

    let associated = ready.len() as u64;
    for pair in ready {
        associate_pair(
            &mut txn,
            pair.id,
            &pair.src_handle,
            &pair.dst_handle,
            pair.dst_chain_id,
        )
        .await?;
    }

    txn.commit().await?;
    Ok(associated)
}

/// Copies the source ciphertext and its digest onto the destination handle. The
/// digest is retargeted to the destination chain so the transaction-sender
/// publishes it there. The ct64/ct128 digests (and therefore the S3 blobs they
/// reference) are unchanged because the ciphertext is identical.
async fn associate_pair(
    txn: &mut Transaction<'_, Postgres>,
    id: i64,
    src_handle: &[u8],
    dst_handle: &[u8],
    dst_chain_id: i64,
) -> Result<(), sqlx::Error> {
    let ciphertext_copied = sqlx::query!(
        r#"
        INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
        SELECT $1, ciphertext, ciphertext_version, ciphertext_type
        FROM ciphertexts
        WHERE handle = $2
        ON CONFLICT (handle, ciphertext_version) DO NOTHING
        "#,
        dst_handle,
        src_handle,
    )
    .execute(txn.as_mut())
    .await?
    .rows_affected()
        > 0;

    // The destination handle may already be materialized by another path (a
    // `grantFallbackPlaintext` recovery converts to a trivial encryption). In that
    // case the ciphertext copy is a no-op, and we must NOT copy the digest either:
    // publishing the source digest while the stored ciphertext is the fallback's
    // would leave the handle with a digest that does not match its ciphertext. We
    // therefore copy the digest only when we actually placed the source ciphertext,
    // keeping the ciphertext and its digest from the same source. Either way the
    // event is marked associated so it is not reprocessed.
    if ciphertext_copied {
        sqlx::query!(
            r#"
            INSERT INTO ciphertext_digest
                (handle, ciphertext, ciphertext128, ciphertext128_format, host_chain_id, key_id_gw)
            SELECT $1, ciphertext, ciphertext128, ciphertext128_format, $2, key_id_gw
            FROM ciphertext_digest
            WHERE handle = $3
            ON CONFLICT (handle) DO NOTHING
            "#,
            dst_handle,
            dst_chain_id,
            src_handle,
        )
        .execute(txn.as_mut())
        .await?;
    }

    sqlx::query!(
        "UPDATE handle_bridged_events SET is_associated = true WHERE id = $1",
        id,
    )
    .execute(txn.as_mut())
    .await?;

    Ok(())
}
