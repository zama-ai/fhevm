//! The `pads` rows of one lane: claim, output, failure, upload, GC, and the lane heartbeat.

use alloy::primitives::{Address, B256};
use ciphertext_attestation::{prf::Version, PrfOutputAttestation};
use sqlx::PgPool;
use std::time::Duration;

/// Backoff of a temporary error, in seconds: this step for each attempt, up to the max.
pub const RETRY_BACKOFF_STEP_SECS: i32 = 5;
pub const RETRY_BACKOFF_MAX_SECS: i32 = 60;

/// A claimed row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Claim {
    pub prf_instance: u16,
    pub label: B256,
    pub pad_length: u16,
}

/// Claims the next pending row of `prf_instance`, if any. The connector with the fewest rows in
/// progress goes first, then the oldest row. Skips expired rows and rows with a live claim.
pub async fn claim(
    pool: &PgPool,
    prf_instance: u16,
    pending_max_age: Duration,
    claim_timeout: Duration,
) -> sqlx::Result<Option<Claim>> {
    let mut tx = pool.begin().await?;
    let row = sqlx::query!(
        r#"WITH in_progress AS (
               SELECT triggered_by, COUNT(*) AS rows
                 FROM pads
                WHERE prf_instance = $1 AND signature IS NULL AND NOT is_error
                  AND claimed_at > NOW() - make_interval(secs => $3)
                GROUP BY triggered_by
           )
           SELECT p.label, p.pad_length
             FROM pads p
             LEFT JOIN in_progress i ON i.triggered_by = p.triggered_by
            WHERE p.prf_instance = $1 AND p.signature IS NULL AND NOT p.is_error
              AND p.created_at > NOW() - make_interval(secs => $2)
              AND (p.claimed_at IS NULL OR p.claimed_at <= NOW() - make_interval(secs => $3))
            ORDER BY COALESCE(i.rows, 0), p.created_at
            LIMIT 1
              FOR UPDATE OF p SKIP LOCKED"#,
        i32::from(prf_instance),
        pending_max_age.as_secs_f64(),
        claim_timeout.as_secs_f64(),
    )
    .fetch_optional(tx.as_mut())
    .await?;
    let Some(row) = row else {
        return Ok(None);
    };
    sqlx::query!(
        "UPDATE pads SET claimed_at = NOW()
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3",
        i32::from(prf_instance),
        row.label,
        row.pad_length,
    )
    .execute(tx.as_mut())
    .await?;
    tx.commit().await?;
    Ok(Some(Claim {
        prf_instance,
        label: B256::try_from(row.label.as_slice()).expect("label is 32 bytes"),
        pad_length: u16::try_from(row.pad_length).expect("pad_length is in range"),
    }))
}

