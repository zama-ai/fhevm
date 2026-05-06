//! `coproc-mngr` - upgrade orchestrator service for the FHEVM coprocessor stack.
//!
//! See `tech-spec/rfcs/021-blue-green-upgrade.md` § `coproc-mngr`.
//!
//! ## What it does
//!
//! - LISTENs on the Postgres `event_upgrade` channel.
//! - Drains rows from `upgrade_events` (written by the future gw-listener
//!   routing path; manually inserted in this iteration for testing).
//! - Drives the per-stack FSM in `service_state`.
//! - Emits `pg_notify` on per-stage channels to coordinate the other services.
//!   In this first iteration **nothing else listens on these channels** - see
//!   the RFC for the consumer-side rollout plan.
//!
//! ## What it does NOT do (yet)
//!
//! - Run `pg_dump` of the BCS DB into the GCS DB (logged as a placeholder).
//! - Submit `signalReady` on-chain (just inserts a row into
//!   `signal_ready_pending`; tx-sender will pick it up in a later iteration).
//! - Compute a real `stateCommitment` (a placeholder zero-hash is written).
//! - Talk to BCS DB at all (the `bcs_database_url` CLI flag is accepted but
//!   only logged; no cross-DB connection is opened).

pub mod commitment;
pub mod config;
pub mod handlers;
pub mod metrics;
pub mod notify;
pub mod readiness;
pub mod service;
pub mod state;

pub use config::ConfigSettings;
pub use service::run;
