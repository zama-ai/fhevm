use std::{sync::Arc, time::Duration};

use alloy::{
    network::{Ethereum, TransactionBuilder},
    primitives::{Address, TxHash},
    providers::{
        fillers::{
            BlobGasFiller, CachedNonceManager, ChainIdFiller, GasFiller, JoinFill, NonceManager,
        },
        PendingTransactionBuilder, Provider,
    },
    rpc::types::{TransactionReceipt, TransactionRequest},
    transports::{RpcError, TransportErrorKind, TransportResult},
};
use futures_util::lock::Mutex;
use tracing::{debug, info, warn};

use crate::config::DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT;

const SEALED_RECEIPT_POLL_INTERVAL: Duration = Duration::from_millis(100);

/// The deadline expired after the transaction executed but before its block was
/// sealed. This error has the same message as a send timeout. Callers classify
/// and report it as a send timeout.
#[derive(Debug, thiserror::Error)]
#[error("eth_sendRawTransactionSync timeout")]
struct UnconfirmedSubmission {
    tx_hash: TxHash,
}

/// Returns the hash of an executed submission that did not get a sealed receipt
/// in time. Callers keep the hash. The next attempt checks that transaction before
/// it sends again.
pub(crate) fn unconfirmed_tx_hash(err: &RpcError<TransportErrorKind>) -> Option<TxHash> {
    match err {
        RpcError::Transport(TransportErrorKind::Custom(err)) => err
            .downcast_ref::<UnconfirmedSubmission>()
            .map(|unconfirmed| unconfirmed.tx_hash),
        _ => None,
    }
}

/// Arbitrum sets `blockHash` to `0x00..0` in the receipt of an executed
/// transaction before it seals the block.
fn is_sealed(receipt: &TransactionReceipt) -> bool {
    receipt.block_hash.is_some_and(|hash| !hash.is_zero())
}

pub type FillersWithoutNonceManagement =
    JoinFill<GasFiller, JoinFill<BlobGasFiller, ChainIdFiller>>;

/// A wrapper around an `alloy` provider that sends transactions with the correct nonce.
/// Note that the given provider by the user must not have nonce management enabled, as this
/// is done by the `NonceManagedProvider` itself.
#[derive(Clone)]
pub struct NonceManagedProvider<P>
where
    P: Provider<Ethereum>,
{
    provider: P,
    nonce_manager: Arc<Mutex<CachedNonceManager>>,
    signer_address: Option<Address>,
}

