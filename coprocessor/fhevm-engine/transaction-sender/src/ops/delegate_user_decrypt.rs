use std::collections::{HashMap, HashSet};
use std::{ops::DerefMut, time::Duration};

use crate::{
    metrics::{DELEGATE_USER_DECRYPT_FAIL_COUNTER, DELEGATE_USER_DECRYPT_SUCCESS_COUNTER},
    nonce_managed_provider::NonceManagedProvider,
    overprovision_gas_limit::try_overprovision_gas_limit,
};

use alloy::primitives::{Address, FixedBytes};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use alloy::transports::{RpcError, TransportErrorKind};
use alloy::{
    eips::BlockNumberOrTag,
    network::{Ethereum, TransactionBuilder},
};

use anyhow::{bail, Result};
use async_trait::async_trait;
use sqlx::{postgres::PgListener, Pool, Postgres};
use tokio::task::JoinSet;
use tracing::{error, info, warn};

use fhevm_engine_common::telemetry;

use super::TransactionOperation;

pub type BlockHash = FixedBytes<32>;
pub type DbTransaction<'l> = sqlx::Transaction<'l, Postgres>;
type ChaindId = alloy::primitives::Uint<256, 4>;

use fhevm_gateway_bindings::multichain_acl::MultichainACL;
use fhevm_gateway_bindings::multichain_acl::MultichainACL::MultichainACLErrors;

#[derive(Clone, Debug)]
pub struct DelegationRow {
    pub delegator: Vec<u8>,
    pub delegate: Vec<u8>,
    pub contract_address: Vec<u8>,
    pub delegation_counter: u64,
    #[allow(dead_code)]
    pub old_expiry_date: u64,
    pub expiry_date: u64,
    pub host_chain_id: u64,
    pub block_hash: Vec<u8>,
    pub block_number: u64,
    pub transaction_id: Option<Vec<u8>>,
}

#[derive(Copy, Clone)]
enum BlockStatus {
    Unkown,    // the status could not be determined
    Stable,    // block is still valid
    Dismissed, // block has been reorged out
}

#[derive(Clone)]
pub struct DelegateUserDecryptOperation<P: Provider<Ethereum> + Clone + 'static> {
    multichain_acl_address: Address,
    gateway_provider: NonceManagedProvider<P>,
    host_chain_provider: P,
    conf: crate::ConfigSettings,
    gas: Option<u64>,
    db_pool: Pool<Postgres>,
}

