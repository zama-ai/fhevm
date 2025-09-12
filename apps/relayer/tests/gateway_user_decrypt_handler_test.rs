mod utils;

#[tokio::test]
async fn test_user_decryption_request() -> Result<(), Box<dyn std::error::Error>> {
    let _context = crate::utils::TestContext::new().await;
    use alloy::network::ReceiptResponse;
    use alloy::primitives::{Address, Bytes, U256};
    use alloy::signers::{local::PrivateKeySigner, Signer};
    use fhevm_relayer::blockchain::ethereum::ComputeCalldata;
    use fhevm_relayer::config::settings::Settings;
    use fhevm_relayer::core::event::{HandleContractPair, RequestValidity};
    use fhevm_relayer::transaction::sender::TransactionManager;
    use fhevm_relayer::transaction::TxConfig;
    use std::str::FromStr;
    use std::sync::Arc;

    // Load configuration
    let settings = Settings::new(None).expect("Failed to load configuration");

    // Get network settings
    let gateway_settings = settings
        .get_network("gateway")
        .cloned()
        .expect("Failed to get gateway settings");

    // Test private key from environment variable or use default
    let private_key =
        std::env::var(&settings.transaction.private_key_gateway).unwrap_or_else(|_| {
            "7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f".to_string()
        });
    let mut gateway_signer: PrivateKeySigner = private_key.parse()?;
    gateway_signer.set_chain_id(Some(gateway_settings.chain_id));

    // Create transaction manager
    println!("Setting up manager with configured private key...");
    let manager = TransactionManager::new(&gateway_settings.ws_url, Arc::new(gateway_signer))
        .await
            .unwrap_or_else(|error| panic!(
                "Failed to create transaction manager. Make sure chain node is running at {}.\n{error}", gateway_settings.ws_url
            ));

    println!("Using address: {:?}", manager.sender_address());

    // Target contract address from config
    let decryption_address = Address::from_str(&settings.contracts.decryption_address)
        .expect("Invaliddecryption contract address");

    println!("Using decryption manager: {decryption_address:?}");

    println!("Checking contract state...");
    let code = manager
        .verify_contract_code(decryption_address)
        .await
        .expect("Failed to verify contract code");
    println!("Contract code size: {} bytes", code.len());

    // Create minimal test data
    println!("Creating minimal test data...");

    let simple_handle = U256::from(123); // Random handle
    let contract_addresses = vec![decryption_address];
    let ct_handle_contract_pairs = vec![HandleContractPair {
        ct_handle: simple_handle,
        contract_address: decryption_address,
    }];
    let request_validity = RequestValidity {
        start_timestamp: U256::from(1672531200), // random unix timestamp
        duration_days: U256::from(10),
    };

    let contracts_chain_id = gateway_settings.chain_id;
    let user_address = manager.sender_address();

    let public_key = Bytes::from(vec![1, 2, 3, 4, 5]);
    let signature = Bytes::from(vec![9, 8, 7, 6, 5]);

    let extra_data = Bytes::from(vec![0x00]);

    use fhevm_relayer::core::event::UserDecryptRequest;
    let user_decrypt_request: UserDecryptRequest = UserDecryptRequest {
        ct_handle_contract_pairs,
        request_validity,
        contracts_chain_id,
        contract_addresses,
        user_address,
        public_key,
        signature,
        extra_data,
    };

    // Create and prepare calldata using your existing function
    let calldata = ComputeCalldata::user_decryption_req(user_decrypt_request)
        .expect("Failed to prepare calldata");

    println!("Calldata prepared: 0x{}", hex::encode(&calldata));

    // Set up transaction config from app config
    let config = TxConfig::from(settings.transaction);

    // Try sending the actual transaction
    println!("Sending transaction...");
    match manager
        .send_transaction_and_wait(decryption_address, calldata, Some(config))
        .await
    {
        Ok(receipt) => {
            use alloy::rpc::types::{AnyReceiptEnvelope, Log, TransactionReceipt};
            let receipt: TransactionReceipt<AnyReceiptEnvelope<Log>> = receipt.inner;
            println!("Receipt status: {}", receipt.status());
            println!("Gas used: {}", receipt.gas_used);

            // Check for events
            for log in receipt.inner.logs() {
                println!(
                    "Log topics: {:?}",
                    log.topics().iter().map(hex::encode).collect::<Vec<_>>()
                );
            }
        }
        Err(e) => {
            println!("Error getting receipt: {e}");
        }
    }
    Ok(())
}

/// Test for diagnosing the user decryption request
/// This test checks if the contract has the expected function and if the function selector is present in the bytecode.
/// It works only with mock contracts because original contracts are deployed behind a proxy
#[tokio::test]
async fn test_diagnose_user_decryption_request() -> Result<(), Box<dyn std::error::Error>> {
    let _context = crate::utils::TestContext::new().await;
    use alloy::primitives::{keccak256, Address};
    use fhevm_relayer::config::settings::Settings;
    use fhevm_relayer::transaction::sender::TransactionManager;
    use std::str::FromStr;
    use std::sync::Arc;

    use alloy::signers::{local::PrivateKeySigner, Signer};

    println!("========== RUNNING DIAGNOSTIC TEST FOR USER DECRYPTION REQUEST ==========");

    // Load configuration
    let settings = Settings::new(None).expect("Failed to load configuration");
    let gateway_settings = settings
        .get_network("gateway")
        .cloned()
        .expect("Failed to get gateway settings");

    // Test private key from environment variable or use default
    let private_key =
        std::env::var(&settings.transaction.private_key_gateway).unwrap_or_else(|_| {
            "7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f".to_string()
        });
    let mut gateway_signer: PrivateKeySigner = private_key.parse()?;
    gateway_signer.set_chain_id(Some(gateway_settings.chain_id));

    println!("Setting up manager with configured private key...");
    let manager = TransactionManager::new(&gateway_settings.ws_url, Arc::new(gateway_signer))
        .await
            .unwrap_or_else(|error| panic!(
                "Failed to create transaction manager. Make sure chain node is running at {}.\n{error}", gateway_settings.ws_url
            ));

    let decryption_address = Address::from_str(&settings.contracts.decryption_address)
        .expect("Invaliddecryption contract address");

    println!("Using decryption manager: {decryption_address:?}");
    println!("Sender address: {:?}", manager.sender_address());

    use alloy::sol_types::SolEvent;
    use fhevm_gateway_bindings::decryption::Decryption::UserDecryptionRequest;
    println!("Looking for topic: {}", UserDecryptionRequest::SIGNATURE);

    // STEP 1: Check if the contract has the expected function
    println!("\nSTEP 1: Checking if contract implements userDecryptionRequest...");

    // Get the function selector for userDecryptionRequest
    let func_selector = &keccak256(
        "UserDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],address,bytes)",
    )[..4];
    println!("Function selector : 0x{}", hex::encode(func_selector));

    // STEP 2: Check contract code size
    let code = manager
        .provider
        .read()
        .await
        .get_code_at(decryption_address)
        .await?;
    println!("Contract code size: {} bytes", code.len());

    // Search for our function selector in the bytecode
    let selector_hex = hex::encode(func_selector);
    let code_hex = hex::encode(&code);
    if code_hex.contains(&selector_hex) {
        println!("✅ Function selector found in contract bytecode");
    } else {
        println!("❓ Function selector not found in bytecode (might be a proxy contract)");
    }

    Ok(())
}
