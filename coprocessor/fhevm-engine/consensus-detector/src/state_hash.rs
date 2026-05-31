//! Notify-driven state_hash computation + S3 upload of GCS hashes.

use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use aws_sdk_s3::primitives::ByteStream;
use sqlx::{postgres::PgListener, Pool, Postgres};
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

pub const EVENT_CIPHERTEXT_COMPUTED: &str = "event_ciphertext_computed";

// TODO: confirm. RFC 021 defers the state_hash S3
// layout to RFC 023, but RFC 023 only specifies the layout for ciphertext
// objects, not state_hash blobs.
//   <s3BucketUrl>/state_hash/chain=<chain_id>/block=<block_number>.bin
//   body = raw 32-byte SHA-256 (decoded from the hex stored in `state_hash`).
pub fn state_hash_key(chain_id: i64, block_number: i64) -> String {
    format!("state_hash/chain={chain_id}/block={block_number}.bin")
}

/// BCS path: compute hashes from `ONLY ciphertexts`, insert into `state_hash`.
async fn compute_and_insert_bcs(pool: &Pool<Postgres>, batch_limit: i64) -> anyhow::Result<()> {
    let pending = sqlx::query!(
        r#"
        SELECT c.host_chain_id AS "host_chain_id!", c.block_number AS "block_number!"
          FROM computations c
         WHERE c.is_completed = true
           AND c.is_error = false
           AND c.block_number IS NOT NULL
           AND NOT EXISTS (
               SELECT 1 FROM state_hash sh
                WHERE sh.chain_id = c.host_chain_id AND sh.block_number = c.block_number)
         GROUP BY c.host_chain_id, c.block_number
         ORDER BY c.block_number
         LIMIT $1
        "#,
        batch_limit
    )
    .fetch_all(pool)
    .await?;

    for row in pending {
        let chain_id = row.host_chain_id;
        let block_number = row.block_number;
        // Errored computations are excluded so operators that classify errors
        // differently still produce stable hashes for the successful subset.
        let hashed = sqlx::query!(
            r#"
            WITH bc AS (
                SELECT output_handle, tenant_id, is_completed
                  FROM computations
                 WHERE host_chain_id = $1 AND block_number = $2 AND is_error = false
            ),
            v AS (SELECT 1 FROM bc HAVING bool_and(is_completed))
            SELECT encode(
                sha256(string_agg(ct.ciphertext, ''::bytea
                                  ORDER BY ct.handle, ct.ciphertext_version)),
                'hex'
            ) AS state_hash
              FROM v CROSS JOIN bc
              JOIN ONLY ciphertexts ct
                ON ct.tenant_id = bc.tenant_id AND ct.handle = bc.output_handle
            "#,
            chain_id,
            block_number
        )
        .fetch_optional(pool)
        .await?
        .and_then(|r| r.state_hash);
        let Some(hash) = hashed else { continue };
        let affected = sqlx::query!(
            "INSERT INTO state_hash (chain_id, block_number, state_hash) VALUES ($1, $2, $3)
             ON CONFLICT (chain_id, block_number) DO NOTHING",
            chain_id,
            block_number,
            hash,
        )
        .execute(pool)
        .await?
        .rows_affected();
        if affected > 0 {
            info!(chain_id, block_number, "state_hash inserted");
        }
    }
    Ok(())
}

/// GCS path: compute hashes from `gcs.ciphertexts`, insert into `gcs.state_hash`
/// with `s3_uploaded_at = NULL`. Upload is done by `upload_pending_state_hashes`
/// so failed PUTs retry from durable state. Schema is qualified explicitly
/// because the consensus-detector connects with the default `public` search_path
/// and operates on both stacks from a single pool.
async fn compute_and_insert_gcs(
    pool: &Pool<Postgres>,
    start: i64,
    end: i64,
    batch_limit: i64,
) -> anyhow::Result<()> {
    let pending = sqlx::query!(
        r#"
        SELECT c.host_chain_id AS "host_chain_id!", c.block_number AS "block_number!"
          FROM gcs.computations c
         WHERE c.is_completed = true
           AND c.is_error = false
           AND c.block_number BETWEEN $2 AND $3
           AND NOT EXISTS (
               SELECT 1 FROM gcs.state_hash sh
                WHERE sh.chain_id = c.host_chain_id AND sh.block_number = c.block_number)
         GROUP BY c.host_chain_id, c.block_number
         ORDER BY c.block_number
         LIMIT $1
        "#,
        batch_limit,
        start,
        end,
    )
    .fetch_all(pool)
    .await?;

    for row in pending {
        let chain_id = row.host_chain_id;
        let block_number = row.block_number;
        let hashed = sqlx::query!(
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
            block_number
        )
        .fetch_optional(pool)
        .await?
        .and_then(|r| r.state_hash);
        let Some(hash) = hashed else { continue };
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

/// Uploads `gcs.state_hash` rows with `s3_uploaded_at IS NULL`.
/// Stamps `NOW()` on success; failures stay NULL and retry next sweep.
async fn upload_pending_state_hashes(
    pool: &Pool<Postgres>,
    s3: &aws_sdk_s3::Client,
    my_bucket: &str,
    batch_limit: i64,
) -> anyhow::Result<()> {
    let pending = sqlx::query!(
        r#"
        SELECT chain_id AS "chain_id!", block_number AS "block_number!",
               state_hash AS "state_hash!"
          FROM gcs.state_hash
         WHERE s3_uploaded_at IS NULL
         ORDER BY chain_id, block_number
         LIMIT $1
        "#,
        batch_limit,
    )
    .fetch_all(pool)
    .await?;

    for row in pending {
        let chain_id = row.chain_id;
        let block_number = row.block_number;
        let bytes = hex::decode(&row.state_hash)
            .with_context(|| format!("decode state_hash for ({chain_id}, {block_number})"))?;
        let key = state_hash_key(chain_id, block_number);
        match s3
            .put_object()
            .bucket(my_bucket)
            .key(&key)
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
                info!(chain_id, block_number, bucket = my_bucket, "state_hash uploaded");
            }
            Err(e) => {
                warn!(chain_id, block_number, error = %e, "state_hash upload failed; will retry");
            }
        }
    }
    Ok(())
}

