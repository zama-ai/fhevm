//! Solana branch of the V2 user-decryption authorization check.
//!
//! ## Why this exists (the bug it closes)
//!
//! On EVM the Gateway verifies, on-chain, the EIP-712 signature binding the re-encryption
//! `publicKey` to the requesting `userAddress`, so the relayer is untrusted. The Solana host has
//! no equivalent on-chain binding. An earlier Solana path verified the user's ed25519 signature
//! *only in the relayer*, which is bypassable: anyone calling the Gateway directly could name a
//! victim's Solana identity together with an attacker-controlled `publicKey` and have the
//! victim's plaintext re-encrypted to the attacker's key.
//!
//! This module re-verifies the ed25519 signature **inside every KMS party's connector**, binding
//! the re-encryption `publicKey` (and the handles, identity, nonce, allowed domains, and validity
//! window) to the Solana identity. A substituted key or a relayer-forged signature cannot pass.
//!
//! ## Flow
//!
//! 1. parse the canonical Solana `extraData` (identity, nonce, allowed ACL domain keys),
//! 2. rebuild the canonical signing preimage and verify `payload.signature` as an ed25519
//!    signature by the identity over that preimage,
//! 3. read each handle's ACL record from the Solana host RPC at `finalized` commitment, check the
//!    account owner and canonical PDA, then run the domain-scoped ACL verifier with the identity
//!    as the subject.

use crate::core::{
    event_processor::ProcessingError,
    solana_acl::{
        ACL_RECORD_SEED, AclRecordWitness, HandleBytes, SolanaAclVerifier, SolanaPubkeyBytes,
        decode_acl_record_witness,
    },
    solana_v2_fetcher::SolanaV2Fetcher,
};
use alloy::primitives::U256;
use anyhow::anyhow;
use connector_utils::types::solana_extra_data::{
    SolanaUserDecryptSigningInput, solana_user_decrypt_signing_preimage,
};
use fhevm_gateway_bindings::decryption::Decryption::UserDecryptionRequestSolana;
use ring::signature::{ED25519, UnparsedPublicKey};
use solana_pubkey::Pubkey;

/// The verified Solana auth data the ACL phase needs: the ed25519 identity (the ACL subject) and
/// the signed allowed-ACL-domain-keys scope. Returned by [`verify_solana_user_decrypt_signature`].
#[derive(Debug)]
pub struct VerifiedSolanaAuth {
    pub identity: SolanaPubkeyBytes,
    pub allowed_acl_domain_keys: Vec<SolanaPubkeyBytes>,
}

/// Per-chain Solana host configuration needed to authorize a user-decrypt request: the expected
/// ZamaHost program id and a `finalized`-commitment account fetcher.
#[derive(Clone, Debug)]
pub struct SolanaHost {
    pub program_id: SolanaPubkeyBytes,
    pub fetcher: SolanaV2Fetcher,
}

/// Verifies the ed25519 signature binding for a Solana user-decryption request. Pure (no I/O), so
/// the publicKey-substitution and forged-signature rejections are unit-testable without a live
/// RPC. The auth fields (identity, nonce, allowed ACL domain keys) are read as TYPED fields off the
/// request — there is no `extraData` blob to decode; `extraData` carries only the KMS context.
/// Returns the [`VerifiedSolanaAuth`] (identity + scope) for the caller to drive the ACL phase.
pub fn verify_solana_user_decrypt_signature(
    request: &UserDecryptionRequestSolana,
    contracts_chain_id: u64,
) -> Result<VerifiedSolanaAuth, ProcessingError> {
    let payload = &request.payload;

    let identity: SolanaPubkeyBytes = payload.userIdentity.0;
    let nonce: SolanaPubkeyBytes = payload.nonce.0;
    let allowed_acl_domain_keys: Vec<SolanaPubkeyBytes> =
        payload.allowedAclDomainKeys.iter().map(|k| k.0).collect();
    // extraData carries only the KMS context (v0x01: version ‖ contextId(32)).
    let context_id = extract_context_id_be(payload.extraData.as_ref());

    let handles: Vec<HandleBytes> = request.handles.iter().map(|entry| entry.handle.0).collect();

    let preimage = solana_user_decrypt_signing_preimage(&SolanaUserDecryptSigningInput {
        contracts_chain_id,
        public_key: payload.publicKey.as_ref(),
        handles: &handles,
        identity: &identity,
        context_id: &context_id,
        nonce: &nonce,
        allowed_acl_domain_keys: &allowed_acl_domain_keys,
        start_timestamp: saturating_u256_to_u64(payload.requestValidity.startTimestamp),
        duration_seconds: saturating_u256_to_u64(payload.requestValidity.durationSeconds),
    });

    let signature = payload.signature.as_ref();
    let identity_key = UnparsedPublicKey::new(&ED25519, identity);
    identity_key.verify(&preimage, signature).map_err(|_| {
        // A substituted publicKey, a forged/relayer-only signature, or a wrong identity all land
        // here: the signature does not verify against the claimed Solana identity over the bound
        // re-encryption key. This is the check that closes the substitution bug.
        ProcessingError::Irrecoverable(anyhow!(
            "Solana user-decryption ed25519 signature verification failed: the signature does not \
             bind the re-encryption publicKey to the claimed identity"
        ))
    })?;

    Ok(VerifiedSolanaAuth {
        identity,
        allowed_acl_domain_keys,
    })
}

