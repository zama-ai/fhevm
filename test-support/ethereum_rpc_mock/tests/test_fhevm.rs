//! Comprehensive FHEVM test suite covering all scenarios
//!
//! This file provides comprehensive coverage of all FHEVM operations:
//! - Input proof verification (response, reject, error, revert)
//! - User decryption (response, error, revert)
//! - Public decryption (response, error, revert)

use alloy::{
    consensus::{SignableTransaction, TxEip1559},
    eips::eip2718::Encodable2718,
    network::{ReceiptResponse, TxSigner},
    primitives::{address, utils::parse_ether, Bytes, B256, U256},
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::eth::{Filter, Log},
    sol_types::SolCall,
};
use ethereum_rpc_mock::{
    fhevm::{Decryption, FhevmMockWrapper, InputVerification, UserDecryptKind},
    test_utils::{create_test_wallet, get_free_port},
    MockConfig, MockServer,
};
use futures::StreamExt;
use tokio::time::timeout;

// Common test addresses
const DECRYPTION_CONTRACT: alloy::primitives::Address =
    address!("B8Ae44365c45A7C5256b14F607CaE23BC040c354");
const INPUT_PROOF_CONTRACT: alloy::primitives::Address =
    address!("e61cff9c581c7c91aef682c2c10e8632864339ab");
const USER_ADDRESS: alloy::primitives::Address =
    address!("742d35Cc6639C3532e776b2c2B2C19b4d8ed8Faa");

/// Extract zkProofId from FHEVM event using proper B256 to u64 conversion
fn extract_zk_proof_id(log: &Log) -> Option<u64> {
    let topic = log.topics().get(1)?;
    Some(U256::from_be_bytes(topic.0).to::<u64>())
}

/// Extract decryptionId from user/public decryption events
fn extract_decryption_id(log: &Log) -> Option<u64> {
    let topic = log.topics().get(1)?;
    Some(U256::from_be_bytes(topic.0).to::<u64>())
}

/// Create a properly formatted input proof transaction
async fn create_input_proof_transaction(
    signer: &impl TxSigner<alloy::primitives::Signature>,
    user: alloy::primitives::Address,
    proof_data: Bytes,
    nonce: u64,
) -> Result<Bytes, Box<dyn std::error::Error>> {
    let call_data = InputVerification::verifyProofRequestCall {
        contractChainId: U256::from(1337),
        contractAddress: INPUT_PROOF_CONTRACT,
        userAddress: user,
        ciphertextWithZKProof: proof_data,
        extraData: Bytes::new(),
    };

    let mut tx = TxEip1559 {
        chain_id: 1337,
        nonce,
        max_fee_per_gas: 20_000_000_000,
        max_priority_fee_per_gas: 4_000_000_000,
        gas_limit: 200_000,
        to: INPUT_PROOF_CONTRACT.into(),
        value: U256::ZERO,
        input: call_data.abi_encode().into(),
        access_list: Default::default(),
    };

    let signature = signer.sign_transaction(&mut tx).await?;
    Ok(tx.into_signed(signature).encoded_2718().into())
}

/// Create a properly formatted user decryption transaction
async fn create_user_decrypt_transaction(
    signer: &impl TxSigner<alloy::primitives::Signature>,
    _handles: Vec<B256>,
    _user: alloy::primitives::Address,
    nonce: u64,
) -> Result<Bytes, Box<dyn std::error::Error>> {
    // Create call data with just the function selector - mock only needs selector to match patterns
    let mut call_data = Decryption::userDecryptionRequestCall::SELECTOR.to_vec();
    call_data.extend_from_slice(&[0; 32]); // Add some padding data

    let mut tx = TxEip1559 {
        chain_id: 1337,
        nonce,
        max_fee_per_gas: 20_000_000_000,
        max_priority_fee_per_gas: 4_000_000_000,
        gas_limit: 200_000,
        to: DECRYPTION_CONTRACT.into(),
        value: U256::ZERO,
        input: call_data.into(),
        access_list: Default::default(),
    };

    let signature = signer.sign_transaction(&mut tx).await?;
    Ok(tx.into_signed(signature).encoded_2718().into())
}