impl<P: Provider<Ethereum> + Clone + 'static> DelegateUserDecryptOperation<P> {
    pub fn new(
        multichain_acl_address: Address,
        gateway_provider: NonceManagedProvider<P>,
        host_chain_provider: P,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
        db_pool: Pool<Postgres>,
    ) -> Self {
        info!(
            gas = gas.unwrap_or(0),
            multichain_acl_address = %multichain_acl_address,
            "Creating AllowHandleOperation"
        );

        Self {
            multichain_acl_address,
            gateway_provider,
            host_chain_provider,
            conf,
            gas,
            db_pool,
        }
    }

    /// Sends a transaction
    async fn send_transaction(
        &self,
        delegation: &DelegationRow,
        txn_request: impl Into<TransactionRequest>,
        src_transaction_id: Option<Vec<u8>>,
    ) -> Result<()> {
        info!(key = ?delegation, "Processing transaction");
        let _t = telemetry::tracer("call_delegate_account", &src_transaction_id);
        let gateway_provider = &self.gateway_provider;
        let transaction_request = try_overprovision_gas_limit(
            txn_request,
            gateway_provider.inner(),
            self.conf.gas_limit_overprovision_percent,
        )
        .await;
        let transaction = gateway_provider
            .send_transaction(transaction_request.clone())
            .await;
        let transaction = match transaction {
            Ok(txn) => txn,
            Err(e) if self.non_applicable_delegation(&e).is_some() => {
                warn!(
                    error = ?self.non_applicable_delegation(&e),
                    ?delegation,
                    "Delegation is not accepted by the contract",
                );
                return Ok(());
            }
            // Consider transport retryable errors, BackendGone and local usage errors as something that must be retried infinitely.
            // Local usage are included as they might be transient due to external AWS KMS signers.
            Err(e)
                if matches!(&e, RpcError::Transport(inner) if inner.is_retry_err() || matches!(inner, TransportErrorKind::BackendGone))
                    || matches!(&e, RpcError::LocalUsageError(_)) =>
            {
                DELEGATE_USER_DECRYPT_FAIL_COUNTER.inc();
                warn!(
                    ?transaction_request,
                    error = %e,
                    ?delegation,
                    "Transaction sending failed with unlimited retry error"
                );
                bail!(e);
            }
            Err(error) => {
                DELEGATE_USER_DECRYPT_FAIL_COUNTER.inc();
                warn!(
                    ?transaction_request,
                    %error,
                    ?delegation,
                    "Transaction sending failed"
                );
                bail!(error);
            }
        };

        // We assume that if we were able to send the transaction, we will be able to get a receipt, eventually. If there is a transport
        // error in-between, we rely on the retry logic to handle it.
        let receipt = transaction
            .with_timeout(Some(Duration::from_secs(
                self.conf.txn_receipt_timeout_secs as u64,
            )))
            .with_required_confirmations(self.conf.required_txn_confirmations as u64)
            .get_receipt()
            .await;
        let receipt = match receipt {
            Ok(receipt) => receipt,
            Err(error) => {
                DELEGATE_USER_DECRYPT_FAIL_COUNTER.inc();
                error!(%error, "Getting receipt failed");
                return Err(anyhow::Error::new(error));
            }
        };

        let transaction_hash = receipt.transaction_hash;

        if receipt.status() {
            info!(
                %transaction_hash,
                ?delegation,
                "Allow txn succeeded"
            );
            DELEGATE_USER_DECRYPT_SUCCESS_COUNTER.inc();
        } else {
            DELEGATE_USER_DECRYPT_FAIL_COUNTER.inc();
            error!(
                %transaction_hash,
                status = receipt.status(),
                ?delegation,
                "delegate txn failed"
            );

            return Err(anyhow::anyhow!(
                "Transaction {} failed with status {}, Delegation: {:?}",
                transaction_hash,
                receipt.status(),
                delegation,
            ));
        }
        Ok(())
    }

    fn non_applicable_delegation(
        &self,
        err: &RpcError<TransportErrorKind>,
    ) -> Option<MultichainACLErrors> {
        use MultichainACLErrors as E;
        err.as_error_resp()
            .and_then(|payload| payload.as_decoded_interface_error::<E>())
            .map(|error| match error {
                E::CoprocessorAlreadyDelegatedOrRevokedUserDecryption(_) => Some(error),
                E::UserDecryptionDelegationCounterTooLow(_) => Some(error),
                _ => None,
            })
            .flatten()
    }

    pub async fn tx_check_ready_delegations(
        &self,
        tx: &mut DbTransaction<'_>,
        last_ready_block: u64,
    ) -> Result<(Vec<DelegationRow>, Vec<Vec<u8>>)> {
        let delegations = delayed_sorted_delegation(tx, last_ready_block).await?;
        let nb_ready_delegations = delegations.len();
        if delegations.is_empty() {
            return Ok((vec![], vec![]));
        }
        info!(nb_ready_delegations, "Checking ready delegations");
        let mut blocks_status = HashMap::new(); // avoid multiple host chain call
        let mut stable_delegations = vec![];
        let mut unsure_block = vec![];
        let mut nb_unsure_delegations = 0;
        let mut handled_block_delegation = vec![];
        for delegation in delegations {
            let block_status = if let Some(status) = blocks_status.get(&delegation.block_number) {
                *status
            } else {
                let status = match self.get_block_hash(delegation.block_number as u64).await {
                    Ok(block_hash) if delegation.block_hash == block_hash.to_vec() => {
                        BlockStatus::Stable
                    }
                    Ok(_block_hash) => BlockStatus::Dismissed,
                    Err(_) => {
                        error!(
                            block_number = delegation.block_number,
                            "Cannot get block hash for delegation, will retry next block"
                        );
                        unsure_block.push(delegation.block_number);
                        BlockStatus::Unkown
                    }
                };
                blocks_status.insert(delegation.block_number, status);
                status
            };
            match block_status {
                BlockStatus::Stable => {
                    handled_block_delegation.push(delegation.block_hash.clone());
                    stable_delegations.push(delegation);
                }
                BlockStatus::Unkown => {
                    // skip the full block, will retry on the delegation on next call
                    nb_unsure_delegations += 1;
                    continue;
                }
                BlockStatus::Dismissed => {
                    // ignoring delegation, but will be deleted
                    handled_block_delegation.push(delegation.block_hash.clone());
                    continue;
                }
            }
        }
        let nb_stable_delegations = stable_delegations.len();
        let nb_dismissed_delegations =
            nb_ready_delegations - nb_stable_delegations - nb_unsure_delegations;
        if nb_dismissed_delegations > 0 {
            info!(
                nb_dismissed_delegations,
                "Some delegations were dismissed due to reorg"
            );
        };
        if nb_unsure_delegations > 0 {
            error!(
                nb_unsure_delegations,
                "Some delegations are dealyed due to unsure status"
            );
        };
        info!(nb_stable_delegations, "Processing ready delegations");

        Ok((stable_delegations, handled_block_delegation))
    }

    async fn get_block_hash(&self, block_number: u64) -> Result<BlockHash> {
        let search_block = BlockNumberOrTag::Number(block_number as u64);
        let some_block = self
            .host_chain_provider
            .get_block_by_number(search_block)
            .await?;
        let Some(block) = some_block else {
            error!(block_number, "A past block cannot be found by number");
            anyhow::bail!("Cannot get past block by number, giving up");
        };
        Ok(block.header.hash)
    }

    async fn wait_last_block_number(&self) -> Result<u64> {
        let mut listener = PgListener::connect_with(&self.db_pool).await?;
        listener.listen(self.channel()).await?;
        let notification = tokio::time::timeout(
            Duration::from_secs(self.conf.delegation_fallback_polling),
            listener.recv(),
        )
        .await;
        let from_chost_chain = || async { Ok(self.host_chain_provider.get_block_number().await?) };
        let Ok(notification) = notification else {
            // timeout
            warn!("delegation handling no notification, proceed based on timeout");
            return from_chost_chain().await;
        };
        let Ok(notification) = notification else {
            // connection error, try to go further in case of a real db issue, db read will fail later
            warn!("delegation handling notification error, will retry");
            return from_chost_chain().await;
        };
        let payload = notification.payload();
        let Ok(block_number) = notification.payload().parse() else {
            error!(payload, "Invalid payload for delegation notification");
            return from_chost_chain().await;
        };
        Ok(block_number)
    }
}

