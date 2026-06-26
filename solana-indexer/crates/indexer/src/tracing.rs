//! Tracing init (json + env-filter), adapted from relayer/src/tracing.rs.

use crate::config::settings::LogConfig;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

/// Installs a global subscriber. Defaults to WARN for deps and INFO for `indexer`;
/// override with `RUST_LOG` (e.g. `RUST_LOG=warn,indexer=debug`).
pub fn init_tracing(log_config: &LogConfig) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn,indexer=info"));

    let registry = tracing_subscriber::registry().with(env_filter);

    match log_config.format.as_str() {
        "json" => registry
            .with(tracing_subscriber::fmt::layer().json())
            .init(),
        "pretty" => registry
            .with(tracing_subscriber::fmt::layer().pretty())
            .init(),
        _ => registry
            .with(tracing_subscriber::fmt::layer().compact())
            .init(),
    }
}
