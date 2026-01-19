use alloy::{
    network::{
        AnyNetwork, AnyTransactionReceipt, Ethereum, EthereumWallet, TransactionBuilder, TxSigner,
    },
    primitives::{Address, Bytes, U256},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, GasFiller, JoinFill, NonceFiller, TxFiller, WalletFiller,
        },
        Identity, Network, Provider, ProviderBuilder, RootProvider,
    },
    rpc::{json_rpc::ErrorPayload, types::TransactionRequest},
    signers::{Signature, Signer},
    transports::{http::reqwest::Url, RpcError},
};
use anyhow::Result;
use std::{fmt::Debug, sync::Arc, time::Instant};
use thiserror::Error;
use tokio::sync::Semaphore;
use tracing::{debug, error, info, warn};

use crate::{
    config::settings::{BlockchainRpcConfig, TxEngineConfig},
    core::job_id::JobId,
    gateway::arbitrum::{
        parse_private_key,
        transaction::{
            nonce_manager::NonceManagerNonOptimistic,
            provider::NonceManagedProvider,
            selectors::{
                SELECTOR_ACCOUNT_NOT_ALLOWED_TO_USE_CIPHERTEXT,
                SELECTOR_CIPHERTEXT_MATERIAL_NOT_READY, SELECTOR_PUBLIC_DECRYPT_NOT_ALLOWED,
            },
        },
    },
    logging::TxEngineStep,
    metrics,
};

pub trait SignerCombined: TxSigner<Signature> + Signer + Send + Sync + Debug {}

// This implementation doesn't need to change. It will still work for any `T`
// as long as `T` now also implements `Debug`.
impl<T: TxSigner<Signature> + Signer + Send + Sync + Debug> SignerCombined for T {}

// TODO: Rework this, with a clean triage.
#[derive(Error, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum GatewayTxnError {
    #[error("Invalid contract address: {0}")]
    InvalidAddress(String),

    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    // Will be useful when adding timeout.
    #[error("Transaction timeout after {0} seconds")]
    TransactionTimeout(u64),

    // TODO: Review this error.
    #[error("Transaction simulation failed: {0}")]
    SimulationFailed(String),

    // TODO: After max retries, we report the service unhealthy, and expect human intervention.
    // Special status retry later, and all tx sender (in-flight and pending should not be send)
    #[error("Transport error: {0}")]
    TransportError(String),
}

impl From<eyre::Report> for GatewayTxnError {
    fn from(err: eyre::Report) -> Self {
        GatewayTxnError::RpcError(err.to_string())
    }
}

pub type CustomFillers = JoinFill<
    JoinFill<
        JoinFill<
            JoinFill<
                Identity,
                JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
            >,
            GasFiller,
        >,
        ChainIdFiller,
    >,
    WalletFiller<EthereumWallet>,
>;

#[derive(Debug, Clone)]
pub struct TransactionEngine<F, P, N = AnyNetwork>
where
    N: Network,
    F: TxFiller<N>,
    P: Provider<N>,
{
    pub provider: Arc<NonceManagedProvider<F, P, N>>,
    pub signer: Arc<dyn SignerCombined>,
    pub nonce_manager: Arc<NonceManagerNonOptimistic>,
    // No need for Arc in this case, since the tx manager is shared by arc at the top level application.
    pub rpc_semaphore: Arc<Semaphore>,
    // NOTE: values of 100 for both are handling 1000 parallel transaction on gw.
    // 3 render 0 of success
    ms_retry_delay: u64,
    tx_max_retries: u32,
    gas_estimation_max_retries: u32,
}

