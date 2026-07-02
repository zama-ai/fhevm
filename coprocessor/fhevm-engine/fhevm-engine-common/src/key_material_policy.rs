//! RFC-029 key-material policy (coprocessor side).
//!
//! RFC-029 is a one-time cutover from legacy material (v0) to the default
//! `CompressedXofKeySet` material. Selection must be deterministic and identical
//! across the fleet -- given the same operation, every coprocessor must pick
//! the same material, or they diverge on ciphertext bytes and break consensus.
//!
//! Each timeline (a host chain, or the gateway) has at most one cutover block:
//! the operation is v1 once its anchoring block reaches that block, else v0.
//! Host compute anchors to the requesting host-chain block; input verification
//! anchors to the gateway block; SnS does not anchor to a block at all -- it is
//! pinned to its source ciphertext's stored version (handled by the caller).
//!
//! `MaterialVersion` is the persisted ciphertext label. Worker execution uses
//! [`KeyMaterialSelection`] so the normal path reads as "default material" and
//! only pre-cutover work is a legacy override.

use crate::chain_id::ChainId;
use anyhow::Result;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

/// Postgres `LISTEN`/`NOTIFY` channel signaling that the legacy-key cutover has
/// been published. Workers load the cutover once and refresh only on this
/// notify, so the happy path never polls.
pub const LEGACY_KEY_CUTOVER_CHANNEL: &str = "legacy_key_cutover_changed";

/// Which key material an operation uses. `0` is the legacy material (today's
/// behavior); `1` is the migrated `CompressedXofKeySet`. A thin `i16` newtype
/// so it round-trips through Postgres `SMALLINT`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaterialVersion(pub i16);

impl MaterialVersion {
    /// Legacy material: the pre-cutover behavior, and the inert default.
    pub const LEGACY: MaterialVersion = MaterialVersion(0);
    /// Migrated material: the RFC-029 `CompressedXofKeySet` cutover.
    pub const MIGRATED_V1: MaterialVersion = MaterialVersion(1);
}

/// Which key material a worker should install for one execution group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyMaterialSelection {
    /// Normal/future material. During RFC-029 post-cutover execution this is
    /// the compressed XOF material and must not fall back to legacy.
    Default,
    /// Exceptional RFC-029 pre-cutover path: force the legacy `sks_key`.
    ForceLegacy,
}

impl KeyMaterialSelection {
    pub fn required_material_version(self) -> MaterialVersion {
        match self {
            Self::Default => MaterialVersion::MIGRATED_V1,
            Self::ForceLegacy => MaterialVersion::LEGACY,
        }
    }
}

/// `keys.compressed_key_state` values for the RFC-029 compressed-key migration.
pub struct CompressedKeyState;

impl CompressedKeyState {
    pub const STAGED_FOR_CUTOVER: i16 = 1;
    pub const CUTOVER_SCHEDULED: i16 = 2;
}

/// The compressed-key cutover rule for a single timeline: v1 once `observed` reaches the
/// `cutover` block, else v0. An absent cutover (no schedule) or an unknown
/// `observed` block (e.g. a pre-migration row) resolves to legacy.
fn version_at(cutover: Option<i64>, observed: Option<i64>) -> MaterialVersion {
    match (cutover, observed) {
        (Some(cutover_block), Some(block)) if block >= cutover_block => {
            MaterialVersion::MIGRATED_V1
        }
        _ => MaterialVersion::LEGACY,
    }
}

fn selection_at(cutover: Option<i64>, observed: Option<i64>) -> KeySelection {
    match version_at(cutover, observed) {
        MaterialVersion::LEGACY => KeySelection::force_legacy(),
        MaterialVersion::MIGRATED_V1 => KeySelection::default_material(),
        _ => KeySelection::force_legacy(),
    }
}

/// In-memory snapshot of the temporary RFC-029 legacy-key cutover.
///
/// Loaded once at startup like [`crate::host_chains::HostChainsCache`] and
/// refreshed on [`LEGACY_KEY_CUTOVER_CHANNEL`]. Uses runtime
/// `sqlx::query` rather than the `query!` macro so a fresh schedule table
/// doesn't require regenerating the offline query cache.
#[derive(Clone, Default)]
pub struct LegacyKeyCutoverPolicy {
    cutover: Option<LegacyKeyCutover>,
}

#[derive(Clone)]
struct LegacyKeyCutover {
    /// Per host chain: before this block, force legacy material.
    host: HashMap<ChainId, i64>,
    /// Gateway cutover block: before this block, force legacy material.
    gateway: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeySelection {
    pub key_read: KeyMaterialSelection,
    pub output_label: MaterialVersion,
}

impl KeySelection {
    pub fn force_legacy() -> Self {
        Self {
            key_read: KeyMaterialSelection::ForceLegacy,
            output_label: MaterialVersion::LEGACY,
        }
    }

    pub fn default_material() -> Self {
        Self {
            key_read: KeyMaterialSelection::Default,
            output_label: MaterialVersion::MIGRATED_V1,
        }
    }
}

impl LegacyKeyCutoverPolicy {
    /// No RFC-029 legacy-key cutover has been applied.
    pub fn empty() -> Self {
        Self::default()
    }

