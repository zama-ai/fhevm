use alloy_primitives::Address;
use alloy_primitives::FixedBytes;
use alloy_primitives::Log;
use alloy_primitives::Uint;
use anyhow::Result;
use bigdecimal::BigDecimal;
use fhevm_engine_common::bridge::{chain_id_from_handle, derive_dst_handle};
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::database::{
    connect_pool_with_options_and_connect_options, PoolRefreshHandle,
};
use fhevm_engine_common::telemetry;
use fhevm_engine_common::types::{
    AllowEvents, SchedulePriority, SupportedFheOperations,
};
use fhevm_engine_common::utils::DatabaseURL;
use fhevm_engine_common::utils::{to_hex, HeartBeat};
use prometheus::{register_int_counter_vec, IntCounterVec};
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::Acquire;
use sqlx::Error as SqlxError;
use sqlx::{PgPool, Postgres};
use std::collections::HashSet;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::LazyLock;
use std::time::Duration;
use time::{Duration as TimeDuration, PrimitiveDateTime};
use tokio::sync::RwLock;
use tracing::error;
use tracing::info;
use tracing::warn;

use crate::cmd::block_history::BlockHash;
use crate::cmd::block_history::BlockSummary;
use crate::contracts::AclContract::AclContractEvents;
use crate::contracts::BridgeContract::BridgeContractEvents;
use crate::contracts::TfheContract;
use crate::contracts::TfheContract::TfheContractEvents;

type FheOperation = i32;
pub type Handle = FixedBytes<32>;
pub type TransactionHash = FixedBytes<32>;
pub type ToType = u8;
pub type ScalarByte = FixedBytes<1>;
pub type ClearConst = Uint<256, 4>;
pub type ChainHash = TransactionHash;

static SLOW_LANE_MARKED_CHAINS_TOTAL: LazyLock<IntCounterVec> = LazyLock::new(
    || {
        register_int_counter_vec!(
            "host_listener_slow_lane_marked_chains_total",
            "Number of dependence chains marked slow by host-listener classification",
            &["chain_id"]
        )
        .expect("host-listener slow-lane metric must register")
    },
);

/// ACL events whose handle could not be resolved to a ciphertext-producer
/// block on the current branch (nor to branchless/legacy state). These are
/// keyed branchless rather than guessing the ACL-event block; a non-zero rate
/// indicates branch metadata is missing or inconsistent, and should alert.
static UNRESOLVED_PRODUCER_BLOCK_TOTAL: LazyLock<IntCounterVec> = LazyLock::new(
    || {
        register_int_counter_vec!(
        "host_listener_unresolved_producer_block_total",
        "ACL-event handles that could not be resolved to a ciphertext producer block and were keyed branchless",
        &["chain_id"]
    )
    .expect("host-listener unresolved-producer-block metric must register")
    },
);

#[derive(Clone, Debug)]
pub struct Chain {
    pub hash: ChainHash,
    pub dependencies: Vec<ChainHash>,
    // Ingest-only metadata for dependency links split by no_fork grouping.
    // Not used by scheduler execution ordering.
    pub split_dependencies: Vec<ChainHash>,
    pub dependents: Vec<ChainHash>,
    pub allowed_handle: Vec<Handle>,
    pub size: u64,
    pub before_size: u64,
    pub new_chain: bool,
}
pub type ChainCache = Arc<RwLock<lru::LruCache<Handle, ChainHash>>>;
pub type OrderedChains = Vec<Chain>;

const MINIMUM_BUCKET_CACHE_SIZE: u16 = 16;
/// Depth bound for the producer-resolution ancestry walk. Fork
/// disambiguation only matters above finality; anything deeper resolves via
/// finalization status. Far above any real finality lag.
const ANCESTRY_WALK_DEPTH: i64 = 1024;
/// Reorgs orphaning blocks closer than this to the branch activation height
/// keep their legacy rows (see the straddle guard in
/// `cleanup_orphaned_branch_state`). Generously above any real finality lag,
/// since reorgs cannot be deeper than finality.
const BRANCH_ACTIVATION_STRADDLE_WINDOW: i64 = 128;
/// Finalized `host_chain_blocks_valid` rows older than this many blocks
/// below the finalized head are eligible for pruning (when unreferenced).
const BLOCKS_VALID_RETENTION: i64 = 10_000;
/// Upper bound of rows removed per pruning pass; keeps each pass short.
const BLOCKS_VALID_PRUNE_BATCH: i64 = 1_000;
const SLOW_LANE_RESET_ADVISORY_LOCK_KEY_BASE: i64 = 1_907_000_000;
const SLOW_LANE_RESET_BATCH_SIZE: i64 = 5_000;
const MAX_RETRY_FOR_TRANSIENT_ERROR: usize = 20;
const MAX_RETRY_ON_UNKNOWN_ERROR: usize = 5;

// short wait in case the database had a short issue
const RECONNECTION_DELAY: Duration = Duration::from_millis(100);

struct ComputationBranchRow<'a> {
    chain_id: i64,
    output_handle: &'a [u8],
    dependencies: &'a [Vec<u8>],
    fhe_operation: i16,
    is_scalar: bool,
    dependence_chain_id: &'a [u8],
    transaction_id: Option<Vec<u8>>,
    is_allowed: bool,
    schedule_order: PrimitiveDateTime,
    producer_block: ProducerBlock,
}

async fn insert_computation_branch_row(
    tx: &mut Transaction<'_>,
    row: ComputationBranchRow<'_>,
) -> Result<bool, SqlxError> {
    let done = sqlx::query!(
        r#"
        INSERT INTO computations_branch (
            output_handle,
            dependencies,
            fhe_operation,
            is_scalar,
            dependence_chain_id,
            transaction_id,
            is_allowed,
            created_at,
            schedule_order,
            is_completed,
            host_chain_id,
            block_number,
            producer_block_hash
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), $8::timestamp, $9, $10, $11, $12)
        ON CONFLICT (output_handle, transaction_id, producer_block_hash) DO NOTHING
        "#,
        row.output_handle,
        row.dependencies,
        row.fhe_operation,
        row.is_scalar,
        row.dependence_chain_id,
        row.transaction_id,
        row.is_allowed,
        row.schedule_order,
        !row.is_allowed,
        row.chain_id,
        row.producer_block.number as i64,
        row.producer_block.hash,
    )
    .execute(tx.deref_mut())
    .await?;
    Ok(done.rows_affected() > 0)
}

async fn insert_pbs_computation_branch_row(
    tx: &mut Transaction<'_>,
    chain_id: i64,
    handle: &[u8],
    transaction_id: Option<Vec<u8>>,
    block_number: i64,
    block_hash: &[u8],
    producer_block_hash: &[u8],
) -> Result<bool, SqlxError> {
    let done = sqlx::query!(
        "INSERT INTO pbs_computations_branch(handle, transaction_id, host_chain_id, block_number, block_hash, producer_block_hash)
         VALUES($1, $2, $3, $4, $5, $6)
         ON CONFLICT DO NOTHING;",
        handle,
        transaction_id,
        chain_id,
        block_number,
        block_hash,
        producer_block_hash,
    )
    .execute(tx.deref_mut())
    .await?;
    Ok(done.rows_affected() > 0)
}

struct AllowedHandleBranchRow<'a> {
    chain_id: i64,
    handle: &'a [u8],
    account_address: &'a str,
    event_type: i16,
    transaction_id: Option<Vec<u8>>,
    producer_block: &'a ProducerBlock,
    acl_block_number: u64,
    acl_block_hash: &'a [u8],
}

struct AllowedHandleInsert<'a> {
    handle: Vec<u8>,
    account_address: String,
    event_type: AllowEvents,
    transaction_id: Option<Vec<u8>>,
    producer_block: &'a ProducerBlock,
    acl_block_number: u64,
    acl_block_hash: &'a [u8],
}

async fn insert_allowed_handle_branch_row(
    tx: &mut Transaction<'_>,
    row: AllowedHandleBranchRow<'_>,
) -> Result<bool, SqlxError> {
    let done = sqlx::query!(
        "INSERT INTO allowed_handles_branch(handle, account_address, event_type, transaction_id, host_chain_id, block_number, block_hash, producer_block_hash)
         VALUES($1, $2, $3, $4, $5, $6, $7, $8)
         ON CONFLICT DO NOTHING;",
        row.handle,
        row.account_address,
        row.event_type,
        row.transaction_id,
        row.chain_id,
        row.acl_block_number as i64,
        row.acl_block_hash,
        row.producer_block.hash,
    )
    .execute(tx.deref_mut())
    .await?;
    Ok(done.rows_affected() > 0)
}

#[derive(Clone, Debug)]
pub struct ProducerBlock {
    hash: Vec<u8>,
    number: u64,
}

impl ProducerBlock {
    pub fn new(hash: &[u8], number: u64) -> Self {
        Self {
            hash: hash.to_vec(),
            number,
        }
    }
}

type DbErrorCode = std::borrow::Cow<'static, str>;
const STATEMENT_CANCELLED: DbErrorCode = DbErrorCode::Borrowed("57014"); // SQLSTATE code for statement cancelled

fn apply_connection_options(options: PgConnectOptions) -> PgConnectOptions {
    options.options([
        // 120s: large-object (lowrite) writes of the KMS server key can take
        // 25s+ under DB contention; 10s was too tight and rolled back the
        // key-activation download every cycle.
        ("statement_timeout", "120000"), // 120 seconds
    ])
}

/// Same as [`apply_connection_options`] but additionally pins
/// `search_path = gcs,public` so every connection routes unqualified writes
/// to the `gcs` schema (with fallback to `public` for shared read-only
/// tables). Used by the host-listener in `--gcs-mode`.
fn apply_connection_options_gcs(options: PgConnectOptions) -> PgConnectOptions {
    options.options([
        ("statement_timeout", "120000"), // 120 seconds; see apply_connection_options
        (
            "search_path",
            fhevm_engine_common::database::GCS_SEARCH_PATH,
        ),
    ])
}

fn slow_lane_reset_advisory_lock_key(chain_id: ChainId) -> i64 {
    SLOW_LANE_RESET_ADVISORY_LOCK_KEY_BASE.saturating_add(chain_id.as_i64())
}

pub fn retry_on_sqlx_error(err: &SqlxError, retry_count: &mut usize) -> bool {
    let is_transient = match err {
        // Transient errors, lots of retries
        SqlxError::Io(_)
        | SqlxError::PoolTimedOut
        | SqlxError::PoolClosed
        | SqlxError::WorkerCrashed
        | SqlxError::Protocol(_) => true,
        SqlxError::Database(err) if err.code() == Some(STATEMENT_CANCELLED) => {
            true
        }
        // Unknown errors, some retries
        _ => false,
    };
    let will_retry = if is_transient {
        *retry_count < MAX_RETRY_FOR_TRANSIENT_ERROR
    } else {
        *retry_count < MAX_RETRY_ON_UNKNOWN_ERROR
    };
    *retry_count += 1;
    will_retry
}

// A pool of connection with some cached information and automatic reconnection
#[derive(Clone)]
pub struct Database {
    url: DatabaseURL,
    pub pool: Arc<RwLock<sqlx::Pool<Postgres>>>,
    pool_refresh_handle: Arc<RwLock<PoolRefreshHandle>>,
    pub chain_id: ChainId,
    pub dependence_chain: ChainCache,
    pub tick: HeartBeat,
    /// When true, every connection in this pool sets
    /// `search_path = gcs,public` so writes resolve to the GCS schema.
    gcs_mode: bool,
    /// Host-chain height at which wave-1 branch dual-writes activate
    /// (`FHEVM_BRANCH_ACTIVATION_BLOCK`, default 0 = from genesis). Below
    /// it, ingestion writes legacy state only and producers resolve as
    /// branchless. Set to a fleet-common height above the rolling upgrade's
    /// completion so branch-row keying is deterministic across operators
    /// that upgrade at different times; the wave-2 cutover
    /// (`FHEVM_BRANCH_CUTOVER_BLOCK`) must be >= this height.
    pub branch_activation_block: u64,
}

#[derive(Debug)]
pub struct LogTfhe {
    pub event: Log<TfheContractEvents>,
    pub transaction_hash: Option<TransactionHash>,
    pub is_allowed: bool,
    pub block_number: u64,
    pub block_hash: BlockHash,
    pub block_timestamp: PrimitiveDateTime,
    pub tx_depth_size: u64,
    pub dependence_chain: TransactionHash,
    // global index per block (not by tx)
    pub log_index: Option<u64>,
}

pub type Transaction<'l> = sqlx::Transaction<'l, Postgres>;

