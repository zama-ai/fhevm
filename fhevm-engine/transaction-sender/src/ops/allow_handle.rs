use std::{
    fmt::{Display, Formatter},
    str::FromStr,
    time::Duration,
};

use crate::{nonce_managed_provider::NonceManagedProvider, ops::common::try_into_array};

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
use fhevm_engine_common::{tenant_keys::query_tenant_info, types::AllowEvents, utils::compact_hex};
use sqlx::{Pool, Postgres};
use tokio::task::JoinSet;
use tracing::{debug, error, info};

sol!(
    #[sol(rpc)]
    ACLManager,
    "artifacts/ACLManager.sol/ACLManager.json"
);

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
pub struct ACLManagerOperation<P: Provider<Ethereum> + Clone + 'static> {
    acl_manager_contract_address: Address,
    provider: NonceManagedProvider<P>,
    conf: crate::ConfigSettings,
    gas: Option<u64>,
    db_pool: Pool<Postgres>,
}

impl<P: Provider<Ethereum> + Clone + 'static> ACLManagerOperation<P> {
    /// Sends a transaction
    ///
    /// TODO: Refactor: Avoid code duplication
    async fn send_transaction(
        &self,
        key: &Key,
        txn_request: impl Into<TransactionRequest>,
    ) -> anyhow::Result<()> {
        let h = compact_hex(&key.handle);

        info!("Processing transaction, handle: {}", h);

        let txn_req = txn_request.into();
        let transaction = match self.provider.send_transaction(txn_req.clone()).await {
            Ok(txn) => txn,
            Err(e) => {
                error!(
                    "Transaction {:?} sending failed with error: {}, handle: {}",
                    txn_req, e, h
                );

                self.increment_db_retry(key, &e.to_string()).await?;
                bail!("Transaction sending failed with error: {}", e);
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
                error!("Getting receipt failed with error: {}", e);
                self.increment_db_retry(key, &e.to_string()).await?;
                return Err(anyhow::Error::new(e));
            }
        };

        if receipt.status() {
            sqlx::query!(
                "UPDATE allowed_handles
                     SET txn_is_sent = true
                     WHERE handle = $1
                     AND account_address = $2
                     AND tenant_id = $3",
                key.handle,
                key.account_addr,
                key.tenant_id
            )
            .execute(&self.db_pool)
            .await?;

            info!("Allow txn: {} succeeded, {}", receipt.transaction_hash, key,);
        } else {
            error!(
                "allowAccount txn: {} failed with status {}, handle: {}",
                receipt.transaction_hash,
                receipt.status(),
                h
            );

            self.increment_db_retry(key, "receipt status = false")
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
}

impl<P: Provider<Ethereum> + Clone + 'static> ACLManagerOperation<P> {
    pub fn new(
        acl_manager_contract_address: Address,
        provider: NonceManagedProvider<P>,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
        db_pool: Pool<Postgres>,
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
            db_pool,
        }
    }

    async fn increment_db_retry(&self, key: &Key, err: &str) -> anyhow::Result<()> {
        debug!("Updating retry count, {}", &key);

        sqlx::query!(
            "UPDATE allowed_handles
            SET
            txn_retry_count = txn_retry_count + 1,
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
impl<P> TransactionOperation<P> for ACLManagerOperation<P>
where
    P: alloy::providers::Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str {
        &self.conf.allow_handle_db_channel
    }

    async fn execute(&self) -> anyhow::Result<bool> {
        let rows = sqlx::query!(
            "
            SELECT handle, tenant_id, account_address, event_type 
            FROM allowed_handles 
            WHERE txn_is_sent = false 
            AND txn_retry_count < $1
            LIMIT $2;
            ",
            self.conf.allow_handle_max_retries as i32,
            self.conf.allow_handle_batch_limit as i32,
        )
        .fetch_all(&self.db_pool)
        .await?;

        let acl_manager: ACLManager::ACLManagerInstance<(), &P> =
            ACLManager::new(self.acl_manager_contract_address, self.provider.inner());

        info!("Selected {} rows to process", rows.len());

        let maybe_has_more_work = rows.len() == self.conf.allow_handle_batch_limit as usize;

        let mut join_set = JoinSet::new();
        for row in rows.into_iter() {
            let tenant = match query_tenant_info(&self.db_pool, row.tenant_id).await {
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
            let event_type = match AllowEvents::try_from(row.event_type) {
                Ok(event_type) => event_type,
                Err(_) => {
                    error!(
                        "Invalid event_type: {} for tenant_id: {}",
                        row.event_type, row.tenant_id
                    );
                    continue;
                }
            };

            let account_addr = row.account_address;
            info!(
                "Allow handle: {}, event_type: {:?}, account: {:?}, chain_id: {},",
                h_as_hex, event_type, account_addr, chain_id,
            );

            let handle_bytes32 = FixedBytes::from(try_into_array::<32>(handle)?);

            let txn_request = match event_type {
                AllowEvents::AllowedForDecryption => {
                    // Call allowPublicDecrypt when account_address is null
                    match &self.gas {
                        Some(gas_limit) => acl_manager
                            .allowPublicDecrypt(U256::from(chain_id), handle_bytes32)
                            .into_transaction_request()
                            .with_gas_limit(*gas_limit),
                        None => acl_manager
                            .allowPublicDecrypt(U256::from(chain_id), handle_bytes32)
                            .into_transaction_request(),
                    }
                }
                AllowEvents::AllowedAccount => {
                    let address = if let Ok(addr) = Address::from_str(&account_addr) {
                        addr
                    } else {
                        error!(
                            "Invalid account address: {:?} for tenant_id: {}",
                            account_addr, row.tenant_id
                        );
                        continue;
                    };

                    match &self.gas {
                        Some(gas_limit) => acl_manager
                            .allowAccount(U256::from(chain_id), handle_bytes32, address)
                            .into_transaction_request()
                            .with_gas_limit(*gas_limit),
                        None => acl_manager
                            .allowAccount(U256::from(chain_id), handle_bytes32, address)
                            .into_transaction_request(),
                    }
                }
            };

            let handle = row.handle;

            let key = Key {
                handle,
                account_addr,
                tenant_id: row.tenant_id,
                event_type,
            };

            let operation = self.clone();
            join_set.spawn(async move { operation.send_transaction(&key, txn_request).await });
        }

        while let Some(res) = join_set.join_next().await {
            res??;
        }

        Ok(maybe_has_more_work)
    }
}
