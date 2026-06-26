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
    solana_encrypted_value_acl::{
        EncryptedValueTarget, decode_encrypted_value_acl, encrypted_value_acl_address,
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
use zama_solana_acl::{EncryptedValueAcl, MmrProof};

/// The verified Solana auth data the ACL phase needs: the ed25519 identity (the ACL subject) and
/// the signed allowed-ACL-domain-keys scope. Returned by [`verify_solana_user_decrypt_signature`].
#[derive(Debug)]
pub struct VerifiedSolanaAuth {
    pub identity: SolanaPubkeyBytes,
    pub allowed_acl_domain_keys: Vec<SolanaPubkeyBytes>,
    /// The lineage identity key signed by the request (all-zero on a current-ACL request). Carried
    /// so the MMR-proof dispatch derives the lineage PDA without re-reading the payload.
    pub acl_value_key: [u8; 32],
    /// The full MMR-proof transport blob committed under the signature (empty on a current-ACL
    /// request): a 1-byte mode prefix (0x01 historical / 0x02 public) ‖ Borsh `MmrProof`.
    pub mmr_proof_bytes: Vec<u8>,
    /// The lineage leaf_count the proof was built against (staleness marker); 0 on current-ACL.
    pub proof_slot: u64,
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
        // MMR-proof fields are committed under the signature: a v2 preimage hashes them verbatim,
        // so a request whose proof/value_key/slot was tampered with after signing fails verify, and
        // a v1 signature (no tail) cannot verify against this v2 domain tag at all.
        acl_value_key: &payload.aclValueKey.0,
        mmr_proof_bytes: payload.mmrProof.as_ref(),
        proof_slot: payload.proofSlot,
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
        acl_value_key: payload.aclValueKey.0,
        mmr_proof_bytes: payload.mmrProof.as_ref().to_vec(),
        proof_slot: payload.proofSlot,
    })
}

/// Fetches and decodes the encrypted-value lineage account for `value_key` at `finalized`
/// commitment. The lineage PDA is derived deterministically from the value key (NOT a handle scan:
/// a historical decrypt's handle is a rotated/past value, so [`find_acl_records_by_handle`] matches
/// only on the lineage's `current_handle` and would miss it). The decoded ACL's owner is checked
/// here, and `verify_canonical` re-derives the PDA from the decoded domain/app/label inside the
/// verifier, so a forged `value_key` is caught as defense-in-depth.
///
/// [`find_acl_records_by_handle`]: SolanaV2Fetcher::find_acl_records_by_handle
pub async fn fetch_encrypted_value_acl(
    host: &SolanaHost,
    value_key: [u8; 32],
) -> Result<(SolanaPubkeyBytes, SolanaPubkeyBytes, EncryptedValueAcl), ProcessingError> {
    let (account_key, _bump) = encrypted_value_acl_address(host.program_id, value_key);

    let account = host
        .fetcher
        .get_account(&account_key)
        .await
        .map_err(ProcessingError::Recoverable)?
        .ok_or_else(|| {
            ProcessingError::Recoverable(anyhow!(
                "Solana encrypted-value lineage account {} for value key {} not found at finalized \
                 commitment",
                Pubkey::new_from_array(account_key),
                Pubkey::new_from_array(value_key),
            ))
        })?;

    if account.owner != host.program_id {
        return Err(ProcessingError::Irrecoverable(anyhow!(
            "Solana encrypted-value lineage account {} is owned by {}, expected ZamaHost program {}",
            Pubkey::new_from_array(account_key),
            Pubkey::new_from_array(account.owner),
            Pubkey::new_from_array(host.program_id),
        )));
    }

    let acl = decode_encrypted_value_acl(&account.data).map_err(|e| {
        ProcessingError::Irrecoverable(anyhow!("failed to decode encrypted-value lineage: {e}"))
    })?;

    Ok((account_key, account.owner, acl))
}

/// MMR-proof mode prefix: a historical-access decrypt (post-rotation handle).
pub const MMR_MODE_HISTORICAL: u8 = 0x01;
/// MMR-proof mode prefix: an exact public decrypt.
pub const MMR_MODE_PUBLIC: u8 = 0x02;
/// Upper bound on the MMR proof's sibling count, bounding untrusted Borsh allocation. The MMR holds
/// up to 2^64 leaves, so 64 siblings is the structural maximum; anything larger is malformed.
pub const MAX_MMR_SIBLINGS: usize = 64;

/// Enforces the single-handle scope of an MMR-proof request: one `proofSlot`/`aclValueKey`/
/// `mmrProof` covers exactly one lineage, so a non-empty proof request must carry exactly one
/// handle. Pure (no I/O) so the rejection is unit-testable without a live host; the caller invokes
/// it only on the MMR-proof branch (after confirming the proof blob is non-empty).
pub fn require_single_handle(handles: &[HandleBytes]) -> Result<HandleBytes, ProcessingError> {
    match handles {
        [handle] => Ok(*handle),
        _ => Err(ProcessingError::Irrecoverable(anyhow!(
            "Solana MMR-proof user-decryption must carry exactly one handle, got {}",
            handles.len()
        ))),
    }
}