/// Gateway BCS path: hash `input_handles → ONLY ciphertexts` per Gateway block, write under `gw_chain_id`.
async fn compute_and_insert_bcs_gateway(
    pool: &Pool<Postgres>,
    gw_chain_id: i64,
    batch_limit: i64,
) -> anyhow::Result<()> {
    let pending = sqlx::query!(
        r#"
        SELECT DISTINCT ih.block_number AS "block_number!"
          FROM input_handles ih
         WHERE NOT EXISTS (
             SELECT 1 FROM state_hash sh
              WHERE sh.chain_id = $1 AND sh.block_number = ih.block_number)
         ORDER BY ih.block_number
         LIMIT $2
        "#,
        gw_chain_id,
        batch_limit,
    )
    .fetch_all(pool)
    .await?;

    for row in pending {
        let block_number = row.block_number;
        let hashed = sqlx::query!(
            r#"
            SELECT encode(
                sha256(string_agg(ct.ciphertext, ''::bytea
                                  ORDER BY ct.handle, ct.ciphertext_version)),
                'hex'
            ) AS state_hash
              FROM input_handles ih
              JOIN ONLY ciphertexts ct ON ct.handle = ih.handle
             WHERE ih.block_number = $1
            "#,
            block_number
        )
        .fetch_optional(pool)
        .await?
        .and_then(|r| r.state_hash);
        let Some(hash) = hashed else { continue };
        let affected = sqlx::query!(
            "INSERT INTO state_hash (chain_id, block_number, state_hash) VALUES ($1, $2, $3)
             ON CONFLICT (chain_id, block_number) DO NOTHING",
            gw_chain_id,
            block_number,
            hash,
        )
        .execute(pool)
        .await?
        .rows_affected();
        if affected > 0 {
            info!(chain_id = gw_chain_id, block_number, "gateway state_hash inserted");
        }
    }
    Ok(())
}

async fn sweep(
    pool: &Pool<Postgres>,
    s3: Option<&aws_sdk_s3::Client>,
    my_bucket: &str,
    gw_chain_id: i64,
    batch_limit: i64,
) -> anyhow::Result<()> {
    // BCS path: always runs.
    compute_and_insert_bcs(pool, batch_limit).await?;
    compute_and_insert_bcs_gateway(pool, gw_chain_id, batch_limit).await?;

    // GCS path: only during an active upgrade window. The Gateway-side GCS
    // path is intentionally absent until `zkproof-worker` is GCS-aware and
    // writes inputs to `ciphertexts_staging`; without it, a sweep over
    // `ONLY ciphertexts_staging` joined with `input_handles` would always
    // be empty.
    if let Some((start, end)) = crate::active_upgrade_window(pool).await? {
        compute_and_insert_gcs(pool, start, end, batch_limit).await?;
    }

    // S3 upload: drains pending rows; runs every sweep so failed PUTs retry.
    if let Some(s3) = s3 {
        upload_pending_state_hashes(pool, s3, my_bucket, batch_limit).await?;
    }
    Ok(())
}

/// Pure pg-notify driven: startup sweep + sweep on each `event_ciphertext_computed`.
pub async fn run(
    pool: Pool<Postgres>,
    s3: Option<Arc<aws_sdk_s3::Client>>,
    my_bucket: String,
    gw_chain_id: i64,
    batch_limit: i64,
    cancel: CancellationToken,
) -> anyhow::Result<()> {
    info!(batch_limit, bucket = %my_bucket, gw_chain_id, "starting state_hash worker");

    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen(EVENT_CIPHERTEXT_COMPUTED).await?;

    if let Err(e) = sweep(&pool, s3.as_deref(), &my_bucket, gw_chain_id, batch_limit).await {
        warn!(error = %e, "initial sweep failed");
    }
    loop {
        select! {
            _ = cancel.cancelled() => return Ok(()),
            recv = listener.recv() => match recv {
                Ok(_) => {
                    debug!("event_ciphertext_computed received");
                    if let Err(e) = sweep(&pool, s3.as_deref(), &my_bucket, gw_chain_id, batch_limit).await {
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
}
