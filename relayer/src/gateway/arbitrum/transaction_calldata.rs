use crate::core::{
    errors::EventProcessingError,
    event::{DelegatedUserDecryptRequest, UserDecryptRequest},
};
use crate::gateway::arbitrum::bindings::{
    Decryption,
    Decryption::CtHandleContractPair,
    IDecryption::{ContractsInfo, DelegationAccounts, RequestValidity},
    InputVerification,
};
use alloy::{
    hex,
    primitives::{Address, Bytes, FixedBytes, U256},
    sol_types::SolCall,
};
use tracing::info;

pub struct ComputeCalldata;

impl ComputeCalldata {
    /// Computes calldata for public decryption request
    ///
    /// This initiates a public decryption request on the gateway network
    pub fn public_decryption_req(
        handles: Vec<FixedBytes<32>>,
        extra_data: Bytes,
    ) -> Result<Bytes, EventProcessingError> {
        let calldata =
            Decryption::publicDecryptionRequestCall::new((handles, extra_data)).abi_encode();

        info!(
            "publicDecryptionRequest calldata: 0x{}",
            hex::encode(&calldata)
        );

        Ok(Bytes::from(calldata))
    }

    /// Computes calldata for user decryption request
    ///
    /// This initiates a user decryption request on the gateway network
    pub fn user_decryption_req(
        user_decrypt_request: UserDecryptRequest,
    ) -> Result<Bytes, EventProcessingError> {
        let ct_handle_contract_pairs = user_decrypt_request
            .ct_handle_contract_pairs
            .iter()
            .map(|d| CtHandleContractPair {
                ctHandle: d.ct_handle.into(),
                contractAddress: d.contract_address,
            })
            .collect::<Vec<_>>();

        let contracts_info = ContractsInfo {
            addresses: user_decrypt_request.contract_addresses,
            chainId: U256::from(user_decrypt_request.contracts_chain_id),
        };

        let validity = RequestValidity {
            startTimestamp: user_decrypt_request.request_validity.start_timestamp,
            durationDays: user_decrypt_request.request_validity.duration_days,
        };

        // Create the userDecryptionRequest call
        let call = Decryption::userDecryptionRequestCall::new((
            ct_handle_contract_pairs,
            validity,
            contracts_info,
            user_decrypt_request.user_address,
            user_decrypt_request.public_key,
            user_decrypt_request.signature,
            user_decrypt_request.extra_data,
        ));

        // Encode the call to get the calldata
        let calldata = Decryption::userDecryptionRequestCall::abi_encode(&call);

        info!(
            "UserDecryptionRequest calldata: 0x{}",
            hex::encode(&calldata)
        );

        Ok(Bytes::from(calldata))
    }

    /// Computes calldata for an RFC-021 (Solana host) user decryption request.
    ///
    /// Counterpart of [`Self::user_decryption_req`] where the user identity is a 32-byte Solana
    /// pubkey. The user's ed25519 signMessage authorization is verified by the relayer before this
    /// calldata is built (the sanctioned Solana auth seam), so the gateway call carries no on-chain
    /// signature. The ciphertext-handle ACL is enforced by the KMS (solana_acl), so no EVM
    /// contract-address list is sent. The ciphertext handles are 32-byte values shared with the EVM
    /// representation; only the dapp/user identities differ.
    pub fn user_decryption_req_solana(
        user_decrypt_request: &UserDecryptRequest,
        user_address: FixedBytes<32>,
    ) -> Result<Bytes, EventProcessingError> {
        let ct_handles = user_decrypt_request
            .ct_handle_contract_pairs
            .iter()
            .map(|pair| pair.ct_handle.into())
            .collect::<Vec<FixedBytes<32>>>();

        let validity = RequestValidity {
            startTimestamp: user_decrypt_request.request_validity.start_timestamp,
            durationDays: user_decrypt_request.request_validity.duration_days,
        };

        let request_call = Decryption::userDecryptionRequestSolanaCall {
            ctHandles: ct_handles,
            requestValidity: validity,
            contractsChainId: U256::from(user_decrypt_request.contracts_chain_id),
            userAddress: user_address,
            publicKey: user_decrypt_request.public_key.clone(),
            extraData: user_decrypt_request.extra_data.clone(),
        };

        let calldata = request_call.abi_encode();
        info!(
            "UserDecryptionRequestSolana calldata: 0x{}",
            hex::encode(&calldata)
        );
        Ok(Bytes::from(calldata))
    }

