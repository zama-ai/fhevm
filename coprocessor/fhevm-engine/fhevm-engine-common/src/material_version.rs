//! RFC-029 key-material version selection (coprocessor side).
//!
//! A `MaterialVersion` selects which key material a coprocessor uses for
//! a given operation. The whole point of the cutover is that the choice
//! must be **deterministic and identical across the fleet** -- every
//! coprocessor, given the same operation, must pick the same version, or
//! they diverge on ciphertext bytes and break consensus.
//!
//! Selection is therefore a pure function of (a) a published cutover
//! schedule and (b) the block at which the operation is anchored:
//!
//! * **Host compute** is anchored to the host chain block that requested
//!   it: `version = highest V whose H_C[chain] <= block` (default 0).
//! * **Input verification (zkpok)** is anchored to the gateway block:
//!   `version = highest V whose G <= gw_block` (default 0).
//! * **SnS** is *pinned* to the material version of its source
//!   ciphertext -- it never re-derives a version from a block, so the
//!   long tail of a pre-cutover ciphertext keeps squashing under the
//!   material it was created with.
//!
//! Two invariants the rest of the coprocessor relies on:
//!
//! 1. **Inert by default.** With no schedule rows published, every
//!    selector returns [`MaterialVersion::LEGACY`] (0), which the fetch
//!    layer maps to today's exact behavior. A node that has never seen a
//!    schedule behaves byte-for-byte like today.
//! 2. **Halt, never substitute.** If selection resolves to a version
//!    whose material a node hasn't ingested yet, the node must halt and
//!    retry that work item ([`SelectionOutcome::HaltRetry`]) -- it must
//!    *never* silently fall back to another version, or it would produce
//!    bytes no one else agrees with.

use crate::chain_id::ChainId;
use anyhow::Result;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

/// Which key material an operation uses. `0` is the legacy material
/// (today's behavior); `1` is the migrated `CompressedXofKeySet`. Kept as
/// a thin `i16` newtype so it round-trips through Postgres `SMALLINT`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaterialVersion(pub i16);

impl MaterialVersion {
    /// Legacy material: the pre-cutover behavior, and the inert default.
    pub const LEGACY: MaterialVersion = MaterialVersion(0);
    /// First migrated material: the RFC-029 `CompressedXofKeySet` cutover.
    pub const MIGRATED_V1: MaterialVersion = MaterialVersion(1);

    #[inline]
    pub fn as_i16(self) -> i16 {
        self.0
    }
}

impl From<i16> for MaterialVersion {
    #[inline]
    fn from(v: i16) -> Self {
        MaterialVersion(v)
    }
}

/// Outcome of pairing a *selected* version with the versions a node
/// currently holds. The fetch layer turns `HaltRetry` into "leave the
/// work item unclaimed and try again later" -- crucially *not* a fallback
/// to a different version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionOutcome {
    /// The selected version's material is available; proceed with it.
    Ready(MaterialVersion),
    /// The selected version's material is not (yet) available. Halt and
    /// retry; do not substitute another version.
    HaltRetry { selected: MaterialVersion },
}

/// Resolves a selected version against the versions a node holds.
///
/// This is where "halt, never substitute" lives: a missing version is a
/// `HaltRetry`, never a downgrade.
pub fn resolve_or_halt(
    selected: MaterialVersion,
    available: &[MaterialVersion],
) -> SelectionOutcome {
    if available.contains(&selected) {
        SelectionOutcome::Ready(selected)
    } else {
        SelectionOutcome::HaltRetry { selected }
    }
}

/// Pure selection over a single timeline's schedule.
///
/// `schedule` is the set of `(version, target_block)` cutovers for one
/// timeline (one host chain, or the gateway). Returns the highest version
/// whose `target_block <= block`, or [`MaterialVersion::LEGACY`] when the
/// block is unknown (`None`, e.g. a pre-migration row) or no cutover
/// applies yet. Order-independent: it does not assume the schedule is
/// sorted or monotonic.
pub fn select_from_schedule(
    schedule: &[(MaterialVersion, i64)],
    block: Option<i64>,
) -> MaterialVersion {
    let Some(block) = block else {
        return MaterialVersion::LEGACY;
    };
    schedule
        .iter()
        .filter(|(_, target_block)| *target_block <= block)
        .map(|(version, _)| *version)
        .max()
        .unwrap_or(MaterialVersion::LEGACY)
}

