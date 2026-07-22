//! Startup validation before workers and HTTP serving.
//!
//! Borrowed carefully from zama-ai/relayer: fail fast on unreachable database /
//! invalid program id rather than binding an unready listener.

use anyhow::{Context, Result};
use solana_proof_store::SqlProofStore;

use crate::config::ServiceConfig;

pub async fn validate_startup_dependencies(
    config: &ServiceConfig,
    store: &SqlProofStore,
) -> Result<()> {
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(store.pool())
        .await
        .context("database startup validation failed")?;

    // Ensure the service-owned schema is present (migrate is called earlier;
    // this catches a mispointed connection string against an empty cluster).
    store
        .integrity_status()
        .await
        .context("solana_proof_progress startup probe failed")?;

    let _ = config
        .program_id_bytes()
        .context("program_id startup validation failed")?;

    if config.solana.rpc_url.is_empty() {
        anyhow::bail!("solana.rpc_url is empty");
    }
    if config.yellowstone.grpc_url.is_empty() {
        anyhow::bail!("yellowstone.grpc_url is empty");
    }

    Ok(())
}
