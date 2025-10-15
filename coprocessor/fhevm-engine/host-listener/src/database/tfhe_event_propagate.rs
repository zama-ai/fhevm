use alloy_primitives::FixedBytes;
use alloy_primitives::Log;
use alloy_primitives::Uint;
use anyhow::Result;
use fhevm_engine_common::telemetry;
use fhevm_engine_common::types::AllowEvents;
use fhevm_engine_common::types::SupportedFheOperations;
use fhevm_engine_common::utils::{compact_hex, HeartBeat};
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Uuid;
use sqlx::Error as SqlxError;
use sqlx::{PgPool, Postgres};
use std::ops::DerefMut;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::error;
use tracing::info;
use tracing::warn;

use crate::cmd::block_history::BlockSummary;
use crate::contracts::AclContract::AclContractEvents;
use crate::contracts::TfheContract;
use crate::contracts::TfheContract::TfheContractEvents;

type CoprocessorApiKey = Uuid;
type FheOperation = i32;
pub type Handle = FixedBytes<32>;
pub type TransactionHash = FixedBytes<32>;
pub type TenantId = i32;
pub type ChainId = u64;
pub type ToType = u8;
pub type ScalarByte = FixedBytes<1>;
pub type ClearConst = Uint<256, 4>;

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
    url: String,
    pub pool: Arc<RwLock<sqlx::Pool<Postgres>>>,
    pub tenant_id: TenantId,
    pub chain_id: ChainId,
    bucket_cache: tokio::sync::RwLock<lru::LruCache<Handle, Handle>>,
    pub tick: HeartBeat,
}

#[derive(Debug)]
pub struct LogTfhe {
    pub event: Log<TfheContractEvents>,
    pub transaction_hash: Option<TransactionHash>,
    pub is_allowed: bool,
    pub block_number: Option<u64>,
}

pub type Transaction<'l> = sqlx::Transaction<'l, Postgres>;

impl Database {
    pub async fn new(
        url: &str,
        coprocessor_api_key: &CoprocessorApiKey,
        bucket_cache_size: u16,
    ) -> Result<Self> {
        let pool = Self::new_pool(url).await;
        let (tenant_id, chain_id) =
            Self::find_tenant_id(&pool, coprocessor_api_key).await?;
        let bucket_cache = tokio::sync::RwLock::new(lru::LruCache::new(
            std::num::NonZeroU16::new(
                bucket_cache_size.max(MINIMUM_BUCKET_CACHE_SIZE),
            )
            .unwrap()
            .into(),
        ));
        Ok(Database {
            url: url.into(),
            tenant_id,
            chain_id,
            pool: Arc::new(RwLock::new(pool)),
            bucket_cache,
            tick: HeartBeat::default(),
        })
    }

