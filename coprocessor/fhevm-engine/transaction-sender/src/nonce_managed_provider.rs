use std::{sync::Arc, time::Duration};

use alloy::providers::PendingTransactionError;
use alloy::{
    network::{Ethereum, TransactionBuilder},
    primitives::Address,
    providers::{
        fillers::{BlobGasFiller, ChainIdFiller, GasFiller, JoinFill},
        PendingTransactionBuilder, Provider,
    },
    rpc::types::{TransactionReceipt, TransactionRequest},
    transports::{TransportErrorKind, TransportResult},
};
use futures_util::lock::Mutex;
use tokio::sync::Semaphore;
use tracing::{debug, warn};

use crate::config::DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT;

pub type FillersWithoutNonceManagement =
    JoinFill<GasFiller, JoinFill<BlobGasFiller, ChainIdFiller>>;

/// Default number of *active attempts* a single signer may have at once.
///
/// A permit is held for the whole attempt: gas estimation, the wait for the
/// nonce mutex, submission, and the wait for inclusion. It therefore bounds
/// concurrent `eth_estimateGas` calls, which are otherwise one per selected row.
///
/// It does **not** bound outstanding on-chain transactions. When an attempt
/// gives up on its receipt the permit is released while that transaction may
/// still be pending at the node, so unmined work can exceed this limit - see
/// `outstanding_transactions_can_exceed_the_admission_limit`. Nothing in this
/// provider caps unmined transactions; the operation layer's retry accounting
/// is what eventually resolves them.
pub const DEFAULT_MAX_INFLIGHT_SENDS: usize = 8;

/// Local nonce sequence for a single signer.
///
/// Replaces `alloy`'s `CachedNonceManager`, which in alloy-provider 1.x seeds
/// itself from `get_transaction_count(address)` at `latest` — i.e. counting only
/// *mined* transactions. Combined with the previous "reset the manager on any
/// send error" behaviour, that handed out nonces already sitting in the mempool
/// and produced `nonce too low` / `already known` / `replacement transaction
/// underpriced` storms.
#[derive(Default)]
struct NonceSequence {
    /// Next nonce to hand out, or `None` when it must be re-seeded from the chain.
    next: Option<u64>,
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
    /// Orders nonce allocation and submission for this signer's nonce sequence.
    ///
    /// Allocation and *submission* must not be separated. A nonce handed to a
    /// send that never reaches the mempool leaves a hole, and every higher nonce
    /// already handed out is then rejected as `nonce too high` by any node that
    /// does not queue gapped transactions - which stalls the whole sequence
    /// rather than failing one transaction. Holding this lock across submission
    /// makes that impossible.
    ///
    /// Inclusion is *not* covered. Once the node acknowledges the raw
    /// transaction the nonce is settled, so waiting for the receipt under this
    /// lock would add nothing to ordering while capping the signer at one
    /// transaction per block. Outstanding transactions are bounded by the
    /// admission semaphore instead; see `DEFAULT_MAX_INFLIGHT_SENDS`.
    ///
    /// Nonce lookup and submission are bounded separately by
    /// `send_txn_sync_timeout_secs`, excluding queue wait and estimation.
    nonce_seq: Arc<Mutex<NonceSequence>>,
    signer_address: Option<Address>,
    /// Bounds concurrent gas estimation, which happens before the nonce lock and
    /// is otherwise unbounded: one task per selected row.
    inflight: Arc<Semaphore>,
}

