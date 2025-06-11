#![allow(clippy::too_many_arguments)]

use alloy::sol;

// New version of DecryptionOracle
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    DecryptionOracle,
    "./artifacts/contract-abis/DecryptionOracle.json"
);

// Decryption contract (Gateway)
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    Decryption,
    "./artifacts/contract-abis/Decryption.json"
);

// Input Verification (Gateway)

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    InputVerification,
    "./artifacts/contract-abis/InputVerification.json"
);

// Define the Transfer event structure using alloy_sol_types
sol! {
    #[derive(Debug)]
    event Transfer(address indexed from, address indexed to, uint256 value);
}
