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

/// Version string of the coprocessor stack this binary belongs to.
///
/// Derived from this crate's (`fhevm-engine-common`) Cargo package version, so
/// every service that links it shares one fleet-wide stack version. Bumped by
/// bumping `fhevm-engine-common`'s version on each release; written into the
/// `versioning` singleton at cutover and surfaced in upgrade notifications.
///
/// Note: `CARGO_PKG_VERSION` resolves at *this* crate's compile time, i.e. to
/// `fhevm-engine-common`'s version — not the calling binary's. That is
/// intentional: it gives all services a single shared value to compare against
/// `versioning.stack_version`. The leading-`v` prefix is optional; the version
/// parser in `versioning::parse_version` tolerates its absence.
pub const STACK_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CIPHERTEXT_VERSION: i16 = 2;