#[async_trait]
impl<P> TransactionOperation<P> for DelegateUserDecryptOperation<P>
where
    P: alloy::providers::Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str {
        // host-listener/src/database/tfhe_event_propagate.rs
        "new_host_block"
    }

    async fn execute(&self) -> Result<bool> {
        let block_number = self.wait_last_block_number().await?;
        let multichain_acl = MultichainACL::new(
            self.multichain_acl_address,
            self.host_chain_provider.clone(),
        );
        let up_to_block_number: u64 = block_number - self.conf.block_delay_for_delegation;
        let mut tx = self.db_pool.begin().await?;
        let delegations = self
            .tx_check_ready_delegations(&mut tx, up_to_block_number)
            .await;
        let Ok((ready_delegations, handled_block_delegation)) = delegations else {
            tx.rollback().await?;
            warn!("Error checking ready delegations, will retry later");
            anyhow::bail!("Error checking ready delegations, will retry later");
        };
        if ready_delegations.is_empty() && handled_block_delegation.is_empty() {
            tx.commit().await?;
            info!("No delegations to handle");
            return Ok(true); // will automatically rewait for new tasks via listen channel
        }
        let mut join_set = JoinSet::new();
        let mut all_transaction_id = HashSet::<Option<Vec<u8>>>::new();
        for delegation in &ready_delegations {
            let tx_id = delegation.transaction_id.clone();
            all_transaction_id.insert(tx_id);
        }
        // we don't split by transition_id because delegations have an internal order
        // it's expected that both order are compatible but we don't now the transation_id order
        let ts = all_transaction_id
            .iter()
            .map(|id| telemetry::tracer("prepare_delegate", &id))
            .collect::<Vec<_>>();
        for delegation in ready_delegations {
            let txn_request = multichain_acl
                .delegateUserDecryption(
                    ChaindId::from(delegation.host_chain_id),
                    Address::from_slice(&delegation.delegator),
                    Address::from_slice(&delegation.delegate),
                    Address::from_slice(&delegation.contract_address),
                    // delegation.old_expiry_date,
                    delegation.expiry_date,
                    delegation.delegation_counter,
                )
                .into_transaction_request();
            let txn_request = if let Some(gaz_limit) = &self.gas {
                txn_request.with_gas_limit(*gaz_limit)
            } else {
                txn_request
            };
            let operation = self.clone();
            join_set.spawn(async move {
                operation
                    .send_transaction(&delegation, txn_request, delegation.transaction_id.clone())
                    .await
            });
        }
        for t in ts {
            t.end();
        }

        while let Some(res) = join_set.join_next().await {
            let Ok(Ok(())) = res else {
                tx.rollback().await?;
                anyhow::bail!("Error sending delegation transaction, will retry later");
            };
        }

        if let Err(_) = clean_delegation(&mut tx, &handled_block_delegation).await {
            error!("Cannot clean table delegations, will be cleaned later");
            // in case of rollback, the delegations are propagated but will be retried/cleaned later
        }
        tx.commit().await?;
        Ok(true) // will automatically rewait for new tasks via listen channel
    }
}

