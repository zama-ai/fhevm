//! `stateCommitment` derivation.
//!
//! For the upgrade round to succeed, every operator must compute the same
//! commitment over the work GCS replayed in `(snapshotBlock, evalBlock]`.
//! That commitment is the keccak256 of every output handle that finished
//! computing in that range, concatenated with the originating block hash,
//! sorted deterministically.
//!
//! Sort order is `(block_hash, output_handle)`. Both fields are byte arrays;
//! Postgres' `bytea` ordering is lexicographic, which is what we want.
//!
//! Schema dependencies (already present, no migration needed):
//! - `computations.output_handle`, `computations.block_number`,
//!   `computations.host_chain_id`, `computations.is_completed`
//! - `host_chain_blocks_valid.(chain_id, block_number, block_hash)`

use anyhow::{Context, Result};
use sha3::{Digest, Keccak256};
use sqlx::{PgPool, Row};
use tracing::info;

/// Compute keccak256 over `(block_hash || output_handle)` for every
/// completed compute output produced in `(snapshot_block, eval_block]`,
/// sorted by `(block_hash, output_handle)`.
///
/// Returns the 32-byte digest. An empty range (no rows) returns
/// `keccak256(<empty>) = c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470`.
pub async fn compute_state_commitment(
    pool: &PgPool,
    snapshot_block: i64,
    eval_block: i64,
) -> Result<[u8; 32]> {
    let rows = sqlx::query(
        r#"
        SELECT c.output_handle, h.block_hash
        FROM   computations c
        JOIN   host_chain_blocks_valid h
          ON   h.chain_id     = c.host_chain_id
         AND   h.block_number = c.block_number
        WHERE  c.block_number >  $1
          AND  c.block_number <= $2
          AND  c.is_completed  = TRUE
        ORDER  BY h.block_hash, c.output_handle
        "#,
    )
    .bind(snapshot_block)
    .bind(eval_block)
    .fetch_all(pool)
    .await
    .context("compute_state_commitment: SELECT computations JOIN host_chain_blocks_valid")?;

    let mut hasher = Keccak256::new();
    let mut count: usize = 0;
    for row in &rows {
        let block_hash: Vec<u8> = row.try_get("block_hash")?;
        let output_handle: Vec<u8> = row.try_get("output_handle")?;
        hasher.update(&block_hash);
        hasher.update(&output_handle);
        count += 1;
    }
    let digest: [u8; 32] = hasher.finalize().into();

    info!(
        snapshot_block,
        eval_block,
        handle_count = count,
        commitment = %hex::encode(digest),
        "Computed stateCommitment"
    );
    Ok(digest)
}

#[cfg(test)]
mod tests {
    use sha3::{Digest, Keccak256};

    /// keccak256 of an empty byte string. Sanity check that our hasher
    /// produces the canonical value the docstring references.
    #[test]
    fn empty_keccak256() {
        let digest: [u8; 32] = Keccak256::new().finalize().into();
        assert_eq!(
            hex::encode(digest),
            "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
        );
    }

    /// Order matters: swapping two rows produces a different commitment.
    #[test]
    fn order_sensitive() {
        let a = [0x11_u8; 32];
        let b = [0x22_u8; 32];
        let h1 = [0xaa_u8; 32];
        let h2 = [0xbb_u8; 32];

        let mut k1 = Keccak256::new();
        k1.update(a);
        k1.update(h1);
        k1.update(b);
        k1.update(h2);

        let mut k2 = Keccak256::new();
        k2.update(b);
        k2.update(h2);
        k2.update(a);
        k2.update(h1);

        let d1: [u8; 32] = k1.finalize().into();
        let d2: [u8; 32] = k2.finalize().into();
        assert_ne!(d1, d2);
    }
}
