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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::sol_types::SolEvent;

    #[test]
    fn test_decryption() {
        println!(
            "DecryptionManager UserDecryptionRequest:\n{}\n{}\n",
            Decryption::UserDecryptionRequest::SIGNATURE,
            Decryption::UserDecryptionRequest::SIGNATURE_HASH
        );
        println!(
            "DecryptionManager UserDecryptionResponse:\n{}\n{}\n",
            Decryption::UserDecryptionResponse::SIGNATURE,
            Decryption::UserDecryptionResponse::SIGNATURE_HASH
        );
    }

    #[test]
    fn test_input_verification() {
        println!(
            "InputVerification VerifyProofRequest:\n{}\n{}\n",
            InputVerification::VerifyProofRequest::SIGNATURE,
            InputVerification::VerifyProofRequest::SIGNATURE_HASH
        );
        println!(
            "InputVerification VerifyProofResponse:\n{}\n{}\n",
            InputVerification::VerifyProofResponse::SIGNATURE,
            InputVerification::VerifyProofResponse::SIGNATURE_HASH
        );
    }
}
