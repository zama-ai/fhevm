pub mod bridge;
pub mod chain_id;
pub mod crs;
pub mod database;
pub mod db_keys;
pub mod drift_revert;
pub mod gcs_activation;
#[cfg(feature = "gpu")]
pub mod gpu_memory;
pub mod healthz_server;
pub mod host_chains;
pub mod keys;
pub mod metrics_server;
pub mod pg_pool;
pub mod synthetic_input;
pub mod telemetry;
pub mod tfhe_ops;
pub mod types;
pub mod utils;
pub mod versioning;
pub mod zk_aux;

pub mod common {
    tonic::include_proto!("fhevm.common");
}

/// Release version shared by all services. Change it every release.
/// Nothing compares it, so it never decides blue/green mode.
pub const STACK_VERSION: &str = "0.15.0";

pub const CIPHERTEXT_VERSION: i16 = 0;

pub const HANDLE_VERSION: i16 = 0;

// Decides blue/green mode. Raise it by one when a release changes the results
// operators must agree on:
//   - new key parameters
//   - the GPU feature is turned on
//   - randomization changes
//   - the scheduling logic changes
// Leave it as is for every other release, which then rolls out without a cutover.
// A macro because the schema name needs it in `concat!`.
macro_rules! consensus_protocol_version {
    () => {
        1
    };
}
pub(crate) use consensus_protocol_version;

pub const CONSENSUS_PROTOCOL_VERSION: u32 = consensus_protocol_version!();

/// Print [`STACK_VERSION`] and exit when `--stack-version` is present.
///
/// Call this before clap parsing so the flag works without other required
/// arguments.
pub fn handle_stack_version_flag() {
    if std::env::args().any(|arg| arg == "--stack-version") {
        println!("{STACK_VERSION}");
        std::process::exit(0);
    }
}
