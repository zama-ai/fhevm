use std::time::Duration;

use crate::{
    metrics::{ADD_CIPHERTEXT_MATERIAL_FAIL_COUNTER, ADD_CIPHERTEXT_MATERIAL_SUCCESS_COUNTER},
    nonce_managed_provider::NonceManagedProvider,
    REVIEW,
};

use super::common::{try_extract_non_retryable_config_error, try_into_array};
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
use fhevm_engine_common::{telemetry, utils::to_hex};
use sqlx::{Pool, Postgres};
use tokio::task::JoinSet;
use tracing::{error, info, warn};

use fhevm_gateway_bindings::ciphertext_commits::CiphertextCommits;
use fhevm_gateway_bindings::ciphertext_commits::CiphertextCommits::CiphertextCommitsErrors;

#[derive(Clone)]
pub struct AddCiphertextOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    ciphertext_commits_address: Address,
    provider: NonceManagedProvider<P>,
    conf: crate::ConfigSettings,
    gas: Option<u64>,
    db_pool: Pool<Postgres>,
}

impl<P> AddCiphertextOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    async fn send_transaction(
        &self,
        handle: &[u8],
        txn_request: impl Into<TransactionRequest>,
        current_limited_retries_count: i32,
        current_unlimited_retries_count: i32,
        src_transaction_id: Option<Vec<u8>>,
    ) -> anyhow::Result<()> {
        let h = to_hex(handle);

        info!(handle = h, "Processing transaction");
        let _t = telemetry::tracer("call_add_ciphertext", &src_transaction_id);

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
            Err(e) => {
                // Consider transport retryable errors, BackendGone and local usage errors as something that must be retried infinitely.
                // Local usage are included as they might be transient due to external AWS KMS signers.
                if matches!(&e, RpcError::Transport(inner) if inner.is_retry_err() || matches!(inner, TransportErrorKind::BackendGone))
                    || matches!(&e, RpcError::LocalUsageError(_))
                {
                    ADD_CIPHERTEXT_MATERIAL_FAIL_COUNTER.inc();
                    warn!(
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
                if let Some(non_retryable_config_error) = try_extract_non_retryable_config_error(&e)
                {
                    ADD_CIPHERTEXT_MATERIAL_FAIL_COUNTER.inc();
                    warn!(
                        error = %non_retryable_config_error,
                        handle = h,
                        "Non-retryable gateway coprocessor config error while adding ciphertext"
                    );
                    self.stop_retrying_add_ciphertext_on_config_error(
                        handle,
                        &non_retryable_config_error.to_string(),
                    )
                    .await?;
                    return Ok(());
                }
                ADD_CIPHERTEXT_MATERIAL_FAIL_COUNTER.inc();
                warn!(
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
            .and_then(|error| match error {
                CiphertextCommitsErrors::CoprocessorAlreadyAdded(c) => Some(c.txSender),
                _ => None,
            })
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

        // Delete the local 128-bit ciphertext after successful transaction
        // The db copy is no longer needed once the ciphertext commit has been added on-chain
        //
        // The deletion happens here but not in the SNS worker after upload because
        // here it is less probable that the deletion fails due to a race condition
        delete_ct128_from_db(&self.db_pool, handle.to_vec()).await?;

        if let Some(txn_hash) = src_transaction_id {
            telemetry::try_end_l1_transaction(&self.db_pool, &txn_hash).await?;
        }

        Ok(())
    }
}

impl<P> AddCiphertextOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
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
        let compact_hex_handle = to_hex(handle);
        if current_retry_count == self.conf.add_ciphertexts_max_retries - 1 {
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
        let compact_hex_handle = to_hex(handle);
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

    async fn stop_retrying_add_ciphertext_on_config_error(
        &self,
        handle: &[u8],
        error: &str,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE ciphertext_digest
            SET
                txn_limited_retries_count = $1,
                txn_last_error = $2,
                txn_last_error_at = NOW()
            WHERE handle = $3",
            self.conf.add_ciphertexts_max_retries,
            error,
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
            SELECT handle, key_id_gw, ciphertext, ciphertext128, host_chain_id, txn_limited_retries_count, txn_unlimited_retries_count, transaction_id
            FROM ciphertext_digest
            WHERE txn_is_sent = false
            AND ciphertext IS NOT NULL
            AND ciphertext128 IS NOT NULL
            AND txn_limited_retries_count < $1
            ORDER BY created_at ASC
            LIMIT $2",
            self.conf.add_ciphertexts_max_retries,
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

            let handle = row.handle.clone();

            let (ciphertext64_digest, ciphertext128_digest) =
                match (row.ciphertext, row.ciphertext128) {
                    (Some(ct), Some(ct128)) => (
                        FixedBytes::from(try_into_array::<32>(ct)?),
                        FixedBytes::from(try_into_array::<32>(ct128)?),
                    ),
                    _ => {
                        error!(handle = to_hex(&handle), "Missing ciphertext(s)");
                        continue;
                    }
                };

            let handle_bytes32 = FixedBytes::from(try_into_array::<32>(handle.clone())?);
            let key_id_gw_bytes32: [u8; 32] =
                row.key_id_gw.try_into().map_err(|bad: Vec<u8>| {
                    anyhow::anyhow!(
                        "Failed to convert key_id_gw to [u8; 32] (len={}): 0x{}",
                        bad.len(),
                        to_hex(&bad)
                    )
                })?;
            let key_id_gw = U256::from_be_bytes(key_id_gw_bytes32);

            info!(
                handle = to_hex(&handle),
                host_chain_id = row.host_chain_id,
                key_id_gw = to_hex(&key_id_gw_bytes32),
                ct64_digest = to_hex(ciphertext64_digest.as_ref()),
                ct128_digest = to_hex(ciphertext128_digest.as_ref()),
                "Adding ciphertext"
            );

            let txn_request = match &self.gas {
                Some(gas_limit) => ciphertext_manager
                    .addCiphertextMaterial(
                        handle_bytes32,
                        key_id_gw,
                        ciphertext64_digest,
                        ciphertext128_digest,
                    )
                    .into_transaction_request()
                    .with_gas_limit(*gas_limit),
                None => ciphertext_manager
                    .addCiphertextMaterial(
                        handle_bytes32,
                        key_id_gw,
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

/// Deletes the local record of a 128-bit ciphertext
async fn delete_ct128_from_db(
    pool: &sqlx::Pool<Postgres>,
    handle: Vec<u8>,
) -> Result<(), sqlx::Error> {
    let rows_affected = sqlx::query!("DELETE FROM ciphertexts128 WHERE  handle = $1", handle)
        .execute(pool)
        .await?
        .rows_affected();

    if rows_affected > 0 {
        info!(
            rows_affected,
            handle = to_hex(&handle),
            "Deleted local ct128"
        );
    }
    Ok(())
}
