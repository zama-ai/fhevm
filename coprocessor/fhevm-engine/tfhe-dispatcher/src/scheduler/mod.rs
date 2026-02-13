pub mod computation_scheduler;
pub mod messages;
pub mod traits;
pub mod types;
pub mod utils;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

pub type Handle = [u8; 32];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockContext {
    pub txn_hash: [u8; 32],
    pub block_number: u64,
    pub block_hash: [u8; 32],
}
