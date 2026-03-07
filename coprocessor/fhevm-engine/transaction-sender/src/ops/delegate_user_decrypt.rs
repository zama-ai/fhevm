use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::{ops::DerefMut, time::Duration};

use crate::metrics::{
    DELEGATE_USER_DECRYPT_ERROR_BACKLOG, DELEGATE_USER_DECRYPT_FAIL_COUNTER,
    DELEGATE_USER_DECRYPT_SUCCESS_COUNTER,
};
use crate::nonce_managed_provider::NonceManagedProvider;
use crate::ops::common::{try_extract_non_retryable_config_error, CoprocessorConfigError};

use alloy::network::{Ethereum, TransactionBuilder};
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use alloy::transports::{RpcError, TransportErrorKind};

use anyhow::Result;
use async_trait::async_trait;
use fhevm_engine_common::chain_id::ChainId;
use sqlx::types::BigDecimal;
use sqlx::{postgres::PgListener, Pool, Postgres};
use tokio::task::JoinSet;
use tracing::{error, info, warn};

use super::TransactionOperation;

pub type DbTransaction<'l> = sqlx::Transaction<'l, Postgres>;

use fhevm_gateway_bindings::multichain_acl::MultichainACL;
use fhevm_gateway_bindings::multichain_acl::MultichainACL::MultichainACLErrors;

#[derive(Clone, Debug)]
pub struct DelegationRow {
    pub key: i64,
    pub delegator: Vec<u8>,
    pub delegate: Vec<u8>,
    pub contract_address: Vec<u8>,
    pub delegation_counter: u64,
    #[allow(dead_code)]
    pub old_expiration_date: u64,
    pub new_expiration_date: u64,
    pub host_chain_id: ChainId,
    pub block_hash: Vec<u8>,
    pub block_number: u64,
    pub transaction_id: Option<Vec<u8>>,
    pub gateway_nb_attempts: u64,
}

#[derive(Copy, Clone)]
enum BlockStatus {
    Unknown,   // the status could not be determined
    Stable,    // block is still valid
    Dismissed, // block has been reorged out
}

#[derive(Clone)]
pub struct DelegateUserDecryptOperation<P: Provider<Ethereum> + Clone + 'static> {
    multichain_acl_address: Address,
    gateway_provider: NonceManagedProvider<P>,
    conf: crate::ConfigSettings,
    gas: Option<u64>,
    db_pool: Pool<Postgres>,
    cancel_token: tokio_util::sync::CancellationToken,
}

enum TxResult {
    Success,
    IdemPotentError,
    TransientError,
    NonRetryableConfigError(CoprocessorConfigError),
    OtherError(String),
}

fn is_transient_error(error: &RpcError<TransportErrorKind>) -> bool {
    match error {
        // Consider transport, retryable errors, BackendGone and local usage errors as something that must be retried infinitely.
        // Local usage are included as they might be transient due to external AWS KMS signers.
        RpcError::LocalUsageError(_) => true,
        RpcError::Transport(TransportErrorKind::BackendGone) => true,
        RpcError::Transport(inner) if inner.is_retry_err() => true,
        _ => false,
    }
}

