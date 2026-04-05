use crate::error::{HostContractError, Result};
use crate::events::HostEvent;
use crate::types::{
    ContextUserInputs, EvmAddress, Handle, SignatureThreshold, MAX_INPUT_HANDLES_PER_PROOF,
    MAX_INPUT_PROOF_BYTES, MAX_VERIFIER_SIGNERS,
};
use borsh::{BorshDeserialize, BorshSerialize};
use sha3::{Digest, Keccak256};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiphertextVerification {
    pub ct_handles: Vec<Handle>,
    pub user_address: EvmAddress,
    pub contract_address: EvmAddress,
    pub contract_chain_id: u64,
    pub extra_data: Vec<u8>,
}

pub trait InputProofVerifier {
    fn verify(
        &self,
        payload: &CiphertextVerification,
        signatures: &[Vec<u8>],
        signers: &[EvmAddress],
        threshold: SignatureThreshold,
        source_chain_id: u64,
        source_contract: EvmAddress,
    ) -> Result<()>;
}

#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InputVerifierSession {
    proof_cache: HashSet<[u8; 32]>,
}

impl InputVerifierSession {
    pub fn contains(&self, cache_key: &[u8; 32]) -> bool {
        self.proof_cache.contains(cache_key)
    }

    pub fn insert(&mut self, cache_key: [u8; 32]) {
        self.proof_cache.insert(cache_key);
    }

    pub fn clear(&mut self) {
        self.proof_cache.clear();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InputVerifierState {
    source_contract: EvmAddress,
    source_chain_id: u64,
    signers: Vec<EvmAddress>,
    threshold: SignatureThreshold,
}

impl InputVerifierState {
    pub fn new(
        source_contract: EvmAddress,
        source_chain_id: u64,
        initial_signers: Vec<EvmAddress>,
        initial_threshold: SignatureThreshold,
    ) -> Result<Self> {
        let mut state = Self {
            source_contract,
            source_chain_id,
            signers: Vec::new(),
            threshold: 0,
        };
        state.define_new_context(&initial_signers, initial_threshold)?;
        Ok(state)
    }

    pub fn define_new_context<S>(
        &mut self,
        new_signers: S,
        new_threshold: SignatureThreshold,
    ) -> Result<HostEvent>
    where
        S: AsRef<[EvmAddress]>,
    {
        let new_signers = new_signers.as_ref();
        if new_signers.is_empty() {
            return Err(HostContractError::SignersSetIsEmpty);
        }
        if new_signers.len() > MAX_VERIFIER_SIGNERS {
            return Err(HostContractError::TooManySigners {
                max: MAX_VERIFIER_SIGNERS,
            });
        }
        let mut seen = HashSet::new();
        for signer in new_signers {
            if *signer == EvmAddress::ZERO {
                return Err(HostContractError::SignerNull);
            }
            if !seen.insert(*signer) {
                return Err(HostContractError::SignerAlreadyRegistered);
            }
        }
        if new_threshold == 0 {
            return Err(HostContractError::ThresholdIsNull);
        }
        if new_threshold as usize > new_signers.len() {
            return Err(HostContractError::ThresholdIsAboveNumberOfSigners);
        }
        self.signers = new_signers.to_vec();
        self.threshold = new_threshold;
        Ok(HostEvent::InputVerifierContextUpdated {
            signers: new_signers.to_vec(),
            threshold: new_threshold,
        })
    }

    pub fn verify_input<V: InputProofVerifier>(
        &mut self,
        context: ContextUserInputs,
        input_handle: Handle,
        input_proof: &[u8],
        session: &mut InputVerifierSession,
        proof_verifier: &V,
        host_chain_id: u64,
    ) -> Result<Handle> {
        if input_proof.is_empty() {
            return Err(HostContractError::EmptyInputProof);
        }
        if input_proof.len() > MAX_INPUT_PROOF_BYTES {
            return Err(HostContractError::InputProofTooLarge {
                max: MAX_INPUT_PROOF_BYTES,
            });
        }

        let recovered_chain_id = input_handle.chain_id();
        if recovered_chain_id != host_chain_id {
            return Err(HostContractError::InvalidChainId {
                expected: host_chain_id,
                found: recovered_chain_id,
            });
        }

        let index_handle = input_handle.index();
        let cache_key = proof_cache_key(input_proof, context);

        if input_proof.len() < 2 {
            return Err(HostContractError::DeserializingInputProofFail);
        }

        let num_handles = input_proof[0] as usize;
        let num_signers = input_proof[1] as usize;
        if num_handles > MAX_INPUT_HANDLES_PER_PROOF {
            return Err(HostContractError::TooManyHandlesInProof {
                max: MAX_INPUT_HANDLES_PER_PROOF,
            });
        }
        if num_signers > MAX_VERIFIER_SIGNERS {
            return Err(HostContractError::TooManySigners {
                max: MAX_VERIFIER_SIGNERS,
            });
        }
        if index_handle as usize >= num_handles || index_handle > 254 {
            return Err(HostContractError::InvalidIndex);
        }

        let extra_data_offset = 2 + (32 * num_handles) + (65 * num_signers);
        if input_proof.len() < extra_data_offset {
            return Err(HostContractError::DeserializingInputProofFail);
        }

        let mut handles = Vec::with_capacity(num_handles);
        let mut offset = 2;
        for _ in 0..num_handles {
            let mut raw_handle = [0_u8; 32];
            raw_handle.copy_from_slice(&input_proof[offset..offset + 32]);
            let handle = Handle::new(raw_handle);
            if handle.version() != crate::types::HANDLE_VERSION {
                return Err(HostContractError::InvalidHandleVersion {
                    expected: crate::types::HANDLE_VERSION,
                    found: handle.version(),
                });
            }
            handles.push(handle);
            offset += 32;
        }

        let mut signatures = Vec::with_capacity(num_signers);
        for _ in 0..num_signers {
            signatures.push(input_proof[offset..offset + 65].to_vec());
            offset += 65;
        }

        let payload = CiphertextVerification {
            ct_handles: handles.clone(),
            user_address: context.user_address,
            contract_address: context.contract_address,
            contract_chain_id: host_chain_id,
            extra_data: input_proof[extra_data_offset..].to_vec(),
        };

        if !session.contains(&cache_key) {
            proof_verifier.verify(
                &payload,
                &signatures,
                &self.signers,
                self.threshold,
                self.source_chain_id,
                self.source_contract,
            )?;
            session.insert(cache_key);
        }

        let expected_handle = handles[index_handle as usize];
        if expected_handle != input_handle {
            return Err(HostContractError::InvalidInputHandle);
        }
        Ok(expected_handle)
    }

    pub fn get_coprocessor_signers(&self) -> &[EvmAddress] {
        &self.signers
    }

    pub fn is_signer(&self, signer: EvmAddress) -> bool {
        self.signers.contains(&signer)
    }

    pub fn get_threshold(&self) -> SignatureThreshold {
        self.threshold
    }
}

fn proof_cache_key(input_proof: &[u8], context: ContextUserInputs) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(input_proof);
    hasher.update(context.user_address.as_bytes());
    hasher.update(context.contract_address.as_bytes());
    hasher.finalize().into()
}