impl<P> NonceManagedProvider<P>
where
    P: Provider<Ethereum>,
{
    pub fn new(provider: P, signer_address: Option<Address>) -> Self {
        Self {
            provider,
            nonce_manager: Default::default(),
            signer_address,
        }
    }

    pub async fn send_transaction(
        &self,
        tx: impl Into<TransactionRequest>,
    ) -> TransportResult<PendingTransactionBuilder<Ethereum>> {
        let mut tx = tx.into();
        if let Some(signer_address) = self.signer_address {
            let nonce_manager = self.nonce_manager.lock().await;
            let nonce = nonce_manager
                .get_next_nonce(&self.provider, signer_address)
                .await?;
            tx.nonce = Some(nonce);
        }
        let res = self.provider.send_transaction(tx).await;
        if res.is_err() {
            // Reset the nonce manager if the transaction sending failed.
            *self.nonce_manager.lock().await = Default::default();
        }
        res
    }

    pub async fn send_transaction_sync(
        &self,
        tx: impl Into<TransactionRequest>,
        timeout: Duration,
    ) -> TransportResult<TransactionReceipt> {
        let mut tx = tx.into();
        if let Some(signer_address) = self.signer_address {
            let nonce_manager = self.nonce_manager.lock().await;
            let nonce = nonce_manager
                .get_next_nonce(&self.provider, signer_address)
                .await?;
            tx.nonce = Some(nonce);
        }
        // The deadline also includes the wait for the sealed receipt. Before the
        // Arbitrum change, the sync call itself included this wait.
        let mut executed = None;
        let res = tokio::time::timeout(timeout, async {
            let receipt = self.provider.send_transaction_sync(tx).await?;
            executed = Some(receipt.transaction_hash);
            Ok(self.wait_for_sealed_receipt(receipt).await)
        })
        .await
        .map_err(|_| match executed {
            Some(tx_hash) => TransportErrorKind::custom(UnconfirmedSubmission { tx_hash }),
            None => TransportErrorKind::custom_str("eth_sendRawTransactionSync timeout"),
        })
        .flatten();
        if res.is_err() {
            // Reset the nonce manager if the transaction sending failed.
            *self.nonce_manager.lock().await = Default::default();
        }
        res
    }

    /// Arbitrum can answer `eth_sendRawTransactionSync` after the transaction
    /// executes but before Arbitrum seals the block. This receipt has a zero block
    /// hash, and Arbitrum can retract it. This function polls until the sealed
    /// receipt is available. Callers mark the submission as complete only after that.
    /// This function retries read errors. The deadline of the caller limits the wait.
    async fn wait_for_sealed_receipt(&self, receipt: TransactionReceipt) -> TransactionReceipt {
        if is_sealed(&receipt) {
            return receipt;
        }
        loop {
            tokio::time::sleep(SEALED_RECEIPT_POLL_INTERVAL).await;
            if let Ok(Some(canonical)) = self
                .provider
                .get_transaction_receipt(receipt.transaction_hash)
                .await
            {
                if is_sealed(&canonical) {
                    return canonical;
                }
            }
        }
    }

    /// Sends `txn_request` as [`Self::send_sync_with_overprovision`] does.
    ///
    /// `pending_txn_hash` can identify an earlier submission that executed but did
    /// not get a sealed receipt before its deadline. In this case, this function
    /// first reads the receipt of that transaction:
    /// - A sealed receipt completes the submission. The function does not send again.
    /// - An unsealed receipt keeps the submission unconfirmed.
    /// - A missing receipt means that the block failed and the transaction is gone.
    ///   The function then sends again.
    ///
    /// This check occurs after the deadline and the retry backoff. Arbitrum seals a
    /// block much faster than this interval. For this reason, a missing receipt is a
    /// reliable result.
    ///
    /// The flag is true if the function cannot read the earlier receipt. The outcome
    /// is then unknown, and the error must not use a limited retry.
    pub(crate) async fn settle_or_send(
        &self,
        pending_txn_hash: Option<Vec<u8>>,
        txn_request: impl Into<TransactionRequest>,
        percent: u32,
        send_sync_timeout: Duration,
    ) -> (TransportResult<TransactionReceipt>, bool) {
        // Only this crate writes the hash. Ignore a malformed hash. If you do not
        // ignore it, every attempt for the row fails.
        let Some(hash) = pending_txn_hash.and_then(|hash| TxHash::try_from(&hash[..]).ok()) else {
            return (
                self.send_sync_with_overprovision(txn_request, percent, send_sync_timeout)
                    .await,
                false,
            );
        };
        match self.provider.get_transaction_receipt(hash).await {
            Ok(Some(sealed)) if is_sealed(&sealed) => {
                info!(
                    transaction_hash = %sealed.transaction_hash,
                    "Earlier submission was sealed, not resubmitting"
                );
                (Ok(sealed), false)
            }
            // The transaction executed, but its block is not sealed yet. A new
            // submission can cause two transactions to land. Report the submission
            // as unconfirmed, so that the caller keeps the hash.
            Ok(Some(_)) => (
                Err(TransportErrorKind::custom(UnconfirmedSubmission {
                    tx_hash: hash,
                })),
                false,
            ),
            Ok(None) => (
                self.send_sync_with_overprovision(txn_request, percent, send_sync_timeout)
                    .await,
                false,
            ),
            Err(e) => (Err(e), true),
        }
    }

    /// If `txn_request.gas` is set, overprovision it by the given percent.
    /// If `txn_request.gas` is not set, estimate the gas limit and then overprovision it by the given percent.
    /// If the percent is less than 100, DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT is used.
    pub async fn overprovision_gas_limit(
        &self,
        txn_request: impl Into<TransactionRequest>,
        percent: u32,
    ) -> TransportResult<TransactionRequest> {
        let percent = if percent < 100 {
            warn!(
                gas_limit_overprovision_percent = percent,
                default_gas_limit_overprovision_percent = DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT,
                "Overprovision percent is less than 100, using default value instead"
            );
            DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT
        } else {
            percent
        };

        let overprovision = |gas: u64| (gas as u128 * percent as u128 / 100) as u64;

        let mut txn: TransactionRequest = txn_request.into();

        let new_gas = match txn.gas {
            Some(existing_gas) => Some(existing_gas),
            None => Some(self.provider.estimate_gas(txn.clone()).await?),
        }
        .map(overprovision);

        if let Some(gas) = new_gas {
            debug!(
                gas_limit = gas,
                gas_limit_overprovision_percent = percent,
                "Overprovisioned gas limit"
            );
            txn.set_gas_limit(gas);
        }

        Ok(txn)
    }

    // Ensure that if gas estimation fails due to a revert, the transaction is not sent and no nonce is consumed.
    pub async fn send_sync_with_overprovision(
        &self,
        txn_request: impl Into<TransactionRequest>,
        percent: u32,
        send_sync_timeout: Duration,
    ) -> TransportResult<alloy::rpc::types::TransactionReceipt> {
        let overprovisioned_txn = self.overprovision_gas_limit(txn_request, percent).await?;
        self.send_transaction_sync(overprovisioned_txn, send_sync_timeout)
            .await
    }

    pub async fn get_chain_id(&self) -> TransportResult<u64> {
        self.provider.get_chain_id().await
    }

    pub async fn get_transaction_count(&self, address: Address) -> TransportResult<u64> {
        self.provider.get_transaction_count(address).await
    }

    pub async fn get_block_number(&self) -> TransportResult<u64> {
        self.provider.get_block_number().await
    }

    pub fn inner(&self) -> &P {
        &self.provider
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{primitives::B256, providers::ProviderBuilder, transports::mock::Asserter};
    use serde_json::{json, Value};

    fn receipt(block_hash: B256, status: bool) -> Value {
        json!({
            "type": "0x2",
            "status": if status { "0x1" } else { "0x0" },
            "cumulativeGasUsed": "0x1",
            "logs": [],
            "logsBloom": format!("0x{}", "00".repeat(256)),
            "transactionHash": B256::with_last_byte(1),
            "transactionIndex": "0x0",
            "blockHash": block_hash,
            "blockNumber": "0x1",
            "gasUsed": "0x1",
            "effectiveGasPrice": "0x1",
            "from": Address::ZERO,
            "to": Address::ZERO,
            "contractAddress": null,
        })
    }

    fn provider(asserter: Asserter) -> NonceManagedProvider<impl Provider<Ethereum>> {
        NonceManagedProvider::new(
            ProviderBuilder::new()
                .disable_recommended_fillers()
                .connect_mocked_client(asserter),
            None,
        )
    }

    #[tokio::test]
    async fn partial_receipt_waits_for_sealed_receipt_and_uses_its_status() {
        let asserter = Asserter::new();
        asserter.push_success(&receipt(B256::ZERO, true));
        asserter.push_success(&Value::Null);
        asserter.push_failure_msg("rate limited");
        asserter.push_success(&receipt(B256::ZERO, true));
        asserter.push_success(&receipt(B256::with_last_byte(7), false));

        let receipt = provider(asserter.clone())
            .send_transaction_sync(TransactionRequest::default(), Duration::from_secs(5))
            .await
            .unwrap();

        assert_eq!(receipt.block_hash, Some(B256::with_last_byte(7)));
        assert!(!receipt.status());
        assert!(asserter.read_q().is_empty());
    }

    #[tokio::test]
    async fn sealed_receipt_is_returned_without_polling() {
        let asserter = Asserter::new();
        asserter.push_success(&receipt(B256::with_last_byte(7), true));

        let receipt = provider(asserter.clone())
            .send_transaction_sync(TransactionRequest::default(), Duration::from_secs(5))
            .await
            .unwrap();

        assert!(receipt.status());
        assert!(asserter.read_q().is_empty());
    }

    #[tokio::test]
    async fn retracted_partial_receipt_is_a_transient_send_timeout() {
        let asserter = Asserter::new();
        asserter.push_success(&receipt(B256::ZERO, true));
        for _ in 0..100 {
            asserter.push_success(&Value::Null);
        }

        let err = provider(asserter)
            .send_transaction_sync(TransactionRequest::default(), Duration::from_millis(300))
            .await
            .unwrap_err();

        // Callers classify this error as a transient send timeout. The error
        // contains the hash that the next attempt checks.
        assert!(matches!(
            err,
            RpcError::Transport(TransportErrorKind::Custom(ref e))
                if e.to_string() == "eth_sendRawTransactionSync timeout"
        ));
        assert_eq!(unconfirmed_tx_hash(&err), Some(B256::with_last_byte(1)));
    }
}
