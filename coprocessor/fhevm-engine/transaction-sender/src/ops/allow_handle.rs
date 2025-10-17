use std::{
    fmt::{Display, Formatter},
    str::FromStr,
    time::Duration,
};

use crate::{
    metrics::{ALLOW_HANDLE_FAIL_COUNTER, ALLOW_HANDLE_SUCCESS_COUNTER},
    nonce_managed_provider::NonceManagedProvider,
    ops::common::try_into_array,
    overprovision_gas_limit::try_overprovision_gas_limit,
    REVIEW,
};

use super::TransactionOperation;
use alloy::{
    network::{Ethereum, TransactionBuilder},
    primitives::{Address, Bytes, FixedBytes},
    providers::Provider,
    rpc::types::TransactionRequest,
    transports::{RpcError, TransportErrorKind},
};
use anyhow::bail;
use async_trait::async_trait;
use fhevm_engine_common::{
    telemetry, tenant_keys::query_tenant_info, types::AllowEvents, utils::compact_hex,
};
use sqlx::{Pool, Postgres};
use tokio::task::JoinSet;
use tracing::{debug, error, info, warn};

use fhevm_gateway_bindings::multichain_acl::MultichainACL;
use fhevm_gateway_bindings::multichain_acl::MultichainACL::MultichainACLErrors;

struct Key {
    handle: Vec<u8>,
    account_addr: String,
    tenant_id: i32,
    event_type: AllowEvents,
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Key {{ handle: {}, account: {}, tenant_id: {}, event_type: {:?} }}",
            compact_hex(&self.handle),
            self.account_addr,
            self.tenant_id,
            self.event_type
        )
    }
}

#[derive(Clone)]
pub struct AllowHandleOperation<P: Provider<Ethereum> + Clone + 'static> {
    multichain_acl_address: Address,
    provider: NonceManagedProvider<P>,
    conf: crate::ConfigSettings,
    gas: Option<u64>,
    db_pool: Pool<Postgres>,
}

