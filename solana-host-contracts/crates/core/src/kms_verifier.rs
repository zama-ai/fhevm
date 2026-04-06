use crate::error::{HostContractError, Result};
use crate::events::HostEvent;
use crate::types::{
    EvmAddress, Handle, KmsContextId, SignatureThreshold, MAX_DECRYPTED_RESULT_BYTES,
    MAX_DECRYPTION_HANDLES, MAX_DECRYPTION_PROOF_BYTES, MAX_VERIFIER_SIGNERS,
};
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicDecryptVerification {
    pub ct_handles: Vec<Handle>,
    pub decrypted_result: Vec<u8>,
    pub extra_data: Vec<u8>,
}

pub trait KmsProofVerifier {
    fn verify(
        &self,
        payload: &PublicDecryptVerification,
        signatures: &[Vec<u8>],
        signers: &[EvmAddress],
        threshold: SignatureThreshold,
        source_chain_id: u64,
        source_contract: EvmAddress,
    ) -> Result<()>;
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct KmsContext {
    pub signers: Vec<EvmAddress>,
    pub threshold: SignatureThreshold,
    pub destroyed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct KmsVerifierState {
    source_contract: EvmAddress,
    source_chain_id: u64,
    current_kms_context_id: KmsContextId,
    contexts: HashMap<KmsContextId, KmsContext>,
}

impl KmsVerifierState {
    pub fn new(
        source_contract: EvmAddress,
        source_chain_id: u64,
        initial_signers: Vec<EvmAddress>,
        initial_threshold: SignatureThreshold,
    ) -> Result<Self> {
        let mut state = Self {
            source_contract,
            source_chain_id,
            current_kms_context_id: KmsContextId::base(),
            contexts: HashMap::new(),
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
                return Err(HostContractError::KmsAlreadySigner);
            }
        }
        if new_threshold == 0 {
            return Err(HostContractError::ThresholdIsNull);
        }
        if new_threshold as usize > new_signers.len() {
            return Err(HostContractError::ThresholdIsAboveNumberOfSigners);
        }

        self.current_kms_context_id = self.current_kms_context_id.next();
        self.contexts.insert(
            self.current_kms_context_id,
            KmsContext {
                signers: new_signers.to_vec(),
                threshold: new_threshold,
                destroyed: false,
            },
        );

        Ok(HostEvent::KmsContextUpdated {
            kms_context_id: self.current_kms_context_id,
            signers: new_signers.to_vec(),
            threshold: new_threshold,
        })
    }

    pub fn destroy_kms_context(&mut self, kms_context_id: KmsContextId) -> Result<HostEvent> {
        if kms_context_id == self.current_kms_context_id {
            return Err(HostContractError::CurrentKmsContextCannotBeDestroyed);
        }
        let context = self
            .contexts
            .get_mut(&kms_context_id)
            .ok_or(HostContractError::InvalidKmsContext(kms_context_id))?;
        if context.destroyed {
            return Err(HostContractError::InvalidKmsContext(kms_context_id));
        }
        context.destroyed = true;
        Ok(HostEvent::KmsContextDestroyed { kms_context_id })
    }

    pub fn verify_decryption_signatures<H, D, V>(
        &self,
        handles_list: H,
        decrypted_result: D,
        decryption_proof: &[u8],
        proof_verifier: &V,
    ) -> Result<bool>
    where
        H: AsRef<[Handle]>,
        D: AsRef<[u8]>,
        V: KmsProofVerifier,
    {
        let handles_list = handles_list.as_ref();
        let decrypted_result = decrypted_result.as_ref();
        if decryption_proof.is_empty() {
            return Err(HostContractError::EmptyDecryptionProof);
        }
        if decryption_proof.len() > MAX_DECRYPTION_PROOF_BYTES {
            return Err(HostContractError::DecryptionProofTooLarge {
                max: MAX_DECRYPTION_PROOF_BYTES,
            });
        }
        if handles_list.len() > MAX_DECRYPTION_HANDLES {
            return Err(HostContractError::TooManyDecryptionHandles {
                max: MAX_DECRYPTION_HANDLES,
            });
        }
        if decrypted_result.len() > MAX_DECRYPTED_RESULT_BYTES {
            return Err(HostContractError::DecryptedResultTooLarge {
                max: MAX_DECRYPTED_RESULT_BYTES,
            });
        }

        let num_signatures = decryption_proof[0] as usize;
        if num_signatures > MAX_VERIFIER_SIGNERS {
            return Err(HostContractError::TooManySigners {
                max: MAX_VERIFIER_SIGNERS,
            });
        }
        let extra_data_offset = 1 + (65 * num_signatures);
        if decryption_proof.len() < extra_data_offset {
            return Err(HostContractError::DeserializingDecryptionProofFail);
        }

        let mut signatures = Vec::with_capacity(num_signatures);
        let mut offset = 1;
        for _ in 0..num_signatures {
            signatures.push(decryption_proof[offset..offset + 65].to_vec());
            offset += 65;
        }

        let extra_data = decryption_proof[extra_data_offset..].to_vec();
        let context_id = self.extract_kms_context_id(&extra_data)?;
        let context = self
            .contexts
            .get(&context_id)
            .filter(|context| !context.destroyed)
            .ok_or(HostContractError::InvalidKmsContext(context_id))?;

        if signatures.len() < context.threshold as usize {
            return Err(HostContractError::SignatureThresholdNotReached {
                got: signatures.len(),
                needed: context.threshold,
            });
        }

        let payload = PublicDecryptVerification {
            ct_handles: handles_list.to_vec(),
            decrypted_result: decrypted_result.to_vec(),
            extra_data,
        };
        proof_verifier.verify(
            &payload,
            &signatures,
            &context.signers,
            context.threshold,
            self.source_chain_id,
            self.source_contract,
        )?;
        Ok(true)
    }

    pub fn get_current_kms_context_id(&self) -> KmsContextId {
        self.current_kms_context_id
    }

    pub fn get_kms_signers(&self) -> Vec<EvmAddress> {
        self.get_signers_for_kms_context(self.current_kms_context_id)
    }

    pub fn is_signer(&self, signer: EvmAddress) -> bool {
        self.contexts
            .get(&self.current_kms_context_id)
            .filter(|context| !context.destroyed)
            .map(|context| context.signers.contains(&signer))
            .unwrap_or(false)
    }

    pub fn get_threshold(&self) -> SignatureThreshold {
        self.contexts
            .get(&self.current_kms_context_id)
            .filter(|context| !context.destroyed)
            .map(|context| context.threshold)
            .unwrap_or_default()
    }

    pub fn get_signers_for_kms_context(&self, kms_context_id: KmsContextId) -> Vec<EvmAddress> {
        self.contexts
            .get(&kms_context_id)
            .filter(|context| !context.destroyed)
            .map(|context| context.signers.clone())
            .unwrap_or_default()
    }

    pub fn get_context_signers_and_threshold_from_extra_data(
        &self,
        extra_data: &[u8],
    ) -> Result<(Vec<EvmAddress>, SignatureThreshold)> {
        let context_id = self.extract_kms_context_id(extra_data)?;
        let context = self
            .contexts
            .get(&context_id)
            .filter(|context| !context.destroyed)
            .ok_or(HostContractError::InvalidKmsContext(context_id))?;
        Ok((context.signers.clone(), context.threshold))
    }

    fn extract_kms_context_id(&self, extra_data: &[u8]) -> Result<KmsContextId> {
        if extra_data.is_empty() || extra_data[0] == 0x00 {
            return Ok(self.current_kms_context_id);
        }
        match extra_data[0] {
            0x01 => {
                if extra_data.len() < 33 {
                    return Err(HostContractError::DeserializingExtraDataFail);
                }
                let mut bytes = [0_u8; 32];
                bytes.copy_from_slice(&extra_data[1..33]);
                Ok(KmsContextId::from_bytes(bytes))
            }
            version => Err(HostContractError::UnsupportedExtraDataVersion(version)),
        }
    }
}
