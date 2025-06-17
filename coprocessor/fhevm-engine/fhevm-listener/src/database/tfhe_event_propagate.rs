use alloy_primitives::FixedBytes;
use alloy_primitives::Log;
use alloy_primitives::Uint;
use fhevm_engine_common::types::AllowEvents;
use fhevm_engine_common::utils::compact_hex;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Uuid;
use sqlx::Error as SqlxError;
use sqlx::{PgPool, Postgres};
use tracing::warn;
use std::time::Duration;
use tracing::error;
use tracing::info;

use fhevm_engine_common::types::SupportedFheOperations;

use crate::contracts::AclContract::AclContractEvents;
use crate::contracts::TfheContract;
use crate::contracts::TfheContract::TfheContractEvents;

type CoprocessorApiKey = Uuid;
type FheOperation = i32;
pub type Handle = FixedBytes<32>;
pub type TenantId = i32;
pub type ChainId = u64;
pub type ToType = u8;
pub type ScalarByte = FixedBytes<1>;
pub type ClearConst = Uint<256, 4>;

pub fn retry_on_sqlx_error(err: &SqlxError) -> bool {
    match err {
        SqlxError::Io(_)
        | SqlxError::PoolTimedOut
        | SqlxError::PoolClosed
        | SqlxError::WorkerCrashed
        | SqlxError::Protocol(_) => true,
        // Other errors should be immdiately propagated up
        _ => false,
    }
}

// A pool of connection with some cached information and automatic reconnection
pub struct Database {
    url: String,
    pool: sqlx::Pool<Postgres>,
    tenant_id: TenantId,
    chain_id: ChainId,
}

