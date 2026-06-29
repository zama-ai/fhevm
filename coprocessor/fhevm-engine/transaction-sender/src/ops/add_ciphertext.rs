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
use sqlx::{Pool, Postgres, Transaction};
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
    async fn materialize_pending_legacy_ciphertext_digests(&self) -> anyhow::Result<u64> {
        let rows = sqlx::query!(
            r#"
            INSERT INTO ciphertext_digest_branch (
                tenant_id,
                handle,
                ciphertext,
                ciphertext128,
                txn_is_sent,
                txn_limited_retries_count,
                txn_last_error,
                txn_last_error_at,
                txn_unlimited_retries_count,
                ciphertext128_format,
                txn_hash,
                txn_block_number,
                transaction_id,
                created_at,
                host_chain_id,
                key_id_gw,
                s3_format_version,
                producer_block_hash,
                block_number,
                block_hash
            )
            SELECT
                d.tenant_id,
                d.handle,
                d.ciphertext,
                d.ciphertext128,
                d.txn_is_sent,
                d.txn_limited_retries_count,
                d.txn_last_error,
                d.txn_last_error_at,
                d.txn_unlimited_retries_count,
                d.ciphertext128_format,
                d.txn_hash,
                d.txn_block_number,
                d.transaction_id,
                d.created_at,
                d.host_chain_id,
                d.key_id_gw,
                d.s3_format_version,
                ''::BYTEA,
                NULL::BIGINT,
                ''::BYTEA
            FROM ciphertext_digest d
            WHERE d.txn_is_sent = false
              AND d.ciphertext IS NOT NULL
              AND d.ciphertext128 IS NOT NULL
              AND d.txn_limited_retries_count < $1
              AND NOT EXISTS (
                  SELECT 1
                  FROM ciphertext_digest_branch b
                  WHERE b.host_chain_id = d.host_chain_id
                    AND b.handle = d.handle
              )
            ORDER BY d.created_at ASC
            LIMIT $2
            ON CONFLICT (handle, producer_block_hash, block_hash) DO NOTHING
            "#,
            self.conf.add_ciphertexts_max_retries,
            self.conf.add_ciphertexts_batch_limit as i64,
        )
        .execute(&self.db_pool)
        .await?
        .rows_affected();

        if rows > 0 {
            info!(
                rows,
                "Materialized pending legacy ciphertext digests as branchless rows"
            );
        }

        Ok(rows)
    }

    #[tracing::instrument(name = "call_add_ciphertext", skip_all, fields(txn_id = tracing::field::Empty))]
    #[allow(clippy::too_many_arguments)]
    async fn send_transaction(
        &self,
        handle: &[u8],
        host_chain_id: i64,
        producer_block_hash: &[u8],
        block_hash: &[u8],
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
                self.set_txn_is_sent(
                    handle,
                    host_chain_id,
                    producer_block_hash,
                    block_hash,
                    None,
                    None,
                    src_transaction_id,
                )
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
                        host_chain_id,
                        producer_block_hash,
                        block_hash,
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
                        host_chain_id,
                        producer_block_hash,
                        block_hash,
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
                    host_chain_id,
                    producer_block_hash,
                    block_hash,
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
                host_chain_id,
                producer_block_hash,
                block_hash,
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
                host_chain_id,
                producer_block_hash,
                block_hash,
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

    #[allow(clippy::too_many_arguments)]
    async fn set_txn_is_sent(
        &self,
        handle: &[u8],
        host_chain_id: i64,
        producer_block_hash: &[u8],
        block_hash: &[u8],
        txn_hash: Option<&[u8]>,
        txn_block_number: Option<i64>,
        src_transaction_id: Option<Vec<u8>>,
    ) -> anyhow::Result<()> {
        // Cutover safety: both writes below target tables execute_cutover merges
        // into `public` (ciphertext_digest, ciphertexts128). Gate them behind the
        // shared cutover lock + retirement re-check so a retired BCS sender cannot
        // clobber a re-armed digest or delete the merged green ct128. These run
        // AFTER the on-chain send as plain DB writes, so no advisory lock is held
        // across the on-chain call. See versioning::cutover_gate.
        let Some(mut tx) =
            fhevm_engine_common::versioning::begin_write_guarded(&self.db_pool, self.conf.gcs_mode)
                .await?
        else {
            info!("Cutover completed — skipping post-send digest/ct128 writes on retired stack");
            return Ok(());
        };

        sqlx::query!(
            r#"
            WITH source AS (
                SELECT host_chain_id, handle, producer_block_hash, key_id_gw, ciphertext, ciphertext128
                FROM ciphertext_digest_branch
                WHERE host_chain_id = $3
                  AND handle = $4
                  AND producer_block_hash = $5
                  AND block_hash = $6
                LIMIT 1
             )
             UPDATE ciphertext_digest_branch d
             SET txn_is_sent = true,
                 txn_hash = $1,
                 txn_block_number = $2
             FROM source s
             WHERE d.host_chain_id = s.host_chain_id
               AND d.handle = s.handle
               AND d.producer_block_hash = s.producer_block_hash
               AND d.key_id_gw = s.key_id_gw
               AND d.ciphertext = s.ciphertext
               AND d.ciphertext128 = s.ciphertext128
               AND d.txn_is_sent = false
               AND d.ciphertext IS NOT NULL
               AND d.ciphertext128 IS NOT NULL
               AND (
                   (d.producer_block_hash = ''::bytea AND d.block_hash = ''::bytea)
                   OR EXISTS (
                       SELECT 1
                       FROM host_chain_blocks_valid b
                       WHERE b.chain_id = d.host_chain_id
                         AND b.block_hash = d.block_hash
                         AND b.block_status = 'finalized'
                   )
               )
            "#,
            txn_hash,
            txn_block_number,
            host_chain_id,
            handle,
            producer_block_hash,
            block_hash,
        )
        .execute(tx.as_mut())
        .await?;

        // Delete the local 128-bit ciphertext after successful transaction
        // The db copy is no longer needed once the ciphertext commit has been added on-chain
        //
        // The deletion happens here but not in the SNS worker after upload because
        // here it is less probable that the deletion fails due to a race condition
        delete_ct128_from_db(&mut tx, handle.to_vec(), producer_block_hash.to_vec()).await?;

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
        host_chain_id: i64,
        producer_block_hash: &[u8],
        block_hash: &[u8],
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
            r#"
            UPDATE ciphertext_digest_branch
             SET txn_limited_retries_count = txn_limited_retries_count + 1,
                 txn_last_error = $1,
                 txn_last_error_at = NOW()
             WHERE host_chain_id = $2
               AND handle = $3
               AND producer_block_hash = $4
               AND block_hash = $5
            "#,
            err,
            host_chain_id,
            handle,
            producer_block_hash,
            block_hash,
        )
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }

    async fn increment_txn_unlimited_retries_count(
        &self,
        handle: &[u8],
        host_chain_id: i64,
        producer_block_hash: &[u8],
        block_hash: &[u8],
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
            r#"
            UPDATE ciphertext_digest_branch
             SET txn_unlimited_retries_count = txn_unlimited_retries_count + 1,
                 txn_last_error = $1,
                 txn_last_error_at = NOW()
             WHERE host_chain_id = $2
               AND handle = $3
               AND producer_block_hash = $4
               AND block_hash = $5
            "#,
            err,
            host_chain_id,
            handle,
            producer_block_hash,
            block_hash,
        )
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }

    async fn stop_retrying_add_ciphertext_on_config_error(
        &self,
        handle: &[u8],
        host_chain_id: i64,
        producer_block_hash: &[u8],
        block_hash: &[u8],
        error: &str,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE ciphertext_digest_branch
             SET txn_limited_retries_count = $1,
                 txn_last_error = $2,
                 txn_last_error_at = NOW()
             WHERE host_chain_id = $3
               AND handle = $4
               AND producer_block_hash = $5
               AND block_hash = $6
            "#,
            self.conf.add_ciphertexts_max_retries,
            error,
            host_chain_id,
            handle,
            producer_block_hash,
            block_hash,
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
        let materialized_legacy_rows = self.materialize_pending_legacy_ciphertext_digests().await?;

        // The service responsible for populating the ciphertext_digest table must
        // ensure that ciphertext and ciphertext128 are non-null only after the
        // ciphertexts have been successfully uploaded to AWS S3 buckets.
        let rows = sqlx::query!(
            r#"
            WITH eligible AS (
                SELECT
                  d.handle,
                  d.producer_block_hash,
                  d.block_hash,
                  d.key_id_gw,
                  d.ciphertext,
                  d.ciphertext128,
                  d.host_chain_id,
                  d.txn_limited_retries_count,
                  d.txn_unlimited_retries_count,
                  d.transaction_id,
                  d.created_at,
                  ROW_NUMBER() OVER (
                      PARTITION BY d.host_chain_id, d.handle, d.producer_block_hash
                      ORDER BY d.created_at ASC
                  ) AS rn
                FROM ciphertext_digest_branch d
                WHERE d.txn_is_sent = false
                AND d.ciphertext IS NOT NULL
                AND d.ciphertext128 IS NOT NULL
                AND d.txn_limited_retries_count < $1
                AND (
                    (d.producer_block_hash = ''::bytea AND d.block_hash = ''::bytea)
                    OR EXISTS (
                        SELECT 1
                        FROM host_chain_blocks_valid b
                        WHERE b.chain_id = d.host_chain_id
                          AND b.block_hash = d.block_hash
                          AND b.block_status = 'finalized'
                    )
                )
            )
            SELECT
                handle AS "handle!",
                producer_block_hash AS "producer_block_hash!",
                block_hash AS "block_hash!",
                key_id_gw AS "key_id_gw!",
                ciphertext,
                ciphertext128,
                host_chain_id AS "host_chain_id!",
                txn_limited_retries_count AS "txn_limited_retries_count!",
                txn_unlimited_retries_count AS "txn_unlimited_retries_count!",
                transaction_id
            FROM eligible
            WHERE rn = 1
            ORDER BY created_at ASC
            LIMIT $2
            "#,
            self.conf.add_ciphertexts_max_retries,
            self.conf.add_ciphertexts_batch_limit as i64,
        )
        .fetch_all(&self.db_pool)
        .await?;

        let ciphertext_manager =
            CiphertextCommits::new(self.ciphertext_commits_address, self.provider.inner());

        info!(rows_count = rows.len(), "Selected rows to process");

        let maybe_has_more_work = rows.len() == self.conf.add_ciphertexts_batch_limit as usize
            || materialized_legacy_rows >= self.conf.add_ciphertexts_batch_limit as u64;

        let mut join_set = JoinSet::new();
        for row in rows.into_iter() {
            let handle = row.handle;
            let producer_block_hash_raw = row.producer_block_hash;
            let block_hash = row.block_hash;
            let key_id_gw_raw = row.key_id_gw;
            let ciphertext = row.ciphertext;
            let ciphertext128 = row.ciphertext128;
            let host_chain_id = row.host_chain_id;
            let txn_limited_retries_count = row.txn_limited_retries_count;
            let txn_unlimited_retries_count = row.txn_unlimited_retries_count;
            let transaction_id = row.transaction_id;

            let _span =
                tracing::info_span!("prepare_add_ciphertext", txn_id = tracing::field::Empty);
            telemetry::record_short_hex_if_some(&_span, "txn_id", transaction_id.as_deref());
            let _enter = _span.enter();

            let producer_block_hash = producer_block_hash_raw;

            let (ciphertext64_digest, ciphertext128_digest) = match (ciphertext, ciphertext128) {
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
                key_id_gw_raw.try_into().map_err(|bad: Vec<u8>| {
                    anyhow::anyhow!(
                        "Failed to convert key_id_gw to [u8; 32] (len={}): 0x{}",
                        bad.len(),
                        to_hex(&bad)
                    )
                })?;
            let key_id_gw = U256::from_be_bytes(key_id_gw_bytes32);

            info!(
                handle = to_hex(&handle),
                host_chain_id = host_chain_id,
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
                        &handle,
                        host_chain_id,
                        &producer_block_hash,
                        &block_hash,
                        txn_request,
                        txn_limited_retries_count,
                        txn_unlimited_retries_count,
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
    producer_block_hash: Vec<u8>,
) -> Result<(), sqlx::Error> {
    let rows_affected = sqlx::query!(
        r#"
        DELETE FROM ciphertexts128_branch
         WHERE handle = $1
           AND producer_block_hash = $2
           AND NOT EXISTS (
               SELECT 1
               FROM ciphertext_digest_branch d
               WHERE d.handle = ciphertexts128_branch.handle
                 AND d.producer_block_hash = ciphertexts128_branch.producer_block_hash
                 AND d.ciphertext128 IS NULL
           )
        "#,
        &handle,
        &producer_block_hash,
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