/// In-memory snapshot of the published cutover schedule.
///
/// Loaded once at startup like [`crate::host_chains::HostChainsCache`]
/// (refreshed on pod restart). Uses runtime `sqlx::query` rather than the
/// `query!` macro so a fresh schedule table doesn't require regenerating
/// the offline query cache.
#[derive(Clone, Default)]
pub struct MigrationScheduleCache {
    /// Per host chain: the `(version, target_block)` cutovers (H_C).
    host: HashMap<ChainId, Vec<(MaterialVersion, i64)>>,
    /// Gateway cutovers (G) for input verification.
    gateway: Vec<(MaterialVersion, i64)>,
}

impl MigrationScheduleCache {
    /// An empty schedule. Every selector resolves to
    /// [`MaterialVersion::LEGACY`] -- the inert default.
    pub fn empty() -> Self {
        Self::default()
    }

    pub async fn load(pool: &PgPool) -> Result<Self> {
        let mut host: HashMap<ChainId, Vec<(MaterialVersion, i64)>> = HashMap::new();
        let host_rows = sqlx::query(
            "SELECT host_chain_id, material_version, target_block_number \
             FROM material_version_host_schedule",
        )
        .fetch_all(pool)
        .await?;
        for row in host_rows {
            let chain_id_raw: i64 = row.try_get("host_chain_id")?;
            let version: i16 = row.try_get("material_version")?;
            let target_block: i64 = row.try_get("target_block_number")?;
            host.entry(ChainId::try_from(chain_id_raw)?)
                .or_default()
                .push((MaterialVersion(version), target_block));
        }

        let mut gateway: Vec<(MaterialVersion, i64)> = Vec::new();
        let gateway_rows = sqlx::query(
            "SELECT material_version, target_block_number \
             FROM material_version_gateway_schedule",
        )
        .fetch_all(pool)
        .await?;
        for row in gateway_rows {
            let version: i16 = row.try_get("material_version")?;
            let target_block: i64 = row.try_get("target_block_number")?;
            gateway.push((MaterialVersion(version), target_block));
        }

        Ok(Self { host, gateway })
    }

    /// Material version for a host-compute operation requested at
    /// `block_number` on `chain_id`. Unknown chain or `None` block →
    /// [`MaterialVersion::LEGACY`].
    pub fn select_host(&self, chain_id: ChainId, block_number: Option<i64>) -> MaterialVersion {
        match self.host.get(&chain_id) {
            Some(schedule) => select_from_schedule(schedule, block_number),
            None => MaterialVersion::LEGACY,
        }
    }

    /// Material version for an input-verification (zkpok) operation
    /// anchored at gateway block `gw_block_number`.
    pub fn select_gateway(&self, gw_block_number: Option<i64>) -> MaterialVersion {
        select_from_schedule(&self.gateway, gw_block_number)
    }

