//! Notify-driven state_hash computation + S3 upload of GCS hashes.

use std::sync::Arc;
use std::time::Duration;

use aws_sdk_s3::primitives::ByteStream;
use sqlx::{postgres::PgListener, Pool, Postgres};
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

/// GCS path: stamp `gcs.state_hash` for blocks in `[start, end]` that don't
/// already have an entry. Empty blocks (`fhe_event_count = 0`) get
/// [`EMPTY_BLOCK_STATE_HASH`]; non-empty blocks get the SHA-256 over their
/// `gcs.ciphertexts`.
async fn compute_and_insert_gcs(
    pool: &Pool<Postgres>,
    start: i64,
    end: i64,
    batch_limit: i64,
) -> anyhow::Result<()> {
    // Sourced from `gcs.host_chain_blocks_valid` (not `public.*`): only the
    // GCS schema is guaranteed populated for the active window. `bool_and`
    // treats a block as empty iff every row reports zero FHE events.
    let pending = sqlx::query!(
        r#"
        SELECT b.chain_id AS "chain_id!", b.block_number AS "block_number!",
               bool_and(b.fhe_event_count = 0) AS "is_empty!"
          FROM gcs.host_chain_blocks_valid b
         WHERE b.block_number BETWEEN $1 AND $2
           AND NOT EXISTS (
               SELECT 1 FROM gcs.state_hash sh
                WHERE sh.chain_id = b.chain_id AND sh.block_number = b.block_number)
         GROUP BY b.chain_id, b.block_number
         ORDER BY b.block_number
         LIMIT $3
        "#,
        start,
        end,
        batch_limit,
    )
    .fetch_all(pool)
    .await?;

    for row in pending {
        let chain_id = row.chain_id;
        let block_number = row.block_number;
        let hash = if row.is_empty {
            Some(EMPTY_BLOCK_STATE_HASH.to_string())
        } else {
            compute_gcs_hash(pool, chain_id, block_number).await?
        };
        let Some(hash) = hash else { continue };
        let affected = sqlx::query!(
            "INSERT INTO gcs.state_hash (chain_id, block_number, state_hash)
             VALUES ($1, $2, $3)
             ON CONFLICT (chain_id, block_number) DO NOTHING",
            chain_id,
            block_number,
            hash,
        )
        .execute(pool)
        .await?
        .rows_affected();
        if affected > 0 {
            info!(chain_id, block_number, "gcs.state_hash inserted");
        }
    }
    Ok(())
}

/// SHA-256 over the block's `gcs.ciphertexts`; `None` until every non-errored
/// computation completes. Errored computations are excluded so operators agree.
async fn compute_gcs_hash(
    pool: &Pool<Postgres>,
    chain_id: i64,
    block_number: i64,
) -> anyhow::Result<Option<String>> {
    let row = sqlx::query!(
        r#"
        WITH bc AS (
            SELECT output_handle, tenant_id, is_completed
              FROM gcs.computations
             WHERE host_chain_id = $1 AND block_number = $2 AND is_error = false
        ),
        v AS (SELECT 1 FROM bc HAVING bool_and(is_completed))
        SELECT encode(
            sha256(string_agg(ct.ciphertext, ''::bytea
                              ORDER BY ct.handle, ct.ciphertext_version)),
            'hex'
        ) AS state_hash
          FROM v CROSS JOIN bc
          JOIN gcs.ciphertexts ct
            ON ct.tenant_id = bc.tenant_id AND ct.handle = bc.output_handle
        "#,
        chain_id,
        block_number,
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.and_then(|r| r.state_hash))
}

/// Uploads `gcs.state_hash` rows with `s3_uploaded_at IS NULL`, attaching the
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
          FROM gcs.state_hash sh
          JOIN gcs.host_chain_blocks_valid b
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
                    "UPDATE gcs.state_hash SET s3_uploaded_at = NOW()
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

async fn sweep(
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

    // S3 upload: drains pending rows; runs every sweep so failed PUTs retry.
    if let Some(s3) = s3 {
        upload_pending_state_hashes(pool, s3, my_bucket, batch_limit).await?;
    }
    Ok(())
}

/// Pure pg-notify driven: startup sweep + sweep on `event_ciphertext_computed`
/// or `event_new_block`. Listening on `event_new_block` covers windows with no
/// FHE activity — `event_ciphertext_computed` only fires when work happens, so
/// empty dry-run windows would otherwise never produce sentinel rows.
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

    if let Err(e) = sweep(&pool, s3.as_deref(), &my_bucket, batch_limit).await {
        warn!(error = %e, "initial sweep failed");
    }
    loop {
        select! {
            _ = cancel.cancelled() => return Ok(()),
            recv = listener.recv() => match recv {
                Ok(n) => {
                    debug!(channel = n.channel(), "sweep trigger received");
                    if let Err(e) = sweep(&pool, s3.as_deref(), &my_bucket, batch_limit).await {
                        warn!(error = %e, "sweep failed");
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
