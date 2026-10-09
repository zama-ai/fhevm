pub mod bootstrap_versioning;
pub mod bridge;
pub mod chain_id;
pub mod crs;
pub mod database;
pub mod db_keys;
pub mod drift_containment;
pub mod drift_revert;
pub mod gcs_activation;
pub mod gpu_arch;
#[cfg(feature = "gpu")]
pub mod gpu_memory;
pub mod healthz_server;
pub mod host_chains;
pub mod keys;
pub mod metrics_server;
pub mod pg_pool;
#[cfg(feature = "test-failpoints")]
pub mod reservation_test_control;
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

/// Single source of truth for the coprocessor stack version.
///
/// Exposed as a macro (not a `const`) so it embeds inside `concat!` — e.g. the
/// versioned GCS schema name in `database.rs` — while staying single-sourced.
#[cfg(not(feature = "stack-version-override"))]
macro_rules! stack_version {
    () => {
        "0.15.0"
    };
}

/// The release comes from `BUILD_STACK_VERSION`. See the feature pair in
/// `Cargo.toml` for who builds this way and why.
#[cfg(feature = "stack-version-override")]
macro_rules! stack_version {
    () => {
        env!(
            "BUILD_STACK_VERSION",
            "stack-version-override needs BUILD_STACK_VERSION set"
        )
    };
}
pub(crate) use stack_version;

/// Version string of the coprocessor stack this binary belongs to. Shared by
/// every service that links this crate, compared against the release a proposal
/// names, written into the singleton at cutover, and surfaced in upgrade
/// notifications. The leading-`v` prefix is optional; the parser in
/// `versioning::parse_version` tolerates its absence.
///
/// Change it every release. It never decides blue/green mode.
pub const STACK_VERSION: &str = stack_version!();

pub use versioning::{format_consensus_epoch, versions_equal};

pub const CIPHERTEXT_VERSION: i16 = 0;

pub const HANDLE_VERSION: i16 = 0;

// Decides blue/green mode. Raise it by one when a release changes the results
// operators must agree on:
//   - new key parameters
//   - the GPU feature is turned on
//   - randomization changes
//   - the scheduling logic changes
// Leave it as is for every other release, which then rolls out without a cutover.
//
// Before raising it, the start-position ladder in the consumer SDK
// (`ListenerConsumer::seed_group`) needs a rung for a versioned predecessor. It
// recognizes an unsuffixed one only, so a stack coming up at vN+1 does not match
// the live vN group, starts at the tip of the stream, and never sees what was
// published before its own group existed. The host-listener poller backfills that
// under the default topology; nothing does when the host side runs on the listener
// stack alone. The gap is silent — the cutover itself succeeds.
#[cfg(not(feature = "consensus-version-override"))]
pub const CONSENSUS_PROTOCOL_VERSION: u32 = 2;

/// The value comes from `BUILD_CONSENSUS_VERSION`. See the feature pair in
/// `Cargo.toml` for who builds this way and why.
#[cfg(feature = "consensus-version-override")]
pub const CONSENSUS_PROTOCOL_VERSION: u32 = match u32::from_str_radix(
    env!(
        "BUILD_CONSENSUS_VERSION",
        "consensus-version-override needs BUILD_CONSENSUS_VERSION set"
    ),
    10,
) {
    Ok(value) => value,
    Err(_) => panic!("BUILD_CONSENSUS_VERSION must be a whole number"),
};

/// If `--stack-version` appears in the process arguments, prints the
/// compiled-in coprocessor [`STACK_VERSION`] to stdout and exits with status 0.
///
/// Call this *before* clap parsing. It scans argv directly rather than reading
/// a parsed flag so it short-circuits like clap's built-in `--version`: it
/// prints and exits even when a service's other required flags are absent
/// (e.g. `consensus-detector --stack-version` with no `--gw-url`). Each service
/// still declares a `--stack-version` clap field so the flag is documented in
/// `--help`.
///
/// `--version` reports the per-crate `CARGO_PKG_VERSION` (which diverges across
/// the workspace); `--stack-version` reports the single fleet-wide value.
pub fn handle_stack_version_flag() {
    if std::env::args().any(|arg| arg == "--stack-version") {
        println!("{STACK_VERSION}");
        std::process::exit(0);
    }
}