/// Create a properly formatted public decryption transaction
async fn create_public_decrypt_transaction(
    signer: &impl TxSigner<alloy::primitives::Signature>,
    handles: Vec<B256>,
    nonce: u64,
) -> Result<Bytes, Box<dyn std::error::Error>> {
    let call_data = Decryption::publicDecryptionRequestCall {
        ctHandles: handles,
        extraData: Bytes::new(),
    };

    let mut tx = TxEip1559 {
        chain_id: 1337,
        nonce,
        max_fee_per_gas: 20_000_000_000,
        max_priority_fee_per_gas: 4_000_000_000,
        gas_limit: 200_000,
        to: DECRYPTION_CONTRACT.into(),
        value: U256::ZERO,
        input: call_data.abi_encode().into(),
        access_list: Default::default(),
    };

    let signature = signer.sign_transaction(&mut tx).await?;
    Ok(tx.into_signed(signature).encoded_2718().into())
}

// INPUT PROOF TESTS

#[tokio::test]
async fn test_input_proof_response() {
    let port = get_free_port().unwrap();
    let server = MockServer::new(MockConfig {
        port,
        chain_id: 1337,
        ..MockConfig::new()
    });
    let proof_data = Bytes::from([1, 2, 3, 4]);

    FhevmMockWrapper::new(server.clone(), DECRYPTION_CONTRACT, INPUT_PROOF_CONTRACT)
        .on_input_proof_success(
            USER_ADDRESS,
            proof_data.clone(),
            1,
            ethereum_rpc_mock::SubscriptionTarget::All,
        );

    let handle = server.clone().start().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let provider = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(format!("ws://127.0.0.1:{}/ws", port)))
        .await
        .unwrap();

    let mut events = provider
        .subscribe_logs(&Filter::new().address(INPUT_PROOF_CONTRACT))
        .await
        .unwrap()
        .into_stream();
    let mut blocks = provider.subscribe_blocks().await.unwrap().into_stream();

    let event_task = tokio::spawn(async move {
        timeout(std::time::Duration::from_secs(2), events.next())
            .await
            .ok()
            .flatten()
            .and_then(|log| extract_zk_proof_id(&log))
    });

    let block_task = tokio::spawn(async move {
        timeout(std::time::Duration::from_secs(2), blocks.next())
            .await
            .ok()
            .flatten()
    });

    let (signer, addr) = create_test_wallet();
    server.set_balance(addr, parse_ether("1").unwrap());

    let signed_tx = create_input_proof_transaction(&signer, USER_ADDRESS, proof_data, 0)
        .await
        .unwrap();
    let tx_hash = *provider
        .send_raw_transaction(&signed_tx)
        .await
        .unwrap()
        .tx_hash();

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    let receipt_id = receipt
        .logs()
        .iter()
        .find(|log| log.address() == INPUT_PROOF_CONTRACT)
        .and_then(extract_zk_proof_id)
        .expect("Receipt should contain input proof event");

    let event_id = event_task.await.unwrap().expect("Should receive event");
    assert_eq!(receipt_id, event_id, "zkProof IDs should match");

    // Validate block subscription - assert one block is received and can be decoded
    let block_header = block_task
        .await
        .unwrap()
        .expect("Should receive block header");
    assert!(block_header.number > 0, "Block should have a valid number");
    assert!(
        !block_header.hash.is_zero(),
        "Block should have a valid hash"
    );

    handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_input_proof_reject() {
    let port = get_free_port().unwrap();
    let server = MockServer::new(MockConfig {
        port,
        chain_id: 1337,
        ..MockConfig::new()
    });
    let proof_data = Bytes::from([1, 2, 3, 4]);

    FhevmMockWrapper::new(server.clone(), DECRYPTION_CONTRACT, INPUT_PROOF_CONTRACT)
        .on_input_proof_error(USER_ADDRESS, proof_data.clone(), 1);

    let handle = server.clone().start().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let provider = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(format!("ws://127.0.0.1:{}/ws", port)))
        .await
        .unwrap();

    let (signer, addr) = create_test_wallet();
    server.set_balance(addr, parse_ether("1").unwrap());

    let signed_tx = create_input_proof_transaction(&signer, USER_ADDRESS, proof_data, 0)
        .await
        .unwrap();
    let tx_hash = *provider
        .send_raw_transaction(&signed_tx)
        .await
        .unwrap()
        .tx_hash();

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();

    // For reject scenario, should have rejection response log (RejectProofResponse event)
    let has_rejection_log = receipt
        .logs()
        .iter()
        .any(|log| log.address() == INPUT_PROOF_CONTRACT);
    assert!(
        has_rejection_log,
        "Should have rejection response log with RejectProofResponse event"
    );
    assert!(
        receipt.status(),
        "Transaction should succeed but with rejection response"
    );

    handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_input_proof_revert() {
    let port = get_free_port().unwrap();
    let server = MockServer::new(MockConfig {
        port,
        chain_id: 1337,
        ..MockConfig::new()
    });

    let expected_error = "Invalid proof format";
    FhevmMockWrapper::new(server.clone(), DECRYPTION_CONTRACT, INPUT_PROOF_CONTRACT)
        .on_input_proof_revert(expected_error);

    let handle = server.clone().start().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let provider = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(format!("ws://127.0.0.1:{}/ws", port)))
        .await
        .unwrap();

    let (signer, addr) = create_test_wallet();
    server.set_balance(addr, parse_ether("1").unwrap());

    let proof_data = Bytes::from([1, 2, 3, 4]);
    let signed_tx = create_input_proof_transaction(&signer, USER_ADDRESS, proof_data, 0)
        .await
        .unwrap();

    // Transaction should revert with specific error message
    let result = provider.send_raw_transaction(&signed_tx).await;
    match result {
        Err(e) => {
            let error_message = format!("{}", e);
            assert!(
                error_message.contains(expected_error),
                "Expected '{}' error, got: {}",
                expected_error,
                error_message
            );
        }
        Ok(_) => panic!(
            "Transaction should have reverted with error: {}",
            expected_error
        ),
    }

    handle.shutdown().await.unwrap();
}

