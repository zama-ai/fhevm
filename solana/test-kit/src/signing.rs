//! Fixture signing: the mock coprocessor and KMS key material, secp256k1 EIP-712 signature
//! encoding, and the two certificate shapes the host verifies — `fromExternal` input
//! attestations and `PublicDecryptVerification` decryption certs.
//!
//! Every suite that mints an attestation or a cert does it through here, against the same fixed
//! keys, so a `HostConfig`/`KmsContext` built from [`crate::HostConfigParams`] /
//! [`crate::kms_context_account`] and a signature minted here always agree on the signer set.

use k256::ecdsa::SigningKey;
use solana_sdk::pubkey::Pubkey;
use zama_host as host;

use crate::{u256_be, DECRYPTION_CONTRACT, GATEWAY_CHAIN_ID, INPUT_VERIFICATION_CONTRACT};

/// KMS signing key backing `PublicDecryptVerification` certs; its EVM address is the sole signer of
/// the fixtures' pinned KMS context.
pub fn kms_signing_key() -> SigningKey {
    SigningKey::from_bytes(&[0x55u8; 32].into()).unwrap()
}

/// A distinct KMS signing key derived from a seed byte (for t-of-n KMS-context tests).
pub fn kms_signing_key_n(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32].into()).unwrap()
}

/// Coprocessor signing key backing `fromExternal` attestations; its EVM address is the
/// registered coprocessor signer of the fixtures' host config.
pub fn coprocessor_signing_key() -> SigningKey {
    SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap()
}

/// A distinct coprocessor signing key derived from a seed byte (for n-of-m signer-set tests).
pub fn coprocessor_signing_key_n(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32].into()).unwrap()
}

/// Recovers the EVM address (keccak(pubkey)[12..]) for a signing key.
pub fn secp_evm_address(key: &SigningKey) -> [u8; 20] {
    let encoded = key.verifying_key().to_encoded_point(false);
    let hash = solana_program::keccak::hash(&encoded.as_bytes()[1..]).to_bytes();
    let mut address = [0u8; 20];
    address.copy_from_slice(&hash[12..]);
    address
}

/// 65-byte `[r || s || v]` recoverable signature over an EIP-712 digest.
pub(crate) fn secp_sign(key: &SigningKey, digest: &[u8; 32]) -> [u8; 65] {
    let (signature, recovery_id) = key.sign_prehash_recoverable(digest).unwrap();
    let mut out = [0u8; 65];
    out[..64].copy_from_slice(&signature.to_bytes());
    out[64] = 27 + recovery_id.to_byte();
    out
}

/// Builds a coprocessor-signed `fromExternal` attestation over `amount_handle`, binding it to
/// (`user`, `contract`), signed by the default single coprocessor key. Consumers check
/// `user == transfer authority` and `contract == the consuming program's compute-signer PDA`;
/// the host re-verifies the signature(s) in-execution.
pub fn amount_attestation_for(
    amount_handle: [u8; 32],
    user: Pubkey,
    contract: Pubkey,
) -> host::CoprocessorInputAttestation {
    amount_attestation_signed_by(amount_handle, user, contract, &[coprocessor_signing_key()])
}

/// Like [`amount_attestation_for`], but produces one signature per key in `keys` (n-of-m
/// attestation building). Passing the same key twice yields duplicate signatures over the same
/// digest.
pub fn amount_attestation_signed_by(
    amount_handle: [u8; 32],
    user: Pubkey,
    contract: Pubkey,
    keys: &[SigningKey],
) -> host::CoprocessorInputAttestation {
    attestation_signed_by(
        amount_handle,
        vec![amount_handle],
        0,
        user,
        contract,
        vec![0x00u8],
        keys,
    )
}

