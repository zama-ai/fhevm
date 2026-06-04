//! On-chain EIP-712 v4 verification of EVM-signed Zama-party certificates,
//! reproduced for Solana with keccak + secp256k1_recover.
//!
//! The KMS (public-decrypt cert) and the coprocessor (input attestation) sign
//! EIP-712 typed data with secp256k1/ECDSA keys, exactly as `KMSVerifier.sol`
//! and `InputVerifier.sol` verify on EVM. We reconstruct the same digest and
//! recover the EVM signer address (the last 20 bytes of keccak(pubkey)) to check
//! it against a configured signer set + threshold. The signatures are the same
//! ones already produced for EVM; only the verification is reproduced here.
//!
//! Domains use the GATEWAY chain id + the gateway verifying-contract address
//! (both configured in `HostConfig`). v0, see zama-ai/fhevm-internal#1494.

use anchor_lang::prelude::*;
use solana_keccak_hasher::hashv as keccak;

const DOMAIN_TYPE: &[u8] =
    b"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";
const PUBLIC_DECRYPT_TYPE: &[u8] =
    b"PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult,bytes extraData)";
// RFC-021 Solana form: EVM uses `address` for user/contract; host-chain addresses
// are widened to bytes32 so Solana 32-byte pubkeys fit.
const CIPHERTEXT_VERIFICATION_TYPE: &[u8] = b"CiphertextVerification(bytes32[] ctHandles,bytes32 userAddress,bytes32 contractAddress,uint256 contractChainId,bytes extraData)";

/// keccak256 over the concatenation of `parts`.
fn k(parts: &[&[u8]]) -> [u8; 32] {
    keccak(parts).to_bytes()
}

/// 32-byte big-endian encoding of a u64 (an EIP-712 uint256 slot).
fn u256(value: u64) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[24..].copy_from_slice(&value.to_be_bytes());
    out
}

/// Left-pad a 20-byte EVM address into a 32-byte EIP-712 word.
fn pad_address(address: &[u8; 20]) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[12..].copy_from_slice(address);
    out
}

/// EIP-712 domain separator.
pub fn domain_separator(
    name: &[u8],
    version: &[u8],
    chain_id: u64,
    verifying_contract: &[u8; 20],
) -> [u8; 32] {
    k(&[
        &k(&[DOMAIN_TYPE]),
        &k(&[name]),
        &k(&[version]),
        &u256(chain_id),
        &pad_address(verifying_contract),
    ])
}

/// Final EIP-712 v4 digest: keccak(0x1901 || domainSeparator || structHash).
pub fn typed_data_digest(domain_separator: &[u8; 32], struct_hash: &[u8; 32]) -> [u8; 32] {
    k(&[&[0x19, 0x01], domain_separator, struct_hash])
}

fn handles_hash(ct_handles: &[[u8; 32]]) -> [u8; 32] {
    let concat: Vec<u8> = ct_handles.iter().flatten().copied().collect();
    k(&[&concat])
}

/// struct hash for the KMS `PublicDecryptVerification` cert.
pub fn public_decrypt_struct_hash(
    ct_handles: &[[u8; 32]],
    decrypted_result: &[u8],
    extra_data: &[u8],
) -> [u8; 32] {
    k(&[
        &k(&[PUBLIC_DECRYPT_TYPE]),
        &handles_hash(ct_handles),
        &k(&[decrypted_result]),
        &k(&[extra_data]),
    ])
}

/// struct hash for the coprocessor `CiphertextVerification` input attestation (RFC-021 bytes32 form).
pub fn ciphertext_verification_struct_hash(
    ct_handles: &[[u8; 32]],
    user_address: &[u8; 32],
    contract_address: &[u8; 32],
    contract_chain_id: u64,
    extra_data: &[u8],
) -> [u8; 32] {
    k(&[
        &k(&[CIPHERTEXT_VERIFICATION_TYPE]),
        &handles_hash(ct_handles),
        user_address,
        contract_address,
        &u256(contract_chain_id),
        &k(&[extra_data]),
    ])
}

