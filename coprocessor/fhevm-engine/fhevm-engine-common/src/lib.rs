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
/// Hard-coded per binary and bumped on each release; written into the
/// `versioning` singleton at cutover and surfaced in upgrade notifications.
pub const STACK_VERSION: &str = "v0.14.0";

pub const CIPHERTEXT_VERSION: i16 = 1;