    /// Computes calldata for the delegated user decryption request
    pub fn delegated_user_decryption_req(
        delegated_user_decrypt_request: DelegatedUserDecryptRequest,
    ) -> Result<Bytes, EventProcessingError> {
        let ct_handle_contract_pairs = delegated_user_decrypt_request
            .ct_handle_contract_pairs
            .iter()
            .map(|d| CtHandleContractPair {
                ctHandle: d.ct_handle.into(),
                contractAddress: d.contract_address,
            })
            .collect::<Vec<_>>();

        let contracts_info = ContractsInfo {
            addresses: delegated_user_decrypt_request.contract_addresses,
            chainId: U256::from(delegated_user_decrypt_request.contracts_chain_id),
        };

        let validity = RequestValidity {
            startTimestamp: delegated_user_decrypt_request.start_timestamp,
            durationDays: delegated_user_decrypt_request.duration_days,
        };

        let delegation_accounts = DelegationAccounts {
            delegatorAddress: delegated_user_decrypt_request.delegator_address,
            delegateAddress: delegated_user_decrypt_request.delegate_address,
        };

        // Create the delegatedUserDecryptionRequest call
        let call = Decryption::delegatedUserDecryptionRequestCall::new((
            ct_handle_contract_pairs,
            validity,
            delegation_accounts,
            contracts_info,
            delegated_user_decrypt_request.public_key,
            delegated_user_decrypt_request.signature,
            delegated_user_decrypt_request.extra_data,
        ));

        // Encode the call to get the calldata
        let calldata = Decryption::delegatedUserDecryptionRequestCall::abi_encode(&call);

        info!(
            "DelegatedUserDecryptionRequest calldata: 0x{}",
            hex::encode(&calldata)
        );

        Ok(Bytes::from(calldata))
    }

    /// Computes calldata for input proof verification request
    ///
    /// This initiates a ZK proof verification request on the gateway network
    pub fn verify_proof_req(
        contract_chain_id: u64,
        contract_address: Address,
        user_address: Address,
        ciphertext_with_zkproof: Bytes,
        extra_data: Bytes,
    ) -> Result<Bytes, EventProcessingError> {
        let request_call = InputVerification::verifyProofRequestCall {
            contractChainId: U256::from(contract_chain_id),
            contractAddress: contract_address,
            userAddress: user_address,
            ciphertextWithZKProof: ciphertext_with_zkproof,
            extraData: extra_data,
        };
        let calldata = request_call.abi_encode();
        Ok(Bytes::from(calldata))
    }

    /// Solana (RFC-021) counterpart of [`Self::verify_proof_req`]: encodes a
    /// `verifyProofRequestSolana` call with 32-byte bytes32 host identities (Solana
    /// program id / pubkey). The contract chain id is the full u64 carrying the
    /// chain-type high bit.
    pub fn verify_proof_req_solana(
        contract_chain_id: u64,
        contract_address: FixedBytes<32>,
        user_address: FixedBytes<32>,
        ciphertext_with_zkproof: Bytes,
        extra_data: Bytes,
    ) -> Result<Bytes, EventProcessingError> {
        let request_call = InputVerification::verifyProofRequestSolanaCall {
            contractChainId: U256::from(contract_chain_id),
            contractAddress: contract_address,
            userAddress: user_address,
            ciphertextWithZKProof: ciphertext_with_zkproof,
            extraData: extra_data,
        };
        Ok(Bytes::from(request_call.abi_encode()))
    }
}

#[cfg(test)]
mod solana_calldata_tests {
    use super::*;
    use alloy::sol_types::SolCall;

    #[test]
    fn verify_proof_req_solana_encodes_bytes32_identities() {
        let contract = FixedBytes::<32>::from([0x11u8; 32]);
        let user = FixedBytes::<32>::from([0x22u8; 32]);
        // RFC-021 Solana host chain id (chain-type high bit set).
        let chain_id = (1u64 << 63) | 12345;

        let calldata = ComputeCalldata::verify_proof_req_solana(
            chain_id,
            contract,
            user,
            Bytes::from(vec![1, 2, 3]),
            Bytes::from(vec![0]),
        )
        .expect("encode solana verify_proof_req");

        // Round-trips through the verifyProofRequestSolana ABI (selector + args).
        let decoded = InputVerification::verifyProofRequestSolanaCall::abi_decode(&calldata)
            .expect("decode verifyProofRequestSolana calldata");
        assert_eq!(decoded.contractAddress, contract);
        assert_eq!(decoded.userAddress, user);
        assert_eq!(decoded.contractChainId, U256::from(chain_id));
        assert_eq!(decoded.ciphertextWithZKProof, Bytes::from(vec![1, 2, 3]));
    }
}
