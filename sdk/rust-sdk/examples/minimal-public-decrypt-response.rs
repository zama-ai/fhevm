use gateway_sdk::{FhevmError, FhevmSdkBuilder};
use std::path::PathBuf;
use tracing::{Level, info};

fn main() -> Result<(), FhevmError> {
    // Initialize logging
    gateway_sdk::logging::init(Level::INFO);

    info!("Starting Public Decryption Example");

    // 1. Create SDK instance
    let sdk = FhevmSdkBuilder::new()
        .with_keys_directory(PathBuf::from("./keys"))
        .with_gateway_chain_id(43113) // Avalanche testnet
        .with_host_chain_id(11155111) // Ethereum Sepolia
        .with_decryption_contract("0xc9bAE822fE6793e3B456144AdB776D5A318CB71e")
        .with_input_verification_contract("0x1234567890123456789012345678901234567aaa")
        .with_acl_contract("0x0987654321098765432109876543210987654321")
        .build()?;

    // 2. Define the encrypted handles we want to decrypt
    // These would typically come from on-chain storage
    let handles =
        vec!["0xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390400".to_string()];

    info!("Preparing to decrypt handle: {}", handles[0]);

    // For this example, we'll use a mock response
    let mock_response = r#"{
        "response": [{
            "decrypted_value": "00000000000000000000000000000000000000000000000000000000000000f20000000000000000000000000000000000000000000000000000000000000060",
            "signatures": [
                "1a320a79075486da76ba3e9009200d298e35b7b4f0cd17364830a6793d74609a5862922b0e4af639e34de9dc40a5a48b390fb5c27fa8523d402e9a167c3444d61c"
            ]
        }]
    }"#;

    // 4. Process the decryption response
    // In a real application, you would:
    // - Use sdk.public_decrypt_request_builder() to build a request
    // - Send that request to the blockchain
    // - Wait for the gateway to process it
    // - Receive the response and process it as shown below
    let results = sdk
        .create_public_decrypt_response_builder()
        .kms_signers(vec![
            "0xF7a67027C94d141e5ebC7AeEE03FDF5fa8E0580C".to_string(),
        ])
        .threshold(1)
        .gateway_chain_id(54321)
        .verifying_contract_address("0xc9bAE822fE6793e3B456144AdB776D5A318CB71e")
        .ct_handles(handles)
        .json_response(mock_response)
        .process()?;

    // 5. Display the decrypted results
    info!("Successfully decrypted values:");
    for (handle, value) in &results {
        info!("Handle {}: {}", handle, value);
    }

    // In a real application you would use these decrypted values in your business logic

    info!("Public Decryption Example completed successfully");
    Ok(())
}
