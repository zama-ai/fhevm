use std::{
    fmt::{Display, Formatter},
    str::FromStr,
    time::Duration,
};

use crate::{
    nonce_managed_provider::NonceManagedProvider, ops::common::try_into_array, ALLOW_HANDLES_TARGET,
};

use super::TransactionOperation;
use alloy::{
    network::{Ethereum, TransactionBuilder},
    primitives::{Address, FixedBytes},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
};
use anyhow::bail;
use async_trait::async_trait;
use fhevm_engine_common::{tenant_keys::query_tenant_info, types::AllowEvents, utils::compact_hex};
use sqlx::{Pool, Postgres};
use tokio::task::JoinSet;
use tracing::{debug, error, info, warn};
use MultichainAcl::MultichainAclErrors;

sol!(
    #[sol(rpc)]
    MultichainAcl,
    "artifacts/MultichainAcl.sol/MultichainAcl.json"
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
pub struct MultichainAclOperation<P: Provider<Ethereum> + Clone + 'static> {
    multichain_acl_address: Address,
    provider: NonceManagedProvider<P>,
    conf: crate::ConfigSettings,
    gas: Option<u64>,
    db_pool: Pool<Postgres>,
}

impl<P: Provider<Ethereum> + Clone + 'static> MultichainAclOperation<P> {
    /// Sends a transaction
    ///
    /// TODO: Refactor: Avoid code duplication
    async fn send_transaction(
        &self,
        key: &Key,
        txn_request: impl Into<TransactionRequest>,
    ) -> anyhow::Result<()> {
        let h = compact_hex(&key.handle);

        info!(target: ALLOW_HANDLES_TARGET, "Processing transaction, handle: {}", h);

        let txn_req = txn_request.into();
        let transaction = match self.provider.send_transaction(txn_req.clone()).await {
            Ok(txn) => txn,
            Err(e) => {
                if let Some(MultichainAclErrors::CoprocessorAlreadyAllowed(_)) = e
                    .as_error_resp()
                    .and_then(|payload| payload.as_decoded_error::<MultichainAclErrors>(true))
                {
                    warn!(target: ALLOW_HANDLES_TARGET, "Coprocessor has already added the ACL entry for handle: {}", h);
                    self.set_txn_is_sent(key).await?;
                    return Ok(());
                }

                error!(target: ALLOW_HANDLES_TARGET,
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
                error!(target: ALLOW_HANDLES_TARGET, "Getting receipt failed with error: {}", e);
                self.increment_db_retry(key, &e.to_string()).await?;
                return Err(anyhow::Error::new(e));
            }
        };

        if receipt.status() {
            self.set_txn_is_sent(key).await?;

            info!(target: ALLOW_HANDLES_TARGET, "Allow txn: {} succeeded, {}", receipt.transaction_hash, key,);
        } else {
            error!(target: ALLOW_HANDLES_TARGET,
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

    async fn set_txn_is_sent(&self, key: &Key) -> anyhow::Result<()> {
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
        Ok(())
    }
}

impl<P: Provider<Ethereum> + Clone + 'static> MultichainAclOperation<P> {
    pub fn new(
        multichain_acl_address: Address,
        provider: NonceManagedProvider<P>,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
        db_pool: Pool<Postgres>,
    ) -> Self {
        info!(target: ALLOW_HANDLES_TARGET,
            "Creating MultichainAclOperation with gas: {} and MultichainAcl address: {}",
            gas.unwrap_or(0),
            multichain_acl_address,
        );

        Self {
            multichain_acl_address,
            provider,
            conf,
            gas,
            db_pool,
        }
    }

    async fn increment_db_retry(&self, key: &Key, err: &str) -> anyhow::Result<()> {
        debug!(target: ALLOW_HANDLES_TARGET, "Updating retry count, {}", &key);

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
impl<P> TransactionOperation<P> for MultichainAclOperation<P>
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

        let multichain_acl: MultichainAcl::MultichainAclInstance<(), &P> =
            MultichainAcl::new(self.multichain_acl_address, self.provider.inner());

        info!(target: ALLOW_HANDLES_TARGET, "Selected {} rows to process", rows.len());

        let maybe_has_more_work = rows.len() == self.conf.allow_handle_batch_limit as usize;

        let mut join_set = JoinSet::new();
        for row in rows.into_iter() {
            let tenant = match query_tenant_info(&self.db_pool, row.tenant_id).await {
                Ok(res) => res,
                Err(_) => {
                    error!(target: ALLOW_HANDLES_TARGET,
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
                    error!(target: ALLOW_HANDLES_TARGET,
                        "Invalid event_type: {} for tenant_id: {}",
                        row.event_type, row.tenant_id
                    );
                    continue;
                }
            };

            let account_addr = row.account_address;
            info!(target: ALLOW_HANDLES_TARGET,
                "Allow handle: {}, event_type: {:?}, account: {:?}, chain_id: {},",
                h_as_hex, event_type, account_addr, chain_id,
            );

            let handle_bytes32 = FixedBytes::from(try_into_array::<32>(handle)?);

            let txn_request = match event_type {
                AllowEvents::AllowedForDecryption => {
                    // Call allowPublicDecrypt when account_address is null
                    match &self.gas {
                        Some(gas_limit) => multichain_acl
                            .allowPublicDecrypt(handle_bytes32)
                            .into_transaction_request()
                            .with_gas_limit(*gas_limit),
                        None => multichain_acl
                            .allowPublicDecrypt(handle_bytes32)
                            .into_transaction_request(),
                    }
                }
                AllowEvents::AllowedAccount => {
                    let address = if let Ok(addr) = Address::from_str(&account_addr) {
                        addr
                    } else {
                        error!(target: ALLOW_HANDLES_TARGET,
                            "Invalid account address: {:?} for tenant_id: {}",
                            account_addr, row.tenant_id
                        );
                        continue;
                    };

                    match &self.gas {
                        Some(gas_limit) => multichain_acl
                            .allowAccount(handle_bytes32, address)
                            .into_transaction_request()
                            .with_gas_limit(*gas_limit),
                        None => multichain_acl
                            .allowAccount(handle_bytes32, address)
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
