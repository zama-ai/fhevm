use crate::core::{
    errors::EventProcessingError,
    event::{UserDecryptPayload, UserDecryptRequest},
};
use crate::gateway::arbitrum::bindings::{
    Decryption,
    Decryption::{CtHandleContractPair, HandleEntry},
    IDecryption::{ContractsInfo, DelegationAccounts, RequestValidity, RequestValiditySeconds},
    InputVerification,
};
use alloy::{
    hex,
    primitives::{Address, Bytes, FixedBytes, U256},
    sol_types::SolCall,
};
use tracing::info;

pub struct ComputeCalldata;

fn legacy_ct_handle_contract_pairs(req: &UserDecryptRequest) -> Vec<CtHandleContractPair> {
    req.ct_handle_contract_pairs
        .iter()
        .map(|d| CtHandleContractPair {
            ctHandle: d.ct_handle.into(),
            contractAddress: d.contract_address,
        })
        .collect()
}

fn legacy_contracts_info(req: &UserDecryptRequest) -> ContractsInfo {
    ContractsInfo {
        addresses: req.contract_addresses.clone(),
        chainId: U256::from(req.contracts_chain_id),
    }
}

fn legacy_request_validity(req: &UserDecryptRequest) -> RequestValidity {
    RequestValidity {
        startTimestamp: req.request_validity.start_timestamp,
        durationDays: req.request_validity.duration_days,
    }
}

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

    /// Computes calldata for a user decryption request. Picks the gateway
    /// overload based on the payload variant:
    ///   - `LegacyDirect`    → `userDecryptionRequest(CtHandleContractPair[], …)` (`_1Call`)
    ///   - `LegacyDelegated` → `delegatedUserDecryptionRequest(…)`
    ///   - `Unified`         → `userDecryptionRequest(HandleEntry[], …)` (`_0Call`)
    pub fn user_decryption_req(
        user_decrypt_request: UserDecryptRequest,
    ) -> Result<Bytes, EventProcessingError> {
        let calldata = match &user_decrypt_request.payload {
            UserDecryptPayload::LegacyDirect { user_address } => {
                let pairs = legacy_ct_handle_contract_pairs(&user_decrypt_request);
                let contracts_info = legacy_contracts_info(&user_decrypt_request);
                let validity = legacy_request_validity(&user_decrypt_request);
                let call = Decryption::userDecryptionRequest_1Call::new((
                    pairs,
                    validity,
                    contracts_info,
                    *user_address,
                    user_decrypt_request.public_key.clone(),
                    user_decrypt_request.signature.clone(),
                    user_decrypt_request.extra_data.clone(),
                ));
                Decryption::userDecryptionRequest_1Call::abi_encode(&call)
            }
            UserDecryptPayload::LegacyDelegated {
                delegator_address,
                delegate_address,
            } => {
                let pairs = legacy_ct_handle_contract_pairs(&user_decrypt_request);
                let contracts_info = legacy_contracts_info(&user_decrypt_request);
                let validity = legacy_request_validity(&user_decrypt_request);
                let delegation_accounts = DelegationAccounts {
                    delegatorAddress: *delegator_address,
                    delegateAddress: *delegate_address,
                };
                let call = Decryption::delegatedUserDecryptionRequestCall::new((
                    pairs,
                    validity,
                    delegation_accounts,
                    contracts_info,
                    user_decrypt_request.public_key.clone(),
                    user_decrypt_request.signature.clone(),
                    user_decrypt_request.extra_data.clone(),
                ));
                Decryption::delegatedUserDecryptionRequestCall::abi_encode(&call)
            }
            UserDecryptPayload::Unified {
                user_address,
                owner_addresses,
                ..
            } => {
                if owner_addresses.len() != user_decrypt_request.ct_handle_contract_pairs.len() {
                    return Err(EventProcessingError::ValidationFailed {
                        field: "handles".to_string(),
                        reason: format!(
                            "owner_addresses length {} != ct_handle_contract_pairs length {}",
                            owner_addresses.len(),
                            user_decrypt_request.ct_handle_contract_pairs.len()
                        ),
                    });
                }
                let handles: Vec<HandleEntry> = user_decrypt_request
                    .ct_handle_contract_pairs
                    .iter()
                    .zip(owner_addresses.iter())
                    .map(|(p, owner)| HandleEntry {
                        handle: p.ct_handle.into(),
                        contractAddress: p.contract_address,
                        ownerAddress: *owner,
                    })
                    .collect();
                // For the unified path, the top-level `request_validity.duration_days`
                // is reinterpreted as durationSeconds (see UserDecryptPayload::Unified).
                let validity = RequestValiditySeconds {
                    startTimestamp: user_decrypt_request.request_validity.start_timestamp,
                    durationSeconds: user_decrypt_request.request_validity.duration_days,
                };
                let call = Decryption::userDecryptionRequest_0Call::new((
                    handles,
                    *user_address,
                    user_decrypt_request.public_key.clone(),
                    user_decrypt_request.contract_addresses.clone(),
                    validity,
                    user_decrypt_request.signature.clone(),
                    user_decrypt_request.extra_data.clone(),
                ));
                Decryption::userDecryptionRequest_0Call::abi_encode(&call)
            }
        };

        info!(
            kind = %user_decrypt_request.payload_kind(),
            "UserDecryptionRequest calldata: 0x{}",
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
}
