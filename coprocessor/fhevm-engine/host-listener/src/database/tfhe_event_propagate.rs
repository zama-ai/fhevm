use alloy_primitives::Address;
use alloy_primitives::FixedBytes;
use alloy_primitives::Log;
use alloy_primitives::Uint;
use anyhow::Result;
use bigdecimal::BigDecimal;
use fhevm_engine_common::chain_id::ChainId;
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
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::LazyLock;
use std::time::Duration;
use time::{Duration as TimeDuration, PrimitiveDateTime};
use tokio::sync::RwLock;
use tracing::error;
use tracing::info;
use tracing::warn;

use crate::cmd::block_history::BlockSummary;
use crate::contracts::AclContract::AclContractEvents;
use crate::contracts::TfheContract;
use crate::contracts::TfheContract::TfheContractEvents;

type FheOperation = i32;
pub type Handle = FixedBytes<32>;
pub type TransactionHash = FixedBytes<32>;
pub type ToType = u8;
pub type ScalarByte = FixedBytes<1>;
pub type ClearConst = Uint<256, 4>;
pub type ChainHash = TransactionHash;

static DEPENDENT_OPS_ALLOWED: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "host_listener_dependent_ops_allowed",
        "Number of dependent ops allowed by the limiter",
        &["chain_id"]
    )
    .unwrap()
});

static DEPENDENT_OPS_THROTTLED: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "host_listener_dependent_ops_throttled",
        "Number of dependent ops deferred by the limiter",
        &["chain_id"]
    )
    .unwrap()
});

#[derive(Clone, Debug)]
pub struct Chain {
    pub hash: ChainHash,
    pub dependencies: Vec<ChainHash>,
    pub dependents: Vec<ChainHash>,
    pub allowed_handle: Vec<Handle>,
    pub size: u64,
    pub before_size: u64,
    pub new_chain: bool,
}
pub type ChainCache = RwLock<lru::LruCache<Handle, ChainHash>>;
pub type OrderedChains = Vec<Chain>;

const MINIMUM_BUCKET_CACHE_SIZE: u16 = 16;
const MAX_RETRY_FOR_TRANSIENT_ERROR: usize = 20;
const MAX_RETRY_ON_UNKNOWN_ERROR: usize = 5;

// short wait in case the database had a short issue
const RECONNECTION_DELAY: Duration = Duration::from_millis(100);

type DbErrorCode = std::borrow::Cow<'static, str>;
const STATEMENT_CANCELLED: DbErrorCode = DbErrorCode::Borrowed("57014"); // SQLSTATE code for statement cancelled

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
pub struct Database {
    url: DatabaseURL,
    pub pool: Arc<RwLock<sqlx::Pool<Postgres>>>,
    pub chain_id: ChainId,
    chain_id_label: String,
    pub dependence_chain: ChainCache,
    pub tick: HeartBeat,
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
}

pub type Transaction<'l> = sqlx::Transaction<'l, Postgres>;

impl Database {
    pub async fn new(
        url: &DatabaseURL,
        chain_id: ChainId,
        dependence_cache_size: u16,
    ) -> Result<Self> {
        let pool = Self::new_pool(url).await;
        let bucket_cache = tokio::sync::RwLock::new(lru::LruCache::new(
            std::num::NonZeroU16::new(
                dependence_cache_size.max(MINIMUM_BUCKET_CACHE_SIZE),
            )
            .unwrap()
            .into(),
        ));
        Ok(Database {
            url: url.clone(),
            chain_id,
            chain_id_label: chain_id.to_string(),
            pool: Arc::new(RwLock::new(pool)),
            dependence_chain: bucket_cache,
            tick: HeartBeat::default(),
        })
    }

    pub(crate) fn record_dependent_ops_metrics(
        &self,
        allowed: u64,
        throttled: u64,
    ) {
        if allowed > 0 {
            DEPENDENT_OPS_ALLOWED
                .with_label_values(&[self.chain_id_label.as_str()])
                .inc_by(allowed);
        }
        if throttled > 0 {
            DEPENDENT_OPS_THROTTLED
                .with_label_values(&[self.chain_id_label.as_str()])
                .inc_by(throttled);
        }
    }

    pub async fn promote_seen_dep_chains_to_fast_priority(
        &self,
        tx: &mut Transaction<'_>,
        seen_dep_chain_ids: &[Vec<u8>],
    ) -> Result<u64, SqlxError> {
        if seen_dep_chain_ids.is_empty() {
            return Ok(0);
        }

        let rows = sqlx::query(
            r#"
            UPDATE dependence_chain dc
            SET schedule_priority = 0
            WHERE dc.schedule_priority <> 0
              AND dc.dependence_chain_id = ANY($1::bytea[])
            "#,
        )
        .bind(seen_dep_chain_ids)
        .execute(tx.deref_mut())
        .await?;
        Ok(rows.rows_affected())
    }

