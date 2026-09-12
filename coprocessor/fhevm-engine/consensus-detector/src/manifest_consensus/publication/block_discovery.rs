use std::collections::{BTreeMap, HashMap};
use std::time::Duration;

use block_manifest::LEGACY_CONSENSUS_EPOCH;
use sqlx::{PgPool, Postgres, Transaction};

use crate::manifest_consensus::ExecutionError;

/// A generation-scoped host block tracked by the manifest publication state
/// machine.
#[derive(Clone, Debug)]
pub(crate) struct PendingBlock {
    pub generation: String,
    pub host_chain_id: i64,
    pub block_number: i64,
    pub block_hash: Vec<u8>,
    pub parent_block_hash: Vec<u8>,
    pub publication_cadence: i64,
    pub block_content_digest: Option<Vec<u8>>,
    pub block_handle_count: Option<i64>,
    pub manifest_revision: i64,
    pub manifest_publisher: Option<Vec<u8>>,
    pub manifest_digest: Option<Vec<u8>>,
    pub manifest_published: bool,
}

/// Stable pagination position while a publisher scans competing block
/// lineages without repeatedly selecting the same blocked candidate.
#[derive(Debug)]
pub(crate) struct ManifestProgressCursor {
    block_number: i64,
    block_hash: Vec<u8>,
}

impl ManifestProgressCursor {
    pub(crate) fn start() -> Self {
        Self {
            block_number: -1,
            block_hash: Vec::new(),
        }
    }

    pub(crate) fn advance_to(&mut self, block: &PendingBlock) {
        self.block_number = block.block_number;
        self.block_hash.clone_from(&block.block_hash);
    }
}

// This query protects lineage ordering only; it must never monopolize a
// database connection while a large recovery backlog is present.
const MANIFEST_WORK_SELECTION_TIMEOUT: Duration = Duration::from_secs(5);
/// How far an established frontier walks back to pick up late producer rows.
const MANIFEST_DISCOVERY_BLOCK_OVERLAP: i64 = 5;

