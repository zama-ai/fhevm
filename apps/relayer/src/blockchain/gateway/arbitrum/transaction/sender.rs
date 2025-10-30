use alloy::{
    network::{AnyNetwork, AnyTransactionReceipt, EthereumWallet, TransactionBuilder, TxSigner},
    primitives::{Address, Bytes, B256, U256},
    providers::{
        fillers::{ChainIdFiller, GasFiller, NonceFiller, WalletFiller},
        PendingTransactionBuilder, Provider, ProviderBuilder,
    },
    rpc::types::TransactionRequest,
    serde::WithOtherFields,
    signers::{Signature, Signer},
    transports::{RpcError, TransportErrorKind},
};

use eyre::Result;
use reqwest::Url;
use std::{fmt, future::IntoFuture};
use std::{sync::Arc, time::Duration};
use thiserror::Error;
use tokio::time::{timeout, Instant};
use tracing::{debug, error, info, instrument, warn};

use crate::{
    blockchain::gateway::arbitrum::transaction::{
        nonce::CachedNonceManagerWithRefresh, TransactionServiceError,
    },
    config::settings::{RetrySettings, TransactionConfig},
};

pub trait SignerCombined: TxSigner<Signature> + Signer + Send + Sync {}

// Automatically implement SignerCombined for any type that satisfies all the required traits
impl<T: TxSigner<Signature> + Signer + Send + Sync> SignerCombined for T {}

// TODO: it's not used anywhere
// // should either be used or removed
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
        }
    }
}

impl From<RetrySettings> for RetryConfig {
    fn from(settings: RetrySettings) -> Self {
        info!(
            max_attempts = settings.max_attempts,
            base_delay_secs = settings.base_delay_secs,
            max_delay_secs = settings.max_delay_secs,
            "Retry configuration"
        );
        RetryConfig {
            max_attempts: settings.max_attempts,
            base_delay: Duration::from_secs(settings.base_delay_secs),
            max_delay: Duration::from_secs(settings.max_delay_secs),
        }
    }
}

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

