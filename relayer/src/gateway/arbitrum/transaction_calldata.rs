use crate::core::{
    errors::EventProcessingError,
    event::{DelegatedUserDecryptRequest, UserDecryptRequest},
};
use crate::gateway::arbitrum::bindings::{
    Decryption,
    Decryption::CtHandleContractPair,
    DecryptionNative,
    DecryptionNativeRequestValidity,
    IDecryption::{ContractsInfo, DelegationAccounts, RequestValidity},
    InputVerification,
    InputVerificationNative,
    NativeContractsInfo,
    NativeCtHandleContractPair as NativeCtHandleContractPairCall,
    NativeDelegationAccounts,
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
        let calldata = if let (None, Some(user_id), Some(contract_ids)) = (
            user_decrypt_request.user_address,
            user_decrypt_request.user_id,
            user_decrypt_request.contract_ids.clone(),
        ) {
            let native_pairs = user_decrypt_request
                .ct_handle_contract_pairs
                .iter()
                .map(|d| NativeCtHandleContractPairCall {
                    ctHandle: d.ct_handle.into(),
                    contractId: d.contract_id.expect("native user decrypt requires contractId"),
                })
                .collect::<Vec<_>>();
            let native_validity = DecryptionNativeRequestValidity {
                startTimestamp: user_decrypt_request.request_validity.start_timestamp,
                durationDays: user_decrypt_request.request_validity.duration_days,
            };
            let native_contracts_info = NativeContractsInfo {
                ids: contract_ids,
                chainId: U256::from(user_decrypt_request.contracts_chain_id),
            };
            let call = DecryptionNative::userDecryptionRequestNativeCall::new((
                native_pairs,
                native_validity,
                native_contracts_info,
                user_id,
                user_decrypt_request.public_key,
                user_decrypt_request.signature,
                user_decrypt_request.extra_data,
            ));
            DecryptionNative::userDecryptionRequestNativeCall::abi_encode(&call)
        } else {
            let ct_handle_contract_pairs = user_decrypt_request
                .ct_handle_contract_pairs
                .iter()
                .map(|d| CtHandleContractPair {
                    ctHandle: d.ct_handle.into(),
                    contractAddress: d
                        .contract_address
                        .expect("legacy user decrypt requires contractAddress"),
                })
                .collect::<Vec<_>>();
            let validity = RequestValidity {
                startTimestamp: user_decrypt_request.request_validity.start_timestamp,
                durationDays: user_decrypt_request.request_validity.duration_days,
            };
            let contracts_info = ContractsInfo {
                addresses: user_decrypt_request.contract_addresses,
                chainId: U256::from(user_decrypt_request.contracts_chain_id),
            };
            let call = Decryption::userDecryptionRequestCall::new((
                ct_handle_contract_pairs,
                validity,
                contracts_info,
                user_decrypt_request.user_address.expect("legacy user decrypt requires userAddress"),
                user_decrypt_request.public_key,
                user_decrypt_request.signature,
                user_decrypt_request.extra_data,
            ));
            Decryption::userDecryptionRequestCall::abi_encode(&call)
        };

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
        let calldata = if let (
            None,
            None,
            Some(delegator_id),
            Some(delegate_id),
            Some(contract_ids),
        ) = (
            delegated_user_decrypt_request.delegator_address,
            delegated_user_decrypt_request.delegate_address,
            delegated_user_decrypt_request.delegator_id,
            delegated_user_decrypt_request.delegate_id,
            delegated_user_decrypt_request.contract_ids.clone(),
        ) {
            let native_pairs = delegated_user_decrypt_request
                .ct_handle_contract_pairs
                .iter()
                .map(|d| NativeCtHandleContractPairCall {
                    ctHandle: d.ct_handle.into(),
                    contractId: d
                        .contract_id
                        .expect("native delegated user decrypt requires contractId"),
                })
                .collect::<Vec<_>>();
            let native_validity = DecryptionNativeRequestValidity {
                startTimestamp: delegated_user_decrypt_request.start_timestamp,
                durationDays: delegated_user_decrypt_request.duration_days,
            };
            let native_delegation_accounts = NativeDelegationAccounts {
                delegatorId: delegator_id,
                delegateId: delegate_id,
            };
            let native_contracts_info = NativeContractsInfo {
                ids: contract_ids,
                chainId: U256::from(delegated_user_decrypt_request.contracts_chain_id),
            };
            let call = DecryptionNative::delegatedUserDecryptionRequestNativeCall::new((
                native_pairs,
                native_validity,
                native_delegation_accounts,
                native_contracts_info,
                delegated_user_decrypt_request.public_key,
                delegated_user_decrypt_request.signature,
                delegated_user_decrypt_request.extra_data,
            ));
            DecryptionNative::delegatedUserDecryptionRequestNativeCall::abi_encode(&call)
        } else {
            let ct_handle_contract_pairs = delegated_user_decrypt_request
                .ct_handle_contract_pairs
                .iter()
                .map(|d| CtHandleContractPair {
                    ctHandle: d.ct_handle.into(),
                    contractAddress: d
                        .contract_address
                        .expect("legacy delegated user decrypt requires contractAddress"),
                })
                .collect::<Vec<_>>();
            let validity = RequestValidity {
                startTimestamp: delegated_user_decrypt_request.start_timestamp,
                durationDays: delegated_user_decrypt_request.duration_days,
            };
            let contracts_info = ContractsInfo {
                addresses: delegated_user_decrypt_request.contract_addresses,
                chainId: U256::from(delegated_user_decrypt_request.contracts_chain_id),
            };
            let delegation_accounts = DelegationAccounts {
                delegatorAddress: delegated_user_decrypt_request
                    .delegator_address
                    .expect("legacy delegated user decrypt requires delegatorAddress"),
                delegateAddress: delegated_user_decrypt_request
                    .delegate_address
                    .expect("legacy delegated user decrypt requires delegateAddress"),
            };
            let call = Decryption::delegatedUserDecryptionRequestCall::new((
                ct_handle_contract_pairs,
                validity,
                delegation_accounts,
                contracts_info,
                delegated_user_decrypt_request.public_key,
                delegated_user_decrypt_request.signature,
                delegated_user_decrypt_request.extra_data,
            ));
            Decryption::delegatedUserDecryptionRequestCall::abi_encode(&call)
        };

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
        let calldata = if let (Some(contract_id), Some(user_id)) =
            (input_proof_request.contract_id, input_proof_request.user_id)
        {
            let request_call = InputVerificationNative::verifyProofRequestNativeCall {
                contractChainId: U256::from(input_proof_request.contract_chain_id),
                contractId: contract_id,
                userId: user_id,
                ciphertextWithZKProof: input_proof_request.ciphetext_with_zk_proof.clone(),
                extraData: input_proof_request.extra_data.clone(),
            };
            request_call.abi_encode()
        } else if let (Some(contract_address), Some(user_address)) = (
            input_proof_request.contract_address,
            input_proof_request.user_address,
        ) {
            let request_call = InputVerification::verifyProofRequestCall {
                contractChainId: U256::from(input_proof_request.contract_chain_id),
                contractAddress: contract_address,
                userAddress: user_address,
                ciphertextWithZKProof: input_proof_request.ciphetext_with_zk_proof.clone(),
                extraData: input_proof_request.extra_data.clone(),
            };
            request_call.abi_encode()
        } else {
            return Err(EventProcessingError::ValidationFailed {
                field: "input_proof_request".to_string(),
                reason: "native requests require contractId/userId and legacy requests require contractAddress/userAddress".to_string(),
            });
        };
        Ok(Bytes::from(calldata))
    }
}
