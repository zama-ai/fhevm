mod common;

use fhevm_relayer::transaction::sender::TransactionManager;

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