/// Recover the EVM signer address from a 65-byte `r||s||v` signature over `digest`.
pub fn recover_evm_address(digest: &[u8; 32], signature: &[u8; 65]) -> Option<[u8; 20]> {
    // EVM EIP-712 signatures use v = 27/28 (recovery id 0/1).
    let recovery_id = signature[64].checked_sub(27)?;
    if recovery_id > 3 {
        return None;
    }
    let pubkey =
        solana_secp256k1_recover::secp256k1_recover(digest, recovery_id, &signature[..64]).ok()?;
    let hash = k(&[&pubkey.to_bytes()]);
    let mut address = [0u8; 20];
    address.copy_from_slice(&hash[12..]);
    Some(address)
}

/// True iff at least `threshold` DISTINCT signatures recover to addresses in `signer_set`.
/// Mirrors the EVM verifiers' recover -> in-set -> unique >= threshold discipline.
pub fn verify_threshold(
    digest: &[u8; 32],
    signatures: &[[u8; 65]],
    signer_set: &[[u8; 20]],
    threshold: u8,
) -> bool {
    if threshold == 0 {
        return false;
    }
    let mut seen: Vec<[u8; 20]> = Vec::new();
    for signature in signatures {
        if let Some(address) = recover_evm_address(digest, signature) {
            if signer_set.contains(&address) && !seen.contains(&address) {
                seen.push(address);
            }
        }
    }
    seen.len() as u8 >= threshold
}

#[cfg(test)]
mod tests {
    use super::*;
    use k256::ecdsa::SigningKey;

    fn evm_address_of(key: &SigningKey) -> [u8; 20] {
        let encoded = key.verifying_key().to_encoded_point(false); // 0x04 || X || Y
        let hash = k(&[&encoded.as_bytes()[1..]]);
        let mut address = [0u8; 20];
        address.copy_from_slice(&hash[12..]);
        address
    }

    fn sign(key: &SigningKey, digest: &[u8; 32]) -> [u8; 65] {
        let (signature, recovery_id) = key.sign_prehash_recoverable(digest).unwrap();
        let mut out = [0u8; 65];
        out[..64].copy_from_slice(&signature.to_bytes());
        out[64] = 27 + recovery_id.to_byte();
        out
    }

    #[test]
    fn recovers_kms_public_decrypt_signer() {
        let key = SigningKey::from_bytes(&[0x11u8; 32].into()).unwrap();
        let signer = evm_address_of(&key);
        let ds = domain_separator(b"Decryption", b"1", 31337, &[0xAAu8; 20]);
        let sh = public_decrypt_struct_hash(&[[7u8; 32], [9u8; 32]], &[1, 2, 3, 4], &[0x00]);
        let digest = typed_data_digest(&ds, &sh);
        let sig = sign(&key, &digest);

        assert_eq!(recover_evm_address(&digest, &sig), Some(signer));
        assert!(verify_threshold(&digest, &[sig], &[signer], 1));
        assert!(!verify_threshold(&digest, &[sig], &[[0xBBu8; 20]], 1)); // signer not in set
        assert!(!verify_threshold(&digest, &[sig], &[signer], 2)); // below threshold
        let mut tampered = digest;
        tampered[0] ^= 1;
        assert_ne!(recover_evm_address(&tampered, &sig), Some(signer)); // wrong digest
    }

    #[test]
    fn recovers_coprocessor_input_signer() {
        let key = SigningKey::from_bytes(&[0x22u8; 32].into()).unwrap();
        let signer = evm_address_of(&key);
        let ds = domain_separator(b"InputVerification", b"1", 31337, &[0xCDu8; 20]);
        let sh = ciphertext_verification_struct_hash(&[[3u8; 32]], &[4u8; 32], &[5u8; 32], 12345, &[0x00]);
        let digest = typed_data_digest(&ds, &sh);
        let sig = sign(&key, &digest);

        assert_eq!(recover_evm_address(&digest, &sig), Some(signer));
        assert!(verify_threshold(&digest, &[sig], &[signer], 1));
    }
}
