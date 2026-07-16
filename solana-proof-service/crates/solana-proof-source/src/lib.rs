//! Confirmed Yellowstone completed-block source for the Solana proof service.
//!
//! This crate owns provider transport and protobuf normalization only. It does
//! not own persistence, applied cursor, reconnect policy, recovery, or
//! readiness — those belong to later service slices.

mod raw_instruction;
mod yellowstone_source;

pub use raw_instruction::RawInstruction;
pub use yellowstone_source::{
    BlockCheckpoint, CanonicalTransaction, CompletedBlock, YellowstoneBlockSource,
    YellowstoneSourceConfig, YellowstoneSourceError, YellowstoneSubscription,
};
