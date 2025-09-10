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
const MINIMUM_BLOCK_TIME_SECONDS: u64 = 1;

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

    pub fn estimated_block_time(&self) -> Option<u64> {
        if self.ordered_blocks.len() < 2 {
            return None;
        };
        let last = self.ordered_blocks.back()?;
        let second_last =
            self.ordered_blocks.get(self.ordered_blocks.len() - 2)?;
        if last.timestamp <= second_last.timestamp {
            return None;
        }
        if last.number <= second_last.number {
            return None;
        }
        let estimation = (last.timestamp - second_last.timestamp) as f64
            / (last.number - second_last.number) as f64;
        Some((estimation.round() as u64).max(MINIMUM_BLOCK_TIME_SECONDS))
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockHash, BlockHistory, BlockSummary};

    #[test]
    fn test_block_history() {
        let mut history = BlockHistory::new(10);
        let block1 = BlockSummary {
            number: 1,
            hash: BlockHash::with_last_byte(1),
            parent_hash: BlockHash::with_last_byte(0),
            timestamp: 0,
        };
        let block2 = BlockSummary {
            number: 2,
            hash: BlockHash::with_last_byte(2),
            parent_hash: BlockHash::with_last_byte(1),
            timestamp: 12,
        };
        let block3 = BlockSummary {
            number: 3,
            hash: BlockHash::with_last_byte(3),
            parent_hash: BlockHash::with_last_byte(2),
            timestamp: 24,
        };
        history.add_block(block1);
        history.add_block(block2);
        assert_eq!(history.size(), 2);
        assert!(history.is_ready_to_detect_reorg());
        assert!(history.is_known(&block1.hash));
        assert!(history.is_known(&block2.hash));
        assert!(history.block_has_not_changed(&block2.hash));
        assert!(!history.block_has_not_changed(&block3.hash));
        assert!(!history.is_known(&block3.hash));
        history.add_block(block3);
        assert_eq!(history.tip().map(|b| b.number), Some(block3.number));
        assert!(history.block_has_not_changed(&block3.hash));
        assert!(history.is_known(&block3.hash));
    }

    #[test]
    fn test_estimated_block_time() {
        let mut history = BlockHistory::new(10);
        let block1 = BlockSummary {
            number: 1,
            hash: BlockHash::with_last_byte(1),
            parent_hash: BlockHash::with_last_byte(0),
            timestamp: 0,
        };
        let block2 = BlockSummary {
            number: 2,
            hash: BlockHash::with_last_byte(2),
            parent_hash: BlockHash::with_last_byte(1),
            timestamp: 12,
        };
        let block3 = BlockSummary {
            number: 5,
            hash: BlockHash::with_last_byte(5),
            parent_hash: BlockHash::with_last_byte(4),
            timestamp: 14,
        };
        let block4 = BlockSummary {
            number: 15,
            hash: BlockHash::with_last_byte(5),
            parent_hash: BlockHash::with_last_byte(4),
            timestamp: 14 + 10 * 12 + 4,
        };
        let block5 = BlockSummary {
            number: 15,
            hash: BlockHash::with_last_byte(5),
            parent_hash: BlockHash::with_last_byte(4),
            timestamp: 14 + 10 * 12 + 6,
        };
        history.add_block(block1);
        history.add_block(block2);
        assert_eq!(history.estimated_block_time(), Some(12));
        history.add_block(block2);
        history.add_block(block1);
        assert_eq!(history.estimated_block_time(), None);
        history.add_block(block2);
        history.add_block(block3);
        assert_eq!(history.estimated_block_time(), Some(1));
        history.add_block(block4);
        assert_eq!(history.estimated_block_time(), Some(12));
        history.add_block(block3);
        history.add_block(block5);
        assert_eq!(history.estimated_block_time(), Some(13));
    }
}