/// Writes the outputs and the attestation; the trigger sends `pad_ready`. Returns false if the row
/// was deleted or completed.
pub async fn write_output(
    pool: &PgPool,
    claim: &Claim,
    ct64: &[u8],
    ct128: &[u8],
    attestation: &PrfOutputAttestation,
) -> sqlx::Result<bool> {
    let result = sqlx::query!(
        "UPDATE pads
            SET ciphertext128 = $4, ciphertext_digest = $5, ciphertext128_digest = $6,
                signer = $7, signature = $8, attestation_version = $9, ciphertext64 = $10
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3
            AND signature IS NULL AND NOT is_error",
        i32::from(claim.prf_instance),
        claim.label.as_slice(),
        i32::from(claim.pad_length),
        ct128,
        attestation.ciphertext_digest.as_slice(),
        attestation.sns_ciphertext_digest.as_slice(),
        attestation.signer.as_slice(),
        attestation.signature,
        i16::from(u8::from(attestation.version)),
        ct64,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}

/// Fails the row for good. The trigger then sends `pad_ready`.
pub async fn fail(pool: &PgPool, claim: &Claim, message: &str) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE pads SET is_error = TRUE, error_message = $4
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3
            AND signature IS NULL AND NOT is_error",
        i32::from(claim.prf_instance),
        claim.label.as_slice(),
        i32::from(claim.pad_length),
        message,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// After a temporary error: holds the claim for a backoff that grows with each attempt. Never fails
/// the row: a row that keeps failing expires.
pub async fn retry(
    pool: &PgPool,
    claim: &Claim,
    message: &str,
    claim_timeout: Duration,
) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE pads
            SET attempts = attempts + 1, error_message = $4,
                claimed_at = NOW() - make_interval(secs => $5)
                             + make_interval(secs => LEAST((attempts + 1) * $6, $7))
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3
            AND signature IS NULL AND NOT is_error",
        i32::from(claim.prf_instance),
        claim.label.as_slice(),
        i32::from(claim.pad_length),
        message,
        claim_timeout.as_secs_f64(),
        RETRY_BACKOFF_STEP_SECS,
        RETRY_BACKOFF_MAX_SECS,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// The agreement of the operators on the reference digest of an instance.
pub struct LaneAgreement {
    pub reference_digest: Option<B256>,
    pub mismatch: bool,
}

pub async fn lane_agreement(pool: &PgPool, prf_instance: u16) -> sqlx::Result<LaneAgreement> {
    let row = sqlx::query!(
        r#"SELECT reference_digest, reference_mismatch_at IS NOT NULL AS "mismatch!"
             FROM prf_lanes WHERE prf_instance = $1"#,
        i32::from(prf_instance),
    )
    .fetch_one(pool)
    .await?;
    Ok(LaneAgreement {
        reference_digest: row
            .reference_digest
            .map(|d| B256::try_from(d.as_slice()).expect("reference_digest is 32 bytes")),
        mismatch: row.mismatch,
    })
}

/// Marks the lane as ready and alive.
pub async fn set_ready(pool: &PgPool, prf_instance: u16) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE prf_lanes SET ready_at = COALESCE(ready_at, NOW()), heartbeat_at = NOW()
          WHERE prf_instance = $1",
        i32::from(prf_instance),
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Marks the lane as alive.
pub async fn heartbeat(pool: &PgPool, prf_instance: u16) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE prf_lanes SET heartbeat_at = NOW() WHERE prf_instance = $1",
        i32::from(prf_instance),
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// A complete row to upload to S3.
pub struct Output {
    pub claim: Claim,
    pub ct128: Vec<u8>,
    pub attestation: PrfOutputAttestation,
}

/// Claims up to `limit` complete rows that have no S3 copy and no live claim.
pub async fn claim_uploads(
    pool: &PgPool,
    prf_instance: u16,
    claim_timeout: Duration,
    limit: i64,
) -> sqlx::Result<Vec<Output>> {
    let rows = sqlx::query!(
        r#"UPDATE pads SET claimed_at = NOW()
            WHERE (prf_instance, label, pad_length) IN (
                  SELECT prf_instance, label, pad_length
                    FROM pads
                   WHERE prf_instance = $1 AND signature IS NOT NULL AND s3_verified_at IS NULL
                     AND (claimed_at IS NULL OR claimed_at <= NOW() - make_interval(secs => $2))
                   ORDER BY created_at
                   LIMIT $3
                     FOR UPDATE SKIP LOCKED)
        RETURNING label, pad_length, ciphertext128 AS "ciphertext128!",
                  ciphertext_digest AS "ciphertext_digest!",
                  ciphertext128_digest AS "ciphertext128_digest!", signer AS "signer!",
                  signature AS "signature!", attestation_version AS "attestation_version!""#,
        i32::from(prf_instance),
        claim_timeout.as_secs_f64(),
        limit,
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| {
            let version = u8::try_from(row.attestation_version)
                .ok()
                .and_then(|v| Version::try_from(v).ok())
                .ok_or_else(|| decode_error("attestation_version"))?;
            Ok(Output {
                claim: Claim {
                    prf_instance,
                    label: B256::try_from(row.label.as_slice()).expect("label is 32 bytes"),
                    pad_length: u16::try_from(row.pad_length).expect("pad_length is in range"),
                },
                ct128: row.ciphertext128,
                attestation: PrfOutputAttestation {
                    version,
                    ciphertext_digest: B256::try_from(row.ciphertext_digest.as_slice())
                        .map_err(|_| decode_error("ciphertext_digest"))?,
                    sns_ciphertext_digest: B256::try_from(row.ciphertext128_digest.as_slice())
                        .map_err(|_| decode_error("ciphertext128_digest"))?,
                    signer: Address::try_from(row.signer.as_slice())
                        .map_err(|_| decode_error("signer"))?,
                    signature: row.signature,
                },
            })
        })
        .collect()
}

fn decode_error(column: &str) -> sqlx::Error {
    sqlx::Error::Decode(format!("bad {column}").into())
}

/// Sets `s3_verified_at`.
pub async fn set_uploaded(pool: &PgPool, claim: &Claim) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE pads SET s3_verified_at = NOW()
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3",
        i32::from(claim.prf_instance),
        claim.label.as_slice(),
        i32::from(claim.pad_length),
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Deletes uploaded or failed rows older than `max_age`, of all instances (also retired lanes).
pub async fn delete_old_outputs(pool: &PgPool, max_age: Duration) -> sqlx::Result<u64> {
    let result = sqlx::query!(
        "DELETE FROM pads
          WHERE (s3_verified_at IS NOT NULL OR is_error)
            AND created_at <= NOW() - make_interval(secs => $1)",
        max_age.as_secs_f64(),
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

/// Deletes expired pending rows with no live claim, of all instances (also dead lanes).
pub async fn delete_expired_pending(
    pool: &PgPool,
    pending_max_age: Duration,
    claim_timeout: Duration,
) -> sqlx::Result<u64> {
    let result = sqlx::query!(
        "DELETE FROM pads
          WHERE signature IS NULL AND NOT is_error
            AND created_at <= NOW() - make_interval(secs => $1)
            AND (claimed_at IS NULL OR claimed_at <= NOW() - make_interval(secs => $2))",
        pending_max_age.as_secs_f64(),
        claim_timeout.as_secs_f64(),
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

/// Clears the ct64 of the rows of all instances older than `max_age`. The digest stays.
pub async fn clear_old_ct64(pool: &PgPool, max_age: Duration) -> sqlx::Result<u64> {
    let result = sqlx::query!(
        "UPDATE pads SET ciphertext64 = NULL
          WHERE ciphertext64 IS NOT NULL AND created_at <= NOW() - make_interval(secs => $1)",
        max_age.as_secs_f64(),
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

/// The bytes of all stored ct64.
pub async fn stored_ct64_bytes(pool: &PgPool) -> sqlx::Result<i64> {
    sqlx::query_scalar!(
        r#"SELECT COALESCE(SUM(OCTET_LENGTH(ciphertext64)), 0)::BIGINT AS "bytes!"
             FROM pads WHERE ciphertext64 IS NOT NULL"#,
    )
    .fetch_one(pool)
    .await
}

/// The age in seconds of the oldest row of an instance, for each state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OldestRows {
    pub prf_instance: u16,
    pub pending: Option<f64>,
    pub not_uploaded: Option<f64>,
}

pub async fn oldest_rows(pool: &PgPool) -> sqlx::Result<Vec<OldestRows>> {
    let rows = sqlx::query!(
        r#"SELECT prf_instance,
                  EXTRACT(EPOCH FROM NOW() - MIN(created_at)
                      FILTER (WHERE signature IS NULL AND NOT is_error))::FLOAT8 AS pending,
                  EXTRACT(EPOCH FROM NOW() - MIN(created_at)
                      FILTER (WHERE signature IS NOT NULL AND s3_verified_at IS NULL))::FLOAT8
                      AS not_uploaded
             FROM pads
            GROUP BY prf_instance"#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| OldestRows {
            prf_instance: u16::try_from(row.prf_instance).expect("prf_instance is in range"),
            pending: row.pending,
            not_uploaded: row.not_uploaded,
        })
        .collect())
}