fn parse_branch_activation_block() -> u64 {
    const ENV_VAR: &str = "FHEVM_BRANCH_ACTIVATION_BLOCK";

    match std::env::var(ENV_VAR) {
        Ok(value) => match value.parse::<u64>() {
            Ok(block) => block,
            Err(err) => {
                error!(
                    env_var = ENV_VAR,
                    value = %value,
                    error = %err,
                    "Invalid branch activation block configuration"
                );
                std::process::exit(1);
            }
        },
        Err(std::env::VarError::NotPresent) => 0,
        Err(err) => {
            error!(
                env_var = ENV_VAR,
                error = %err,
                "Invalid branch activation block configuration"
            );
            std::process::exit(1);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatsForConsumer {
    pub number_of_new_gaps: i64,
    pub total_new_gap_size: i64,
    pub number_of_duplicated_inserts: i64,
}

impl Database {
    pub async fn new(
        url: &DatabaseURL,
        chain_id: ChainId,
        dependence_cache_size: u16,
    ) -> Result<Self> {
        Self::new_with_gcs_mode(url, chain_id, dependence_cache_size, false)
            .await
    }

    pub async fn new_with_gcs_mode(
        url: &DatabaseURL,
        chain_id: ChainId,
        dependence_cache_size: u16,
        gcs_mode: bool,
    ) -> Result<Self> {
        let (pool, pool_refresh_handle) = Self::new_pool(url, gcs_mode).await;
        let bucket_cache =
            Arc::new(tokio::sync::RwLock::new(lru::LruCache::new(
                std::num::NonZeroU16::new(
                    dependence_cache_size.max(MINIMUM_BUCKET_CACHE_SIZE),
                )
                .unwrap()
                .into(),
            )));
        let branch_activation_block = parse_branch_activation_block();
        let db = Database {
            url: url.clone(),
            chain_id,
            pool: Arc::new(RwLock::new(pool)),
            pool_refresh_handle: Arc::new(RwLock::new(pool_refresh_handle)),
            dependence_chain: bucket_cache,
            tick: HeartBeat::default(),
            gcs_mode,
            branch_activation_block,
        };
        // Wave-1 deploy safety: a binary may start before the branch-context
        // migration has applied (e.g. a rolling deploy where the db-migration
        // Job is still running). Wait for the branch schema rather than
        // crash-looping on `*_branch` / `parent_hash` writes. Returns instantly
        // once present, which is always the case after the runbook's
        // migrate-first step and in tests (migrations run before construction).
        db.wait_for_branch_schema().await?;
        Ok(db)
    }

    /// Block until the wave-1 branch-context schema is present (the `*_branch`
    /// tables, `coprocessor_settlement`, and `host_chain_blocks_valid.parent_hash`).
    /// This degrades a pre-migration start to a bounded wait instead of a
    /// crash-loop. Bounded so a genuinely-absent migration surfaces as an error
    /// (→ restart) rather than hanging forever.
    pub async fn wait_for_branch_schema(&self) -> Result<()> {
        const MAX_ATTEMPTS: u32 = 60; // ~2 minutes at one poll / 2s
        const POLL_INTERVAL: Duration = Duration::from_secs(2);
        for attempt in 0..MAX_ATTEMPTS {
            // Wait between polls, but not before the first one.
            if attempt > 0 {
                tokio::time::sleep(POLL_INTERVAL).await;
            }
            let pool = self.pool().await;
            let ready: bool = sqlx::query_scalar!(
                r#"
                SELECT (
                    to_regclass('public.computations_branch') IS NOT NULL
                    AND to_regclass('public.ciphertexts_branch') IS NOT NULL
                    AND to_regclass('public.coprocessor_settlement') IS NOT NULL
                    AND EXISTS (
                        SELECT 1
                        FROM information_schema.columns
                        WHERE table_name = 'host_chain_blocks_valid'
                          AND column_name = 'parent_hash'
                    )
                ) AS "ready!"
                "#,
            )
            .fetch_one(&pool)
            .await?;
            if ready {
                if attempt > 0 {
                    info!(
                        attempt,
                        "Branch-context schema present; proceeding."
                    );
                }
                return Ok(());
            }
            warn!(
                attempt,
                "Branch-context (wave1) schema not yet present; waiting before ingesting..."
            );
        }
        anyhow::bail!(
            "branch-context (wave1) schema not present after waiting; \
             is the db-migration job complete?"
        );
    }

    pub(crate) fn record_slow_lane_marked_chains(&self, count: u64) {
        if count > 0 {
            let chain_id_label = self.chain_id.to_string();
            SLOW_LANE_MARKED_CHAINS_TOTAL
                .with_label_values(&[chain_id_label.as_str()])
                .inc_by(count);
        }
    }

    pub async fn promote_all_dep_chains_to_fast_priority(
        &self,
    ) -> Result<u64, SqlxError> {
        let lock_key = slow_lane_reset_advisory_lock_key(self.chain_id);
        let mut connection = self.pool().await.acquire().await?;
        let mut tx = connection.begin().await?;
        let lock_acquired = sqlx::query_scalar!(
            r#"SELECT pg_try_advisory_xact_lock($1) AS "lock_acquired!""#,
            lock_key
        )
        .fetch_one(tx.as_mut())
        .await?;
        if !lock_acquired {
            info!("Slow-lane reset already in progress; skipping promotion attempt");
            tx.commit().await?;
            return Ok(0);
        }

        let mut total_promoted: u64 = 0;
        loop {
            let updated = sqlx::query!(
                r#"
                WITH candidate AS (
                    SELECT dependence_chain_id
                    FROM dependence_chain
                    WHERE schedule_priority <> $1
                    ORDER BY dependence_chain_id
                    LIMIT $2
                    FOR UPDATE SKIP LOCKED
                )
                UPDATE dependence_chain dc
                SET schedule_priority = $1
                FROM candidate
                WHERE dc.dependence_chain_id = candidate.dependence_chain_id
                "#,
                i16::from(SchedulePriority::Fast),
                SLOW_LANE_RESET_BATCH_SIZE,
            )
            .execute(tx.as_mut())
            .await?;
            let updated = updated.rows_affected();
            total_promoted = total_promoted.saturating_add(updated);
            if updated == 0 {
                break;
            }
        }

        tx.commit().await?;

        Ok(total_promoted)
    }

    pub async fn find_slow_dep_chain_ids(
        &self,
        tx: &mut Transaction<'_>,
        dep_chain_ids: &[Vec<u8>],
    ) -> Result<HashSet<ChainHash>, SqlxError> {
        if dep_chain_ids.is_empty() {
            return Ok(HashSet::new());
        }

        let rows = sqlx::query!(
            r#"
            SELECT dependence_chain_id
            FROM dependence_chain
            WHERE schedule_priority = $1
              AND dependence_chain_id = ANY($2::bytea[])
            "#,
            i16::from(SchedulePriority::Slow),
            dep_chain_ids as _
        )
        .fetch_all(tx.deref_mut())
        .await?;

        let mut slow_dep_chain_ids =
            HashSet::with_capacity(rows.len() + dep_chain_ids.len());
        for row in rows {
            let dep_chain_id = row.dependence_chain_id;
            if let Ok(dep_chain_bytes) =
                <[u8; 32]>::try_from(dep_chain_id.as_slice())
            {
                slow_dep_chain_ids.insert(ChainHash::from(dep_chain_bytes));
            }
        }
        Ok(slow_dep_chain_ids)
    }

    async fn new_pool(
        url: &DatabaseURL,
        gcs_mode: bool,
    ) -> (PgPool, PoolRefreshHandle) {
        let transform: fn(PgConnectOptions) -> PgConnectOptions = if gcs_mode {
            apply_connection_options_gcs
        } else {
            apply_connection_options
        };
        let connect = || {
            connect_pool_with_options_and_connect_options(
                url,
                PgPoolOptions::new()
                    .min_connections(2)
                    .max_lifetime(Duration::from_secs(10 * 60))
                    .max_connections(8)
                    .acquire_timeout(Duration::from_secs(5)),
                None,
                transform,
            )
        };
        let mut pool = connect().await;
        while let Err(err) = pool {
            error!(
                error = %err,
                "Database connection failed. Will retry indefinitely."
            );
            tokio::time::sleep(Duration::from_secs(5)).await;
            pool = connect().await;
        }
        pool.expect("unreachable")
    }

    /// Begin a write transaction fenced against cutover. Runs `assert_not_retired`
    /// at BEGIN (via `begin_guarded_pool`), then takes the shared cutover advisory
    /// lock and re-checks retirement. Returns `Ok(None)` if a committed cutover has
    /// retired this stack — the caller must skip the write. GCS-mode connections
    /// (`self.gcs_mode`) skip the gate: they write the gcs schema, not the cutover
    /// target. See `versioning::cutover_gate`.
    pub async fn new_transaction(
        &self,
    ) -> Result<Option<Transaction<'_>>, SqlxError> {
        let pool = self.pool().await;
        fhevm_engine_common::versioning::begin_write_guarded(
            &pool,
            self.gcs_mode,
        )
        .await
    }

    pub async fn pool(&self) -> sqlx::Pool<Postgres> {
        self.pool.read().await.clone()
    }

    pub async fn reconnect(&mut self) {
        tokio::time::sleep(RECONNECTION_DELAY).await;
        let (old_pool, old_refresh_handle) = {
            let (new_pool, new_refresh_handle) =
                Self::new_pool(&self.url, self.gcs_mode).await;
            let mut pool = self.pool.write().await;
            let mut pool_refresh_handle =
                self.pool_refresh_handle.write().await;
            let old_pool = std::mem::replace(&mut *pool, new_pool);
            let old_refresh_handle = std::mem::replace(
                &mut *pool_refresh_handle,
                new_refresh_handle,
            );
            (old_pool, old_refresh_handle)
        };
        // doing the close outside out of lock
        old_pool.close().await;
        drop(old_refresh_handle);
    }

    #[allow(clippy::too_many_arguments)]
    async fn insert_computation_bytes(
        &self,
        tx: &mut Transaction<'_>,
        result: &Handle,
        dependencies_handles: &[&Handle],
        dependencies_bytes: &[Vec<u8>], /* always added after
                                         * dependencies_handles */
        fhe_operation: FheOperation,
        scalar_byte: &FixedBytes<1>,
        log: &LogTfhe,
    ) -> Result<bool, SqlxError> {
        let dependencies_handles = dependencies_handles
            .iter()
            .map(|d| d.to_vec())
            .collect::<Vec<_>>();
        let dependencies = [&dependencies_handles, dependencies_bytes].concat();
        self.insert_computation_inner(
            tx,
            result,
            dependencies,
            fhe_operation,
            scalar_byte,
            log,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn insert_computation(
        &self,
        tx: &mut Transaction<'_>,
        result: &Handle,
        dependencies: &[&Handle],
        fhe_operation: FheOperation,
        scalar_byte: &FixedBytes<1>,
        log: &LogTfhe,
    ) -> Result<bool, SqlxError> {
        let dependencies =
            dependencies.iter().map(|d| d.to_vec()).collect::<Vec<_>>();
        self.insert_computation_inner(
            tx,
            result,
            dependencies,
            fhe_operation,
            scalar_byte,
            log,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn insert_computation_inner(
        &self,
        tx: &mut Transaction<'_>,
        result: &Handle,
        dependencies: Vec<Vec<u8>>,
        fhe_operation: FheOperation,
        scalar_byte: &FixedBytes<1>,
        log: &LogTfhe,
    ) -> Result<bool, SqlxError> {
        let is_scalar = !scalar_byte.is_zero();
        let output_handle = result.to_vec();
        // Below the fleet-common activation height only legacy state is
        // written: per-node dual-write start times would otherwise key branch
        // rows divergently across operators during a rolling upgrade.
        if log.block_number < self.branch_activation_block {
            return self
                .insert_computation_legacy_row(
                    tx,
                    &output_handle,
                    &dependencies,
                    fhe_operation,
                    is_scalar,
                    log,
                )
                .await;
        }
        // Wave-1 producer writes require the branch row so wave-2 consumers have
        // complete block-scoped state; branch write failures abort this ingest
        // transaction and are retried with the rest of the event.
        let inserted = insert_computation_branch_row(
            tx,
            ComputationBranchRow {
                chain_id: self.chain_id.as_i64(),
                output_handle: &output_handle,
                dependencies: &dependencies,
                fhe_operation: fhe_operation as i16,
                is_scalar,
                dependence_chain_id: log.dependence_chain.as_slice(),
                transaction_id: log.transaction_hash.map(|txh| txh.to_vec()),
                is_allowed: log.is_allowed,
                schedule_order: log.block_timestamp.saturating_add(
                    TimeDuration::microseconds(log.tx_depth_size as i64),
                ),
                producer_block: ProducerBlock {
                    hash: log.block_hash.to_vec(),
                    number: log.block_number,
                },
            },
        )
        .await?;
        // Wave-1 dual-write: the legacy pipeline still executes from the
        // legacy tables, so every branch row is mirrored there until the
        // block-scoped readers take over in wave 2.
        let legacy_inserted = self
            .insert_computation_legacy_row(
                tx,
                &output_handle,
                &dependencies,
                fhe_operation,
                is_scalar,
                log,
            )
            .await?;
        Ok(inserted || legacy_inserted)
    }

    async fn insert_computation_legacy_row(
        &self,
        tx: &mut Transaction<'_>,
        output_handle: &[u8],
        dependencies: &[Vec<u8>],
        fhe_operation: FheOperation,
        is_scalar: bool,
        log: &LogTfhe,
    ) -> Result<bool, SqlxError> {
        // Schema isolation handles BCS/GCS routing at the connection layer
        // (`search_path = gcs,public` for GCS, default `public` for BCS), so
        // this INSERT references `computations` unqualified.
        let query = sqlx::query!(
            r#"
            INSERT INTO computations (
                output_handle,
                dependencies,
                fhe_operation,
                is_scalar,
                dependence_chain_id,
                transaction_id,
                is_allowed,
                created_at,
                schedule_order,
                is_completed,
                host_chain_id,
                block_number
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), $8::timestamp, $9, $10, $11)
            ON CONFLICT (output_handle, transaction_id) DO NOTHING
            "#,
            output_handle,
            dependencies,
            fhe_operation as i16,
            is_scalar,
            log.dependence_chain.to_vec(),
            log.transaction_hash.map(|txh| txh.to_vec()),
            log.is_allowed,
            log.block_timestamp
                .saturating_add(TimeDuration::microseconds(
                    log.tx_depth_size as i64
                )),
            !log.is_allowed,
            self.chain_id.as_i64(),
            log.block_number as i64
        );
        query
            .execute(tx.deref_mut())
            .await
            .map(|result| result.rows_affected() > 0)
    }

    #[rustfmt::skip]
    #[tracing::instrument(name = "handle_tfhe_event", skip_all, fields(txn_id = tracing::field::Empty))]
    pub async fn insert_tfhe_event(
        &self,
        tx: &mut Transaction<'_>,
        log: &LogTfhe,
    ) -> Result<bool, SqlxError> {
        use TfheContract as C;
        use TfheContractEvents as E;
        const HAS_SCALAR : FixedBytes::<1> = FixedBytes([1]); // if any dependency is a scalar.
        const NO_SCALAR : FixedBytes::<1> = FixedBytes([0]); // if all dependencies are handles.
        // ciphertext type
        let event = &log.event;
        let ty = |to_type: &ToType| vec![*to_type];
        let as_bytes = |x: &ClearConst| x.to_be_bytes_vec();
        let fhe_operation = event_to_op_int(event);
        telemetry::record_short_hex_if_some(
            &tracing::Span::current(),
            "txn_id",
            log.transaction_hash.as_ref(),
        );
        let insert_computation = |tx, result, dependencies, scalar_byte| {
            self.insert_computation(tx, result, dependencies, fhe_operation, scalar_byte, log)
        };
        let insert_computation_bytes = |tx, result, dependencies_handles, dependencies_bytes, scalar_byte| {
            self.insert_computation_bytes(tx, result, dependencies_handles, dependencies_bytes, fhe_operation, scalar_byte, log)
        };

        // Record the transaction if this is a computation event
        if !matches!(
            &event.data,
            E::Initialized(_)
                |  E::Upgraded(_)
                |  E::VerifyInput(_)
        ) {
            self.record_transaction_begin(
                &log.transaction_hash.map(|h| h.to_vec()),
                log.block_number,
            ).await;
        };

        match &event.data {
            E::Cast(C::Cast {ct, toType, result, ..})
            => insert_computation_bytes(tx, result, &[ct], &[ty(toType)], &HAS_SCALAR).await,

            E::FheAdd(C::FheAdd {lhs, rhs, scalarByte, result, ..})
            | E::FheBitAnd(C::FheBitAnd {lhs, rhs, scalarByte, result, ..})
            | E::FheBitOr(C::FheBitOr {lhs, rhs, scalarByte, result, ..})
            | E::FheBitXor(C::FheBitXor {lhs, rhs, scalarByte, result, ..} )
            | E::FheDiv(C::FheDiv {lhs, rhs, scalarByte, result, ..})
            | E::FheMax(C::FheMax {lhs, rhs, scalarByte, result, ..})
            | E::FheMin(C::FheMin {lhs, rhs, scalarByte, result, ..})
            | E::FheMul(C::FheMul {lhs, rhs, scalarByte, result, ..})
            | E::FheRem(C::FheRem {lhs, rhs, scalarByte, result, ..})
            | E::FheRotl(C::FheRotl {lhs, rhs, scalarByte, result, ..})
            | E::FheRotr(C::FheRotr {lhs, rhs, scalarByte, result, ..})
            | E::FheShl(C::FheShl {lhs, rhs, scalarByte, result, ..})
            | E::FheShr(C::FheShr {lhs, rhs, scalarByte, result, ..})
            | E::FheSub(C::FheSub {lhs, rhs, scalarByte, result, ..})
            => insert_computation(tx, result, &[lhs, rhs], scalarByte).await,

            E::FheIfThenElse(C::FheIfThenElse {control, ifTrue, ifFalse, result, ..})
            => insert_computation(tx, result, &[control, ifTrue, ifFalse], &NO_SCALAR).await,

            | E::FheEq(C::FheEq {lhs, rhs, scalarByte, result, ..})
            | E::FheGe(C::FheGe {lhs, rhs, scalarByte, result, ..})
            | E::FheGt(C::FheGt {lhs, rhs, scalarByte, result, ..})
            | E::FheLe(C::FheLe {lhs, rhs, scalarByte, result, ..})
            | E::FheLt(C::FheLt {lhs, rhs, scalarByte, result, ..})
            | E::FheNe(C::FheNe {lhs, rhs, scalarByte, result, ..})
            => insert_computation(tx, result, &[lhs, rhs], scalarByte).await,


            E::FheNeg(C::FheNeg {ct, result, ..})
            | E::FheNot(C::FheNot {ct, result, ..})
            => insert_computation(tx, result, &[ct], &NO_SCALAR).await,

            | E::FheRand(C::FheRand {randType, seed, result, ..})
            => insert_computation_bytes(tx, result, &[], &[seed.to_vec(), ty(randType)], &HAS_SCALAR).await,

            | E::FheRandBounded(C::FheRandBounded {upperBound, randType, seed, result, ..})
            => insert_computation_bytes(tx, result, &[], &[seed.to_vec(), as_bytes(upperBound), ty(randType)], &HAS_SCALAR).await,

            | E::TrivialEncrypt(C::TrivialEncrypt {pt, toType, result, ..})
            => insert_computation_bytes(tx, result, &[], &[as_bytes(pt), ty(toType)], &HAS_SCALAR).await,

            E::FheSum(C::FheSum { values, result, .. }) => {
                let deps: Vec<&Handle> = values.iter().collect();
                insert_computation(tx, result, &deps, &NO_SCALAR).await
            }

            E::FheIsIn(C::FheIsIn { value, values, result, .. }) => {
                let mut deps: Vec<&Handle> = vec![value];
                deps.extend(values.iter());
                insert_computation(tx, result, &deps, &NO_SCALAR).await
            }

            E::FheMulDiv(C::FheMulDiv { factor1, factor2, divisor, scalarByte, result, .. }) => {
                if fhe_mul_div_factor2_is_scalar(scalarByte) {
                    insert_computation_bytes(tx, result, &[factor1], &[factor2.to_vec(), divisor.to_vec()], &HAS_SCALAR).await
                } else {
                    insert_computation_bytes(tx, result, &[factor1, factor2], &[divisor.to_vec()], &NO_SCALAR).await
                }
            }

            | E::Initialized(_)
            | E::Upgraded(_)
            | E::VerifyInput(_)
            => Ok(false),
        }
    }

    /// Finalizes `(block_number, block_hash)` and orphans its observed
    /// sibling forks. Returns `Ok(Some(orphaned_hashes))` on success and
    /// `Ok(None)` when finalization was REFUSED (row missing, already
    /// orphaned, or parent linkage contradicting the finalized predecessor);
    /// callers must stop their ascending batch on `None`.
    pub async fn update_block_as_finalized(
        &self,
        tx: &mut Transaction<'_>,
        block_number: i64,
        block_hash: &BlockHash,
    ) -> Result<Option<Vec<Vec<u8>>>, SqlxError> {
        // Finalization is destructive (orphaned siblings' state is deleted by
        // the caller), yet the hash comes from a single by-number RPC lookup.
        // Two defenses gate it: the (number, hash) row must have been
        // observed by ingestion (pre-existing), and its recorded parent hash
        // must not contradict the finalized predecessor — a stale or poisoned
        // RPC answering with a fork sibling fails the linkage check and the
        // block is retried on a later pass instead of orphaning the true
        // chain. Rows with the '' parent sentinel (recorded before parent
        // tracking, not yet repaired) pass vacuously: only positive evidence
        // of a mismatch blocks finalization.
        let finalized_result = sqlx::query!(
            r#"
            UPDATE host_chain_blocks_valid
            SET block_status = 'finalized'
            WHERE chain_id = $1
              AND block_hash = $2
              AND block_number = $3
              AND block_status <> 'orphaned'
              AND NOT EXISTS (
                  SELECT 1
                  FROM host_chain_blocks_valid prev
                  WHERE prev.chain_id = $1
                    AND prev.block_number = $3 - 1
                    AND prev.block_status = 'finalized'
                    AND host_chain_blocks_valid.parent_hash <> ''::BYTEA
                    AND host_chain_blocks_valid.parent_hash <> prev.block_hash
              )
            "#,
            self.chain_id.as_i64(),
            block_hash.to_vec(),
            block_number,
        )
        .execute(tx.deref_mut())
        .await?;

        if finalized_result.rows_affected() == 0 {
            tracing::warn!(
                chain_id = self.chain_id.as_i64(),
                block_number,
                block_hash = %to_hex(block_hash.as_slice()),
                "skipping finalization: block missing, already orphaned, or parent \
                 linkage contradicts the finalized predecessor"
            );
            // Distinguishable from "finalized, no siblings": the caller must
            // STOP its ascending batch here — the next height's linkage
            // check would pass vacuously (no finalized predecessor) and a
            // poisoned RPC could finalize a fork block right behind the
            // refusal.
            return Ok(None);
        }

        let orphaned_rows = sqlx::query!(
            r#"
            UPDATE host_chain_blocks_valid
            SET block_status = 'orphaned'
            WHERE block_number = $2
              AND chain_id = $1
              AND block_hash <> $3
            RETURNING block_hash
            "#,
            self.chain_id.as_i64(),
            block_number,
            block_hash.to_vec(),
        )
        .fetch_all(tx.deref_mut())
        .await?;

        let direct_orphaned_hashes = orphaned_rows
            .into_iter()
            .map(|row| row.block_hash)
            .collect::<Vec<_>>();
        if direct_orphaned_hashes.is_empty() {
            return Ok(Some(vec![]));
        }

        // Finalizing one block orphans every observed sibling branch at the
        // same height. Walk descendants from those direct orphan roots so the
        // whole observed fork is marked immediately. The recursion follows
        // child.parent_hash and is bounded by the blocks already recorded for
        // that fork, not by chain history. Doing this in the finalization
        // transaction keeps producer resolution and branch cleanup on one view
        // of canonicality.
        let orphaned_branch_rows = sqlx::query!(
            r#"
            WITH RECURSIVE orphaned_branch AS (
                SELECT block_hash
                FROM host_chain_blocks_valid
                WHERE chain_id = $1
                  AND block_hash = ANY($2::bytea[])
                UNION
                SELECT child.block_hash
                FROM host_chain_blocks_valid child
                JOIN orphaned_branch parent
                  ON child.parent_hash = parent.block_hash
                WHERE child.chain_id = $1
            )
            UPDATE host_chain_blocks_valid blocks
            SET block_status = 'orphaned'
            FROM orphaned_branch
            WHERE blocks.chain_id = $1
              AND blocks.block_hash = orphaned_branch.block_hash
            RETURNING blocks.block_hash
            "#,
            self.chain_id.as_i64(),
            direct_orphaned_hashes as _,
        )
        .fetch_all(tx.deref_mut())
        .await?;

        let mut orphaned_hashes = direct_orphaned_hashes;
        orphaned_hashes
            .extend(orphaned_branch_rows.into_iter().map(|row| row.block_hash));
        orphaned_hashes.sort();
        orphaned_hashes.dedup();

        Ok(Some(orphaned_hashes))
    }

    pub async fn cleanup_orphaned_branch_state(
        &self,
        tx: &mut Transaction<'_>,
        orphaned_block_hashes: &[Vec<u8>],
    ) -> Result<(), SqlxError> {
        if orphaned_block_hashes.is_empty() {
            return Ok(());
        }

        // This cleanup is part of the same finalization transaction that marks
        // blocks orphaned. Once host_chain_blocks_valid exposes an orphaned
        // branch, branch-scoped rows for that branch should no longer be
        // visible to readers. If the listener is down, finalization does not
        // advance; catchup reruns this idempotent cleanup when it later marks
        // the branch orphaned.
        //
        // Capture producer tuples whose ciphertext bytes were actually
        // produced on an orphaned branch. ACL/PBS rows can now be orphaned by
        // their event block while still referencing a canonical producer; those
        // should not delete the canonical ciphertext bytes. Rows that reference
        // an orphaned producer still drive byte cleanup, even when their own
        // event block is not in the orphaned set.
        let orphaned_ciphertext_pairs = sqlx::query!(
            r#"
            SELECT handle AS "handle!", producer_block_hash AS "producer_block_hash!"
             FROM (
                SELECT output_handle AS handle, producer_block_hash
                FROM computations_branch
                WHERE host_chain_id = $1
                  AND producer_block_hash = ANY($2::bytea[])
                UNION
                SELECT handle, producer_block_hash
                FROM pbs_computations_branch
                WHERE host_chain_id = $1
                  AND producer_block_hash = ANY($2::bytea[])
                UNION
                SELECT handle, producer_block_hash
                FROM ciphertext_digest_branch
                WHERE host_chain_id = $1
                  AND producer_block_hash = ANY($2::bytea[])
             ) orphaned_pairs
            "#,
            self.chain_id.as_i64(),
            orphaned_block_hashes as _,
        )
        .fetch_all(tx.deref_mut())
        .await?;

        let orphaned_ciphertext_handles: Vec<Vec<u8>> =
            orphaned_ciphertext_pairs
                .iter()
                .map(|row| row.handle.clone())
                .collect();
        let orphaned_ciphertext_hashes: Vec<Vec<u8>> =
            orphaned_ciphertext_pairs
                .iter()
                .map(|row| row.producer_block_hash.clone())
                .collect();

        let orphaned_legacy_computation_handles = sqlx::query!(
            r#"
            SELECT DISTINCT output_handle AS "handle!"
             FROM computations_branch
             WHERE host_chain_id = $1
               AND producer_block_hash = ANY($2::bytea[])
            "#,
            self.chain_id.as_i64(),
            orphaned_block_hashes as _,
        )
        .fetch_all(tx.deref_mut())
        .await?
        .into_iter()
        .map(|row| row.handle)
        .collect::<Vec<_>>();

        let orphaned_legacy_pbs_handles = sqlx::query!(
            r#"
            SELECT DISTINCT handle AS "handle!"
             FROM pbs_computations_branch
             WHERE host_chain_id = $1
               AND (
                    block_hash = ANY($2::bytea[])
                    OR producer_block_hash = ANY($2::bytea[])
               )
            "#,
            self.chain_id.as_i64(),
            orphaned_block_hashes as _,
        )
        .fetch_all(tx.deref_mut())
        .await?
        .into_iter()
        .map(|row| row.handle)
        .collect::<Vec<_>>();

        let orphaned_allowed_rows = sqlx::query!(
            r#"
            SELECT DISTINCT handle AS "handle!", account_address AS "account_address!", event_type AS "event_type!"
             FROM allowed_handles_branch
             WHERE host_chain_id = $1
               AND (
                    block_hash = ANY($2::bytea[])
                    OR producer_block_hash = ANY($2::bytea[])
               )
            "#,
            self.chain_id.as_i64(),
            orphaned_block_hashes as _,
        )
        .fetch_all(tx.deref_mut())
        .await?;
        let orphaned_allowed_handles = orphaned_allowed_rows
            .iter()
            .map(|row| row.handle.clone())
            .collect::<Vec<_>>();
        let orphaned_allowed_accounts = orphaned_allowed_rows
            .iter()
            .map(|row| row.account_address.clone())
            .collect::<Vec<_>>();
        let orphaned_allowed_event_types = orphaned_allowed_rows
            .iter()
            .map(|row| row.event_type)
            .collect::<Vec<_>>();

        if !orphaned_ciphertext_pairs.is_empty() {
            // This removes only DB branch state and materialized DB bytes. S3
            // objects are not branch-addressed in the wave-1 path, so orphaned
            // objects are left as harmless garbage and no longer selected once
            // their branch rows are removed.
            sqlx::query!(
                r#"
                DELETE FROM ciphertexts_branch
                WHERE (handle, producer_block_hash) IN (
                    SELECT * FROM UNNEST($1::bytea[], $2::bytea[])
                )
                "#,
                &orphaned_ciphertext_handles as _,
                &orphaned_ciphertext_hashes as _,
            )
            .execute(tx.deref_mut())
            .await?;

            sqlx::query!(
                r#"
                DELETE FROM ciphertexts128_branch
                WHERE (handle, producer_block_hash) IN (
                    SELECT * FROM UNNEST($1::bytea[], $2::bytea[])
                )
                "#,
                &orphaned_ciphertext_handles as _,
                &orphaned_ciphertext_hashes as _,
            )
            .execute(tx.deref_mut())
            .await?;
        }

        sqlx::query!(
            r#"
            DELETE FROM delegate_user_decrypt
            WHERE host_chain_id = $1
              AND block_hash = ANY($2::bytea[])
            "#,
            self.chain_id.as_i64(),
            orphaned_block_hashes as _,
        )
        .execute(tx.deref_mut())
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM ciphertext_digest_branch
             WHERE host_chain_id = $1
               AND (
                    block_hash = ANY($2::bytea[])
                    OR producer_block_hash = ANY($2::bytea[])
               )
            "#,
            self.chain_id.as_i64(),
            orphaned_block_hashes as _,
        )
        .execute(tx.deref_mut())
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM allowed_handles_branch
             WHERE host_chain_id = $1
               AND (
                    block_hash = ANY($2::bytea[])
                    OR producer_block_hash = ANY($2::bytea[])
               )
            "#,
            self.chain_id.as_i64(),
            orphaned_block_hashes as _,
        )
        .execute(tx.deref_mut())
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM pbs_computations_branch
             WHERE host_chain_id = $1
               AND (
                    block_hash = ANY($2::bytea[])
                    OR producer_block_hash = ANY($2::bytea[])
               )
            "#,
            self.chain_id.as_i64(),
            orphaned_block_hashes as _,
        )
        .execute(tx.deref_mut())
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM computations_branch
             WHERE host_chain_id = $1
               AND producer_block_hash = ANY($2::bytea[])
            "#,
            self.chain_id.as_i64(),
            orphaned_block_hashes as _,
        )
        .execute(tx.deref_mut())
        .await?;

        // Activation-boundary straddle guard: a transaction included on a
        // fork block >= FHEVM_BRANCH_ACTIVATION_BLOCK writes branch rows,
        // while its canonical re-inclusion BELOW the activation height writes
        // legacy-only. Orphaning the fork then removes the handle's only
        // branch context, and the NOT EXISTS guards below would delete the
        // legacy rows the canonical below-activation inclusion depends on.
        // For reorgs near the boundary, skip the legacy deletions entirely —
        // a small, bounded residue of dead legacy rows (same accepted class
        // as the retained ciphertext bytes) instead of permanent loss.
        let near_activation_boundary = self.branch_activation_block > 0
            && sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(
                     SELECT 1 FROM host_chain_blocks_valid
                     WHERE chain_id = $1
                       AND block_hash = ANY($2::bytea[])
                       AND block_number < $3
                 )",
            )
            .bind(self.chain_id.as_i64())
            .bind(orphaned_block_hashes)
            .bind(
                (self.branch_activation_block as i64)
                    .saturating_add(BRANCH_ACTIVATION_STRADDLE_WINDOW),
            )
            .fetch_one(tx.deref_mut())
            .await?;
        if near_activation_boundary {
            tracing::warn!(
                chain_id = self.chain_id.as_i64(),
                activation = self.branch_activation_block,
                "Reorg near the branch activation boundary: keeping legacy rows"
            );
        }

        // Wave-1 dual-write: legacy readers are still live. Remove legacy
        // computation/ACL/PBS/digest rows only when no retained branch context
        // remains for the same logical work, i.e. only for handles that existed
        // solely on the now-orphaned fork (NOT EXISTS guard). Legacy ciphertext
        // bytes (`ciphertexts`/`ciphertexts128`) are intentionally NOT deleted
        // here.
        if near_activation_boundary {
            // Legacy deletions skipped (see the straddle guard above).
        } else if !orphaned_legacy_computation_handles.is_empty() {
            sqlx::query!(
                r#"
                DELETE FROM computations c
                WHERE c.host_chain_id = $1
                  AND c.output_handle = ANY($2::bytea[])
                  AND NOT EXISTS (
                      SELECT 1
                      FROM computations_branch b
                      WHERE b.host_chain_id = c.host_chain_id
                        AND b.output_handle = c.output_handle
                  )
                "#,
                self.chain_id.as_i64(),
                &orphaned_legacy_computation_handles as _,
            )
            .execute(tx.deref_mut())
            .await?;
        }

        if !near_activation_boundary && !orphaned_legacy_pbs_handles.is_empty()
        {
            // Ordering contract with sns-worker: pbs_computations rows (an
            // sns provenance witness) are deleted BEFORE ciphertext_digest
            // rows. The sns batch transaction holds FOR UPDATE locks on the
            // pbs rows it is processing (work acquisition) and
            // enqueue_upload_task takes FOR KEY SHARE on the pbs row before
            // inserting a digest row, so this DELETE orders against any
            // in-flight digest insert: either it waits and the digest DELETE
            // below removes the just-committed row, or — because the digest
            // mirror triggers take advisory stripe locks in the opposite
            // order on both sides — Postgres resolves the collision as a
            // deadlock and aborts one transaction whole. Both outcomes
            // converge: an aborted finalization pass re-runs from scratch
            // (nothing was committed, blocks stay pending), an aborted sns
            // batch is re-fetched, and a later witness read sees the
            // committed deletion and skips. With the opposite delete order, a
            // digest row inserted by a concurrent sns transaction would be
            // silently resurrected after this cleanup commits and drive a
            // phantom addCiphertextMaterial publication.
            sqlx::query!(
                r#"
                DELETE FROM pbs_computations p
                WHERE p.host_chain_id = $1
                  AND p.handle = ANY($2::bytea[])
                  AND NOT EXISTS (
                      SELECT 1
                      FROM pbs_computations_branch b
                      WHERE b.host_chain_id = p.host_chain_id
                        AND b.handle = p.handle
                  )
                "#,
                self.chain_id.as_i64(),
                &orphaned_legacy_pbs_handles as _,
            )
            .execute(tx.deref_mut())
            .await?;

            sqlx::query!(
                r#"
                DELETE FROM ciphertext_digest d
                WHERE d.host_chain_id = $1
                  AND d.handle = ANY($2::bytea[])
                  AND NOT EXISTS (
                      SELECT 1
                      FROM pbs_computations_branch p
                      WHERE p.host_chain_id = d.host_chain_id
                        AND p.handle = d.handle
                  )
                "#,
                self.chain_id.as_i64(),
                &orphaned_legacy_pbs_handles as _,
            )
            .execute(tx.deref_mut())
            .await?;
        }

        if !near_activation_boundary && !orphaned_allowed_rows.is_empty() {
            sqlx::query!(
                r#"
                DELETE FROM allowed_handles a
                USING (
                    SELECT *
                    FROM UNNEST($2::bytea[], $3::text[], $4::int2[])
                        AS row(handle, account_address, event_type)
                ) orphaned
                WHERE a.host_chain_id = $1
                  AND a.handle = orphaned.handle
                  AND a.account_address = orphaned.account_address
                  AND a.event_type = orphaned.event_type
                  AND NOT EXISTS (
                      SELECT 1
                      FROM allowed_handles_branch b
                      WHERE b.host_chain_id = a.host_chain_id
                        AND b.handle = a.handle
                        AND b.account_address = a.account_address
                        AND b.event_type = a.event_type
                  )
                "#,
                self.chain_id.as_i64(),
                &orphaned_allowed_handles as _,
                &orphaned_allowed_accounts as _,
                &orphaned_allowed_event_types as _,
            )
            .execute(tx.deref_mut())
            .await?;
        }

        // Sweep ciphertexts_branch / ciphertexts128_branch again to catch any
        // rows inserted by in-flight workers that raced finalization while
        // producer-row deletes held locks.
        if !orphaned_ciphertext_pairs.is_empty() {
            sqlx::query!(
                r#"
                DELETE FROM ciphertexts_branch
                WHERE (handle, producer_block_hash) IN (
                    SELECT * FROM UNNEST($1::bytea[], $2::bytea[])
                )
                "#,
                &orphaned_ciphertext_handles as _,
                &orphaned_ciphertext_hashes as _,
            )
            .execute(tx.deref_mut())
            .await?;

            sqlx::query!(
                r#"
                DELETE FROM ciphertexts128_branch
                WHERE (handle, producer_block_hash) IN (
                    SELECT * FROM UNNEST($1::bytea[], $2::bytea[])
                )
                "#,
                &orphaned_ciphertext_handles as _,
                &orphaned_ciphertext_hashes as _,
            )
            .execute(tx.deref_mut())
            .await?;
        }

        // Confidential bridge: destination-side reorg retraction. The bridge
        // worker sets `is_associated` in the same transaction as the
        // ciphertext copy, so a flagged observation in an orphaned block is
        // the provenance of a materialization that must be retracted — or
        // re-attributed to a surviving sibling observation, see below (a
        // fallback-materialized handle is never flagged and its state is
        // compute-pipeline state, cleaned above). Deleting the copied
        // `ciphertext_digest` row cancels a not-yet-sent
        // `addCiphertextMaterial`; an already-sent one cannot be recalled and
        // re-association after canonical re-inclusion re-sends it, which the
        // contract's `CoprocessorAlreadyAdded` path treats as benign. The
        // single DELETE .. RETURNING both removes the orphaned observations
        // and captures the flag under the row lock, so an association racing
        // finalization is either retracted or never happens.
        let retracted_bridged = sqlx::query!(
            r#"
            DELETE FROM handle_bridged_events
            WHERE dst_chain_id = $1
              AND block_hash = ANY($2::bytea[])
            RETURNING dst_handle AS "dst_handle!", is_associated AS "is_associated!"
            "#,
            self.chain_id.as_i64(),
            orphaned_block_hashes as _,
        )
        .fetch_all(tx.deref_mut())
        .await?;

        let retracted_dst_handles = retracted_bridged
            .into_iter()
            .filter(|row| row.is_associated)
            .map(|row| row.dst_handle)
            .collect::<Vec<_>>();

        if !retracted_dst_handles.is_empty() {
            // The same destination handle can be observed in several blocks
            // (the HandleBridged event re-included on competing forks). When a
            // surviving non-orphaned observation exists, the materialized copy
            // is still canonically justified — deleting it would tear the
            // ciphertext out from under destination-chain readers until
            // re-association. Transfer the association flag to one surviving
            // observation instead (keeping the retraction contract: the copy
            // is always attributed to a live observation, so a later orphaning
            // of that block retracts it properly). Only handles with no
            // surviving observation lose their copy.
            let transferred = sqlx::query!(
                r#"
                UPDATE handle_bridged_events h
                SET is_associated = TRUE
                WHERE h.id IN (
                    SELECT DISTINCT ON (s.dst_handle) s.id
                    FROM handle_bridged_events s
                    WHERE s.dst_handle = ANY($1::bytea[])
                      AND s.dst_chain_id = $2
                      AND (
                            s.block_hash = ''::BYTEA
                            OR NOT EXISTS (
                                SELECT 1
                                FROM host_chain_blocks_valid b
                                WHERE b.chain_id = s.dst_chain_id
                                  AND b.block_hash = s.block_hash
                                  AND b.block_status = 'orphaned'
                            )
                      )
                    ORDER BY s.dst_handle, s.id
                )
                RETURNING h.dst_handle AS "dst_handle!"
                "#,
                &retracted_dst_handles as _,
                self.chain_id.as_i64(),
            )
            .fetch_all(tx.deref_mut())
            .await?;

            let transferred: std::collections::HashSet<Vec<u8>> =
                transferred.into_iter().map(|r| r.dst_handle).collect();
            let orphan_only_handles: Vec<Vec<u8>> = retracted_dst_handles
                .into_iter()
                .filter(|h| !transferred.contains(h))
                .collect();

            if !orphan_only_handles.is_empty() {
                sqlx::query!(
                    "DELETE FROM ciphertexts WHERE handle = ANY($1::bytea[])",
                    &orphan_only_handles as _,
                )
                .execute(tx.deref_mut())
                .await?;

                sqlx::query!(
                    "DELETE FROM ciphertext_digest
                     WHERE handle = ANY($1::bytea[]) AND host_chain_id = $2",
                    &orphan_only_handles as _,
                    self.chain_id.as_i64(),
                )
                .execute(tx.deref_mut())
                .await?;
            }
        }

        // Source-side approvals from orphaned blocks were never consumable
        // (their read path requires a finalized block); just remove them.
        sqlx::query!(
            r#"
            DELETE FROM bridge_handle_events
            WHERE src_chain_id = $1
              AND block_hash = ANY($2::bytea[])
            "#,
            self.chain_id.as_i64(),
            orphaned_block_hashes as _,
        )
        .execute(tx.deref_mut())
        .await?;

        // Fallback-grant observations from orphaned blocks: the grant's
        // synthetic computation rows are context-keyed and cleaned above; a
        // canonical re-inclusion carries (and re-synthesizes from) its own
        // observation row.
        sqlx::query!(
            r#"
            DELETE FROM fallback_granted_events
            WHERE dst_chain_id = $1
              AND block_hash = ANY($2::bytea[])
            "#,
            self.chain_id.as_i64(),
            orphaned_block_hashes as _,
        )
        .execute(tx.deref_mut())
        .await?;

        Ok(())
    }

    pub async fn mark_block_as_valid(
        &self,
        tx: &mut Transaction<'_>,
        block_summary: &BlockSummary,
        finalized: bool,
        fhe_event_count: i32,
        allow_event_count: i32,
    ) -> Result<(), SqlxError> {
        let status = if finalized { "finalized" } else { "pending" };
        // Insert with per-block event counts (written once at first insert and
        // not touched on later finalization transitions). On conflict, preserve
        // existing state but repair missing/stale ancestry so branch resolution
        // remains available after restarts.
        sqlx::query!(
            r#"
            INSERT INTO host_chain_blocks_valid
                (chain_id, block_hash, parent_hash, block_number, block_status,
                 fhe_event_count, allow_event_count)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (chain_id, block_hash) DO UPDATE
            -- A block hash immutably determines its parent, so a freshly
            -- observed non-empty parent_hash is authoritative: prefer it. This
            -- both fills a missing value and corrects a stale/wrong one, while
            -- never clobbering a known parent with a NULL/empty re-observation.
            SET parent_hash = COALESCE(NULLIF(EXCLUDED.parent_hash, ''::BYTEA), host_chain_blocks_valid.parent_hash);
            "#,
            self.chain_id.as_i64(),
            block_summary.hash.to_vec(),
            block_summary.parent_hash.to_vec(),
            block_summary.number as i64,
            status,
            fhe_event_count,
            allow_event_count,
        )
        .execute(tx.deref_mut())
        .await?;

        // 2. Finalize this block or orphan the competing observed branch, then
        // clean branch-scoped computations/ACL/PBS/digest/bytes for that
        // orphan branch in the same transaction. This path just recorded the
        // block it finalizes (ingestion of a finalized block), so a linkage
        // refusal (None) means the recorded row genuinely contradicts the
        // finalized predecessor: skip cleanup and leave it for the
        // finalization loop to sort out.
        if finalized {
            if let Some(orphaned_hashes) = self
                .update_block_as_finalized(
                    tx,
                    block_summary.number as i64,
                    &block_summary.hash,
                )
                .await?
            {
                self.cleanup_orphaned_branch_state(tx, &orphaned_hashes)
                    .await?;
            }
        }
        Ok(())
    }

    pub async fn mark_block_as_seen_by_consumer(
        &self,
        block_summary: &BlockSummary,
        catchup: bool,
    ) -> Result<f64, SqlxError> {
        let duplicate_count_increase = if catchup { 0 } else { 1 };
        let Some(mut tx) = self.new_transaction().await? else {
            return Ok(0.0);
        };
        let delay_seconds = sqlx::query_scalar!(
            r#"
            WITH upserted AS (
                INSERT INTO host_chain_consumer_blocks (
                    chain_id,
                    block_hash,
                    block_number
                )
                VALUES ($1, $2, $3)
                ON CONFLICT (chain_id, block_hash) DO UPDATE
                SET duplicate_count =
                    host_chain_consumer_blocks.duplicate_count + $4
                RETURNING chain_id, block_hash, created_at
            )
            SELECT
                COALESCE(
                    GREATEST(
                        EXTRACT(EPOCH FROM (c.created_at - v.created_at)),
                        0
                    ),
                    0
                )::DOUBLE PRECISION AS "delay_seconds!"
            FROM upserted c
            LEFT JOIN host_chain_blocks_valid v
              ON v.chain_id = c.chain_id
             AND v.block_hash = c.block_hash
            WHERE c.chain_id = $1
              AND c.block_hash = $2
            "#,
            self.chain_id.as_i64(),
            block_summary.hash.to_vec(),
            block_summary.number as i64,
            duplicate_count_increase
        )
        .fetch_one(tx.as_mut())
        .await?;
        tx.commit().await?;

        Ok(delay_seconds)
    }

    // /// Called at regular interval
    pub async fn detect_gap_seen_by_consumer(
        &self,
        finalization_margin: i64,
    ) -> Result<StatsForConsumer, SqlxError> {
        // This CTE marks rows `stats_processed` (a write), so gate it against
        // cutover. On a retired stack, report zero stats and touch nothing.
        let Some(mut tx) = self.new_transaction().await? else {
            return Ok(StatsForConsumer {
                number_of_new_gaps: 0,
                total_new_gap_size: 0,
                number_of_duplicated_inserts: 0,
            });
        };
        let row = sqlx::query!(
            r#"
            WITH last_block_number AS (
                SELECT MAX(block_number) AS last_block_number
                FROM host_chain_consumer_blocks
                WHERE chain_id = $1
            ),
            eligible_blocks AS (
                SELECT
                    c.block_number,
                    c.duplicate_count,
                    LAG(c.block_number) OVER (ORDER BY c.block_number) AS prev_block
                FROM host_chain_consumer_blocks c
                CROSS JOIN last_block_number l
                WHERE c.stats_processed = FALSE
                AND c.chain_id = $1
                AND c.block_number < l.last_block_number - $2::bigint
            ),
            gaps AS (
                SELECT
                    (block_number - prev_block - 1) AS gap_size
                FROM eligible_blocks
                WHERE prev_block IS NOT NULL
                AND block_number > prev_block + 1
            ),
            stats AS (
                SELECT
                    COALESCE((SELECT COUNT(*) FROM gaps), 0) AS number_of_new_gaps,
                    COALESCE((SELECT SUM(gap_size) FROM gaps), 0) AS total_new_gap_size,
                    COALESCE(SUM(duplicate_count), 0) AS number_of_duplicated_inserts
                FROM eligible_blocks
            ),
            mark_processed AS (
                UPDATE host_chain_consumer_blocks
                SET stats_processed = TRUE
                FROM last_block_number l
                WHERE stats_processed = FALSE
                AND chain_id = $1
                AND block_number < l.last_block_number - $2::bigint
            )
            SELECT
                COALESCE((SELECT number_of_new_gaps FROM stats), 0)::bigint AS "number_of_new_gaps!",
                COALESCE((SELECT total_new_gap_size FROM stats), 0)::bigint AS "total_new_gap_size!",
                COALESCE((SELECT number_of_duplicated_inserts FROM stats), 0)::bigint AS "number_of_duplicated_inserts!"
            "#,
            self.chain_id.as_i64(),
            finalization_margin,
        )
        .fetch_one(tx.as_mut())
        .await?;
        tx.commit().await?;

        Ok(StatsForConsumer {
            number_of_new_gaps: row.number_of_new_gaps,
            total_new_gap_size: row.total_new_gap_size,
            number_of_duplicated_inserts: row.number_of_duplicated_inserts,
        })
    }

    pub async fn get_finalized_blocks_number(
        tx: &mut Transaction<'_>,
        last_block_max: i64,
        chain_id: ChainId,
    ) -> Result<HashSet<i64>, SqlxError> {
        // Most of the time there is only 1 block pending. Under a backlog,
        // take the OLDEST pending blocks: finalization must progress
        // oldest-first so each block's parent-linkage check anchors on a
        // finalized predecessor instead of passing vacuously.
        let blocks_number = sqlx::query!(
            r#"
            SELECT block_number FROM host_chain_blocks_valid
            WHERE block_status = 'pending' AND block_number <= $1 AND chain_id = $2
            ORDER BY block_number ASC
            LIMIT 10
            "#,
            last_block_max,
            chain_id.as_i64(),
        )
        .fetch_all(tx.deref_mut())
        .await?;
        Ok(blocks_number
            .into_iter()
            .map(|record| record.block_number)
            .collect())
    }

    /// Prunes old, unreferenced, finalized `host_chain_blocks_valid` rows.
    ///
    /// The table records one row per observed block and nothing deleted it,
    /// so it grew without bound (and with it every ancestry probe). Rows are
    /// deleted only when they are (a) finalized, (b) older than
    /// [`BLOCKS_VALID_RETENTION`] blocks below the finalized head, and
    /// (c) referenced by NO branch, bridge or fallback state — so producer
    /// resolution (which matches old producers by finalization status),
    /// orphan guards (orphaned rows are never pruned) and bridge/fallback
    /// readiness checks are unaffected by construction. Most blocks carry no
    /// FHE activity, so in steady state nearly everything old is prunable.
    /// Batched to [`BLOCKS_VALID_PRUNE_BATCH`] rows per call.
    pub async fn prune_finalized_block_history(
        &self,
        last_finalized_block: i64,
    ) -> Result<u64, SqlxError> {
        let prune_below =
            last_finalized_block.saturating_sub(BLOCKS_VALID_RETENTION);
        if prune_below <= 0 {
            return Ok(0);
        }
        let pool = self.pool.read().await.clone();
        let deleted = sqlx::query!(
            r#"
            DELETE FROM host_chain_blocks_valid b
            WHERE b.ctid IN (
                SELECT c.ctid
                FROM host_chain_blocks_valid c
                WHERE c.chain_id = $1
                  AND c.block_status = 'finalized'
                  AND c.block_number < $2
                  AND NOT EXISTS (
                      SELECT 1 FROM computations_branch r
                      WHERE r.host_chain_id = $1
                        AND r.producer_block_hash = c.block_hash
                  )
                  AND NOT EXISTS (
                      SELECT 1 FROM pbs_computations_branch r
                      WHERE r.host_chain_id = $1
                        AND r.producer_block_hash = c.block_hash
                  )
                  AND NOT EXISTS (
                      SELECT 1 FROM pbs_computations_branch r
                      WHERE r.host_chain_id = $1 AND r.block_hash = c.block_hash
                  )
                  AND NOT EXISTS (
                      SELECT 1 FROM allowed_handles_branch r
                      WHERE r.host_chain_id = $1
                        AND r.producer_block_hash = c.block_hash
                  )
                  AND NOT EXISTS (
                      SELECT 1 FROM allowed_handles_branch r
                      WHERE r.host_chain_id = $1 AND r.block_hash = c.block_hash
                  )
                  AND NOT EXISTS (
                      SELECT 1 FROM ciphertext_digest_branch r
                      WHERE r.host_chain_id = $1
                        AND r.producer_block_hash = c.block_hash
                  )
                  AND NOT EXISTS (
                      SELECT 1 FROM ciphertext_digest_branch r
                      WHERE r.host_chain_id = $1 AND r.block_hash = c.block_hash
                  )
                  -- ciphertexts(128)_branch have no hash-leading index; probe
                  -- through their block_number partial indexes instead.
                  AND NOT EXISTS (
                      SELECT 1 FROM ciphertexts_branch r
                      WHERE r.block_number = c.block_number
                        AND r.producer_block_hash = c.block_hash
                  )
                  AND NOT EXISTS (
                      SELECT 1 FROM ciphertexts128_branch r
                      WHERE r.block_number = c.block_number
                        AND r.producer_block_hash = c.block_hash
                  )
                  AND NOT EXISTS (
                      SELECT 1 FROM bridge_handle_events r
                      WHERE r.block_hash = c.block_hash
                  )
                  AND NOT EXISTS (
                      SELECT 1 FROM handle_bridged_events r
                      WHERE r.block_hash = c.block_hash
                  )
                  AND NOT EXISTS (
                      SELECT 1 FROM fallback_granted_events r
                      WHERE r.block_hash = c.block_hash
                  )
                  -- KMS key/CRS activation staging joins the finalized row by
                  -- (chain_id, block_hash) when a 'ready' event activates;
                  -- pruning it would strand the activation forever. Small
                  -- tables, guarded unconditionally.
                  AND NOT EXISTS (
                      SELECT 1 FROM kms_key_activation_events r
                      WHERE r.chain_id = $1 AND r.block_hash = c.block_hash
                  )
                  AND NOT EXISTS (
                      SELECT 1 FROM kms_crs_activation_events r
                      WHERE r.chain_id = $1 AND r.block_hash = c.block_hash
                  )
                ORDER BY c.block_number ASC
                LIMIT $3
            )
            "#,
            self.chain_id.as_i64(),
            prune_below,
            BLOCKS_VALID_PRUNE_BATCH,
        )
        .execute(&pool)
        .await?
        .rows_affected();
        Ok(deleted)
    }

    pub async fn poller_get_last_caught_up_block(
        &self,
        chain_id: ChainId,
    ) -> Result<Option<i64>, SqlxError> {
        let pool = self.pool.read().await.clone();
        sqlx::query_scalar!(
            r#"
            SELECT last_caught_up_block AS "last_caught_up_block!"
            FROM host_listener_poller_state
            WHERE chain_id = $1
            "#,
            chain_id.as_i64(),
        )
        .fetch_optional(&pool)
        .await
    }

    pub async fn poller_set_last_caught_up_block(
        &self,
        chain_id: ChainId,
        block: i64,
    ) -> Result<(), SqlxError> {
        let Some(mut tx) = self.new_transaction().await? else {
            return Ok(());
        };
        sqlx::query(
            r#"
            INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block)
            VALUES ($1, $2)
            ON CONFLICT (chain_id) DO UPDATE
            SET last_caught_up_block = EXCLUDED.last_caught_up_block,
                updated_at = NOW()
            "#,
        )
        .bind(chain_id.as_i64())
        .bind(block)
        .execute(tx.as_mut())
        .await?;
        tx.commit().await?;

        Ok(())
    }

    pub async fn read_last_valid_block(&self) -> Option<i64> {
        let query = sqlx::query!(
            r#"
            SELECT MAX(block_number) FROM host_chain_blocks_valid WHERE chain_id = $1;
            "#,
            self.chain_id.as_i64(),
        );
        let pool = self.pool.read().await.clone();
        match query.fetch_one(&pool).await {
            Ok(record) => record.max,
            Err(_err) => None, // table could be empty
        }
    }

    async fn resolve_handle_producer_block(
        &self,
        tx: &mut Transaction<'_>,
        handle: &[u8],
        current_block_hash: &[u8],
        current_parent_hash: &[u8],
        current_block_number: u64,
    ) -> Result<ProducerBlock, SqlxError> {
        // Pre-activation blocks carry no branch rows by construction:
        // resolve as branchless without querying, deterministically across
        // operators regardless of when each node started dual-writing.
        if current_block_number < self.branch_activation_block {
            return Ok(ProducerBlock {
                hash: Vec::new(),
                number: current_block_number,
            });
        }
        // The recursive walk disambiguates producers among live forks, so it
        // only needs to cover the un-finalized region near the head; it is
        // depth-bounded (ANCESTRY_WALK_DEPTH blocks, far above any real
        // finality lag) so its cost no longer grows with recorded chain
        // history. Producers below that window sit on the finalized chain by
        // construction — exactly one non-orphaned row per height — and are
        // matched by finalization status directly. A pending block older than
        // the window (finalization stalled for >1024 blocks) would be
        // unresolvable here, but such a chain is already outside operating
        // limits.
        let producer_row = sqlx::query!(
            r#"
            WITH RECURSIVE ancestry(block_number, block_hash, parent_hash) AS (
                SELECT $2::BIGINT, $3::BYTEA, $4::BYTEA
                UNION ALL
                SELECT parent.block_number, parent.block_hash, parent.parent_hash
                FROM host_chain_blocks_valid parent
                JOIN ancestry child
                  ON parent.chain_id = $1
                 AND parent.block_hash = child.parent_hash
                 AND parent.block_number = child.block_number - 1
                WHERE child.block_number > GREATEST($2::BIGINT - $6::BIGINT, 0)
                  AND parent.block_status <> 'orphaned'
            )
            SELECT c.producer_block_hash, c.block_number
            FROM computations_branch c
            WHERE c.host_chain_id = $1
              AND c.output_handle = $5
              AND (
                    EXISTS (
                        SELECT 1
                        FROM ancestry a
                        WHERE c.producer_block_hash = a.block_hash
                          AND c.block_number IS NOT DISTINCT FROM a.block_number
                    )
                    OR EXISTS (
                        SELECT 1
                        FROM host_chain_blocks_valid f
                        WHERE f.chain_id = $1
                          AND f.block_hash = c.producer_block_hash
                          AND f.block_number = c.block_number
                          AND f.block_status = 'finalized'
                    )
              )
            ORDER BY c.block_number DESC
            LIMIT 1
            "#,
            self.chain_id.as_i64(),
            current_block_number as i64,
            current_block_hash,
            current_parent_hash,
            handle,
            ANCESTRY_WALK_DEPTH,
        )
        .fetch_optional(tx.deref_mut())
        .await?;

        if let Some(row) = producer_row {
            return Ok(ProducerBlock {
                hash: row.producer_block_hash,
                number: row.block_number.unwrap_or(0).max(0) as u64,
            });
        }

        let has_branchless_ciphertext = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM ciphertexts_branch
                WHERE handle = $1
                  AND producer_block_hash = ''::BYTEA
            ) AS "exists!"
            "#,
            handle,
        )
        .fetch_one(tx.deref_mut())
        .await?;
        if has_branchless_ciphertext {
            return Ok(ProducerBlock {
                hash: Vec::new(),
                number: current_block_number,
            });
        }

        let has_legacy_ciphertext = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM ciphertexts
                WHERE handle = $1
            ) AS "exists!"
            "#,
            handle,
        )
        .fetch_one(tx.deref_mut())
        .await?;
        if has_legacy_ciphertext {
            return Ok(ProducerBlock {
                hash: Vec::new(),
                number: current_block_number,
            });
        }

        let chain_id_label = self.chain_id.as_i64().to_string();
        UNRESOLVED_PRODUCER_BLOCK_TOTAL
            .with_label_values(&[chain_id_label.as_str()])
            .inc();
        error!(
            handle = %to_hex(handle),
            block_number = current_block_number,
            acl_block_hash = %to_hex(current_block_hash),
            "Could not resolve handle producer block for ACL event; keying branchless instead of the ACL block"
        );
        Ok(ProducerBlock {
            hash: Vec::new(),
            number: current_block_number,
        })
    }

    /// Handles all types of ACL events
    #[tracing::instrument(skip_all, fields(txn_id = tracing::field::Empty))]
    pub async fn handle_acl_event(
        &self,
        tx: &mut Transaction<'_>,
        event: &Log<AclContractEvents>,
        transaction_hash: &Option<Handle>,
        block_summary: &BlockSummary,
    ) -> Result<bool, SqlxError> {
        let data = &event.data;
        let block_number = block_summary.number;
        let block_hash = block_summary.hash.as_slice();
        let parent_hash = block_summary.parent_hash.as_slice();
        telemetry::record_short_hex_if_some(
            &tracing::Span::current(),
            "txn_id",
            transaction_hash.as_ref(),
        );

        let transaction_hash = transaction_hash.map(|h| h.to_vec());

        // Record only Allowed or AllowedForDecryption events
        if matches!(
            data,
            AclContractEvents::Allowed(_)
                | AclContractEvents::AllowedForDecryption(_)
                | AclContractEvents::DelegatedForUserDecryption(_)
                | AclContractEvents::RevokedDelegationForUserDecryption(_)
        ) {
            self.record_transaction_begin(&transaction_hash, block_number)
                .await;
        }
        let mut inserted = false;
        match data {
            AclContractEvents::Allowed(allowed) => {
                let handle = allowed.handle.to_vec();
                let producer_block = self
                    .resolve_handle_producer_block(
                        tx,
                        &handle,
                        block_hash,
                        parent_hash,
                        block_number,
                    )
                    .await?;

                inserted |= self
                    .insert_allowed_handle_resolved(
                        tx,
                        AllowedHandleInsert {
                            handle: handle.clone(),
                            account_address: allowed.account.to_string(),
                            event_type: AllowEvents::AllowedAccount,
                            transaction_id: transaction_hash.clone(),
                            producer_block: &producer_block,
                            acl_block_number: block_number,
                            acl_block_hash: block_hash,
                        },
                    )
                    .await?;

                inserted |= self
                    .insert_pbs_computations_resolved(
                        tx,
                        &[(handle, producer_block)],
                        transaction_hash,
                        block_number,
                        block_hash,
                    )
                    .await?;
            }
            AclContractEvents::AllowedForDecryption(allowed_for_decryption) => {
                let handles = allowed_for_decryption
                    .handlesList
                    .iter()
                    .map(|h| h.to_vec())
                    .collect::<Vec<_>>();
                let mut pbs_handles = Vec::with_capacity(handles.len());

                for handle in handles.clone() {
                    info!(
                        handle = to_hex(&handle),
                        "Allowed for public decryption"
                    );
                    let producer_block = self
                        .resolve_handle_producer_block(
                            tx,
                            &handle,
                            block_hash,
                            parent_hash,
                            block_number,
                        )
                        .await?;

                    inserted |= self
                        .insert_allowed_handle_resolved(
                            tx,
                            AllowedHandleInsert {
                                handle: handle.clone(),
                                account_address: "".to_string(),
                                event_type: AllowEvents::AllowedForDecryption,
                                transaction_id: transaction_hash.clone(),
                                producer_block: &producer_block,
                                acl_block_number: block_number,
                                acl_block_hash: block_hash,
                            },
                        )
                        .await?;
                    pbs_handles.push((handle, producer_block));
                }

                inserted |= self
                    .insert_pbs_computations_resolved(
                        tx,
                        &pbs_handles,
                        transaction_hash.clone(),
                        block_number,
                        block_hash,
                    )
                    .await?;
            }
            AclContractEvents::DelegatedForUserDecryption(delegation) => {
                info!(?delegation, "Delegation for user decryption");
                inserted |= Self::insert_delegation(
                    tx,
                    delegation.delegator,
                    delegation.delegate,
                    delegation.contractAddress,
                    delegation.delegationCounter,
                    delegation.oldExpirationDate,
                    delegation.newExpirationDate,
                    self.chain_id,
                    block_hash,
                    block_number,
                    transaction_hash.clone(),
                )
                .await?;
            }
            AclContractEvents::RevokedDelegationForUserDecryption(
                delegation,
            ) => {
                info!(?delegation, "Revoke delegation for user decryption");
                inserted |= Self::insert_delegation(
                    tx,
                    delegation.delegator,
                    delegation.delegate,
                    delegation.contractAddress,
                    delegation.delegationCounter,
                    delegation.oldExpirationDate,
                    0, // end the delegation
                    self.chain_id,
                    block_hash,
                    block_number,
                    transaction_hash.clone(),
                )
                .await?;
            }
            AclContractEvents::Initialized(initialized) => {
                warn!(event = ?initialized, "unhandled Acl::Initialized event");
            }
            AclContractEvents::OwnershipTransferStarted(
                ownership_transfer_started,
            ) => {
                warn!(
                    event = ?ownership_transfer_started,
                    "unhandled Acl::OwnershipTransferStarted event"
                );
            }
            AclContractEvents::OwnershipTransferred(ownership_transferred) => {
                warn!(
                    event = ?ownership_transferred,
                    "unhandled Acl::OwnershipTransferred event"
                );
            }
            AclContractEvents::Upgraded(upgraded) => {
                warn!(
                    event = ?upgraded,
                    "unhandled Acl::Upgraded event"
                );
            }
            AclContractEvents::Paused(paused) => {
                warn!(
                    event = ?paused,
                    "unhandled Acl::Paused event"
                );
            }
            AclContractEvents::Unpaused(unpaused) => {
                warn!(
                    event = ?unpaused,
                    "unhandled Acl::Unpaused event"
                );
            }
            AclContractEvents::BlockedAccount(blocked_account) => {
                warn!(
                    event = ?blocked_account,
                    "unhandled Acl::BlockedAccount event"
                );
            }
            AclContractEvents::UnblockedAccount(unblocked_account) => {
                warn!(
                    event = ?unblocked_account,
                    "unhandled Acl::UnblockedAccount event"
                );
            }
            AclContractEvents::DecryptionSignaturesInvalidated(
                decryption_signatures_invalidated,
            ) => {
                warn!(
                    event = ?decryption_signatures_invalidated,
                    "unhandled Acl::DecryptionSignaturesInvalidated event"
                );
            }
        }
        self.tick.update();
        Ok(inserted)
    }

    /// Handles confidential-bridge events (see RFC 008). Each event is recorded
    /// once; re-observation is a no-op (`ON CONFLICT DO NOTHING`).
    /// The block-number column is therefore first-seen.
    #[allow(clippy::too_many_arguments)]
    pub async fn handle_bridge_event(
        &self,
        tx: &mut Transaction<'_>,
        event: &Log<BridgeContractEvents>,
        transaction_hash: &Option<Handle>,
        block_number: u64,
        block_hash: &BlockHash,
        prev_block_hash: &BlockHash,
        block_timestamp: u64,
        acl_contract_address: &Option<Address>,
    ) -> Result<bool, SqlxError> {
        let transaction_id = transaction_hash.map(|h| h.to_vec());
        let inserted = match &event.data {
            BridgeContractEvents::BridgeHandle(e) => {
                // Trust anchor: the chain id embedded in srcHandle (bytes 22-29)
                // must match the chain that emitted the event. Ignore otherwise.
                let embedded = chain_id_from_handle(&e.srcHandle.0);
                if embedded != self.chain_id.as_u64() {
                    warn!(
                        src_handle = to_hex(e.srcHandle.as_slice()),
                        embedded_chain_id = embedded,
                        chain_id = %self.chain_id,
                        "Ignoring BridgeHandle: srcHandle chain id does not match source chain"
                    );
                    return Ok(false);
                }
                let Ok(dst_chain_id) = ChainId::try_from(e.dstChainId) else {
                    warn!(
                        src_handle = to_hex(e.srcHandle.as_slice()),
                        dst_chain_id = e.dstChainId,
                        "Ignoring BridgeHandle: dstChainId out of range"
                    );
                    return Ok(false);
                };
                info!(
                    src_handle = to_hex(e.srcHandle.as_slice()),
                    dst_chain_id = e.dstChainId,
                    "BridgeHandle event"
                );
                sqlx::query!(
                    "INSERT INTO bridge_handle_events
                        (src_handle, dst_chain_id, src_chain_id, sender_dapp,
                         guid, block_number, block_hash, transaction_id)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                     ON CONFLICT (src_handle, dst_chain_id, block_hash) DO NOTHING",
                    e.srcHandle.as_slice(),
                    dst_chain_id.as_i64(),
                    self.chain_id.as_i64(),
                    e.senderDapp.as_slice(),
                    e.guid.as_slice(),
                    block_number as i64,
                    block_hash.as_slice(),
                    transaction_id,
                )
                .execute(tx.deref_mut())
                .await?
                .rows_affected()
                    > 0
            }
            BridgeContractEvents::HandleBridged(e) => {
                // Verify the destination handle was correctly derived and ignore the
                // event otherwise.
                let Some(acl) = acl_contract_address else {
                    warn!("Cannot verify HandleBridged derivation: ACL address not configured");
                    return Ok(false);
                };
                let expected = derive_dst_handle(
                    &e.srcHandle.0,
                    &acl.into_array(),
                    self.chain_id.as_u64(),
                    &prev_block_hash.0,
                    block_timestamp,
                );
                if expected != e.dstHandle.0 {
                    error!(
                        src_handle = to_hex(e.srcHandle.as_slice()),
                        dst_handle = to_hex(e.dstHandle.as_slice()),
                        expected = to_hex(&expected),
                        "Ignoring HandleBridged: destination handle derivation check failed"
                    );
                    return Ok(false);
                }
                info!(
                    src_handle = to_hex(e.srcHandle.as_slice()),
                    dst_handle = to_hex(e.dstHandle.as_slice()),
                    "HandleBridged event"
                );
                sqlx::query!(
                    "INSERT INTO handle_bridged_events
                        (src_handle, dst_handle, dst_chain_id, receiver_dapp,
                         guid, block_number, block_hash, transaction_id)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                     ON CONFLICT (dst_handle, block_hash) DO NOTHING",
                    e.srcHandle.as_slice(),
                    e.dstHandle.as_slice(),
                    self.chain_id.as_i64(),
                    e.receiverDapp.as_slice(),
                    e.guid.as_slice(),
                    block_number as i64,
                    block_hash.as_slice(),
                    transaction_id,
                )
                .execute(tx.deref_mut())
                .await?
                .rows_affected()
                    > 0
            }
            // `FallbackGrantedPlaintext` is converted to a synthetic TrivialEncrypt
            // computation during ingest (see ingest.rs), so it is not handled here.
            // Other events the coprocessor does not consume.
            _ => false,
        };
        self.tick.update();
        Ok(inserted)
    }

    /// Adds handles to the pbs_computations table and alerts the SnS worker
    /// about new of PBS work.
    pub async fn insert_pbs_computations(
        &self,
        tx: &mut Transaction<'_>,
        handles: &[Vec<u8>],
        transaction_id: Option<Vec<u8>>,
        block_number: u64,
        producer_block_hash: &[u8],
    ) -> Result<bool, SqlxError> {
        let producer_block = ProducerBlock {
            hash: producer_block_hash.to_vec(),
            number: block_number,
        };
        let handles = handles
            .iter()
            .cloned()
            .map(|handle| (handle, producer_block.clone()))
            .collect::<Vec<_>>();
        self.insert_pbs_computations_resolved(
            tx,
            &handles,
            transaction_id,
            block_number,
            producer_block_hash,
        )
        .await
    }

    async fn insert_pbs_computations_resolved(
        &self,
        tx: &mut Transaction<'_>,
        handles: &[(Vec<u8>, ProducerBlock)],
        transaction_id: Option<Vec<u8>>,
        acl_block_number: u64,
        acl_block_hash: &[u8],
    ) -> Result<bool, SqlxError> {
        let mut inserted = false;
        for (handle, producer_block) in handles {
            // Below the activation height only legacy state is written (see
            // Database::branch_activation_block).
            if acl_block_number >= self.branch_activation_block {
                inserted |= insert_pbs_computation_branch_row(
                    tx,
                    self.chain_id.as_i64(),
                    handle,
                    transaction_id.clone(),
                    acl_block_number as i64,
                    acl_block_hash,
                    &producer_block.hash,
                )
                .await?;
            }
            // Wave-1 dual-write: keep feeding the legacy sns-worker until
            // wave 2 switches it to the branch tables.
            let query = sqlx::query!(
                "INSERT INTO pbs_computations(handle, transaction_id, host_chain_id, block_number) VALUES($1, $2, $3, $4)
                 ON CONFLICT DO NOTHING;",
                handle,
                transaction_id,
                self.chain_id.as_i64(),
                acl_block_number as i64,
            );
            inserted |=
                query.execute(tx.deref_mut()).await?.rows_affected() > 0;
        }
        Ok(inserted)
    }

    /// Returns whether a computation producing `output_handle` already exists.
    pub async fn computation_exists(
        &self,
        tx: &mut Transaction<'_>,
        output_handle: &[u8],
    ) -> Result<bool, SqlxError> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(
                   SELECT 1 FROM computations WHERE output_handle = $1
               ) AS "exists!""#,
            output_handle,
        )
        .fetch_one(tx.deref_mut())
        .await?;
        Ok(exists)
    }

    pub async fn ciphertext_exists(
        &self,
        tx: &mut Transaction<'_>,
        handle: &[u8],
    ) -> Result<bool, SqlxError> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(
                   SELECT 1 FROM ciphertexts WHERE handle = $1
               ) AS "exists!""#,
            handle,
        )
        .fetch_one(tx.deref_mut())
        .await?;
        Ok(exists)
    }

    /// Records a FallbackGrantedPlaintext observation durably, keyed by the
    /// observing block (mirrors handle_bridged_events). Observations are
    /// facts about what the chain emitted; whether a synthetic computation is
    /// created for them is decided separately by
    /// [`Self::fallback_grant_conflicts`].
    #[allow(clippy::too_many_arguments)]
    pub async fn record_fallback_grant_observation(
        &self,
        tx: &mut Transaction<'_>,
        dst_handle: &[u8],
        plaintext: &[u8],
        transaction_hash: &Option<Handle>,
        block_number: u64,
        block_hash: &[u8],
    ) -> Result<(), SqlxError> {
        sqlx::query!(
            r#"
            INSERT INTO fallback_granted_events
                (dst_chain_id, dst_handle, plaintext, block_number,
                 block_hash, transaction_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (dst_handle, block_hash) DO NOTHING
            "#,
            self.chain_id.as_i64(),
            dst_handle,
            plaintext,
            block_number as i64,
            block_hash,
            transaction_hash.as_ref().map(|h| h.to_vec()),
        )
        .execute(tx.deref_mut())
        .await?;
        Ok(())
    }

    /// Decides whether a FallbackGrantedPlaintext observation must be
    /// suppressed instead of synthesizing its TrivialEncrypt computation.
    ///
    /// Suppress when:
    /// 1. a computation exists for the handle from a DIFFERENT transaction —
    ///    an earlier, different grant (first-wins per the contract) or a real
    ///    computation owns the handle;
    /// 2. this exact (handle, transaction, block) context was already
    ///    synthesized — pure re-observation, the pipeline inserts would all
    ///    no-op;
    /// 3. the handle has a ciphertext but no computation at all — a bridge
    ///    association copied the real ciphertext (which writes no
    ///    computations row) and a real association always beats the fallback.
    ///
    /// Crucially, the SAME grant re-observed in a different block (fork
    /// sibling or canonical re-inclusion after a reorg) is NOT suppressed:
    /// it creates a branch computation row for its own context while the
    /// legacy insert no-ops (ON CONFLICT (output_handle, transaction_id)),
    /// so reorg cleanup of one fork can never erase the grant from the
    /// surviving fork.
    pub async fn fallback_grant_conflicts(
        &self,
        tx: &mut Transaction<'_>,
        dst_handle: &[u8],
        transaction_hash: &Option<Handle>,
        block_hash: &[u8],
    ) -> Result<bool, SqlxError> {
        let transaction_id = transaction_hash.as_ref().map(|h| h.to_vec());
        let conflicts = sqlx::query_scalar!(
            r#"
            SELECT (
                EXISTS(
                    SELECT 1 FROM computations
                    WHERE output_handle = $1
                      AND transaction_id IS DISTINCT FROM $2
                )
                OR EXISTS(
                    SELECT 1 FROM computations_branch
                    WHERE output_handle = $1
                      AND transaction_id IS NOT DISTINCT FROM $2
                      AND producer_block_hash = $3
                )
                OR (
                    NOT EXISTS(
                        SELECT 1 FROM computations WHERE output_handle = $1
                    )
                    AND EXISTS(
                        SELECT 1 FROM ciphertexts WHERE handle = $1
                    )
                )
            ) AS "conflicts!"
            "#,
            dst_handle,
            transaction_id as _,
            block_hash,
        )
        .fetch_one(tx.deref_mut())
        .await?;
        Ok(conflicts)
    }

    /// Add the handle to the allowed_handles table
    pub async fn insert_allowed_handle(
        &self,
        tx: &mut Transaction<'_>,
        handle: Vec<u8>,
        account_address: String,
        event_type: AllowEvents,
        transaction_id: Option<Vec<u8>>,
        producer_block: ProducerBlock,
    ) -> Result<bool, SqlxError> {
        self.insert_allowed_handle_resolved(
            tx,
            AllowedHandleInsert {
                handle,
                account_address,
                event_type,
                transaction_id,
                producer_block: &producer_block,
                acl_block_number: producer_block.number,
                acl_block_hash: &producer_block.hash,
            },
        )
        .await
    }

    async fn insert_allowed_handle_resolved(
        &self,
        tx: &mut Transaction<'_>,
        insert: AllowedHandleInsert<'_>,
    ) -> Result<bool, SqlxError> {
        // Below the activation height only legacy state is written (see
        // Database::branch_activation_block).
        let branch_inserted =
            if insert.acl_block_number >= self.branch_activation_block {
                insert_allowed_handle_branch_row(
                    tx,
                    AllowedHandleBranchRow {
                        chain_id: self.chain_id.as_i64(),
                        handle: &insert.handle,
                        account_address: &insert.account_address,
                        event_type: insert.event_type as i16,
                        transaction_id: insert.transaction_id.clone(),
                        producer_block: insert.producer_block,
                        acl_block_number: insert.acl_block_number,
                        acl_block_hash: insert.acl_block_hash,
                    },
                )
                .await?
            } else {
                false
            };
        // Wave-1 dual-write: keep feeding the legacy readers until wave 2
        // switches them to the branch tables.
        let query = sqlx::query!(
            "INSERT INTO allowed_handles(handle, account_address, event_type, transaction_id, host_chain_id, block_number) VALUES($1, $2, $3, $4, $5, $6)
                    ON CONFLICT DO NOTHING;",
            insert.handle,
            insert.account_address,
            insert.event_type as i16,
            insert.transaction_id,
            self.chain_id.as_i64(),
            insert.acl_block_number as i64
        );
        let legacy_inserted =
            query.execute(tx.deref_mut()).await?.rows_affected() > 0;
        Ok(branch_inserted || legacy_inserted)
    }

    async fn record_transaction_begin(
        &self,
        transaction_hash: &Option<Vec<u8>>,
        block_number: u64,
    ) {
        if let Some(txn_id) = transaction_hash {
            let pool = self.pool.read().await.clone();
            let _ = telemetry::try_begin_transaction(
                &pool,
                self.chain_id,
                txn_id.as_ref(),
                block_number,
            )
            .await;
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn insert_delegation(
        tx: &mut Transaction<'_>,
        delegator: Address,
        delegate: Address,
        contract_address: Address,
        delegation_counter: u64,
        old_expiration_date: u64,
        new_expiration_date: u64,
        chain_id: ChainId,
        block_hash: &[u8],
        block_number: u64,
        transaction_id: Option<Vec<u8>>,
    ) -> Result<bool, SqlxError> {
        // ON CONFLICT is done on Unique constraint
        let query = sqlx::query!(
            "INSERT INTO delegate_user_decrypt(
                delegator, delegate, contract_address, delegation_counter, old_expiration_date, new_expiration_date, host_chain_id, block_number, block_hash, transaction_id, on_gateway, reorg_out)
            VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, false, false)
            ON CONFLICT DO NOTHING",
            &delegator.into_array(),
            &delegate.into_array(),
            &contract_address.into_array(),
            delegation_counter as i64,
            BigDecimal::from(old_expiration_date),
            BigDecimal::from(new_expiration_date),
            chain_id.as_i64(),
            block_number as i64,
            block_hash,
            transaction_id
        );
        let inserted = query.execute(tx.deref_mut()).await?.rows_affected() > 0;
        Ok(inserted)
    }

    pub async fn block_notification(&mut self) -> Result<(), SqlxError> {
        let query = sqlx::query!("NOTIFY new_host_block",);
        query.execute(&self.pool().await).await?;
        Ok(())
    }

    pub async fn update_dependence_chain(
        &self,
        tx: &mut Transaction<'_>,
        chains: OrderedChains,
        block_timestamp: PrimitiveDateTime,
        block_summary: &BlockSummary,
        slow_dep_chain_ids: &HashSet<ChainHash>,
    ) -> Result<(), SqlxError> {
        for chain in chains {
            let schedule_priority = if slow_dep_chain_ids.contains(&chain.hash)
            {
                SchedulePriority::Slow
            } else {
                SchedulePriority::Fast
            };
            let last_updated_at = block_timestamp.saturating_add(
                TimeDuration::microseconds(chain.before_size as i64),
            );
            let dependents = chain
                .dependents
                .iter()
                .map(|h| h.to_vec())
                .collect::<Vec<_>>();
            sqlx::query!(
                r#"
                INSERT INTO dependence_chain(
                    dependence_chain_id,
                    status,
                    last_updated_at,
                    dependency_count,
                    dependents,
                    block_hash,
                    block_height,
                    schedule_priority
                ) VALUES (
                  $1, 'updated', $2::timestamp, $3, $4, $5, $6, $7
                )
                ON CONFLICT (dependence_chain_id) DO UPDATE
                SET status = 'updated',
                    last_updated_at = CASE
                        WHEN dependence_chain.status = 'processed' THEN EXCLUDED.last_updated_at
                        ELSE LEAST(dependence_chain.last_updated_at, EXCLUDED.last_updated_at)
                    END,
                    dependents = (
                        SELECT ARRAY(
                            SELECT DISTINCT d
                            FROM unnest(dependence_chain.dependents || EXCLUDED.dependents) AS d
                        )
                    )
                    ,
                    schedule_priority = GREATEST(
                        dependence_chain.schedule_priority,
                        EXCLUDED.schedule_priority
                    )
                "#,
                chain.hash.to_vec(),
                last_updated_at,
                chain.dependencies.len() as i64,
                &dependents,
                block_summary.hash.to_vec(),
                block_summary.number as i64,
                i16::from(schedule_priority),
            )
            .execute(tx.deref_mut())
            .await?;
        }
        Ok(())
    }
}

