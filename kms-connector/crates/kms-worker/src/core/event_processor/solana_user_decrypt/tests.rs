//! Signature-path unit tests: `verify_solana_user_decrypt_signature` (positive + the
//! substitution / forgery / chain-binding / domain-scope negatives) and amount-handle ACL
//! authorization. Extracted from `solana_user_decrypt.rs`; `use super::*` reaches the parent
//! module (including the shared `pkcs8_from_seed`).

use super::*;
use crate::core::solana_acl::{ACL_ROLE_USE, SubjectRole, acl_nonce_key, acl_record_address};
use alloy::primitives::{Address, Bytes, FixedBytes};
use fhevm_gateway_bindings::decryption::{
    Decryption::{HandleEntry, SnsCiphertextMaterial, UserDecryptionRequestSolana},
    IDecryption::{RequestValiditySeconds, UserDecryptionRequestSolanaPayload},
};
use ring::signature::{Ed25519KeyPair, KeyPair};

const CHAIN_ID: u64 = 7777;
const HOST_PROGRAM_ID: SolanaPubkeyBytes = [42u8; 32];
const APP_ACCOUNT: SolanaPubkeyBytes = [2u8; 32];
const LABEL: [u8; 32] = *b"balance_________________________";
const DOMAIN: SolanaPubkeyBytes = [1u8; 32];

/// Wraps a raw ed25519 seed in a minimal PKCS#8 v1 document (the form ring's
/// `from_pkcs8_maybe_unchecked` accepts for Ed25519, public key omitted).
fn identity_keypair() -> Ed25519KeyPair {
    let seed = [11u8; 32];
    let pkcs8 = pkcs8_from_seed(&seed);
    Ed25519KeyPair::from_pkcs8_maybe_unchecked(&pkcs8).unwrap()
}

fn handle_with_chain(byte: u8) -> [u8; 32] {
    let mut h = [byte; 32];
    h[22..30].copy_from_slice(&CHAIN_ID.to_be_bytes());
    h
}

/// Builds a typed Solana request signed by `identity_keypair()` over the canonical preimage.
fn signed_request(
    public_key: Vec<u8>,
    allowed_acl_domain_keys: Vec<SolanaPubkeyBytes>,
) -> (UserDecryptionRequestSolana, SolanaPubkeyBytes) {
    let kp = identity_keypair();
    let identity: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
    let nonce = [5u8; 32];
    let context_id = [0u8; 32];
    let handle = handle_with_chain(7);
    let start: u64 = 1_000;
    let duration: u64 = 3_600;

    let preimage = solana_user_decrypt_signing_preimage(&SolanaUserDecryptSigningInput {
        contracts_chain_id: CHAIN_ID,
        public_key: &public_key,
        handles: &[handle],
        identity: &identity,
        context_id: &context_id,
        nonce: &nonce,
        allowed_acl_domain_keys: &allowed_acl_domain_keys,
        start_timestamp: start,
        duration_seconds: duration,
        acl_value_key: &[0u8; 32],
        mmr_proof_bytes: &[],
        proof_slot: 0,
    });
    let signature = kp.sign(&preimage);

    // extraData is context-only (v0x01: version ‖ contextId) — no auth blob.
    let mut extra_data = vec![0x01u8];
    extra_data.extend_from_slice(&context_id);

    let payload = UserDecryptionRequestSolanaPayload {
        userIdentity: FixedBytes::from(identity),
        publicKey: Bytes::from(public_key),
        allowedAclDomainKeys: allowed_acl_domain_keys
            .iter()
            .map(|k| FixedBytes::from(*k))
            .collect(),
        requestValidity: RequestValiditySeconds {
            startTimestamp: U256::from(start),
            durationSeconds: U256::from(duration),
        },
        nonce: FixedBytes::from(nonce),
        extraData: Bytes::from(extra_data),
        signature: Bytes::from(signature.as_ref().to_vec()),
        aclValueKey: FixedBytes::ZERO,
        mmrProof: Bytes::new(),
        proofSlot: 0,
    };
    let request = UserDecryptionRequestSolana {
        decryptionId: U256::from(1u64),
        snsCtMaterials: vec![SnsCiphertextMaterial {
            ctHandle: FixedBytes::from(handle),
            ..Default::default()
        }],
        handles: vec![HandleEntry {
            handle: FixedBytes::from(handle),
            contractAddress: Address::ZERO,
            ownerAddress: Address::ZERO,
        }],
        payload,
    };
    (request, identity)
}