impl<P> NonceManagedProvider<P>
where
    P: Provider<Ethereum>,
{
    pub fn new(provider: P, signer_address: Option<Address>) -> Self {
        Self {
            provider,
            nonce_seq: Default::default(),
            signer_address,
            inflight: Arc::new(Semaphore::new(DEFAULT_MAX_INFLIGHT_SENDS)),
        }
    }

    /// Overrides how many transactions may be prepared/submitted concurrently.
    pub fn with_max_inflight(mut self, max_inflight: usize) -> Self {
        let max_inflight = if max_inflight == 0 {
            warn!(
                default_max_inflight = DEFAULT_MAX_INFLIGHT_SENDS,
                "max_inflight_sends is 0, using default value instead"
            );
            DEFAULT_MAX_INFLIGHT_SENDS
        } else {
            max_inflight
        };
        self.inflight = Arc::new(Semaphore::new(max_inflight));
        self
    }

    /// Returns the nonce to use, seeding from the chain when the sequence is cold.
    ///
    /// Seeds from `pending` so that transactions already broadcast but not yet
    /// mined are counted. `latest` would hand back a nonce that is already in the
    /// mempool.
    async fn next_nonce(
        &self,
        seq: &mut NonceSequence,
        signer_address: Address,
    ) -> TransportResult<u64> {
        if let Some(nonce) = seq.next {
            return Ok(nonce);
        }
        let nonce = self
            .provider
            .get_transaction_count(signer_address)
            .pending()
            .await?;
        debug!(
            nonce = nonce,
            signer_address = %signer_address,
            "Seeded nonce sequence from pending transaction count"
        );
        seq.next = Some(nonce);
        Ok(nonce)
    }

    /// Records the outcome of a send against the sequence.
    ///
    /// On success the sequence advances. On failure it is invalidated rather than
    /// rewound: the next send re-seeds from `pending`, including transactions
    /// visible in the queried node's mempool. This snapshot does not resolve
    /// every ambiguous submission outcome. Rewinding to `nonce` would reuse a nonce that a
    /// client-side timeout had merely failed to *observe*.
    fn record_outcome<T, E>(seq: &mut NonceSequence, nonce: u64, res: &Result<T, E>) {
        match res {
            Ok(_) => seq.next = Some(nonce + 1),
            Err(_) => seq.next = None,
        }
    }

    pub async fn send_transaction(
        &self,
        tx: impl Into<TransactionRequest>,
    ) -> TransportResult<PendingTransactionBuilder<Ethereum>> {
        let mut tx = tx.into();
        let Some(signer_address) = self.signer_address else {
            return self.provider.send_transaction(tx).await;
        };
        let mut seq = self.nonce_seq.lock().await;
        let nonce = self.next_nonce(&mut seq, signer_address).await?;
        tx.nonce = Some(nonce);
        let res = self.provider.send_transaction(tx).await;
        Self::record_outcome(&mut seq, nonce, &res);
        res
    }

    /// Submits under the nonce mutex and waits for inclusion *outside* it.
    ///
    /// The mutex is held only for nonce selection and submission acknowledgment,
    /// which is what the sequence actually needs to stay gap-free: once the node
    /// has accepted the raw transaction, the next nonce is settled and the next
    /// caller may proceed. Waiting for inclusion inside the mutex instead made
    /// throughput one transaction per block for the whole signer, across both
    /// operations, because the node cannot include a transaction it has not been
    /// offered yet.
    ///
    /// Submission uncertainty and receipt uncertainty are deliberately handled
    /// differently:
    ///
    /// * A submission error or timeout leaves acceptance *unknown*, so the
    ///   sequence is invalidated and the next attempt re-seeds from `pending`.
    /// * A receipt timeout concerns a transaction the node already acknowledged.
    ///   Later transactions hold valid, higher nonces, so the sequence must not
    ///   be touched - resetting here would hand out nonces that are legitimately
    ///   outstanding and reproduce the `already known` / `nonce too low` storm
    ///   this provider exists to prevent. The operation layer retries the row.
    ///
    /// Known gap, unchanged from the serialized design: if an acknowledged
    /// transaction is later dropped from the mempool without being mined, the
    /// nonces already handed out above it cannot be included until that hole is
    /// filled. Recovery is indirect - a subsequent submission is rejected
    /// (`nonce too high` on nodes that do not queue gapped transactions), which
    /// invalidates the sequence and re-seeds from `pending`. Durable
    /// reconciliation is out of scope here.
    pub async fn send_transaction_sync(
        &self,
        tx: impl Into<TransactionRequest>,
        submit_timeout: Duration,
        receipt_timeout: Duration,
    ) -> TransportResult<TransactionReceipt> {
        let mut tx = tx.into();
        let Some(signer_address) = self.signer_address else {
            let pending =
                Self::with_submit_timeout(self.provider.send_transaction(tx), submit_timeout)
                    .await?;
            return self.await_receipt(pending, receipt_timeout).await;
        };

        // ---- ordered phase: nonce selection and submission acknowledgment ----
        let pending = {
            let mut seq = self.nonce_seq.lock().await;
            // A cold lookup holds the shared lock too. Bound it separately so a
            // stalled pending-count RPC cannot block both operation loops
            // forever. Until this completes, this attempt has not submitted.
            let nonce =
                tokio::time::timeout(submit_timeout, self.next_nonce(&mut seq, signer_address))
                    .await
                    .map_err(|_| {
                        TransportErrorKind::custom_str("eth_getTransactionCount(pending) timeout")
                    })??;
            tx.nonce = Some(nonce);

            let submitted =
                Self::with_submit_timeout(self.provider.send_transaction(tx), submit_timeout).await;
            Self::record_outcome(&mut seq, nonce, &submitted);
            submitted?
            // The guard drops here: the next caller may take its nonce while
            // this transaction is still waiting to be mined.
        };

        // ---- unordered phase: inclusion ----
        self.await_receipt(pending, receipt_timeout).await
    }

    /// Waits for inclusion without holding the nonce sequence.
    ///
    /// The whole phase is bounded by `receipt_timeout`, including the final
    /// direct lookup: a small budget is carved out of it rather than added to
    /// it, so a stalled receipt RPC cannot park an attempt - and its admission
    /// permit - past the deadline the operator configured. An unbounded lookup
    /// here would reintroduce exactly the blocking hole the nonce-lookup
    /// amendment closed.
    ///
    /// On timeout the transaction remains acknowledged by the node and may still
    /// be mined; the caller must treat that as an unresolved attempt, not as a
    /// free nonce.
    async fn await_receipt(
        &self,
        pending: PendingTransactionBuilder<Ethereum>,
        receipt_timeout: Duration,
    ) -> TransportResult<TransactionReceipt> {
        let tx_hash = *pending.tx_hash();
        // Reserve a slice of the deadline for the final lookup below.
        let final_budget = (receipt_timeout / 10).clamp(
            Duration::from_millis(250).min(receipt_timeout),
            Duration::from_secs(2),
        );
        let watch_budget = receipt_timeout.saturating_sub(final_budget);

        // Dropping get_receipt() only drops its waiting receiver. Alloy's
        // heartbeat retains the registered watcher unless it has its own expiry.
        // Keep the outer deadline/fallback too: registration and receipt RPCs
        // can stall independently of the heartbeat's internal timeout.
        let pending = pending.with_timeout(Some(receipt_timeout));
        match tokio::time::timeout(watch_budget, pending.get_receipt()).await {
            Ok(Ok(receipt)) => Ok(receipt),
            // Preserve the underlying transport error so the existing
            // `is_backend_gone` classification still recognizes a receipt-stage
            // backend failure. Wrapping it in a formatted string erased the
            // type and silently disabled the stop-and-restart path.
            Ok(Err(PendingTransactionError::TransportError(e))) => {
                warn!(%tx_hash, error = %e, "Transport error while watching for receipt");
                Err(e)
            }
            Ok(Err(e)) => {
                warn!(%tx_hash, error = %e, "Receipt watch failed");
                Err(TransportErrorKind::custom_str(&format!(
                    "receipt watch failed: {e}"
                )))
            }
            Err(_) => {
                // Last look before giving up: the watcher can miss a receipt that
                // landed while it was being registered. Bounded, and its own
                // errors are surfaced rather than swallowed.
                match tokio::time::timeout(
                    final_budget,
                    self.provider.get_transaction_receipt(tx_hash),
                )
                .await
                {
                    Ok(Ok(Some(receipt))) => Ok(receipt),
                    Ok(Ok(None)) => {
                        warn!(%tx_hash, "Receipt deadline reached; transaction still unmined");
                        Err(TransportErrorKind::custom_str("receipt timeout"))
                    }
                    Ok(Err(e)) => {
                        // Same reasoning as above: keep the transport error.
                        warn!(%tx_hash, error = %e, "Final receipt lookup failed");
                        Err(e)
                    }
                    Err(_) => {
                        warn!(%tx_hash, "Final receipt lookup timed out");
                        Err(TransportErrorKind::custom_str("receipt timeout"))
                    }
                }
            }
        }
    }

    async fn with_submit_timeout<T>(
        fut: impl std::future::Future<Output = TransportResult<T>>,
        timeout: Duration,
    ) -> TransportResult<T> {
        tokio::time::timeout(timeout, fut)
            .await
            // Acceptance is unknown when this fires: the request may have
            // reached the node. Reported distinctly from a receipt timeout.
            .map_err(|_| TransportErrorKind::custom_str("eth_sendRawTransaction timeout"))
            .flatten()
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
        submit_timeout: Duration,
        gas_estimation_timeout: Duration,
        receipt_timeout: Duration,
    ) -> TransportResult<alloy::rpc::types::TransactionReceipt> {
        // Bound how many transactions may be in preparation at once. Without
        // this, one task per selected row runs `estimate_gas` concurrently.
        let _permit = self
            .inflight
            .acquire()
            .await
            .map_err(|_| TransportErrorKind::custom_str("in-flight send semaphore closed"))?;
        // `estimate_gas` is otherwise unbounded while the provider is configured
        // with a very high retry count, so a degraded RPC could park a task
        // indefinitely while holding its permit.
        // Estimation has its own deadline. The binary leaves gas unset on all
        // attempts so terminal revert handling can recognize completed work.
        // Pre-set gas remains supported for existing library callers.
        let overprovisioned_txn = tokio::time::timeout(
            gas_estimation_timeout,
            self.overprovision_gas_limit(txn_request, percent),
        )
        .await
        .map_err(|_| TransportErrorKind::custom_str("gas estimation timeout"))
        .flatten()?;
        self.send_transaction_sync(overprovisioned_txn, submit_timeout, receipt_timeout)
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