fn event_to_op_int(op: &TfheContractEvents) -> FheOperation {
    use SupportedFheOperations as O;
    use TfheContractEvents as E;
    match op {
        E::FheAdd(_) => O::FheAdd as i32,
        E::FheSub(_) => O::FheSub as i32,
        E::FheMul(_) => O::FheMul as i32,
        E::FheDiv(_) => O::FheDiv as i32,
        E::FheRem(_) => O::FheRem as i32,
        E::FheBitAnd(_) => O::FheBitAnd as i32,
        E::FheBitOr(_) => O::FheBitOr as i32,
        E::FheBitXor(_) => O::FheBitXor as i32,
        E::FheShl(_) => O::FheShl as i32,
        E::FheShr(_) => O::FheShr as i32,
        E::FheRotl(_) => O::FheRotl as i32,
        E::FheRotr(_) => O::FheRotr as i32,
        E::FheEq(_) => O::FheEq as i32,
        E::FheNe(_) => O::FheNe as i32,
        E::FheGe(_) => O::FheGe as i32,
        E::FheGt(_) => O::FheGt as i32,
        E::FheLe(_) => O::FheLe as i32,
        E::FheLt(_) => O::FheLt as i32,
        E::FheMin(_) => O::FheMin as i32,
        E::FheMax(_) => O::FheMax as i32,
        E::FheNeg(_) => O::FheNeg as i32,
        E::FheNot(_) => O::FheNot as i32,
        E::Cast(_) => O::FheCast as i32,
        E::TrivialEncrypt(_) => O::FheTrivialEncrypt as i32,
        E::FheIfThenElse(_) => O::FheIfThenElse as i32,
        E::FheRand(_) => O::FheRand as i32,
        E::FheRandBounded(_) => O::FheRandBounded as i32,
        E::FheSum(_) => O::FheSum as i32,
        E::FheIsIn(_) => O::FheIsIn as i32,
        E::FheMulDiv(_) => O::FheMulDiv as i32,
        // Not tfhe ops
        E::Initialized(_) | E::Upgraded(_) | E::VerifyInput(_) => -1,
    }
}