/// Per-chain span of `block_manifest_state` already stored for this generation.
///
/// Presence of this value means discovery is incremental (`EstablishedFrontier`),
/// not a first-pass bootstrap. Loaded as `MIN`/`MAX(block_number)` grouped by
/// `host_chain_id`.
///
/// `first_block` is the durable lower bound: the legacy bootstrap tip, an
/// upgrade `start_block`, or whatever was first inserted. Later passes must
/// not walk below it (and not below the generation window when one exists).
///
/// `last_block` is the highest height already tracked. New blocks after it
/// are meant to arrive via parent→child discovery. Producer catch-up only
/// looks back `MANIFEST_DISCOVERY_BLOCK_OVERLAP` from this height to pick up
/// late `handle_producer_block` rows.
struct ExistingFrontier {
    first_block: i64,
    last_block: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DiscoveryKind {
    /// Rows already exist; revisit a small overlap from the highest known block.
    EstablishedFrontier,
    /// First `legacy` pass; the current tip is the arbitrary history start.
    LegacyTip,
    /// First upgrade-generation pass; start at the configured window.
    UpgradeWindow,
}

struct DiscoveryBound {
    host_chain_id: i64,
    first_block: i64,
    /// Inclusive upper bound for producer-handle catch-up. `None` means
    /// unbounded (bootstrap). Established frontiers cap at `ExistingFrontier.last_block`.
    last_block: Option<i64>,
    publication_cadence: i64,
    kind: DiscoveryKind,
}

/// Default cadence: Ethereum L1 (~12s) publishes every 5 blocks, Polygon L2
/// (~2s) and unknown chains every 30. Overlays from
/// `--manifest-publication-cadence CHAIN_ID:CADENCE` win for new inserts only.
pub(crate) fn publication_cadence(chain_id: i64, overrides: &BTreeMap<i64, i64>) -> i64 {
    overrides
        .get(&chain_id)
        .copied()
        .unwrap_or_else(|| default_publication_cadence(chain_id))
}

fn default_publication_cadence(chain_id: i64) -> i64 {
    match chain_id {
        1 | 11155111 | 560048 => 5, // Ethereum mainnet, Sepolia, Hoodi
        137 | 80002 => 30,          // Polygon mainnet, Amoy
        _ => 30,
    }
}

/// Parses one `--manifest-publication-cadence CHAIN_ID:CADENCE` overlay.
pub fn parse_publication_cadence_override(value: &str) -> Result<(i64, i64), String> {
    let (chain, cadence) = value
        .split_once(':')
        .ok_or_else(|| format!("expected CHAIN_ID:CADENCE, got {value:?}"))?;
    let chain_id = chain
        .parse::<i64>()
        .map_err(|_| format!("invalid chain id in cadence override {value:?}"))?;
    let cadence = cadence
        .parse::<i64>()
        .map_err(|_| format!("invalid cadence in override {value:?}"))?;
    if chain_id < 0 {
        return Err(format!(
            "chain id must be >= 0 in cadence override {value:?}"
        ));
    }
    if cadence <= 0 {
        return Err(format!("cadence must be > 0 in override {value:?}"));
    }
    Ok((chain_id, cadence))
}

pub fn publication_cadence_overrides(
    pairs: impl IntoIterator<Item = (i64, i64)>,
) -> Result<BTreeMap<i64, i64>, String> {
    let mut overrides = BTreeMap::new();
    for (chain_id, cadence) in pairs {
        if overrides.insert(chain_id, cadence).is_some() {
            return Err(format!(
                "duplicate --manifest-publication-cadence for chain {chain_id}"
            ));
        }
    }
    Ok(overrides)
}

fn discovery_bound_for_chain(
    generation: &str,
    host_chain_id: i64,
    latest_block: i64,
    frontier: Option<&ExistingFrontier>,
    window_start: Option<i64>,
    cadence_overrides: &BTreeMap<i64, i64>,
) -> Option<DiscoveryBound> {
    let publication_cadence = publication_cadence(host_chain_id, cadence_overrides);
    if let Some(frontier) = frontier {
        let floor = window_start.unwrap_or(frontier.first_block);
        let first_block = frontier
            .last_block
            .saturating_sub(MANIFEST_DISCOVERY_BLOCK_OVERLAP)
            .max(floor);
        return Some(DiscoveryBound {
            host_chain_id,
            first_block,
            last_block: Some(frontier.last_block),
            publication_cadence,
            kind: DiscoveryKind::EstablishedFrontier,
        });
    }
    if generation == LEGACY_CONSENSUS_EPOCH {
        return Some(DiscoveryBound {
            host_chain_id,
            first_block: latest_block,
            last_block: None,
            publication_cadence,
            kind: DiscoveryKind::LegacyTip,
        });
    }
    Some(DiscoveryBound {
        host_chain_id,
        first_block: window_start?,
        last_block: None,
        publication_cadence,
        kind: DiscoveryKind::UpgradeWindow,
    })
}

/// Seeds manifest processing from the stack-local host-chain view and the
/// immutable handle-to-producer-block associations written by its listener.
///
/// Generation ownership is established by stack routing, not by a block-number
/// predicate on this association. Blue reads the public table; Green reads the
/// GCS table through its pool's `search_path`. Green remains parked until
/// pre-`start_block` work has been pruned, and Blue follows the same rule until
/// its stack-version gate retires it at cutover.
///
/// `block_manifest_state` is also the durable discovery frontier. The first
/// generation starts at the latest block already known on each chain because
/// its historical boundary is intentionally arbitrary. Upgrade generations
/// start at their configured per-chain `start_block`. Once a frontier exists,
/// each pass revisits a small block window to tolerate recently-arrived rows,
/// without ever crossing an upgrade generation's start boundary.
#[cfg(test)]
pub(crate) async fn discover_blocks(pool: &PgPool) -> Result<u64, ExecutionError> {
    let generation = crate::manifest_consensus::storage::active::load_generation(pool).await?;
    discover_blocks_for_generation(pool, &generation, &BTreeMap::new()).await
}

pub(crate) async fn discover_blocks_for_generation(
    pool: &PgPool,
    generation: &str,
    cadence_overrides: &BTreeMap<i64, i64>,
) -> Result<u64, ExecutionError> {
    let frontiers = load_existing_frontiers(pool, generation).await?;
    let windows = load_generation_windows(pool, generation).await?;
    let latest_valid = load_latest_valid_blocks(pool).await?;

    let mut inserted = 0_u64;
    for latest in latest_valid {
        let Some(bound) = discovery_bound_for_chain(
            generation,
            latest.host_chain_id,
            latest.block_number,
            frontiers.get(&latest.host_chain_id),
            windows.get(&latest.host_chain_id).copied(),
            cadence_overrides,
        ) else {
            continue;
        };

        inserted += match bound.kind {
            DiscoveryKind::EstablishedFrontier => {
                insert_blocks_with_allowed_handles(pool, generation, &bound).await?
            }
            DiscoveryKind::LegacyTip | DiscoveryKind::UpgradeWindow => {
                insert_bootstrap_blocks(pool, generation, &bound).await?
            }
        };
    }
    Ok(inserted)
}

struct LatestValidBlock {
    host_chain_id: i64,
    block_number: i64,
}

async fn load_existing_frontiers(
    pool: &PgPool,
    generation: &str,
) -> Result<HashMap<i64, ExistingFrontier>, ExecutionError> {
    let rows = sqlx::query!(
        r#"
        SELECT host_chain_id,
               MIN(block_number) AS "first_block!",
               MAX(block_number) AS "last_block!"
          FROM block_manifest_state
         WHERE generation = $1
         GROUP BY host_chain_id
        "#,
        generation,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| {
            (
                row.host_chain_id,
                ExistingFrontier {
                    first_block: row.first_block,
                    last_block: row.last_block,
                },
            )
        })
        .collect())
}