impl<P: Provider<Ethereum> + Clone + 'static> AllowHandleOperation<P> {
    /// Sends a transaction
    ///
    /// TODO: Refactor: Avoid code duplication
    async fn send_transaction(
        &self,
        key: &Key,
        txn_request: impl Into<TransactionRequest>,
        current_limited_retries_count: i32,
        current_unlimited_retries_count: i32,
        src_transaction_id: Option<Vec<u8>>,
    ) -> anyhow::Result<()> {
        let h = compact_hex(&key.handle);

        info!(handle = h, "Processing transaction");
        let _t = telemetry::tracer("call_allow_account", &src_transaction_id);

        let overprovisioned_txn_req = try_overprovision_gas_limit(
            txn_request,
            self.provider.inner(),
            self.conf.gas_limit_overprovision_percent,
        )
        .await;
        let transaction = match self
            .provider
            .send_transaction(overprovisioned_txn_req.clone())
            .await
        {
            Ok(txn) => txn,
            Err(e) if self.already_allowed_error(&e).is_some() => {
                warn!(
                    address = ?self.already_allowed_error(&e),
                    handle = h,
                    "Coprocessor has already added the ACL entry"
                );
                self.set_txn_is_sent(key, None, None, src_transaction_id)
                    .await?;
                return Ok(());
            }
            // Consider transport retryable errors, BackendGone and local usage errors as something that must be retried infinitely.
            // Local usage are included as they might be transient due to external AWS KMS signers.
            Err(e)
                if matches!(&e, RpcError::Transport(inner) if inner.is_retry_err() || matches!(inner, TransportErrorKind::BackendGone))
                    || matches!(&e, RpcError::LocalUsageError(_)) =>
            {
                ALLOW_HANDLE_FAIL_COUNTER.inc();
                warn!(
                    transaction_request = ?overprovisioned_txn_req,
                    error = %e,
                    handle = h,
                    "Transaction sending failed with unlimited retry error"
                );
                self.increment_txn_unlimited_retries_count(
                    key,
                    &e.to_string(),
                    current_unlimited_retries_count,
                )
                .await?;
                bail!(e);
            }
            Err(e) => {
                ALLOW_HANDLE_FAIL_COUNTER.inc();
                warn!(
                    transaction_request = ?overprovisioned_txn_req,
                    error = %e,
                    handle = h,
                    "Transaction sending failed"
                );
                self.increment_txn_limited_retries_count(
                    key,
                    &e.to_string(),
                    current_limited_retries_count,
                )
                .await?;
                bail!(e);
            }
        };

        // We assume that if we were able to send the transaction, we will be able to get a receipt, eventually. If there is a transport
        // error in-between, we rely on the retry logic to handle it.
        let receipt = match transaction
            .with_timeout(Some(Duration::from_secs(
                self.conf.txn_receipt_timeout_secs as u64,
            )))
            .with_required_confirmations(self.conf.required_txn_confirmations as u64)
            .get_receipt()
            .await
        {
            Ok(receipt) => receipt,
            Err(e) => {
                ALLOW_HANDLE_FAIL_COUNTER.inc();
                error!(error = %e, "Getting receipt failed");
                self.increment_txn_limited_retries_count(
                    key,
                    &e.to_string(),
                    current_limited_retries_count,
                )
                .await?;
                return Err(anyhow::Error::new(e));
            }
        };

        if receipt.status() {
            self.set_txn_is_sent(
                key,
                Some(receipt.transaction_hash.as_slice()),
                receipt.block_number.map(|bn| bn as i64),
                src_transaction_id,
            )
            .await?;

            info!(
                transaction_hash = %receipt.transaction_hash,
                key = %key,
                "Allow txn succeeded"
            );
            ALLOW_HANDLE_SUCCESS_COUNTER.inc();
        } else {
            ALLOW_HANDLE_FAIL_COUNTER.inc();
            error!(
                transaction_hash = %receipt.transaction_hash,
                status = receipt.status(),
                handle = h,
                "allowAccount txn failed"
            );

            self.increment_txn_limited_retries_count(
                key,
                "receipt status = false",
                current_limited_retries_count,
            )
            .await?;

            return Err(anyhow::anyhow!(
                "Transaction {} failed with status {}, handle: {}",
                receipt.transaction_hash,
                receipt.status(),
                h,
            ));
        }
        Ok(())
    }

    fn already_allowed_error(&self, err: &RpcError<TransportErrorKind>) -> Option<Address> {
        use MultichainACLErrors as E;
        err.as_error_resp()
            .and_then(|payload| payload.as_decoded_interface_error::<MultichainACLErrors>())
            .map(|error| match error {
                E::CoprocessorAlreadyAllowedAccount(c) => Some(c.txSender), /* coprocessor address */
                E::CoprocessorAlreadyAllowedPublicDecrypt(c) => Some(c.txSender),
                _ => None
            })
            .flatten()
    }

    async fn set_txn_is_sent(
        &self,
        key: &Key,
        txn_hash: Option<&[u8]>,
        txn_block_number: Option<i64>,
        src_transaction_id: Option<Vec<u8>>,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE allowed_handles
                 SET
                    txn_is_sent = true,
                    txn_hash = $1,
                    txn_block_number = $2
                 WHERE handle = $3
                 AND account_address = $4
                 AND tenant_id = $5",
            txn_hash,
            txn_block_number,
            key.handle,
            key.account_addr,
            key.tenant_id
        )
        .execute(&self.db_pool)
        .await?;

        telemetry::try_end_l1_transaction(&self.db_pool, &src_transaction_id.unwrap_or_default())
            .await?;

        Ok(())
    }
}

impl<P: Provider<Ethereum> + Clone + 'static> AllowHandleOperation<P> {
    pub fn new(
        multichain_acl_address: Address,
        provider: NonceManagedProvider<P>,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
        db_pool: Pool<Postgres>,
    ) -> Self {
        info!(
            gas = gas.unwrap_or(0),
            multichain_acl_address = %multichain_acl_address,
            "Creating AllowHandleOperation"
        );

        Self {
            multichain_acl_address,
            provider,
            conf,
            gas,
            db_pool,
        }
    }

    async fn increment_txn_limited_retries_count(
        &self,
        key: &Key,
        err: &str,
        current_limited_retries_count: i32,
    ) -> anyhow::Result<()> {
        debug!("Updating retry count for key {}", key);

        if current_limited_retries_count == (self.conf.allow_handle_max_retries as i32) - 1 {
            error!(
                action = REVIEW,
                key = %key,
                max_retries = self.conf.allow_handle_max_retries,
                "Max limited retries reached"
            );
        } else {
            warn!(
                limited_reties_count = current_limited_retries_count + 1,
                key = %key,
                "Updating limited retry count"
            );
        }

        sqlx::query!(
            "UPDATE allowed_handles
            SET
            txn_limited_retries_count = txn_limited_retries_count + 1,
            txn_last_error = $1,
            txn_last_error_at = NOW()
            WHERE handle = $2
            AND account_address = $3
            AND tenant_id = $4",
            err,
            key.handle,
            key.account_addr,
            key.tenant_id
        )
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }

    async fn increment_txn_unlimited_retries_count(
        &self,
        key: &Key,
        err: &str,
        current_unlimited_retries_count: i32,
    ) -> anyhow::Result<()> {
        debug!("Updating unlimited retries count, {}", key);

        if current_unlimited_retries_count == (self.conf.review_after_unlimited_retries as i32) - 1
        {
            error!(
                action = REVIEW,
                unlimited_retries_count = current_unlimited_retries_count,
                key = %key,
                "Unlimited retries threshold reached"
            );
        } else {
            warn!(
                unlimited_retries_count = current_unlimited_retries_count + 1,
                key = %key,
                "Updating unlimited retries count"
            );
        }

        sqlx::query!(
            "UPDATE allowed_handles
            SET
            txn_unlimited_retries_count = txn_unlimited_retries_count + 1,
            txn_last_error = $1,
            txn_last_error_at = NOW()
            WHERE handle = $2
            AND account_address = $3
            AND tenant_id = $4",
            err,
            key.handle,
            key.account_addr,
            key.tenant_id
        )
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }
}