pub fn event_name(op: &TfheContractEvents) -> &'static str {
    use TfheContractEvents as E;
    match op {
        E::FheAdd(_) => "FheAdd",
        E::FheSub(_) => "FheSub",
        E::FheMul(_) => "FheMul",
        E::FheDiv(_) => "FheDiv",
        E::FheRem(_) => "FheRem",
        E::FheBitAnd(_) => "FheBitAnd",
        E::FheBitOr(_) => "FheBitOr",
        E::FheBitXor(_) => "FheBitXor",
        E::FheShl(_) => "FheShl",
        E::FheShr(_) => "FheShr",
        E::FheRotl(_) => "FheRotl",
        E::FheRotr(_) => "FheRotr",
        E::FheEq(_) => "FheEq",
        E::FheNe(_) => "FheNe",
        E::FheGe(_) => "FheGe",
        E::FheGt(_) => "FheGt",
        E::FheLe(_) => "FheLe",
        E::FheLt(_) => "FheLt",
        E::FheMin(_) => "FheMin",
        E::FheMax(_) => "FheMax",
        E::FheNeg(_) => "FheNeg",
        E::FheNot(_) => "FheNot",
        E::Cast(_) => "FheCast",
        E::TrivialEncrypt(_) => "FheTrivialEncrypt",
        E::FheIfThenElse(_) => "FheIfThenElse",
        E::FheRand(_) => "FheRand",
        E::FheRandBounded(_) => "FheRandBounded",
        E::FheSum(_) => "FheSum",
        E::FheIsIn(_) => "FheIsIn",
        E::FheMulDiv(_) => "FheMulDiv",
        E::Initialized(_) => "Initialized",
        E::Upgraded(_) => "Upgraded",
        E::VerifyInput(_) => "VerifyInput",
    }
}

