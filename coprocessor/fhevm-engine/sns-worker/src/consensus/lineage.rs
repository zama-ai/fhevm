use alloy_primitives::B256;

/// Compact in-memory cover of one retained block lineage.
///
/// The durable restart point is the preceding signed manifest. This structure
/// is reconstructed from that manifest and the immutable `block_consensus_range`
/// rows, then advanced while preparing the next manifest.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct RangeFrontier {
    ranges: Vec<RangeNode>,
}

impl RangeFrontier {
    pub(crate) fn from_ranges(ranges: Vec<RangeNode>) -> Self {
        Self { ranges }
    }

    pub(crate) fn as_slice(&self) -> &[RangeNode] {
        &self.ranges
    }

    pub(crate) fn as_mut_vec(&mut self) -> &mut Vec<RangeNode> {
        &mut self.ranges
    }
}

/// One immutable aligned range root on a specific host-chain lineage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RangeNode {
    pub start: i64,
    pub end: i64,
    pub scale: i32,
    pub start_block_hash: B256,
    pub start_parent_block_hash: B256,
    pub end_block_hash: B256,
    pub digest: B256,
}

impl RangeNode {
    pub(crate) fn is_left_sibling_of(&self, right: &Self) -> bool {
        if self.scale != right.scale {
            return false;
        }

        let Some(size) = 1_i64.checked_shl(self.scale as u32) else {
            return false;
        };
        let Some(parent_size) = size.checked_mul(2) else {
            return false;
        };

        self.end.checked_add(1) == Some(right.start)
            && self.start.rem_euclid(parent_size) == 0
            && right.start == self.start + size
            && self.end_block_hash == right.start_parent_block_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(start: i64, scale: i32, hash: u8, parent: u8) -> RangeNode {
        let size = 1_i64 << scale;
        RangeNode {
            start,
            end: start + size - 1,
            scale,
            start_block_hash: B256::repeat_byte(hash),
            start_parent_block_hash: B256::repeat_byte(parent),
            end_block_hash: B256::repeat_byte(hash.wrapping_add(size as u8 - 1)),
            digest: B256::repeat_byte(hash.wrapping_add(100)),
        }
    }

    #[test]
    fn only_adjacent_aligned_lineage_siblings_merge() {
        let left = node(4, 1, 4, 3);
        let right = node(6, 1, 6, 5);
        assert!(left.is_left_sibling_of(&right));

        let mut wrong_lineage = right.clone();
        wrong_lineage.start_parent_block_hash = B256::repeat_byte(0xff);
        assert!(!left.is_left_sibling_of(&wrong_lineage));
        assert!(!node(2, 1, 2, 1).is_left_sibling_of(&left));
    }
}
