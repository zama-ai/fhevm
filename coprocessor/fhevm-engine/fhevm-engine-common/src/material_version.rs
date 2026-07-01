//! RFC-029 key-material selection (coprocessor side).
//!
//! RFC-029 is a one-time cutover from legacy material (v0) to the migrated
//! `CompressedXofKeySet` (v1). Selection must be deterministic and identical
//! across the fleet -- given the same operation, every coprocessor must pick
//! the same version, or they diverge on ciphertext bytes and break consensus.
//!
//! Each timeline (a host chain, or the gateway) has at most one cutover block:
//! the operation is v1 once its anchoring block reaches that block, else v0.
//! Host compute anchors to the requesting host-chain block; input verification
//! anchors to the gateway block; SnS does not anchor to a block at all -- it is
//! pinned to its source ciphertext's stored version (handled by the caller).
//!
//! With no schedule published every selector returns [`MaterialVersion::LEGACY`].

use crate::chain_id::ChainId;
use anyhow::Result;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

/// Postgres `LISTEN`/`NOTIFY` channel signaling that the compressed-key
/// migration schedule has been published. Workers load the schedule once and
/// refresh only on this notify, so the happy path never polls.
pub const COMPRESSED_KEY_MIGRATION_SCHEDULE_CHANNEL: &str =
    "compressed_key_migration_schedule_changed";

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

/// `keys.compressed_key_migration_status` values for the RFC-029 compressed-key migration.
pub struct CompressedKeyMigrationStatus;

impl CompressedKeyMigrationStatus {
    pub const MATERIAL_READY: i16 = 1;
    pub const SCHEDULED: i16 = 2;
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

/// In-memory snapshot of the compressed-key migration schedule.
///
/// Loaded once at startup like [`crate::host_chains::HostChainsCache`] and
/// refreshed on [`COMPRESSED_KEY_MIGRATION_SCHEDULE_CHANNEL`]. Uses runtime
/// `sqlx::query` rather than the `query!` macro so a fresh schedule table
/// doesn't require regenerating the offline query cache.
#[derive(Clone, Default)]
pub struct CompressedKeyMigrationScheduleCache {
    schedule: Option<CompressedKeyMigrationSchedule>,
}

#[derive(Clone)]
struct CompressedKeyMigrationSchedule {
    /// Per host chain: the block at/after which that chain uses migrated material.
    host: HashMap<ChainId, i64>,
    /// The gateway cutover block (block at/after which inputs use migrated material), if scheduled.
    gateway: Option<i64>,
}

impl CompressedKeyMigrationScheduleCache {
    /// No compressed-key migration schedule has been applied.
    pub fn empty() -> Self {
        Self::default()
    }

    pub async fn load(pool: &PgPool) -> Result<Self> {
        let mut host: HashMap<ChainId, i64> = HashMap::new();
        let host_rows =
            sqlx::query("SELECT host_chain_id, target_block FROM material_version_host_schedule")
                .fetch_all(pool)
                .await?;
        for row in host_rows {
            let chain_id_raw: i64 = row.try_get("host_chain_id")?;
            let target_block: i64 = row.try_get("target_block")?;
            host.insert(ChainId::try_from(chain_id_raw)?, target_block);
        }

        let gateway: Option<i64> =
            sqlx::query("SELECT target_block FROM material_version_gateway_schedule")
                .fetch_optional(pool)
                .await?
                .map(|row| row.try_get("target_block"))
                .transpose()?;

        let schedule = if gateway.is_some() || !host.is_empty() {
            Some(CompressedKeyMigrationSchedule { host, gateway })
        } else {
            None
        };

        Ok(Self { schedule })
    }

    /// Material version for an input-verification operation anchored at gateway
    /// block `gw_block_number`.
    pub fn gateway_input_material_version(&self, gw_block_number: Option<i64>) -> MaterialVersion {
        self.schedule
            .as_ref()
            .map(|schedule| version_at(schedule.gateway, gw_block_number))
            .unwrap_or(MaterialVersion::LEGACY)
    }

    /// Material version for a host-compute operation requested at `block_number`.
    /// Once the compressed-key migration schedule exists, a host chain with no
    /// row is treated as post-migration and uses compressed material; this
    /// covers hosts added after the migration.
    pub fn host_compute_material_version(
        &self,
        host_chain_id: i64,
        block_number: Option<i64>,
    ) -> MaterialVersion {
        let Some(schedule) = &self.schedule else {
            return MaterialVersion::LEGACY;
        };

        match ChainId::try_from(host_chain_id) {
            Ok(chain_id) => match schedule.host.get(&chain_id).copied() {
                Some(cutover) => version_at(Some(cutover), block_number),
                None => MaterialVersion::MIGRATED_V1,
            },
            Err(_) => MaterialVersion::LEGACY,
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

    // --- cache selectors --------------------------------------------

    #[test]
    fn empty_cache_is_legacy() {
        let cache = CompressedKeyMigrationScheduleCache::empty();
        assert_eq!(cache.host_compute_material_version(1, Some(10_000)), V0);
        assert_eq!(cache.gateway_input_material_version(Some(10_000)), V0);
    }

    #[test]
    fn cache_host_schedule_is_per_chain() {
        // Each host chain carries its own cutover block on its own height; a
        // block is only ever compared against the same chain's cutover, so
        // different block times / heights across chains cannot cross-contaminate.
        let mut host = HashMap::new();
        host.insert(chain(1), 100);
        // chain 2 has no compressed-key cutover scheduled.
        let cache = CompressedKeyMigrationScheduleCache {
            schedule: Some(CompressedKeyMigrationSchedule {
                host,
                gateway: None,
            }),
        };
        assert_eq!(cache.host_compute_material_version(1, Some(99)), V0);
        assert_eq!(cache.host_compute_material_version(1, Some(100)), V1);
        // chain 2 has no row after scheduling, so it is a post-migration host.
        assert_eq!(cache.host_compute_material_version(2, Some(100)), V1);
        assert_eq!(cache.host_compute_material_version(999, Some(10_000)), V1);
    }

    #[test]
    fn host_compute_material_version_resolves_per_chain_and_falls_back_to_legacy() {
        let mut host = HashMap::new();
        host.insert(chain(1), 100);
        let cache = CompressedKeyMigrationScheduleCache {
            schedule: Some(CompressedKeyMigrationSchedule {
                host,
                gateway: None,
            }),
        };
        // Valid chain id resolves against that chain's cutover block.
        assert_eq!(cache.host_compute_material_version(1, Some(99)), V0);
        assert_eq!(cache.host_compute_material_version(1, Some(100)), V1);
        // A chain not in the schedule is a post-migration host. An out-of-range
        // id still falls back to legacy because it cannot identify a host.
        assert_eq!(cache.host_compute_material_version(2, Some(10_000)), V1);
        assert_eq!(
            cache.host_compute_material_version(i64::MIN, Some(10_000)),
            V0
        );
    }

    #[test]
    fn cache_gateway_schedule_is_independent_of_host() {
        let cache = CompressedKeyMigrationScheduleCache {
            schedule: Some(CompressedKeyMigrationSchedule {
                host: HashMap::new(),
                gateway: Some(500),
            }),
        };
        assert_eq!(cache.gateway_input_material_version(Some(499)), V0);
        assert_eq!(cache.gateway_input_material_version(Some(500)), V1);
        assert_eq!(cache.gateway_input_material_version(None), V0);
        // A scheduled migration with no host row means a post-migration host.
        assert_eq!(cache.host_compute_material_version(1, Some(10_000)), V1);
    }
}
