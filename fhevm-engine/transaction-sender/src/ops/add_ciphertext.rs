use super::TransactionOperation;
use alloy::{
    network::{Ethereum, TransactionBuilder},
    primitives::{Address, FixedBytes, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
};
use async_trait::async_trait;
use fhevm_engine_common::{tenant_keys::query_tenant_info, utils::to_hex};
use sqlx::{Pool, Postgres};
use tokio::task::JoinSet;
use tracing::{error, info};

sol!(
    #[sol(rpc)]
    CiphertextManager,
    "artifacts/CiphertextManager.sol/CiphertextManager.json"
);

macro_rules! try_into_array {
    ($vec:expr, $size:expr) => {{
        if $vec.len() != $size {
            panic!("invalid len, expected {} but got {}", $size, $vec.len());
        }
        let arr: [u8; $size] = $vec.try_into().expect("Failed to convert Vec to array");
        arr
    }};
}

#[derive(Clone)]
pub struct AddCiphertextOperation<P: Provider<Ethereum> + Clone + 'static> {
    ciphertext_manager_address: Address,
    provider: P,
    conf: crate::ConfigSettings,
    gas: Option<u64>,
}

impl<P: Provider<Ethereum> + Clone + 'static> AddCiphertextOperation<P> {
    async fn send_transaction(
        db_pool: Pool<Postgres>,
        provider: P,
        handle: Vec<u8>,
        txn_request: impl Into<TransactionRequest>,
    ) -> anyhow::Result<()> {
        let h = to_hex(&handle);

        info!("Processing transaction, handle: {}", h);

        let txn_req = txn_request.into();
        let transaction = match provider.send_transaction(txn_req.clone()).await {
            Ok(txn) => txn,
            Err(e) => {
                error!(
                    "Transaction {:?} sending failed with error: {}, handle: {}",
                    txn_req, e, h
                );

                Self::increment_db_retry(&db_pool, handle, &e.to_string()).await?;
                return Ok(());
            }
        };

        // Here, we assume we are sending the transaction to a rollup, hence the
        // confirmations of 1.
        let receipt = match transaction
            .with_required_confirmations(1)
            .get_receipt()
            .await
        {
            Ok(receipt) => receipt,
            Err(e) => {
                error!("Getting receipt failed with error: {}", e);
                Self::increment_db_retry(&db_pool, handle, &e.to_string()).await?;
                return Err(anyhow::Error::new(e));
            }
        };

        if receipt.status() {
            sqlx::query!(
                "UPDATE ciphertext_digest
                SET txn_is_sent = true
                WHERE handle = $1 AND txn_is_sent = false",
                handle
            )
            .execute(&db_pool)
            .await?;

            info!(
                "Transaction {} succeeded, handle: {}",
                receipt.transaction_hash, h
            );
        } else {
            error!(
                "Transaction {} failed with status {}, handle: {}",
                receipt.transaction_hash,
                receipt.status(),
                h
            );

            Self::increment_db_retry(&db_pool, handle, "receipt status = false").await?;

            return Err(anyhow::anyhow!(
                "Transaction {} failed with status {}, handle: {}",
                receipt.transaction_hash,
                receipt.status(),
                h,
            ));
        }
        Ok(())
    }
}

impl<P: Provider<Ethereum> + Clone + 'static> AddCiphertextOperation<P> {
    pub fn new(
        ciphertext_manager_address: Address,
        provider: P,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
    ) -> Self {
        info!(
            "Creating AddCiphertextOperation with gas: {} and CiphertextManager address: {}",
            gas.unwrap_or(0),
            ciphertext_manager_address,
        );

        Self {
            ciphertext_manager_address,
            provider,
            conf,
            gas,
        }
    }

    async fn increment_db_retry(
        db_pool: &Pool<Postgres>,
        handle: Vec<u8>,
        err: &str,
    ) -> anyhow::Result<()> {
        info!("Updating retry count, handle {}", to_hex(&handle));
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
        .execute(db_pool)
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

    async fn execute(&self, db_pool: &Pool<Postgres>) -> anyhow::Result<bool> {
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
            self.conf.verify_proof_resp_max_retries as i64,
            self.conf.add_ciphertexts_batch_limit as i64,
        )
        .fetch_all(db_pool)
        .await?;

        let ciphertext_manager: CiphertextManager::CiphertextManagerInstance<(), &P> =
            CiphertextManager::new(self.ciphertext_manager_address, &self.provider);

        info!("Selected {} rows to process", rows.len());

        let mut join_set = JoinSet::new();
        for row in rows.into_iter() {
            let (key_id, chain_id) = match query_tenant_info(db_pool, row.tenant_id).await {
                Ok(res) => res,
                Err(_) => {
                    error!(
                        "Failed to get key_id for tenant
                    id: {}",
                        row.tenant_id
                    );
                    continue;
                }
            };

            let handle: Vec<u8> = row.handle.clone();
            let h_as_hex = to_hex(&handle);

            let (ciphertext64_digest, ciphertext128_digest) =
                match (row.ciphertext, row.ciphertext128) {
                    (Some(ct), Some(ct128)) => (
                        FixedBytes::from(try_into_array!(ct, 32)),
                        FixedBytes::from(try_into_array!(ct128, 32)),
                    ),
                    _ => {
                        error!("Missing ciphertext(s), handle {}", h_as_hex);
                        continue;
                    }
                };

            let handle_u256 = U256::from_be_bytes(try_into_array!(handle, 32));

            info!(
                "Adding ciphertext, handle: {}, chain_id: {}, key_id: {}, ct64: {}, ct128: {}",
                h_as_hex,
                chain_id,
                key_id,
                to_hex(ciphertext64_digest.as_ref()),
                to_hex(ciphertext128_digest.as_ref()),
            );

            let txn_request = match &self.gas {
                Some(gas_limit) => ciphertext_manager
                    .addCiphertextMaterial(
                        handle_u256,
                        U256::from(key_id),
                        U256::from(chain_id),
                        ciphertext64_digest,
                        ciphertext128_digest,
                    )
                    .into_transaction_request()
                    .with_gas_limit(*gas_limit),
                None => ciphertext_manager
                    .addCiphertextMaterial(
                        handle_u256,
                        U256::from(key_id),
                        U256::from(chain_id),
                        ciphertext64_digest,
                        ciphertext128_digest,
                    )
                    .into_transaction_request(),
            };

            let db_pool = db_pool.clone();
            let provider = self.provider.clone();
            let handle = row.handle;

            join_set.spawn(async move {
                Self::send_transaction(db_pool, provider, handle, txn_request).await
            });
        }

        while let Some(res) = join_set.join_next().await {
            res??;
        }

        Ok(false)
    }
}
