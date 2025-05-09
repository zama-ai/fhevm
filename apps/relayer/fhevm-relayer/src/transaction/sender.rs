use alloy::{
    network::{AnyNetwork, AnyTransactionReceipt, ReceiptResponse, TransactionBuilder},
    primitives::{Address, Bytes, B256, U256},
    providers::{
        fillers::{CachedNonceManager, GasFiller, NonceFiller},
        Provider, ProviderBuilder,
    },
    rpc::types::TransactionRequest,
    serde::WithOtherFields,
    signers::Signer,
};

use eyre::Result;
use reqwest::Url;
use std::fmt;
use std::{sync::Arc, time::Duration};
use thiserror::Error;
use tokio::time::{timeout, Instant};
use tracing::{debug, error, info, warn};

use crate::{
    config::settings::{RetrySettings, TransactionConfig},
    core::errors::TransactionServiceError,
};

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub mock_mode: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
            mock_mode: false,
        }
    }
}

impl From<RetrySettings> for RetryConfig {
    fn from(settings: RetrySettings) -> Self {
        info!(
            max_attempts = settings.max_attempts,
            base_delay_secs = settings.base_delay_secs,
            max_delay_secs = settings.max_delay_secs,
            mock_mode = settings.mock_mode,
            "Retry configuration"
        );
        RetryConfig {
            max_attempts: settings.max_attempts,
            base_delay: Duration::from_secs(settings.base_delay_secs),
            max_delay: Duration::from_secs(settings.max_delay_secs),
            mock_mode: settings.mock_mode,
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
    pub provider: Arc<dyn Provider<AnyNetwork> + Send + Sync>,
    signer: Arc<dyn Signer + Sync + Send>,
}

impl fmt::Debug for TransactionManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransactionManager")
            .field("chain_id", &self.signer.chain_id())
            .field("wallet_address", &self.signer.address())
            .field("provider", &"<provider>") // Skip detailed provider debug
            .finish()
    }
}

// Diagnostics:
// 1. expected a type, found a trait [E0782]
// 2. use a new generic type parameter, constrained by `Signer`: `T`, `<T: Signer>` [E0782]
// 3. you can also use an opaque type, but users won't be able to specify the type parameter when calling the `fn`, having to rely exclusively on type inference: `impl ` [E0782]
// 4. alternatively, use a trait object to accept any type that implements `Signer`, accessing its methods at runtime using dynamic dispatch: `&dyn ` [E0782]
impl TransactionManager {
    pub async fn new(
        rpc_url: &str,
        // private_key: &str,
        signer: Arc<dyn Signer + Sync + Send>,
    ) -> Result<Self, TransactionError> {
        let url = Url::parse(rpc_url)
            .map_err(|e| TransactionError::InvalidAddress(format!("Invalid URL: {}", e)))?;

        let provider = ProviderBuilder::new()
            .network::<AnyNetwork>()
            .filler(NonceFiller::new(CachedNonceManager::default()))
            .filler(GasFiller)
            .on_http(url);

        let provider = Arc::new(provider);
        // let wallet = EthereumWallet::from(signer);

        info!(
            address = ?signer.address(),
            chain_id = ?signer.chain_id(),
            "Initialized TransactionManager"
        );

        Ok(Self { provider, signer })
    }

    pub fn provider(&self) -> &Arc<dyn Provider<AnyNetwork> + Send + Sync> {
        &self.provider
    }

    pub fn sender_address(&self) -> Address {
        self.signer.address()
    }

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
            .provider
            .estimate_gas(request)
            .await
            .map_err(|e| TransactionServiceError::GasEstimation(e.to_string()))?;

        // Add 10% buffer to estimated gas
        let gas_with_buffer = (gas as f64 * 1.1) as u64;

