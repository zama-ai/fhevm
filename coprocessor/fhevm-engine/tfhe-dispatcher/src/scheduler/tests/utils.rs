use crate::scheduler::{messages, BlockContext, Handle};
use fhevm_engine_common::types::SupportedFheOperations;
use std::time::SystemTime;

#[derive(Clone)]
pub struct TestContext {
    block_info: BlockContext,
}

impl TestContext {
    pub fn new(block_number: u64, txn_hash: [u8; 32], block_hash: [u8; 32]) -> Self {
        Self {
            block_info: BlockContext {
                txn_hash,
                block_number,
                block_hash,
            },
        }
    }

    pub fn event_log(
        &self,
        output_handle: [u8; 32],
        dependencies: Vec<[u8; 32]>,
        op: SupportedFheOperations,
        is_allowed: bool,
    ) -> messages::FheLog {
        messages::FheLog {
            output_handle,
            fhe_operation: op,
            is_allowed,
            is_scalar: false,
            created_at: SystemTime::now(),
            dependencies,
            block_info: BlockContext {
                txn_hash: self.block_info.txn_hash,
                block_number: self.block_info.block_number,
                block_hash: self.block_info.block_hash,
            },
        }
    }
}