    pub async fn load(pool: &PgPool) -> Result<Self> {
        let mut host: HashMap<ChainId, i64> = HashMap::new();
        let host_rows =
            sqlx::query("SELECT host_chain_id, target_block FROM legacy_key_host_cutover")
                .fetch_all(pool)
                .await?;
        for row in host_rows {
            let chain_id_raw: i64 = row.try_get("host_chain_id")?;
            let target_block: i64 = row.try_get("target_block")?;
            host.insert(ChainId::try_from(chain_id_raw)?, target_block);
        }

        let gateway: Option<i64> =
            sqlx::query("SELECT target_block FROM legacy_key_gateway_cutover")
                .fetch_optional(pool)
                .await?
                .map(|row| row.try_get("target_block"))
                .transpose()?;

        let cutover = if gateway.is_some() || !host.is_empty() {
            Some(LegacyKeyCutover { host, gateway })
        } else {
            None
        };

        Ok(Self { cutover })
    }

    /// Key selection for an input-verification operation anchored at gateway block.
    pub fn gateway_input_selection(&self, gw_block_number: Option<i64>) -> KeySelection {
        self.cutover
            .as_ref()
            .map(|cutover| selection_at(cutover.gateway, gw_block_number))
            .unwrap_or_else(KeySelection::force_legacy)
    }

    /// Key selection for a host-compute operation requested at `block_number`.
    /// Once the cutover exists, a host chain with no row is treated as
    /// post-cutover and uses default material; this covers hosts added after
    /// the migration.
    pub fn host_compute_selection(
        &self,
        host_chain_id: i64,
        block_number: Option<i64>,
    ) -> KeySelection {
        let Some(cutover) = &self.cutover else {
            return KeySelection::force_legacy();
        };

        match ChainId::try_from(host_chain_id) {
            Ok(chain_id) => match cutover.host.get(&chain_id).copied() {
                Some(target_block) => selection_at(Some(target_block), block_number),
                None => KeySelection::default_material(),
            },
            Err(_) => KeySelection::force_legacy(),
        }
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

    // --- version_at: the core cutover rule ---------------------------

    #[test]
    fn no_cutover_is_always_legacy() {
        assert_eq!(version_at(None, Some(0)), V0);
        assert_eq!(version_at(None, Some(1_000_000)), V0);
        assert_eq!(version_at(None, None), V0);
    }

    #[test]
    fn null_block_is_legacy_even_with_a_cutover() {
        // Pre-migration rows carry NULL block and must resolve to legacy.
        assert_eq!(version_at(Some(100), None), V0);
    }

    #[test]
    fn cutover_boundary_is_inclusive() {
        assert_eq!(version_at(Some(100), Some(99)), V0); // before the cutover block
        assert_eq!(version_at(Some(100), Some(100)), V1); // at the cutover block
        assert_eq!(version_at(Some(100), Some(101)), V1); // after the cutover block
        assert_eq!(version_at(Some(100), Some(10_000)), V1);
    }

    // --- policy selectors -------------------------------------------

    #[test]
    fn empty_policy_forces_legacy() {
        let policy = LegacyKeyCutoverPolicy::empty();
        assert_eq!(
            policy.host_compute_selection(1, Some(10_000)).output_label,
            V0
        );
        assert_eq!(
            policy.gateway_input_selection(Some(10_000)).key_read,
            KeyMaterialSelection::ForceLegacy
        );
    }

    #[test]
    fn cache_host_schedule_is_per_chain() {
        // Each host chain carries its own cutover block on its own height; a
        // block is only ever compared against the same chain's cutover, so
        // different block times / heights across chains cannot cross-contaminate.
        let mut host = HashMap::new();
        host.insert(chain(1), 100);
        // chain 2 has no compressed-key cutover scheduled.
        let policy = LegacyKeyCutoverPolicy {
            cutover: Some(LegacyKeyCutover {
                host,
                gateway: None,
            }),
        };
        assert_eq!(policy.host_compute_selection(1, Some(99)).output_label, V0);
        assert_eq!(policy.host_compute_selection(1, Some(100)).output_label, V1);
        // chain 2 has no row after scheduling, so it is a post-migration host.
        assert_eq!(policy.host_compute_selection(2, Some(100)).output_label, V1);
        assert_eq!(
            policy
                .host_compute_selection(999, Some(10_000))
                .output_label,
            V1
        );
    }

    #[test]
    fn host_compute_selection_resolves_per_chain_and_falls_back_to_legacy() {
        let mut host = HashMap::new();
        host.insert(chain(1), 100);
        let policy = LegacyKeyCutoverPolicy {
            cutover: Some(LegacyKeyCutover {
                host,
                gateway: None,
            }),
        };
        // Valid chain id resolves against that chain's cutover block.
        assert_eq!(policy.host_compute_selection(1, Some(99)).output_label, V0);
        assert_eq!(policy.host_compute_selection(1, Some(100)).output_label, V1);
        // A chain not in the schedule is a post-migration host. An out-of-range
        // id still falls back to legacy because it cannot identify a host.
        assert_eq!(
            policy.host_compute_selection(2, Some(10_000)).output_label,
            V1
        );
        assert_eq!(
            policy
                .host_compute_selection(i64::MIN, Some(10_000))
                .output_label,
            V0
        );
    }

    #[test]
    fn cache_gateway_schedule_is_independent_of_host() {
        let policy = LegacyKeyCutoverPolicy {
            cutover: Some(LegacyKeyCutover {
                host: HashMap::new(),
                gateway: Some(500),
            }),
        };
        assert_eq!(policy.gateway_input_selection(Some(499)).output_label, V0);
        assert_eq!(policy.gateway_input_selection(Some(500)).output_label, V1);
        assert_eq!(policy.gateway_input_selection(None).output_label, V0);
        // A scheduled migration with no host row means a post-migration host.
        assert_eq!(
            policy.host_compute_selection(1, Some(10_000)).output_label,
            V1
        );
    }
}
