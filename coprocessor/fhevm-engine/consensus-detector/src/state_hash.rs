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

async fn active_window(pool: &Pool<Postgres>) -> sqlx::Result<Option<(i64, i64)>> {
    let row: Option<(String, Option<i64>, Option<i64>)> = sqlx::query_as(
        "SELECT state, start_block, end_block FROM upgrade_state
          WHERE stack_role = 'GCS' AND status = 'in_progress'",
    )
    .fetch_optional(pool)
    .await?;
    Ok(match row {
        Some((s, Some(start), Some(end)))
            if matches!(s.as_str(), "UpgradeActivated" | "DryRunStarted") =>
        {
            Some((start, end))
        }
        _ => None,
    })
}

/// BCS path: compute hashes from `ONLY ciphertexts`, insert into `state_hash`.
async fn compute_and_insert_bcs(pool: &Pool<Postgres>, batch_limit: i64) -> anyhow::Result<()> {
    let pending = sqlx::query!(
        r#"
        SELECT c.host_chain_id AS "host_chain_id!", c.block_number AS "block_number!"
          FROM computations c
         WHERE c.is_completed = true
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
        let hashed = sqlx::query!(
            r#"
            WITH bc AS (
                SELECT output_handle, tenant_id, is_completed
                  FROM computations WHERE block_number = $1
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

/// GCS path: compute hashes from `ONLY ciphertexts_staging`, insert into
/// `state_hash_staging`, return inserted rows for S3 upload.
async fn compute_and_insert_gcs(
    pool: &Pool<Postgres>,
    start: i64,
    end: i64,
    batch_limit: i64,
) -> anyhow::Result<Vec<(i64, i64, String)>> {
    let pending = sqlx::query!(
        r#"
        SELECT c.host_chain_id AS "host_chain_id!", c.block_number AS "block_number!"
          FROM computations c
         WHERE c.is_completed = true
           AND c.block_number BETWEEN $2 AND $3
           AND NOT EXISTS (
               SELECT 1 FROM state_hash_staging sh
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

    let mut inserted = Vec::new();
    for row in pending {
        let chain_id = row.host_chain_id;
        let block_number = row.block_number;
        let hashed = sqlx::query!(
            r#"
            WITH bc AS (
                SELECT output_handle, tenant_id, is_completed
                  FROM computations WHERE block_number = $1
            ),
            v AS (SELECT 1 FROM bc HAVING bool_and(is_completed))
            SELECT encode(
                sha256(string_agg(ct.ciphertext, ''::bytea
                                  ORDER BY ct.handle, ct.ciphertext_version)),
                'hex'
            ) AS state_hash
              FROM v CROSS JOIN bc
              JOIN ONLY ciphertexts_staging ct
                ON ct.tenant_id = bc.tenant_id AND ct.handle = bc.output_handle
            "#,
            block_number
        )
        .fetch_optional(pool)
        .await?
        .and_then(|r| r.state_hash);
        let Some(hash) = hashed else { continue };
        let affected = sqlx::query!(
            "INSERT INTO state_hash_staging (chain_id, block_number, state_hash)
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
            info!(chain_id, block_number, "state_hash_staging inserted");
            inserted.push((chain_id, block_number, hash));
        }
    }
    Ok(inserted)
}

async fn sweep(
    pool: &Pool<Postgres>,
    s3: Option<&aws_sdk_s3::Client>,
    my_bucket: &str,
    batch_limit: i64,
) -> anyhow::Result<()> {
    // BCS path: always runs.
    compute_and_insert_bcs(pool, batch_limit).await?;

    // GCS path: only during an active upgrade window with a configured bucket.
    let (Some(s3), Some((start, end))) = (s3, active_window(pool).await?) else {
        return Ok(());
    };
    let inserted = compute_and_insert_gcs(pool, start, end, batch_limit).await?;
    for (chain_id, block_number, hex_hash) in inserted {
        let bytes = hex::decode(&hex_hash)
            .with_context(|| format!("decode state_hash for {block_number}"))?;
        let key = state_hash_key(chain_id, block_number);
        if let Err(e) = s3
            .put_object()
            .bucket(my_bucket)
            .key(&key)
            .body(ByteStream::from(bytes))
            .send()
            .await
        {
            warn!(chain_id, block_number, error = %e, "state_hash upload failed");
        } else {
            info!(chain_id, block_number, bucket = my_bucket, "state_hash uploaded");
        }
    }
    Ok(())
}

/// Pure pg-notify driven: startup sweep + sweep on each `event_ciphertext_computed`.
pub async fn run(
    pool: Pool<Postgres>,
    s3: Option<Arc<aws_sdk_s3::Client>>,
    my_bucket: String,
    batch_limit: i64,
    cancel: CancellationToken,
) -> anyhow::Result<()> {
    info!(batch_limit, bucket = %my_bucket, "starting state_hash worker");

    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen(EVENT_CIPHERTEXT_COMPUTED).await?;

    if let Err(e) = sweep(&pool, s3.as_deref(), &my_bucket, batch_limit).await {
        warn!(error = %e, "initial sweep failed");
    }
    loop {
        select! {
            _ = cancel.cancelled() => return Ok(()),
            recv = listener.recv() => match recv {
                Ok(_) => {
                    debug!("event_ciphertext_computed received");
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
}
