// Re-export public modules
pub mod config;
pub mod core;
pub mod gateway;
pub mod host;
pub mod http;
pub mod logging;
pub mod metrics;
pub mod orchestrator;
pub mod readiness;
pub mod startup;
pub mod startup_recovery;
pub mod store;
#[cfg(feature = "integration-tests")]
pub mod test_support;
pub mod tracing;

// Re-export commonly used types
pub use core::errors::Error;
pub use startup::run_fhevm_relayer;