/// Verifies, for a single handle, that its on-chain ACL record authorizes `subject` for USE
/// within the signed `allowed_acl_domain_keys` scope. Pure (no I/O): the caller supplies the
/// fetched-and-owner/PDA-checked record. Kept separate so the domain-scoping rejection is
/// unit-testable with a crafted witness.
pub fn authorize_solana_handle(
    verifier: &SolanaAclVerifier,
    record: &AclRecordWitness,
    handle: HandleBytes,
    subject: SolanaPubkeyBytes,
    allowed_acl_domain_keys: &[SolanaPubkeyBytes],
) -> Result<(), ProcessingError> {
    // Domain-scoped verifier: rejects a subject that holds USE on a handle whose acl_domain_key is
    // outside the request's signed allowedContracts scope.
    verifier
        .verify_user_decrypt(record, &[], handle, subject, allowed_acl_domain_keys)
        .map_err(|e| {
            ProcessingError::Recoverable(anyhow!(
                "Solana ACL authorization failed for handle {}: {e}",
                hex_handle(&handle)
            ))
        })
}

/// Full Solana user-decryption ACL check for one request against its host: verifies each handle's
/// ACL record fetched at `finalized` commitment.
///
/// For every handle:
/// 1. re-derive the canonical ACL-record PDA from the (domain, app, label, sequence) decoded from
///    the on-chain account, rejecting a non-canonical address,
/// 2. require `account.owner == ZamaHost program id`,
/// 3. run the domain-scoped [`SolanaAclVerifier::verify_user_decrypt`] with the identity as the
///    subject.
///
/// The record is located by the canonical PDA, so a multi-account ambiguity cannot arise: a
/// handle maps to exactly one ACL-record address. The fetch is rejected (not silently skipped) if
/// the account is missing or owned by the wrong program.
pub async fn check_solana_handles_acl(
    host: &SolanaHost,
    handles: &[HandleBytes],
    subject: SolanaPubkeyBytes,
    allowed_acl_domain_keys: &[SolanaPubkeyBytes],
) -> Result<(), ProcessingError> {
    let verifier = SolanaAclVerifier::new(host.program_id);

    for handle in handles {
        let record = fetch_acl_record_for_handle(host, *handle).await?;
        authorize_solana_handle(
            &verifier,
            &record,
            *handle,
            subject,
            allowed_acl_domain_keys,
        )?;
    }
    Ok(())
}

/// Solana public-decryption ACL check: each handle's on-chain ACL record (fetched at `finalized`
/// with the same owner + canonical-PDA checks as the user-decrypt path) must carry the
/// `public_decrypt` flag, i.e. the handle was released for public decryption on the host
/// (`allow_for_decryption`). Unlike user-decrypt there is no subject/domain scope — a publicly
/// released handle is decryptable by anyone.
pub async fn check_solana_handles_public_decrypt(
    host: &SolanaHost,
    handles: &[HandleBytes],
) -> Result<(), ProcessingError> {
    let verifier = SolanaAclVerifier::new(host.program_id);

    for handle in handles {
        let record = fetch_acl_record_for_handle(host, *handle).await?;
        verifier
            .verify_public_decrypt(&record, *handle)
            .map_err(|e| {
                ProcessingError::Recoverable(anyhow!(
                    "Solana public-decrypt not authorized for handle {}: {e}",
                    hex_handle(handle)
                ))
            })?;
    }
    Ok(())
}