impl<P: Provider<Ethereum> + Clone + 'static> DelegateUserDecryptOperation<P> {
    pub fn new(
        multichain_acl_address: Address,
        gateway_provider: NonceManagedProvider<P>,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
        db_pool: Pool<Postgres>,
        cancel_token: tokio_util::sync::CancellationToken,
    ) -> Self {
        info!(
            gas = gas.unwrap_or(0),
            multichain_acl_address = %multichain_acl_address,
            "Creating DelegateUserDecryptOperation"
        );

        Self {
            multichain_acl_address,
            gateway_provider,
            conf,
            gas,
            db_pool,
            cancel_token,
        }
    }
    /// Sends a transaction
    #[tracing::instrument(name = "call_delegate_user_decrypt", skip_all, fields(txn_id = tracing::field::Empty))]
    async fn send_transaction(
        &self,
        delegation: &DelegationRow,
        txn_request: impl Into<TransactionRequest>,
    ) -> TxResult {
        fhevm_engine_common::telemetry::record_short_hex_if_some(
            &tracing::Span::current(),
            "txn_id",
            delegation.transaction_id.as_deref(),
        );
        info!(key = ?delegation, "Processing transaction for DelegateUserDecryptOperation");
        let operation = if delegation.new_expiration_date == 0 {
            "RevokeUserDecryptionDelegation"
        } else {
            "DelegateUserDecryption"
        };

        let receipt = match self
            .gateway_provider
            .send_sync_with_overprovision(
                txn_request,
                self.conf.gas_limit_overprovision_percent,
                Duration::from_secs(self.conf.send_txn_sync_timeout_secs.into()),
            )
            .await
        {
            Ok(txn) => txn,
            Err(e) if self.idempotency_error(&e).is_some() => {
                warn!(
                    error = ?self.idempotency_error(&e),
                    ?delegation,
                    "{operation} is already included in contract",
                );
                return TxResult::IdemPotentError;
            }
            Err(error) => {
                if is_transient_error(&error) {
                    warn!(
                        %error,
                        ?delegation,
                        "{operation} sending with transient error. Will retry indefinitely"
                    );
                    return TxResult::TransientError;
                }
                if let Some(non_retryable_config_error) =
                    try_extract_non_retryable_config_error(&error)
                {
                    warn!(
                        error = %non_retryable_config_error,
                        ?delegation,
                        "{operation} failed with non-retryable gateway coprocessor config error"
                    );
                    return TxResult::NonRetryableConfigError(non_retryable_config_error);
                }
                warn!(
                    %error,
                    ?delegation,
                    "{operation} sending failed with unexpected error"
                );
                return TxResult::OtherError(error.to_string());
            }
        };

        let transaction_hash = receipt.transaction_hash;

        if receipt.status() {
            info!(
                %transaction_hash,
                ?delegation,
                "{operation} txn succeeded"
            );
            TxResult::Success
        } else {
            error!(
                %transaction_hash,
                status = receipt.status(),
                ?delegation,
                "{operation} txn failed"
            );
            TxResult::OtherError(format!(
                "Transaction {} failed with status {}, Delegation: {:?}",
                transaction_hash,
                receipt.status(),
                delegation,
            ))
        }
    }

    fn idempotency_error(&self, err: &RpcError<TransportErrorKind>) -> Option<MultichainACLErrors> {
        use MultichainACLErrors as E;
        err.as_error_resp()
            .and_then(|payload| payload.as_decoded_interface_error::<E>())
            .and_then(|error| match error {
                E::CoprocessorAlreadyDelegatedUserDecryption(_) => Some(error),
                E::CoprocessorAlreadyRevokedUserDecryption(_) => Some(error),
                E::UserDecryptionDelegationCounterTooLow(_) => Some(error),
                _ => None,
            })
    }

    pub async fn tx_check_ready_delegations(
        &self,
        tx: &mut DbTransaction<'_>,
    ) -> Result<(Vec<DelegationRow>, Vec<DelegationRow>)> {
        let delegations = delayed_sorted_delegation(tx, self.conf.delegation_max_retry).await?;
        let nb_ready_delegations = delegations.len();
        if delegations.is_empty() {
            return Ok((vec![], vec![]));
        }
        let max_error_level = delegations
            .iter()
            .map(|d| d.gateway_nb_attempts)
            .max()
            .unwrap_or(0);
        let retry_up_to_error_level = retry_error_up_to_error_level(max_error_level);
        info!(
            nb_ready_delegations,
            max_error_level, retry_up_to_error_level, "Ready delegations"
        );
        let mut blocks_status = HashMap::new(); // cache db access
        let mut stable_delegations = vec![];
        let mut nb_unsure_delegations = 0;
        let mut reorg_out_delegations = vec![];
        let mut past_error_backlog = 0;
        for delegation in delegations {
            if delegation.gateway_nb_attempts > 0 {
                past_error_backlog += 1;
            }
            if delegation.gateway_nb_attempts > self.conf.delegation_max_retry {
                continue;
            }
            if delegation.gateway_nb_attempts > retry_up_to_error_level {
                continue;
            }
            let block_status = if let Some(status) = blocks_status.get(&delegation.block_hash) {
                *status
            } else {
                let status = self.get_block_status(&delegation).await;
                blocks_status.insert(delegation.block_hash.clone(), status);
                status
            };
            match block_status {
                BlockStatus::Dismissed => {
                    warn!(
                        delegation_block_hash = ?delegation.block_hash,
                        block_number = delegation.block_number,
                        "Block hash mismatch for delegation, block was reorged out"
                    );
                    // ignoring delegation due to reorg, will be marked as reorg_out
                    reorg_out_delegations.push(delegation.clone());
                }
                BlockStatus::Unknown => {
                    warn!(
                        block_number = delegation.block_number,
                        "Cannot get block hash for delegation, will retry next block"
                    );
                    nb_unsure_delegations += 1;
                }
                BlockStatus::Stable => {
                    stable_delegations.push(delegation.clone());
                }
            }
        }
        DELEGATE_USER_DECRYPT_ERROR_BACKLOG.set(past_error_backlog);
        let nb_stable_delegations = stable_delegations.len();
        let dismissed = reorg_out_delegations.len();
        if dismissed > 0 {
            info!(dismissed, "Some delegations were dismissed due to reorg");
        };
        if nb_unsure_delegations > 0 {
            error!(
                nb_unsure_delegations,
                "Some delegations are delayed due to unsure status"
            );
        };
        info!(nb_stable_delegations, "Processing ready delegations");

        Ok((stable_delegations, reorg_out_delegations))
    }

    async fn get_block_status(&self, delegation: &DelegationRow) -> BlockStatus {
        let status = sqlx::query!(
            r#"
            SELECT block_status
            FROM host_chain_blocks_valid
            WHERE block_hash = $2 AND chain_id = $1
            "#,
            delegation.host_chain_id.as_i64(),
            delegation.block_hash,
        )
        .fetch_optional(&self.db_pool)
        .await;
        match status {
            Ok(Some(record)) => match record.block_status.as_str() {
                "finalized" => BlockStatus::Stable,
                "orphaned" => BlockStatus::Dismissed,
                "unknown" => {
                    warn!(
                        ?delegation,
                        "Block with unknown status for delegation, delegation was introduced during migration, please manually fix block status in host_chain_blocks_valid table to process the delegation"
                    );
                    BlockStatus::Unknown
                }
                "pending" => BlockStatus::Unknown,
                _ => {
                    error!(
                        ?delegation,
                        status = record.block_status,
                        "Invalid block status for delegation, manually fix it to process the delegation",
                    );
                    BlockStatus::Unknown
                }
            },
            Ok(None) => {
                error!(?delegation, "No block status found for delegation");
                BlockStatus::Unknown
            }
            Err(e) => {
                error!(
                    %e,
                    ?delegation,
                    "Error querying block status from database"
                );
                BlockStatus::Unknown
            }
        }
    }

    async fn wait_new_block(&self) -> Result<()> {
        let mut listener = PgListener::connect_with(&self.db_pool).await?;
        listener.listen(self.channel()).await?;
        tokio::select! {
            _ = self.cancel_token.cancelled() => anyhow::bail!("Operation cancelled"),
            notification = listener.recv() => {
                match notification {
                    Ok(notification) => {
                        info!(?notification, "Received new block notification");
                    }
                    Err(e) => {
                        tokio::time::sleep(Duration::from_secs(1)).await; // avoid busy loop if db connection is lost
                        error!(%e, "Error receiving new block notification, will process delegations");
                    }
                }
                Ok(())
            }
            _ = tokio::time::sleep(Duration::from_secs(self.conf.delegation_fallback_polling)) => Ok(()),
        }
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
        self.wait_new_block().await?;
        let multichain_acl =
            MultichainACL::new(self.multichain_acl_address, self.gateway_provider.inner());
        let mut tx = self.db_pool.begin().await?;
        let delegations = self.tx_check_ready_delegations(&mut tx).await;

        let Ok((ready_delegations, reorg_out_delegations)) = delegations else {
            tx.rollback().await?;
            warn!("Error checking ready delegations, will retry later");
            anyhow::bail!("Error checking ready delegations, will retry later");
        };
        if ready_delegations.is_empty() && reorg_out_delegations.is_empty() {
            tx.commit().await?;
            info!("No delegations to handle");
            return Ok(true); // will automatically rewait for new tasks via listen channel
        }
        if let Err(err) = update_reorged_delegations(&mut tx, &reorg_out_delegations).await {
            error!(?err, "Cannot update reorged delegations, will retry later, continuing on finalized delegations");
        }
        let mut requests = Vec::with_capacity(ready_delegations.len());
        let to_transaction = |delegation: &DelegationRow| {
            let is_revoke = delegation.new_expiration_date == 0;
            if is_revoke {
                multichain_acl
                    .revokeUserDecryptionDelegation(
                        delegation.host_chain_id.into(),
                        Address::from_slice(&delegation.delegator),
                        Address::from_slice(&delegation.delegate),
                        Address::from_slice(&delegation.contract_address),
                        delegation.delegation_counter,
                        delegation.old_expiration_date,
                    )
                    .into_transaction_request()
            } else {
                multichain_acl
                    .delegateUserDecryption(
                        delegation.host_chain_id.into(),
                        Address::from_slice(&delegation.delegator),
                        Address::from_slice(&delegation.delegate),
                        Address::from_slice(&delegation.contract_address),
                        delegation.delegation_counter,
                        delegation.new_expiration_date,
                    )
                    .into_transaction_request()
            }
        };
        for delegation in ready_delegations {
            let prepare_delegate_span =
                tracing::info_span!("prepare_delegate", txn_id = tracing::field::Empty);
            fhevm_engine_common::telemetry::record_short_hex_if_some(
                &prepare_delegate_span,
                "txn_id",
                delegation.transaction_id.as_deref(),
            );
            let txn_request = prepare_delegate_span.in_scope(|| to_transaction(&delegation));
            let txn_request = if let Some(gaz_limit) = &self.gas {
                txn_request.with_gas_limit(*gaz_limit)
            } else {
                txn_request
            };
            requests.push((delegation, txn_request));
        }
        let mut join_set = JoinSet::new();
        for (delegation, txn_request) in requests.iter() {
            // parallel transaction can fail if any of the transaction fail
            // with a nonce too high error
            // so we maintain the joint set of successful delegations
            let operation = self.clone();
            let delegation = delegation.clone();
            let txn_request = txn_request.clone();
            join_set.spawn(async move {
                let tx_result = operation
                    .send_transaction(&delegation, txn_request.clone())
                    .await;
                (delegation.clone(), tx_result)
            });
        }
        let mut other_error = false;
        let mut transient_error = false;
        let mut nb_success = 0;
        let mut nb_errors = 0;
        while let Some(res) = join_set.join_next().await {
            let Ok((delegation, tx_result)) = res else {
                transient_error = true;
                error!("Join error in delegation transaction sending");
                continue;
            };
            match tx_result {
                TxResult::Success | TxResult::IdemPotentError => {
                    nb_success += 1;
                    update_transmitted_delegation(&mut tx, &delegation).await;
                }
                TxResult::TransientError => {
                    nb_errors += 1;
                    transient_error = true;
                    update_error_delegation(&mut tx, &delegation, "transient_error").await;
                }
                TxResult::NonRetryableConfigError(e) => {
                    nb_errors += 1;
                    other_error = true;
                    stop_retrying_delegation_on_config_error(
                        &mut tx,
                        &delegation,
                        &e.to_string(),
                        self.conf.delegation_max_retry + 1,
                    )
                    .await;
                }
                TxResult::OtherError(e) => {
                    nb_errors += 1;
                    update_error_delegation(&mut tx, &delegation, &e.to_string()).await;
                    other_error = true
                }
            }
        }
        tx.commit().await?;
        DELEGATE_USER_DECRYPT_SUCCESS_COUNTER.inc_by(nb_success);
        DELEGATE_USER_DECRYPT_FAIL_COUNTER.inc_by(nb_errors);
        if transient_error {
            // force a restart in case of backend gone or a wait before retry
            anyhow::bail!("Delegation transactions failed, will retry later");
        }
        Ok(!other_error) // either immediately rewait notification or wait a bit more
    }
}

