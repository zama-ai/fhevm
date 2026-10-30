use std::{sync::Arc, time::Duration};

use alloy::{
    consensus::{transaction::SignerRecoverable, Transaction, TxEnvelope},
    eips::eip2718::{Decodable2718, Encodable2718},
    network::{Ethereum, EthereumWallet, TransactionBuilder},
    primitives::{Address, B256},
    providers::{
        fillers::{BlobGasFiller, ChainIdFiller, GasFiller, JoinFill, NonceManager},
        PendingTransactionBuilder, Provider, ProviderBuilder,
    },
    rpc::types::{TransactionReceipt, TransactionRequest},
    transports::{TransportErrorKind, TransportResult},
};
use nonce_manager::NonceManagerNonOptimistic;
use tracing::{debug, warn};

use crate::config::DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT;

pub type FillersWithoutNonceManagement =
    JoinFill<GasFiller, JoinFill<BlobGasFiller, ChainIdFiller>>;

/// A locally signed transaction. Persist this before any broadcast.
#[derive(Clone, Debug)]
pub struct PreparedTransaction {
    pub raw: Vec<u8>,
    pub hash: B256,
    pub nonce: u64,
}

impl PreparedTransaction {
    pub fn decode(raw: Vec<u8>) -> TransportResult<Self> {
        let envelope =
            TxEnvelope::decode_2718(&mut raw.as_slice()).map_err(TransportErrorKind::custom)?;
        Ok(Self {
            hash: *envelope.tx_hash(),
            nonce: envelope.nonce(),
            raw,
        })
    }
}