pub fn tfhe_result_handle(op: &TfheContractEvents) -> Option<Handle> {
    use TfheContract as C;
    use TfheContractEvents as E;
    match op {
        E::Cast(C::Cast { result, .. })
        | E::FheAdd(C::FheAdd { result, .. })
        | E::FheBitAnd(C::FheBitAnd { result, .. })
        | E::FheBitOr(C::FheBitOr { result, .. })
        | E::FheBitXor(C::FheBitXor { result, .. })
        | E::FheDiv(C::FheDiv { result, .. })
        | E::FheMax(C::FheMax { result, .. })
        | E::FheMin(C::FheMin { result, .. })
        | E::FheMul(C::FheMul { result, .. })
        | E::FheRem(C::FheRem { result, .. })
        | E::FheRotl(C::FheRotl { result, .. })
        | E::FheRotr(C::FheRotr { result, .. })
        | E::FheShl(C::FheShl { result, .. })
        | E::FheShr(C::FheShr { result, .. })
        | E::FheSub(C::FheSub { result, .. })
        | E::FheIfThenElse(C::FheIfThenElse { result, .. })
        | E::FheEq(C::FheEq { result, .. })
        | E::FheGe(C::FheGe { result, .. })
        | E::FheGt(C::FheGt { result, .. })
        | E::FheLe(C::FheLe { result, .. })
        | E::FheLt(C::FheLt { result, .. })
        | E::FheNe(C::FheNe { result, .. })
        | E::FheNeg(C::FheNeg { result, .. })
        | E::FheNot(C::FheNot { result, .. })
        | E::FheRand(C::FheRand { result, .. })
        | E::FheRandBounded(C::FheRandBounded { result, .. })
        | E::TrivialEncrypt(C::TrivialEncrypt { result, .. })
        | E::FheSum(C::FheSum { result, .. })
        | E::FheIsIn(C::FheIsIn { result, .. })
        | E::FheMulDiv(C::FheMulDiv { result, .. }) => Some(*result),

        E::Initialized(_) | E::Upgraded(_) | E::VerifyInput(_) => None,
    }
}