fn expiration_date_to_u64(value: BigDecimal) -> u64 {
    let value = value.round(0); // round to integer
    let (integer, _scale) = value.as_bigint_and_exponent();
    // Clip to range
    let (sign, digits) = integer.to_u64_digits();
    if sign == bigdecimal::num_bigint::Sign::Minus {
        error!("Negative value for expiration date, setting to 0");
        0
    } else if digits.len() > 1 {
        error!("Too big value value for expiration date, setting to u64::MAX");
        u64::MAX
    } else if digits.len() == 1 {
        digits[0]
    } else {
        0
    }
}

pub async fn delayed_sorted_delegation(
    tx: &mut DbTransaction<'_>,
    delegation_max_retry: u64,
) -> Result<Vec<DelegationRow>> {
    let query = sqlx::query!(
        r#"
        SELECT key, delegator, delegate, contract_address, delegation_counter, old_expiration_date, new_expiration_date, host_chain_id, block_number, block_hash, transaction_id, gateway_nb_attempts
        FROM delegate_user_decrypt
        WHERE on_gateway = false
        AND reorg_out = false
        AND gateway_nb_attempts <= $1
        ORDER BY block_number ASC, delegation_counter ASC, transaction_id ASC
        FOR UPDATE
        "#,
        delegation_max_retry as i64, // excludes delegations retired after a non-retryable config error (set to max_retry + 1)
    );
    let delegations_rows = query.fetch_all(tx.deref_mut()).await?;
    let mut delegations = Vec::with_capacity(delegations_rows.len());
    for delegation in delegations_rows {
        let delegation = DelegationRow {
            key: delegation.key,
            delegator: delegation.delegator,
            delegate: delegation.delegate,
            contract_address: delegation.contract_address,
            delegation_counter: delegation.delegation_counter as u64,
            old_expiration_date: expiration_date_to_u64(delegation.old_expiration_date),
            new_expiration_date: expiration_date_to_u64(delegation.new_expiration_date),
            host_chain_id: ChainId::try_from(delegation.host_chain_id)
                .map_err(|e| anyhow::anyhow!("invalid host_chain_id in DB: {e}"))?,
            block_hash: delegation.block_hash,
            block_number: delegation.block_number as u64,
            transaction_id: delegation.transaction_id,
            gateway_nb_attempts: delegation.gateway_nb_attempts as u64,
        };
        delegations.push(delegation);
    }
    Ok(delegations)
}