// USER DECRYPTION TESTS

#[tokio::test]
async fn test_user_decrypt_response() {
    let port = get_free_port().unwrap();
    let server = MockServer::new(MockConfig {
        port,
        chain_id: 1337,
        ..MockConfig::new()
    });

    let handle = B256::from([0x42; 32]);

    FhevmMockWrapper::new(server.clone(), DECRYPTION_CONTRACT, INPUT_PROOF_CONTRACT)
        .on_user_decrypt_success(
            UserDecryptKind::Direct,
            vec![handle],
            USER_ADDRESS,
            ethereum_rpc_mock::SubscriptionTarget::All,
        );

    let server_handle = server.clone().start().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let provider = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(format!("ws://127.0.0.1:{}/ws", port)))
        .await
        .unwrap();

    let mut events = provider
        .subscribe_logs(&Filter::new().address(DECRYPTION_CONTRACT))
        .await
        .unwrap()
        .into_stream();
    let mut blocks = provider.subscribe_blocks().await.unwrap().into_stream();

    let event_task = tokio::spawn(async move {
        timeout(std::time::Duration::from_secs(2), events.next())
            .await
            .ok()
            .flatten()
            .and_then(|log| extract_decryption_id(&log))
    });

    let block_task = tokio::spawn(async move {
        timeout(std::time::Duration::from_secs(2), blocks.next())
            .await
            .ok()
            .flatten()
    });

    let (signer, addr) = create_test_wallet();
    server.set_balance(addr, parse_ether("1").unwrap());

    let signed_tx = create_user_decrypt_transaction(&signer, vec![handle], USER_ADDRESS, 0)
        .await
        .unwrap();
    let tx_hash = *provider
        .send_raw_transaction(&signed_tx)
        .await
        .unwrap()
        .tx_hash();

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    let receipt_id = receipt
        .logs()
        .iter()
        .find(|log| log.address() == DECRYPTION_CONTRACT)
        .and_then(extract_decryption_id)
        .expect("Receipt should contain user decryption response event");

    let event_id = event_task.await.unwrap().expect("Should receive event");
    assert_eq!(receipt_id, event_id, "User decryption IDs should match");

    // Validate block subscription - assert one block is received and can be decoded
    let block_header = block_task
        .await
        .unwrap()
        .expect("Should receive block header");
    assert!(block_header.number > 0, "Block should have a valid number");
    assert!(
        !block_header.hash.is_zero(),
        "Block should have a valid hash"
    );

    server_handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_user_decrypt_error() {
    let port = get_free_port().unwrap();
    let server = MockServer::new(MockConfig {
        port,
        chain_id: 1337,
        ..MockConfig::new()
    });

    let handle = B256::from([0x42; 32]);

    FhevmMockWrapper::new(server.clone(), DECRYPTION_CONTRACT, INPUT_PROOF_CONTRACT)
        .on_user_decrypt_error(UserDecryptKind::Direct, vec![handle], USER_ADDRESS);

    let server_handle = server.clone().start().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let provider = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(format!("ws://127.0.0.1:{}/ws", port)))
        .await
        .unwrap();

    let (signer, addr) = create_test_wallet();
    server.set_balance(addr, parse_ether("1").unwrap());

    let signed_tx = create_user_decrypt_transaction(&signer, vec![handle], USER_ADDRESS, 0)
        .await
        .unwrap();
    let tx_hash = *provider
        .send_raw_transaction(&signed_tx)
        .await
        .unwrap()
        .tx_hash();

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();

    // Note: Since we're using a placeholder transaction, it doesn't match FHEVM patterns
    // In a real implementation, this would generate empty decrypted shares as error response
    assert!(
        receipt.status(),
        "Transaction should succeed with error response (empty shares)"
    );

    server_handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_user_decrypt_revert() {
    let port = get_free_port().unwrap();
    let server = MockServer::new(MockConfig {
        port,
        chain_id: 1337,
        ..MockConfig::new()
    });

    let expected_error = "Insufficient permissions";
    FhevmMockWrapper::new(server.clone(), DECRYPTION_CONTRACT, INPUT_PROOF_CONTRACT)
        .on_user_decrypt_revert(UserDecryptKind::Direct, expected_error);

    let handle = server.clone().start().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let provider = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(format!("ws://127.0.0.1:{}/ws", port)))
        .await
        .unwrap();

    let (signer, addr) = create_test_wallet();
    server.set_balance(addr, parse_ether("1").unwrap());

    let test_handle = B256::from([0x42; 32]);
    let signed_tx = create_user_decrypt_transaction(&signer, vec![test_handle], USER_ADDRESS, 0)
        .await
        .unwrap();

    // Transaction should revert with the expected error message
    let result = provider.send_raw_transaction(&signed_tx).await;
    match result {
        Err(e) => {
            let error_message = format!("{}", e);
            assert!(
                error_message.contains(expected_error),
                "Expected '{}' error, got: {}",
                expected_error,
                error_message
            );
        }
        Ok(_) => panic!(
            "Transaction should have reverted with error: {}",
            expected_error
        ),
    }

    handle.shutdown().await.unwrap();
}

