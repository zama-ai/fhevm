pub mod chain_id;
pub mod crs;
pub mod database;
pub mod db_keys;
pub mod drift_revert;
#[cfg(feature = "gpu")]
pub mod gpu_memory;
pub mod healthz_server;
pub mod host_chains;
pub mod keys;
pub mod metrics_server;
pub mod pg_pool;
pub mod telemetry;
pub mod tfhe_ops;
pub mod types;
pub mod utils;
pub mod versioning;

pub mod common {
    tonic::include_proto!("fhevm.common");
}

/// Single source of truth for the coprocessor stack version.
///
/// Hard-coded on purpose — deliberately NOT derived from any crate's
/// `CARGO_PKG_VERSION`. Per-crate package versions diverge across the workspace
/// (the workers are versioned independently of `fhevm-engine-common`), so
/// tying the stack version to one crate's package version was misleading: a
/// worker at package `0.7.0` would report stack version `0.14.0`. This makes
/// the stack version one explicit, fleet-wide value, bumped by a deliberate
/// edit here on each blue/green stack upgrade.
///
/// Exposed as a macro (not just a `const`) so it can be embedded inside
/// `concat!` — e.g. the versioned GCS schema name in `database.rs` — while
/// staying single-sourced.
macro_rules! stack_version {
    () => {
        "0.14.0"
    };
}
pub(crate) use stack_version;

/// Version string of the coprocessor stack this binary belongs to. Shared by
/// every service that links this crate, compared against
/// `versioning.stack_version`, written into the singleton at cutover, and
/// surfaced in upgrade notifications. The leading-`v` prefix is optional; the
/// parser in `versioning::parse_version` tolerates its absence.
pub const STACK_VERSION: &str = stack_version!();

pub const CIPHERTEXT_VERSION: i16 = 2;