pub async fn update_error_delegation(
    tx: &mut DbTransaction<'_>,
    delegation: &DelegationRow,
    error: &str,
) -> () {
    // update on_gateway
    error!(%error, ?delegation, "Updating error delegation");
    let Ok(res) = sqlx::query!(
        r#"
        UPDATE delegate_user_decrypt
        SET gateway_nb_attempts = gateway_nb_attempts + 1,
            gateway_last_error = $1
        WHERE key = $2
        "#,
        error,
        delegation.key,
    )
    .execute(tx.deref_mut())
    .await
    else {
        error!(error, ?delegation, "Cannot update error delegation");
        return;
    };
    if res.rows_affected() == 0 {
        error!(
            error,
            ?delegation,
            "No rows updated when updating error delegation"
        );
    }
}

pub async fn stop_retrying_delegation_on_config_error(
    tx: &mut DbTransaction<'_>,
    delegation: &DelegationRow,
    error: &str,
    max_attempts: u64,
) {
    let res = match sqlx::query!(
        r#"
        UPDATE delegate_user_decrypt
        SET gateway_nb_attempts = $1,
            gateway_last_error = $2
        WHERE key = $3
        "#,
        max_attempts as i64,
        error,
        delegation.key,
    )
    .execute(tx.deref_mut())
    .await
    {
        Ok(res) => res,
        Err(db_err) => {
            error!(%db_err, ?delegation, "Cannot mark non-retryable delegation");
            return;
        }
    };
    if res.rows_affected() == 0 {
        error!(
            error,
            ?delegation,
            "No rows updated when marking non-retryable delegation"
        );
    }
}

