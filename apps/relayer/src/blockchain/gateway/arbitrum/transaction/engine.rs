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

use crate::blockchain::gateway::arbitrum::transaction::{
    nonce_manager::ZamaNonceManager, provider::NonceManagedProvider,
};

pub trait SignerCombined: TxSigner<Signature> + Signer + Send + Sync + Debug {}

// This implementation doesn't need to change. It will still work for any `T`
// as long as `T` now also implements `Debug`.
impl<T: TxSigner<Signature> + Signer + Send + Sync + Debug> SignerCombined for T {}

// TODO: Rework a proper trsnsaction error manager.
#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),

    #[error("Invalid contract address: {0}")]
    InvalidAddress(String),

    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Transaction timeout after {0} seconds")]
    TransactionTimeout(u64),

    #[error("Gas estimation failed: {0}")]
    GasEstimationFailed(String),

    #[error(
        "Transaction monitoring timed out after {0} seconds, but transaction may still succeed"
    )]
    MonitoringTimeout(u64),

    #[error("Receipt not found after {0} attempts")]
    ReceiptNotFound(u32),

    #[error("Insufficient confirmations: required {required}, got {actual}")]
    InsufficientConfirmations { required: u64, actual: u64 },

    #[error("Network connectivity error: {0}")]
    NetworkError(String),

    #[error("Transport error: {0}")]
    TransportError(#[from] alloy::transports::TransportError),
    #[error("Invalid chain-id: {0}")]
    InvalidChainId(String),
}

