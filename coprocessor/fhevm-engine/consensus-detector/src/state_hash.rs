//! Notify-driven state_hash computation + S3 upload of GCS hashes.

use std::sync::Arc;
use std::time::Duration;

use aws_sdk_s3::primitives::ByteStream;
use fhevm_engine_common::database::GCS_SCHEMA_QUOTED;
use sqlx::{postgres::PgListener, Pool, Postgres, Row};
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

pub const EVENT_CIPHERTEXT_COMPUTED: &str = "event_ciphertext_computed";

/// Sentinel for blocks with `fhe_event_count = 0`. Generated via
/// `printf '' | shasum -a 256` (equivalently `encode(sha256(''::bytea), 'hex')`).
pub const EMPTY_BLOCK_STATE_HASH: &str =
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

// TODO: confirm. RFC 021 defers the state_hash S3
// layout to RFC 023, but RFC 023 only specifies the layout for ciphertext
// objects, not state_hash blobs.
//   <s3BucketUrl>/state_hash/chain=<chain_id>/block=<block_number>.bin
//   body = raw 32-byte SHA-256 (decoded from the hex stored in `state_hash`).
pub fn state_hash_key(chain_id: i64, block_number: i64) -> String {
    format!("state_hash/chain={chain_id}/block={block_number}.bin")
}

/// GCS path: stamp the GCS `state_hash` for blocks in `[start, end]` that don't
/// already have an entry. Empty blocks (`fhe_event_count = 0`) get
/// [`EMPTY_BLOCK_STATE_HASH`]; non-empty blocks get the SHA-256 over their
/// GCS `ciphertexts`.
async fn compute_and_insert_gcs(
    pool: &Pool<Postgres>,
    start: i64,
    end: i64,
    batch_limit: i64,
) -> anyhow::Result<()> {
    // This is the write path, so both the pending-scan read and the INSERT
    // explicitly qualify the versioned GCS schema (`GCS_SCHEMA_QUOTED`, e.g.
    // `"gcs-0.14.0"`) instead of relying on the connection search_path. An
    // accidental fallback to `public.state_hash` would corrupt the live (BCS)
    // table, so these must never resolve anywhere but the GCS schema — and a
    // missing GCS schema errors loudly rather than silently writing to public.
    // Runtime `sqlx::query` (not the `query!` macro) because the schema name is
    // only known at runtime. `bool_and` treats a block as empty iff every row
    // reports zero FHE events.
    let pending_sql = format!(
        r#"
        SELECT b.chain_id, b.block_number,
               bool_and(b.fhe_event_count = 0) AS is_empty
          FROM {GCS_SCHEMA_QUOTED}.host_chain_blocks_valid b
         WHERE b.block_number BETWEEN $1 AND $2
           AND NOT EXISTS (
               SELECT 1 FROM {GCS_SCHEMA_QUOTED}.state_hash sh
                WHERE sh.chain_id = b.chain_id AND sh.block_number = b.block_number)
         GROUP BY b.chain_id, b.block_number
         ORDER BY b.block_number
         LIMIT $3
        "#
    );
    let pending = sqlx::query(&pending_sql)
        .bind(start)
        .bind(end)
        .bind(batch_limit)
        .fetch_all(pool)
        .await?;

    let insert_sql = format!(
        "INSERT INTO {GCS_SCHEMA_QUOTED}.state_hash (chain_id, block_number, state_hash)
         VALUES ($1, $2, $3)
         ON CONFLICT (chain_id, block_number) DO NOTHING"
    );
    for row in pending {
        let chain_id: i64 = row.try_get("chain_id")?;
        let block_number: i64 = row.try_get("block_number")?;
        let is_empty: bool = row.try_get("is_empty")?;
        let hash = if is_empty {
            Some(EMPTY_BLOCK_STATE_HASH.to_string())
        } else {
            compute_gcs_hash(pool, chain_id, block_number).await?
        };
        let Some(hash) = hash else { continue };
        let affected = sqlx::query(&insert_sql)
            .bind(chain_id)
            .bind(block_number)
            .bind(&hash)
            .execute(pool)
            .await?
            .rows_affected();
        if affected > 0 {
            info!(chain_id, block_number, "gcs.state_hash inserted");
        }
    }
    Ok(())
}

/// SHA-256 over the block's GCS `ciphertexts`; `None` until every non-errored
/// computation completes. Errored computations are excluded so operators agree.
///
/// Like [`compute_and_insert_gcs`], the source tables are explicitly qualified
/// with the versioned GCS schema (`GCS_SCHEMA_QUOTED`) and never fall back to
/// `public`: hashing the live (BCS) `computations`/`ciphertexts` would silently
/// produce the wrong digest, so a missing GCS schema must error rather than
/// read public. Runtime `sqlx::query` because the schema name is only known at
/// runtime.
async fn compute_gcs_hash(
    pool: &Pool<Postgres>,
    chain_id: i64,
    block_number: i64,
) -> anyhow::Result<Option<String>> {
    let sql = format!(
        r#"
        WITH bc AS (
            SELECT output_handle, tenant_id, is_completed
              FROM {GCS_SCHEMA_QUOTED}.computations
             WHERE host_chain_id = $1 AND block_number = $2 AND is_error = false
        ),
        v AS (SELECT 1 FROM bc HAVING bool_and(is_completed))
        SELECT encode(
            sha256(string_agg(ct.ciphertext, ''::bytea
                              ORDER BY ct.handle, ct.ciphertext_version)),
            'hex'
        ) AS state_hash
          FROM v CROSS JOIN bc
          JOIN {GCS_SCHEMA_QUOTED}.ciphertexts ct
            ON ct.tenant_id = bc.tenant_id AND ct.handle = bc.output_handle
        "#
    );
    let Some(row) = sqlx::query(&sql)
        .bind(chain_id)
        .bind(block_number)
        .fetch_optional(pool)
        .await?
    else {
        return Ok(None);
    };
    let state_hash: Option<String> = row.try_get("state_hash")?;
    Ok(state_hash)
}

