use crate::core::{
    errors::EventProcessingError,
    event::{HandleContractPair, UserDecryptRequest},
};
use crate::gateway::arbitrum::bindings::{Decryption, IDecryption, InputVerification};
use alloy::{
    hex,
    primitives::{Address, Bytes, FixedBytes, B256, U256},
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

    /// Computes calldata for a user decryption request. Picks the gateway
    /// overload based on the attestation format:
    ///   - `LegacyDirect`     → `userDecryptionRequest(CtHandleContractPair[], …)` (`_1Call`)
    ///   - `LegacyDelegated`  → `delegatedUserDecryptionRequest(…)`
    ///   - `Eip712UnifiedV1`  → `userDecryptionRequest(HandleEntry[], …)` (`_0Call`)
    pub fn user_decryption_req(
        user_decrypt_request: UserDecryptRequest,
    ) -> Result<Bytes, EventProcessingError> {
        let kind = user_decrypt_request.attestation_kind();

        let calldata = match user_decrypt_request {
            UserDecryptRequest::LegacyDirect {
                ct_handle_contract_pairs,
                request_validity,
                contracts_chain_id,
                contract_addresses,
                user_address,
                signature,
                public_key,
                extra_data,
            } => {
                let pairs = encode_ct_handle_contract_pairs(&ct_handle_contract_pairs);
                let contracts_info = IDecryption::ContractsInfo {
                    addresses: contract_addresses,
                    chainId: U256::from(contracts_chain_id),
                };
                let validity = IDecryption::RequestValidity {
                    startTimestamp: request_validity.start_timestamp,
                    durationDays: request_validity.duration_days,
                };
                let call = Decryption::userDecryptionRequest_1Call::new((
                    pairs,
                    validity,
                    contracts_info,
                    user_address,
                    public_key,
                    signature,
                    extra_data,
                ));
                Decryption::userDecryptionRequest_1Call::abi_encode(&call)
            }
            UserDecryptRequest::LegacyDelegated {
                ct_handle_contract_pairs,
                request_validity,
                contracts_chain_id,
                contract_addresses,
                delegator_address,
                delegate_address,
                signature,
                public_key,
                extra_data,
            } => {
                let pairs = encode_ct_handle_contract_pairs(&ct_handle_contract_pairs);
                let contracts_info = IDecryption::ContractsInfo {
                    addresses: contract_addresses,
                    chainId: U256::from(contracts_chain_id),
                };
                let validity = IDecryption::RequestValidity {
                    startTimestamp: request_validity.start_timestamp,
                    durationDays: request_validity.duration_days,
                };
                let delegation_accounts = IDecryption::DelegationAccounts {
                    delegatorAddress: delegator_address,
                    delegateAddress: delegate_address,
                };
                let call = Decryption::delegatedUserDecryptionRequestCall::new((
                    pairs,
                    validity,
                    delegation_accounts,
                    contracts_info,
                    public_key,
                    signature,
                    extra_data,
                ));
                Decryption::delegatedUserDecryptionRequestCall::abi_encode(&call)
            }
            UserDecryptRequest::Eip712UnifiedV1 {
                handles,
                user_address,
                allowed_contracts,
                request_validity,
                signature,
                public_key,
                extra_data,
            } => {
                let handle_entries: Vec<Decryption::HandleEntry> = handles
                    .iter()
                    .map(|h| Decryption::HandleEntry {
                        handle: h.ct_handle.into(),
                        contractAddress: h.contract_address,
                        ownerAddress: h.owner_address,
                    })
                    .collect();
                let validity = IDecryption::RequestValiditySeconds {
                    startTimestamp: request_validity.start_timestamp,
                    durationSeconds: request_validity.duration_seconds,
                };
                let call = Decryption::userDecryptionRequest_0Call::new((
                    handle_entries,
                    user_address,
                    public_key,
                    allowed_contracts,
                    validity,
                    signature,
                    extra_data,
                ));
                Decryption::userDecryptionRequest_0Call::abi_encode(&call)
            }
            UserDecryptRequest::SolanaUnifiedV1 {
                handles,
                user_identity,
                allowed_acl_domain_keys,
                request_validity,
                nonce,
                signature,
                public_key,
                extra_data,
                mmr,
            } => {
                let handle_entries: Vec<Decryption::HandleEntry> = handles
                    .iter()
                    .map(|h| Decryption::HandleEntry {
                        handle: h.ct_handle.into(),
                        contractAddress: h.contract_address,
                        ownerAddress: h.owner_address,
                    })
                    .collect();
                let validity = IDecryption::RequestValiditySeconds {
                    startTimestamp: request_validity.start_timestamp,
                    durationSeconds: request_validity.duration_seconds,
                };
                // Encrypted-value-ACL lineage-decrypt info for a confidential-balance / total-supply
                // decrypt. An amount handle (`None`) flattens to the all-zero / empty / zero ABI
                // sentinel the gateway expects. Forwarded verbatim; the KMS Connector verifies the
                // proof against the live on-chain lineage peaks.
                let (acl_value_key, mmr_proof, proof_slot) = match mmr {
                    Some(mmr) => (mmr.value_key, mmr.proof, mmr.proof_slot),
                    None => (B256::ZERO, Bytes::new(), 0),
                };
                // The ed25519 auth fields travel as typed payload fields; `extraData` is
                // context-only. The KMS Connector verifies the ed25519 signature off-chain.
                let payload = IDecryption::UserDecryptionRequestSolanaPayload {
                    userIdentity: user_identity,
                    publicKey: public_key,
                    allowedAclDomainKeys: allowed_acl_domain_keys,
                    requestValidity: validity,
                    nonce,
                    extraData: extra_data,
                    signature,
                    aclValueKey: acl_value_key,
                    mmrProof: mmr_proof,
                    proofSlot: proof_slot,
                };
                let call =
                    Decryption::userDecryptionRequestSolanaCall::new((handle_entries, payload));
                Decryption::userDecryptionRequestSolanaCall::abi_encode(&call)
            }
        };

        info!(
            kind = %kind,
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

fn encode_ct_handle_contract_pairs(
    pairs: &[HandleContractPair],
) -> Vec<Decryption::CtHandleContractPair> {
    pairs
        .iter()
        .map(|d| Decryption::CtHandleContractPair {
            ctHandle: d.ct_handle.into(),
            contractAddress: d.contract_address,
        })
        .collect()
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
