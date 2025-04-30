use std::time::Duration;

use crate::{nonce_managed_provider::NonceManagedProvider, ADD_CIPHERTEXTS_TARGET};

use super::common::try_into_array;
use super::TransactionOperation;
use alloy::{
    network::{Ethereum, TransactionBuilder},
    primitives::{Address, FixedBytes, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
};
use anyhow::bail;
use async_trait::async_trait;
use fhevm_engine_common::{tenant_keys::query_tenant_info, utils::compact_hex};
use sqlx::{Pool, Postgres};
use tokio::task::JoinSet;
use tracing::{error, info, warn};
use CiphertextCommits::CiphertextCommitsErrors;

sol!(
    #[sol(rpc)]
    CiphertextCommits,
    "artifacts/CiphertextCommits.sol/CiphertextCommits.json"
);

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
        handle: Vec<u8>,
        txn_request: impl Into<TransactionRequest>,
    ) -> anyhow::Result<()> {
        let h = compact_hex(&handle);

        info!(target: ADD_CIPHERTEXTS_TARGET, "Processing transaction, handle: {}", h);

        let txn_req = txn_request.into();
        let transaction = match self.provider.send_transaction(txn_req.clone()).await {
            Ok(txn) => txn,
            Err(e) => {
                if let Some(CiphertextCommitsErrors::CoprocessorTxSenderAlreadyAdded(_)) = e
                    .as_error_resp()
                    .and_then(|payload| payload.as_decoded_error::<CiphertextCommitsErrors>(true))
                {
                    warn!(target: ADD_CIPHERTEXTS_TARGET, "Coprocessor has already added the ciphertext commit for handle: {}", h);
                    self.set_txn_is_sent(handle).await?;
                    return Ok(());
                } else {
                    error!(target: ADD_CIPHERTEXTS_TARGET,
                        "Transaction {:?} sending failed with error: {}, handle: {}",
                        txn_req, e, h
                    );
                    self.increment_db_retry(handle, &e.to_string()).await?;
                    bail!("Transaction sending failed with error: {}", e);
                }
            }
        };

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
                error!(target: ADD_CIPHERTEXTS_TARGET, "Getting receipt failed with error: {}", e);
                self.increment_db_retry(handle, &e.to_string()).await?;
                return Err(anyhow::Error::new(e));
            }
        };

        if receipt.status() {
            self.set_txn_is_sent(handle).await?;
            info!(target: ADD_CIPHERTEXTS_TARGET,
                "addCiphertext txn: {} succeeded, handle: {}",
                receipt.transaction_hash, h
            );
        } else {
            error!(target: ADD_CIPHERTEXTS_TARGET,
                "addCiphertext txn: {} failed with status {}, handle: {}",
                receipt.transaction_hash,
                receipt.status(),
                h
            );

            self.increment_db_retry(handle, "receipt status = false")
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

    async fn set_txn_is_sent(&self, handle: Vec<u8>) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE ciphertext_digest
            SET txn_is_sent = true
            WHERE handle = $1",
            handle,
        )
        .execute(&self.db_pool)
        .await?;
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
        info!(target: ADD_CIPHERTEXTS_TARGET,
            "Creating AddCiphertextOperation with gas: {} and CiphertextCommits address: {}",
            gas.unwrap_or(0),
            ciphertext_commits_address,
        );

        Self {
            db_pool,
            ciphertext_commits_address,
            provider,
            conf,
            gas,
        }
    }

    async fn increment_db_retry(&self, handle: Vec<u8>, err: &str) -> anyhow::Result<()> {
        info!(target: ADD_CIPHERTEXTS_TARGET, "Updating retry count, handle {}", compact_hex(&handle));
        sqlx::query!(
            "UPDATE ciphertext_digest
            SET
            txn_retry_count = txn_retry_count + 1,
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
            SELECT handle, ciphertext, ciphertext128, tenant_id
            FROM ciphertext_digest
            WHERE txn_is_sent = false
            AND ciphertext IS NOT NULL
            AND ciphertext128 IS NOT NULL
            AND txn_retry_count < $1
            LIMIT $2",
            self.conf.add_ciphertexts_max_retries as i64,
            self.conf.add_ciphertexts_batch_limit as i64,
        )
        .fetch_all(&self.db_pool)
        .await?;

        let ciphertext_manager: CiphertextCommits::CiphertextCommitsInstance<(), &P> =
            CiphertextCommits::new(self.ciphertext_commits_address, self.provider.inner());

        info!(target: ADD_CIPHERTEXTS_TARGET, "Selected {} rows to process", rows.len());

        let maybe_has_more_work = rows.len() == self.conf.add_ciphertexts_batch_limit as usize;

        let mut join_set = JoinSet::new();
        for row in rows.into_iter() {
            let tenant_info = match query_tenant_info(&self.db_pool, row.tenant_id).await {
                Ok(res) => res,
                Err(_) => {
                    error!(target: ADD_CIPHERTEXTS_TARGET,
                        "Failed to get key_id for tenant
                    id: {}",
                        row.tenant_id
                    );
                    continue;
                }
            };

            let handle: Vec<u8> = row.handle.clone();

            let (ciphertext64_digest, ciphertext128_digest) = match (
                row.ciphertext,
                row.ciphertext128,
            ) {
                (Some(ct), Some(ct128)) => (
                    FixedBytes::from(try_into_array::<32>(ct)?),
                    FixedBytes::from(try_into_array::<32>(ct128)?),
                ),
                _ => {
                    error!(target: ADD_CIPHERTEXTS_TARGET, "Missing ciphertext(s), handle {}", compact_hex(&handle));
                    continue;
                }
            };

            let handle_bytes32 = FixedBytes::from(try_into_array::<32>(handle.clone())?);
            let key_id = U256::from_be_bytes(tenant_info.key_id);

            info!(target: ADD_CIPHERTEXTS_TARGET,
                "Adding ciphertext, handle: {}, chain_id: {}, key_id: {}, ct64: {}, ct128: {}",
                compact_hex(&handle),
                tenant_info.chain_id,
                compact_hex(&tenant_info.key_id),
                compact_hex(ciphertext64_digest.as_ref()),
                compact_hex(ciphertext128_digest.as_ref()),
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

            let operation = self.clone();
            join_set
                .spawn(async move { operation.send_transaction(row.handle, txn_request).await });
        }

        while let Some(res) = join_set.join_next().await {
            res??;
        }

        Ok(maybe_has_more_work)
    }
}