impl From<TransactionConfig> for TxConfig {
    fn from(config: TransactionConfig) -> Self {
        Self {
            gas_limit: config.gas_limit,
            max_priority_fee: config.get_max_priority_fee().ok().flatten(),
            value: None, // No value transfer by default
            nonce: None, // Let the manager handle nonce
            confirmations: config.confirmations,
            timeout_secs: config.timeout_secs,
            retry_config: Some(RetryConfig::from(config.retry)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TxConfig {
    pub gas_limit: Option<u64>,
    pub max_priority_fee: Option<u128>,
    pub value: Option<U256>,
    pub nonce: Option<u64>,
    pub confirmations: Option<u64>,
    pub timeout_secs: Option<u64>,
    pub retry_config: Option<RetryConfig>,
}

impl Default for TxConfig {
    fn default() -> Self {
        Self {
            gas_limit: Some(500000),
            max_priority_fee: Some(3_000_000_000), // 3 gwei
            value: Some(U256::ZERO),
            nonce: None,
            confirmations: Some(1),
            timeout_secs: Some(60),
            retry_config: Some(RetryConfig::default()),
        }
    }
}

pub struct TransactionManager {
    pub provider: Box<dyn Provider<AnyNetwork> + Send + Sync>,
    pub signer: Arc<dyn SignerCombined>,
    pub nonce_manager: Arc<CachedNonceManagerWithRefresh>,
}

impl fmt::Debug for TransactionManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransactionManager")
            .field("wallet_address", &self.signer.address())
            .field("provider", &"<provider>") // Skip detailed provider debug
            .finish()
    }
}

impl TransactionManager {
    pub async fn new(
        http_rpc_url: &str,
        // private_key: &str,
        signer: Arc<dyn SignerCombined>,
    ) -> Result<Self, TransactionError> {
        let wallet = EthereumWallet::from(signer.clone());

        let rpc_url = Url::parse(http_rpc_url)
            .map_err(|e| TransactionError::InvalidAddress(format!("Invalid URL: {e}")))?;

        // NOTE: nonce-manager that allows for nonce-resync
        let nonce_manager = CachedNonceManagerWithRefresh::default();
        // TODO: Add fallback layers.
        // Add retry layer.

        let concrete_provider = ProviderBuilder::new()
            .network::<AnyNetwork>()
            .filler(NonceFiller::new(nonce_manager.clone()))
            .filler(GasFiller)
            .filler(ChainIdFiller::new(signer.chain_id()))
            .filler(WalletFiller::new(wallet))
            // .layer(Layer::new(retry_policy))
            .connect_http(rpc_url.clone());

        let provider_http = Box::new(concrete_provider);

        info!(
            address = ?signer.address(),
            chain_id = ?signer.chain_id(),
            "Initialized TransactionManager"
        );

        Ok(Self {
            provider: provider_http,
            nonce_manager: Arc::new(nonce_manager),
            signer,
        })
    }

    pub fn provider(&self) -> &(dyn Provider<AnyNetwork> + Send + Sync) {
        self.provider.as_ref()
    }

    pub fn sender_address(&self) -> Address {
        self.signer.address()
    }

    async fn call_provider<F, Fut, T>(
        &self,
        operation: F,
    ) -> Result<T, RpcError<TransportErrorKind>>
    where
        F: for<'a> FnOnce(&'a dyn Provider<AnyNetwork>) -> Fut,
        Fut: std::future::Future<Output = Result<T, RpcError<TransportErrorKind>>>,
    {
        let provider = &self.provider;
        let result = operation(provider.as_ref()).await;

        match result {
            Ok(value) => Ok(value),
            Err(err) => {
                // 🔍 Handle retryable errors gracefully
                match &err {
                    RpcError::Transport(kind) => match kind {
                        TransportErrorKind::BackendGone => {
                            warn!(?kind, "Transient network or backend issue detected");
                            // The RetryLayer already retries automatically,
                            // so here we just log — no manual reset required.
                        }
                        _ => {
                            error!(?kind, "Non-retryable transport error");
                        }
                    },
                    _ => {
                        error!(?err, "RPC-level error");
                    }
                }
                Err(err)
            }
        }
    }

    // TODO: actually use this method
    pub async fn estimate_gas(
        &self,
        target: Address,
        calldata: Bytes,
        config: Option<TxConfig>,
    ) -> Result<u64> {
        let config = config.unwrap_or_default();

        let request = TransactionRequest::default()
            .with_from(self.sender_address())
            .with_to(target)
            .with_input(calldata)
            .with_value(config.value.unwrap_or_default());
        let request = WithOtherFields::new(request);

        let gas = self
            .call_provider(|p| p.estimate_gas(request).into_future())
            .await
            .map_err(|error| TransactionServiceError::GasEstimation(error.to_string()))?;

        // NOTE: shouldn't this be exposed as tx-manager configuration?
        // Add 20% buffer to estimated gas
        let gas_with_buffer = (gas as f64 * 1.2) as u64;

        Ok(gas_with_buffer)
    }

    pub async fn call_view(&self, target: Address, calldata: Bytes) -> Result<Bytes> {
        let request = TransactionRequest::default()
            .with_from(self.sender_address())
            .with_to(target)
            .with_input(calldata);
        let request = WithOtherFields::new(request);

        let result = self
            .call_provider(|p| p.call(request).into_future())
            .await
            .map_err(
                |e: alloy::transports::RpcError<alloy::transports::TransportErrorKind>| {
                    TransactionError::RpcError(e.to_string())
                },
            )?;

        Ok(result)
    }

    /// TODO: hide under a feature flag or remove
    /// Debug a transaction using Anvil's tracing features
    pub async fn debug_transaction_call(
        &self,
        target: Address,
        calldata: &Bytes,
        config: &TxConfig,
    ) -> Result<(), TransactionError> {
        println!("\n🔍 Enhanced Debug Information:");

        // Decode function selector
        if calldata.len() >= 4 {
            let selector = &calldata[..4];
            println!("Function selector: 0x{}", hex::encode(selector));

            // Print parameter data in chunks
            if calldata.len() > 4 {
                println!("\nParameters (in 32-byte chunks):");
                for (i, chunk) in calldata[4..].chunks(32).enumerate() {
                    println!("Param {}: 0x{}", i, hex::encode(chunk));

                    // Try to interpret the parameter
                    if chunk.len() == 32 {
                        // Try as uint256
                        let as_uint = U256::from_be_bytes::<32>(chunk.try_into().unwrap());
                        println!("  As uint: {as_uint}");

                        // Try as address if starts with zeros
                        if chunk[..12].iter().all(|&x| x == 0) {
                            let addr = Address::from_slice(&chunk[12..]);
                            println!("  As address: {addr:#x}");
                        }
                    }
                }
            }
        }

        // Try the call
        let request = TransactionRequest::default()
            .with_from(self.sender_address())
            .with_to(target)
            .with_input(calldata.clone())
            .with_value(config.value.unwrap_or_default());
        let request = WithOtherFields::new(request);

        // NOTE: this one executes the transaction without publishing it
        // https://www.alchemy.com/docs/node/ethereum/ethereum-api-endpoints/eth-call
        match self.call_provider(|p| p.call(request).into_future()).await {
            Ok(_) => {
                debug!("\n✅ Call simulation succeeded");
                Ok(())
            }
            Err(e) => {
                debug!("\n❌ Call simulation failed:");
                debug!("Target: {:#x}", target);
                debug!("From: {:#x}", self.sender_address());
                debug!("Value: {:#x}", config.value.unwrap_or_default());

                // More detailed error analysis
                debug!("\nError Analysis:");
                debug!("Type: Contract Revert");
                debug!("Code: 3 (Standard EVM revert)");

                debug!("\nFull error: {:#?}", e);

                Err(TransactionError::TransactionFailed(e.to_string()))
            }
        }
    }

    pub async fn cancel_transaction(&self, nonce: u64) -> Result<B256, TransactionError> {
        // Get current account info
        let address = self.sender_address();

        // Calculate new gas price (e.g., 15% higher)
        // TODO: double check gas estimation here and what's provided in request

        let base_fee = self.call_provider(|p| p.get_gas_price()).await?;
        let max_priority_fee = self
            .call_provider(|p| p.get_max_priority_fee_per_gas())
            .await?;
        // Build cancellation transaction (send 0 ETH to self)
        let request = TransactionRequest::default()
            .with_from(address)
            .with_to(address)
            .with_value(U256::ZERO)
            .with_nonce(nonce)
            .with_max_fee_per_gas(base_fee)
            .with_max_priority_fee_per_gas(max_priority_fee);

        let request = WithOtherFields::new(request);
        let tx = self.send_transaction_with_retry(request).await?;
        let tx_hash = tx.tx_hash();
        info!(?tx_hash, "Transaction submitted successfully");

        Ok(*tx_hash)
    }

    /// Send transaction with retry
    ///
    /// This function will send a transaction and detect any retriable compatible error
    #[instrument(skip_all)]
    pub async fn send_transaction_with_retry(
        &self,
        request: WithOtherFields<TransactionRequest>,
    ) -> Result<PendingTransactionBuilder<AnyNetwork>, TransactionError> {
        let pending_tx: PendingTransactionBuilder<AnyNetwork>;
        // TODO: define different failure modes scopes
        // i.e. if the transaction is reverted is not the responsability of the TransactionManager
        // but if the nonce is out-of-sync, it's the TransactionManager's responsability to re-emit
        // the transaction.
        // Same for transactions that get stuck
        // NOTE: imo anything that is not at the application level should be handled here
        // - Nonce issue
        //  - Nonce too low (instant failure)
        //  - Nonce too high (tx is never included)
        // - Gas issue
        //  - Tx is stuck because gas-fee is too low
        // - Connectivity issue
        //  - Can't reach the RPC endpoint
        //  - Rate limite in the RPC endpoint
        // - Funding error
        //  - Modes:
        //   - Using a wallet that isn't funded yet
        //   - Using a wallet that hasn't enough funds
        //  - Mitigation strategies
        //   - Retry with another wallet (if we have a pool of signers)
        //   - Retry with the same wallet (hoping for funding to arrive in the meantime)
        // Some errors are by nature unrecoverable (c.f. [`RpcError`] enum) and should return a
        // TransactionError
        // I don't think the TxConfig should exist as everything that is configured in there should
        // be the responsability of the [`TransactionManager`]
        // TODO: checkout [`ErrorPayload`] too (used in [`RpcError`])
        // TODO: check if it makes sense to keep both [`TransactionManager`] and
        // [`TransactionService`]
        loop {
            match self.provider.send_transaction(request.clone()).await {
                Ok(value) => {
                    pending_tx = value;
                    break;
                }
                Err(e) => {
                    // Any instant recovery mechanism should be implement here
                    // TODO: we should probably consider the retry mechanism from the TxConfig
                    // to avoid infinite retries here
                    let err_msg = e.to_string();

                    // TODO: properly match different response errors and adapt retry mechanism
                    match e {
                        RpcError::ErrorResp(response_error) => {
                            // TODO: Nonce reset is a temporary fix for "nonce too high" scenario.
                            // Proper fix should be identified.
                            let response_error_string = response_error.to_string();
                            if response_error_string.contains("nonce too low")
                                | response_error_string.contains("nonce too high")
                            {
                                let provider_guard = &self.provider;
                                let _ = self
                                    .nonce_manager
                                    .sync_nonce(provider_guard.as_ref(), self.signer.address())
                                    .await;
                            } else {
                                return Err(TransactionError::TransactionFailed(err_msg));
                            }
                        }
                        _ => {
                            return Err(TransactionError::TransactionFailed(err_msg));
                        }
                    }
                }
            };
        }
        Ok(pending_tx)
    }

    #[instrument(skip_all, fields(target=%target, config=?config))]
    pub async fn send_transaction(
        &self,
        target: Address,
        calldata: Bytes,
        config: Option<TxConfig>,
    ) -> Result<B256, TransactionError> {
        let config = config.unwrap_or_default();

        // Check if contract exists
        info!("Checking contract code at {:#x}", target);
        let code = self.provider.get_code_at(target).await.map_err(|e| {
            TransactionError::TransactionFailed(format!("Failed to check contract code: {e}"))
        })?;

        if code.is_empty() {
            error!("No code at target address: {:?} !", target);
            return Err(TransactionError::InvalidAddress(format!(
                "No code at target address: {target:#x}"
            )));
        }

        info!("Preparing request: {:#x}", target);
        let mut request = TransactionRequest::default()
            .with_from(self.sender_address())
            .with_to(target)
            .with_input(calldata.clone())
            .with_value(config.value.unwrap_or_default());

        if let Ok(gas_limit_estimate) = self.estimate_gas(target, calldata, Some(config)).await {
            request = request.with_gas_limit(gas_limit_estimate);
        } else {
            warn!("Gas estimation failed");
        }

        let request = WithOtherFields::new(request);
        let tx = self.send_transaction_with_retry(request).await?;
        let tx_hash = tx.tx_hash();
        info!(?tx_hash, "Transaction submitted successfully");

        Ok(*tx_hash)
    }

    pub async fn deploy_contract(&self, bytecode: Bytes, config: Option<TxConfig>) -> Result<B256> {
        let config = config.unwrap_or_default();

        let request = TransactionRequest::default()
            .with_from(self.sender_address())
            .with_deploy_code(bytecode)
            .with_value(config.value.unwrap_or_default());
        let request = WithOtherFields::new(request);

        let timeout_duration = Duration::from_secs(config.timeout_secs.unwrap_or(60));

        // Send and watch for the transaction
        let result = timeout(
            timeout_duration,
            self.provider.send_transaction(request).await?.watch(),
        )
        .await;

        match result {
            Ok(tx_hash) => {
                let tx_hash =
                    tx_hash.map_err(|e| TransactionError::TransactionFailed(e.to_string()))?;

                info!(
                    ?tx_hash,
                    "Contract deployment transaction sent successfully"
                );
                Ok(tx_hash)
            }
            Err(_) => {
                error!(
                    timeout_secs = ?timeout_duration.as_secs(),
                    "Contract deployment timed out"
                );
                Err(TransactionError::TransactionTimeout(timeout_duration.as_secs()).into())
            }
        }
    }

    pub fn encode_function_call(selector: [u8; 4], params: Vec<Vec<u8>>) -> Bytes {
        let mut calldata = Vec::with_capacity(4 + params.len() * 32);
        calldata.extend_from_slice(&selector);
        for param in params {
            let mut padded = vec![0u8; 32];
            let start = 32 - std::cmp::min(32, param.len());
            padded[start..].copy_from_slice(&param[..std::cmp::min(32, param.len())]);
            calldata.extend_from_slice(&padded);
        }
        Bytes::from(calldata)
    }

    pub async fn wait_for_confirmation_receipt_with_timeout(
        &self,
        tx_hash: B256,
        min_confirmations: u64,
        timeout: Duration,
    ) -> Result<AnyTransactionReceipt, eyre::Error> {
        let config = TxConfig {
            confirmations: Some(min_confirmations),
            ..Default::default()
        };

        match self
            .wait_for_receipt_with_timeout(tx_hash, &config, timeout)
            .await
        {
            Ok(receipt) => Ok(receipt),
            Err(e) => Err(eyre::eyre!("Failed to get confirmation: {}", e)),
        }
    }

    /// Wait for a transaction receipt with configurable polling and timeout
    ///
    /// This function uses an exponential backoff strategy with jitter to
    /// efficiently poll for transaction receipts while minimizing network load.
    ///
    /// # Arguments
    /// * `tx_hash` - The transaction hash to wait for
    /// * `config` - Transaction configuration including timeout and confirmations
    ///
    /// # Returns
    /// * `Ok(TransactionReceipt)` - The transaction receipt once confirmed
    /// * `Err(TransactionError)` - Various errors based on polling results
    pub async fn wait_for_receipt_with_timeout(
        &self,
        tx_hash: B256,
        config: &TxConfig,
        timeout: Duration,
    ) -> Result<AnyTransactionReceipt, TransactionError> {
        info!("TX MANAGER: wait_for_receipt_with_timeout");
        let start_time = Instant::now();
        let required_confirmation = config.confirmations.unwrap_or(1);

        // Arbitrum mining block time could be very low, 250 to 100 ms if the blocks are full.
        let poll_interval = Duration::from_millis(50);

        loop {
            if start_time.elapsed() > timeout {
                error!(?tx_hash, "Timed out waiting for transaction receipt.");
                return Err(TransactionError::TransactionTimeout(timeout.as_secs()));
            }
            match self.provider.get_transaction_receipt(tx_hash).await {
                Ok(Some(receipt)) => {
                    if required_confirmation == 0 {
                        info!(?tx_hash, "Receipt found with 0 confirmations required.");
                        return Ok(receipt);
                    }

                    if let Some(receipt_block) = receipt.block_number {
                        match self.provider.get_block_number().await {
                            Ok(current_block) => {
                                let confirmations = current_block.saturating_sub(receipt_block) + 1;
                                if confirmations > required_confirmation {
                                    info!(?tx_hash, ?confirmations, "Getting receipt and transaction confirmed after {:?} confirmations", confirmations);
                                    return Ok(receipt);
                                } else {
                                    debug!(
                                        ?tx_hash,
                                        ?confirmations,
                                        ?receipt_block,
                                        ?current_block,
                                        "Receipt found, waiting for more confirmations."
                                    );
                                }
                            }
                            Err(e) => {
                                warn!(?tx_hash, error = %e, "Could not get current block number to check confirmations: will retry");
                            }
                        }
                    } else {
                        debug!(
                            ?tx_hash,
                            "Receipt found, but still has no block number yet: Waiting"
                        );
                    }
                }
                Ok(None) => {
                    // Transaction not mined yet.
                    debug!(?tx_hash, "Receipt not available yet.");
                }
                Err(e) => {
                    // TODO: Check here with retry layer implementation, if renders errors and when.
                    // Error occured during polling.
                    warn!(?tx_hash, error=%e, "Error when polling for receipt, retrying.");
                }
            }

            // Wait before the next polling loop.
            tokio::time::sleep(poll_interval).await;
        }
    }

    pub async fn wait_for_receipt(
        &self,
        tx_hash: B256,
        config: &TxConfig,
    ) -> Result<AnyTransactionReceipt, TransactionError> {
        let required_confirmation = config.confirmations.unwrap_or(1);

        // Arbitrum mining block time could be very low, 250 to 100 ms if the blocks are full.
        let poll_interval = Duration::from_millis(50);

        loop {
            match self.provider.get_transaction_receipt(tx_hash).await {
                Ok(Some(receipt)) => {
                    if required_confirmation == 0 {
                        info!(?tx_hash, "Receipt found with 0 confirmations required.");
                        return Ok(receipt);
                    }

                    if let Some(receipt_block) = receipt.block_number {
                        match self.provider.get_block_number().await {
                            Ok(current_block) => {
                                let confirmations = current_block.saturating_sub(receipt_block) + 1;
                                if confirmations > required_confirmation {
                                    info!(?tx_hash, ?confirmations, "Getting receipt and transaction confirmed after {:?} confirmations", confirmations);
                                    return Ok(receipt);
                                } else {
                                    debug!(
                                        ?tx_hash,
                                        ?confirmations,
                                        ?receipt_block,
                                        ?current_block,
                                        "Receipt found, waiting for more confirmations."
                                    );
                                }
                            }
                            Err(e) => {
                                warn!(?tx_hash, error = %e, "Could not get current block number to check confirmations: will retry");
                            }
                        }
                    } else {
                        debug!(
                            ?tx_hash,
                            "Receipt found, but still has no block number yet: Waiting"
                        );
                    }
                }
                Ok(None) => {
                    // Transaction not mined yet.
                    debug!(?tx_hash, "Receipt not available yet.");
                }
                Err(e) => {
                    // TODO: Check here with retry layer implementation, if renders errors and when.
                    // Error occured during polling.
                    warn!(?tx_hash, error=%e, "Error when polling for receipt, retrying.");
                }
            }

            // Wait before the next polling loop.
            tokio::time::sleep(poll_interval).await;
        }
    }

    /// Send a transaction and wait for its receipt
    /// This combines transaction sending and receipt waiting into one method
    pub async fn send_transaction_and_wait(
        &self,
        target: Address,
        calldata: Bytes,
        config: Option<TxConfig>,
    ) -> Result<AnyTransactionReceipt, TransactionError> {
        let config = config.unwrap_or_default();
        let tx_hash = self
            .send_transaction(target, calldata, Some(config.clone()))
            .await?;

        debug!(?tx_hash, "Transaction sent, waiting for receipt");
        self.wait_for_receipt(tx_hash, &config).await
    }

    pub async fn verify_contract_code(&self, address: Address) -> Result<Bytes> {
        let code = self
            .call_provider(|p| p.get_code_at(address).into_future())
            .await?;
        debug!("Deployed bytecode: 0x{}", hex::encode(&code));
        Ok(code)
    }
}

// TODO: add test with un-funded wallet