        Ok(gas_with_buffer)
    }

    pub async fn call_view(&self, target: Address, calldata: Bytes) -> Result<Bytes> {
        let request = TransactionRequest::default()
            .with_from(self.sender_address())
            .with_to(target)
            .with_input(calldata);
        let request = WithOtherFields::new(request);

        let result = self
            .provider
            .call(request)
            .await
            .map_err(|e| TransactionError::RpcError(e.to_string()))?;

        Ok(result)
    }

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
                        println!("  As uint: {}", as_uint);

                        // Try as address if starts with zeros
                        if chunk[..12].iter().all(|&x| x == 0) {
                            let addr = Address::from_slice(&chunk[12..]);
                            println!("  As address: {:#x}", addr);
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

        match self.provider.call(request).await {
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

    pub async fn send_transaction(
        &self,
        target: Address,
        calldata: Bytes,
        config: Option<TxConfig>,
    ) -> Result<B256, TransactionError> {
        let config = config.unwrap_or_default();

        // Only run debug if log level is Debug or lower
        if tracing::enabled!(tracing::Level::DEBUG) {
            debug!("Starting detailed transaction debug...");
            if let Err(e) = self
                .debug_transaction_call(target, &calldata, &config)
                .await
            {
                warn!("Debug simulation failed: {}", e);
            }
        }

        // Check if contract exists
        info!("Checking contract code at {:#x}", target);
        let code = self.provider.get_code_at(target).await.map_err(|e| {
            TransactionError::TransactionFailed(format!("Failed to check contract code: {}", e))
        })?;

        if code.is_empty() {
            error!("No code at target address: {:?} !", target);
            return Err(TransactionError::InvalidAddress(format!(
                "No code at target address: {:#x}",
                target
            )));
        }

        info!("Preparing request: {:#x}", target);
        let request = TransactionRequest::default()
            .with_from(self.sender_address())
            .with_to(target)
            .with_input(calldata)
            .with_value(config.value.unwrap_or_default());
        let request = WithOtherFields::new(request);

        let pending_tx = self
            .provider
            .send_transaction(request)
            .await
            .map_err(|e| TransactionError::TransactionFailed(e.to_string()))?;

        // Get hash immediately
        let tx_hash = pending_tx.tx_hash();
        info!(?tx_hash, "Transaction submitted successfully");

        Ok(*tx_hash)
    }

    pub async fn deploy_contract(&self, bytecode: Bytes, config: Option<TxConfig>) -> Result<B256> {
        let config = config.unwrap_or_default();

        let request = TransactionRequest::default()
            .with_from(self.sender_address())
            .with_input(bytecode)
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

    pub async fn wait_for_confirmation(
        &self,
        tx_hash: B256,
        min_confirmations: u64,
    ) -> Result<bool, eyre::Error> {
        let config = TxConfig {
            confirmations: Some(min_confirmations),
            ..Default::default()
        };

        match self.wait_for_receipt(tx_hash, &config).await {
            Ok(receipt) => Ok(receipt.status()),
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
    pub async fn wait_for_receipt(
        &self,
        tx_hash: B256,
        config: &TxConfig,
    ) -> Result<AnyTransactionReceipt, TransactionError> {
        let timeout = Duration::from_secs(config.timeout_secs.unwrap_or(60));
        let start = Instant::now();
        let mut interval = tokio::time::interval(Duration::from_millis(500));

        loop {
            // Check if we've exceeded timeout
            if start.elapsed() > timeout {
                return Err(TransactionError::TransactionTimeout(timeout.as_secs()));
            }

            // Try to get receipt
            match self.provider.get_transaction_receipt(tx_hash).await {
                Ok(Some(receipt)) => {
                    // If confirmation checks required
                    if let Some(required_confirmations) = config.confirmations {
                        if required_confirmations <= 1 {
                            return Ok(receipt);
                        }

                        // Check block confirmations
                        if let Ok(current_block) = self.provider.get_block_number().await {
                            if let Some(receipt_block) = receipt.block_number {
                                let confirmations = current_block.saturating_sub(receipt_block) + 1;

                                if confirmations >= required_confirmations {
                                    return Ok(receipt);
                                }

                                info!(
                                    ?tx_hash,
                                    ?receipt_block,
                                    ?current_block,
                                    ?confirmations,
                                    required = ?required_confirmations,
                                    "Waiting for more confirmations"
                                );
                            }
                        }
                    } else {
                        // No confirmations required
                        return Ok(receipt);
                    }
                }
                Ok(None) => {
                    // No receipt yet
                    debug!(
                        ?tx_hash,
                        elapsed = ?start.elapsed().as_secs(),
                        "Receipt not available yet, waiting before retry"
                    );
                }
                Err(e) => {
                    // Error retrieving receipt
                    warn!(
                        ?tx_hash,
                        error = %e,
                        "Error retrieving receipt, will retry"
                    );
                }
            }

            // Wait with interval before next attempt
            interval.tick().await;
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
        let code = self.provider.get_code_at(address).await?;
        debug!("Deployed bytecode: 0x{}", hex::encode(&code));
        Ok(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{hex, keccak256, U256};
    use alloy::signers::local::PrivateKeySigner;

    #[tokio::test]
    /// For this test having a running node is MANDATORY
    /// url: http://localhost:8756
    /// chain_id: 123456
    /// The wallet associated to this private key should have fund:
    /// 7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f
    async fn test_counter_contract() {
        // Test private key (default)
        let private_key = std::env::var("TEST_PRIVATE_KEY").unwrap_or_else(|_| {
            "34aacca926bab195601bcf5702786d35cab968159b718ae671b226de11b9afee".to_string()
        });

        println!("Setting up manager with test private key...");
        let mut signer: PrivateKeySigner = private_key.parse().unwrap();
        signer.set_chain_id(Some(123456));

        let manager = TransactionManager::new("http://localhost:8756", Arc::new(signer))
            .await
            .expect("Failed to create transaction manager");

        println!("Using address: {:?}", manager.sender_address());

        // Calculate function selectors
        let increment_selector: [u8; 4] = keccak256("increment()")[..4].try_into().unwrap();
        let get_count_selector: [u8; 4] = keccak256("getCount()")[..4].try_into().unwrap();

        println!("Increment selector: 0x{}", hex::encode(increment_selector));
        println!("GetCount selector: 0x{}", hex::encode(get_count_selector));

        // Deployment bytecode
        let bytecode_str = "6080604052348015600e575f80fd5b505f80819055506102fc806100225f395ff3fe608060405234801561000f575f80fd5b506004361061003f575f3560e01c80632baeceb714610043578063a87d942c1461004d578063d09de08a1461006b575b5f80fd5b61004b610075565b005b61005561010a565b604051610062919061017c565b60405180910390f35b610073610112565b005b5f8054116100b8576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016100af90610215565b60405180910390fd5b60015f808282546100c99190610260565b925050819055507f0ef4482aceb854636f33f9cd319f9e1cd6fe3aa2e60523f3583c287b893824455f54604051610100919061017c565b60405180910390a1565b5f8054905090565b60015f808282546101239190610293565b925050819055507f0ef4482aceb854636f33f9cd319f9e1cd6fe3aa2e60523f3583c287b893824455f5460405161015a919061017c565b60405180910390a1565b5f819050919050565b61017681610164565b82525050565b5f60208201905061018f5f83018461016d565b92915050565b5f82825260208201905092915050565b7f436f756e7465723a20636f756e742063616e6e6f74206265206e6567617469765f8201527f6500000000000000000000000000000000000000000000000000000000000000602082015250565b5f6101ff602183610195565b915061020a826101a5565b604082019050919050565b5f6020820190508181035f83015261022c816101f3565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61026a82610164565b915061027583610164565b925082820390508181111561028d5761028c610233565b5b92915050565b5f61029d82610164565b91506102a883610164565b92508282019050808211156102c0576102bf610233565b5b9291505056fea26469706673582212208b12750c7f9f58edb6802284393e24cd61899134d8bc4ae2979ca244e989dbc464736f6c634300081a0033";
        let contract_bytecode =
            hex::decode(bytecode_str.trim_start_matches("0x")).expect("Failed to decode bytecode");

        println!("Bytecode length: {}", contract_bytecode.len());

        // Deploy contract
        let estimated_gas = manager
            .estimate_gas(
                Address::default(),
                Bytes::from(contract_bytecode.clone()),
                None,
            )
            .await
            .expect("Failed to estimate gas");

        println!("Estimated deployment gas: {}", estimated_gas);

        let config = TxConfig {
            gas_limit: Some((estimated_gas as f64 * 1.2) as u64), // 20% buffer
            max_priority_fee: Some(2_000_000_000),
            timeout_secs: Some(60),
            ..Default::default()
        };

        // Deploy contract
        let tx_hash = manager
            .deploy_contract(Bytes::from(contract_bytecode), Some(config))
            .await
            .expect("Failed to deploy contract");

        println!("Deployment transaction hash: 0x{}", hex::encode(tx_hash));

        let receipt = manager
            .provider
            .get_transaction_receipt(tx_hash)
            .await
            .expect("Failed to get receipt")
            .expect("Receipt not found");

        println!("Deployment receipt status: {:?}", receipt.status());
        println!("Gas used: {}", receipt.gas_used);

        let contract_address = receipt
            .contract_address
            .expect("Contract address not found in receipt");

        println!("Contract deployed at: {:?}", contract_address);

        // Verify contract code
        tokio::time::sleep(Duration::from_secs(2)).await; // Give node time to process
        let code = manager
            .provider
            .get_code_at(contract_address)
            .await
            .expect("Failed to get code");

        println!("Deployed bytecode length: {}", code.len());
        assert!(!code.is_empty(), "Contract code should not be empty");

        // Get initial count
        let get_count_calldata =
            TransactionManager::encode_function_call(get_count_selector, vec![]);
        println!("Calling view function to get initial count...");
        let count_bytes = manager
            .call_view(contract_address, get_count_calldata.clone())
            .await
            .expect("Failed to get count");

        let initial_count = U256::from_be_slice(count_bytes.as_ref());
        println!("Initial count: {}", initial_count);

        // Increment count
        let increment_calldata =
            TransactionManager::encode_function_call(increment_selector, vec![]);

        println!("Estimating gas for increment...");
        let estimated_gas = manager
            .estimate_gas(contract_address, increment_calldata.clone(), None)
            .await
            .expect("Failed to estimate gas");

        println!("Estimated gas for increment: {}", estimated_gas);

        let config = TxConfig {
            gas_limit: Some(estimated_gas),
            max_priority_fee: Some(2_000_000_000),
            confirmations: Some(1),
            timeout_secs: Some(30),
            ..Default::default()
        };

        println!("Sending increment transaction...");
        let tx_hash = manager
            .send_transaction(contract_address, increment_calldata, Some(config))
            .await
            .expect("Failed to send increment transaction");

        println!("Increment transaction hash: 0x{}", hex::encode(tx_hash));

        tokio::time::sleep(Duration::from_secs(2)).await; // Give node time to process

        // Wait for confirmation and check receipt
        let receipt = manager
            .provider
            .get_transaction_receipt(tx_hash)
            .await
            .expect("Failed to get receipt")
            .expect("Receipt not found");

        println!("Increment transaction status: {:?}", receipt.status());
        assert!(receipt.status(), "Increment transaction failed");

        // Get updated count
        println!("Getting updated count...");
        let new_count_bytes = manager
            .call_view(contract_address, get_count_calldata)
            .await
            .expect("Failed to get new count");

        let new_count = U256::from_be_slice(new_count_bytes.as_ref());
        println!("New count: {}", new_count);

        assert_eq!(
            new_count,
            initial_count + U256::from(1),
            "Count should have increased by 1"
        );
    }

    #[test]
    fn test_encode_function_call() {
        // Test encoding a simple function call
        let selector = [0xab, 0xcd, 0xef, 0x12];
        let params = vec![
            vec![1u8; 8], // will be padded to 32 bytes
        ];

        let encoded = TransactionManager::encode_function_call(selector, params);

        // Check selector
        assert_eq!(&encoded[..4], selector);

        // Check padding of first parameter
        assert_eq!(encoded.len(), 36); // 4 bytes selector + 32 bytes param
        assert_eq!(&encoded[4..28], &[0u8; 24]); // First 24 bytes should be zero
        assert_eq!(&encoded[28..36], &[1u8; 8]); // Last 8 bytes should be our input
    }
}
