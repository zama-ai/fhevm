//! Confirmed Yellowstone completed-block source and bounded RPC recovery for
//! the Solana proof service.
//!
//! Yellowstone is the live filtered confirmed source. [`RpcRecoveryClient`] fills
//! parent-chain gaps into the same [`CompletedBlock`] boundary. Persistence,
//! reconnect policy, and readiness remain outside this crate.

mod raw_instruction;
mod rpc_recovery;
mod yellowstone_source;

pub use raw_instruction::RawInstruction;
pub use rpc_recovery::{
    history_complete_justified, normalize_rpc_block_json, RecoveryBounds, RecoveryError,
    RpcRecoveryClient, RpcRecoveryConfig,
};
pub use yellowstone_source::{
    BlockCheckpoint, CanonicalTransaction, CompletedBlock, YellowstoneBlockSource,
    YellowstoneSourceConfig, YellowstoneSourceError, YellowstoneSubscription,
};
