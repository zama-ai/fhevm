use crate::error::{HostContractError, Result};
use crate::input_verifier::{CiphertextVerification, InputProofVerifier};
use crate::kms_verifier::{KmsProofVerifier, PublicDecryptVerification};
use crate::types::{EvmAddress, SignatureThreshold};
use sha3::{Digest, Keccak256};
use solana_program::secp256k1_recover::secp256k1_recover;
use std::collections::HashSet;

const EIP712_DOMAIN_TYPE: &str =
    "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";
const INPUT_NAME: &str = "InputVerification";
const INPUT_VERSION: &str = "1";
const INPUT_TYPE: &str =
    "CiphertextVerification(bytes32[] ctHandles,address userAddress,address contractAddress,uint256 contractChainId,bytes extraData)";
const INPUT_TYPE_V1: &str =
    "CiphertextVerificationV1(bytes32[] ctHandles,bytes32 contractId,bytes32 userId,uint256 contractChainId,bytes extraData)";
const KMS_NAME: &str = "Decryption";
const KMS_VERSION: &str = "1";
const KMS_TYPE: &str =
    "PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult,bytes extraData)";
const INPUT_PROOF_IDENTITIES_VERSION_1: u8 = 0x01;
const INPUT_PROOF_IDENTITIES_V1_EXTRA_DATA_LENGTH: usize = 65;
const SECP256K1_HALF_ORDER: [u8; 32] = [
    0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0x5d, 0x57, 0x6e, 0x73, 0x57, 0xa4, 0x50, 0x1d, 0xdf, 0xe9, 0x2f, 0x46, 0x68, 0x1b, 0x20, 0xa0,
];

#[derive(Clone, Copy, Debug, Default)]
pub struct Secp256k1ProofVerifier;

impl Secp256k1ProofVerifier {
    pub fn input_verification_message(
        payload: &CiphertextVerification,
        source_chain_id: u64,
        source_contract: EvmAddress,
    ) -> Vec<u8> {
        typed_data_message(
            eip712_domain_separator(INPUT_NAME, INPUT_VERSION, source_chain_id, source_contract),
            input_struct_hash(payload),
        )
    }

    pub fn decryption_message(
        payload: &PublicDecryptVerification,
        source_chain_id: u64,
        source_contract: EvmAddress,
    ) -> Vec<u8> {
        typed_data_message(
            eip712_domain_separator(KMS_NAME, KMS_VERSION, source_chain_id, source_contract),
            decryption_struct_hash(payload),
        )
    }
}

impl InputProofVerifier for Secp256k1ProofVerifier {
    fn verify(
        &self,
        payload: &CiphertextVerification,
        signatures: &[Vec<u8>],
        signers: &[EvmAddress],
        threshold: SignatureThreshold,
        source_chain_id: u64,
        source_contract: EvmAddress,
    ) -> Result<()> {
        let digest = keccak(Self::input_verification_message(
            payload,
            source_chain_id,
            source_contract,
        ));
        verify_signatures(digest, signatures, signers, threshold, false)
    }
}

impl KmsProofVerifier for Secp256k1ProofVerifier {
    fn verify(
        &self,
        payload: &PublicDecryptVerification,
        signatures: &[Vec<u8>],
        signers: &[EvmAddress],
        threshold: SignatureThreshold,
        source_chain_id: u64,
        source_contract: EvmAddress,
    ) -> Result<()> {
        let digest = keccak(Self::decryption_message(
            payload,
            source_chain_id,
            source_contract,
        ));
        verify_signatures(digest, signatures, signers, threshold, true)
    }
}

fn verify_signatures(
    digest: [u8; 32],
    signatures: &[Vec<u8>],
    signers: &[EvmAddress],
    threshold: SignatureThreshold,
    is_kms: bool,
) -> Result<()> {
    if signatures.len() < threshold as usize {
        return Err(HostContractError::SignatureThresholdNotReached {
            got: signatures.len(),
            needed: threshold,
        });
    }

    let allowed_signers: HashSet<EvmAddress> = signers.iter().copied().collect();
    let mut unique_valid_signers = HashSet::new();

    for signature in signatures {
        let signer =
            recover_signer(&digest, signature).map_err(|_| invalid_signer_error(is_kms))?;
        if !allowed_signers.contains(&signer) {
            return Err(invalid_signer_error(is_kms));
        }
        unique_valid_signers.insert(signer);
        if unique_valid_signers.len() >= threshold as usize {
            return Ok(());
        }
    }

    Err(HostContractError::SignatureThresholdNotReached {
        got: unique_valid_signers.len(),
        needed: threshold,
    })
}

fn recover_signer(digest: &[u8; 32], signature: &[u8]) -> std::result::Result<EvmAddress, ()> {
    if signature.len() != 65 {
        return Err(());
    }

    let recovery_id = normalize_recovery_id(signature[64]).ok_or(())?;
    let signature_bytes: [u8; 64] = signature[..64].try_into().map_err(|_| ())?;
    let s_bytes: [u8; 32] = signature_bytes[32..64].try_into().map_err(|_| ())?;
    if !is_low_s(&s_bytes) {
        return Err(());
    }

    let recovered = secp256k1_recover(digest, recovery_id, &signature_bytes).map_err(|_| ())?;
    Ok(ethereum_address_from_pubkey(recovered.to_bytes()))
}

