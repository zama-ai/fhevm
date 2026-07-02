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
use fhevm_engine_common::types::AllowEvents;
use fhevm_engine_common::types::SchedulePriority;
use fhevm_engine_common::types::SupportedFheOperations;
use fhevm_engine_common::utils::DatabaseURL;
use fhevm_engine_common::utils::{to_hex, HeartBeat};
use prometheus::{register_int_counter_vec, IntCounterVec};
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
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
const SLOW_LANE_RESET_ADVISORY_LOCK_KEY_BASE: i64 = 1_907_000_000;
const SLOW_LANE_RESET_BATCH_SIZE: i64 = 5_000;
const MAX_RETRY_FOR_TRANSIENT_ERROR: usize = 20;
const MAX_RETRY_ON_UNKNOWN_ERROR: usize = 5;

// short wait in case the database had a short issue
const RECONNECTION_DELAY: Duration = Duration::from_millis(100);

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
}

#[derive(Debug)]
pub struct LogTfhe {
    pub event: Log<TfheContractEvents>,
    pub transaction_hash: Option<TransactionHash>,
    pub is_allowed: bool,
    pub block_number: u64,
    pub block_timestamp: PrimitiveDateTime,
    pub tx_depth_size: u64,
    pub dependence_chain: TransactionHash,
    // global index per block (not by tx)
    pub log_index: Option<u64>,
}

