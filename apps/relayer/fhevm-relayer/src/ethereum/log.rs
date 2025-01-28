use alloy::primitives::{Address, Bytes};

#[derive(Debug, Clone)]
pub struct EthereumLog {
    pub address: Address,
    pub topics: Vec<[u8; 32]>,
    pub data: Bytes,
    pub block_number: u64,
    pub transaction_hash: [u8; 32],
    pub log_index: u64,
}

impl EthereumLog {
    pub fn new(
        address: Address,
        topics: Vec<[u8; 32]>,
        data: Bytes,
        block_number: u64,
        transaction_hash: [u8; 32],
        log_index: u64,
    ) -> Self {
        Self {
            address,
            topics,
            data,
            block_number,
            transaction_hash,
            log_index,
        }
    }
}
