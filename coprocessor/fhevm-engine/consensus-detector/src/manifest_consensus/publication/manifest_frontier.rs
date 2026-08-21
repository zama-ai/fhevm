use std::collections::BTreeMap;

use alloy_primitives::U256;
use block_manifest::{
    canonical_history_scale as wire_history_scale, dyadic_range_digest, ManifestPayload,
    ManifestVersion,
};

use crate::manifest_consensus::ExecutionError;

use crate::manifest_consensus::lineage::{RangeFrontier, RangeNode};

pub(super) fn rebuild_frontier_from_manifest(
    previous: &ManifestPayload,
    historical_ranges: Vec<RangeNode>,
) -> Result<(RangeFrontier, Vec<RangeNode>), ExecutionError> {
    if historical_ranges.len() != previous.historical_ranges.len() {
        return Err(internal(
            "hydrated BlockRange count does not match the previous manifest",
        ));
    }
    for (range, historical) in historical_ranges
        .iter()
        .zip(previous.historical_ranges.iter().rev())
    {
        if range.start != i64_from_u256("historical range start", historical.start_block_number)?
            || range.end != i64_from_u256("historical range end", historical.end_block_number)?
            || range.scale
                != i32::try_from(historical.scale)
                    .map_err(|_| internal("historical range scale exceeds INTEGER"))?
            || range.end_block_hash != historical.end_block_hash
            || u64::try_from(range.end_block_generation).ok()
                != Some(historical.end_block_generation)
            || range.digest != historical.digest
        {
            return Err(internal(
                "hydrated BlockRange does not match the previous manifest",
            ));
        }
    }

    let host_chain_id = i64_from_u256("host chain id", previous.host_chain_id)?;
    let mut frontier = RangeFrontier::from_ranges(historical_ranges);
    validate_canonical_frontier(frontier.as_slice())?;
    let mut reconstructed_ranges = Vec::new();
    for block in &previous.detailed_range.blocks {
        let number = i64_from_u256("block number", block.block_number)?;
        let leaf = RangeNode {
            end_block_generation: i64::try_from(block.generation)
                .map_err(|_| internal("block generation exceeds BIGINT"))?,
            start: number,
            end: number,
            scale: 0,
            start_block_hash: block.block_hash,
            start_parent_block_hash: block.parent_block_hash,
            end_block_hash: block.block_hash,
            digest: block.block_content_digest,
        };
        reconstructed_ranges.push(leaf.clone());
        reconstructed_ranges.extend(append_leaf(
            host_chain_id,
            previous.coprocessor_context_id,
            &mut frontier,
            leaf,
        )?);
    }
    validate_canonical_frontier(frontier.as_slice())?;
    Ok((frontier, reconstructed_ranges))
}

pub(super) fn append_leaf(
    host_chain_id: i64,
    coprocessor_context_id: U256,
    frontier: &mut RangeFrontier,
    leaf: RangeNode,
) -> Result<Vec<RangeNode>, ExecutionError> {
    validate_canonical_frontier(frontier.as_slice())?;
    if leaf.scale != 0 || leaf.start != leaf.end {
        return Err(internal("only a size-one BlockRange can advance history"));
    }
    if let Some(newest) = frontier.as_slice().last() {
        if newest.end.checked_add(1) != Some(leaf.start)
            || newest.end_block_hash != leaf.start_parent_block_hash
        {
            return Err(internal(
                "new BlockRange is not contiguous with the canonical history",
            ));
        }
    }
    let generation_start = frontier
        .as_slice()
        .first()
        .map_or(leaf.start, |oldest| oldest.start);
    if frontier.as_slice().iter().any(|range| {
        range.end_block_generation != leaf.end_block_generation || range.start < generation_start
    }) {
        return Err(internal(
            "range frontier crosses a manifest generation boundary",
        ));
    }

    let mut available = BTreeMap::new();
    for range in frontier.as_slice() {
        insert_available_range(&mut available, range.clone())?;
    }
    insert_available_range(&mut available, leaf.clone())?;

    let mut created_parents = Vec::new();
    let mut newest_to_oldest = Vec::new();
    let mut upper = leaf
        .end
        .checked_add(1)
        .ok_or_else(|| internal("BlockRange upper boundary overflow"))?;
    let mut previous_scale = 0;

    while upper > 0 {
        let scale = canonical_history_scale(upper, previous_scale)?;
        let size = range_size(scale)?;
        let Some(start) = upper.checked_sub(size) else {
            break;
        };
        let MaterializedRange::Available(range) = materialize_range(
            host_chain_id,
            coprocessor_context_id,
            start,
            scale,
            generation_start,
            &mut available,
            &mut created_parents,
        )?
        else {
            break;
        };
        newest_to_oldest.push(range);
        upper = start;
        previous_scale = scale;
    }

    newest_to_oldest.reverse();
    *frontier.as_mut_vec() = newest_to_oldest;
    validate_canonical_frontier(frontier.as_slice())?;
    Ok(created_parents)
}

