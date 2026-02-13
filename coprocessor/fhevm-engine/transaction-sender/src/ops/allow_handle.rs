use std::{
    fmt::{Display, Formatter},
    str::FromStr,
    time::Duration,
};

use crate::{
    metrics::{ALLOW_HANDLE_FAIL_COUNTER, ALLOW_HANDLE_SUCCESS_COUNTER},
    nonce_managed_provider::NonceManagedProvider,
    ops::common::try_into_array,
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
use fhevm_engine_common::{telemetry, types::AllowEvents, utils::to_hex};
use sqlx::{Pool, Postgres};
use tokio::task::JoinSet;
use tracing::{debug, error, info, warn};

use fhevm_gateway_bindings::multichain_acl::MultichainACL;
use fhevm_gateway_bindings::multichain_acl::MultichainACL::MultichainACLErrors;

struct Key {
    handle: Vec<u8>,
    account_addr: String,
    event_type: AllowEvents,
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Key {{ handle: {}, account: {}, event_type: {:?} }}",
            to_hex(&self.handle),
            self.account_addr,
            self.event_type
        )
    }
}

#[derive(Clone)]
pub struct AllowHandleOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    multichain_acl_address: Address,
    provider: NonceManagedProvider<P>,
    conf: crate::ConfigSettings,
    gas: Option<u64>,
    db_pool: Pool<Postgres>,
}

impl<P> AllowHandleOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    /// Sends a transaction
    ///
    /// TODO: Refactor: Avoid code duplication
    #[tracing::instrument(skip_all, fields(operation = "call_allow_account"))]
    async fn send_transaction(
        &self,
        key: &Key,
        txn_request: impl Into<TransactionRequest>,
        current_limited_retries_count: i32,
        current_unlimited_retries_count: i32,
        src_transaction_id: Option<Vec<u8>>,
    ) -> anyhow::Result<()> {
        let h = to_hex(&key.handle);

        info!(handle = h, "Processing transaction");

        let receipt = match self
            .provider
            .send_sync_with_overprovision(
                txn_request,
                self.conf.gas_limit_overprovision_percent,
                Duration::from_secs(self.conf.send_txn_sync_timeout_secs.into()),
            )
            .await
        {
            Ok(receipt) => receipt,
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
            .and_then(|error| match error {
                E::CoprocessorAlreadyAllowedAccount(c) => Some(c.txSender), /* coprocessor address */
                E::CoprocessorAlreadyAllowedPublicDecrypt(c) => Some(c.txSender),
                _ => None
            })
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
                 AND account_address = $4",
            txn_hash,
            txn_block_number,
            key.handle,
            key.account_addr
        )
        .execute(&self.db_pool)
        .await?;

        telemetry::try_end_l1_transaction(&self.db_pool, &src_transaction_id.unwrap_or_default())
            .await?;

        Ok(())
    }
}

impl<P> AllowHandleOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
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

        if current_limited_retries_count == self.conf.allow_handle_max_retries - 1 {
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
            AND account_address = $3",
            err,
            key.handle,
            key.account_addr,
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
            AND account_address = $3",
            err,
            key.handle,
            key.account_addr,
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
            SELECT handle, account_address, event_type, txn_limited_retries_count, txn_unlimited_retries_count, transaction_id
            FROM allowed_handles 
            WHERE txn_is_sent = false 
            AND txn_limited_retries_count < $1
            LIMIT $2;
            ",
            self.conf.allow_handle_max_retries,
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
            let _span_guard =
                tracing::info_span!("prepare_allow_account", operation = "prepare_allow_account")
                    .entered();

            let handle = row.handle.clone();
            let chain_id = u64::from_be_bytes(handle[22..30].try_into()?);
            let h_as_hex = to_hex(&handle);
            let event_type = match AllowEvents::try_from(row.event_type) {
                Ok(event_type) => event_type,
                Err(_) => {
                    error!(event_type = row.event_type, "Invalid event_type");
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
                            handle = h_as_hex,
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
                event_type,
            };

            drop(_span_guard);

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
