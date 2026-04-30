//! Readiness predicates for "the coprocessor stack is fully settled at
//! block N." Used in two places:
//!
//! 1. Before triggering `pg_dump`, the BCS must be settled at `snapshotBlock`.
//! 2. Before transitioning REPLAYING -> READY, the GCS must be settled at
//!    `evalBlock`.
//!
//! The same predicate set works for both: every compute output through the
//! given block has finished, every Allow-event SNS row is complete, and the
//! ciphertext registration tx has been submitted.
//!
//! The function takes whichever pool the caller wants to check against:
//! the `proposedUpgrade` handler passes a transient BCS pool for
//! `snapshotBlock`, and the GCS pool for `evalBlock`. The schema requirements
//! (`computations.block_number`, `pbs_computations.block_number`,
//! `ciphertext_digest.txn_is_sent`) are identical on both DBs.

use anyhow::{Context, Result};
use sqlx::{PgPool, Row};

#[derive(Clone, Copy, Debug)]
pub struct Readiness {
    pub compute_done: bool,
    pub sns_done: bool,
    pub tx_done: bool,
}

impl Readiness {
    pub fn fully_settled(&self) -> bool {
        self.compute_done && self.sns_done && self.tx_done
    }
}

pub async fn check_settled_at(pool: &PgPool, block_number: i64) -> Result<Readiness> {
    let row = sqlx::query(
        r#"
        SELECT
            NOT EXISTS (
                SELECT 1 FROM computations
                WHERE  block_number IS NOT NULL
                  AND  block_number <= $1
                  AND  is_completed = FALSE
            ) AS compute_done,

            NOT EXISTS (
                SELECT 1 FROM pbs_computations
                WHERE  block_number IS NOT NULL
                  AND  block_number <= $1
                  AND  is_completed = FALSE
            ) AS sns_done,

            NOT EXISTS (
                SELECT 1
                FROM   ciphertext_digest cd
                JOIN   computations c ON c.output_handle = cd.handle
                WHERE  c.block_number IS NOT NULL
                  AND  c.block_number <= $1
                  AND  cd.txn_is_sent = FALSE
            ) AS tx_done
        "#,
    )
    .bind(block_number)
    .fetch_one(pool)
    .await
    .context("readiness check query")?;

    Ok(Readiness {
        compute_done: row.try_get("compute_done")?,
        sns_done: row.try_get("sns_done")?,
        tx_done: row.try_get("tx_done")?,
    })
}
