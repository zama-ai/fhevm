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

/// secp256k1 group order n divided by 2, big-endian. A signature is malleable
/// (has a valid sibling with `n - s`) unless `s <= n/2`; OpenZeppelin's `ECDSA.recover`,
/// which both EVM verifiers use, rejects the upper half. We mirror that here so a
/// recovered address cannot be produced from a malleated copy of another signature.
const SECP256K1_HALF_ORDER: [u8; 32] = [
    0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0x5d, 0x57, 0x6e, 0x73, 0x57, 0xa4, 0x50, 0x1d, 0xdf, 0xe9, 0x2f, 0x46, 0x68, 0x1b, 0x20, 0xa0,
];

/// Resolve the KMS context id a public-decrypt certificate is bound to, mirroring the EVM
/// gateway `_extractContextId`: empty or version-0 `extra_data` selects the current context;
/// version 1 carries a big-endian context id in `extra_data[1..33]`. Because the KMS signs over
/// `extra_data`, the returned id is authenticated by the certificate, so a cert minted under
/// context N cannot be verified against a different context after a rotation. Returns `None` for
/// an unsupported version or a short version-1 payload.
///
/// The protocol KMS context id is a `uint256` whose high bytes carry a chain-type tag (e.g. the
/// canonical Solana-host context id is `0x07‖..‖<u64>`), while the on-chain `kms_context` is keyed
/// by the low-64-bit id (the bootstrap `define_kms_context` registers that u64). So the low 8 bytes
/// are taken as the context id; the high tag bytes are not part of the on-chain id. This does not
/// weaken context binding: the full `extra_data` is still signed by the KMS, and the resolved id
/// must equal the on-chain `kms_context.context_id`, so a rotated/mismatched context still fails.
pub fn extract_kms_context_id(extra_data: &[u8], current_context_id: u64) -> Option<u64> {
    match extra_data.first() {
        None | Some(0) => Some(current_context_id),
        Some(1) => {
            let id_bytes = extra_data.get(1..33)?;
            // On-chain kms_context is keyed by the low-64-bit id; the high bytes carry the
            // protocol context's chain-type tag (e.g. 0x07 for the Solana host) and are not
            // part of the u64 id.
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&id_bytes[24..32]);
            Some(u64::from_be_bytes(buf))
        }
        Some(_) => None,
    }
}