/// A wrapper around an `alloy` provider that sends transactions with the correct nonce.
/// Note that the given provider by the user must not have nonce management enabled, as this
/// is done by the `NonceManagedProvider` itself.
#[derive(Clone)]
pub struct NonceManagedProvider<P>
where
    P: Provider<Ethereum>,
{
    provider: P,
    nonce_manager: Arc<NonceManagerNonOptimistic>,
    signer_address: Option<Address>,
    prepared_nonce_seed: Arc<tokio::sync::OnceCell<()>>,
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
            prepared_nonce_seed: Default::default(),
        }
    }

    pub fn signer_address(&self) -> Option<Address> {
        self.signer_address
    }

    pub async fn send_transaction(
        &self,
        tx: impl Into<TransactionRequest>,
    ) -> TransportResult<PendingTransactionBuilder<Ethereum>> {
        let mut tx = tx.into();
        let nonce = self.assign_nonce(&mut tx).await?;
        let res = self.provider.send_transaction(tx).await;
        self.finish_nonce(nonce, &res).await;
        res
    }

    pub async fn send_transaction_sync(
        &self,
        tx: impl Into<TransactionRequest>,
        timeout: Duration,
    ) -> TransportResult<TransactionReceipt> {
        let mut tx = tx.into();
        let nonce = self.assign_nonce(&mut tx).await?;
        let res = tokio::time::timeout(timeout, self.provider.send_transaction_sync(tx))
            .await
            .map_err(|_| TransportErrorKind::custom_str("eth_sendRawTransactionSync timeout"))
            .flatten();
        // Settle the reservation outside the timed future, including local timeouts.
        self.finish_nonce(nonce, &res).await;
        res
    }

    /// Fill and sign locally using the sender's signer, without broadcasting.
    pub async fn prepare_transaction(
        &self,
        tx: TransactionRequest,
        percent: u32,
        signer: crate::AbstractSigner,
    ) -> TransportResult<PreparedTransaction>
    where
        P: Clone,
    {
        let mut tx = self.overprovision_gas_limit(tx, percent).await?;
        if let Some(address) = self.signer_address {
            self.prepared_nonce_seed
                .get_or_try_init(|| async {
                    self.nonce_manager
                        .restore_nonces(&self.provider, address, &[])
                        .await?;
                    // A second confirmed read catches a stale initial seed before any broadcast.
                    // This initialization runs once, before concurrent tracked allocation.
                    self.nonce_manager
                        .reconcile_nonce(&self.provider, address)
                        .await?;
                    Ok::<_, alloy::transports::TransportError>(())
                })
                .await?;
        }
        let nonce = self.assign_nonce(&mut tx).await?;
        let filled = ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(EthereumWallet::from(signer))
            .connect_provider(self.provider.clone())
            .fill(tx)
            .await;
        let result = filled.and_then(|filled| {
            let envelope = filled
                .try_into_envelope()
                .map_err(TransportErrorKind::custom)?;
            PreparedTransaction::decode(envelope.encoded_2718())
        });
        if result.is_err() {
            if let (Some(address), Some(nonce)) = (self.signer_address, nonce) {
                self.nonce_manager.release_nonce(address, nonce).await;
            }
        }
        result
    }

    pub async fn broadcast_prepared(
        &self,
        tx: &PreparedTransaction,
        timeout: Duration,
    ) -> TransportResult<TransactionReceipt> {
        // The caller settles the reservation only after reconciling the outcome.
        tokio::time::timeout(timeout, self.provider.send_raw_transaction_sync(&tx.raw))
            .await
            .map_err(|_| TransportErrorKind::custom_str("eth_sendRawTransactionSync timeout"))?
    }

    pub async fn reconcile_nonce(&self) -> TransportResult<u64> {
        let address = self.signer_address.ok_or_else(|| {
            TransportErrorKind::custom_str("nonce reconciliation requires a signing account")
        })?;
        self.nonce_manager
            .reconcile_nonce(&self.provider, address)
            .await
    }

    pub async fn settle_prepared(&self, tx: &PreparedTransaction, mined: bool) {
        if let Some(address) = self.signer_address {
            if mined {
                self.nonce_manager.confirm_nonce(address, tx.nonce).await;
            } else {
                self.nonce_manager.release_nonce(address, tx.nonce).await;
            }
        }
    }

    /// Call once under exclusive sender ownership, before allocating new nonces.
    pub async fn restore_prepared(&self, transactions: &[Vec<u8>]) -> TransportResult<()> {
        if transactions.is_empty() {
            return Ok(());
        }
        let mut nonces = Vec::new();
        let chain_id = self.get_chain_id().await?;
        for raw in transactions {
            let envelope =
                TxEnvelope::decode_2718(&mut raw.as_slice()).map_err(TransportErrorKind::custom)?;
            if envelope.chain_id() != Some(chain_id)
                || Some(
                    envelope
                        .recover_signer()
                        .map_err(TransportErrorKind::custom)?,
                ) != self.signer_address
            {
                return Err(TransportErrorKind::custom_str(
                    "persisted submission belongs to a different chain or signing account",
                ));
            }
            nonces.push(envelope.nonce());
        }
        if let Some(address) = self.signer_address {
            self.nonce_manager
                .restore_nonces(&self.provider, address, &nonces)
                .await?;
            self.nonce_manager
                .reconcile_nonce(&self.provider, address)
                .await?;
            let _ = self.prepared_nonce_seed.set(());
        }
        Ok(())
    }

    async fn assign_nonce(&self, tx: &mut TransactionRequest) -> TransportResult<Option<u64>> {
        if let Some(address) = self.signer_address {
            let nonce = self
                .nonce_manager
                .get_next_nonce(&self.provider, address)
                .await?;
            tx.nonce = Some(nonce);
            Ok(Some(nonce))
        } else {
            Ok(None)
        }
    }

    async fn finish_nonce<T>(&self, nonce: Option<u64>, result: &TransportResult<T>) {
        let (Some(address), Some(nonce)) = (self.signer_address, nonce) else {
            return;
        };
        // Nodes use different RPC codes for the same nonce rejection. "Already
        // known" retires the nonce locally; the operation still returns an error.
        let retire = match result {
            Ok(_) => true,
            Err(err) => err.as_error_resp().is_some_and(|response| {
                let message = response.message.to_ascii_lowercase();
                matches!(response.code, -32000 | -32003 | -32603)
                    && (message.starts_with("nonce too low")
                        || message.starts_with("already known"))
            }),
        };
        if retire {
            self.nonce_manager.confirm_nonce(address, nonce).await;
        } else {
            // This preserves other in-flight allocations. It does not reconcile
            // an ambiguous broadcast, which may have reached the node already.
            self.nonce_manager.release_nonce(address, nonce).await;
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
    use alloy::{primitives::B256, providers::RootProvider, transports::mock::Asserter};
    use tokio::sync::{mpsc, oneshot};

    type ObservedSend = (u64, oneshot::Sender<TransportResult<TransactionReceipt>>);

    fn receipt() -> TransactionReceipt {
        serde_json::from_value(serde_json::json!({
            "type": "0x2",
            "transactionHash": B256::ZERO,
            "from": Address::ZERO,
            "gasUsed": "0x0",
            "cumulativeGasUsed": "0x0",
            "status": "0x1",
            "logs": [],
            "logsBloom": alloy::primitives::Bloom::ZERO,
        }))
        .unwrap()
    }

    #[derive(Clone)]
    struct ControlledProvider {
        root: RootProvider,
        sends: mpsc::UnboundedSender<ObservedSend>,
    }

    #[async_trait::async_trait]
    impl Provider for ControlledProvider {
        fn root(&self) -> &RootProvider {
            &self.root
        }

        async fn send_transaction_sync(
            &self,
            tx: TransactionRequest,
        ) -> TransportResult<TransactionReceipt> {
            let (reply, result) = oneshot::channel();
            self.sends
                .send((tx.nonce.expect("nonce assigned"), reply))
                .unwrap();
            result.await.expect("test supplies send outcome")
        }

        async fn send_transaction(
            &self,
            tx: TransactionRequest,
        ) -> TransportResult<PendingTransactionBuilder<Ethereum>> {
            self.send_transaction_sync(tx).await?;
            Ok(PendingTransactionBuilder::new(
                self.root.clone(),
                B256::ZERO,
            ))
        }
    }

    fn setup() -> (
        NonceManagedProvider<ControlledProvider>,
        mpsc::UnboundedReceiver<ObservedSend>,
    ) {
        let asserter = Asserter::new();
        asserter.push_success(&"0xa");
        let (sends, receiver) = mpsc::unbounded_channel();
        let provider = ControlledProvider {
            root: alloy::providers::ProviderBuilder::default().connect_mocked_client(asserter),
            sends,
        };
        (
            NonceManagedProvider::new(provider, Some(Address::ZERO)),
            receiver,
        )
    }

    fn send(
        provider: NonceManagedProvider<ControlledProvider>,
    ) -> tokio::task::JoinHandle<TransportResult<TransactionReceipt>> {
        tokio::spawn(async move {
            provider
                .send_transaction_sync(TransactionRequest::default(), Duration::from_secs(5))
                .await
        })
    }

    async fn next_send(receiver: &mut mpsc::UnboundedReceiver<ObservedSend>) -> ObservedSend {
        tokio::time::timeout(Duration::from_secs(5), receiver.recv())
            .await
            .unwrap()
            .unwrap()
    }

    #[tokio::test]
    async fn failed_send_reuses_its_nonce_without_resetting_other_in_flight_sends() {
        let (provider, mut receiver) = setup();
        let first = send(provider.clone());
        let (first_nonce, first_reply) = next_send(&mut receiver).await;
        let second = send(provider.clone());
        let (second_nonce, second_reply) = next_send(&mut receiver).await;
        assert_eq!((first_nonce, second_nonce), (10, 11));

        first_reply
            .send(Err(TransportErrorKind::custom_str("send rejected")))
            .unwrap();
        assert!(first.await.unwrap().is_err());

        let retry = send(provider.clone());
        let (retry_nonce, retry_reply) = next_send(&mut receiver).await;
        assert_eq!(retry_nonce, first_nonce);
        retry_reply.send(Ok(receipt())).unwrap();
        retry.await.unwrap().unwrap();

        let fresh = send(provider.clone());
        let (fresh_nonce, fresh_reply) = next_send(&mut receiver).await;
        assert_eq!(
            fresh_nonce, 12,
            "nonce 11 remains reserved by the second send"
        );
        fresh_reply.send(Ok(receipt())).unwrap();
        fresh.await.unwrap().unwrap();
        second_reply.send(Ok(receipt())).unwrap();
        second.await.unwrap().unwrap();
        let state = provider
            .nonce_manager
            .get_account_details(Address::ZERO)
            .await
            .unwrap();
        assert!(state.locked_nonces.is_empty());
        assert!(state.available_nonces.is_empty());
    }

    #[tokio::test]
    async fn local_timeout_settles_the_reservation_outside_the_cancelled_rpc() {
        let (provider, mut receiver) = setup();
        let sender = provider.clone();
        let attempt = tokio::spawn(async move {
            sender
                .send_transaction_sync(TransactionRequest::default(), Duration::ZERO)
                .await
        });
        let (nonce, reply) = next_send(&mut receiver).await;
        let error = attempt.await.unwrap().unwrap_err();
        assert_eq!(error.to_string(), "eth_sendRawTransactionSync timeout");
        assert!(reply.is_closed(), "RPC future must have been cancelled");
        let state = provider
            .nonce_manager
            .get_account_details(Address::ZERO)
            .await
            .unwrap();
        assert!(state.locked_nonces.is_empty());
        assert_eq!(state.available_nonces, [nonce].into());

        let retry = send(provider);
        let (retry_nonce, reply) = next_send(&mut receiver).await;
        assert_eq!(retry_nonce, nonce);
        reply.send(Ok(receipt())).unwrap();
        retry.await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn known_nonce_errors_retire_the_nonce_but_preserve_the_operation_error() {
        for (code, message) in [-32000, -32003, -32603]
            .into_iter()
            .flat_map(|code| ["nonce too low", "already known"].map(|message| (code, message)))
        {
            let (provider, mut receiver) = setup();
            let attempt = send(provider.clone());
            let (nonce, reply) = next_send(&mut receiver).await;
            let error = alloy::transports::RpcError::ErrorResp(
                serde_json::from_value(serde_json::json!({"code": code, "message": message}))
                    .unwrap(),
            );
            reply.send(Err(error)).unwrap();
            assert!(attempt.await.unwrap().is_err());
            let retry = send(provider);
            let (next_nonce, reply) = next_send(&mut receiver).await;
            assert_eq!(next_nonce, nonce + 1);
            reply.send(Ok(receipt())).unwrap();
            retry.await.unwrap().unwrap();
        }
    }

    #[tokio::test]
    async fn other_rpc_errors_release_the_nonce_for_retry() {
        for (code, message) in [
            (-32000, "nonce too high"),
            (-32003, "replacement transaction underpriced"),
            (-32603, "execution reverted: nonce too low"),
            (-32602, "nonce too low"),
        ] {
            let (provider, mut receiver) = setup();
            let attempt = send(provider.clone());
            let (nonce, reply) = next_send(&mut receiver).await;
            let error = alloy::transports::RpcError::ErrorResp(
                serde_json::from_value(serde_json::json!({"code": code, "message": message}))
                    .unwrap(),
            );
            reply.send(Err(error)).unwrap();
            assert!(attempt.await.unwrap().is_err());

            let retry = send(provider);
            let (retry_nonce, reply) = next_send(&mut receiver).await;
            assert_eq!(retry_nonce, nonce);
            reply.send(Ok(receipt())).unwrap();
            retry.await.unwrap().unwrap();
        }
    }

    #[tokio::test]
    async fn asynchronous_send_releases_failures_and_retires_successes() {
        let (provider, mut receiver) = setup();
        for (expected_nonce, fail) in [(10, true), (10, false), (11, false)] {
            let sender = provider.clone();
            let attempt = tokio::spawn(async move {
                sender.send_transaction(TransactionRequest::default()).await
            });
            let (nonce, reply) = next_send(&mut receiver).await;
            assert_eq!(nonce, expected_nonce);
            let result = if fail {
                Err(TransportErrorKind::custom_str("send rejected"))
            } else {
                Ok(receipt())
            };
            reply.send(result).unwrap();
            assert_eq!(attempt.await.unwrap().is_err(), fail);
        }
    }
}