    /// True when no cutover has been published at all -- the inert state.
    pub fn is_inert(&self) -> bool {
        self.host.is_empty() && self.gateway.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chain(id: i64) -> ChainId {
        ChainId::try_from(id).unwrap()
    }

    const V0: MaterialVersion = MaterialVersion::LEGACY;
    const V1: MaterialVersion = MaterialVersion::MIGRATED_V1;
    const V2: MaterialVersion = MaterialVersion(2);

    // --- select_from_schedule: the core cutover rule -----------------

    #[test]
    fn empty_schedule_is_always_legacy() {
        assert_eq!(select_from_schedule(&[], Some(0)), V0);
        assert_eq!(select_from_schedule(&[], Some(1_000_000)), V0);
        assert_eq!(select_from_schedule(&[], None), V0);
    }

    #[test]
    fn null_block_is_legacy_even_with_a_schedule() {
        // Pre-migration rows carry NULL block_number and must resolve to
        // legacy, never to a migrated version.
        let schedule = [(V1, 100)];
        assert_eq!(select_from_schedule(&schedule, None), V0);
    }

    #[test]
    fn single_cutover_boundary_is_inclusive_at_hc() {
        let schedule = [(V1, 100)];
        assert_eq!(select_from_schedule(&schedule, Some(99)), V0); // before H_C
        assert_eq!(select_from_schedule(&schedule, Some(100)), V1); // exactly H_C
        assert_eq!(select_from_schedule(&schedule, Some(101)), V1); // after H_C
    }

    #[test]
    fn multiple_cutovers_pick_highest_applicable() {
        let schedule = [(V1, 100), (V2, 200)];
        assert_eq!(select_from_schedule(&schedule, Some(50)), V0);
        assert_eq!(select_from_schedule(&schedule, Some(100)), V1);
        assert_eq!(select_from_schedule(&schedule, Some(150)), V1);
        assert_eq!(select_from_schedule(&schedule, Some(200)), V2);
        assert_eq!(select_from_schedule(&schedule, Some(10_000)), V2);
    }

    #[test]
    fn selection_is_order_independent() {
        // Same schedule, rows in reverse: selection must not depend on
        // row order coming back from the DB.
        let forward = [(V1, 100), (V2, 200)];
        let reversed = [(V2, 200), (V1, 100)];
        for b in [50, 100, 150, 200, 250] {
            assert_eq!(
                select_from_schedule(&forward, Some(b)),
                select_from_schedule(&reversed, Some(b)),
                "mismatch at block {b}"
            );
        }
    }

    // --- resolve_or_halt: "halt, never substitute" -------------------

    #[test]
    fn ready_when_selected_version_is_available() {
        assert_eq!(resolve_or_halt(V0, &[V0]), SelectionOutcome::Ready(V0));
        assert_eq!(resolve_or_halt(V1, &[V0, V1]), SelectionOutcome::Ready(V1));
        assert_eq!(resolve_or_halt(V0, &[V0, V1]), SelectionOutcome::Ready(V0));
    }

    #[test]
    fn halt_retry_when_selected_version_is_missing() {
        // The cutover says v1 but this node hasn't ingested v1 yet: halt,
        // do NOT downgrade to v0.
        assert_eq!(
            resolve_or_halt(V1, &[V0]),
            SelectionOutcome::HaltRetry { selected: V1 }
        );
        // And never the reverse downgrade either.
        assert_eq!(
            resolve_or_halt(V2, &[V0, V1]),
            SelectionOutcome::HaltRetry { selected: V2 }
        );
    }

    #[test]
    fn halt_retry_when_node_holds_nothing() {
        assert_eq!(
            resolve_or_halt(V0, &[]),
            SelectionOutcome::HaltRetry { selected: V0 }
        );
    }

    // --- cache selectors --------------------------------------------

    #[test]
    fn empty_cache_is_inert_and_legacy() {
        let cache = MigrationScheduleCache::empty();
        assert!(cache.is_inert());
        assert_eq!(cache.select_host(chain(1), Some(10_000)), V0);
        assert_eq!(cache.select_gateway(Some(10_000)), V0);
    }

    #[test]
    fn cache_host_schedule_is_per_chain() {
        let mut host = HashMap::new();
        host.insert(chain(1), vec![(V1, 100)]);
        // chain 2 has no cutover scheduled.
        let cache = MigrationScheduleCache {
            host,
            gateway: vec![],
        };
        assert!(!cache.is_inert());
        // chain 1 crosses at block 100...
        assert_eq!(cache.select_host(chain(1), Some(99)), V0);
        assert_eq!(cache.select_host(chain(1), Some(100)), V1);
        // ...while chain 2 stays on legacy at the same block.
        assert_eq!(cache.select_host(chain(2), Some(100)), V0);
        // unknown chain → legacy.
        assert_eq!(cache.select_host(chain(999), Some(10_000)), V0);
    }

    #[test]
    fn cache_gateway_schedule_is_independent_of_host() {
        let cache = MigrationScheduleCache {
            host: HashMap::new(),
            gateway: vec![(V1, 500)],
        };
        assert_eq!(cache.select_gateway(Some(499)), V0);
        assert_eq!(cache.select_gateway(Some(500)), V1);
        assert_eq!(cache.select_gateway(None), V0);
        // gateway schedule must not leak into host selection.
        assert_eq!(cache.select_host(chain(1), Some(10_000)), V0);
    }
}
