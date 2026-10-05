//! Follows the zama-host program on a Solana cluster and decodes what it executed. The
//! coprocessor's Solana host listener and the Merkle indexer each run one, with their own sink.

use std::time::Duration;

mod args;
mod follower;
pub mod host;
mod source;

pub use args::{Follower, FollowerArgs};
pub use follower::{
    block_checkpoint, run, track_confirmed_slot, BlockCheckpoint, BlockSink,
    FollowerConfig, IngestFailure, PreparedBlock, PreparedTransaction,
    StartPosition,
};
pub use source::SealedBlock;

/// Timeout of one JSON-RPC request to the live or the archive endpoint.
pub const SOLANA_RPC_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