/// Uploads GCS `state_hash` rows with `s3_uploaded_at IS NULL`, attaching the
/// host-chain block_hash as S3 object metadata. Stamps `NOW()` on success;
/// failures stay NULL and retry next sweep.
async fn upload_pending_state_hashes(
    pool: &Pool<Postgres>,
    s3: &aws_sdk_s3::Client,
    my_bucket: &str,
    batch_limit: i64,
) -> anyhow::Result<()> {
    let pending = sqlx::query!(
        r#"
        SELECT sh.chain_id AS "chain_id!", sh.block_number AS "block_number!",
               sh.state_hash AS "state_hash!", b.block_hash AS "block_hash!"
          FROM state_hash sh
          JOIN host_chain_blocks_valid b
            ON b.chain_id = sh.chain_id AND b.block_number = sh.block_number
         WHERE sh.s3_uploaded_at IS NULL
         ORDER BY sh.chain_id, sh.block_number
         LIMIT $1
        "#,
        batch_limit,
    )
    .fetch_all(pool)
    .await?;

    for row in pending {
        let chain_id = row.chain_id;
        let block_number = row.block_number;
        let bytes = match hex::decode(&row.state_hash) {
            Ok(b) => b,
            Err(e) => {
                warn!(chain_id, block_number, error = %e, "malformed state_hash hex in DB; skipping row");
                continue;
            }
        };
        let block_hash_hex = format!("0x{}", hex::encode(&row.block_hash));
        let key = state_hash_key(chain_id, block_number);
        match s3
            .put_object()
            .bucket(my_bucket)
            .key(&key)
            .metadata("block-hash", &block_hash_hex)
            .body(ByteStream::from(bytes))
            .send()
            .await
        {
            Ok(_) => {
                sqlx::query!(
                    "UPDATE state_hash SET s3_uploaded_at = NOW()
                      WHERE chain_id = $1 AND block_number = $2",
                    chain_id,
                    block_number,
                )
                .execute(pool)
                .await?;
                info!(chain_id, block_number, block_hash = %block_hash_hex, bucket = my_bucket, "state_hash uploaded");
            }
            Err(e) => {
                warn!(chain_id, block_number, error = %e, "state_hash upload failed; will retry");
            }
        }
    }
    Ok(())
}

/// One reconciliation pass: compute + insert any pending GCS state hashes for
/// the active upgrade window, then upload any rows not yet in S3. Both steps are
/// idempotent and drain-on-each-call, so a failed insert or PUT simply retries
/// on the next pass.
async fn compute_and_upload_state_hashes(
    pool: &Pool<Postgres>,
    s3: Option<&aws_sdk_s3::Client>,
    my_bucket: &str,
    batch_limit: i64,
) -> anyhow::Result<()> {
    // GCS hashes are only produced during an active upgrade window; BCS hashes
    // are not produced because they would never be uploaded or consumed.
    if let Some((start, end)) = crate::active_upgrade_window(pool).await? {
        compute_and_insert_gcs(pool, start, end, batch_limit).await?;
    }

    // S3 upload: drains pending rows; runs every pass so failed PUTs retry.
    if let Some(s3) = s3 {
        upload_pending_state_hashes(pool, s3, my_bucket, batch_limit).await?;
    }
    Ok(())
}

/// Pure pg-notify driven: one pass at startup, then one on every
/// `event_ciphertext_computed` or `event_new_block`. Listening on
/// `event_new_block` covers windows with no FHE activity —
/// `event_ciphertext_computed` only fires when work happens, so empty dry-run
/// windows would otherwise never produce sentinel rows.
pub async fn run(
    pool: Pool<Postgres>,
    s3: Option<Arc<aws_sdk_s3::Client>>,
    my_bucket: String,
    batch_limit: i64,
    cancel: CancellationToken,
) -> anyhow::Result<()> {
    info!(batch_limit, bucket = %my_bucket, "starting state_hash worker");

    let mut listener = PgListener::connect_with(&pool).await?;
    listener
        .listen_all([EVENT_CIPHERTEXT_COMPUTED, crate::NEW_BLOCK_CHANNEL])
        .await?;

    if let Err(e) = compute_and_upload_state_hashes(&pool, s3.as_deref(), &my_bucket, batch_limit).await
    {
        warn!(error = %e, "initial state_hash pass failed");
    }
    loop {
        select! {
            _ = cancel.cancelled() => return Ok(()),
            recv = listener.recv() => match recv {
                Ok(n) => {
                    debug!(channel = n.channel(), "state_hash pass triggered");
                    if let Err(e) =
                        compute_and_upload_state_hashes(&pool, s3.as_deref(), &my_bucket, batch_limit).await
                    {
                        warn!(error = %e, "state_hash pass failed");
                    }
                }
                Err(e) => {
                    error!(error = %e, "listener recv error; sleeping");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_format_is_stable() {
        assert_eq!(state_hash_key(1, 42), "state_hash/chain=1/block=42.bin");
    }

    #[test]
    fn empty_block_state_hash_is_well_formed() {
        assert_eq!(EMPTY_BLOCK_STATE_HASH.len(), 64);
        assert!(EMPTY_BLOCK_STATE_HASH.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(
            EMPTY_BLOCK_STATE_HASH,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
