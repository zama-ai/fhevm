//! Startup validation before workers and HTTP serving.
//!
//! Fail fast on unreachable database / missing service schema rather than
//! binding an unready listener. Parsed config fields are already validated by
//! [`crate::config::ServiceConfig::validate`] at load time.

use anyhow::{Context, Result};
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

    Ok(())
}
