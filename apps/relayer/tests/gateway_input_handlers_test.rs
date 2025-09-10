mod utils;

#[tokio::test]
async fn test_input_verification_request() -> Result<(), Box<dyn std::error::Error>> {
    use alloy::network::ReceiptResponse;
    use alloy::primitives::{Address, Bytes};
    use alloy::signers::{local::PrivateKeySigner, Signer};
    use fhevm_relayer::blockchain::ethereum::ComputeCalldata;
    use fhevm_relayer::config::settings::Settings;
    use fhevm_relayer::transaction::sender::TransactionManager;
    use fhevm_relayer::transaction::TxConfig;
    use std::str::FromStr;
    use std::sync::Arc;

    println!("\n========== INPUT VERIFICATION (ZK PROOF) TEST ==========\n");

    // Load configuration
    let settings = Settings::new(None).expect("Failed to load configuration");

    // Get network settings
    let gateway_settings = settings
        .get_network("gateway")
        .cloned()
        .expect("Failed to get gateway settings");

    println!("Network URL: {}", gateway_settings.http_url);
    println!("Chain ID: {}", gateway_settings.chain_id);

    // Test private key from environment variable or use default
    let private_key =
        std::env::var(&settings.transaction.private_key_gateway).unwrap_or_else(|_| {
            "9f5e213176c6d97cba246563083794ebeb8098c51dbcaf91e9f71a29db2ffd88".to_string()
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

    let sender_address = manager.sender_address();
    println!("Sender address: {sender_address:#x}");

    // Get the input verification contract address from config
    let input_verification_address =
        Address::from_str(&settings.contracts.input_verification_address)
            .expect("Invalid input verification contract address");

    println!("Target input verification contract: {input_verification_address:#x}");

    // Check contract code
    println!("Checking contract state...");
    let code = manager
        .verify_contract_code(input_verification_address)
        .await
        .expect("Failed to verify contract code");
    println!("Contract code size: {} bytes", code.len());

    if code.is_empty() {
        println!(
            "⚠️ WARNING: No code at target address! The contract might be a proxy or not deployed."
        );
    }

    // Create test data for the input verification request
    println!("\nCreating test data for input verification...");

    // Target contract for the proof
    let target_contract_address = input_verification_address; // Using same address for simplicity
    let target_contract_chain_id = gateway_settings.chain_id;
    let user_address = sender_address;

    // Create dummy proof data - in a real scenario this would be actual ZK proof data
    // Note: This is just for testing - a real ZK proof would be generated properly
    let proof_data = vec![
        // Mock header (8 bytes)
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        // Mock proof body (variable length)
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
        0x00, // Mock ciphertext data (variable length)
        0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd,
        0xef,
    ];

    let ciphertext_with_zk_proof = Bytes::from(proof_data);

    println!("Proof data size: {} bytes", ciphertext_with_zk_proof.len());
    println!("Target contract: {target_contract_address:#x}");
    println!("Target chain ID: {target_contract_chain_id}");
    println!("User address: {user_address:#x}");

    // Generate calldata for the verification request
    let calldata = ComputeCalldata::verify_proof_req(
        target_contract_chain_id,
        target_contract_address,
        user_address,
        ciphertext_with_zk_proof,
        Bytes::from(vec![0x00]),
    )
    .expect("Failed to prepare calldata");

    println!(
        "Calldata prepared: 0x{}...",
        hex::encode(&calldata[..std::cmp::min(64, calldata.len())])
    );
    println!("Total calldata length: {} bytes", calldata.len());

    // Simulate transaction first (dry run)
    println!("\nSimulating transaction call...");
    let simulation_result = manager
        .call_view(input_verification_address, calldata.clone())
        .await;
    match simulation_result {
        Ok(result) => {
            println!("Simulation successful!");
            if !result.is_empty() {
                println!("Result: 0x{}", hex::encode(&result));
            } else {
                println!("No return data (this is normal for many transactions)");
            }
        }
        Err(e) => {
            println!("Simulation failed: {e}");
            println!("This indicates the transaction would likely revert.");
        }
    }

    // Estimate gas for the transaction
    println!("\nEstimating gas...");
    let gas_result = manager
        .estimate_gas(input_verification_address, calldata.clone(), None)
        .await;

    match gas_result {
        Ok(gas) => {
            println!("Gas estimation successful: {gas} gas units");

            // Set up transaction config
            let mut config = TxConfig::from(settings.transaction.clone());

            // Override with our estimated gas (plus buffer)
            let gas_with_buffer = (gas as f64 * 1.2) as u64; // 20% buffer
            config.gas_limit = Some(gas_with_buffer);

            println!("Using gas limit: {gas_with_buffer} (added 20% buffer)");

            // Send the transaction without asking for confirmation
            println!("Sending transaction...");
            match manager
                .send_transaction_and_wait(input_verification_address, calldata, Some(config))
                .await
            {
                Ok(receipt) => {
                    use alloy::rpc::types::{AnyReceiptEnvelope, Log, TransactionReceipt};
                    let receipt: TransactionReceipt<AnyReceiptEnvelope<Log>> = receipt.inner;
                    println!("\n✅ TRANSACTION SUCCESSFUL!");
                    println!("Transaction hash: {:#x}", receipt.transaction_hash);
                    println!("Block number: {}", receipt.block_number.unwrap_or_default());
                    println!("Gas used: {}", receipt.gas_used);
                    println!(
                        "Status: {}",
                        if receipt.status() {
                            "SUCCESS"
                        } else {
                            "FAILED"
                        }
                    );

                    // Look for the VerifyProofRequest event
                    println!("\nEvent logs ({}):", receipt.inner.logs().len());
                    // println!("\nEvent logs ({}):", receipt.logs().len());

                    for (i, log) in receipt.inner.logs().iter().enumerate() {
                        println!("Log #{}:", i + 1);

                        let topics: Vec<String> = log
                            .topics()
                            .iter()
                            .map(|t| format!("0x{}", hex::encode(t)))
                            .collect();

                        println!("  Topics: {topics:?}");

                        // Safe way to handle log data
                        let data = log.data();
                        if !data.data.is_empty() {
                            // Only show first 64 bytes if data is longer
                            println!("  Data length: {} bytes", data.data.len());
                        } else {
                            println!("  Data: Empty");
                        }
                    }
                }
                Err(e) => {
                    println!("\n❌ TRANSACTION FAILED: {e}");
                }
            }
        }
        Err(e) => {
            println!("Gas estimation failed: {e}");
            println!("This indicates the transaction would likely revert if sent.");
        }
    }

    Ok(())
}