    async fn new_pool(url: &DatabaseURL) -> PgPool {
        let options: PgConnectOptions = url.parse().expect("bad url");
        let options = options.options([
            ("statement_timeout", "10000"), // 5 seconds
        ]);
        let connect = || {
            PgPoolOptions::new()
                .min_connections(2)
                .max_lifetime(Duration::from_secs(10 * 60))
                .max_connections(8)
                .acquire_timeout(Duration::from_secs(5))
                .connect_with(options.clone())
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

    pub async fn new_transaction(&self) -> Result<Transaction<'_>, SqlxError> {
        self.pool().await.begin().await
    }

    pub async fn pool(&self) -> sqlx::Pool<Postgres> {
        self.pool.read().await.clone()
    }

    pub async fn reconnect(&mut self) {
        tokio::time::sleep(RECONNECTION_DELAY).await;
        let old_pool = {
            let new_pool = Self::new_pool(&self.url).await;
            let mut pool = self.pool.write().await;
            std::mem::replace(&mut *pool, new_pool)
        };
        // doing the close outside out of lock
        old_pool.close().await;
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
        let dependence_chain_id = log.dependence_chain.to_vec();
        let schedule_order =
            log.block_timestamp
                .saturating_add(TimeDuration::microseconds(
                    log.tx_depth_size as i64,
                ));
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
                host_chain_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), $8::timestamp, $9, $10)
            ON CONFLICT (output_handle, transaction_id) DO NOTHING
            "#,
            output_handle,
            &dependencies,
            fhe_operation as i16,
            is_scalar,
            dependence_chain_id,
            log.transaction_hash.map(|txh| txh.to_vec()),
            log.is_allowed,
            schedule_order,
            !log.is_allowed,
            self.chain_id.as_i64()
        );
        query
            .execute(tx.deref_mut())
            .await
            .map(|result| result.rows_affected() > 0)
    }

    #[rustfmt::skip]
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
        let insert_computation = |tx, result, dependencies, scalar_byte| {
            self.insert_computation(tx, result, dependencies, fhe_operation, scalar_byte, log)
        };
        let insert_computation_bytes = |tx, result, dependencies_handles, dependencies_bytes, scalar_byte| {
            self.insert_computation_bytes(tx, result, dependencies_handles, dependencies_bytes, fhe_operation, scalar_byte, log)
        };

        let _t = telemetry::tracer(
            "handle_tfhe_event",
            &log.transaction_hash.map(|h| h.to_vec()),
        );

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

            | E::Initialized(_)
            | E::Upgraded(_)
            | E::VerifyInput(_)
            => Ok(false),
        }
    }

    pub async fn mark_block_as_valid(
        &self,
        tx: &mut Transaction<'_>,
        block_summary: &BlockSummary,
    ) -> Result<(), SqlxError> {
        sqlx::query!(
            r#"
            INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number)
            VALUES ($1, $2, $3)
            ON CONFLICT (chain_id, block_hash) DO NOTHING;
            "#,
            self.chain_id.as_i64(),
            block_summary.hash.to_vec(),
            block_summary.number as i64,
        )
        .execute(tx.deref_mut())
        .await?;
        Ok(())
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
        let pool = self.pool.read().await.clone();
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
        .execute(&pool)
        .await?;

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

        let transaction_hash = transaction_hash.map(|h| h.to_vec());

        let _t = telemetry::tracer("handle_acl_event", &transaction_hash);

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
                    )
                    .await?;

                inserted |= self
                    .insert_pbs_computations(
                        tx,
                        &vec![handle],
                        transaction_hash,
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
                        )
                        .await?;
                }

                inserted |= self
                    .insert_pbs_computations(
                        tx,
                        &handles,
                        transaction_hash.clone(),
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
        }
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
    ) -> Result<bool, SqlxError> {
        let mut inserted = false;
        for handle in handles {
            let query = sqlx::query!(
                "INSERT INTO pbs_computations(handle, transaction_id, host_chain_id) VALUES($1, $2, $3)
                 ON CONFLICT DO NOTHING;",
                handle,
                transaction_id,
                self.chain_id.as_i64(),
            );
            inserted |=
                query.execute(tx.deref_mut()).await?.rows_affected() > 0;
        }
        Ok(inserted)
    }

    /// Add the handle to the allowed_handles table
    pub async fn insert_allowed_handle(
        &self,
        tx: &mut Transaction<'_>,
        handle: Vec<u8>,
        account_address: String,
        event_type: AllowEvents,
        transaction_id: Option<Vec<u8>>,
    ) -> Result<bool, SqlxError> {
        let query = sqlx::query!(
            "INSERT INTO allowed_handles(handle, account_address, event_type, transaction_id) VALUES($1, $2, $3, $4)
                    ON CONFLICT DO NOTHING;",
            handle,
            account_address,
            event_type as i16,
            transaction_id
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

    pub async fn block_notification(
        &mut self,
        last_block_number: u64,
    ) -> Result<(), SqlxError> {
        let query = sqlx::query!(
            "SELECT pg_notify($1, $2)",
            "new_host_block",
            last_block_number.to_string()
        );
        query.execute(&self.pool().await).await?;
        Ok(())
    }

    pub async fn update_dependence_chain(
        &self,
        tx: &mut Transaction<'_>,
        chains: OrderedChains,
        block_timestamp: PrimitiveDateTime,
        block_summary: &BlockSummary,
        schedule_priority_by_chain: &HashMap<ChainHash, SchedulePriority>,
    ) -> Result<(), SqlxError> {
        for chain in chains {
            let dep_chain_id = chain.hash.to_vec();
            let schedule_priority = *schedule_priority_by_chain
                .get(&chain.hash)
                .unwrap_or(&SchedulePriority::FAST);
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
                dep_chain_id,
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
        | E::TrivialEncrypt(C::TrivialEncrypt { result, .. }) => Some(*result),

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
        | AclContractEvents::UnblockedAccount(_) => vec![],
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

        E::Initialized(_) | E::Upgraded(_) | E::VerifyInput(_) => vec![],
    }
}