async fn load_generation_windows(
    pool: &PgPool,
    generation: &str,
) -> Result<HashMap<i64, i64>, ExecutionError> {
    let rows = sqlx::query!(
        r#"
        SELECT host_chain_id, start_block
          FROM generation_block_window
         WHERE generation = $1
        "#,
        generation,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| (row.host_chain_id, row.start_block))
        .collect())
}

async fn load_latest_valid_blocks(pool: &PgPool) -> Result<Vec<LatestValidBlock>, ExecutionError> {
    let rows = sqlx::query!(
        r#"
        SELECT chain_id AS host_chain_id,
               MAX(block_number) AS "block_number!"
          FROM host_chain_blocks_valid
         WHERE block_status <> 'orphaned'
           AND OCTET_LENGTH(parent_hash) = 32
         GROUP BY chain_id
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| LatestValidBlock {
            host_chain_id: row.host_chain_id,
            block_number: row.block_number,
        })
        .collect())
}

async fn insert_bootstrap_blocks(
    pool: &PgPool,
    generation: &str,
    bound: &DiscoveryBound,
) -> Result<u64, ExecutionError> {
    let anchored = insert_generation_anchor(pool, generation, bound).await?;
    if anchored == 0 {
        return Ok(0);
    }
    Ok(anchored + insert_blocks_with_allowed_handles(pool, generation, bound).await?)
}

async fn insert_generation_anchor(
    pool: &PgPool,
    generation: &str,
    bound: &DiscoveryBound,
) -> Result<u64, ExecutionError> {
    let inserted = sqlx::query_scalar!(
        r#"
        WITH inserted AS (
            INSERT INTO block_manifest_state (
                generation,
                host_chain_id,
                block_number,
                block_hash,
                parent_block_hash,
                publication_cadence
            )
            SELECT $1,
                   host.chain_id,
                   host.block_number,
                   host.block_hash,
                   host.parent_hash,
                   $3
              FROM host_chain_blocks_valid host
             WHERE host.chain_id = $2
               AND host.block_number = $4
               AND host.block_status <> 'orphaned'
               AND OCTET_LENGTH(host.parent_hash) = 32
            ON CONFLICT (generation, host_chain_id, block_hash) DO NOTHING
            RETURNING 1
        )
        SELECT COUNT(*)::BIGINT AS "inserted!" FROM inserted
        "#,
        generation,
        bound.host_chain_id,
        bound.publication_cadence,
        bound.first_block,
    )
    .fetch_one(pool)
    .await?;
    Ok(u64::try_from(inserted).expect("insert count is non-negative"))
}