impl Database {
    pub async fn new(
        url: &str,
        coprocessor_api_key: &CoprocessorApiKey,
        chain_id: ChainId,
    ) -> Self {
        let pool = Self::new_pool(url).await;
        let tenant_id =
            Self::find_tenant_id_or_panic(&pool, coprocessor_api_key).await;
        Database {
            url: url.into(),
            tenant_id,
            chain_id,
            pool,
        }
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

    async fn reconnect(&mut self) {
        self.pool.close().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
        self.pool = Self::new_pool(&self.url).await;
    }

    pub async fn find_tenant_id_or_panic(
        pool: &sqlx::Pool<Postgres>,
        tenant_api_key: &CoprocessorApiKey,
    ) -> TenantId {
        let query = || {
            sqlx::query_scalar!(
                r#"SELECT tenant_id FROM tenants WHERE tenant_api_key = $1"#,
                tenant_api_key.into()
            )
            .fetch_one(pool)
        };
        // retry mecanism
        loop {
            match query().await {
                Ok(tenant_id) => return tenant_id,
                Err(err) if retry_on_sqlx_error(&err) => {
                    error!(error = %err, "Error requesting tenant id, retrying");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Err(SqlxError::RowNotFound) => {
                    panic!("No tenant found for the provided API key, please check your API key")
                }
                Err(err) => {
                    panic!("Error requesting tenant id {err}, aborting")
                }
            }
        }
    }

    async fn insert_computation_bytes(
        &mut self,
        tenant_id: TenantId,
        result: &Handle,
        dependencies_handles: &[&Handle],
        dependencies_bytes: &[Vec<u8>], /* always added after
                                         * dependencies_handles */
        fhe_operation: FheOperation,
        scalar_byte: &FixedBytes<1>,
    ) -> Result<(), SqlxError> {
        let dependencies_handles = dependencies_handles
            .iter()
            .map(|d| d.to_vec())
            .collect::<Vec<_>>();
        let dependencies = [&dependencies_handles, dependencies_bytes].concat();
        self.insert_computation_inner(
            tenant_id,
            result,
            dependencies,
            fhe_operation,
            scalar_byte,
        )
        .await
    }

    async fn insert_computation(
        &mut self,
        tenant_id: TenantId,
        result: &Handle,
        dependencies: &[&Handle],
        fhe_operation: FheOperation,
        scalar_byte: &FixedBytes<1>,
    ) -> Result<(), SqlxError> {
        let dependencies =
            dependencies.iter().map(|d| d.to_vec()).collect::<Vec<_>>();
        self.insert_computation_inner(
            tenant_id,
            result,
            dependencies,
            fhe_operation,
            scalar_byte,
        )
        .await
    }

    async fn insert_computation_inner(
        &mut self,
        tenant_id: TenantId,
        result: &Handle,
        dependencies: Vec<Vec<u8>>,
        fhe_operation: FheOperation,
        scalar_byte: &FixedBytes<1>,
    ) -> Result<(), SqlxError> {
        let is_scalar = !scalar_byte.is_zero();
        let output_handle = result.to_vec();
        let query = || {
            sqlx::query!(
                r#"
            INSERT INTO computations (
                tenant_id,
                output_handle,
                dependencies,
                fhe_operation,
                is_scalar
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (tenant_id, output_handle) DO NOTHING
            "#,
                tenant_id as i32,
                output_handle,
                &dependencies,
                fhe_operation as i16,
                is_scalar
            )
        };
        // retry mecanism
        loop {
            match query().execute(&self.pool).await {
                Ok(_) => return Ok(()),
                Err(err) if retry_on_sqlx_error(&err) => {
                    error!(
                        error = %err,
                        "Database I/O error, will retry indefinitely"
                    );
                    self.reconnect().await;
                }
                Err(sqlx_err) => {
                    return Err(sqlx_err);
                }
            }
        }
    }

    #[rustfmt::skip]
    pub async fn insert_tfhe_event(
        &mut self,
        event: &Log<TfheContractEvents>
    ) -> Result<(), SqlxError> {
        use TfheContract as C;
        use TfheContractEvents as E;
        const HAS_SCALAR : FixedBytes::<1> = FixedBytes([1]); // if any dependency is a scalar.
        const NO_SCALAR : FixedBytes::<1> = FixedBytes([0]); // if all dependencies are handles.
        // ciphertext type
        let ty = |to_type: &ToType| vec![*to_type];
        let as_bytes = |x: &ClearConst| x.to_be_bytes_vec();
        let tenant_id = self.tenant_id;
        let fhe_operation = event_to_op_int(event);
        match &event.data {
            E::Cast(C::Cast {ct, toType, result, ..})
            => self.insert_computation_bytes(tenant_id, result, &[ct], &[ty(toType)], fhe_operation, &HAS_SCALAR).await,

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
            => self.insert_computation(tenant_id, result, &[lhs, rhs], fhe_operation, scalarByte).await,

            E::FheIfThenElse(C::FheIfThenElse {control, ifTrue, ifFalse, result, ..})
            => self.insert_computation(tenant_id, result, &[control, ifTrue, ifFalse], fhe_operation, &NO_SCALAR).await,

            | E::FheEq(C::FheEq {lhs, rhs, scalarByte, result, ..})
            | E::FheGe(C::FheGe {lhs, rhs, scalarByte, result, ..})
            | E::FheGt(C::FheGt {lhs, rhs, scalarByte, result, ..})
            | E::FheLe(C::FheLe {lhs, rhs, scalarByte, result, ..})
            | E::FheLt(C::FheLt {lhs, rhs, scalarByte, result, ..})
            | E::FheNe(C::FheNe {lhs, rhs, scalarByte, result, ..})
            => self.insert_computation(tenant_id, result, &[lhs, rhs], fhe_operation, scalarByte).await,


            E::FheNeg(C::FheNeg {ct, result, ..})
            | E::FheNot(C::FheNot {ct, result, ..})
            => self.insert_computation(tenant_id, result, &[ct], fhe_operation, &NO_SCALAR).await,

            | E::FheRand(C::FheRand {randType, seed, result, ..})
            => self.insert_computation_bytes(tenant_id, result, &[], &[seed.to_vec(), ty(randType)], fhe_operation, &HAS_SCALAR).await,

            | E::FheRandBounded(C::FheRandBounded {upperBound, randType, seed, result, ..})
            => self.insert_computation_bytes(tenant_id, result, &[], &[seed.to_vec(), as_bytes(upperBound), ty(randType)], fhe_operation, &HAS_SCALAR).await,

            | E::TrivialEncrypt(C::TrivialEncrypt {pt, toType, result, ..})
            => self.insert_computation_bytes(tenant_id, result, &[], &[as_bytes(pt), ty(toType)], fhe_operation, &HAS_SCALAR).await,

            | E::Initialized(_)
            | E::OwnershipTransferStarted(_)
            | E::OwnershipTransferred(_)
            | E::Upgraded(_)
            | E::VerifyCiphertext(_)
            => Ok(()),
        }
    }

    pub async fn mark_prev_block_as_valid(
        &mut self,
        opt_event: &Option<alloy_rpc_types::Log>,
        opt_prev_event: &Option<alloy_rpc_types::Log>,
    ) -> Option<u64> {
        let Some(prev_event) = opt_prev_event else {
            return None;
        };
        let Some(event) = opt_event else {
            return None;
        };
        if prev_event.block_number == event.block_number {
            return None;
        }
        let prev_event = if prev_event.block_number < event.block_number {
            event
        } else {
            prev_event
        };
        let Some(block_number) = prev_event.block_number else {
            return None;
        };
        let Some(block_hash) = prev_event.block_hash else {
            return Some(block_number); // but cannot write to db
        };
        let _ = sqlx::query!(
            r#"
            INSERT INTO blocks_valid (chain_id, block_hash, block_number, listener_tfhe)
            VALUES ($1, $2, $3, true)
            ON CONFLICT (chain_id, block_hash) DO UPDATE SET listener_tfhe = true;
            "#,
            self.chain_id as i64,
            block_hash.to_vec(),
            block_number as i64,
        )
        .execute(&self.pool)
        .await;
        return Some(block_number);
    }

    pub async fn read_last_valid_block(&mut self) -> Option<i64> {
        let query = || {
            sqlx::query!(
                r#"
            SELECT block_number FROM blocks_valid WHERE chain_id = $1 ORDER BY block_number DESC LIMIT 1;
            "#,
                self.chain_id as i64,
            )
            .fetch_one(&self.pool)
        };
        match query().await {
            Ok(record) => Some(record.block_number),
            Err(_err) => None, // table could be empty
        }
    }

    /// Handles all types of ACL events
    pub async fn handle_acl_event(
        &mut self,
        event: &Log<AclContractEvents>,
    ) -> Result<(), SqlxError> {
        let data = &event.data;

        match data {
            AclContractEvents::Allowed(allowed) => {
                let handle = allowed.handle.to_vec();

                self.insert_allowed_handle(
                    handle.clone(),
                    allowed.account.to_string(),
                    AllowEvents::AllowedAccount,
                )
                .await?;

                self.insert_pbs_computations(&vec![handle]).await?;
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
                        handle,
                        "".to_string(),
                        AllowEvents::AllowedForDecryption,
                    )
                    .await?;
                }

                self.insert_pbs_computations(&handles).await?;
            }
            AclContractEvents::Initialized(initialized) => {
                warn!(event = ?initialized, "unhandled Acl::Initialized event");
            }
            AclContractEvents::NewDelegation(new_delegation) => {
                warn!(
                    event = ?new_delegation,
                    "unhandled Acl::NewDelegation event"
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
            AclContractEvents::OwnershipTransferred(ownership_transferred) => {
                warn!(
                    event = ?ownership_transferred,
                    "unhandled Acl::OwnershipTransferred event"
                );
            }
            AclContractEvents::RevokedDelegation(revoked_delegation) => {
                warn!(
                    event = ?revoked_delegation,
                    "unhandled Acl::RevokedDelegation event"
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
            AclContractEvents::UpdatePauser(update_pauser) => {
                warn!(
                    event = ?update_pauser,
                    "unhandled Acl::UpdatePauser event"
                );
            }
        }

        Ok(())
    }

    /// Adds handles to the pbs_computations table and alerts the SnS worker
    /// about new of PBS work.
    pub async fn insert_pbs_computations(
        &mut self,
        handles: &Vec<Vec<u8>>,
    ) -> Result<(), SqlxError> {
        let tenant_id = self.tenant_id;
        for handle in handles {
            let query = || {
                sqlx::query!(
                    "INSERT INTO pbs_computations(tenant_id, handle) VALUES($1, $2) 
                         ON CONFLICT DO NOTHING;",
                    tenant_id,
                    handle,
                )
            };

            loop {
                match query().execute(&self.pool).await {
                    Ok(_) => break,
                    Err(err) if retry_on_sqlx_error(&err) => {
                        error!(error = %err, "Database I/O error, will retry indefinitely");
                        self.reconnect().await;
                    }
                    Err(sqlx_err) => {
                        return Err(sqlx_err);
                    }
                }
            }
        }

        Ok(())
    }

    /// Add the handle to the allowed_handles table
    pub async fn insert_allowed_handle(
        &mut self,
        handle: Vec<u8>,
        account_address: String,
        event_type: AllowEvents,
    ) -> Result<(), SqlxError> {
        let tenant_id = self.tenant_id;

        let query = || {
            sqlx::query!(
                "INSERT INTO allowed_handles(tenant_id, handle, account_address, event_type) VALUES($1, $2, $3, $4)
                     ON CONFLICT DO NOTHING;",
                tenant_id,
                handle,
                account_address,
                event_type as i16,
            )
        };

        loop {
            match query().execute(&self.pool).await {
                Ok(_) => break,
                Err(err) if retry_on_sqlx_error(&err) => {
                    error!(error = %err, "Database I/O error, will retry indefinitely");
                    self.reconnect().await;
                }
                Err(sqlx_err) => {
                    return Err(sqlx_err);
                }
            }
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
        E::Initialized(_)
        | E::OwnershipTransferStarted(_)
        | E::OwnershipTransferred(_)
        | E::Upgraded(_)
        | E::VerifyCiphertext(_) => -1,
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
        E::OwnershipTransferStarted(_) => "OwnershipTransferStarted",
        E::OwnershipTransferred(_) => "OwnershipTransferred",
        E::Upgraded(_) => "Upgraded",
        E::VerifyCiphertext(_) => "VerifyCiphertext",
    }
}
