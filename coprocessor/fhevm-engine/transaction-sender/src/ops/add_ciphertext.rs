use std::sync::Arc;

use crate::{
    metrics::{ADD_CIPHERTEXT_MATERIAL_FAIL_COUNTER, ADD_CIPHERTEXT_MATERIAL_SUCCESS_COUNTER},
    nonce_managed_provider::NonceManagedProvider,
    REVIEW,
};

use super::common::{
    is_retryable_nonce_error, try_extract_non_retryable_config_error, try_into_array,
};
use super::state::{Attempt, Status, Task};
use super::{begin_live_write, TransactionOperation};
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
use sqlx::{Pool, Postgres, Transaction};
use tokio::{sync::Semaphore, task::JoinSet};
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
    signer: crate::AbstractSigner,
    conf: crate::ConfigSettings,
    gas: Option<u64>,
    db_pool: Pool<Postgres>,
    submission_slots: Arc<Semaphore>,
}

impl<P> AddCiphertextOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    #[tracing::instrument(name = "call_add_ciphertext", skip_all, fields(txn_id = tracing::field::Empty))]
    async fn send_transaction(
        &self,
        mut attempt: Attempt,
        txn_request: impl Into<TransactionRequest>,
        current_limited_retries_count: i32,
        current_unlimited_retries_count: i32,
        src_transaction_id: Option<Vec<u8>>,
    ) -> anyhow::Result<()> {
        let Task::Ciphertext(handle) = &attempt.task.clone() else {
            unreachable!()
        };
        telemetry::record_short_hex_if_some(
            &tracing::Span::current(),
            "txn_id",
            src_transaction_id.as_deref(),
        );
        let h = to_hex(handle);

        info!(handle = h, "Processing transaction");

        // Share admission with proof responses and retain it through bookkeeping.
        // Waiting tasks must not hold connections or start pool-acquisition timers.
        let _permit = self.submission_slots.acquire().await?;
        let Some(mut submission_fence) = begin_live_write(&self.db_pool).await? else {
            return Ok(());
        };
        let submission = attempt
            .submit(
                &self.db_pool,
                &mut submission_fence,
                &self.provider,
                self.signer.clone(),
                txn_request.into(),
                &self.conf,
            )
            .await;
        submission_fence.rollback().await?;
        let receipt = match submission? {
            Ok(receipt) => receipt,
            Err(e) if self.already_added_error(&e).is_some() => {
                warn!(
                    handle = h,
                    address = ?self.already_added_error(&e),
                    "Coprocessor has already added the ciphertext commit",
                );
                self.set_txn_is_sent(&attempt, None, None, src_transaction_id)
                    .await?;
                return Ok(());
            }
            Err(e) => {
                // Nonce recovery must not consume the job's finite retry budget.
                // Keep the existing transport and local signer retry policies too.
                // Local usage are included as they might be transient due to external AWS KMS signers.
                if matches!(&e, RpcError::Transport(inner) if inner.is_retry_err() || matches!(inner, TransportErrorKind::BackendGone))
                    || matches!(&e, RpcError::LocalUsageError(_))
                    || is_retryable_nonce_error(&e)
                    || attempt.raw.is_some()
                {
                    ADD_CIPHERTEXT_MATERIAL_FAIL_COUNTER.inc();
                    warn!(
                        error = %crate::diagnostics::safe_rpc_error(&e),
                        handle = h,
                        "Transaction sending failed with unlimited retry error"
                    );
                    self.increment_txn_unlimited_retries_count(
                        &attempt,
                        &crate::diagnostics::safe_rpc_error(&e),
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
                        &attempt,
                        &non_retryable_config_error.to_string(),
                    )
                    .await?;
                    return Ok(());
                }
                ADD_CIPHERTEXT_MATERIAL_FAIL_COUNTER.inc();
                warn!(
                    error = %crate::diagnostics::safe_rpc_error(&e),
                    handle = h,
                    "Transaction sending failed"
                );
                self.increment_txn_limited_retries_count(
                    &attempt,
                    &crate::diagnostics::safe_rpc_error(&e),
                    current_limited_retries_count,
                )
                .await?;
                bail!(e);
            }
        };

        if receipt.status() {
            self.set_txn_is_sent(
                &attempt,
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
                &attempt,
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
        attempt: &Attempt,
        txn_hash: Option<&[u8]>,
        txn_block_number: Option<i64>,
        src_transaction_id: Option<Vec<u8>>,
    ) -> anyhow::Result<()> {
        let Task::Ciphertext(handle) = &attempt.task.clone() else {
            unreachable!()
        };
        let Some(mut tx) = attempt
            .transition(&self.db_pool, Status::Completed, None, 0)
            .await?
        else {
            info!("Cutover completed — skipping post-send digest/ct128 writes on retired stack");
            return Ok(());
        };

        let changed = sqlx::query!(
            "UPDATE ciphertext_digest
            SET
                txn_is_sent = true,
                txn_hash = $1,
                txn_block_number = $2
            WHERE handle = $3
              AND ciphertext IS NOT NULL
              AND ciphertext128 IS NOT NULL
              AND s3_publication_verified_at IS NOT NULL
              AND s3_publication_verified_digest IS NOT DISTINCT FROM ciphertext",
            txn_hash,
            txn_block_number,
            handle
        )
        .execute(tx.as_mut())
        .await?
        .rows_affected();
        anyhow::ensure!(
            changed == 1,
            "ciphertext publication evidence changed during submission"
        );

        // Delete the local 128-bit ciphertext after successful transaction
        // The db copy is no longer needed once the ciphertext commit has been added on-chain
        //
        // The deletion happens here but not in the SNS worker after upload because
        // here it is less probable that the deletion fails due to a race condition
        delete_ct128_from_db(&mut tx, handle.to_vec()).await?;

        tx.commit().await?;

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
        signer: crate::AbstractSigner,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
        db_pool: Pool<Postgres>,
        submission_slots: Arc<Semaphore>,
    ) -> Self {
        info!(
            gas = gas.unwrap_or(0),
            ciphertext_commits_address = %ciphertext_commits_address,
            "Creating AddCiphertextOperation"
        );

        Self {
            db_pool,
            submission_slots,
            ciphertext_commits_address,
            provider,
            signer,
            conf,
            gas,
        }
    }

    async fn increment_txn_limited_retries_count(
        &self,
        attempt: &Attempt,
        err: &str,
        current_retry_count: i32,
    ) -> anyhow::Result<()> {
        let Task::Ciphertext(handle) = &attempt.task.clone() else {
            unreachable!()
        };
        let status = if current_retry_count + 1 >= self.conf.add_ciphertexts_max_retries {
            Status::Stopped
        } else {
            Status::RetryWait
        };
        let Some(mut tx) = attempt
            .transition(
                &self.db_pool,
                status,
                Some(err),
                self.conf.error_sleep_max_secs,
            )
            .await?
        else {
            return Ok(());
        };
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
        .execute(tx.as_mut())
        .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn increment_txn_unlimited_retries_count(
        &self,
        attempt: &Attempt,
        err: &str,
        current_unlimited_retries_count: i32,
    ) -> anyhow::Result<()> {
        let Task::Ciphertext(handle) = &attempt.task.clone() else {
            unreachable!()
        };
        let Some(mut tx) = attempt
            .transition(
                &self.db_pool,
                attempt.retry_status(),
                Some(err),
                self.conf.error_sleep_max_secs,
            )
            .await?
        else {
            return Ok(());
        };
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
        .execute(tx.as_mut())
        .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn stop_retrying_add_ciphertext_on_config_error(
        &self,
        attempt: &Attempt,
        error: &str,
    ) -> anyhow::Result<()> {
        let Task::Ciphertext(handle) = &attempt.task.clone() else {
            unreachable!()
        };
        let Some(mut tx) = attempt
            .transition(&self.db_pool, Status::Stopped, Some(error), 0)
            .await?
        else {
            return Ok(());
        };
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
        .execute(tx.as_mut())
        .await?;
        tx.commit().await?;
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
        // Initialize terminal state for legacy jobs and jobs affected by a lower configured cap.
        if let Some(mut tx) = begin_live_write(&self.db_pool).await? {
            sqlx::query!(
                "UPDATE ciphertext_digest SET sender_status = 'STOPPED',
                sender_status_updated_at = NOW(), sender_next_action_at = NULL,
                sender_last_error = COALESCE(txn_last_error, 'retry budget exhausted')
                WHERE txn_is_sent = FALSE AND txn_limited_retries_count >= $1
                  AND sender_status IN ('QUEUED', 'RETRY_WAIT') AND sender_raw_tx IS NULL",
                self.conf.add_ciphertexts_max_retries
            )
            .execute(tx.as_mut())
            .await?;
            tx.commit().await?;
        }
        // Computed digests becoming non-NULL does not prove S3 publication.
        // Only a current postflight verification witness makes a row eligible.
        let rows = sqlx::query!(
            "
            SELECT handle, key_id_gw, ciphertext, ciphertext128, host_chain_id, txn_limited_retries_count, txn_unlimited_retries_count, transaction_id, sender_attempt_no
            FROM ciphertext_digest
            WHERE sender_status IN ('QUEUED', 'RETRY_WAIT', 'AWAITING_OUTCOME')
            AND (sender_next_action_at IS NULL OR sender_next_action_at <= NOW())
            AND (sender_raw_tx IS NOT NULL OR (txn_is_sent = false
            AND ciphertext IS NOT NULL
            AND ciphertext128 IS NOT NULL
            AND s3_publication_verified_at IS NOT NULL
            AND s3_publication_verified_digest IS NOT DISTINCT FROM ciphertext
            AND txn_limited_retries_count < $1))
            ORDER BY created_at ASC
            LIMIT $2",
            self.conf.add_ciphertexts_max_retries,
            self.conf.add_ciphertexts_batch_limit as i64,
        )
        .fetch_all(&self.db_pool)
        .await?;

        info!(rows_count = rows.len(), "Selected rows to process");

        let maybe_has_more_work = rows.len() == self.conf.add_ciphertexts_batch_limit as usize;

        let mut join_set = JoinSet::new();
        for row in rows.into_iter() {
            let operation = self.clone();
            join_set.spawn(async move {
                let Some(attempt) = Attempt::claim(
                    &operation.db_pool,
                    Task::Ciphertext(row.handle.clone()),
                    row.sender_attempt_no,
                )
                .await?
                else {
                    return Ok(());
                };
                let result: anyhow::Result<()> = async {
                    let ciphertext_manager = CiphertextCommits::new(
                        operation.ciphertext_commits_address,
                        operation.provider.inner(),
                    );
                    let transaction_id = row.transaction_id.clone();
                    let _span = tracing::info_span!(
                        "prepare_add_ciphertext",
                        txn_id = tracing::field::Empty
                    );
                    telemetry::record_short_hex_if_some(
                        &_span,
                        "txn_id",
                        transaction_id.as_deref(),
                    );
                    let _enter = _span.enter();

                    let txn_request = if attempt.raw.is_some() {
                        TransactionRequest::default()
                    } else {
                        let handle = row.handle.clone();

                        let (ciphertext64_digest, ciphertext128_digest) =
                            match (row.ciphertext, row.ciphertext128) {
                                (Some(ct), Some(ct128)) => (
                                    FixedBytes::from(try_into_array::<32>(ct)?),
                                    FixedBytes::from(try_into_array::<32>(ct128)?),
                                ),
                                _ => {
                                    error!(handle = to_hex(&handle), "Missing ciphertext(s)");
                                    return Ok(());
                                }
                            };

                        let handle_bytes32 =
                            FixedBytes::from(try_into_array::<32>(handle.clone())?);
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

                        let txn_request = match &operation.gas {
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

                        txn_request
                    };
                    drop(_enter);
                    drop(_span);
                    operation
                        .send_transaction(
                            attempt.clone(),
                            txn_request,
                            row.txn_limited_retries_count,
                            row.txn_unlimited_retries_count,
                            transaction_id,
                        )
                        .await
                }
                .await;
                if let Err(error) = &result {
                    if let Some(tx) = attempt
                        .transition(
                            &operation.db_pool,
                            Status::AwaitingOutcome,
                            Some(&crate::diagnostics::safe_error(error.as_ref())),
                            operation.conf.error_sleep_max_secs,
                        )
                        .await?
                    {
                        tx.commit().await?;
                    }
                }
                result
            });
        }

        // Let every submission settle its persisted attempt and nonce before returning an error.
        // Dropping the JoinSet here could leave other nonce reservations locked.
        let mut first_error = None;
        while let Some(res) = join_set.join_next().await {
            if let Err(error) = res.map_err(anyhow::Error::from).and_then(|result| result) {
                if first_error.is_none() || crate::is_backend_gone(&error) {
                    first_error = Some(error);
                }
            }
        }
        if let Some(error) = first_error {
            return Err(error);
        }

        Ok(maybe_has_more_work)
    }
}

/// Deletes the local record of a 128-bit ciphertext
async fn delete_ct128_from_db(
    tx: &mut Transaction<'_, Postgres>,
    handle: Vec<u8>,
) -> Result<(), sqlx::Error> {
    let rows_affected = sqlx::query!(
        "DELETE FROM ciphertexts128 c
         WHERE c.handle = $1
           AND EXISTS (
               SELECT 1
                 FROM ciphertext_digest d
                WHERE d.handle = c.handle
                  AND d.ciphertext IS NOT NULL
                  AND d.ciphertext128 IS NOT NULL
                  AND d.ciphertext128_format IS NOT NULL
                  AND d.s3_publication_verified_at IS NOT NULL
                  AND d.s3_publication_verified_digest IS NOT DISTINCT FROM d.ciphertext
           )",
        handle
    )
    .execute(tx.as_mut())
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