async fn insert_blocks_with_allowed_handles(
    pool: &PgPool,
    generation: &str,
    bound: &DiscoveryBound,
) -> Result<u64, ExecutionError> {
    let inserted = sqlx::query_scalar!(
        r#"
        WITH inserted AS (
            INSERT INTO block_manifest_state (
                generation,
                host_chain_id,
                block_number,
                block_hash,
                parent_block_hash,
                publication_cadence
            )
            SELECT $1,
                   producer.host_chain_id,
                   host.block_number,
                   host.block_hash,
                   host.parent_hash,
                   $3
              FROM handle_producer_block producer
              JOIN host_chain_blocks_valid host
                ON host.chain_id = producer.host_chain_id
               AND host.block_number = producer.producer_block_number
               AND host.block_hash = producer.producer_block_hash
             WHERE producer.host_chain_id = $2
               AND producer.producer_block_number >= $4
               AND ($5::BIGINT IS NULL OR producer.producer_block_number <= $5)
               AND OCTET_LENGTH(host.parent_hash) = 32
               AND host.block_status <> 'orphaned'
            ON CONFLICT (generation, host_chain_id, block_hash) DO NOTHING
            RETURNING 1
        )
        SELECT COUNT(*)::BIGINT AS "inserted!" FROM inserted
        "#,
        generation,
        bound.host_chain_id,
        bound.publication_cadence,
        bound.first_block,
        bound.last_block,
    )
    .fetch_one(pool)
    .await?;
    Ok(u64::try_from(inserted).expect("insert count is non-negative"))
}

#[cfg(test)]
pub(crate) async fn pending_chain_ids(pool: &PgPool) -> Result<Vec<i64>, ExecutionError> {
    let generation = crate::manifest_consensus::storage::active::load_generation(pool).await?;
    pending_chain_ids_for_generation(pool, &generation).await
}

pub(crate) async fn pending_chain_ids_for_generation(
    pool: &PgPool,
    generation: &str,
) -> Result<Vec<i64>, ExecutionError> {
    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT host_chain_id
         FROM block_manifest_state candidate
         WHERE candidate.generation = $1
           AND (block_content_digest IS NULL
            OR (
                manifest_required
                AND manifest_published = FALSE
                AND (
                    publication_error_count = 0
                    OR publication_next_retry_at <= NOW()
                )
            ))
         ORDER BY host_chain_id
        "#,
        generation,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|row| row.host_chain_id).collect())
}

/// Locks the earliest eligible work row after `cursor`. `SKIP LOCKED` lets
/// another worker progress an independent lineage while this transaction holds
/// the selected row. `cursor` is only this caller's local scan position.
///
/// A required parent whose retry budget is exhausted
/// (`publication_next_retry_at IS NULL` after errors) is not a blocker: that
/// publication identity is skipped and descendants may progress. Cadence is
/// `MOD(block_number, publication_cadence) = 0`; skipping one identity does
/// not move the next due height.
#[cfg(test)]
pub(crate) async fn lock_next_block_to_progress(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    cursor: &ManifestProgressCursor,
) -> Result<Option<PendingBlock>, ExecutionError> {
    let generation = sqlx::query_scalar::<_, String>(
        "SELECT generation FROM blue_green_generation WHERE singleton = TRUE",
    )
    .fetch_one(trx.as_mut())
    .await?;
    lock_next_block_to_progress_for_generation(trx, host_chain_id, cursor, &generation).await
}