/// The general attestation mint behind [`amount_attestation_signed_by`]: caller-chosen covered
/// handle set, `extra_data`, and signer set, with the EIP-712 digest computed over exactly what is
/// carried. Boundary probes use it to build attestations at the host's size caps
/// (`MAX_INPUT_ATTESTATION_HANDLES` covered handles, `MAX_INPUT_ATTESTATION_EXTRA_DATA` bytes of
/// extra data); `ct_handles[handle_index]` must be `input_handle`.
pub fn attestation_signed_by(
    input_handle: [u8; 32],
    ct_handles: Vec<[u8; 32]>,
    handle_index: u8,
    user: Pubkey,
    contract: Pubkey,
    extra_data: Vec<u8>,
    keys: &[SigningKey],
) -> host::CoprocessorInputAttestation {
    assert_eq!(
        ct_handles.get(usize::from(handle_index)),
        Some(&input_handle),
        "the attested input handle must sit at handle_index within ct_handles"
    );
    let contract_chain_id = host::SOLANA_POC_CHAIN_ID;
    let digest = host::eip712::typed_data_digest(
        &host::eip712::domain_separator(
            b"InputVerification",
            b"1",
            GATEWAY_CHAIN_ID,
            &INPUT_VERIFICATION_CONTRACT,
        ),
        &host::eip712::ciphertext_verification_struct_hash(
            &ct_handles,
            &user.to_bytes(),
            &contract.to_bytes(),
            contract_chain_id,
            &extra_data,
        ),
    );
    host::CoprocessorInputAttestation {
        input_handle,
        ct_handles,
        handle_index,
        user_address: user.to_bytes(),
        contract_address: contract.to_bytes(),
        contract_chain_id,
        extra_data,
        signatures: keys.iter().map(|key| secp_sign(key, &digest)).collect(),
    }
}

/// Builds a KMS `PublicDecryptVerification` secp256k1 cert over `handle` / `cleartext` (a 32-byte
/// big-endian `uint256`), signed by [`kms_signing_key`]. `extra_data` binds the KMS context (empty /
/// `[0x00]` selects the current context; a version-1 payload commits an explicit context id).
pub fn kms_public_decrypt_cert(
    handle: [u8; 32],
    cleartext: [u8; 32],
    gateway_chain_id: u64,
    decryption_contract: &[u8; 20],
    extra_data: &[u8],
) -> Vec<[u8; 65]> {
    kms_public_decrypt_cert_signed_by(
        handle,
        cleartext,
        gateway_chain_id,
        decryption_contract,
        extra_data,
        &[kms_signing_key()],
    )
}

/// Like [`kms_public_decrypt_cert`], but produces one signature per key in `keys` — a t-of-n cert.
/// The carried signature payload scales with the threshold t (t x 65 bytes), independent of how many
/// signers are registered in the context.
pub fn kms_public_decrypt_cert_signed_by(
    handle: [u8; 32],
    cleartext: [u8; 32],
    gateway_chain_id: u64,
    decryption_contract: &[u8; 20],
    extra_data: &[u8],
    keys: &[SigningKey],
) -> Vec<[u8; 65]> {
    let digest = host::eip712::typed_data_digest(
        &host::eip712::domain_separator(b"Decryption", b"1", gateway_chain_id, decryption_contract),
        &host::eip712::public_decrypt_struct_hash(&[handle], &cleartext, extra_data),
    );
    keys.iter().map(|key| secp_sign(key, &digest)).collect()
}

/// Version-1 `extra_data` committing an explicit KMS context id in `[1..33]` (EVM `_extractContextId`
/// parity). Used to mint a cert bound to a rotated-out context id.
pub fn context_extra_data_v1(context_id: u64) -> Vec<u8> {
    let mut extra_data = vec![1u8];
    extra_data.extend_from_slice(&[0u8; 24]);
    extra_data.extend_from_slice(&context_id.to_be_bytes());
    extra_data
}

/// A v0 KMS `PublicDecryptVerification` cert over a `u64` amount, bound to the fixtures' gateway
/// and `Decryption` contract, signed by the default KMS key. Returns the `(signatures,
/// extra_data)` pair the verifying instructions take; `extra_data == [0x00]` binds through the
/// current context's signer set.
pub fn amount_public_decrypt_cert(handle: [u8; 32], amount: u64) -> (Vec<[u8; 65]>, Vec<u8>) {
    amount_public_decrypt_cert_signed_by(handle, amount, &[kms_signing_key()])
}

/// Like [`amount_public_decrypt_cert`], but with one signature per key in `keys` (t-of-n cert
/// building — the carried payload scales with the threshold t, not the party count n).
pub fn amount_public_decrypt_cert_signed_by(
    handle: [u8; 32],
    amount: u64,
    keys: &[SigningKey],
) -> (Vec<[u8; 65]>, Vec<u8>) {
    let extra_data = vec![0x00u8];
    let signatures = kms_public_decrypt_cert_signed_by(
        handle,
        u256_be(amount),
        GATEWAY_CHAIN_ID,
        &DECRYPTION_CONTRACT,
        &extra_data,
        keys,
    );
    (signatures, extra_data)
}
