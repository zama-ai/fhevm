//! Follows the zama-host program on a Solana cluster and decodes what it executed. The
//! coprocessor's Solana host listener and the Merkle indexer each run one, with their own sink.

mod follower;
pub mod host;
mod source;

pub use follower::{
    block_checkpoint, run, track_confirmed_slot, BlockCheckpoint, BlockSink, FollowerConfig,
    IngestFailure, PreparedBlock, PreparedTransaction, StartPosition,
};
pub use source::SealedBlock;
