use std::time::Duration;

use crate::{
    metrics::{ADD_CIPHERTEXT_MATERIAL_FAIL_COUNTER, ADD_CIPHERTEXT_MATERIAL_SUCCESS_COUNTER},
    nonce_managed_provider::NonceManagedProvider,
    overprovision_gas_limit::try_overprovision_gas_limit,
    REVIEW,
};

use super::common::try_into_array;
use super::TransactionOperation;
use alloy::{
    network::{Ethereum, TransactionBuilder},
    primitives::{Address, FixedBytes, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
    transports::{RpcError, TransportErrorKind},
};
use anyhow::bail;
use async_trait::async_trait;
use fhevm_engine_common::{telemetry, tenant_keys::query_tenant_info, utils::compact_hex};
use sqlx::{Pool, Postgres};
use tokio::task::JoinSet;
use tracing::{error, info, warn};

use fhevm_gateway_bindings::ciphertext_commits::CiphertextCommits;
use fhevm_gateway_bindings::ciphertext_commits::CiphertextCommits::CiphertextCommitsErrors;

#[derive(Clone)]
pub struct AddCiphertextOperation<P: Provider<Ethereum> + Clone + 'static> {
    ciphertext_commits_address: Address,
    provider: NonceManagedProvider<P>,
    conf: crate::ConfigSettings,
    gas: Option<u64>,
    db_pool: Pool<Postgres>,
}

