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
    primitives::{Bytes, FixedBytes, U256},
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
        input_proof_request: &crate::core::event::InputProofRequest,
    ) -> Result<Bytes, EventProcessingError> {
        let (Some(contract_address), Some(user_address)) = (
            input_proof_request.contract_address,
            input_proof_request.user_address,
        ) else {
            return Err(EventProcessingError::ValidationFailed {
                field: "input_proof_request".to_string(),
                reason: "contractAddress and userAddress are required".to_string(),
            });
        };

        let request_call = InputVerification::verifyProofRequestCall {
            contractChainId: U256::from(input_proof_request.contract_chain_id),
            contractAddress: contract_address,
            userAddress: user_address,
            ciphertextWithZKProof: input_proof_request.ciphetext_with_zk_proof.clone(),
            extraData: input_proof_request.extra_data.clone(),
        };
        let calldata = request_call.abi_encode();
        Ok(Bytes::from(calldata))
    }
}