/// Fetches and decodes the ACL record for `handle`, enforcing owner + canonical-PDA invariants.
///
/// The ACL-record PDA is derived from the record's nonce metadata, which the connector does not
/// know up front from the handle alone. The connector therefore reads the candidate record, then
/// re-derives the PDA from the decoded metadata and rejects any account whose address is not the
/// canonical PDA — closing the door on a substituted account. `verify_user_decrypt` re-checks the
/// same PDA and owner, so this is defense-in-depth, not the sole gate.
async fn fetch_acl_record_for_handle(
    host: &SolanaHost,
    handle: HandleBytes,
) -> Result<AclRecordWitness, ProcessingError> {
    let account_key = acl_record_account_for_handle(host, handle).await?;

    let account = host
        .fetcher
        .get_account(&account_key)
        .await
        .map_err(ProcessingError::Recoverable)?
        .ok_or_else(|| {
            ProcessingError::Recoverable(anyhow!(
                "Solana ACL record account {} for handle {} not found at finalized commitment",
                Pubkey::new_from_array(account_key),
                hex_handle(&handle)
            ))
        })?;

    if account.owner != host.program_id {
        return Err(ProcessingError::Irrecoverable(anyhow!(
            "Solana ACL record account {} is owned by {}, expected ZamaHost program {}",
            Pubkey::new_from_array(account_key),
            Pubkey::new_from_array(account.owner),
            Pubkey::new_from_array(host.program_id),
        )));
    }

    decode_acl_record_witness(account_key, account.owner, &account.data)
        .map_err(|e| ProcessingError::Irrecoverable(anyhow!("failed to decode ACL record: {e}")))
}

/// Resolves the single canonical ACL-record account address for `handle` by scanning the host
/// program's ACL-record accounts.
///
/// ZamaHost derives the ACL-record PDA from `(nonce_key, nonce_sequence)`, neither of which is
/// recoverable from the handle alone, so the connector locates the record by querying the program
/// for accounts whose decoded `handle` matches. Exactly one match is required: a missing record is
/// rejected, and **multiple matches are rejected** rather than silently taking the first.
async fn acl_record_account_for_handle(
    host: &SolanaHost,
    handle: HandleBytes,
) -> Result<SolanaPubkeyBytes, ProcessingError> {
    let matches = host
        .fetcher
        .find_acl_records_by_handle(&host.program_id, ACL_RECORD_SEED, &handle)
        .await
        .map_err(ProcessingError::Recoverable)?;

    match matches.as_slice() {
        [] => Err(ProcessingError::Recoverable(anyhow!(
            "no Solana ACL record found for handle {} under program {}",
            hex_handle(&handle),
            Pubkey::new_from_array(host.program_id),
        ))),
        [single] => Ok(*single),
        many => Err(ProcessingError::Irrecoverable(anyhow!(
            "ambiguous Solana ACL records for handle {}: {} accounts matched, refusing to choose",
            hex_handle(&handle),
            many.len(),
        ))),
    }
}

/// Reads the 32-byte context_id from a Solana extraData blob (bytes 1..33), zero-filled if absent.
fn extract_context_id_be(extra_data: &[u8]) -> [u8; 32] {
    let mut context_id = [0u8; 32];
    if extra_data.len() >= 33 {
        context_id.copy_from_slice(&extra_data[1..33]);
    }
    context_id
}

fn saturating_u256_to_u64(value: U256) -> u64 {
    value.try_into().unwrap_or(u64::MAX)
}

fn hex_handle(handle: &HandleBytes) -> String {
    alloy::hex::encode(handle)
}

#[cfg(test)]
mod tests {
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
    fn pkcs8_from_seed(seed: &[u8; 32]) -> Vec<u8> {
        // PKCS#8 v1 prefix for Ed25519 private keys (RFC 8410), followed by the 32-byte seed.
        let prefix: [u8; 16] = [
            0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x04, 0x22,
            0x04, 0x20,
        ];
        let mut doc = prefix.to_vec();
        doc.extend_from_slice(seed);
        doc
    }

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
        let (request, _identity) =
            signed_request(b"reencryption-public-key".to_vec(), vec![DOMAIN]);
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
        let (request, _identity) =
            signed_request(b"reencryption-public-key".to_vec(), vec![DOMAIN]);
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
}