impl<P: Provider<Ethereum> + Clone + 'static> AddCiphertextOperation<P> {
    async fn send_transaction(
        &self,
        handle: &[u8],
        txn_request: impl Into<TransactionRequest>,
        current_limited_retries_count: i32,
        current_unlimited_retries_count: i32,
        src_transaction_id: Option<Vec<u8>>,
    ) -> anyhow::Result<()> {
        let h = compact_hex(handle);

        info!(handle = h, "Processing transaction");
        let _t = telemetry::tracer("call_add_ciphertext", &src_transaction_id);

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
            Err(e) if self.already_added_error(&e).is_some() => {
                warn!(
                    handle = h,
                    address = ?self.already_added_error(&e),
                    "Coprocessor has already added the ciphertext commit",
                );
                self.set_txn_is_sent(handle, None, None, src_transaction_id)
                    .await?;
                return Ok(());
            }
            // Consider transport retryable errors, BackendGone and local usage errors as something that must be retried infinitely.
            // Local usage are included as they might be transient due to external AWS KMS signers.
            Err(e)
                if matches!(&e, RpcError::Transport(inner) if inner.is_retry_err() || matches!(inner, TransportErrorKind::BackendGone))
                    || matches!(&e, RpcError::LocalUsageError(_)) =>
            {
                ADD_CIPHERTEXT_MATERIAL_FAIL_COUNTER.inc();
                warn!(
                    transaction_request = ?overprovisioned_txn_req,
                    error = %e,
                    handle = h,
                    "Transaction sending failed with unlimited retry error"
                );
                self.increment_txn_unlimited_retries_count(
                    handle,
                    &e.to_string(),
                    current_unlimited_retries_count,
                )
                .await?;
                bail!(e);
            }
            Err(e) => {
                ADD_CIPHERTEXT_MATERIAL_FAIL_COUNTER.inc();
                warn!(
                    transaction_request = ?overprovisioned_txn_req,
                    error = %e,
                    handle = h,
                    "Transaction sending failed"
                );
                self.increment_txn_limited_retries_count(
                    handle,
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
                ADD_CIPHERTEXT_MATERIAL_FAIL_COUNTER.inc();
                error!(error = %e, "Getting receipt failed");
                self.increment_txn_limited_retries_count(
                    handle,
                    &e.to_string(),
                    current_limited_retries_count,
                )
                .await?;
                return Err(anyhow::Error::new(e));
            }
        };

        if receipt.status() {
            self.set_txn_is_sent(
                handle,
                Some(receipt.transaction_hash.as_slice()),
                receipt.block_number.map(|bn| bn as i64),
                src_transaction_id,
            )
            .await?;
            info!(
                transaction_hash = %receipt.transaction_hash,
                handle = h,
                "addCiphertext txn succeeded"
            );
            ADD_CIPHERTEXT_MATERIAL_SUCCESS_COUNTER.inc();
        } else {
            ADD_CIPHERTEXT_MATERIAL_FAIL_COUNTER.inc();
            error!(
                transaction_hash = %receipt.transaction_hash,
                status = receipt.status(),
                handle = h,
                "addCiphertext txn failed"
            );

            self.increment_txn_limited_retries_count(
                handle,
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

    fn already_added_error(&self, err: &RpcError<TransportErrorKind>) -> Option<Address> {
        err.as_error_resp()
            .and_then(|payload| payload.as_decoded_interface_error::<CiphertextCommitsErrors>())
            .map(|error| match error {
                CiphertextCommitsErrors::CoprocessorAlreadyAdded(c) => {
                    Some(Address::from_slice(c.ctHandle.as_ref()))
                }
                _ => None,
            })
            .flatten()
    }

    async fn set_txn_is_sent(
        &self,
        handle: &[u8],
        txn_hash: Option<&[u8]>,
        txn_block_number: Option<i64>,
        src_transaction_id: Option<Vec<u8>>,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE ciphertext_digest
            SET
                txn_is_sent = true,
                txn_hash = $1,
                txn_block_number = $2
            WHERE handle = $3",
            txn_hash,
            txn_block_number,
            handle
        )
        .execute(&self.db_pool)
        .await?;

        if let Some(txn_hash) = src_transaction_id {
            telemetry::try_end_l1_transaction(&self.db_pool, &txn_hash).await?;
        }

        Ok(())
    }
}

impl<P: Provider<Ethereum> + Clone + 'static> AddCiphertextOperation<P> {
    pub fn new(
        ciphertext_commits_address: Address,
        provider: NonceManagedProvider<P>,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
        db_pool: Pool<Postgres>,
    ) -> Self {
        info!(
            gas = gas.unwrap_or(0),
            ciphertext_commits_address = %ciphertext_commits_address,
            "Creating AddCiphertextOperation"
        );

        Self {
            db_pool,
            ciphertext_commits_address,
            provider,
            conf,
            gas,
        }
    }

    async fn increment_txn_limited_retries_count(
        &self,
        handle: &[u8],
        err: &str,
        current_retry_count: i32,
    ) -> anyhow::Result<()> {
        let compact_hex_handle = compact_hex(handle);
        if current_retry_count == (self.conf.add_ciphertexts_max_retries as i32) - 1 {
            error!(
                action = REVIEW,
                max_retries = self.conf.add_ciphertexts_max_retries,
                handle = compact_hex_handle,
                "Max retries reached for adding ciphertext"
            );
        } else {
            warn!(
                retry_count = current_retry_count + 1,
                handle = compact_hex_handle,
                "Updating limited retries count"
            );
        }
        sqlx::query!(
            "UPDATE ciphertext_digest
            SET
            txn_limited_retries_count = txn_limited_retries_count + 1,
            txn_last_error = $1,
            txn_last_error_at = NOW()
            WHERE handle = $2",
            err,
            handle,
        )
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }

    async fn increment_txn_unlimited_retries_count(
        &self,
        handle: &[u8],
        err: &str,
        current_unlimited_retries_count: i32,
    ) -> anyhow::Result<()> {
        let compact_hex_handle = compact_hex(handle);
        if current_unlimited_retries_count >= (self.conf.review_after_unlimited_retries as i32) - 1
        {
            error!(
                action = REVIEW,
                unlimited_retries = current_unlimited_retries_count,
                handle = compact_hex_handle,
                "Unlimited retries threshold reached for adding ciphertext"
            );
        } else {
            warn!(
                unlimited_retries = current_unlimited_retries_count + 1,
                handle = compact_hex_handle,
                "Updating unlimited retries count"
            );
        }
        sqlx::query!(
            "UPDATE ciphertext_digest
            SET
            txn_unlimited_retries_count = txn_unlimited_retries_count + 1,
            txn_last_error = $1,
            txn_last_error_at = NOW()
            WHERE handle = $2",
            err,
            handle,
        )
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }
}

