//! RFC-029 compressed-key cutover: material selection policy.
//!
//! During the one-time live migration to compressed XOF key material,
//! every coprocessor must select the same material for the same
//! operation from consensus-visible inputs only. This module is the
//! single home of that selection logic; workers call it and must not
//! branch on cutover state themselves.
//!
//! With no cutover scheduled (`None` everywhere below), selection is
//! disabled and key loading behaves exactly as before this feature —
//! including GPU-CI deployments that start with compressed material
//! natively.

use std::collections::BTreeMap;

/// Which byte representation of the (unchanged) key an operation uses.
///
/// Deliberately named, not versioned: this is a one-time cutover, and
/// kms-core already uses "MaterialVersions" for serialization-format
/// versioning.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum KeyMaterialKind {
    /// Pre-migration bytes (`keys.sks_key`, and `keys.sns_pk` for SnS).
    Legacy = 0,
    /// Post-migration bytes (`keys.compressed_xof_keyset`).
    CompressedXof = 1,
}

impl KeyMaterialKind {
    pub fn as_i16(self) -> i16 {
        self as i16
    }

    pub fn from_i16(value: i16) -> anyhow::Result<Self> {
        match value {
            0 => Ok(Self::Legacy),
            1 => Ok(Self::CompressedXof),
            other => anyhow::bail!("invalid key_material_kind {other}"),
        }
    }
}

/// The scheduled cutover boundaries, ingested from the finalized
/// on-chain `CompressedKeyCutoverScheduled` record and immutable
/// afterwards.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompressedKeyCutover {
    /// Per-host-chain cutover block (`hostCutoverBlock[C]`).
    pub host_cutover_blocks: BTreeMap<u64, u64>,
    /// Gateway cutover block (`gatewayCutoverBlock`).
    pub gateway_cutover_block: u64,
}

impl CompressedKeyCutover {
    /// Material for host-chain compute anchored at `block_number` on
    /// `chain_id`. Boundary-inclusive on the new side:
    /// `block_number >= hostCutoverBlock[C]` selects `CompressedXof`.
    ///
    /// A chain absent from the schedule never cuts over (stays
    /// `Legacy`). This is deterministic across coprocessors because
    /// the schedule itself is consensus-visible; it is logged loudly
    /// at ingestion time as a probable governance omission.
    pub fn kind_for_host_block(&self, chain_id: u64, block_number: u64) -> KeyMaterialKind {
        match self.host_cutover_blocks.get(&chain_id) {
            Some(&cutover) if block_number >= cutover => KeyMaterialKind::CompressedXof,
            _ => KeyMaterialKind::Legacy,
        }
    }

    /// Material for input verification anchored at the finalized
    /// Gateway block of the request event. Boundary-inclusive on the
    /// new side.
    pub fn kind_for_gateway_block(&self, gateway_block_number: u64) -> KeyMaterialKind {
        if gateway_block_number >= self.gateway_cutover_block {
            KeyMaterialKind::CompressedXof
        } else {
            KeyMaterialKind::Legacy
        }
    }
}

/// Selection entry points for workers. `None` cutover means no
/// migration is scheduled: callers must use the default key-loading
/// path (pre-feature behavior), signaled by returning `None`.
pub fn select_host_kind(
    cutover: Option<&CompressedKeyCutover>,
    chain_id: u64,
    block_number: u64,
) -> Option<KeyMaterialKind> {
    cutover.map(|c| c.kind_for_host_block(chain_id, block_number))
}

pub fn select_gateway_kind(
    cutover: Option<&CompressedKeyCutover>,
    gateway_block_number: u64,
) -> Option<KeyMaterialKind> {
    cutover.map(|c| c.kind_for_gateway_block(gateway_block_number))
}

/// The selected material is not loadable locally (yet). Always a
/// retryable availability condition, never a reason to substitute the
/// other material: substitution is byte divergence by construction.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error(
    "key material {kind:?} unavailable for key_id {key_id}; retry until ingested, never substitute"
)]
pub struct KeyMaterialUnavailable {
    pub key_id: String,
    pub kind: KeyMaterialKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cutover() -> CompressedKeyCutover {
        CompressedKeyCutover {
            host_cutover_blocks: BTreeMap::from([(1, 100), (2, 200)]),
            gateway_cutover_block: 50,
        }
    }

    #[test]
    fn host_boundary_is_inclusive_on_the_new_side() {
        let c = cutover();
        for (block, expected) in [
            (0, KeyMaterialKind::Legacy),
            (99, KeyMaterialKind::Legacy),
            (100, KeyMaterialKind::CompressedXof),
            (101, KeyMaterialKind::CompressedXof),
            (u64::MAX, KeyMaterialKind::CompressedXof),
        ] {
            assert_eq!(c.kind_for_host_block(1, block), expected, "block {block}");
        }
    }

    #[test]
    fn host_chains_are_independent() {
        let c = cutover();
        assert_eq!(
            c.kind_for_host_block(1, 150),
            KeyMaterialKind::CompressedXof
        );
        assert_eq!(c.kind_for_host_block(2, 150), KeyMaterialKind::Legacy);
    }

    #[test]
    fn unscheduled_chain_never_cuts_over() {
        let c = cutover();
        assert_eq!(
            c.kind_for_host_block(999, u64::MAX),
            KeyMaterialKind::Legacy
        );
    }

    #[test]
    fn gateway_boundary_is_inclusive_on_the_new_side() {
        let c = cutover();
        for (block, expected) in [
            (0, KeyMaterialKind::Legacy),
            (49, KeyMaterialKind::Legacy),
            (50, KeyMaterialKind::CompressedXof),
            (u64::MAX, KeyMaterialKind::CompressedXof),
        ] {
            assert_eq!(c.kind_for_gateway_block(block), expected, "block {block}");
        }
    }

    #[test]
    fn no_cutover_selects_nothing() {
        assert_eq!(select_host_kind(None, 1, u64::MAX), None);
        assert_eq!(select_gateway_kind(None, u64::MAX), None);
    }

    #[test]
    fn selection_is_a_pure_function_of_block_data() {
        // Replay determinism: re-evaluating any (chain, block) pair
        // yields the same kind regardless of call order or repetition.
        let c = cutover();
        for chain in [1u64, 2, 999] {
            for block in [0u64, 99, 100, 199, 200, 201] {
                let first = c.kind_for_host_block(chain, block);
                for _ in 0..3 {
                    assert_eq!(c.kind_for_host_block(chain, block), first);
                }
            }
        }
    }

    #[test]
    fn kind_roundtrips_through_i16() {
        for kind in [KeyMaterialKind::Legacy, KeyMaterialKind::CompressedXof] {
            assert_eq!(KeyMaterialKind::from_i16(kind.as_i16()).unwrap(), kind);
        }
        assert!(KeyMaterialKind::from_i16(2).is_err());
    }
}