type RangeKey = (i64, i32);

fn insert_available_range(
    available: &mut BTreeMap<RangeKey, RangeNode>,
    range: RangeNode,
) -> Result<(), ExecutionError> {
    let key = (
        range
            .virtual_start()
            .ok_or_else(|| internal("invalid virtual BlockRange boundary"))?,
        range.scale,
    );
    if let Some(existing) = available.get(&key) {
        if existing != &range {
            return Err(internal(format!(
                "conflicting BlockRange [{}, {}] at scale {}",
                range.start, range.end, range.scale,
            )));
        }
        return Ok(());
    }
    available.insert(key, range);
    Ok(())
}

enum MaterializedRange {
    Empty,
    Available(RangeNode),
    Missing,
}

fn materialize_range(
    host_chain_id: i64,
    coprocessor_context_id: U256,
    start: i64,
    scale: i32,
    generation_start: i64,
    available: &mut BTreeMap<RangeKey, RangeNode>,
    created_parents: &mut Vec<RangeNode>,
) -> Result<MaterializedRange, ExecutionError> {
    if let Some(range) = available.get(&(start, scale)) {
        return Ok(MaterializedRange::Available(range.clone()));
    }
    let size = range_size(scale)?;
    let end = start
        .checked_add(size - 1)
        .ok_or_else(|| internal("dyadic range boundary overflow"))?;
    if end < generation_start {
        return Ok(MaterializedRange::Empty);
    }
    let Some(child_scale) = scale.checked_sub(1) else {
        return Ok(MaterializedRange::Missing);
    };
    if child_scale < 0 {
        return Ok(MaterializedRange::Missing);
    }
    let child_size = range_size(child_scale)?;
    let right_start = start
        .checked_add(child_size)
        .ok_or_else(|| internal("dyadic child boundary overflow"))?;
    let left = materialize_range(
        host_chain_id,
        coprocessor_context_id,
        start,
        child_scale,
        generation_start,
        available,
        created_parents,
    )?;
    let right = materialize_range(
        host_chain_id,
        coprocessor_context_id,
        right_start,
        child_scale,
        generation_start,
        available,
        created_parents,
    )?;
    let (left, right) = match (left, right) {
        (MaterializedRange::Available(left), MaterializedRange::Available(right)) => (left, right),
        (MaterializedRange::Empty, MaterializedRange::Available(right)) => {
            let promoted = RangeNode {
                end_block_generation: right.end_block_generation,
                start: right.start,
                end: right.end,
                scale,
                start_block_hash: right.start_block_hash,
                start_parent_block_hash: right.start_parent_block_hash,
                end_block_hash: right.end_block_hash,
                digest: right.digest,
            };
            insert_available_range(available, promoted.clone())?;
            created_parents.push(promoted.clone());
            return Ok(MaterializedRange::Available(promoted));
        }
        (MaterializedRange::Missing, _) | (_, MaterializedRange::Missing) => {
            return Ok(MaterializedRange::Missing);
        }
        _ => return Ok(MaterializedRange::Missing),
    };
    if !left.is_left_sibling_of(&right) {
        return Err(internal(format!(
            "BlockRanges [{}, {}] and [{}, {}] are not lineage siblings",
            left.start, left.end, right.start, right.end,
        )));
    }

    let parent = RangeNode {
        end_block_generation: right.end_block_generation,
        start: left.start,
        end: right.end,
        scale,
        start_block_hash: left.start_block_hash,
        start_parent_block_hash: left.start_parent_block_hash,
        end_block_hash: right.end_block_hash,
        digest: dyadic_range_digest(
            ManifestVersion::V1,
            coprocessor_context_id,
            non_negative_u256("host chain id", host_chain_id)?,
            non_negative_u256("range start", left.start)?,
            non_negative_u256("range end", right.end)?,
            u32::try_from(scale).map_err(|_| internal("negative range scale"))?,
            right.end_block_hash,
            left.digest,
            right.digest,
        ),
    };
    insert_available_range(available, parent.clone())?;
    created_parents.push(parent.clone());
    Ok(MaterializedRange::Available(parent))
}

fn canonical_history_scale(upper: i64, previous_scale: i32) -> Result<i32, ExecutionError> {
    let upper = u64::try_from(upper).map_err(|_| internal("negative history boundary"))?;
    let previous_scale =
        u32::try_from(previous_scale).map_err(|_| internal("negative dyadic range scale"))?;
    let scale = wire_history_scale(U256::from(upper), previous_scale)
        .map_err(|error| internal(error.to_string()))?;
    i32::try_from(scale).map_err(|_| internal("dyadic range scale exceeds INTEGER"))
}