pub(crate) async fn lock_next_block_to_progress_for_generation(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    cursor: &ManifestProgressCursor,
    generation: &str,
) -> Result<Option<PendingBlock>, ExecutionError> {
    set_local_statement_timeout(trx, MANIFEST_WORK_SELECTION_TIMEOUT).await?;
    let result = sqlx::query!(
        r#"
        WITH RECURSIVE blocked_descendants AS (
            SELECT child.generation,
                   child.host_chain_id,
                   child.block_hash
              FROM block_manifest_state blocker
              JOIN block_manifest_state child
                ON child.generation = blocker.generation
               AND child.host_chain_id = blocker.host_chain_id
               AND child.parent_block_hash = blocker.block_hash
             WHERE blocker.host_chain_id = $1
               AND blocker.generation = $4
               AND (
                    blocker.block_content_digest IS NULL
                    OR (
                        blocker.manifest_required
                        AND blocker.manifest_published = FALSE
                        AND (
                            blocker.publication_error_count = 0
                            OR blocker.publication_next_retry_at IS NOT NULL
                        )
                    )
               )
            UNION
            SELECT child.generation,
                   child.host_chain_id,
                   child.block_hash
              FROM blocked_descendants blocked
              JOIN block_manifest_state child
                ON child.generation = blocked.generation
               AND child.host_chain_id = blocked.host_chain_id
               AND child.parent_block_hash = blocked.block_hash
        )
        SELECT candidate.generation,
               candidate.host_chain_id,
               candidate.block_number,
               candidate.block_hash,
               candidate.parent_block_hash,
               candidate.publication_cadence,
               candidate.block_content_digest,
               candidate.block_handle_count,
               candidate.manifest_revision,
               candidate.manifest_publisher,
               candidate.manifest_digest,
               candidate.manifest_published
          FROM block_manifest_state candidate
         WHERE candidate.host_chain_id = $1
           AND candidate.generation = $4
           AND (
                candidate.block_content_digest IS NULL
                OR (
                    candidate.manifest_required
                    AND candidate.manifest_published = FALSE
                    AND (
                        candidate.publication_error_count = 0
                        OR candidate.publication_next_retry_at <= NOW()
                    )
                )
           )
           AND NOT EXISTS (
                SELECT 1
                  FROM blocked_descendants blocked
                 WHERE blocked.host_chain_id = candidate.host_chain_id
                   AND blocked.generation = candidate.generation
                   AND blocked.block_hash = candidate.block_hash
           )
           AND (
                candidate.block_number > $2
                OR (
                    candidate.block_number = $2
                    AND candidate.block_hash > $3
                )
           )
         ORDER BY candidate.block_number, candidate.block_hash
         LIMIT 1
           FOR UPDATE SKIP LOCKED
        "#,
        host_chain_id,
        cursor.block_number,
        &cursor.block_hash,
        generation,
    )
    .fetch_optional(trx.as_mut())
    .await;

    // The bound is only for the recursive selector. Later manifest preparation
    // may legitimately issue several ordinary queries in this transaction.
    if result.is_ok() {
        set_local_statement_timeout(trx, Duration::ZERO).await?;
    }
    let row = result?;

    Ok(row.map(|row| PendingBlock {
        generation: row.generation,
        host_chain_id: row.host_chain_id,
        block_number: row.block_number,
        block_hash: row.block_hash,
        parent_block_hash: row.parent_block_hash,
        publication_cadence: row.publication_cadence,
        block_content_digest: row.block_content_digest,
        block_handle_count: row.block_handle_count,
        manifest_revision: row.manifest_revision,
        manifest_publisher: row.manifest_publisher,
        manifest_digest: row.manifest_digest,
        manifest_published: row.manifest_published,
    }))
}

pub(super) async fn set_local_statement_timeout(
    trx: &mut Transaction<'_, Postgres>,
    timeout: Duration,
) -> Result<(), ExecutionError> {
    let timeout = format!("{}ms", timeout.as_millis());
    sqlx::query!("SELECT set_config('statement_timeout', $1, TRUE)", timeout)
        .fetch_one(trx.as_mut())
        .await?;
    Ok(())
}