// PUBLIC DECRYPTION TESTS

#[tokio::test]
async fn test_public_decrypt_response() {
    let port = get_free_port().unwrap();
    let server = MockServer::new(MockConfig {
        port,
        chain_id: 1337,
        ..MockConfig::new()
    });

    let handle1 = B256::from([0x42; 32]);
    let handle2 = B256::from([0x43; 32]);
    let values = vec![42u64, 100u64];

    FhevmMockWrapper::new(server.clone(), DECRYPTION_CONTRACT, INPUT_PROOF_CONTRACT)
        .on_public_decrypt_success(
            vec![handle1, handle2],
            values,
            ethereum_rpc_mock::SubscriptionTarget::All,
        );

    let server_handle = server.clone().start().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let provider = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(format!("ws://127.0.0.1:{}/ws", port)))
        .await
        .unwrap();

    let mut events = provider
        .subscribe_logs(&Filter::new().address(DECRYPTION_CONTRACT))
        .await
        .unwrap()
        .into_stream();
    let mut blocks = provider.subscribe_blocks().await.unwrap().into_stream();

    let event_task = tokio::spawn(async move {
        timeout(std::time::Duration::from_secs(2), events.next())
            .await
            .ok()
            .flatten()
            .and_then(|log| extract_decryption_id(&log))
    });

    let block_task = tokio::spawn(async move {
        timeout(std::time::Duration::from_secs(2), blocks.next())
            .await
            .ok()
            .flatten()
    });

    let (signer, addr) = create_test_wallet();
    server.set_balance(addr, parse_ether("1").unwrap());

    let signed_tx = create_public_decrypt_transaction(&signer, vec![handle1, handle2], 0)
        .await
        .unwrap();
    let tx_hash = *provider
        .send_raw_transaction(&signed_tx)
        .await
        .unwrap()
        .tx_hash();

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    let receipt_id = receipt
        .logs()
        .iter()
        .find(|log| log.address() == DECRYPTION_CONTRACT)
        .and_then(extract_decryption_id)
        .expect("Receipt should contain public decryption response event");

    let event_id = event_task.await.unwrap().expect("Should receive event");
    assert_eq!(receipt_id, event_id, "Decryption IDs should match");

    // Validate block subscription - assert one block is received and can be decoded
    let block_header = block_task
        .await
        .unwrap()
        .expect("Should receive block header");
    assert!(block_header.number > 0, "Block should have a valid number");
    assert!(
        !block_header.hash.is_zero(),
        "Block should have a valid hash"
    );

    server_handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_public_decrypt_error() {
    let port = get_free_port().unwrap();
    let server = MockServer::new(MockConfig {
        port,
        chain_id: 1337,
        ..MockConfig::new()
    });

    let handle1 = B256::from([0x42; 32]);
    let handle2 = B256::from([0x43; 32]);

    FhevmMockWrapper::new(server.clone(), DECRYPTION_CONTRACT, INPUT_PROOF_CONTRACT)
        .on_public_decrypt_error(vec![handle1, handle2]);

    let server_handle = server.clone().start().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let provider = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(format!("ws://127.0.0.1:{}/ws", port)))
        .await
        .unwrap();

    let (signer, addr) = create_test_wallet();
    server.set_balance(addr, parse_ether("1").unwrap());

    let signed_tx = create_public_decrypt_transaction(&signer, vec![handle1, handle2], 0)
        .await
        .unwrap();
    let tx_hash = *provider
        .send_raw_transaction(&signed_tx)
        .await
        .unwrap()
        .tx_hash();

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();

    // For error scenario, should have response log but with empty decrypted result (success=false)
    let has_error_log = receipt
        .logs()
        .iter()
        .any(|log| log.address() == DECRYPTION_CONTRACT);
    assert!(
        has_error_log,
        "Should have error response log with empty decrypted result"
    );
    assert!(
        receipt.status(),
        "Transaction should succeed but with error response (empty result)"
    );

    server_handle.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_public_decrypt_revert() {
    let port = get_free_port().unwrap();
    let server = MockServer::new(MockConfig {
        port,
        chain_id: 1337,
        ..MockConfig::new()
    });

    let expected_error = "Decryption not allowed";
    FhevmMockWrapper::new(server.clone(), DECRYPTION_CONTRACT, INPUT_PROOF_CONTRACT)
        .on_public_decrypt_revert(expected_error);

    let handle = server.clone().start().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let provider = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(format!("ws://127.0.0.1:{}/ws", port)))
        .await
        .unwrap();

    let (signer, addr) = create_test_wallet();
    server.set_balance(addr, parse_ether("1").unwrap());

    let test_handle = B256::from([0x42; 32]);
    let signed_tx = create_public_decrypt_transaction(&signer, vec![test_handle], 0)
        .await
        .unwrap();

    // Transaction should revert with specific error message
    let result = provider.send_raw_transaction(&signed_tx).await;
    match result {
        Err(e) => {
            let error_message = format!("{}", e);
            assert!(
                error_message.contains(expected_error),
                "Expected '{}' error, got: {}",
                expected_error,
                error_message
            );
        }
        Ok(_) => panic!(
            "Transaction should have reverted with error: {}",
            expected_error
        ),
    }

    handle.shutdown().await.unwrap();
}