/// Authorizes a single-handle historical/public confidential-balance decrypt against a fetched
/// lineage, given the already-verified auth and the decoded `(account_key, owner, acl)`.
///
/// Pure (no I/O) so the verify-first authorization and failure classification are unit-testable
/// without a live RPC, mirroring [`authorize_solana_handle`]. The caller (the decryption processor)
/// does the fetch and the single-handle guard, then hands the decoded lineage here.
///
/// Steps:
/// 1. parse the 1-byte mode prefix and Borsh-decode the [`MmrProof`] (rejecting > [`MAX_MMR_SIBLINGS`]),
/// 2. **verify FIRST** against the LIVE peaks via the mode-specific verifier
///    (`verify_historical_user_decrypt` / `verify_public_decrypt_exact`), whose `verify_canonical`
///    re-derives the PDA from the decoded ACL (defense-in-depth against a forged `acl_value_key`).
///    `proof_slot` (the build-time `leaf_count`) is NOT an acceptance gate: a proof built against an
///    older `leaf_count` still verifies until its own MMR mountain merges, so gating on
///    `leaf_count == proof_slot` would force needless rebuilds. Verification against the live peaks is
///    the ground truth.
/// 3. **classify a verify failure** using `proof_slot`: if `acl.leaf_count != auth.proof_slot` the
///    lineage moved since the proof was built (or the KMS read is behind finalized), so the proof may
///    just be stale — RETRYABLE (the client rebuilds against the live lineage; the historical/public
///    fact is append-only and permanent). If `acl.leaf_count == auth.proof_slot` the MMR is
///    byte-identical to build time (`leaf_count` is monotonic + append-only + deterministic), so a
///    failure can only be a genuinely invalid proof — Irrecoverable. The stale arm is RETRYABLE and
///    budget-safe: the attempt-budget arm in `processor.rs` matches only
///    `PublicDecryption | UserDecryption | UserDecryptionV2`, NOT `UserDecryptionSolana`, so a
///    Recoverable Solana error never decrements the attempt counter. If that arm is ever extended to
///    Solana, this stale path would start burning retries — keep them out of sync deliberately.
pub fn dispatch_solana_mmr_proof(
    verifier: &SolanaAclVerifier,
    account_key: SolanaPubkeyBytes,
    owner: SolanaPubkeyBytes,
    acl: &EncryptedValueAcl,
    handle: HandleBytes,
    auth: &VerifiedSolanaAuth,
) -> Result<(), ProcessingError> {
    let (mode, body) = auth
        .mmr_proof_bytes
        .split_first()
        .ok_or_else(|| ProcessingError::Irrecoverable(anyhow!("empty MMR proof blob")))?;

    let proof: MmrProof = borsh::from_slice(body).map_err(|e| {
        ProcessingError::Irrecoverable(anyhow!("failed to Borsh-decode MMR proof: {e}"))
    })?;
    if proof.siblings.len() > MAX_MMR_SIBLINGS {
        return Err(ProcessingError::Irrecoverable(anyhow!(
            "MMR proof carries {} siblings, exceeding the {MAX_MMR_SIBLINGS} cap",
            proof.siblings.len()
        )));
    }

    // Verify FIRST against the live MMR peaks — they are the ground truth. A proof built against an
    // older leaf_count keeps verifying until its own mountain merges, so we deliberately do NOT gate
    // on `leaf_count == proof_slot` (that would force needless rebuilds whenever the lineage advanced
    // without merging this proof's mountain). `proof_slot` (the build-time leaf_count) is used ONLY to
    // classify a verify FAILURE below.
    let target = EncryptedValueTarget {
        account_key,
        owner,
        acl,
        encrypted_value: handle,
    };
    let result = match *mode {
        MMR_MODE_HISTORICAL => verifier.verify_historical_user_decrypt(
            target,
            auth.identity,
            &auth.allowed_acl_domain_keys,
            &proof,
        ),
        MMR_MODE_PUBLIC => verifier.verify_public_decrypt_exact(target, &proof),
        other => {
            return Err(ProcessingError::Irrecoverable(anyhow!(
                "unknown MMR proof mode byte 0x{other:02x} (expected 0x01 historical / 0x02 public)"
            )));
        }
    };

    use crate::core::solana_acl::SolanaAclVerificationError as E;
    match result {
        Ok(()) => Ok(()),
        // The proof did not verify against the LIVE peaks. Classify by whether the lineage moved:
        //  - live leaf_count != build-time leaf_count: the lineage rotated since the proof was built
        //    (merging this proof's mountain), or the KMS read is behind finalized. The historical /
        //    public fact is append-only and permanent, so the client can rebuild against the current
        //    lineage — RETRYABLE (budget-safe: the processor.rs attempt-budget arm never matches
        //    UserDecryptionSolana, so this never decrements the counter).
        //  - live leaf_count == build-time leaf_count: leaf_count is monotonic, append-only and
        //    deterministic, so the MMR is byte-identical to build time — a failure here can only be a
        //    genuinely invalid proof. IRRECOVERABLE.
        Err(e @ (E::HistoricalAccessProofInvalid | E::PublicDecryptProofInvalid)) => {
            if acl.leaf_count != auth.proof_slot {
                Err(ProcessingError::Recoverable(anyhow!(
                    "MMR proof does not verify against the live lineage, which advanced since the \
                     proof was built (proof leaf_count={}, live leaf_count={}): stale proof — \
                     rebuild against the current lineage and resubmit ({e})",
                    auth.proof_slot,
                    acl.leaf_count,
                )))
            } else {
                Err(ProcessingError::Irrecoverable(anyhow!(
                    "MMR proof does not verify against the live lineage whose leaf_count is \
                     unchanged ({}) since the proof was built: the proof is invalid ({e})",
                    acl.leaf_count,
                )))
            }
        }
        Err(other) => Err(map_encrypted_value_acl_error(other)),
    }
}