/// Discovers children only below host blocks whose lineage can still change.
/// Finalized and orphaned parents are closed after their currently visible
/// children have been copied, keeping polling cost bounded by the finality window.
#[cfg(test)]
pub(crate) async fn discover_children(pool: &PgPool) -> Result<u64, ExecutionError> {
    let generation = crate::manifest_consensus::storage::active::load_generation(pool).await?;
    discover_children_for_generation(pool, &generation).await
}

pub(crate) async fn discover_children_for_generation(
    pool: &PgPool,
    generation: &str,
) -> Result<u64, ExecutionError> {
    let row = sqlx::query!(
        r#"
        WITH inserted AS (
            INSERT INTO block_manifest_state (
                generation,
                host_chain_id,
                block_number,
                block_hash,
                parent_block_hash,
                publication_cadence
            )
            SELECT parent.generation,
                   child.chain_id,
                   child.block_number,
                   child.block_hash,
                   child.parent_hash,
                   parent.publication_cadence
              FROM block_manifest_state parent
              JOIN host_chain_blocks_valid parent_host
                ON parent_host.chain_id = parent.host_chain_id
               AND parent_host.block_hash = parent.block_hash
              JOIN host_chain_blocks_valid child
                ON child.chain_id = parent.host_chain_id
               AND child.parent_hash = parent.block_hash
             WHERE NOT parent.child_block_discovery_closed
               AND parent.generation = $1
               AND parent_host.block_status <> 'orphaned'
               AND child.block_status <> 'orphaned'
               AND OCTET_LENGTH(child.parent_hash) = 32
            ON CONFLICT (generation, host_chain_id, block_hash) DO NOTHING
            RETURNING 1
        ), closed AS (
            UPDATE block_manifest_state block
               SET child_block_discovery_closed = TRUE,
                   updated_at = NOW()
              FROM host_chain_blocks_valid host
             WHERE NOT block.child_block_discovery_closed
               AND block.generation = $1
               AND host.chain_id = block.host_chain_id
               AND host.block_hash = block.block_hash
               AND host.block_status IN ('finalized', 'orphaned')
            RETURNING 1
        )
        SELECT COUNT(*) AS "inserted!" FROM inserted
        "#,
        generation,
    )
    .fetch_one(pool)
    .await?;
    Ok(u64::try_from(row.inserted).expect("insert count is non-negative"))
}

pub(crate) async fn discover_children_of(
    trx: &mut Transaction<'_, Postgres>,
    block: &PendingBlock,
) -> Result<u64, ExecutionError> {
    let row = sqlx::query!(
        r#"
        WITH inserted AS (
            INSERT INTO block_manifest_state (
                generation,
                host_chain_id,
                block_number,
                block_hash,
                parent_block_hash,
                publication_cadence
            )
            SELECT $4,
                   child.chain_id,
                   child.block_number,
                   child.block_hash,
                   child.parent_hash,
                   $3
              FROM host_chain_blocks_valid child
             WHERE child.chain_id = $1
               AND child.parent_hash = $2
               AND child.block_status <> 'orphaned'
               AND OCTET_LENGTH(child.parent_hash) = 32
            ON CONFLICT (generation, host_chain_id, block_hash) DO NOTHING
            RETURNING 1
        )
        SELECT COUNT(*) AS "inserted!" FROM inserted
        "#,
        block.host_chain_id,
        &block.block_hash,
        block.publication_cadence,
        block.generation,
    )
    .fetch_one(trx.as_mut())
    .await?;
    Ok(u64::try_from(row.inserted).expect("insert count is non-negative"))
}

#[cfg(test)]
mod discovery_bound_tests {
    use super::*;

    fn none() -> BTreeMap<i64, i64> {
        BTreeMap::new()
    }