pub type Transaction<'l> = sqlx::Transaction<'l, Postgres>;

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
        Ok(Database {
            url: url.clone(),
            chain_id,
            pool: Arc::new(RwLock::new(pool)),
            pool_refresh_handle: Arc::new(RwLock::new(pool_refresh_handle)),
            dependence_chain: bucket_cache,
            tick: HeartBeat::default(),
            gcs_mode,
        })
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
        sqlx::query("SELECT pg_advisory_lock($1)")
            .bind(lock_key)
            .execute(connection.deref_mut())
            .await?;

        let rows = async {
            let mut total_promoted: u64 = 0;
            loop {
                let updated = sqlx::query(
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
                )
                .bind(i16::from(SchedulePriority::Fast))
                .bind(SLOW_LANE_RESET_BATCH_SIZE)
                .execute(connection.deref_mut())
                .await?
                .rows_affected();

                total_promoted = total_promoted.saturating_add(updated);
                if updated == 0 {
                    break;
                }
            }
            Ok(total_promoted)
        }
        .await;

        let unlock_res =
            sqlx::query_scalar::<_, bool>("SELECT pg_advisory_unlock($1)")
                .bind(lock_key)
                .fetch_one(connection.deref_mut())
                .await;
        if let Err(err) = unlock_res {
            warn!(error = %err, "Failed to release slow-lane reset advisory lock");
        }

        rows
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
    pub async fn new_transaction(&self) -> Result<Option<Transaction<'_>>, SqlxError> {
        let pool = self.pool().await;
        fhevm_engine_common::versioning::begin_write_guarded(&pool, self.gcs_mode).await
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
            &dependencies,
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

    pub async fn update_block_as_finalized(
        &self,
        tx: &mut Transaction<'_>,
        block_number: i64,
        block_hash: &BlockHash,
    ) -> Result<(), SqlxError> {
        sqlx::query!(
            r#"
            UPDATE host_chain_blocks_valid
            SET block_status = CASE
                WHEN block_hash = $2
                    THEN 'finalized'
                    ELSE 'orphaned'
                END
            WHERE block_number = $3 AND chain_id = $1
            "#,
            self.chain_id.as_i64(),
            block_hash.to_vec(),
            block_number,
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
        // 1. Insert if not exists (never overwrites existing row).
        //    Event counts are written at first insert and not touched on
        //    later finalization transitions.
        sqlx::query!(
            r#"
            INSERT INTO host_chain_blocks_valid
                (chain_id, block_hash, block_number, block_status,
                 fhe_event_count, allow_event_count)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (chain_id, block_hash) DO NOTHING;
            "#,
            self.chain_id.as_i64(),
            block_summary.hash.to_vec(),
            block_summary.number as i64,
            status,
            fhe_event_count,
            allow_event_count,
        )
        .execute(tx.deref_mut())
        .await?;

        // 2. Update to finalized or orphan if needed
        if finalized {
            self.update_block_as_finalized(
                tx,
                block_summary.number as i64,
                &block_summary.hash,
            )
            .await?;
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
        // most of the time there is only 1 block pending
        let blocks_number = sqlx::query!(
            r#"
            SELECT block_number FROM host_chain_blocks_valid
            WHERE block_status = 'pending' AND block_number <= $1 AND chain_id = $2
            ORDER BY block_number DESC
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

    pub async fn poller_get_last_caught_up_block(
        &self,
        chain_id: ChainId,
    ) -> Result<Option<i64>, SqlxError> {
        let pool = self.pool.read().await.clone();
        sqlx::query_scalar(
            r#"
            SELECT last_caught_up_block
            FROM host_listener_poller_state
            WHERE chain_id = $1
            "#,
        )
        .bind(chain_id.as_i64())
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

    /// Handles all types of ACL events
    #[tracing::instrument(skip_all, fields(txn_id = tracing::field::Empty))]
    pub async fn handle_acl_event(
        &self,
        tx: &mut Transaction<'_>,
        event: &Log<AclContractEvents>,
        transaction_hash: &Option<Handle>,
        chain_id: ChainId,
        block_hash: &[u8],
        block_number: u64,
    ) -> Result<bool, SqlxError> {
        let data = &event.data;
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

                inserted |= self
                    .insert_allowed_handle(
                        tx,
                        handle.clone(),
                        allowed.account.to_string(),
                        AllowEvents::AllowedAccount,
                        transaction_hash.clone(),
                        block_number,
                    )
                    .await?;

                inserted |= self
                    .insert_pbs_computations(
                        tx,
                        &vec![handle],
                        transaction_hash,
                        block_number,
                    )
                    .await?;
            }
            AclContractEvents::AllowedForDecryption(allowed_for_decryption) => {
                let handles = allowed_for_decryption
                    .handlesList
                    .iter()
                    .map(|h| h.to_vec())
                    .collect::<Vec<_>>();

                for handle in handles.clone() {
                    info!(
                        handle = to_hex(&handle),
                        "Allowed for public decryption"
                    );

                    inserted |= self
                        .insert_allowed_handle(
                            tx,
                            handle,
                            "".to_string(),
                            AllowEvents::AllowedForDecryption,
                            transaction_hash.clone(),
                            block_number,
                        )
                        .await?;
                }

                inserted |= self
                    .insert_pbs_computations(
                        tx,
                        &handles,
                        transaction_hash.clone(),
                        block_number,
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
                    chain_id,
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
                    chain_id,
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
                         guid, block_number, transaction_id)
                     VALUES ($1, $2, $3, $4, $5, $6, $7)
                     ON CONFLICT (src_handle, dst_chain_id) DO NOTHING",
                    e.srcHandle.as_slice(),
                    dst_chain_id.as_i64(),
                    self.chain_id.as_i64(),
                    e.senderDapp.as_slice(),
                    e.guid.as_slice(),
                    block_number as i64,
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
                         guid, block_number, transaction_id)
                     VALUES ($1, $2, $3, $4, $5, $6, $7)
                     ON CONFLICT (dst_handle) DO NOTHING",
                    e.srcHandle.as_slice(),
                    e.dstHandle.as_slice(),
                    self.chain_id.as_i64(),
                    e.receiverDapp.as_slice(),
                    e.guid.as_slice(),
                    block_number as i64,
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
        handles: &Vec<Vec<u8>>,
        transaction_id: Option<Vec<u8>>,
        block_number: u64,
    ) -> Result<bool, SqlxError> {
        let mut inserted = false;
        for handle in handles {
            let query = sqlx::query!(
                "INSERT INTO pbs_computations(handle, transaction_id, host_chain_id, block_number) VALUES($1, $2, $3, $4)
                 ON CONFLICT DO NOTHING;",
                handle,
                transaction_id,
                self.chain_id.as_i64(),
                block_number as i64,
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

    /// Add the handle to the allowed_handles table
    pub async fn insert_allowed_handle(
        &self,
        tx: &mut Transaction<'_>,
        handle: Vec<u8>,
        account_address: String,
        event_type: AllowEvents,
        transaction_id: Option<Vec<u8>>,
        block_number: u64,
    ) -> Result<bool, SqlxError> {
        let query = sqlx::query!(
            "INSERT INTO allowed_handles(handle, account_address, event_type, transaction_id, host_chain_id, block_number) VALUES($1, $2, $3, $4, $5, $6)
                    ON CONFLICT DO NOTHING;",
            handle,
            account_address,
            event_type as i16,
            transaction_id,
            self.chain_id.as_i64(),
            block_number as i64
        );
        let inserted = query.execute(tx.deref_mut()).await?.rows_affected() > 0;
        Ok(inserted)
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
