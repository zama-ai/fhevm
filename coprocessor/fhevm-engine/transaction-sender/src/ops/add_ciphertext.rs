use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use crate::{
    metrics::{ADD_CIPHERTEXT_MATERIAL_FAIL_COUNTER, ADD_CIPHERTEXT_MATERIAL_SUCCESS_COUNTER},
    nonce_managed_provider::NonceManagedProvider,
    REVIEW,
};

use super::common::{try_extract_non_retryable_config_error, try_into_array};
use super::{begin_live_write, TransactionOperation};
use alloy::{
    network::{Ethereum, TransactionBuilder},
    primitives::{Address, FixedBytes, U256},
    providers::Provider,
    rpc::types::{BlockNumberOrTag, TransactionRequest},
    transports::{RpcError, TransportErrorKind},
};
use anyhow::{bail, Context};
use async_trait::async_trait;
use fhevm_engine_common::{telemetry, utils::to_hex};
use sqlx::{Pool, Postgres, Row, Transaction};
use tokio::{
    sync::{Mutex, Semaphore},
    task::JoinSet,
};
use tracing::{error, info, warn};

use fhevm_gateway_bindings::ciphertext_commits::CiphertextCommits;
use fhevm_gateway_bindings::ciphertext_commits::CiphertextCommits::CiphertextCommitsErrors;

const RECENT_PUBLICATION_REORG_WINDOW_BLOCKS: u64 = 256;
const PUBLICATION_RECONCILIATION_INTERVAL: Duration = Duration::from_secs(60);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PublicationReceiptCheck {
    CanonicalSuccess(u64),
    CanonicalFailure,
    Orphaned,
    Indeterminate,
}

fn classify_publication_receipt(
    status: bool,
    block_number: Option<u64>,
    receipt_block_hash: Option<FixedBytes<32>>,
    canonical_block_hash: Option<FixedBytes<32>>,
) -> PublicationReceiptCheck {
    match (block_number, receipt_block_hash, canonical_block_hash) {
        (Some(number), Some(receipt_hash), Some(canonical_hash))
            if receipt_hash == canonical_hash && status =>
        {
            PublicationReceiptCheck::CanonicalSuccess(number)
        }
        (Some(_), Some(receipt_hash), Some(canonical_hash)) if receipt_hash == canonical_hash => {
            PublicationReceiptCheck::CanonicalFailure
        }
        (Some(_), Some(receipt_hash), Some(canonical_hash)) if receipt_hash != canonical_hash => {
            PublicationReceiptCheck::Orphaned
        }
        _ => PublicationReceiptCheck::Indeterminate,
    }
}

#[derive(Default)]
struct PublicationReconciliationState {
    cursor: Option<Vec<u8>>,
    sweep_in_progress: bool,
    next_sweep_at: Option<Instant>,
}

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
    submission_slots: Arc<Semaphore>,
    reconciliation_state: Arc<Mutex<PublicationReconciliationState>>,
}