    async fn new_pool(url: &str) -> PgPool {
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
        self.pool.read().await.clone().begin().await
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

    async fn find_tenant_id(
        pool: &sqlx::Pool<Postgres>,
        tenant_api_key: &CoprocessorApiKey,
    ) -> Result<(TenantId, ChainId)> {
        let query = || {
            sqlx::query!(
                r#"SELECT tenant_id, chain_id FROM tenants WHERE tenant_api_key = $1"#,
                tenant_api_key.into()
            )
            .fetch_one(pool)
        };
        // retry mecanism
        let mut retry_count = 0;
        loop {
            match query().await {
                Ok(record) => {
                    return Ok((record.tenant_id, record.chain_id as u64))
                }
                Err(SqlxError::RowNotFound) => {
                    anyhow::bail!("No tenant found for the provided API key");
                }
                Err(err) if retry_on_sqlx_error(&err, &mut retry_count) => {
                    error!(error = %err, "Error requesting tenant id, retrying");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
                Err(err) => {
                    return Err(err.into());
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn insert_computation_bytes(
        &self,
        tx: &mut Transaction<'_>,
        tenant_id: TenantId,
        result: &Handle,
        dependencies_handles: &[&Handle],
        dependencies_bytes: &[Vec<u8>], /* always added after
                                         * dependencies_handles */
        fhe_operation: FheOperation,
        scalar_byte: &FixedBytes<1>,
        log: &LogTfhe,
    ) -> Result<(), SqlxError> {
        let bucket = self
            .sort_computation_into_bucket(
                result,
                dependencies_handles,
                &log.transaction_hash,
            )
            .await;
        let dependencies_handles = dependencies_handles
            .iter()
            .map(|d| d.to_vec())
            .collect::<Vec<_>>();
        let dependencies = [&dependencies_handles, dependencies_bytes].concat();
        self.insert_computation_inner(
            tx,
            tenant_id,
            result,
            dependencies,
            fhe_operation,
            scalar_byte,
            log,
            &bucket,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn insert_computation(
        &self,
        tx: &mut Transaction<'_>,
        tenant_id: TenantId,
        result: &Handle,
        dependencies: &[&Handle],
        fhe_operation: FheOperation,
        scalar_byte: &FixedBytes<1>,
        log: &LogTfhe,
    ) -> Result<(), SqlxError> {
        let bucket = self
            .sort_computation_into_bucket(
                result,
                dependencies,
                &log.transaction_hash,
            )
            .await;
        let dependencies =
            dependencies.iter().map(|d| d.to_vec()).collect::<Vec<_>>();
        self.insert_computation_inner(
            tx,
            tenant_id,
            result,
            dependencies,
            fhe_operation,
            scalar_byte,
            log,
            &bucket,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn insert_computation_inner(
        &self,
        tx: &mut Transaction<'_>,
        tenant_id: TenantId,
        result: &Handle,
        dependencies: Vec<Vec<u8>>,
        fhe_operation: FheOperation,
        scalar_byte: &FixedBytes<1>,
        log: &LogTfhe,
        bucket: &Handle,
    ) -> Result<(), SqlxError> {
        let is_scalar = !scalar_byte.is_zero();
        let output_handle = result.to_vec();
        let query = sqlx::query!(
            r#"
            INSERT INTO computations (
                tenant_id,
                output_handle,
                dependencies,
                fhe_operation,
                is_scalar,
                dependence_chain_id,
                transaction_id,
                is_allowed
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (tenant_id, output_handle, transaction_id) DO NOTHING
            "#,
            tenant_id as i32,
            output_handle,
            &dependencies,
            fhe_operation as i16,
            is_scalar,
            bucket.to_vec(),
            log.transaction_hash.map(|txh| txh.to_vec()),
            log.is_allowed,
        );
        query.execute(tx.deref_mut()).await.map(|_| ())
    }

    async fn sort_computation_into_bucket(
        &self,
        output: &Handle,
        dependencies: &[&Handle],
        transaction_hash: &Option<Handle>,
    ) -> Handle {
        // If the transaction ID is a hit in the cache, update its
        // last use and add the output handle in the bucket
        if let Some(txh) = transaction_hash {
            // We need a write access here as get updates the LRUcache
            let mut bucket_cache_write = self.bucket_cache.write().await;
            if let Some(ce) = bucket_cache_write.get(txh).cloned() {
                bucket_cache_write.put(*output, ce);
                return ce;
            }
        }
        // If any input dependence is a match, return its bucket. This
        // computation is in a connected component with other ops in
        // this bucket
        let bucket_cache_read = self.bucket_cache.read().await;
        for d in dependencies {
            // We peek here as the reuse is less likely than the use
            // of the new handle which we add - because handles
            // operate under single assinment
            if let Some(ce) = bucket_cache_read.peek(*d).cloned() {
                drop(bucket_cache_read);
                let mut bucket_cache_write = self.bucket_cache.write().await;
                bucket_cache_write.put(*output, ce);
                // As the transaction hash was not in the cache, add
                // it to this bucket as well
                if let Some(txh) = transaction_hash {
                    bucket_cache_write.put(*txh, ce);
                }
                return ce;
            }
        }
        drop(bucket_cache_read);
        // If this computation is not linked to any others, assign it
        // to a new empty bucket and add output handle and transaction
        // hash where relevant
        let mut bucket_cache_write = self.bucket_cache.write().await;
        bucket_cache_write.put(*output, *output);
        if let Some(txh) = transaction_hash {
            bucket_cache_write.put(*txh, *output);
        }
        *output
    }

    #[rustfmt::skip]
    pub async fn insert_tfhe_event(
        &self,
        tx: &mut Transaction<'_>,
        log: &LogTfhe,
    ) -> Result<(), SqlxError> {
        use TfheContract as C;
        use TfheContractEvents as E;
        const HAS_SCALAR : FixedBytes::<1> = FixedBytes([1]); // if any dependency is a scalar.
        const NO_SCALAR : FixedBytes::<1> = FixedBytes([0]); // if all dependencies are handles.
        // ciphertext type
        let event = &log.event;
        let ty = |to_type: &ToType| vec![*to_type];
        let as_bytes = |x: &ClearConst| x.to_be_bytes_vec();
        let tenant_id = self.tenant_id;
        let fhe_operation = event_to_op_int(event);
        let insert_computation = |tx, result, dependencies, scalar_byte| {
            self.insert_computation(tx, tenant_id, result, dependencies, fhe_operation, scalar_byte, log)
        };
        let insert_computation_bytes = |tx, result, dependencies_handles, dependencies_bytes, scalar_byte| {
            self.insert_computation_bytes(tx, tenant_id, result, dependencies_handles, dependencies_bytes, fhe_operation, scalar_byte, log)
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
                &log.block_number,
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
            => Ok(()),
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
            self.chain_id as i64,
            block_summary.hash.to_vec(),
            block_summary.number as i64,
        )
        .execute(tx.deref_mut())
        .await?;
        Ok(())
    }

    pub async fn read_last_valid_block(&mut self) -> Option<i64> {
        let query = sqlx::query!(
            r#"
            SELECT MAX(block_number) FROM host_chain_blocks_valid WHERE chain_id = $1;
            "#,
            self.chain_id as i64,
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
        block_number: &Option<u64>,
    ) -> Result<(), SqlxError> {
        let data = &event.data;

        let transaction_hash = transaction_hash.map(|h| h.to_vec());

        let _t = telemetry::tracer("handle_acl_event", &transaction_hash);

        // Record only Allowed or AllowedForDecryption events
        if matches!(
            data,
            AclContractEvents::Allowed(_)
                | AclContractEvents::AllowedForDecryption(_)
        ) {
            self.record_transaction_begin(&transaction_hash, block_number)
                .await;
        }

        match data {
            AclContractEvents::Allowed(allowed) => {
                let handle = allowed.handle.to_vec();

                self.insert_allowed_handle(
                    tx,
                    handle.clone(),
                    allowed.account.to_string(),
                    AllowEvents::AllowedAccount,
                    transaction_hash.clone(),
                )
                .await?;

                self.insert_pbs_computations(
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
                        handle = compact_hex(&handle),
                        "Allowed for public decryption"
                    );

                    self.insert_allowed_handle(
                        tx,
                        handle,
                        "".to_string(),
                        AllowEvents::AllowedForDecryption,
                        transaction_hash.clone(),
                    )
                    .await?;
                }

                self.insert_pbs_computations(
                    tx,
                    &handles,
                    transaction_hash.clone(),
                )
                .await?;
            }
            AclContractEvents::Initialized(initialized) => {
                warn!(event = ?initialized, "unhandled Acl::Initialized event");
            }
            AclContractEvents::DelegatedForUserDecryption(delegate_account) => {
                warn!(
                    event = ?delegate_account,
                    "unhandled Acl::DelegatedForUserDecryption event"
                );
            }
            AclContractEvents::OwnershipTransferStarted(
                ownership_transfer_started,
            ) => {
                warn!(
                    event = ?ownership_transfer_started,
                    "unhandled Acl::OwnershipTransferStarted event"
                );
            }
            AclContractEvents::RevokedDelegationForUserDecryption(revoked_delegation) => {
                warn!(
                    event = ?revoked_delegation,
                    "unhandled Acl::RevokedDelegationForUserDecryption event"
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
        }
        self.tick.update();
        Ok(())
    }

    /// Adds handles to the pbs_computations table and alerts the SnS worker
    /// about new of PBS work.
    pub async fn insert_pbs_computations(
        &self,
        tx: &mut Transaction<'_>,
        handles: &Vec<Vec<u8>>,
        transaction_id: Option<Vec<u8>>,
    ) -> Result<(), SqlxError> {
        let tenant_id = self.tenant_id;
        for handle in handles {
            let query = sqlx::query!(
                "INSERT INTO pbs_computations(tenant_id, handle, transaction_id) VALUES($1, $2, $3)
                 ON CONFLICT DO NOTHING;",
                tenant_id,
                handle,
                transaction_id
            );
            query.execute(tx.deref_mut()).await?;
        }
        Ok(())
    }

    /// Add the handle to the allowed_handles table
    pub async fn insert_allowed_handle(
        &self,
        tx: &mut Transaction<'_>,
        handle: Vec<u8>,
        account_address: String,
        event_type: AllowEvents,
        transaction_id: Option<Vec<u8>>,
    ) -> Result<(), SqlxError> {
        let tenant_id = self.tenant_id;
        let query = sqlx::query!(
            "INSERT INTO allowed_handles(tenant_id, handle, account_address, event_type, transaction_id) VALUES($1, $2, $3, $4, $5)
                    ON CONFLICT DO NOTHING;",
            tenant_id,
            handle,
            account_address,
            event_type as i16,
            transaction_id
        );
        query.execute(tx.deref_mut()).await?;
        Ok(())
    }

    async fn record_transaction_begin(
        &self,
        transaction_hash: &Option<Vec<u8>>,
        block_number: &Option<u64>,
    ) {
        if let Some(txn_id) = transaction_hash {
            let pool = self.pool.read().await.clone();
            let _ = telemetry::try_begin_transaction(
                &pool,
                self.chain_id as i64,
                txn_id.as_ref(),
                block_number.unwrap_or_default(),
            )
            .await;
        }
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
        | AclContractEvents::OwnershipTransferStarted(_)
        | AclContractEvents::OwnershipTransferred(_)
        | AclContractEvents::RevokedDelegationForUserDecryption(_)
        | AclContractEvents::Upgraded(_)
        | AclContractEvents::Paused(_)
        | AclContractEvents::Unpaused(_) => vec![],
    }
}