#[test]
fn accepts_valid_signature() {
    let (request, _identity) = signed_request(b"reencryption-public-key".to_vec(), vec![DOMAIN]);
    let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();
    assert_eq!(auth.allowed_acl_domain_keys, vec![DOMAIN]);
}

// (a) publicKey substitution: a request whose publicKey differs from the key the signature
// committed to is REJECTED.
#[test]
fn rejects_public_key_substitution() {
    let (mut request, _identity) =
        signed_request(b"reencryption-public-key".to_vec(), vec![DOMAIN]);
    // Swap in an attacker-controlled re-encryption key, keeping the user's signature.
    request.payload.publicKey = Bytes::from_static(b"attacker-public-key");

    let result = verify_solana_user_decrypt_signature(&request, CHAIN_ID);
    assert!(
        matches!(result, Err(ProcessingError::Irrecoverable(_))),
        "publicKey substitution must be rejected, got {result:?}",
    );
}

// (b) forged / relayer-bypass signature: a signature not from the claimed identity is REJECTED.
#[test]
fn rejects_forged_signature() {
    let (mut request, _identity) =
        signed_request(b"reencryption-public-key".to_vec(), vec![DOMAIN]);
    // Forge a signature with a *different* key over the same preimage — i.e. a relayer that
    // never held the user's identity key.
    let attacker_seed = [99u8; 32];
    let attacker =
        Ed25519KeyPair::from_pkcs8_maybe_unchecked(&pkcs8_from_seed(&attacker_seed)).unwrap();
    // Re-sign whatever preimage the connector will build (publicKey unchanged), but with the
    // attacker key. The identity in extraData still names the victim.
    let forged = attacker.sign(b"any-bytes");
    request.payload.signature = Bytes::from(forged.as_ref().to_vec());

    let result = verify_solana_user_decrypt_signature(&request, CHAIN_ID);
    assert!(
        matches!(result, Err(ProcessingError::Irrecoverable(_))),
        "forged signature must be rejected, got {result:?}",
    );
}

#[test]
fn rejects_wrong_chain_id_binding() {
    let (request, _identity) = signed_request(b"reencryption-public-key".to_vec(), vec![DOMAIN]);
    // Verifier uses a different contracts_chain_id than the signer committed to.
    let result = verify_solana_user_decrypt_signature(&request, CHAIN_ID + 1);
    assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))));
}

fn record_for(
    handle: [u8; 32],
    domain: SolanaPubkeyBytes,
    subject: SolanaPubkeyBytes,
) -> AclRecordWitness {
    let nonce_key = acl_nonce_key(domain, APP_ACCOUNT, LABEL);
    let (account_key, bump) = acl_record_address(HOST_PROGRAM_ID, nonce_key, 3);
    AclRecordWitness {
        account_key,
        owner: HOST_PROGRAM_ID,
        handle,
        nonce_key,
        nonce_sequence: 3,
        acl_domain_key: domain,
        app_account: APP_ACCOUNT,
        encrypted_value_label: LABEL,
        subjects: vec![SubjectRole {
            subject,
            role_flags: ACL_ROLE_USE,
        }],
        overflow_subject_count: 0,
        public_decrypt: false,
        material_commitment: [0u8; 32],
        material_commitment_hash: [0u8; 32],
        material_key_id: [0u8; 32],
        created_slot: 1,
        bump,
    }
}

// (d) domain scoping: a subject holding USE on a handle whose acl_domain_key is NOT in the
// request's allowedContracts scope is REJECTED (when scope is non-empty).
#[test]
fn rejects_out_of_scope_domain() {
    let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
    let subject = [77u8; 32];
    let handle = handle_with_chain(7);
    let record = record_for(handle, DOMAIN, subject);

    // In scope → accepted.
    assert!(authorize_solana_handle(&verifier, &record, handle, subject, &[DOMAIN]).is_ok());

    // Out of scope (some other domain authorized) → rejected, even though the subject holds
    // USE on this record.
    let other_domain = [200u8; 32];
    let result = authorize_solana_handle(&verifier, &record, handle, subject, &[other_domain]);
    assert!(
        matches!(result, Err(ProcessingError::Recoverable(_))),
        "out-of-scope domain must be rejected, got {result:?}",
    );
}