pub async fn delayed_sorted_delegation(
    tx: &mut DbTransaction<'_>,
    up_to_block_number: u64,
) -> Result<Vec<DelegationRow>> {
    let query = sqlx::query!(
        r#"
        SELECT delegator, delegate, contract_address, delegation_counter, old_expiry_date, expiry_date, host_chain_id, block_number, block_hash, transaction_id
        FROM delegate_user_decrypt
        WHERE block_number <= $1
        ORDER BY block_number ASC, delegation_counter ASC, transaction_id ASC
        "#,
        up_to_block_number as i64,
    );
    let delegations_rows = query.fetch_all(tx.deref_mut()).await?;
    let mut delegations = Vec::with_capacity(delegations_rows.len());
    for delegation in delegations_rows {
        let delegation = DelegationRow {
            delegator: delegation.delegator,
            delegate: delegation.delegate,
            contract_address: delegation.contract_address,
            delegation_counter: delegation.delegation_counter as u64,
            old_expiry_date: delegation.old_expiry_date as u64,
            expiry_date: delegation.expiry_date as u64,
            host_chain_id: delegation.host_chain_id as u64,
            block_hash: delegation.block_hash,
            block_number: delegation.block_number as u64,
            transaction_id: delegation.transaction_id,
        };
        delegations.push(delegation);
    }
    Ok(delegations) // delegations)
}

pub async fn clean_delegation(tx: &mut DbTransaction<'_>, blocks_hash: &[Vec<u8>]) -> Result<()> {
    if blocks_hash.is_empty() {
        return Ok(());
    }
    let query = sqlx::query!(
        r#"
        DELETE FROM delegate_user_decrypt
        WHERE block_hash IN (SELECT unnest($1::bytea[]))
        "#,
        blocks_hash,
    );
    query.execute(tx.deref_mut()).await?;
    Ok(())
}
