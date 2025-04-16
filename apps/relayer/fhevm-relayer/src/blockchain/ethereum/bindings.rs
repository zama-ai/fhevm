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

// TFHE EXecutor

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    TFHEExecutor,
    "./artifacts/contract-abis/TFHEExecutor.json"
);

// Decryption Manager (Rollup)
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    DecyptionManager,
    "./artifacts/contract-abis/DecryptionManager.json"
);

// ZKPoK  Manager (Rollup)

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    ZKPoKManager,
    "./artifacts/contract-abis/ZKPoKManager.json"
);

// Define the Transfer event structure using alloy_sol_types
sol! {
    #[derive(Debug)]
    event Transfer(address indexed from, address indexed to, uint256 value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_sol_types::SolEvent;

    #[test]
    fn test_decryption() {
        println!(
            "DecryptionManager UserDecryptionRequest:\n{}\n{}\n",
            DecyptionManager::UserDecryptionRequest::SIGNATURE,
            DecyptionManager::UserDecryptionRequest::SIGNATURE_HASH
        );
        println!(
            "DecryptionManager UserDecryptionResponse:\n{}\n{}\n",
            DecyptionManager::UserDecryptionResponse::SIGNATURE,
            DecyptionManager::UserDecryptionResponse::SIGNATURE_HASH
        );
    }

    #[test]
    fn test_zkpok() {
        println!(
            "ZKPoKManager VerifyProofRequest:\n{}\n{}\n",
            ZKPoKManager::VerifyProofRequest::SIGNATURE,
            ZKPoKManager::VerifyProofRequest::SIGNATURE_HASH
        );
        println!(
            "ZKPoKManager VerifyProofResponse:\n{}\n{}\n",
            ZKPoKManager::VerifyProofResponse::SIGNATURE,
            ZKPoKManager::VerifyProofResponse::SIGNATURE_HASH
        );
    }
}