pub fn acl_result_handles(event: &Log<AclContractEvents>) -> Vec<Handle> {
    let data = &event.data;
    match data {
        AclContractEvents::Allowed(allowed) => vec![allowed.handle],
        AclContractEvents::AllowedForDecryption(allowed_for_decryption) => {
            allowed_for_decryption.handlesList.clone()
        }
        AclContractEvents::Initialized(_)
        | AclContractEvents::DelegatedForUserDecryption(_)
        | AclContractEvents::RevokedDelegationForUserDecryption(_)
        | AclContractEvents::OwnershipTransferStarted(_)
        | AclContractEvents::OwnershipTransferred(_)
        | AclContractEvents::Upgraded(_)
        | AclContractEvents::Paused(_)
        | AclContractEvents::Unpaused(_)
        | AclContractEvents::BlockedAccount(_)
        | AclContractEvents::UnblockedAccount(_)
        | AclContractEvents::DecryptionSignaturesInvalidated(_) => vec![],
    }
}

pub fn tfhe_inputs_handle(op: &TfheContractEvents) -> Vec<Handle> {
    use TfheContract as C;
    use TfheContractEvents as E;
    match op {
        E::Cast(C::Cast { ct, .. })
        | E::FheNeg(C::FheNeg { ct, .. })
        | E::FheNot(C::FheNot { ct, .. }) => vec![*ct],

        E::FheAdd(C::FheAdd {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitAnd(C::FheBitAnd {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitOr(C::FheBitOr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitXor(C::FheBitXor {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheDiv(C::FheDiv {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMax(C::FheMax {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMin(C::FheMin {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMul(C::FheMul {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRem(C::FheRem {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRotl(C::FheRotl {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRotr(C::FheRotr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheShl(C::FheShl {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheShr(C::FheShr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheSub(C::FheSub {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheEq(C::FheEq {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheGe(C::FheGe {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheGt(C::FheGt {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheLe(C::FheLe {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheLt(C::FheLt {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheNe(C::FheNe {
            lhs,
            rhs,
            scalarByte,
            ..
        }) => {
            if scalarByte.const_is_zero() {
                vec![*lhs, *rhs]
            } else {
                vec![*lhs]
            }
        }

        E::FheIfThenElse(C::FheIfThenElse {
            control,
            ifTrue,
            ifFalse,
            ..
        }) => {
            vec![*control, *ifTrue, *ifFalse]
        }

        E::FheRand(_) | E::FheRandBounded(_) | E::TrivialEncrypt(_) => vec![],

        E::FheSum(C::FheSum { values, .. }) => values.clone(),

        E::FheIsIn(C::FheIsIn { value, values, .. }) => {
            let mut handles = vec![*value];
            handles.extend(values.iter().copied());
            handles
        }

        E::FheMulDiv(C::FheMulDiv {
            factor1,
            factor2,
            scalarByte,
            ..
        }) => {
            if fhe_mul_div_factor2_is_scalar(scalarByte) {
                vec![*factor1]
            } else {
                vec![*factor1, *factor2]
            }
        }

        E::Initialized(_) | E::Upgraded(_) | E::VerifyInput(_) => vec![],
    }
}

/// `fheMulDiv` `scalarByte` bit 1 — factor2 is a plaintext scalar (bit 0 is the
/// always-scalar divisor).
fn fhe_mul_div_factor2_is_scalar(scalar_byte: &ScalarByte) -> bool {
    scalar_byte.0[0] & 0b10 != 0
}