pub(super) fn range_size(scale: i32) -> Result<i64, ExecutionError> {
    let scale = u32::try_from(scale).map_err(|_| internal("negative dyadic range scale"))?;
    1_i64
        .checked_shl(scale)
        .ok_or_else(|| internal("dyadic range scale exceeds BIGINT"))
}

pub(super) fn derive_range_scale(start: i64, end: i64) -> Result<i32, ExecutionError> {
    let size = end
        .checked_sub(start)
        .and_then(|span| span.checked_add(1))
        .ok_or_else(|| internal("invalid dyadic range bounds"))?;
    let size = u64::try_from(size).map_err(|_| internal("invalid dyadic range bounds"))?;
    if !size.is_power_of_two() {
        return Err(internal(format!("unaligned dyadic range [{start}, {end}]")));
    }
    let size = i64::try_from(size).map_err(|_| internal("dyadic range size exceeds BIGINT"))?;
    if start.rem_euclid(size) != 0 {
        return Err(internal(format!("unaligned dyadic range [{start}, {end}]")));
    }
    i32::try_from(size.ilog2()).map_err(|_| internal("dyadic range scale exceeds INTEGER"))
}

pub(super) fn validate_canonical_frontier(frontier: &[RangeNode]) -> Result<(), ExecutionError> {
    for pair in frontier.windows(2) {
        if pair[0].end.checked_add(1) != Some(pair[1].start)
            || pair[0].end_block_hash != pair[1].start_parent_block_hash
        {
            return Err(internal("range frontier is not one contiguous lineage"));
        }
    }

    let Some(newest) = frontier.last() else {
        return Ok(());
    };
    let mut upper = newest
        .end
        .checked_add(1)
        .ok_or_else(|| internal("BlockRange upper boundary overflow"))?;
    let mut previous_scale = 0;
    for (index, range) in frontier.iter().rev().enumerate() {
        let expected_scale = canonical_history_scale(upper, previous_scale)?;
        let expected_size = range_size(expected_scale)?;
        let expected_virtual_start = upper
            .checked_sub(expected_size)
            .ok_or_else(|| internal("canonical BlockRange starts before block zero"))?;
        let is_oldest = index + 1 == frontier.len();
        if range.scale != expected_scale
            || range.end.checked_add(1) != Some(upper)
            || (!is_oldest && range.start != expected_virtual_start)
            || (is_oldest && (range.start < expected_virtual_start || range.start > range.end))
        {
            return Err(internal(format!(
                "non-canonical BlockRange [{}, {}] at scale {}; expected virtual [{}, {}] at scale {}",
                range.start,
                range.end,
                range.scale,
                expected_virtual_start,
                upper - 1,
                expected_scale,
            )));
        }
        upper = expected_virtual_start;
        previous_scale = range.scale;
    }
    Ok(())
}

fn non_negative_u256(field: &str, value: i64) -> Result<U256, ExecutionError> {
    let value = u64::try_from(value)
        .map_err(|_| internal(format!("{field} must be non-negative, got {value}")))?;
    Ok(U256::from(value))
}

fn i64_from_u256(field: &str, value: U256) -> Result<i64, ExecutionError> {
    i64::try_from(value).map_err(|_| internal(format!("{field} exceeds BIGINT")))
}

