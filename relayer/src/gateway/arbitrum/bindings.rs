#![allow(clippy::too_many_arguments)]

use alloy::sol;

pub use fhevm_gateway_bindings::{
    ciphertext_commits::CiphertextCommits,
    decryption::{Decryption, IDecryption},
    input_verification::InputVerification,
};

sol! {
    #[derive(Debug)]
    struct NativeCtHandleContractPair {
        bytes32 ctHandle;
        bytes32 contractId;
    }

    #[derive(Debug)]
    struct NativeContractsInfo {
        uint256 chainId;
        bytes32[] ids;
    }

    #[derive(Debug)]
    struct NativeDelegationAccounts {
        bytes32 delegatorId;
        bytes32 delegateId;
    }

    #[derive(Debug)]
    struct DecryptionNativeRequestValidity {
        uint256 startTimestamp;
        uint256 durationDays;
    }

    #[derive(Debug)]
    struct NativeSnsCiphertextMaterial {
        bytes32 ctHandle;
        uint256 keyId;
        bytes32 snsCiphertextDigest;
        address[] coprocessorTxSenderAddresses;
    }

    #[sol(rpc)]
    interface DecryptionNative {
        event UserDecryptionRequestNative(
            uint256 indexed decryptionId,
            NativeSnsCiphertextMaterial[] snsCtMaterials,
            bytes32 userId,
            bytes publicKey,
            bytes extraData
        );

        function userDecryptionRequestNative(
            NativeCtHandleContractPair[] ctHandleContractPairs,
            DecryptionNativeRequestValidity requestValidity,
            NativeContractsInfo contractsInfo,
            bytes32 userId,
            bytes publicKey,
            bytes signature,
            bytes extraData
        ) external;

        function delegatedUserDecryptionRequestNative(
            NativeCtHandleContractPair[] ctHandleContractPairs,
            DecryptionNativeRequestValidity requestValidity,
            NativeDelegationAccounts delegationAccounts,
            NativeContractsInfo contractsInfo,
            bytes publicKey,
            bytes signature,
            bytes extraData
        ) external;

        function isUserDecryptionReadyNative(
            NativeCtHandleContractPair[] ctHandleContractPairs,
            bytes extraData
        ) external view returns (bool);

        function isDelegatedUserDecryptionReadyNative(
            NativeCtHandleContractPair[] ctHandleContractPairs,
            bytes extraData
        ) external view returns (bool);
    }

    #[sol(rpc)]
    interface InputVerificationNative {
        event VerifyProofRequestNative(
            uint256 indexed zkProofId,
            uint256 indexed contractChainId,
            bytes32 contractId,
            bytes32 userId,
            bytes ciphertextWithZKProof,
            bytes extraData
        );

        function verifyProofRequestNative(
            uint256 contractChainId,
            bytes32 contractId,
            bytes32 userId,
            bytes ciphertextWithZKProof,
            bytes extraData
        ) external;
    }
}

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
