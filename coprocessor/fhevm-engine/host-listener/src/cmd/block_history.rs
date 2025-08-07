use std::collections::VecDeque;

use alloy::primitives::FixedBytes;
use alloy::rpc::types::Block;

pub type BlockHash = FixedBytes<32>;

#[derive(Clone, Copy, Debug)]
pub struct BlockSummary {
    pub number: u64, // for display only since it can change in reorg
    pub hash: BlockHash,
    pub parent_hash: BlockHash,
    pub timestamp: u64,
}

impl From<Block> for BlockSummary {
    fn from(block: Block) -> Self {
        Self {
            number: block.header.number,
            hash: block.header.hash,
            parent_hash: block.header.parent_hash,
            timestamp: block.header.timestamp,
        }
    }
}

pub struct BlockHistory {
    ordered_blocks: VecDeque<BlockSummary>,
}

const MAXIMUM_NUMBER_OF_COMPETING_CHAIN: usize = 5;
const MINIMUM_HISTORY_SIZE: usize = 2; // current block + at least old block

impl BlockHistory {
    pub fn new(expected_reorg_duration: usize) -> Self {
        // we take extra margin for history
        let capacity =
            expected_reorg_duration * 2 * MAXIMUM_NUMBER_OF_COMPETING_CHAIN;
        Self {
            ordered_blocks: VecDeque::with_capacity(capacity),
        }
    }

    pub fn size(&self) -> usize {
        self.ordered_blocks.len()
    }

    pub fn is_ready_to_detect_reorg(&self) -> bool {
        // it needs to have some data before using it to detect reorg
        // e.g. at start, an unknown ancestor in history is considered a reorg block
        self.ordered_blocks.len() >= MINIMUM_HISTORY_SIZE
    }

    pub fn is_known(&self, block_hash: &BlockHash) -> bool {
        // we process the history in reverse to have O(1) on no reorg
        let slices = self.ordered_blocks.as_slices();
        for history_slice in [slices.1, slices.0].iter() {
            for historic_block in history_slice.iter().rev() {
                if historic_block.hash == *block_hash {
                    return true;
                }
            }
        }
        false
    }

    pub fn block_has_not_changed(&self, block_hash: &BlockHash) -> bool {
        self.ordered_blocks.back().map(|b| &b.hash) == Some(block_hash)
    }

    pub fn tip(&self) -> Option<BlockSummary> {
        self.ordered_blocks.back().copied()
    }

    pub fn add_block(&mut self, block: BlockSummary) {
        if self.ordered_blocks.len() == self.ordered_blocks.capacity() {
            self.ordered_blocks.pop_front();
        }
        self.ordered_blocks.push_back(block);
    }
}