pub async fn update_transmitted_delegation(
    tx: &mut DbTransaction<'_>,
    delegation: &DelegationRow,
) -> () {
    // update on_gateway
    let Ok(res) = sqlx::query!(
        r#"
        UPDATE delegate_user_decrypt
        SET on_gateway = true
        WHERE key = $1
        "#,
        delegation.key,
    )
    .execute(tx.deref_mut())
    .await
    else {
        error!(?delegation, "Cannot update transmitted delegation");
        return;
    };
    if res.rows_affected() == 0 {
        error!(
            ?delegation,
            "No rows updated when updating transmitted delegation"
        );
    }
}

pub async fn update_reorged_delegations(
    tx: &mut DbTransaction<'_>,
    reorged_delegations: &[DelegationRow],
) -> Result<()> {
    // update reorg out
    if !reorged_delegations.is_empty() {
        let keys = reorged_delegations
            .iter()
            .map(|d| d.key)
            .collect::<Vec<_>>();
        let reorg_out = sqlx::query!(
            r#"
            UPDATE delegate_user_decrypt
            SET reorg_out = true
            WHERE key = ANY($1)
            "#,
            &keys
        )
        .execute(tx.deref_mut())
        .await?;
        if reorg_out.rows_affected() == 0 {
            error!(
                nb_delegations = keys.len(),
                "No rows updated when updating reorg out delegation"
            );
        }
    }
    Ok(())
}