fn normalize_recovery_id(recovery_id: u8) -> Option<u8> {
    match recovery_id {
        0 | 1 => Some(recovery_id),
        27 | 28 => Some(recovery_id - 27),
        _ => None,
    }
}

fn is_low_s(s: &[u8; 32]) -> bool {
    s <= &SECP256K1_HALF_ORDER
}

fn ethereum_address_from_pubkey(pubkey: [u8; 64]) -> EvmAddress {
    let hash = keccak(pubkey);
    let mut address = [0_u8; 20];
    address.copy_from_slice(&hash[12..]);
    EvmAddress::new(address)
}

fn input_struct_hash(payload: &CiphertextVerification) -> [u8; 32] {
    if let Some((contract_id, user_id)) = parse_versioned_identities(&payload.extra_data) {
        let mut encoded = Vec::with_capacity(32 * 6);
        encoded.extend_from_slice(&keccak(INPUT_TYPE_V1.as_bytes()));
        encoded.extend_from_slice(&handles_hash(&payload.ct_handles));
        encoded.extend_from_slice(&contract_id);
        encoded.extend_from_slice(&user_id);
        encoded.extend_from_slice(&u256_word_from_u64(payload.contract_chain_id));
        encoded.extend_from_slice(&keccak(&payload.extra_data));
        return keccak(encoded);
    }

    let mut encoded = Vec::with_capacity(32 * 6);
    encoded.extend_from_slice(&keccak(INPUT_TYPE.as_bytes()));
    encoded.extend_from_slice(&handles_hash(&payload.ct_handles));
    encoded.extend_from_slice(&address_word(payload.user_address));
    encoded.extend_from_slice(&address_word(payload.contract_address));
    encoded.extend_from_slice(&u256_word_from_u64(payload.contract_chain_id));
    encoded.extend_from_slice(&keccak(&payload.extra_data));
    keccak(encoded)
}

fn decryption_struct_hash(payload: &PublicDecryptVerification) -> [u8; 32] {
    let mut encoded = Vec::with_capacity(32 * 4);
    encoded.extend_from_slice(&keccak(KMS_TYPE.as_bytes()));
    encoded.extend_from_slice(&handles_hash(&payload.ct_handles));
    encoded.extend_from_slice(&keccak(&payload.decrypted_result));
    encoded.extend_from_slice(&keccak(&payload.extra_data));
    keccak(encoded)
}

fn eip712_domain_separator(
    name: &str,
    version: &str,
    source_chain_id: u64,
    source_contract: EvmAddress,
) -> [u8; 32] {
    let mut encoded = Vec::with_capacity(32 * 5);
    encoded.extend_from_slice(&keccak(EIP712_DOMAIN_TYPE.as_bytes()));
    encoded.extend_from_slice(&keccak(name.as_bytes()));
    encoded.extend_from_slice(&keccak(version.as_bytes()));
    encoded.extend_from_slice(&u256_word_from_u64(source_chain_id));
    encoded.extend_from_slice(&address_word(source_contract));
    keccak(encoded)
}

fn typed_data_message(domain_separator: [u8; 32], struct_hash: [u8; 32]) -> Vec<u8> {
    let mut message = Vec::with_capacity(66);
    message.extend_from_slice(b"\x19\x01");
    message.extend_from_slice(&domain_separator);
    message.extend_from_slice(&struct_hash);
    message
}

fn handles_hash(handles: &[crate::Handle]) -> [u8; 32] {
    let mut packed = Vec::with_capacity(handles.len() * 32);
    for handle in handles {
        packed.extend_from_slice(handle.as_bytes());
    }
    keccak(packed)
}

fn address_word(address: EvmAddress) -> [u8; 32] {
    let mut word = [0_u8; 32];
    word[12..].copy_from_slice(address.as_bytes());
    word
}

fn u256_word_from_u64(value: u64) -> [u8; 32] {
    let mut word = [0_u8; 32];
    word[24..].copy_from_slice(&value.to_be_bytes());
    word
}

fn parse_versioned_identities(extra_data: &[u8]) -> Option<([u8; 32], [u8; 32])> {
    if extra_data.len() != INPUT_PROOF_IDENTITIES_V1_EXTRA_DATA_LENGTH {
        return None;
    }
    if extra_data.first().copied() != Some(INPUT_PROOF_IDENTITIES_VERSION_1) {
        return None;
    }

    let contract_id = extra_data.get(1..33)?.try_into().ok()?;
    let user_id = extra_data.get(33..65)?.try_into().ok()?;
    Some((contract_id, user_id))
}

fn invalid_signer_error(is_kms: bool) -> HostContractError {
    if is_kms {
        HostContractError::InvalidKmsSigner
    } else {
        HostContractError::InvalidSigner
    }
}

fn keccak(data: impl AsRef<[u8]>) -> [u8; 32] {
    Keccak256::digest(data.as_ref()).into()
}