/// Current-decrypt of a LIVE lineage handle (balance / total_supply) — no proof. Authorizes the
/// subject against the lineage's `current_handle` + membership (within the request's domain scope),
/// using the lineage read at FRESHEST finalized so a rotation / subject revocation is caught.
/// Used when a no-proof request carries a non-zero `aclValueKey`; balances/total_supply no longer
/// have a V1 `AclRecord`, so this — not the AclRecord path — is how their live handle authorizes.
pub fn dispatch_solana_current(
    verifier: &SolanaAclVerifier,
    account_key: SolanaPubkeyBytes,
    owner: SolanaPubkeyBytes,
    acl: &EncryptedValueAcl,
    handle: HandleBytes,
    auth: &VerifiedSolanaAuth,
) -> Result<(), ProcessingError> {
    verifier
        .verify_current_user_decrypt(
            account_key,
            owner,
            acl,
            handle,
            auth.identity,
            &auth.allowed_acl_domain_keys,
        )
        .map_err(map_encrypted_value_acl_error)
}

/// Maps an encrypted-value ACL verification error to a `ProcessingError`. A malformed account /
/// non-canonical PDA is Irrecoverable (the request can never succeed); an authorization miss
/// (subject not a member, proof does not authorize, domain out of scope) is Recoverable, matching
/// the [`authorize_solana_handle`] mapping for the current-ACL path.
fn map_encrypted_value_acl_error(
    error: crate::core::solana_acl::SolanaAclVerificationError,
) -> ProcessingError {
    use crate::core::solana_acl::SolanaAclVerificationError as E;
    match error {
        E::InvalidAccountOwner
        | E::NonCanonicalEncryptedValueAcl
        | E::EncryptedValueAclBumpMismatch
        | E::InvalidAccountData
        | E::AccountDataLengthMismatch
        | E::AccountDiscriminatorMismatch
        | E::MmrStateInconsistent => ProcessingError::Irrecoverable(anyhow!(
            "Solana encrypted-value lineage is malformed: {error}"
        )),
        // A proof that does not verify against the live peaks is a permanent client fault: a wrong
        // hash or leaf index can never be retried into success. Irrecoverable, not Recoverable —
        // otherwise the budget-exempt Solana path would re-queue a cryptographically dead proof
        // forever (see `dispatch_solana_mmr_proof` and the processor.rs budget arm).
        E::HistoricalAccessProofInvalid | E::PublicDecryptProofInvalid => {
            ProcessingError::Irrecoverable(anyhow!(
                "Solana encrypted-value MMR proof does not verify against the live lineage: {error}"
            ))
        }
        // Authorization misses (handle not current, subject not a member, domain out of scope) can
        // change with an ACL update, so they stay Recoverable.
        other => ProcessingError::Recoverable(anyhow!(
            "Solana encrypted-value decrypt not authorized: {other}"
        )),
    }
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

/// PKCS#8 v1 wrapper for an Ed25519 seed (RFC 8410): the fixed 16-byte prefix ‖ the 32-byte seed.
/// Shared by the test modules below (both `use super::*`).
#[cfg(test)]
fn pkcs8_from_seed(seed: &[u8; 32]) -> Vec<u8> {
    let prefix: [u8; 16] = [
        0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x04, 0x22, 0x04,
        0x20,
    ];
    let mut doc = prefix.to_vec();
    doc.extend_from_slice(seed);
    doc
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mmr_proof_tests;
