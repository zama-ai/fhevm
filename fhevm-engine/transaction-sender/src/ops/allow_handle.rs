use std::str::FromStr;

use crate::ops::common::try_into_array;

use super::TransactionOperation;
use alloy::{
    network::{Ethereum, TransactionBuilder},
    primitives::{Address, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
};
use anyhow::bail;
use async_trait::async_trait;
use fhevm_engine_common::{tenant_keys::query_tenant_info, utils::compact_hex};
use sqlx::{Pool, Postgres};
use tokio::task::JoinSet;
use tracing::{error, info};

sol!(
    #[sol(rpc)]
    ACLManager,
    "artifacts/ACLManager.sol/ACLManager.json"
);

#[derive(Clone)]
pub struct ACLManagerOperation<P: Provider<Ethereum> + Clone + 'static> {
    acl_manager_contract_address: Address,
    provider: P,
    conf: crate::ConfigSettings,
    gas: Option<u64>,
}

impl<P: Provider<Ethereum> + Clone + 'static> ACLManagerOperation<P> {
    /// Sends a transaction
    ///
    /// TODO: Refactor: Avoid code duplication
    async fn send_transaction(
        db_pool: Pool<Postgres>,
        provider: P,
        handle: Vec<u8>,
        txn_request: impl Into<TransactionRequest>,
    ) -> anyhow::Result<()> {
        let h = compact_hex(&handle);

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
                bail!("Transaction sending failed with error: {}", e);
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
                "UPDATE allowed_handles
                 SET txn_is_sent = true
                 WHERE handle = $1",
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

impl<P: Provider<Ethereum> + Clone + 'static> ACLManagerOperation<P> {
    pub fn new(
        acl_manager_contract_address: Address,
        provider: P,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
    ) -> Self {
        info!(
            "Creating ACLManagerOperation with gas: {} and ACLManager address: {}",
            gas.unwrap_or(0),
            acl_manager_contract_address,
        );

        Self {
            acl_manager_contract_address,
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
        info!("Updating retry count, handle {}", compact_hex(&handle));
        sqlx::query!(
            "UPDATE allowed_handles
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
impl<P> TransactionOperation<P> for ACLManagerOperation<P>
where
    P: alloy::providers::Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str {
        &self.conf.allow_handle_db_channel
    }

    async fn execute(&self, db_pool: &Pool<Postgres>) -> anyhow::Result<bool> {
        let rows = sqlx::query!(
            "
            SELECT handle, tenant_id, account_address
            FROM allowed_handles
            WHERE account_address IS NOT NULL
                AND TRIM(account_address) <> ''
                AND txn_is_sent = false
                AND txn_retry_count < $1
            LIMIT $2;
            ",
            self.conf.allow_handle_max_retries as i32,
            self.conf.allow_handle_batch_limit as i32,
        )
        .fetch_all(db_pool)
        .await?;

        let acl_manager: ACLManager::ACLManagerInstance<(), &P> =
            ACLManager::new(self.acl_manager_contract_address, &self.provider);

        info!("Selected {} rows to process", rows.len());

        let maybe_has_more_work = rows.len() == self.conf.allow_handle_batch_limit as usize;

        let mut join_set = JoinSet::new();
        for row in rows.into_iter() {
            let tenant = match query_tenant_info(db_pool, row.tenant_id).await {
                Ok(res) => res,
                Err(_) => {
                    error!(
                        "Failed to get chain_id for tenant
                    id: {}",
                        row.tenant_id
                    );
                    continue;
                }
            };

            let chain_id = tenant.chain_id;
            let handle: Vec<u8> = row.handle.clone();
            let h_as_hex = compact_hex(&handle);

            let account_addr = row.account_address;
            info!(
                "Allow handle: {}, chain_id: {}, account: {}",
                h_as_hex, chain_id, account_addr
            );

            let handle_u256 = U256::from_be_bytes(try_into_array::<32>(handle)?);
            let address = match Address::from_str(&account_addr) {
                Ok(addr) => addr,
                Err(e) => {
                    error!("Failed to parse address: {}, error: {}", account_addr, e);
                    continue;
                }
            };

            let txn_request = match &self.gas {
                Some(gas_limit) => acl_manager
                    .allowAccount(U256::from(chain_id), handle_u256, address)
                    .into_transaction_request()
                    .with_gas_limit(*gas_limit),
                None => acl_manager
                    .allowAccount(U256::from(chain_id), handle_u256, address)
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

        Ok(maybe_has_more_work)
    }
}
