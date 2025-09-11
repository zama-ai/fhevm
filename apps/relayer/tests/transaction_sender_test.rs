mod utils;

use alloy::network::ReceiptResponse;
use alloy::primitives::{hex, keccak256, Address, Bytes, U256};
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::Signer;
use fhevm_relayer::transaction::nonce::DebugNonceManager;
use fhevm_relayer::transaction::sender::TransactionManager;
use fhevm_relayer::transaction::TxConfig;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
/// For this test having a running node is MANDATORY
/// url: http://localhost:8756
/// chain_id: 123456
/// The wallet associated to this private key should have fund:
/// 7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f
async fn test_counter_contract() {
    let _ = crate::utils::ensure_relayer_started().await;
    // Test private key (default)
    let private_key = std::env::var("TEST_PRIVATE_KEY").unwrap_or_else(|_| {
        "34aacca926bab195601bcf5702786d35cab968159b718ae671b226de11b9afee".to_string()
    });
    let chain_ws_url = "ws://localhost:8756";
    let chain_id = 123456;

    println!("Setting up manager with test private key...");
    let mut signer: PrivateKeySigner = private_key.parse().unwrap();
    signer.set_chain_id(Some(chain_id));

    let manager = TransactionManager::new(chain_ws_url, Arc::new(signer))
        .await
        .unwrap_or_else(|error| panic!(
            "Failed to create transaction manager. Make sure chain node is running at {chain_ws_url}.\n{error}"
        ));

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
    println!("Estimated deployment gas: {estimated_gas}");

    let config = TxConfig {
        gas_limit: Some((estimated_gas as f64 * 1.2) as u64), // 20% buffer
        max_priority_fee: Some(2_000_000_000),
        timeout_secs: Some(60),
        ..Default::default()
    };

    let current_nonce = DebugNonceManager::current_nonce(
        manager.nonce_manager.as_ref(),
        &**manager.provider.read().await,
        manager.sender_address(),
    )
    .await
    .expect("Failed to get nonce through Nonce Manager");
    println!("Nonce {current_nonce}",);

    // Deploy contract
    let tx_hash = manager
        .deploy_contract(Bytes::from(contract_bytecode), Some(config))
        .await
        .expect("Failed to deploy contract");
    println!("Deployment transaction hash: 0x{}", hex::encode(tx_hash));

    let receipt = manager
        .provider
        .read()
        .await
        .get_transaction_receipt(tx_hash)
        .await
        .expect("Failed to get receipt")
        .expect("Receipt not found");

    let current_nonce = DebugNonceManager::current_nonce(
        manager.nonce_manager.as_ref(),
        &**manager.provider().read().await,
        manager.sender_address(),
    )
    .await
    .expect("Failed to get nonce through Nonce Manager");
    println!("Nonce {current_nonce}",);

    println!("Deployment receipt status: {:?}", receipt.status());
    println!("Gas used: {}", receipt.gas_used);

    let contract_address = receipt
        .contract_address
        .expect("Contract address not found in receipt");

    println!("Contract deployed at: {contract_address:?}");

    // Verify contract code
    tokio::time::sleep(Duration::from_secs(2)).await; // Give node time to process
    let code = manager
        .provider
        .read()
        .await
        .get_code_at(contract_address)
        .await
        .expect("Failed to get code");

    println!("Deployed bytecode length: {}", code.len());
    assert!(!code.is_empty(), "Contract code should not be empty");

    let nonce_after_deploy = DebugNonceManager::current_nonce(
        manager.nonce_manager.as_ref(),
        &**manager.provider.read().await,
        manager.sender_address(),
    )
    .await
    .expect("Failed to get nonce through Nonce Manager");
    println!("Nonce: {nonce_after_deploy}",);

    // Decreasing a nonce < 0 will result in a panic
    // NOTE: we could probably modify the decrease function to return an error if the current
    // nonce is at 0
    assert!(nonce_after_deploy > 0);
    // NOTE: Manually decrease nonce to create a make sure that our library properly syncs with
    // RPC node in case of a Nonce-error
    DebugNonceManager::decrease_nonce(
        manager.nonce_manager.as_ref(),
        &**manager.provider.read().await,
        manager.sender_address(),
    )
    .await
    .expect("Failed to decrease nonce through Nonce Manager");
    println!(
        "Nonce after decrease: {}",
        DebugNonceManager::current_nonce(
            manager.nonce_manager.as_ref(),
            &**manager.provider.read().await,
            manager.sender_address(),
        )
        .await
        .expect("Failed to get nonce through Nonce Manager"),
    );

    // Get initial count
    let get_count_calldata = TransactionManager::encode_function_call(get_count_selector, vec![]);
    println!("Calling view function to get initial count...");
    let count_bytes = manager
        .call_view(contract_address, get_count_calldata.clone())
        .await
        .expect("Failed to get count");
    println!(
        "Nonce after view: {}",
        DebugNonceManager::current_nonce(
            manager.nonce_manager.as_ref(),
            &**manager.provider.read().await,
            manager.sender_address(),
        )
        .await
        .expect("Failed to get nonce through Nonce Manager"),
    );

    let initial_count = U256::from_be_slice(count_bytes.as_ref());
    println!("Initial count: {initial_count}");

    // Increment count
    let increment_calldata = TransactionManager::encode_function_call(increment_selector, vec![]);

    println!("Estimating gas for increment...");
    let estimated_gas = manager
        .estimate_gas(contract_address, increment_calldata.clone(), None)
        .await
        .expect("Failed to estimate gas");

    println!("Estimated gas for increment: {estimated_gas}");

    let config = TxConfig {
        gas_limit: Some(estimated_gas),
        max_priority_fee: Some(2_000_000_000),
        confirmations: Some(1),
        timeout_secs: Some(30),
        ..Default::default()
    };

    // NOTE: the nonce here is wrong in the CacheNonceManagerWithRefresh but the sync will be
    // handled automatically.
    println!("Sending increment transaction...");
    let tx_hash = manager
        .send_transaction(contract_address, increment_calldata, Some(config))
        .await
        .expect("Couldn't broadcast transaction");

    let nonce_after_increment = DebugNonceManager::current_nonce(
        manager.nonce_manager.as_ref(),
        &**manager.provider.read().await,
        manager.sender_address(),
    )
    .await
    .expect("Failed to get nonce through Nonce Manager");
    assert_eq!(nonce_after_increment, nonce_after_deploy + 1);

    println!("Increment transaction hash: 0x{}", hex::encode(tx_hash));

    tokio::time::sleep(Duration::from_secs(2)).await; // Give node time to process

    // Wait for confirmation and check receipt
    let receipt = manager
        .provider
        .read()
        .await
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
    println!("New count: {new_count}");

    assert_eq!(
        new_count,
        initial_count + U256::from(1),
        "Count should have increased by 1"
    );
}

#[test]
fn test_encode_function_call() {
    use fhevm_relayer::transaction::sender::TransactionManager;

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