fn internal(message: impl Into<String>) -> ExecutionError {
    ExecutionError::InternalError(message.into())
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{Address, B256};
    use block_manifest::{
        block_content_digest, detailed_range_digest, DetailedRange, HistoricalRange,
        ManifestBlockEntry,
    };

    use super::*;

    const HOST_CHAIN_ID: i64 = 7;
    const CONTEXT_ID: U256 = U256::ONE;

    fn hash(number: u8) -> B256 {
        B256::repeat_byte(number)
    }

    fn leaf(number: i64) -> RangeNode {
        RangeNode {
            end_block_generation: 0,
            start: number,
            end: number,
            scale: 0,
            start_block_hash: hash(number as u8),
            start_parent_block_hash: hash(number.wrapping_sub(1) as u8),
            end_block_hash: hash(number as u8),
            digest: B256::repeat_byte(number as u8 + 100),
        }
    }

    fn block(number: i64) -> ManifestBlockEntry {
        let block_hash = hash(number as u8);
        let digest = block_content_digest(
            ManifestVersion::V1,
            CONTEXT_ID,
            U256::from(HOST_CHAIN_ID as u64),
            U256::from(number as u64),
            block_hash,
            &[],
        )
        .unwrap();
        ManifestBlockEntry {
            generation: 0,
            block_number: U256::from(number as u64),
            block_hash,
            parent_block_hash: hash(number.wrapping_sub(1) as u8),
            block_content_digest: digest,
            ciphertexts: Vec::new(),
        }
    }

    #[test]
    fn creates_frontier_from_scratch() {
        let mut frontier = RangeFrontier::default();
        let mut created = Vec::new();
        for number in 0..=2 {
            created.extend(
                append_leaf(HOST_CHAIN_ID, CONTEXT_ID, &mut frontier, leaf(number)).unwrap(),
            );
        }

        assert_eq!(created.len(), 1);
        assert_eq!(
            frontier
                .as_slice()
                .iter()
                .map(|range| (range.start, range.end, range.scale))
                .collect::<Vec<_>>(),
            vec![(0, 1, 1), (2, 2, 0)],
        );
    }

    #[test]
    fn rejects_a_non_canonical_dyadic_cover() {
        let ranges = vec![leaf(0), leaf(1), leaf(2)];
        assert!(validate_canonical_frontier(&ranges)
            .unwrap_err()
            .to_string()
            .contains("non-canonical BlockRange"),);
    }

    #[test]
    fn creates_the_unique_right_anchored_history() {
        let mut frontier = RangeFrontier::default();
        for number in 0..=7 {
            append_leaf(HOST_CHAIN_ID, CONTEXT_ID, &mut frontier, leaf(number)).unwrap();
        }

        assert_eq!(
            frontier
                .as_slice()
                .iter()
                .rev()
                .map(|range| (range.start, range.end, range.scale))
                .collect::<Vec<_>>(),
            vec![(6, 7, 1), (4, 5, 1), (0, 3, 2)],
        );
    }

    #[test]
    fn retains_a_generation_truncated_oldest_virtual_range() {
        let mut frontier = RangeFrontier::default();
        for number in 1..=7 {
            append_leaf(HOST_CHAIN_ID, CONTEXT_ID, &mut frontier, leaf(number)).unwrap();
        }

        assert_eq!(
            frontier
                .as_slice()
                .iter()
                .rev()
                .map(|range| (range.start, range.end, range.scale))
                .collect::<Vec<_>>(),
            vec![(6, 7, 1), (4, 5, 1), (1, 3, 2)],
        );

        for number in 8..=15 {
            append_leaf(HOST_CHAIN_ID, CONTEXT_ID, &mut frontier, leaf(number)).unwrap();
        }
        assert_eq!(
            frontier
                .as_slice()
                .iter()
                .rev()
                .map(|range| (range.start, range.end, range.scale))
                .collect::<Vec<_>>(),
            vec![(14, 15, 1), (12, 13, 1), (8, 11, 2), (1, 7, 3)],
        );
    }

    #[test]
    fn rebuilds_frontier_from_last_local_publication() {
        let blocks = vec![block(2), block(3), block(4)];
        let block_digests = blocks
            .iter()
            .map(|block| block.block_content_digest)
            .collect::<Vec<_>>();
        let historical_digest = B256::repeat_byte(0x90);
        let previous = ManifestPayload {
            version: ManifestVersion::V1,
            generation: 0,
            publisher: Address::ZERO,
            coprocessor_context_id: CONTEXT_ID,
            host_chain_id: U256::from(HOST_CHAIN_ID as u64),
            publication_block_number: U256::from(4),
            publication_block_hash: hash(4),
            publication_parent_block_hash: hash(3),
            revision: 0,
            detailed_range: DetailedRange {
                first_block_number: U256::from(2),
                last_block_number: U256::from(4),
                digest: detailed_range_digest(
                    ManifestVersion::V1,
                    CONTEXT_ID,
                    U256::from(HOST_CHAIN_ID as u64),
                    U256::from(2),
                    U256::from(4),
                    &block_digests,
                ),
                blocks,
            },
            historical_ranges: vec![HistoricalRange {
                end_block_generation: 0,
                start_block_number: U256::ZERO,
                end_block_number: U256::ONE,
                scale: 1,
                end_block_hash: hash(1),
                digest: historical_digest,
            }],
        };
        previous.validate().unwrap();

        let historical = RangeNode {
            end_block_generation: 0,
            start: 0,
            end: 1,
            scale: 1,
            start_block_hash: hash(0),
            start_parent_block_hash: hash(u8::MAX),
            end_block_hash: hash(1),
            digest: historical_digest,
        };
        let (frontier, reconstructed) =
            rebuild_frontier_from_manifest(&previous, vec![historical]).unwrap();

        assert_eq!(reconstructed.len(), 4);
        assert_eq!(
            frontier
                .as_slice()
                .iter()
                .map(|range| (range.start, range.end, range.scale))
                .collect::<Vec<_>>(),
            vec![(0, 1, 1), (2, 3, 1), (4, 4, 0)],
        );
        assert_eq!(frontier.as_slice().last().unwrap().end_block_hash, hash(4));
    }
}