impl<P> AddCiphertextOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    #[tracing::instrument(name = "call_add_ciphertext", skip_all, fields(txn_id = tracing::field::Empty))]
    async fn send_transaction(
        &self,
        handle: &[u8],
        txn_request: impl Into<TransactionRequest>,
        current_limited_retries_count: i32,
        current_unlimited_retries_count: i32,
        src_transaction_id: Option<Vec<u8>>,
    ) -> anyhow::Result<()> {
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
        // Keep the acceptance fence across submission. Release it before the
        // bookkeeping helpers open their own guarded transactions.
        let Some(submission_fence) = begin_live_write(&self.db_pool).await? else {
            return Ok(());
        };
        let submission = self
            .provider
            .send_sync_with_overprovision(
                txn_request,
                self.conf.gas_limit_overprovision_percent,
                Duration::from_secs(self.conf.send_txn_sync_timeout_secs.into()),
            )
            .await;
        submission_fence.rollback().await?;
        let receipt = match submission {
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
                        error = %crate::diagnostics::safe_rpc_error(&e),
                        handle = h,
                        "Transaction sending failed with unlimited retry error"
                    );
                    self.increment_txn_unlimited_retries_count(
                        handle,
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
                        handle,
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
                    handle,
                    &crate::diagnostics::safe_rpc_error(&e),
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
        let Some(mut tx) = begin_live_write(&self.db_pool).await? else {
            info!("Cutover completed — skipping post-send digest/ct128 writes on retired stack");
            return Ok(());
        };

        // Duplicate Gateway acknowledgments carry no receipt. Keep a receipt
        // already recorded by a concurrent replica, and replace the identity
        // only when this call has a successful receipt hash.
        sqlx::query!(
            "UPDATE ciphertext_digest
            SET
                txn_is_sent = true,
                txn_hash = CASE WHEN $1::bytea IS NULL THEN txn_hash ELSE $1::bytea END,
                txn_block_number = CASE WHEN $1::bytea IS NULL THEN txn_block_number ELSE $2::bigint END
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
        .await?;

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

    /// Reconcile recent receipt acknowledgments against canonical chain state.
    /// A canonical successful receipt preserves a sender's contribution even
    /// before quorum, when Gateway's public sender-list getter is empty. If the
    /// receipt is orphaned and Gateway has not finalized the handle through
    /// another canonical contribution, restore the row to the retry queue.
    async fn reconcile_recent_publications(&self) -> anyhow::Result<()> {
        let Some(sender_address) = self.provider.signer_address() else {
            return Ok(());
        };

        let mut state = self.reconciliation_state.lock().await;
        let now = Instant::now();
        if state.next_sweep_at.is_some_and(|next| now < next) {
            return Ok(());
        }

        if !state.sweep_in_progress {
            state.cursor = None;
            state.sweep_in_progress = true;
        }

        let latest_block = self.provider.get_block_number().await?;
        let first_reorgable_block =
            i64::try_from(latest_block.saturating_sub(RECENT_PUBLICATION_REORG_WINDOW_BLOCKS))
                .unwrap_or(i64::MAX);
        let limit = self.conf.add_ciphertexts_batch_limit.max(1) as i64;
        let rows = sqlx::query(
            "SELECT handle, txn_hash, txn_block_number, key_id_gw, ciphertext, ciphertext128
             FROM ciphertext_digest
             WHERE txn_is_sent = TRUE
               AND txn_hash IS NOT NULL
               AND (txn_block_number IS NULL OR txn_block_number >= $1)
               AND ($2::BYTEA IS NULL OR handle > $2)
             ORDER BY handle ASC
             LIMIT $3",
        )
        .bind(first_reorgable_block)
        .bind(state.cursor.clone())
        .bind(limit)
        .fetch_all(&self.db_pool)
        .await?;

        if rows.is_empty() {
            state.cursor = None;
            state.sweep_in_progress = false;
            state.next_sweep_at = Some(Instant::now() + PUBLICATION_RECONCILIATION_INTERVAL);
            return Ok(());
        }

        let commits =
            CiphertextCommits::new(self.ciphertext_commits_address, self.provider.inner());
        for row in &rows {
            let handle: Vec<u8> = row.try_get("handle")?;
            let txn_hash: Option<Vec<u8>> = row.try_get("txn_hash")?;
            let txn_block_number: Option<i64> = row.try_get("txn_block_number")?;
            let handle_bytes32 = FixedBytes::from(try_into_array::<32>(handle.clone())?);

            let txn_hash_bytes = txn_hash
                .as_deref()
                .context("reconciliation selected a sent publication without a transaction hash")?;
            let txn_hash_bytes32 = FixedBytes::from(try_into_array::<32>(txn_hash_bytes.to_vec())?);

            // The Gateway's public sender-list getter only exposes contributors
            // after quorum. A successful canonical receipt is therefore the
            // evidence that preserves a pre-quorum contribution across restart.
            let receipt = self
                .provider
                .inner()
                .get_transaction_receipt(txn_hash_bytes32)
                .await?;
            if let Some(receipt) = receipt.as_ref() {
                let canonical_block_hash = if let Some(block_number) = receipt.block_number {
                    self.provider
                        .inner()
                        .get_block_by_number(BlockNumberOrTag::Number(block_number))
                        .await?
                        .map(|block| block.header.hash)
                } else {
                    None
                };
                match classify_publication_receipt(
                    receipt.status(),
                    receipt.block_number,
                    receipt.block_hash,
                    canonical_block_hash,
                ) {
                    PublicationReceiptCheck::CanonicalSuccess(block_number) => {
                        let canonical_block_number = i64::try_from(block_number).context(
                            "canonical publication block number does not fit PostgreSQL BIGINT",
                        )?;
                        if txn_block_number != Some(canonical_block_number) {
                            self.update_canonical_publication_block(
                                &handle,
                                txn_hash_bytes,
                                txn_block_number,
                                canonical_block_number,
                            )
                            .await?;
                        }
                        state.cursor = Some(handle);
                        continue;
                    }
                    PublicationReceiptCheck::Indeterminate => {
                        // Missing receipt metadata or an unavailable canonical
                        // block is not evidence that the transaction was lost.
                        state.cursor = Some(handle);
                        continue;
                    }
                    PublicationReceiptCheck::CanonicalFailure
                    | PublicationReceiptCheck::Orphaned => {}
                }
            } else if self
                .provider
                .inner()
                .get_transaction_by_hash(txn_hash_bytes32)
                .await?
                .is_some()
            {
                // Keep rows for pending or mined transactions when the RPC
                // cannot provide a receipt. Retrying on an incomplete backend
                // view could duplicate an accepted Gateway contribution.
                state.cursor = Some(handle);
                continue;
            }

            // No canonical receipt remains. A finalized handle only protects
            // this row when the Gateway's exact key and digests match it; the
            // handle-level finalized flag alone can hide a conflicting
            // publication by another sender.
            let contributors = commits
                .getAddCiphertextMaterialConsensusTxSenders(handle_bytes32)
                .call()
                .await?;
            let material_is_finalized = commits
                .isCiphertextMaterialAdded(handle_bytes32)
                .call()
                .await?;

            if material_is_finalized {
                let regular = commits
                    .getCiphertextMaterials(vec![handle_bytes32])
                    .call()
                    .await?;
                let sns = commits
                    .getSnsCiphertextMaterials(vec![handle_bytes32])
                    .call()
                    .await?;
                let local_key_id: Option<Vec<u8>> = row.try_get("key_id_gw")?;
                let local_ciphertext: Option<Vec<u8>> = row.try_get("ciphertext")?;
                let local_ciphertext128: Option<Vec<u8>> = row.try_get("ciphertext128")?;
                let expected = match (local_key_id, local_ciphertext, local_ciphertext128) {
                    (Some(key_id), Some(ciphertext), Some(ciphertext128)) => Some((
                        U256::from_be_bytes(try_into_array::<32>(key_id)?),
                        FixedBytes::from(try_into_array::<32>(ciphertext)?),
                        FixedBytes::from(try_into_array::<32>(ciphertext128)?),
                    )),
                    _ => None,
                };
                let matches_local = match (expected, regular.as_slice(), sns.as_slice()) {
                    (Some((key_id, ciphertext, ciphertext128)), [regular], [sns]) => {
                        regular.ctHandle == handle_bytes32
                            && regular.keyId == key_id
                            && regular.ciphertextDigest == ciphertext
                            && sns.ctHandle == handle_bytes32
                            && sns.keyId == key_id
                            && sns.snsCiphertextDigest == ciphertext128
                    }
                    _ => false,
                };

                if !matches_local
                    && self
                        .record_finalized_material_conflict(
                            &handle,
                            txn_hash_bytes,
                            txn_block_number,
                        )
                        .await?
                {
                    warn!(
                        handle = %to_hex(&handle),
                        "Gateway finalized this handle with material that differs from the acknowledged local publication; retaining the row for operator investigation"
                    );
                }
            } else if !contributors.contains(&sender_address) {
                self.restore_retryable_publication(&handle, txn_hash.as_deref(), txn_block_number)
                    .await?;
            }
            state.cursor = Some(handle);
        }

        if rows.len() < limit as usize {
            state.cursor = None;
            state.sweep_in_progress = false;
            state.next_sweep_at = Some(Instant::now() + PUBLICATION_RECONCILIATION_INTERVAL);
        }
        Ok(())
    }

    async fn update_canonical_publication_block(
        &self,
        handle: &[u8],
        txn_hash: &[u8],
        old_block_number: Option<i64>,
        canonical_block_number: i64,
    ) -> anyhow::Result<()> {
        let Some(mut tx) = begin_live_write(&self.db_pool).await? else {
            return Ok(());
        };
        sqlx::query(
            "UPDATE ciphertext_digest
             SET txn_block_number = $1
             WHERE handle = $2
               AND txn_is_sent = TRUE
               AND txn_hash IS NOT DISTINCT FROM $3
               AND txn_block_number IS NOT DISTINCT FROM $4",
        )
        .bind(canonical_block_number)
        .bind(handle)
        .bind(txn_hash)
        .bind(old_block_number)
        .execute(tx.as_mut())
        .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn restore_retryable_publication(
        &self,
        handle: &[u8],
        txn_hash: Option<&[u8]>,
        txn_block_number: Option<i64>,
    ) -> anyhow::Result<()> {
        let Some(mut tx) = begin_live_write(&self.db_pool).await? else {
            return Ok(());
        };
        let updated = sqlx::query(
            "UPDATE ciphertext_digest
             SET txn_is_sent = FALSE, txn_hash = NULL, txn_block_number = NULL
             WHERE handle = $1
               AND txn_is_sent = TRUE
               AND txn_hash IS NOT DISTINCT FROM $2
               AND txn_block_number IS NOT DISTINCT FROM $3",
        )
        .bind(handle)
        .bind(txn_hash)
        .bind(txn_block_number)
        .execute(tx.as_mut())
        .await?
        .rows_affected();

        if updated > 0 {
            warn!(
                handle = %to_hex(handle),
                "Gateway no longer records the acknowledged publication; restoring it to the retry queue"
            );
            sqlx::query("SELECT pg_notify($1, '')")
                .bind(&self.conf.add_ciphertexts_db_channel)
                .execute(tx.as_mut())
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn record_finalized_material_conflict(
        &self,
        handle: &[u8],
        txn_hash: &[u8],
        txn_block_number: Option<i64>,
    ) -> anyhow::Result<bool> {
        const CONFLICT: &str = "gateway-finalized-material-mismatch";
        let Some(mut tx) = begin_live_write(&self.db_pool).await? else {
            return Ok(false);
        };
        let updated = sqlx::query(
            "UPDATE ciphertext_digest
             SET txn_last_error = $1, txn_last_error_at = NOW()
             WHERE handle = $2
               AND txn_is_sent = TRUE
               AND txn_hash IS NOT DISTINCT FROM $3
               AND txn_block_number IS NOT DISTINCT FROM $4
               AND txn_last_error IS DISTINCT FROM $1",
        )
        .bind(CONFLICT)
        .bind(handle)
        .bind(txn_hash)
        .bind(txn_block_number)
        .execute(tx.as_mut())
        .await?
        .rows_affected()
            > 0;
        tx.commit().await?;
        Ok(updated)
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
            reconciliation_state: Arc::new(Mutex::new(PublicationReconciliationState::default())),
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
        let Some(mut tx) = begin_live_write(&self.db_pool).await? else {
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
        handle: &[u8],
        err: &str,
        current_unlimited_retries_count: i32,
    ) -> anyhow::Result<()> {
        let Some(mut tx) = begin_live_write(&self.db_pool).await? else {
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
        handle: &[u8],
        error: &str,
    ) -> anyhow::Result<()> {
        let Some(mut tx) = begin_live_write(&self.db_pool).await? else {
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
        self.reconcile_recent_publications().await?;
        // Computed digests becoming non-NULL does not prove S3 publication.
        // Only a current postflight verification witness makes a row eligible.
        let rows = sqlx::query!(
            "
            SELECT handle, key_id_gw, ciphertext, ciphertext128, host_chain_id, txn_limited_retries_count, txn_unlimited_retries_count, transaction_id
            FROM ciphertext_digest
            WHERE txn_is_sent = false
            AND ciphertext IS NOT NULL
            AND ciphertext128 IS NOT NULL
            AND s3_publication_verified_at IS NOT NULL
            AND s3_publication_verified_digest IS NOT DISTINCT FROM ciphertext
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
            let _span =
                tracing::info_span!("prepare_add_ciphertext", txn_id = tracing::field::Empty);
            telemetry::record_short_hex_if_some(&_span, "txn_id", transaction_id.as_deref());
            let _enter = _span.enter();

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

            drop(_enter);
            drop(_span);

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

#[cfg(test)]
mod publication_receipt_tests {
    use super::{classify_publication_receipt, PublicationReceiptCheck};
    use alloy::primitives::FixedBytes;

    #[test]
    fn classifies_complete_receipts_against_the_canonical_block() {
        let receipt_hash = FixedBytes::from([0x11; 32]);
        let competing_hash = FixedBytes::from([0x22; 32]);

        assert_eq!(
            classify_publication_receipt(true, Some(42), Some(receipt_hash), Some(receipt_hash)),
            PublicationReceiptCheck::CanonicalSuccess(42)
        );
        assert_eq!(
            classify_publication_receipt(false, Some(42), Some(receipt_hash), Some(receipt_hash)),
            PublicationReceiptCheck::CanonicalFailure
        );
        assert_eq!(
            classify_publication_receipt(true, Some(42), Some(receipt_hash), Some(competing_hash)),
            PublicationReceiptCheck::Orphaned
        );
    }

    #[test]
    fn incomplete_receipt_or_canonical_block_data_is_indeterminate() {
        let hash = FixedBytes::from([0x11; 32]);

        for check in [
            classify_publication_receipt(true, None, Some(hash), Some(hash)),
            classify_publication_receipt(true, Some(42), None, Some(hash)),
            classify_publication_receipt(true, Some(42), Some(hash), None),
        ] {
            assert_eq!(check, PublicationReceiptCheck::Indeterminate);
        }
    }
}