    fn bound(
        generation: &str,
        host_chain_id: i64,
        latest_block: i64,
        frontier: Option<&ExistingFrontier>,
        window_start: Option<i64>,
    ) -> Option<DiscoveryBound> {
        discovery_bound_for_chain(
            generation,
            host_chain_id,
            latest_block,
            frontier,
            window_start,
            &none(),
        )
    }

    #[test]
    fn publication_cadence_matches_known_chains() {
        assert_eq!(publication_cadence(1, &none()), 5);
        assert_eq!(publication_cadence(11155111, &none()), 5);
        assert_eq!(publication_cadence(560048, &none()), 5);
        assert_eq!(publication_cadence(137, &none()), 30);
        assert_eq!(publication_cadence(80002, &none()), 30);
        assert_eq!(publication_cadence(8453, &none()), 30);
        assert_eq!(publication_cadence(84532, &none()), 30);
        assert_eq!(publication_cadence(31337, &none()), 30);
    }

    #[test]
    fn publication_cadence_override_applies_to_one_chain() {
        let overrides = publication_cadence_overrides([(31337, 1)]).unwrap();
        assert_eq!(publication_cadence(31337, &overrides), 1);
        assert_eq!(publication_cadence(1, &overrides), 5);
        assert_eq!(publication_cadence(137, &overrides), 30);
        let bound =
            discovery_bound_for_chain(LEGACY_CONSENSUS_EPOCH, 31337, 50, None, None, &overrides)
                .unwrap();
        assert_eq!(bound.publication_cadence, 1);
    }

    #[test]
    fn publication_cadence_override_rejects_duplicates_and_bad_values() {
        assert!(parse_publication_cadence_override("31337").is_err());
        assert!(parse_publication_cadence_override("31337:0").is_err());
        assert!(parse_publication_cadence_override("-1:5").is_err());
        assert_eq!(
            parse_publication_cadence_override("31337:1").unwrap(),
            (31337, 1)
        );
        assert!(publication_cadence_overrides([(1, 5), (1, 1)]).is_err());
    }

    #[test]
    fn legacy_without_frontier_starts_at_the_tip() {
        let bound = bound(LEGACY_CONSENSUS_EPOCH, 137, 50, None, None).unwrap();
        assert_eq!(bound.first_block, 50);
        assert_eq!(bound.kind, DiscoveryKind::LegacyTip);
        assert_eq!(bound.publication_cadence, 30);
    }

    #[test]
    fn upgrade_without_frontier_starts_at_the_window() {
        let bound = bound("1", 137, 80, None, Some(60)).unwrap();
        assert_eq!(bound.first_block, 60);
        assert_eq!(bound.kind, DiscoveryKind::UpgradeWindow);
    }

    #[test]
    fn upgrade_without_window_is_skipped() {
        assert!(bound("1", 137, 80, None, None).is_none());
    }

    #[test]
    fn established_frontier_revisits_five_blocks() {
        let frontier = ExistingFrontier {
            first_block: 100,
            last_block: 110,
        };
        let bound = bound(LEGACY_CONSENSUS_EPOCH, 137, 200, Some(&frontier), None).unwrap();
        assert_eq!(bound.first_block, 105);
        assert_eq!(bound.last_block, Some(110));
        assert_eq!(bound.kind, DiscoveryKind::EstablishedFrontier);
    }

    #[test]
    fn established_frontier_does_not_walk_below_the_upgrade_window() {
        let frontier = ExistingFrontier {
            first_block: 50,
            last_block: 110,
        };
        let bound = bound("1", 137, 200, Some(&frontier), Some(108)).unwrap();
        assert_eq!(bound.first_block, 108);
        assert_eq!(bound.last_block, Some(110));
        assert_eq!(bound.kind, DiscoveryKind::EstablishedFrontier);
    }

    #[test]
    fn bootstrap_bounds_have_no_upper_cap() {
        let legacy = bound(LEGACY_CONSENSUS_EPOCH, 137, 50, None, None).unwrap();
        assert_eq!(legacy.last_block, None);
        let upgrade = bound("1", 137, 80, None, Some(60)).unwrap();
        assert_eq!(upgrade.last_block, None);
    }
}
