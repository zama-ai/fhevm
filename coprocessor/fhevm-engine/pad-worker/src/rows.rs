//! The `pads` rows of one lane: claim, output, failure, and the lane heartbeat.

use alloy::primitives::B256;
use ciphertext_attestation::PrfOutputAttestation;
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

/// Writes the output and its attestation; the trigger sends `pad_ready`. No change if the row was
/// deleted or completed.
pub async fn write_output(
    pool: &PgPool,
    claim: &Claim,
    ct128: &[u8],
    attestation: &PrfOutputAttestation,
) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE pads
            SET ciphertext128 = $4, ciphertext_digest = $5, ciphertext128_digest = $6,
                signer = $7, signature = $8, attestation_version = $9
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
    )
    .execute(pool)
    .await?;
    Ok(())
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