fn retry_error_up_to_error_level(maximum_error_level: u64) -> u64 {
    // 2 properties:
    // 1. error replayed are spaced with free of past terror period
    //    using a cool down/debt system, so the system is not overloaded with retries
    // 2. lower error levels are replayed with higher probability than highier error levels
    //    an error level is repayed with probability 1/n for level n
    static PSEUDO_RANDOM: AtomicU64 = AtomicU64::new(0);
    static DEBT: AtomicI64 = AtomicI64::new(1);
    let pseudo_random = PSEUDO_RANDOM.fetch_add(1, Ordering::Relaxed);
    let prev_debt = DEBT.fetch_sub(1, Ordering::Relaxed);
    let debt = prev_debt - 1;
    if debt <= 0 {
        for error_level in (1..=maximum_error_level).rev() {
            let modulo = if error_level == maximum_error_level {
                error_level
            } else {
                // error level n + 1 also counts for proba of level n
                // 1 / n - 1 / (n + 1) == 1 / n (n + 1)
                error_level * (error_level + 1)
            };
            if pseudo_random.is_multiple_of(modulo) {
                DEBT.store(error_level as i64, Ordering::Relaxed);
                return error_level;
            }
        }
    }
    // either no debt to replay or no error level selected
    0 // no error replay
}