/// Recover the EVM signer address from a 65-byte `r||s||v` signature over `digest`.
pub fn recover_evm_address(digest: &[u8; 32], signature: &[u8; 65]) -> Option<[u8; 20]> {
    // EVM EIP-712 signatures use v = 27/28 (recovery id 0/1).
    let recovery_id = signature[64].checked_sub(27)?;
    if recovery_id > 3 {
        return None;
    }
    // Reject high-s (malleable) signatures for EVM parity with OpenZeppelin ECDSA.
    if signature[32..64] > SECP256K1_HALF_ORDER[..] {
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

/// v0 verifier config: the EVM EIP-712 domain + signer set a Zama party signs under.
/// Held on-chain (a config PDA) and set by the host admin.
pub struct Eip712VerifierConfig<'a> {
    /// Gateway chain id used in the EIP-712 domain.
    pub gateway_chain_id: u64,
    /// Gateway verifying-contract address (Decryption or InputVerification).
    pub verifying_contract: [u8; 20],
    /// Authorized signer EVM addresses (v0: typically one).
    pub signers: &'a [[u8; 20]],
    /// Minimum distinct valid signatures required.
    pub threshold: u8,
}

/// Verify a KMS `PublicDecryptVerification` certificate (cert-secp).
pub fn verify_kms_public_decrypt(
    config: &Eip712VerifierConfig,
    ct_handles: &[[u8; 32]],
    decrypted_result: &[u8],
    extra_data: &[u8],
    signatures: &[[u8; 65]],
) -> bool {
    let domain = domain_separator(
        b"Decryption",
        b"1",
        config.gateway_chain_id,
        &config.verifying_contract,
    );
    let struct_hash = public_decrypt_struct_hash(ct_handles, decrypted_result, extra_data);
    let digest = typed_data_digest(&domain, &struct_hash);
    verify_threshold(&digest, signatures, config.signers, config.threshold)
}

/// Verify a coprocessor `CiphertextVerification` input attestation (input-bind-secp).
pub fn verify_coprocessor_input(
    config: &Eip712VerifierConfig,
    ct_handles: &[[u8; 32]],
    user_address: &[u8; 32],
    contract_address: &[u8; 32],
    contract_chain_id: u64,
    extra_data: &[u8],
    signatures: &[[u8; 65]],
) -> bool {
    let domain = domain_separator(
        b"InputVerification",
        b"1",
        config.gateway_chain_id,
        &config.verifying_contract,
    );
    let struct_hash = ciphertext_verification_struct_hash(
        ct_handles,
        user_address,
        contract_address,
        contract_chain_id,
        extra_data,
    );
    let digest = typed_data_digest(&domain, &struct_hash);
    verify_threshold(&digest, signatures, config.signers, config.threshold)
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

    /// The malleable sibling of a signature: `(r, n - s, v ^ 1)`. It recovers to the
    /// SAME public key as the original — that is exactly the malleability a low-s rule
    /// exists to prevent. Used to prove `recover_evm_address` rejects high-s inputs.
    fn high_s_sibling(sig: &[u8; 65]) -> [u8; 65] {
        // secp256k1 group order n, big-endian.
        const N: [u8; 32] = [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xfe, 0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b, 0xbf, 0xd2, 0x5e, 0x8c,
            0xd0, 0x36, 0x41, 0x41,
        ];
        let mut out_s = [0u8; 32];
        let mut borrow = 0i16;
        for i in (0..32).rev() {
            let diff = N[i] as i16 - sig[32 + i] as i16 - borrow;
            if diff < 0 {
                out_s[i] = (diff + 256) as u8;
                borrow = 1;
            } else {
                out_s[i] = diff as u8;
                borrow = 0;
            }
        }
        let mut out = *sig;
        out[32..64].copy_from_slice(&out_s);
        out[64] ^= 1; // flip 27 <-> 28
        out
    }

    #[test]
    fn rejects_high_s_malleable_signature() {
        let key = SigningKey::from_bytes(&[0x44u8; 32].into()).expect("valid signing key");
        let signer = evm_address_of(&key);
        let ds = domain_separator(b"Decryption", b"1", 31337, &[0xAAu8; 20]);
        let digest = typed_data_digest(
            &ds,
            &public_decrypt_struct_hash(&[[1u8; 32]], &[7u8; 8], &[0x00]),
        );
        // k256 emits a low-s signature, which must verify.
        let low = sign(&key, &digest);
        assert!(
            low[32..64] <= SECP256K1_HALF_ORDER[..],
            "k256 should emit low-s"
        );
        assert_eq!(recover_evm_address(&digest, &low), Some(signer));

        // The high-s sibling recovers to the same key mathematically, but the malleability
        // guard must reject it outright (matching OpenZeppelin ECDSA / both EVM verifiers).
        let high = high_s_sibling(&low);
        assert!(
            high[32..64] > SECP256K1_HALF_ORDER[..],
            "sibling must be high-s"
        );
        assert_eq!(
            recover_evm_address(&digest, &high),
            None,
            "high-s signature must be rejected"
        );
        // A signer cannot be double-counted toward a threshold by pairing a sig with its sibling.
        assert!(!verify_threshold(&digest, &[low, high], &[signer], 2));
    }

    #[test]
    fn extract_kms_context_id_mirrors_evm_extractcontextid() {
        // Empty or version-0 extra_data -> current context.
        assert_eq!(extract_kms_context_id(&[], 7), Some(7));
        assert_eq!(extract_kms_context_id(&[0u8], 7), Some(7));
        // Version 1 -> low-64-bit context id from bytes [25..33].
        let mut v1 = vec![1u8];
        v1.extend_from_slice(&[0u8; 24]);
        v1.extend_from_slice(&42u64.to_be_bytes());
        assert_eq!(extract_kms_context_id(&v1, 7), Some(42));
        // Version 1 with a chain-type tag in the high bytes (e.g. the Solana-host protocol context
        // id `0x07‖..‖1`) -> the low-64-bit id (1). The high tag bytes are not part of the u64 id.
        let mut tagged = vec![1u8];
        tagged.push(0x07);
        tagged.extend_from_slice(&[0u8; 23]);
        tagged.extend_from_slice(&1u64.to_be_bytes());
        assert_eq!(extract_kms_context_id(&tagged, 7), Some(1));
        // Version 1 with a short payload -> rejected.
        assert_eq!(extract_kms_context_id(&[1u8, 0, 0], 7), None);
        // Unsupported version -> rejected.
        assert_eq!(extract_kms_context_id(&[2u8], 7), None);
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
        let sh = ciphertext_verification_struct_hash(
            &[[3u8; 32]],
            &[4u8; 32],
            &[5u8; 32],
            12345,
            &[0x00],
        );
        let digest = typed_data_digest(&ds, &sh);
        let sig = sign(&key, &digest);

        assert_eq!(recover_evm_address(&digest, &sig), Some(signer));
        assert!(verify_threshold(&digest, &[sig], &[signer], 1));
    }

    #[test]
    fn verifies_full_kms_cert_flow() {
        let key = SigningKey::from_bytes(&[0x33u8; 32].into()).unwrap();
        let config = Eip712VerifierConfig {
            gateway_chain_id: 31337,
            verifying_contract: [0xDEu8; 20],
            signers: &[evm_address_of(&key)],
            threshold: 1,
        };
        let handles = [[0xA1u8; 32]];
        let result = [42u8; 8];
        let ds = domain_separator(b"Decryption", b"1", 31337, &[0xDEu8; 20]);
        let digest =
            typed_data_digest(&ds, &public_decrypt_struct_hash(&handles, &result, &[0x00]));
        let sig = sign(&key, &digest);
        assert!(verify_kms_public_decrypt(
            &config,
            &handles,
            &result,
            &[0x00],
            &[sig]
        ));
        // a different decrypted result yields a different digest -> rejected
        assert!(!verify_kms_public_decrypt(
            &config,
            &handles,
            &[0u8; 8],
            &[0x00],
            &[sig]
        ));
    }

    #[test]
    fn verifies_full_coprocessor_input_flow() {
        let key = SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap();
        let config = Eip712VerifierConfig {
            gateway_chain_id: 31337,
            verifying_contract: [0xCDu8; 20],
            signers: &[evm_address_of(&key)],
            threshold: 1,
        };
        let handles = [[0xB2u8; 32]];
        let (user, contract) = ([0x01u8; 32], [0x02u8; 32]);
        let ds = domain_separator(b"InputVerification", b"1", 31337, &[0xCDu8; 20]);
        let digest = typed_data_digest(
            &ds,
            &ciphertext_verification_struct_hash(&handles, &user, &contract, 12345, &[0x00]),
        );
        let sig = sign(&key, &digest);
        assert!(verify_coprocessor_input(
            &config,
            &handles,
            &user,
            &contract,
            12345,
            &[0x00],
            &[sig]
        ));
        // wrong contract chain id -> rejected
        assert!(!verify_coprocessor_input(
            &config,
            &handles,
            &user,
            &contract,
            999,
            &[0x00],
            &[sig]
        ));
    }

    #[test]
    fn threshold_requires_two_distinct_signers() {
        let k1 = SigningKey::from_bytes(&[0x55u8; 32].into()).unwrap();
        let k2 = SigningKey::from_bytes(&[0x66u8; 32].into()).unwrap();
        let config = Eip712VerifierConfig {
            gateway_chain_id: 31337,
            verifying_contract: [0u8; 20],
            signers: &[evm_address_of(&k1), evm_address_of(&k2)],
            threshold: 2,
        };
        let handles = [[7u8; 32]];
        let ds = domain_separator(b"Decryption", b"1", 31337, &[0u8; 20]);
        let digest = typed_data_digest(&ds, &public_decrypt_struct_hash(&handles, &[1], &[0x00]));
        let s1 = sign(&k1, &digest);
        let s2 = sign(&k2, &digest);
        assert!(!verify_kms_public_decrypt(
            &config,
            &handles,
            &[1],
            &[0x00],
            &[s1]
        )); // 1 < 2
        assert!(!verify_kms_public_decrypt(
            &config,
            &handles,
            &[1],
            &[0x00],
            &[s1, s1]
        )); // dup -> 1 distinct
        assert!(verify_kms_public_decrypt(
            &config,
            &handles,
            &[1],
            &[0x00],
            &[s1, s2]
        )); // 2 distinct
    }
}