#[async_trait]
impl<P> TransactionOperation<P> for AddCiphertextOperation<P>
where
    P: alloy::providers::Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str {
        &self.conf.add_ciphertexts_db_channel
    }

    async fn execute(&self) -> anyhow::Result<bool> {
        // The service responsible for populating the ciphertext_digest table must
        // ensure that ciphertext and ciphertext128 are non-null only after the
        // ciphertexts have been successfully uploaded to AWS S3 buckets.
        let rows = sqlx::query!(
            "
            SELECT handle, ciphertext, ciphertext128, tenant_id, txn_limited_retries_count, txn_unlimited_retries_count, transaction_id
            FROM ciphertext_digest
            WHERE txn_is_sent = false
            AND ciphertext IS NOT NULL
            AND ciphertext128 IS NOT NULL
            AND txn_limited_retries_count < $1
            LIMIT $2",
            self.conf.add_ciphertexts_max_retries as i64,
            self.conf.add_ciphertexts_batch_limit as i64,
        )
        .fetch_all(&self.db_pool)
        .await?;

        let ciphertext_manager =
            CiphertextCommits::new(self.ciphertext_commits_address, self.provider.inner());

        info!(rows_count = rows.len(), "Selected rows to process");

        let maybe_has_more_work = rows.len() == self.conf.add_ciphertexts_batch_limit as usize;

        let mut join_set = JoinSet::new();
        for row in rows.into_iter() {
            let transaction_id = row.transaction_id.clone();
            let t = telemetry::tracer("prepare_add_ciphertext", &transaction_id);

            let tenant_info = match query_tenant_info(&self.db_pool, row.tenant_id).await {
                Ok(res) => res,
                Err(_) => {
                    error!(tenant_id = row.tenant_id, "Failed to get key_id for tenant");
                    continue;
                }
            };

            let handle = row.handle.clone();

            let (ciphertext64_digest, ciphertext128_digest) =
                match (row.ciphertext, row.ciphertext128) {
                    (Some(ct), Some(ct128)) => (
                        FixedBytes::from(try_into_array::<32>(ct)?),
                        FixedBytes::from(try_into_array::<32>(ct128)?),
                    ),
                    _ => {
                        error!(handle = compact_hex(&handle), "Missing ciphertext(s)");
                        continue;
                    }
                };

            let handle_bytes32 = FixedBytes::from(try_into_array::<32>(handle.clone())?);
            let key_id = U256::from_be_bytes(tenant_info.key_id);

            info!(
                handle = compact_hex(&handle),
                chain_id = tenant_info.chain_id,
                key_id = compact_hex(&tenant_info.key_id),
                ct64 = compact_hex(ciphertext64_digest.as_ref()),
                ct128 = compact_hex(ciphertext128_digest.as_ref()),
                "Adding ciphertext"
            );

            let txn_request = match &self.gas {
                Some(gas_limit) => ciphertext_manager
                    .addCiphertextMaterial(
                        handle_bytes32,
                        key_id,
                        ciphertext64_digest,
                        ciphertext128_digest,
                    )
                    .into_transaction_request()
                    .with_gas_limit(*gas_limit),
                None => ciphertext_manager
                    .addCiphertextMaterial(
                        handle_bytes32,
                        key_id,
                        ciphertext64_digest,
                        ciphertext128_digest,
                    )
                    .into_transaction_request(),
            };

            t.end();

            let operation = self.clone();
            join_set.spawn(async move {
                operation
                    .send_transaction(
                        &row.handle,
                        txn_request,
                        row.txn_limited_retries_count,
                        row.txn_unlimited_retries_count,
                        transaction_id,
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