#[async_trait]
impl<P> TransactionOperation<P> for AllowHandleOperation<P>
where
    P: alloy::providers::Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str {
        &self.conf.allow_handle_db_channel
    }

    async fn execute(&self) -> anyhow::Result<bool> {
        let rows = sqlx::query!(
            "
            SELECT handle, tenant_id, account_address, event_type, txn_limited_retries_count, txn_unlimited_retries_count, transaction_id
            FROM allowed_handles 
            WHERE txn_is_sent = false 
            AND txn_limited_retries_count < $1
            LIMIT $2;
            ",
            self.conf.allow_handle_max_retries as i32,
            self.conf.allow_handle_batch_limit as i32,
        )
        .fetch_all(&self.db_pool)
        .await?;

        let multichain_acl = MultichainACL::new(self.multichain_acl_address, self.provider.inner());

        info!(rows_count = rows.len(), "Selected rows to process");

        let maybe_has_more_work = rows.len() == self.conf.allow_handle_batch_limit as usize;

        let mut join_set = JoinSet::new();
        for row in rows.into_iter() {
            let src_transaction_id = row.transaction_id.clone();
            let t = telemetry::tracer("prepare_allow_account", &src_transaction_id);

            let tenant = match query_tenant_info(&self.db_pool, row.tenant_id).await {
                Ok(res) => res,
                Err(_) => {
                    error!(
                        tenant_id = row.tenant_id,
                        "Failed to get chain_id for tenant"
                    );
                    continue;
                }
            };

            let chain_id = tenant.chain_id;
            let handle = row.handle.clone();
            let h_as_hex = compact_hex(&handle);
            let event_type = match AllowEvents::try_from(row.event_type) {
                Ok(event_type) => event_type,
                Err(_) => {
                    error!(
                        event_type = row.event_type,
                        tenant_id = row.tenant_id,
                        "Invalid event_type"
                    );
                    continue;
                }
            };

            let account_addr = row.account_address;
            info!(
                handle = h_as_hex,
                event_type = ?event_type,
                account = ?account_addr,
                chain_id = chain_id,
                "Allow handle"
            );

            let handle_bytes32 = FixedBytes::from(try_into_array::<32>(handle)?);
            let extra_data = Bytes::new();

            let txn_request = match event_type {
                AllowEvents::AllowedForDecryption => {
                    // Call allowPublicDecrypt when account_address is null
                    match &self.gas {
                        Some(gas_limit) => multichain_acl
                            .allowPublicDecrypt(handle_bytes32, extra_data)
                            .into_transaction_request()
                            .with_gas_limit(*gas_limit),
                        None => multichain_acl
                            .allowPublicDecrypt(handle_bytes32, extra_data)
                            .into_transaction_request(),
                    }
                }
                AllowEvents::AllowedAccount => {
                    let address = if let Ok(addr) = Address::from_str(&account_addr) {
                        addr
                    } else {
                        error!(
                            account_address = ?account_addr,
                            tenant_id = row.tenant_id,
                            "Invalid account address"
                        );
                        continue;
                    };
                    match &self.gas {
                        Some(gas_limit) => multichain_acl
                            .allowAccount(handle_bytes32, address, extra_data)
                            .into_transaction_request()
                            .with_gas_limit(*gas_limit),
                        None => multichain_acl
                            .allowAccount(handle_bytes32, address, extra_data)
                            .into_transaction_request(),
                    }
                }
            };

            let handle = row.handle;

            let key = Key {
                handle,
                account_addr: account_addr.to_string(),
                tenant_id: row.tenant_id,
                event_type,
            };

            t.end();

            let operation = self.clone();
            join_set.spawn(async move {
                operation
                    .send_transaction(
                        &key,
                        txn_request,
                        row.txn_limited_retries_count,
                        row.txn_unlimited_retries_count,
                        src_transaction_id,
                    )
                    .await
            });
        }

        while let Some(res) = join_set.join_next().await {
            res??;
        }

        Ok(maybe_has_more_work)
    }
}
