pub mod branch;
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
/// The fleet-wide stack version. Baseline is the hard-coded value below; only
/// with the `stack-version-override` feature does it come from the build-time
/// `BUILD_STACK_VERSION` env (defaulted by build.rs). The blue-green GCS image
/// builds with the feature + a newer version; release builds omit the feature
/// and cannot be overridden. Deliberately NOT a crate `CARGO_PKG_VERSION`
/// (those diverge per-worker across the workspace).
///
/// Exposed as a macro (not a `const`) so it embeds inside `concat!` — e.g. the
/// versioned GCS schema name in `database.rs` — while staying single-sourced.
/// `env!` keeps the override a compile-time literal.
#[cfg(feature = "stack-version-override")]
macro_rules! stack_version {
    () => {
        env!("BUILD_STACK_VERSION")
    };
}
#[cfg(not(feature = "stack-version-override"))]
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

pub const CIPHERTEXT_VERSION: i16 = 0;

pub const HANDLE_VERSION: i16 = 0;

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
/// the workspace); `--stack-version` reports the single fleet-wide value used
/// for blue/green cutover decisions.
pub fn handle_stack_version_flag() {
    if std::env::args().any(|arg| arg == "--stack-version") {
        println!("{STACK_VERSION}");
        std::process::exit(0);
    }
}