impl
    TransactionEngine<
        CustomFillers, // F: Assumes your manager uses the standard NonceFiller.
        RootProvider,  // P: The concrete `DynProvider` wrapper, not `dyn Provider`.
        Ethereum,      // N: The network.
    >
{
    pub fn new(
        blockchain_rpc_config: BlockchainRpcConfig,
        tx_engine_config: TxEngineConfig,
    ) -> Self {
        let mut signer = parse_private_key(&tx_engine_config.private_key).unwrap();
        signer.set_chain_id(Some(blockchain_rpc_config.chain_id));

        // Clone the signer for multiple consumers
        let signer = Arc::new(signer);

        let signer_address = <dyn SignerCombined as alloy::signers::Signer>::address(&*signer);
        let wallet = EthereumWallet::from(signer.clone());

        let rpc_url = Url::parse(&blockchain_rpc_config.http_url)
            .map_err(|e| GatewayTxnError::InvalidAddress(format!("Invalid URL: {e}")))
            .unwrap();

        let provider = ProviderBuilder::new()
            .network::<Ethereum>() // Use the concrete network type
            .filler(GasFiller)
            .filler(ChainIdFiller::new(Some(blockchain_rpc_config.chain_id)))
            .filler(WalletFiller::new(wallet))
            .connect_http(rpc_url.clone());

        let nonce_manager = Arc::new(NonceManagerNonOptimistic::new());
        let managed_provider =
            NonceManagedProvider::new(provider, signer_address, nonce_manager.clone());

        Self {
            provider: Arc::new(managed_provider),
            signer,
            nonce_manager,
            rpc_semaphore: Arc::new(Semaphore::new(tx_engine_config.max_concurrency as usize)),
            ms_retry_delay: tx_engine_config.retry.retry_interval_ms,
            tx_max_retries: tx_engine_config.retry.max_attempts,
            gas_estimation_max_retries: tx_engine_config.retry.max_attempts,
        }
    }

    pub fn sender_address(&self) -> Address {
        self.signer.address()
    }

    pub async fn prepare_transaction(
        &self,
        job_id: &JobId,
        target: Address,
        calldata: Bytes,
        // TODO: Remove value with None value.
        value: Option<U256>,
    ) -> Result<TransactionRequest, GatewayTxnError> {
        // TODO: Check for allowance (this account or other accounts) for fees.
        let code = self.provider.inner.get_code_at(target).await.map_err(|e| {
            GatewayTxnError::TransactionFailed(format!("Failed to check contract code: {e}"))
        })?;

        if code.is_empty() {
            metrics::track_engine_error(metrics::TransactionErrorType::InvalidAddress);
            error!("No code at target address: {:?} !", target);
            return Err(GatewayTxnError::InvalidAddress(format!(
                "No code at target address: {target:#x}"
            )));
        }

        let mut request = TransactionRequest::default()
            .with_from(self.sender_address())
            .with_to(target)
            .with_input(calldata.clone())
            .with_value(value.unwrap_or_default());

        let gas_limit_estimate = match self
            .estimate_gas(job_id, target, calldata.clone(), value)
            .await
        {
            Ok(gas) => gas,
            Err(e) => {
                // If gas estimation fails with an unrecoverable error, we must not proceed.
                error!(
                    int_job_id = %job_id,
                    step = %TxEngineStep::TxFailed,
                    error = ?e,
                    "Gas estimation failed"
                );
                return Err(e);
            }
        };
        // TODO: Balance (of signer) before sending a transaction for gas with a buffer as we used in estimateGas
        request = request.with_gas_limit(gas_limit_estimate);

        debug!(
            int_job_id = %job_id,
            step = %TxEngineStep::TxPrepared,
            gas_limit = gas_limit_estimate,
            target = ?target,
            "Transaction prepared"
        );

        Ok(request)
    }

    pub async fn estimate_gas(
        &self,
        job_id: &JobId,
        target: Address,
        calldata: Bytes,
        value: Option<U256>,
    ) -> Result<u64, GatewayTxnError> {
        let request = TransactionRequest::default()
            .with_from(self.sender_address())
            .with_to(target)
            .with_input(calldata)
            .with_value(value.unwrap_or_default());

        let mut retries = 0;
        loop {
            debug!("Number of retries for gas estimation: {}", retries);
            if retries >= self.gas_estimation_max_retries {
                metrics::track_engine_error(metrics::TransactionErrorType::MaxRetriesExceeded);
                return Err(GatewayTxnError::TransactionFailed(format!(
                    "Gas estimation failed after {} retries",
                    self.gas_estimation_max_retries
                )));
            }

            let _permit = match self.rpc_semaphore.acquire().await {
                Ok(p) => p,
                Err(_) => {
                    // This error is fatal. It means the semaphore was closed,
                    // which should not happen during normal operation.
                    error!(
                        int_job_id = %job_id,
                        step = %TxEngineStep::TxRetrying,
                        alert = true,
                        "RPC semaphore closed during gas estimation"
                    );
                    retries += 1;
                    continue;
                }
            };

            let res = self.provider.inner.estimate_gas(request.clone()).await;
            // TODO, find a way to drop semaphore right after the call.
            // drop(permit);

            match res {
                Ok(gas) => {
                    // NOTE: shouldn't this be exposed as tx-manager configuration?
                    // Add 20% buffer to estimated gas
                    let gas_with_buffer = (gas as f64 * 1.2) as u64;
                    return Ok(gas_with_buffer);
                }
                Err(e) => {
                    let err_msg = e.to_string();
                    match e {
                        // This is a blockchain-level error (e.g., revert).
                        // These are generally not recoverable by retrying.
                        RpcError::ErrorResp(ErrorPayload { message, .. }) => {
                            let response_error_string = message.to_lowercase();

                            // Control flow for ACL check passed error due to RPC node inconsistency.
                            // Should NEVER happen with consistent state.
                            // NOTE (Nico): This control flow makes the tx engine less generic.
                            if response_error_string
                                .contains(SELECTOR_CIPHERTEXT_MATERIAL_NOT_READY)
                                || response_error_string
                                    .contains(SELECTOR_PUBLIC_DECRYPT_NOT_ALLOWED)
                                || response_error_string
                                    .contains(SELECTOR_ACCOUNT_NOT_ALLOWED_TO_USE_CIPHERTEXT)
                            {
                                // Not passing in loop initial check to get a max retry exceeded fails 1 retry before.
                                if retries >= self.gas_estimation_max_retries - 1 {
                                    metrics::track_engine_error(
                                        metrics::TransactionErrorType::RevertedACLSelector,
                                    );
                                    error!(error = %err_msg, alert=true, "Simulation failed due to revert for RPC node inconsistent state and ACL readiness: Unrecoverable");
                                    return Err(GatewayTxnError::SimulationFailed(format!(
                                        "Execution reverted: {}",
                                        message
                                    )));
                                }
                                warn!(
                                    int_job_id = %job_id,
                                    step = %TxEngineStep::TxRetrying,
                                    retries = %retries,
                                    error = %err_msg,
                                    "Simulation reverted due to RPC node inconsistency"
                                );
                                retries += 1;
                                tokio::time::sleep(std::time::Duration::from_millis(
                                    self.ms_retry_delay,
                                ))
                                .await;
                                continue;
                            }

                            // Classic reverts control flow.
                            if response_error_string.contains("execution reverted") {
                                metrics::track_engine_error(
                                    metrics::TransactionErrorType::Reverted,
                                );
                                error!(error = %err_msg, "Simulation failed due to revert: Likely unrecoverable");
                                return Err(GatewayTxnError::SimulationFailed(format!(
                                    "Execution reverted: {}",
                                    message
                                )));
                            }
                            // For other RPC errors, we will treat them as potentially transient for now and retry.
                            // You can add more specific checks here for other fatal errors.
                            metrics::track_engine_error(metrics::TransactionErrorType::Rpc);
                            warn!(error = %err_msg, "Gas estimation failed with RPC error");
                            // For now since we don't know what is going on we revert !
                            return Err(GatewayTxnError::RpcError(err_msg));
                        }
                        // This is a network/transport error. It's recoverable, and should be retried.
                        RpcError::Transport(transport_err) => {
                            metrics::track_engine_error(metrics::TransactionErrorType::Transport);
                            warn!(
                                int_job_id = %job_id,
                                step = %TxEngineStep::TxRetrying,
                                error = %transport_err,
                                "Transport error during gas estimation"
                            );
                        }
                        _ => {
                            metrics::track_engine_error(metrics::TransactionErrorType::Unknown);
                            error!(error = %err_msg, "Unknown error during gas estimation");
                            return Err(GatewayTxnError::RpcError(err_msg));
                        }
                    }
                }
            }
            // If we've reached here, it means we are retrying.
            retries += 1;
            tokio::time::sleep(std::time::Duration::from_millis(self.ms_retry_delay)).await;
        }
    }

    // TODO: Add gas bump
    // TODO: Match all thoses errors code, and make a triage accordingly: https://ethereum-json-rpc.com/errors + Combine with parsing error message for get a clear triage.
    pub async fn send_raw_transaction_sync_with_retries(
        &self,
        job_id: &JobId,
        mut tx: TransactionRequest,
    ) -> Result<AnyTransactionReceipt, GatewayTxnError> {
        let pending_receipt: AnyTransactionReceipt;
        let start_time = Instant::now();
        let mut retries = 0;

        loop {
            // We could add a max number of retries here.
            if retries >= self.tx_max_retries {
                metrics::track_engine_error(metrics::TransactionErrorType::MaxRetriesExceeded);
                return Err(GatewayTxnError::TransactionFailed(format!(
                    "Transaction failed after {} retries.",
                    self.tx_max_retries
                )));
            }
            // TODO: timeout retry delay
            // TODO: have a mutable variable that reference the previous error scenario here, so when we go to the next iteration

            debug!("Number of retries for send raw tx sync: {}", retries);

            let signer_addr = self.sender_address();

            let nonce_result = self
                .nonce_manager
                .get_increase_and_lock_nonce(&self.provider.inner, signer_addr)
                .await;

            let nonce = match nonce_result {
                Ok(n) => n,
                Err(e) => {
                    // If getting the nonce fails (e.g., a transport error while fetching the
                    // initial transaction count), log it and retry the whole loop.
                    warn!(
                        int_job_id = %job_id,
                        step = %TxEngineStep::TxRetrying,
                        error = %e,
                        "Nonce acquisition failed"
                    );
                    retries += 1;
                    // Wait a bit before the next attempt to avoid hammering the nonce manager again.
                    tokio::time::sleep(std::time::Duration::from_millis(self.ms_retry_delay)).await;
                    continue; // Jump to the next loop iteration.
                }
            };

            debug!(
                int_job_id = %job_id,
                step = %TxEngineStep::NonceAcquired,
                nonce = nonce,
                "Nonce acquired"
            );

            // SETTING THE NONCE BEFORE SENDING THE TX.
            tx.set_nonce(nonce);

            let _permit = match self.rpc_semaphore.acquire().await {
                Ok(p) => p,
                Err(_) => {
                    // This error is fatal. It means the semaphore was closed,
                    // which should not happen during normal operation.
                    error!(
                        int_job_id = %job_id,
                        step = %TxEngineStep::TxRetrying,
                        alert = true,
                        "RPC semaphore closed during tx send"
                    );
                    retries += 1;
                    continue;
                }
            };

            debug!(
                int_job_id = %job_id,
                step = %TxEngineStep::TxSending,
                nonce = nonce,
                "Submitting transaction"
            );

            let result = self.provider.send_raw_transaction_sync(tx.clone()).await;
            // TODO: find a way to drop the permit right after the rpc call, even with the condition.
            // drop(permit);

            match result {
                Ok(receipt) => {
                    self.nonce_manager
                        .confirm_nonce(self.sender_address(), nonce)
                        .await;
                    pending_receipt = receipt;
                    break; // Exit the loop on success.
                }
                Err(e) => {
                    // This part is now correctly handling errors during the transaction broadcast.
                    let err_msg = e.to_string();

                    match e {
                        // For now we don't use code, but we could.
                        RpcError::ErrorResp(ErrorPayload {
                            code: _, message, ..
                        }) => {
                            let response_error_string = message.to_lowercase();
                            if response_error_string.contains("nonce too low")
                                || response_error_string.contains("already known")
                            {
                                metrics::track_engine_error(metrics::TransactionErrorType::Nonce);
                                warn!(
                                    int_job_id = %job_id,
                                    step = %TxEngineStep::TxRetrying,
                                    nonce = tx.nonce,
                                    error = %err_msg,
                                    "Nonce too low"
                                );
                                self.nonce_manager
                                    .confirm_nonce(self.sender_address(), nonce)
                                    .await;
                            } else if response_error_string.contains("nonce too high") {
                                metrics::track_engine_error(metrics::TransactionErrorType::Nonce);
                                warn!(
                                    int_job_id = %job_id,
                                    step = %TxEngineStep::TxRetrying,
                                    nonce = tx.nonce,
                                    error = %err_msg,
                                    "Nonce too high"
                                );
                                self.nonce_manager
                                    .release_nonce(self.sender_address(), nonce)
                                    .await;
                            } else {
                                // NOTE: be careful here: Now we are letting pass inconsistent node state and retrying for some reverts,
                                // could result in this flow, if inconsistency is propagated here, ok for nonce release.
                                metrics::track_engine_error(metrics::TransactionErrorType::Rpc);
                                // TODO create a proper Rpc error response triage error: e.g, "transaction underpriced" and so on.
                                error!("Non-handled RPC error response: {:?}", err_msg);
                                self.nonce_manager
                                    .release_nonce(self.sender_address(), nonce)
                                    .await;
                                // For unexpected blockchain errors, we might want to fail fast.
                                return Err(GatewayTxnError::TransactionFailed(err_msg));
                            }
                        }
                        RpcError::Transport(transport_err) => {
                            metrics::track_engine_error(metrics::TransactionErrorType::Transport);
                            warn!(
                                int_job_id = %job_id,
                                step = %TxEngineStep::TxRetrying,
                                nonce = tx.nonce,
                                error = %transport_err,
                                "Transport error"
                            );
                            self.nonce_manager
                                .release_nonce(self.sender_address(), nonce)
                                .await;
                        }
                        // TODO: Add an exhaustive triage error here
                        _ => {
                            metrics::track_engine_error(metrics::TransactionErrorType::Unknown);
                            error!(nonce = tx.nonce, "Unknown RPC error: {:?}", err_msg);
                            self.nonce_manager
                                .release_nonce(self.sender_address(), nonce)
                                .await;
                            // Fail on truly unknown errors.
                            return Err(GatewayTxnError::RpcError(err_msg));
                        }
                    }

                    // If we haven't returned, we are retrying.
                    retries += 1;
                    tokio::time::sleep(std::time::Duration::from_millis(self.ms_retry_delay)).await;
                    continue;
                }
            }
        }

        let elapsed_time = format!("{:?}ms", start_time.elapsed().as_millis());
        info!(
            int_job_id = %job_id,
            step = %TxEngineStep::TxSent,
            elapsed_time = elapsed_time,
            nonce = tx.nonce,
            tx_hash = ?pending_receipt.transaction_hash,
            "Transaction confirmed"
        );
        Ok(pending_receipt)
    }
}
