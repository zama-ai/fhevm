use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, Bytes, B256, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::{local::PrivateKeySigner, Signer},
    transports::http::{Client, Http},
};
use eyre::Result;
use reqwest::Url;
use std::fmt;
use std::{sync::Arc, time::Duration};
use thiserror::Error;
use tokio::time::timeout;
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

#[derive(Clone)]
pub struct TransactionManager {
    pub provider: Arc<dyn Provider<Http<Client>>>,
    wallet: EthereumWallet,
    chain_id: u64,
}

impl fmt::Debug for TransactionManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransactionManager")
            .field("chain_id", &self.chain_id)
            .field("wallet_address", &self.wallet.default_signer().address())
            .field("provider", &"<provider>") // Skip detailed provider debug
            .finish()
    }
}

impl TransactionManager {
    pub async fn new(
        rpc_url: &str,
        private_key: &str,
        chain_id: u64,
    ) -> Result<Self, TransactionError> {
        let url = Url::parse(rpc_url)
            .map_err(|e| TransactionError::InvalidAddress(format!("Invalid URL: {}", e)))?;

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_http(url);

        let mut signer: PrivateKeySigner = private_key.parse().map_err(|e| {
            TransactionError::InvalidPrivateKey(format!("Invalid private key format: {}", e))
        })?;

        signer.set_chain_id(Some(chain_id));
        let wallet = EthereumWallet::from(signer);

        info!(
            address = ?wallet.default_signer().address(),
            chain_id,
            "Initialized TransactionManager"
        );

        Ok(Self {
            provider: Arc::new(provider),
            wallet,
            chain_id,
        })
    }

    pub fn provider(&self) -> &Arc<dyn Provider<Http<Client>>> {
        &self.provider
    }

    pub fn sender_address(&self) -> Address {
        self.wallet.default_signer().address()
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

        let gas = self
            .provider
            .estimate_gas(&request)
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

        let result = self
            .provider
            .call(&request)
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

        match self.provider.call(&request).await {
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
    ) -> Result<B256> {
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
        let code = self.provider.get_code_at(target).await.map_err(|e| {
            TransactionError::TransactionFailed(format!("Failed to check contract code: {}", e))
        })?;

        if code.is_empty() {
            error!("⚠️  Warning: No code at target address!");
        }

        let request = TransactionRequest::default()
            .with_from(self.sender_address())
            .with_to(target)
            .with_input(calldata)
            .with_value(config.value.unwrap_or_default());

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

                info!(?tx_hash, "Transaction sent successfully");
                Ok(tx_hash)
            }
            Err(_) => {
                error!(
                    timeout_secs = ?timeout_duration.as_secs(),
                    "Transaction timed out"
                );
                Err(TransactionError::TransactionTimeout(timeout_duration.as_secs()).into())
            }
        }
    }

    pub async fn deploy_contract(&self, bytecode: Bytes, config: Option<TxConfig>) -> Result<B256> {
        let config = config.unwrap_or_default();

        let request = TransactionRequest::default()
            .with_from(self.sender_address())
            .with_input(bytecode)
            .with_value(config.value.unwrap_or_default());

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
        _min_confirmations: u64,
    ) -> Result<bool> {
        let receipt = self
            .provider
            .get_transaction_receipt(tx_hash)
            .await?
            .ok_or_else(|| TransactionError::TransactionFailed("Receipt not found".into()))?;

        Ok(receipt.status())
    }

    pub async fn verify_contract_code(&self, address: Address) -> Result<Bytes> {
        let code = self.provider.get_code_at(address).await?;
        println!("Deployed bytecode: 0x{}", hex::encode(&code));
        Ok(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{hex, keccak256, U256};

    #[tokio::test]
    /// For this test having a running node is MANDATORY
    /// url: http://localhost:8756
    /// chain_id: 123456
    /// The wallet associated to this private key should have fund:
    /// 7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f
    async fn test_counter_contract() {
        // Test private key (default)
        let private_key = std::env::var("TEST_PRIVATE_KEY").unwrap_or_else(|_| {
            "7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f".to_string()
        });

        println!("Setting up manager with test private key...");
        let manager = TransactionManager::new("http://localhost:8756", &private_key, 123456)
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

    #[tokio::test]
    async fn test_decryption_manager_request() -> Result<(), Box<dyn std::error::Error>> {
        // Calculate function selector for publicDecryptionRequest(uint256[])
        let selector = &keccak256("publicDecryptionRequest(uint256[])")[..4];
        println!("Function selector: 0x{}", hex::encode(selector));

        let private_key = std::env::var("TEST_PRIVATE_KEY").unwrap_or_else(|_| {
            "7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f".to_string()
        });

        let manager =
            TransactionManager::new("http://localhost:8757", &private_key, 654321).await?;
        let sender = manager.sender_address();
        println!("Using sender address: {:?}", sender);

        let contract_address = hex!("2Fb4341027eb1d2aD8B5D9708187df8633cAFA92").into();
        println!("Contract address: {:?}", contract_address);

        // Create test ciphertext handles
        let handles = vec![U256::from(1), U256::from(2)];

        // Encode the parameters properly following ABI encoding rules
        let mut calldata = Vec::new();

        // 1. Add function selector
        calldata.extend_from_slice(selector);

        // 2. Add offset to start of array (32 bytes from start of parameters)
        calldata.extend_from_slice(&U256::from(32).to_be_bytes::<32>());

        // 3. Add array length
        calldata.extend_from_slice(&U256::from(handles.len()).to_be_bytes::<32>());

        // 4. Add array elements
        for handle in handles {
            calldata.extend_from_slice(&handle.to_be_bytes::<32>());
        }

        println!("Full calldata: 0x{}", hex::encode(&calldata));

        // Try with hardcoded gas first to bypass estimation
        let config = TxConfig {
            gas_limit: Some(200000), // Fixed gas limit
            max_priority_fee: Some(2_000_000_000),
            confirmations: Some(1),
            timeout_secs: Some(30),
            ..Default::default()
        };

        println!("Sending publicDecryptionRequest transaction...");
        let tx_hash = manager
            .send_transaction(contract_address, calldata.into(), Some(config))
            .await?;

        println!("Transaction sent! Hash: 0x{}", hex::encode(tx_hash));

        // Wait for confirmation
        println!("Waiting for confirmation...");
        let success = manager.wait_for_confirmation(tx_hash, 1).await?;
        assert!(success, "Transaction failed");
        println!("Transaction confirmed successfully!");

        Ok(())
    }
}
