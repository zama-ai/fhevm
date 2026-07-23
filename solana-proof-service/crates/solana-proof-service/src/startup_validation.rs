//! Startup validation before workers and HTTP serving.
//!
//! Fail fast on unreachable database / missing service schema rather than
//! binding an unready listener. Parsed config fields are already validated by
//! [`crate::config::ServiceConfig::validate`] at load time.

use anyhow::{bail, Context, Result};
use solana_proof_store::SqlProofStore;

pub async fn validate_startup_dependencies(store: &SqlProofStore) -> Result<()> {
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(store.pool())
        .await
        .context("database startup validation failed")?;

    // Ensure the service-owned schema is present (migrate is called earlier;
    // this catches a wrong connection string against an empty cluster).
    store
        .integrity_status()
        .await
        .context("solana_proof_progress startup probe failed")?;

    // Fail closed on a store populated before the #1721 semantic-columns migration: its
    // NULL-semantic leaf rows count in leaf_count but resolve to no semantic key, so they would
    // serve terminal 404s for on-chain leaves. Such a store must be rebuilt from genesis.
    if store
        .has_pre_semantic_leaf_rows()
        .await
        .context("semantic leaf-column startup probe failed")?
    {
        bail!(
            "solana_proof_leaves contains pre-semantic (NULL leaf_kind/handle) rows written before \
             the #1721 migration; these count in leaf_count but resolve to no semantic key and \
             would serve terminal 404s for on-chain leaves. Rebuild the store from genesis \
             (drop + re-ingest from slot 0); there is no backfill from leaf hashes."
        );
    }

    Ok(())
}