impl From<eyre::Report> for TransactionError {
    fn from(err: eyre::Report) -> Self {
        TransactionError::RpcError(err.to_string())
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

// TODO: Add all this in constructor.
const RETRY_DELAY: u64 = 500;
// TODO: Add max retries if necessary.
// const MAX_GAS_ESTIMATION_RETRIES: i32 = 50;
// TODO: Check if necessary inside send_raw_transaction_sync_with_retries function.
// const MAX_TX_RETRIES: u32 = 100;

#[derive(Debug, Clone)]
pub struct TransactionEngine<F, P, N = AnyNetwork>
where
    N: Network,
    F: TxFiller<N>,
    P: Provider<N>,
{
    pub provider: Arc<NonceManagedProvider<F, P, N>>,
    pub signer: Arc<dyn SignerCombined>,
    pub nonce_manager: Arc<ZamaNonceManager>,
    // No need for Arc in this case, since the tx manager is shared by arc at the top level application.
    limit_concurrent_requests: bool,
    pub rpc_semaphore: Arc<Semaphore>,
}

impl
    TransactionEngine<
        CustomFillers, // F: Assumes your manager uses the standard NonceFiller.
        RootProvider,  // P: The concrete `DynProvider` wrapper, not `dyn Provider`.
        Ethereum,      // N: The network.
    >
{
    pub fn new(
        http_rpc_url: &str,
        signer: Arc<dyn SignerCombined>,
        limit_concurrent_requests: bool,
        max_concurrent_rpc_requests: usize,
    ) -> Self {
        let signer_address = <dyn SignerCombined as alloy::signers::Signer>::address(&*signer);
        let wallet = EthereumWallet::from(signer.clone());

        let rpc_url = Url::parse(http_rpc_url)
            .map_err(|e| TransactionError::InvalidAddress(format!("Invalid URL: {e}")))
            .unwrap();

        let provider = ProviderBuilder::new()
            .network::<Ethereum>() // Use the concrete network type
            .filler(GasFiller)
            .filler(ChainIdFiller::new(signer.chain_id()))
            .filler(WalletFiller::new(wallet))
            .connect_http(rpc_url.clone());

        let nonce_manager = Arc::new(ZamaNonceManager::new());
        let managed_provider =
            NonceManagedProvider::new(provider, signer_address, nonce_manager.clone());

        Self {
            provider: Arc::new(managed_provider),
            signer,
            nonce_manager,
            limit_concurrent_requests,
            rpc_semaphore: Arc::new(Semaphore::new(max_concurrent_rpc_requests)),
        }
    }

    pub fn sender_address(&self) -> Address {
        self.signer.address()
    }

    pub async fn send_raw_transaction_sync(
        &self,
        target: Address,
        calldata: Bytes,
        value: Option<U256>,
        contract_call: bool,
    ) -> Result<AnyTransactionReceipt, TransactionError> {
        if contract_call {
            let code = self.provider.inner.get_code_at(target).await.map_err(|e| {
                TransactionError::TransactionFailed(format!("Failed to check contract code: {e}"))
            })?;

            if code.is_empty() {
                error!("No code at target address: {:?} !", target);
                return Err(TransactionError::InvalidAddress(format!(
                    "No code at target address: {target:#x}"
                )));
            }
        }

        let mut request = TransactionRequest::default()
            .with_from(self.sender_address())
            .with_to(target)
            .with_input(calldata.clone())
            .with_value(value.unwrap_or_default());

        let gas_limit_estimate = match self.estimate_gas(target, calldata.clone(), value).await {
            Ok(gas) => gas,
            Err(e) => {
                // If gas estimation fails with an unrecoverable error, we must not proceed.
                warn!(
                    "Gas estimation failed, transaction will not be sent: {:?}",
                    e
                );
                return Err(TransactionError::GasEstimationFailed(
                    "Could not estimate gas".to_string(),
                ));
            }
        };
        request = request.with_gas_limit(gas_limit_estimate);

        let receipt = self.send_raw_transaction_sync_with_retries(request).await?;

        Ok(receipt)
    }

    pub async fn estimate_gas(
        &self,
        target: Address,
        calldata: Bytes,
        value: Option<U256>,
    ) -> Result<u64, TransactionError> {
        let request = TransactionRequest::default()
            .with_from(self.sender_address())
            .with_to(target)
            .with_input(calldata)
            .with_value(value.unwrap_or_default());

        let mut retries = 0;
        loop {
            debug!("Number of retries for gas estimation: {}", retries);
            // if retries >= MAX_GAS_ESTIMATION_RETRIES {
            //     return Err(TransactionError::GasEstimationFailed(format!(
            //         "Gas estimation failed after {} retries",
            //         MAX_GAS_ESTIMATION_RETRIES
            //     )));
            // }

            if self.limit_concurrent_requests {
                let _permit = match self.rpc_semaphore.acquire().await {
                    Ok(p) => p,
                    Err(_) => {
                        // This error is fatal. It means the semaphore was closed,
                        // which should not happen during normal operation.
                        warn!(
                            "RPC semaphore has been closed on estimate gas. This is a critical error: Retrying"
                        );
                        retries += 1;
                        continue;
                    }
                };
            }

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
                            if response_error_string.contains("execution reverted") {
                                error!(error = %err_msg, "Gas estimation failed due to revert: Likely unrecoverable");
                                return Err(TransactionError::GasEstimationFailed(format!(
                                    "Execution reverted: {}",
                                    message
                                )));
                            }
                            // For other RPC errors, we will treat them as potentially transient for now and retry.
                            // You can add more specific checks here for other fatal errors.
                            error!(error = %err_msg, "Gas estimation failed with RPC error");
                            // For now since we don't know what is going on we revert !
                            return Err(TransactionError::GasEstimationFailed(err_msg));
                        }
                        // This is a network/transport error. It's recoverable, and should be retried.
                        RpcError::Transport(transport_err) => {
                            warn!(error = %transport_err, "Transport error during gas estimation: Retrying");
                        }
                        _ => {
                            error!(error = %err_msg, "Unknown error during gas estimation");
                            return Err(TransactionError::GasEstimationFailed(err_msg));
                        }
                    }
                }
            }
            // If we've reached here, it means we are retrying.
            retries += 1;
            tokio::time::sleep(std::time::Duration::from_millis(RETRY_DELAY)).await;
        }
    }

    // TODO: Add gas bump
    // TODO: Match all thoses errors code, and make a triage accordingly: https://ethereum-json-rpc.com/errors + Combine with parsing error message for get a clear triage.
    async fn send_raw_transaction_sync_with_retries(
        &self,
        mut tx: TransactionRequest,
    ) -> Result<AnyTransactionReceipt, TransactionError> {
        let pending_receipt: AnyTransactionReceipt;
        let start_time = Instant::now();
        let mut retries = 0;

        loop {
            // We could add a max number of retries here.
            // if retries >= MAX_TX_RETRIES {
            // return Err(TransactionError::TransactionFailed(format!(
            // "Transaction failed after {} retries.",
            // MAX_TX_RETRIES
            // )));
            // }
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
                    error!(error = %e, "Couldn't aquire next nonce: Retrying");
                    retries += 1;
                    // Wait a bit before the next attempt to avoid hammering the nonce manager again.
                    tokio::time::sleep(std::time::Duration::from_millis(RETRY_DELAY)).await;
                    continue; // Jump to the next loop iteration.
                }
            };

            // SETTING THE NONCE BEFORE SENDING THE TX.
            tx.set_nonce(nonce);

            debug!(
                nonce = tx.nonce,
                "Launching transaction with nonce assigned"
            );

            if self.limit_concurrent_requests {
                let _permit = match self.rpc_semaphore.acquire().await {
                    Ok(p) => p,
                    Err(_) => {
                        // This error is fatal. It means the semaphore was closed,
                        // which should not happen during normal operation.
                        warn!(
                            "RPC semaphore has been closed on estimate gas. This is a critical error: Retrying"
                        );
                        retries += 1;
                        continue;
                    }
                };
            }

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
                                warn!(
                                    nonce = tx.nonce,
                                    "Nonce too low error: {:?}, Retrying", err_msg
                                );
                                self.nonce_manager
                                    .confirm_nonce(self.sender_address(), nonce)
                                    .await;
                            } else if response_error_string.contains("nonce too high") {
                                warn!(
                                    nonce = tx.nonce,
                                    "Nonce too high error: {:?}, Retrying", err_msg
                                );
                                self.nonce_manager
                                    .release_nonce(self.sender_address(), nonce)
                                    .await;
                            } else {
                                // TODO create a proper Rpc error response triage error: e.g, "transaction underpriced" and so on.
                                error!("Non-handled RPC error response: {:?}", err_msg);
                                self.nonce_manager
                                    .release_nonce(self.sender_address(), nonce)
                                    .await;
                                // For unexpected blockchain errors, we might want to fail fast.
                                return Err(TransactionError::TransactionFailed(err_msg));
                            }
                        }
                        RpcError::Transport(transport_err) => {
                            warn!(nonce = tx.nonce, error = %transport_err, "Transport error during send: Retrying");
                            self.nonce_manager
                                .release_nonce(self.sender_address(), nonce)
                                .await;
                        }
                        // TODO: Add an exhaustive triage error here
                        _ => {
                            error!(nonce = tx.nonce, "Unknown RPC error: {:?}", err_msg);
                            self.nonce_manager
                                .release_nonce(self.sender_address(), nonce)
                                .await;
                            // Fail on truly unknown errors.
                            return Err(TransactionError::RpcError(err_msg));
                        }
                    }

                    // If we haven't returned, we are retrying.
                    retries += 1;
                    tokio::time::sleep(std::time::Duration::from_millis(RETRY_DELAY)).await;
                    continue;
                }
            }
        }

        let elapsed_time = format!("{:?}ms", start_time.elapsed().as_millis());
        info!(
            elapsed_time = elapsed_time,
            nonce = tx.nonce,
            ?pending_receipt.transaction_hash,
            "Transaction has produced a receipt"
        );
        Ok(pending_receipt)
    }
}
