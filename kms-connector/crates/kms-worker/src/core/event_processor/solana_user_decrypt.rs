//! Solana branch of the V2 user-decryption authorization check (RFC-024 `EncryptedValue` rewrite).
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
//! the re-encryption `publicKey` (handles, identity, nonce, allowed domains, validity window) AND
//! — new in this rewrite — the MMR proof tail (`acl_value_key`, proof leaf count carried in
//! the legacy `proof_slot` wire field, `mmr_proof_bytes`)
//! to the Solana identity. A relayer cannot substitute the proof, the lineage, or the slot any
//! more than it could substitute the re-encryption key: `SOLANA_USER_DECRYPT_DOMAIN_TAG` was
//! bumped to `v2` specifically so a `v1` signature (proof-less) can never verify against the `v2`
//! preimage, closing the same class of relayer-bypass bug for the new MMR fields.
//!
//! ## DEVIATION FROM THE REFERENCE DESIGN: MMR fields travel in `extraData`, not typed fields
//!
//! The reference design assumed `aclValueKey` / `mmrProof` / `proofSlot` are typed
//! `UserDecryptionRequestSolanaPayload` fields. Those fields do not exist on the
//! `gateway-contracts` Solidity interface / generated Rust bindings today, and adding them is a
//! `gateway-contracts` change outside this workstream's scope (`kms-connector/` +
//! `sdk/js-sdk/` only). Instead, they are packed into the existing `extraData` blob, versioned
//! (see `connector_utils::types::solana_extra_data`): `v0x01` = context-only (no MMR tail, as
//! before), `v0x03` = context + MMR tail. This is transport-only: the signed preimage bytes and
//! the signature-binding property are unchanged from the reference design.
//!
//! ## Dual-path ACL check
//!
//! Every Solana user-decrypt / public-decrypt request now names its `EncryptedValue` lineage by
//! `acl_value_key` and is single-handle (see [`require_single_handle`] — a proof, when present,
//! authorizes exactly one handle, so multi-handle Solana requests are out of scope for this
//! rewrite):
//!
//! - **current-handle** (`mmr_proof_bytes` empty): the lineage account, fetched at `confirmed`
//!   commitment, is checked for canonical PDA derivation + program ownership + `current_handle ==
//!   handle` + `subject ∈ subjects` ([`dispatch_solana_current`]).
//! - **historical / public** (`mmr_proof_bytes` non-empty): the request carries an MMR inclusion
//!   proof, verified against the live confirmed peaks (the account is always freshly fetched, never
//!   a cached/proof-time snapshot) via the shared crate's `authorize_historical` /
//!   `authorize_public` ([`dispatch_solana_mmr_proof`]). There is no live "is public" flag any
//!   more — public-ness is only provable via a `PublicDecryptLeaf` MMR leaf.
//!
//! ## Freshness contract
//!
//! [`dispatch_solana_mmr_proof`] verifies the proof against the live peaks FIRST. Only after a
//! inclusion-proof failure does the proof's leaf count classify the result:
//! - **proof count < live count**: terminal; the immutable queued proof is stale.
//! - **proof count >= live count**: recoverable through the ordinary bounded attempt budget; the
//!   proof service and KMS may be observing different confirmed forks or the KMS may be behind, so
//!   the same proof may verify after catch-up.
//!
//! Domain, owner, canonical-account, bump, and inconsistent-MMR failures remain terminal regardless
//! of the two counts because confirmed-view catch-up cannot repair them.
//!
//! A proof that still verifies against the live peaks despite `proof_leaf_count != leaf_count` (count
//! drifted but the relevant mountain never merged) is accepted — verify-first, never rebuilt from
//! a mere count mismatch.

use crate::core::{
    event_processor::ProcessingError,
    solana_acl::{HandleBytes, SolanaAclVerifier, SolanaPubkeyBytes},
    solana_encrypted_value_acl::{
        DecodedEncryptedValueAcl, EncryptedValueTarget, decode_encrypted_value_acl,
        encrypted_value_acl_address,
    },
    solana_v2_fetcher::SolanaV2Fetcher,
};
use alloy::primitives::U256;
use anyhow::anyhow;
use borsh::BorshDeserialize;
use connector_utils::types::solana_extra_data::{
    SolanaUserDecryptSigningInput, parse_solana_mmr_proof_extra_data,
    parse_solana_user_decrypt_extra_data, solana_user_decrypt_signing_preimage,
};
use fhevm_gateway_bindings::decryption::Decryption::UserDecryptionRequestSolana;
use ring::signature::{ED25519, UnparsedPublicKey};
use solana_pubkey::Pubkey;
use zama_solana_acl::{EncryptedValue, MmrProof};

/// Transport-blob mode byte for a historical-access MMR proof (see `mmr_proof_bytes`).
pub const MMR_MODE_HISTORICAL: u8 = 0x01;
/// Transport-blob mode byte for a public-decrypt MMR proof.
pub const MMR_MODE_PUBLIC: u8 = 0x02;
/// Upper bound on `MmrProof::siblings` accepted from an untrusted request, matching the MMR's
/// `u64` height ceiling (`mmr.rs` iterates heights `0..64`); bounds the decode-time allocation.
pub const MAX_MMR_SIBLINGS: usize = 64;

/// The verified Solana auth data the ACL phase needs.
#[derive(Debug)]
pub struct VerifiedSolanaAuth {
    pub identity: SolanaPubkeyBytes,
    pub allowed_acl_domain_keys: Vec<SolanaPubkeyBytes>,
    /// The `EncryptedValue` lineage this request targets. All-zero is only meaningful for the
    /// synthetic "current-ACL-shaped" preimage used to guard against MMR-proof injection (see the
    /// `current_acl_mmr_proof_injection_rejected` test) — a real request always names its lineage.
    pub acl_value_key: [u8; 32],
    /// The full MMR-proof transport blob (mode byte ‖ Borsh `MmrProof`); empty for a current-ACL
    /// (no-proof) request.
    pub mmr_proof_bytes: Vec<u8>,
    /// The lineage `leaf_count` the proof was built against; 0 for a current-ACL request.
    pub proof_leaf_count: u64,
}

/// Per-chain Solana host configuration needed to authorize a user-decrypt request: the expected
/// ZamaHost program id and a `confirmed`-commitment account fetcher.
#[derive(Clone, Debug)]
pub struct SolanaHost {
    pub program_id: SolanaPubkeyBytes,
    pub fetcher: SolanaV2Fetcher,
}

/// Verifies the ed25519 signature binding for a Solana user-decryption request, over the `v2`
/// preimage (handles, identity, nonce, allowed domains, validity window, AND the MMR-proof tail).
/// Pure (no I/O), so the publicKey-substitution, forged-signature, and proof-tail-tampering
/// rejections are unit-testable without a live RPC.
pub fn verify_solana_user_decrypt_signature(
    request: &UserDecryptionRequestSolana,
    contracts_chain_id: u64,
) -> Result<VerifiedSolanaAuth, ProcessingError> {
    let payload = &request.payload;

    let identity: SolanaPubkeyBytes = payload.userIdentity.0;
    let nonce: SolanaPubkeyBytes = payload.nonce.0;
    let allowed_acl_domain_keys: Vec<SolanaPubkeyBytes> =
        payload.allowedAclDomainKeys.iter().map(|k| k.0).collect();

    let extra = parse_solana_user_decrypt_extra_data(payload.extraData.as_ref());

    let handles: Vec<HandleBytes> = request.handles.iter().map(|entry| entry.handle.0).collect();

    let preimage = solana_user_decrypt_signing_preimage(&SolanaUserDecryptSigningInput {
        contracts_chain_id,
        public_key: payload.publicKey.as_ref(),
        handles: &handles,
        identity: &identity,
        context_id: &extra.context_id,
        nonce: &nonce,
        allowed_acl_domain_keys: &allowed_acl_domain_keys,
        start_timestamp: saturating_u256_to_u64(payload.requestValidity.startTimestamp),
        duration_seconds: saturating_u256_to_u64(payload.requestValidity.durationSeconds),
        acl_value_key: &extra.acl_value_key,
        mmr_proof_bytes: &extra.mmr_proof_bytes,
        proof_slot: extra.proof_slot,
    });

    let signature = payload.signature.as_ref();
    let identity_key = UnparsedPublicKey::new(&ED25519, identity);
    identity_key.verify(&preimage, signature).map_err(|_| {
        // A substituted publicKey, a substituted/mutated MMR-proof tail, a forged/relayer-only
        // signature, or a wrong identity all land here.
        ProcessingError::Irrecoverable(anyhow!(
            "Solana user-decryption ed25519 signature verification failed: the signature does not \
             bind the re-encryption publicKey and MMR-proof tail to the claimed identity"
        ))
    })?;

    Ok(VerifiedSolanaAuth {
        identity,
        allowed_acl_domain_keys,
        acl_value_key: extra.acl_value_key,
        mmr_proof_bytes: extra.mmr_proof_bytes,
        proof_leaf_count: extra.proof_slot,
    })
}

/// Enforces the single-handle scope of the `EncryptedValue` rewrite: an MMR proof (when present)
/// authorizes exactly one handle, and the current-ACL path is likewise scoped to the one lineage
/// named by `acl_value_key` — so every Solana user-decrypt/public-decrypt request under this
/// rewrite must name exactly one handle. Pure so the rejection is unit-testable without a host.
pub fn require_single_handle(handles: &[HandleBytes]) -> Result<HandleBytes, ProcessingError> {
    match handles {
        [single] => Ok(*single),
        other => Err(ProcessingError::Irrecoverable(anyhow!(
            "Solana EncryptedValue user-decrypt requires exactly one handle per request, got {}",
            other.len()
        ))),
    }
}

/// Fetches the `EncryptedValue` lineage for `value_key` at `confirmed` commitment and decodes it.
/// Never a snapshot: every call re-reads the live account, which is what lets
/// [`dispatch_solana_mmr_proof`] verify against the LIVE peaks.
async fn fetch_encrypted_value_acl(
    host: &SolanaHost,
    value_key: [u8; 32],
) -> Result<(SolanaPubkeyBytes, DecodedEncryptedValueAcl), ProcessingError> {
    let (account_key, _bump) = encrypted_value_acl_address(host.program_id, value_key);

    let account = host
        .fetcher
        .get_account(&account_key)
        .await
        .map_err(ProcessingError::Recoverable)?
        .ok_or_else(|| {
            ProcessingError::Recoverable(anyhow!(
                "Solana EncryptedValue lineage account {} not found at confirmed commitment",
                Pubkey::new_from_array(account_key),
            ))
        })?;

    if account.owner != host.program_id {
        return Err(ProcessingError::Irrecoverable(anyhow!(
            "Solana EncryptedValue lineage account {} is owned by {}, expected ZamaHost program {}",
            Pubkey::new_from_array(account_key),
            Pubkey::new_from_array(account.owner),
            Pubkey::new_from_array(host.program_id),
        )));
    }

    let acl = decode_encrypted_value_acl(&account.data).map_err(|e| {
        ProcessingError::Irrecoverable(anyhow!("failed to decode EncryptedValue lineage: {e}"))
    })?;
    Ok((account_key, acl))
}

/// Current-handle path: no MMR proof, `handle` must be the lineage's live `current_handle` and
/// `auth.identity` a current member, within the signed domain scope.
pub fn dispatch_solana_current(
    verifier: &SolanaAclVerifier,
    account_key: SolanaPubkeyBytes,
    owner: SolanaPubkeyBytes,
    decoded: &DecodedEncryptedValueAcl,
    handle: HandleBytes,
    auth: &VerifiedSolanaAuth,
) -> Result<(), ProcessingError> {
    verifier
        .verify_current_user_decrypt(
            account_key,
            owner,
            decoded,
            handle,
            auth.identity,
            &auth.allowed_acl_domain_keys,
        )
        .map_err(|e| {
            ProcessingError::Recoverable(anyhow!(
                "Solana current-lineage ACL check failed for handle {}: {e}",
                hex_handle(&handle)
            ))
        })
}

/// Historical/public path: decodes the mode byte + `MmrProof` from `auth.mmr_proof_bytes`,
/// verifies it against the LIVE peaks in `acl`, and classifies a verification failure by
/// `auth.proof_leaf_count` vs `acl.leaf_count` (see the module doc's freshness contract).
pub fn dispatch_solana_mmr_proof(
    verifier: &SolanaAclVerifier,
    account_key: SolanaPubkeyBytes,
    owner: SolanaPubkeyBytes,
    acl: &EncryptedValue,
    handle: HandleBytes,
    auth: &VerifiedSolanaAuth,
) -> Result<(), ProcessingError> {
    let (mode, proof) = decode_solana_mmr_proof_blob(&auth.mmr_proof_bytes)?;

    let target = EncryptedValueTarget {
        account_key,
        owner,
        acl,
        encrypted_value: handle,
    };

    let result = match mode {
        MMR_MODE_HISTORICAL => verifier.verify_historical_user_decrypt(
            target,
            auth.identity,
            &auth.allowed_acl_domain_keys,
            &proof,
        ),
        MMR_MODE_PUBLIC => verifier.verify_public_decrypt_exact(target, &proof),
        other => {
            return Err(ProcessingError::Irrecoverable(anyhow!(
                "unknown Solana MMR proof mode byte {other:#04x}"
            )));
        }
    };

    result.map_err(|e| classify_mmr_verification_failure(e, auth.proof_leaf_count, acl.leaf_count))
}

fn decode_solana_mmr_proof_blob(mmr_proof_bytes: &[u8]) -> Result<(u8, MmrProof), ProcessingError> {
    let [mode, proof_body @ ..] = mmr_proof_bytes else {
        return Err(ProcessingError::Irrecoverable(anyhow!(
            "Solana MMR-proof blob is empty (missing mode byte)"
        )));
    };
    let mut cursor = proof_body;
    let proof = MmrProof::deserialize(&mut cursor).map_err(|e| {
        ProcessingError::Irrecoverable(anyhow!("failed to decode Solana MMR proof: {e}"))
    })?;
    if !cursor.is_empty() {
        return Err(ProcessingError::Irrecoverable(anyhow!(
            "Solana MMR-proof blob has {} trailing byte(s) after the Borsh proof",
            cursor.len()
        )));
    }
    if proof.siblings.len() > MAX_MMR_SIBLINGS {
        return Err(ProcessingError::Irrecoverable(anyhow!(
            "Solana MMR proof carries {} siblings, exceeding the cap of {MAX_MMR_SIBLINGS}",
            proof.siblings.len()
        )));
    }
    Ok((*mode, proof))
}

fn dispatch_solana_public_mmr_proof(
    verifier: &SolanaAclVerifier,
    account_key: SolanaPubkeyBytes,
    owner: SolanaPubkeyBytes,
    acl: &EncryptedValue,
    handle: HandleBytes,
    proof_leaf_count: u64,
    mmr_proof_bytes: &[u8],
) -> Result<(), ProcessingError> {
    let (mode, proof) = decode_solana_mmr_proof_blob(mmr_proof_bytes)?;
    if mode != MMR_MODE_PUBLIC {
        return Err(ProcessingError::Irrecoverable(anyhow!(
            "Solana public decryption requires MMR proof mode {MMR_MODE_PUBLIC:#04x}, got {mode:#04x}"
        )));
    }

    let target = EncryptedValueTarget {
        account_key,
        owner,
        acl,
        encrypted_value: handle,
    };
    verifier
        .verify_public_decrypt_exact(target, &proof)
        .map_err(|e| classify_mmr_verification_failure(e, proof_leaf_count, acl.leaf_count))
}

/// Verify-first failure classification. An inclusion-proof mismatch at or ahead of the KMS
/// confirmed view can heal through catch-up or a confirmed-fork change, so it gets an ordinary
/// bounded retry. All other verifier errors, and proof mismatches behind the live count, are terminal.
fn classify_mmr_verification_failure(
    error: crate::core::solana_acl::SolanaAclVerificationError,
    proof_leaf_count: u64,
    live_leaf_count: u64,
) -> ProcessingError {
    let proof_invalid = matches!(
        error,
        crate::core::solana_acl::SolanaAclVerificationError::HistoricalAccessProofInvalid
            | crate::core::solana_acl::SolanaAclVerificationError::PublicDecryptProofInvalid
    );
    if proof_invalid && proof_leaf_count >= live_leaf_count {
        ProcessingError::Recoverable(anyhow!(
            "Solana MMR proof does not verify against the KMS confirmed view: proof leaf_count={proof_leaf_count}, \
             live confirmed leaf_count={live_leaf_count}; retrying within the normal attempt budget \
             while confirmed views converge ({error})"
        ))
    } else if proof_invalid && proof_leaf_count < live_leaf_count {
        ProcessingError::Irrecoverable(anyhow!(
            "Solana MMR proof is stale and immutable: proof leaf_count={proof_leaf_count}, live \
             confirmed leaf_count={live_leaf_count} ({error})"
        ))
    } else {
        ProcessingError::Irrecoverable(anyhow!(
            "Solana MMR authorization failed irrecoverably: proof leaf_count={proof_leaf_count}, \
             live confirmed leaf_count={live_leaf_count} ({error})"
        ))
    }
}

/// Full Solana user-decryption ACL check for one request: fetches the named `EncryptedValue`
/// lineage at `confirmed` commitment and dispatches to the current or MMR-proof path.
pub async fn check_solana_handles_acl(
    host: &SolanaHost,
    handles: &[HandleBytes],
    auth: &VerifiedSolanaAuth,
) -> Result<(), ProcessingError> {
    let handle = require_single_handle(handles)?;
    let verifier = SolanaAclVerifier::new(host.program_id);
    let (account_key, decoded) = fetch_encrypted_value_acl(host, auth.acl_value_key).await?;

    if auth.mmr_proof_bytes.is_empty() {
        dispatch_solana_current(
            &verifier,
            account_key,
            host.program_id,
            &decoded,
            handle,
            auth,
        )
    } else {
        dispatch_solana_mmr_proof(
            &verifier,
            account_key,
            host.program_id,
            &decoded.acl,
            handle,
            auth,
        )
    }
}

/// Solana public-decryption ACL check. There is no live "is public" flag: public-ness is only
/// provable via a `PublicDecryptLeaf` MMR leaf carried in `extraData`.
pub async fn check_solana_handles_public_decrypt(
    host: &SolanaHost,
    handles: &[HandleBytes],
    extra_data: &[u8],
) -> Result<(), ProcessingError> {
    let handle = require_single_handle(handles)?;
    let Some(extra) = parse_solana_mmr_proof_extra_data(extra_data) else {
        return Err(public_decrypt_requires_proof(handles.len()));
    };
    if extra.mmr_proof_bytes.is_empty() {
        return Err(public_decrypt_requires_proof(handles.len()));
    }

    let verifier = SolanaAclVerifier::new(host.program_id);
    let (account_key, decoded) = fetch_encrypted_value_acl(host, extra.acl_value_key).await?;
    dispatch_solana_public_mmr_proof(
        &verifier,
        account_key,
        host.program_id,
        &decoded.acl,
        handle,
        extra.proof_slot,
        &extra.mmr_proof_bytes,
    )
}

fn public_decrypt_requires_proof(handle_count: usize) -> ProcessingError {
    ProcessingError::Irrecoverable(anyhow!(
        "Solana public decryption for {} handle(s) requires a PublicDecryptLeaf MMR proof, which \
         the gateway public-decryption request did not carry; refusing rather than granting or \
         reading a deleted on-chain flag",
        handle_count
    ))
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
    use alloy::primitives::{Address, Bytes, FixedBytes};
    use connector_utils::types::solana_extra_data::{
        SOLANA_USER_DECRYPT_DOMAIN_TAG, encode_solana_extra_data_context_only,
        encode_solana_extra_data_mmr_proof,
    };
    use fhevm_gateway_bindings::decryption::{
        Decryption::{HandleEntry, SnsCiphertextMaterial, UserDecryptionRequestSolana},
        IDecryption::{RequestValiditySeconds, UserDecryptionRequestSolanaPayload},
    };
    use ring::signature::{Ed25519KeyPair, KeyPair};
    use zama_solana_acl::{
        derive_value_key, historical_access_leaf_commitment, mmr_append, mmr_build_proof,
        public_decrypt_leaf_commitment,
    };

    const CHAIN_ID: u64 = 7777;
    const HOST: SolanaPubkeyBytes = [42u8; 32];
    const DOMAIN: SolanaPubkeyBytes = [1u8; 32];
    const APP: SolanaPubkeyBytes = [2u8; 32];
    const LABEL: [u8; 32] = *b"balance_________________________";
    /// Wraps a raw ed25519 seed in a minimal PKCS#8 v1 document.
    fn pkcs8_from_seed(seed: &[u8; 32]) -> Vec<u8> {
        let prefix: [u8; 16] = [
            0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x04, 0x22,
            0x04, 0x20,
        ];
        let mut doc = prefix.to_vec();
        doc.extend_from_slice(seed);
        doc
    }

    fn identity_kp(seed: u8) -> Ed25519KeyPair {
        Ed25519KeyPair::from_pkcs8_maybe_unchecked(&pkcs8_from_seed(&[seed; 32])).unwrap()
    }

    fn h(tag: u8) -> HandleBytes {
        [tag; 32]
    }

    /// A lineage whose account bytes and proofs are produced by the shared crate, mirroring the
    /// helper in `solana_encrypted_value_acl.rs`.
    struct Lineage {
        acl: EncryptedValue,
        account: SolanaPubkeyBytes,
        leaves: Vec<[u8; 32]>,
    }

    fn lineage(handle: HandleBytes, subjects: &[SolanaPubkeyBytes]) -> Lineage {
        let value_key = derive_value_key(DOMAIN, APP, LABEL);
        let (account, bump) = encrypted_value_acl_address(HOST, value_key);
        Lineage {
            acl: EncryptedValue {
                acl_domain_key: DOMAIN,
                app_account: APP,
                encrypted_value_label: LABEL,
                current_handle: handle,
                subjects: subjects.to_vec(),
                leaf_count: 0,
                peaks: Vec::new(),
                bump,
            },
            account,
            leaves: Vec::new(),
        }
    }

    impl Lineage {
        fn value_key(&self) -> [u8; 32] {
            derive_value_key(
                self.acl.acl_domain_key,
                self.acl.app_account,
                self.acl.encrypted_value_label,
            )
        }
        fn append(&mut self, commitment: [u8; 32]) {
            mmr_append(&mut self.acl.peaks, &mut self.acl.leaf_count, commitment).unwrap();
            self.leaves.push(commitment);
        }
        fn rotate(&mut self, new_handle: HandleBytes) {
            let old = self.acl.current_handle;
            for i in 0..self.acl.subjects.len() {
                let idx = self.acl.leaf_count;
                self.append(historical_access_leaf_commitment(
                    self.account,
                    idx,
                    old,
                    self.acl.subjects[i],
                ));
            }
            self.acl.current_handle = new_handle;
        }
        fn mark_public(&mut self) {
            let idx = self.acl.leaf_count;
            self.append(public_decrypt_leaf_commitment(
                self.account,
                idx,
                self.acl.current_handle,
            ));
        }
        fn proof(&self, i: u64) -> MmrProof {
            mmr_build_proof(&self.leaves, i).unwrap()
        }
        fn proof_for_empty(&self) -> MmrProof {
            MmrProof {
                leaf_index: 0,
                siblings: Vec::new(),
            }
        }
    }

    /// The full transport blob: 1-byte mode prefix ‖ Borsh(MmrProof).
    fn proof_blob(mode: u8, proof: &MmrProof) -> Vec<u8> {
        let mut blob = vec![mode];
        blob.extend_from_slice(&borsh::to_vec(proof).unwrap());
        blob
    }

    fn decoded(l: &Lineage) -> DecodedEncryptedValueAcl {
        DecodedEncryptedValueAcl { acl: l.acl.clone() }
    }

    /// Builds a v2-signed single-handle request. `proof_blob`/`value_key`/`proof_slot` (when
    /// non-empty/non-zero) are packed into `extraData` and bound into the signature exactly as
    /// production does.
    fn signed_mmr_request(
        identity_kp: &Ed25519KeyPair,
        handle: HandleBytes,
        value_key: [u8; 32],
        proof_blob: Vec<u8>,
        proof_slot: u64,
    ) -> UserDecryptionRequestSolana {
        let identity: SolanaPubkeyBytes = identity_kp.public_key().as_ref().try_into().unwrap();
        let public_key = b"reencryption-public-key".to_vec();
        let nonce = [5u8; 32];
        let context_id = [0u8; 32];
        let start: u64 = 1_000;
        let duration: u64 = 3_600;

        let preimage = solana_user_decrypt_signing_preimage(&SolanaUserDecryptSigningInput {
            contracts_chain_id: CHAIN_ID,
            public_key: &public_key,
            handles: &[handle],
            identity: &identity,
            context_id: &context_id,
            nonce: &nonce,
            allowed_acl_domain_keys: &[DOMAIN],
            start_timestamp: start,
            duration_seconds: duration,
            acl_value_key: &value_key,
            mmr_proof_bytes: &proof_blob,
            proof_slot,
        });
        let signature = identity_kp.sign(&preimage);

        let extra_data = if proof_blob.is_empty() && value_key == [0u8; 32] {
            encode_solana_extra_data_context_only(context_id)
        } else {
            encode_solana_extra_data_mmr_proof(context_id, value_key, proof_slot, &proof_blob)
        };

        let payload = UserDecryptionRequestSolanaPayload {
            userIdentity: FixedBytes::from(identity),
            publicKey: Bytes::from(public_key),
            allowedAclDomainKeys: vec![FixedBytes::from(DOMAIN)],
            requestValidity: RequestValiditySeconds {
                startTimestamp: U256::from(start),
                durationSeconds: U256::from(duration),
            },
            nonce: FixedBytes::from(nonce),
            extraData: Bytes::from(extra_data),
            signature: Bytes::from(signature.as_ref().to_vec()),
        };
        UserDecryptionRequestSolana {
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
        }
    }

    #[test]
    fn signing_preimage_with_mmr_tail_matches_shared_vector() {
        let identity = [0x07u8; 32];
        let nonce = [0x09u8; 32];
        let mut context_id = [0u8; 32];
        context_id[30] = 0x12;
        context_id[31] = 0x34;
        let domain_keys = [[0x01u8; 32], [0x02u8; 32]];
        let public_key = b"public-key-bytes";
        let handles = [[0x03u8; 32], [0xaau8; 32]];
        let acl_value_key = [0x55u8; 32];
        let mmr_proof_bytes = [0x01u8, 0x02, 0x03];
        let proof_slot = 42;

        let preimage = solana_user_decrypt_signing_preimage(&SolanaUserDecryptSigningInput {
            contracts_chain_id: 0xcafe,
            public_key,
            handles: &handles,
            identity: &identity,
            context_id: &context_id,
            nonce: &nonce,
            allowed_acl_domain_keys: &domain_keys,
            start_timestamp: 1000,
            duration_seconds: 3600,
            acl_value_key: &acl_value_key,
            mmr_proof_bytes: &mmr_proof_bytes,
            proof_slot,
        });
        assert_eq!(
            format!("0x{}", alloy::hex::encode(preimage)),
            "0x7a616d612d736f6c616e612d757365722d646563727970742d7632000000000000cafe000000107075626c69632d6b65792d6279746573000000020303030303030303030303030303030303030303030303030303030303030303aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa070707070707070707070707070707070707070707070707070707070707070700000000000000000000000000000000000000000000000000000000000012340909090909090909090909090909090909090909090909090909090909090909000000020101010101010101010101010101010101010101010101010101010101010101020202020202020202020202020202020202020202020202020202020202020200000000000003e80000000000000e105555555555555555555555555555555555555555555555555555555555555555000000000000002a00000003010203"
        );

        let extra_data = encode_solana_extra_data_mmr_proof(
            context_id,
            acl_value_key,
            proof_slot,
            &mmr_proof_bytes,
        );
        assert_eq!(
            format!("0x{}", alloy::hex::encode(extra_data)),
            "0x0300000000000000000000000000000000000000000000000000000000000012345555555555555555555555555555555555555555555555555555555555555555000000000000002a00000003010203"
        );
    }

    #[test]
    fn accepts_valid_signature() {
        let kp = identity_kp(1);
        let request = signed_mmr_request(&kp, h(1), [0u8; 32], Vec::new(), 0);
        let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();
        assert_eq!(auth.allowed_acl_domain_keys, vec![DOMAIN]);
    }

    #[test]
    fn rejects_public_key_substitution() {
        let kp = identity_kp(1);
        let mut request = signed_mmr_request(&kp, h(1), [0u8; 32], Vec::new(), 0);
        request.payload.publicKey = Bytes::from_static(b"attacker-public-key");
        let result = verify_solana_user_decrypt_signature(&request, CHAIN_ID);
        assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))));
    }

    #[test]
    fn rejects_forged_signature() {
        let kp = identity_kp(1);
        let mut request = signed_mmr_request(&kp, h(1), [0u8; 32], Vec::new(), 0);
        let attacker = identity_kp(99);
        let forged = attacker.sign(b"any-bytes");
        request.payload.signature = Bytes::from(forged.as_ref().to_vec());
        let result = verify_solana_user_decrypt_signature(&request, CHAIN_ID);
        assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))));
    }

    #[test]
    fn rejects_wrong_chain_id_binding() {
        let kp = identity_kp(1);
        let request = signed_mmr_request(&kp, h(1), [0u8; 32], Vec::new(), 0);
        let result = verify_solana_user_decrypt_signature(&request, CHAIN_ID + 1);
        assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))));
    }

    // (1) HISTORICAL ACCEPT
    #[test]
    fn historical_accept() {
        let kp = identity_kp(11);
        let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
        let mut l = lineage(h(10), &[owner]);
        l.rotate(h(11));
        let proof = l.proof(0);
        let blob = proof_blob(MMR_MODE_HISTORICAL, &proof);

        let request = signed_mmr_request(&kp, h(10), l.value_key(), blob, l.acl.leaf_count);
        let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();

        let verifier = SolanaAclVerifier::new(HOST);
        dispatch_solana_mmr_proof(&verifier, l.account, HOST, &l.acl, h(10), &auth)
            .expect("historical decrypt must authorize");
    }

    #[test]
    fn trailing_bytes_after_mmr_proof_rejected() {
        let kp = identity_kp(11);
        let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
        let mut l = lineage(h(10), &[owner]);
        l.rotate(h(11));
        let proof = l.proof(0);
        let mut blob = proof_blob(MMR_MODE_HISTORICAL, &proof);
        blob.extend_from_slice(&[0xde, 0xad, 0xbe, 0xef]);

        let request = signed_mmr_request(&kp, h(10), l.value_key(), blob, l.acl.leaf_count);
        let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();

        let verifier = SolanaAclVerifier::new(HOST);
        let err = dispatch_solana_mmr_proof(&verifier, l.account, HOST, &l.acl, h(10), &auth)
            .expect_err("proof blobs with trailing bytes must be rejected");
        match err {
            ProcessingError::Irrecoverable(e) => {
                assert!(e.to_string().contains("trailing byte"), "got: {e}");
            }
            other => panic!("trailing proof bytes must be Irrecoverable, got {other:?}"),
        }
    }

    // (2) PUBLIC ACCEPT
    #[test]
    fn public_accept() {
        let kp = identity_kp(12);
        let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
        let mut l = lineage(h(20), &[owner]);
        l.mark_public();
        l.rotate(h(21));
        let proof = l.proof(0);
        let blob = proof_blob(MMR_MODE_PUBLIC, &proof);

        let request = signed_mmr_request(&kp, h(20), l.value_key(), blob, l.acl.leaf_count);
        let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();

        let verifier = SolanaAclVerifier::new(HOST);
        dispatch_solana_mmr_proof(&verifier, l.account, HOST, &l.acl, h(20), &auth)
            .expect("public decrypt must authorize");
    }

    #[test]
    fn public_decrypt_path_accepts_only_public_mode() {
        let kp = identity_kp(12);
        let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
        let mut l = lineage(h(20), &[owner]);
        l.mark_public();
        l.rotate(h(21));
        let public_blob = proof_blob(MMR_MODE_PUBLIC, &l.proof(0));
        let historical_blob = proof_blob(MMR_MODE_HISTORICAL, &l.proof(0));

        let verifier = SolanaAclVerifier::new(HOST);
        dispatch_solana_public_mmr_proof(
            &verifier,
            l.account,
            HOST,
            &l.acl,
            h(20),
            l.acl.leaf_count,
            &public_blob,
        )
        .expect("public decrypt must authorize with a mode-0x02 PublicDecryptLeaf proof");

        let err = dispatch_solana_public_mmr_proof(
            &verifier,
            l.account,
            HOST,
            &l.acl,
            h(20),
            l.acl.leaf_count,
            &historical_blob,
        )
        .expect_err("public decrypt must reject mode-0x01 proof blobs");
        assert!(matches!(err, ProcessingError::Irrecoverable(_)));
    }

    #[test]
    fn public_decrypt_path_rejects_rotated_or_non_public_handle() {
        let kp = identity_kp(12);
        let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
        let mut public_lineage = lineage(h(20), &[owner]);
        public_lineage.mark_public();
        public_lineage.rotate(h(21));
        let public_blob = proof_blob(MMR_MODE_PUBLIC, &public_lineage.proof(0));
        let verifier = SolanaAclVerifier::new(HOST);

        let err = dispatch_solana_public_mmr_proof(
            &verifier,
            public_lineage.account,
            HOST,
            &public_lineage.acl,
            h(21),
            public_lineage.acl.leaf_count,
            &public_blob,
        )
        .expect_err("a PublicDecryptLeaf for the old handle must not authorize the rotated handle");
        assert!(matches!(err, ProcessingError::Recoverable(_)));

        let mut non_public_lineage = lineage(h(30), &[owner]);
        non_public_lineage.rotate(h(31));
        let historical_leaf_blob = proof_blob(MMR_MODE_PUBLIC, &non_public_lineage.proof(0));
        let err = dispatch_solana_public_mmr_proof(
            &verifier,
            non_public_lineage.account,
            HOST,
            &non_public_lineage.acl,
            h(30),
            non_public_lineage.acl.leaf_count,
            &historical_leaf_blob,
        )
        .expect_err("a historical-access leaf must not authorize public decrypt");
        assert!(matches!(err, ProcessingError::Recoverable(_)));
    }

    // (3) STALE TERMINAL
    #[test]
    fn stale_merged_proof_is_terminal() {
        let kp = identity_kp(11);
        let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
        let mut l = lineage(h(10), &[owner]);
        l.rotate(h(11));
        l.rotate(h(12));
        l.rotate(h(13));
        assert_eq!(l.acl.leaf_count, 3);
        let blob = proof_blob(MMR_MODE_HISTORICAL, &l.proof(2));
        let proof_slot = 3u64;
        l.rotate(h(14));
        assert_eq!(l.acl.leaf_count, 4);

        let request = signed_mmr_request(&kp, h(12), l.value_key(), blob, proof_slot);
        let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();

        let verifier = SolanaAclVerifier::new(HOST);
        let err = dispatch_solana_mmr_proof(&verifier, l.account, HOST, &l.acl, h(12), &auth)
            .expect_err("a stale (merged) proof must be rejected");
        match err {
            ProcessingError::Irrecoverable(e) => {
                let msg = e.to_string();
                assert!(msg.contains("stale and immutable"), "got: {msg}");
                assert!(msg.contains("leaf_count=3"), "got: {msg}");
                assert!(msg.contains("leaf_count=4"), "got: {msg}");
            }
            other => {
                panic!("a stale (merged) proof must be terminal, got {other:?}")
            }
        }
    }

    #[test]
    fn invalid_proof_at_live_leaf_count_uses_normal_recoverable_budget() {
        let err = classify_mmr_verification_failure(
            crate::core::solana_acl::SolanaAclVerificationError::HistoricalAccessProofInvalid,
            4,
            4,
        );

        match err {
            ProcessingError::Recoverable(e) => {
                let msg = e.to_string();
                assert!(msg.contains("proof leaf_count=4"), "got: {msg}");
                assert!(msg.contains("live confirmed leaf_count=4"), "got: {msg}");
                assert!(msg.contains("normal attempt budget"), "got: {msg}");
            }
            other => {
                panic!("equal-count confirmed fork disagreement must be retryable, got {other:?}")
            }
        }
    }

    #[test]
    fn proof_ahead_of_live_state_uses_normal_recoverable_budget() {
        let err = classify_mmr_verification_failure(
            crate::core::solana_acl::SolanaAclVerificationError::PublicDecryptProofInvalid,
            2,
            1,
        );

        match err {
            ProcessingError::Recoverable(e) => {
                let msg = e.to_string();
                assert!(msg.contains("proof leaf_count=2"), "got: {msg}");
                assert!(msg.contains("live confirmed leaf_count=1"), "got: {msg}");
                assert!(msg.contains("normal attempt budget"), "got: {msg}");
            }
            other => panic!("a proof ahead of confirmed state must be recoverable, got {other:?}"),
        }
    }

    // (3b) VERIFY-FIRST ACCEPTS COUNT DRIFT WITHOUT A MERGE
    #[test]
    fn valid_proof_survives_count_drift_without_merge() {
        let kp = identity_kp(13);
        let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
        let mut l = lineage(h(30), &[owner]);
        l.rotate(h(31));
        l.rotate(h(32));
        assert_eq!(l.acl.leaf_count, 2);
        let blob = proof_blob(MMR_MODE_HISTORICAL, &l.proof(0));
        let proof_slot = 2u64;
        l.rotate(h(33));
        assert_eq!(l.acl.leaf_count, 3);

        let request = signed_mmr_request(&kp, h(30), l.value_key(), blob, proof_slot);
        let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();

        let verifier = SolanaAclVerifier::new(HOST);
        dispatch_solana_mmr_proof(&verifier, l.account, HOST, &l.acl, h(30), &auth)
            .expect("a proof that still verifies against live peaks must be accepted");
    }

    #[test]
    fn ahead_count_domain_and_canonical_failures_remain_terminal() {
        let kp = identity_kp(14);
        let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
        let mut l = lineage(h(70), &[owner]);
        l.rotate(h(71));
        let proof_slot = l.acl.leaf_count + 1;
        let blob = proof_blob(MMR_MODE_HISTORICAL, &l.proof(0));
        assert!(proof_slot > l.acl.leaf_count);

        let request = signed_mmr_request(&kp, h(70), l.value_key(), blob, proof_slot);
        let mut auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();
        auth.allowed_acl_domain_keys = vec![[0x99u8; 32]];

        let verifier = SolanaAclVerifier::new(HOST);
        let err = dispatch_solana_mmr_proof(&verifier, l.account, HOST, &l.acl, h(70), &auth)
            .expect_err("an ahead count must not make a domain failure retryable");
        assert!(matches!(err, ProcessingError::Irrecoverable(_)));

        let canonical_err = classify_mmr_verification_failure(
            crate::core::solana_acl::SolanaAclVerificationError::NonCanonicalEncryptedValueAcl,
            2,
            1,
        );
        assert!(
            matches!(canonical_err, ProcessingError::Irrecoverable(_)),
            "an ahead count must not make a canonical-account failure retryable"
        );
    }

    // (3c) CURRENT ACCEPT
    #[test]
    fn current_lineage_accept() {
        let kp = identity_kp(21);
        let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
        let l = lineage(h(40), &[owner]);

        let request = signed_mmr_request(&kp, h(40), l.value_key(), Vec::new(), 0);
        let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();
        assert!(auth.mmr_proof_bytes.is_empty());
        assert_ne!(auth.acl_value_key, [0u8; 32]);

        let verifier = SolanaAclVerifier::new(HOST);
        dispatch_solana_current(&verifier, l.account, HOST, &decoded(&l), h(40), &auth)
            .expect("current-lineage decrypt of the live handle by a subject must authorize");
    }

    // (3d) CURRENT NON-SUBJECT REJECTED
    #[test]
    fn current_lineage_non_subject_rejected() {
        let kp = identity_kp(22);
        let other: SolanaPubkeyBytes = [99u8; 32];
        let l = lineage(h(50), &[other]);

        let request = signed_mmr_request(&kp, h(50), l.value_key(), Vec::new(), 0);
        let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();

        let verifier = SolanaAclVerifier::new(HOST);
        let err = dispatch_solana_current(&verifier, l.account, HOST, &decoded(&l), h(50), &auth)
            .expect_err("a non-subject must not be authorized for the current handle");
        assert!(matches!(err, ProcessingError::Recoverable(_)));
    }

    // (3e) CURRENT REJECTS A ROTATED-AWAY HANDLE
    #[test]
    fn current_lineage_rejects_rotated_away_handle() {
        let kp = identity_kp(23);
        let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
        let mut l = lineage(h(60), &[owner]);
        l.rotate(h(61));

        let request = signed_mmr_request(&kp, h(60), l.value_key(), Vec::new(), 0);
        let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();

        let verifier = SolanaAclVerifier::new(HOST);
        let err = dispatch_solana_current(&verifier, l.account, HOST, &decoded(&l), h(60), &auth)
            .expect_err("a rotated-away handle must not authorize via the no-proof current path");
        assert!(matches!(err, ProcessingError::Recoverable(_)));
    }

    // (4) V1-SIGNED REJECTED UNDER V2
    #[test]
    fn v1_signature_rejected_under_v2() {
        let kp = identity_kp(11);
        let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
        let mut l = lineage(h(10), &[owner]);
        l.rotate(h(11));
        let proof = l.proof(0);
        let blob = proof_blob(MMR_MODE_HISTORICAL, &proof);
        let value_key = l.value_key();

        assert_eq!(
            SOLANA_USER_DECRYPT_DOMAIN_TAG,
            b"zama-solana-user-decrypt-v2"
        );

        let identity: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
        let public_key = b"reencryption-public-key".to_vec();
        let nonce = [5u8; 32];
        let context_id = [0u8; 32];
        let (start, duration) = (1_000u64, 3_600u64);
        let mut v1_preimage = b"zama-solana-user-decrypt-v1".to_vec();
        v1_preimage.extend_from_slice(&CHAIN_ID.to_be_bytes());
        v1_preimage.extend_from_slice(&(public_key.len() as u32).to_be_bytes());
        v1_preimage.extend_from_slice(&public_key);
        v1_preimage.extend_from_slice(&1u32.to_be_bytes());
        v1_preimage.extend_from_slice(&h(10));
        v1_preimage.extend_from_slice(&identity);
        v1_preimage.extend_from_slice(&context_id);
        v1_preimage.extend_from_slice(&nonce);
        v1_preimage.extend_from_slice(&1u32.to_be_bytes());
        v1_preimage.extend_from_slice(&DOMAIN);
        v1_preimage.extend_from_slice(&start.to_be_bytes());
        v1_preimage.extend_from_slice(&duration.to_be_bytes());
        let v1_signature = kp.sign(&v1_preimage);

        let extra_data =
            encode_solana_extra_data_mmr_proof(context_id, value_key, l.acl.leaf_count, &blob);
        let payload = UserDecryptionRequestSolanaPayload {
            userIdentity: FixedBytes::from(identity),
            publicKey: Bytes::from(public_key),
            allowedAclDomainKeys: vec![FixedBytes::from(DOMAIN)],
            requestValidity: RequestValiditySeconds {
                startTimestamp: U256::from(start),
                durationSeconds: U256::from(duration),
            },
            nonce: FixedBytes::from(nonce),
            extraData: Bytes::from(extra_data),
            signature: Bytes::from(v1_signature.as_ref().to_vec()),
        };
        let request = UserDecryptionRequestSolana {
            decryptionId: U256::from(1u64),
            snsCtMaterials: vec![SnsCiphertextMaterial {
                ctHandle: FixedBytes::from(h(10)),
                ..Default::default()
            }],
            handles: vec![HandleEntry {
                handle: FixedBytes::from(h(10)),
                contractAddress: Address::ZERO,
                ownerAddress: Address::ZERO,
            }],
            payload,
        };

        let result = verify_solana_user_decrypt_signature(&request, CHAIN_ID);
        assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))));
    }

    // (5) PROOF-FIELD BINDING
    #[test]
    fn proof_fields_bound_into_signature() {
        let kp = identity_kp(11);
        let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
        let mut l = lineage(h(10), &[owner]);
        l.rotate(h(11));
        let blob = proof_blob(MMR_MODE_HISTORICAL, &l.proof(0));

        let mut req = signed_mmr_request(&kp, h(10), l.value_key(), blob.clone(), l.acl.leaf_count);
        let mut tampered = blob.clone();
        *tampered.last_mut().unwrap() ^= 0xff;
        req.payload.extraData = Bytes::from(encode_solana_extra_data_mmr_proof(
            [0u8; 32],
            l.value_key(),
            l.acl.leaf_count,
            &tampered,
        ));
        assert!(matches!(
            verify_solana_user_decrypt_signature(&req, CHAIN_ID),
            Err(ProcessingError::Irrecoverable(_))
        ));

        let mut req = signed_mmr_request(&kp, h(10), l.value_key(), blob.clone(), l.acl.leaf_count);
        req.payload.extraData = Bytes::from(encode_solana_extra_data_mmr_proof(
            [0u8; 32],
            [0x99u8; 32],
            l.acl.leaf_count,
            &blob,
        ));
        assert!(matches!(
            verify_solana_user_decrypt_signature(&req, CHAIN_ID),
            Err(ProcessingError::Irrecoverable(_))
        ));

        let mut req = signed_mmr_request(&kp, h(10), l.value_key(), blob.clone(), l.acl.leaf_count);
        req.payload.extraData = Bytes::from(encode_solana_extra_data_mmr_proof(
            [0u8; 32],
            l.value_key(),
            l.acl.leaf_count + 1,
            &blob,
        ));
        assert!(matches!(
            verify_solana_user_decrypt_signature(&req, CHAIN_ID),
            Err(ProcessingError::Irrecoverable(_))
        ));
    }

    // (6) CURRENT-ACL → MMR PATH-CONFUSION
    #[test]
    fn current_acl_mmr_proof_injection_rejected() {
        let kp = identity_kp(11);
        let l = lineage(h(10), &[kp.public_key().as_ref().try_into().unwrap()]);

        let mut req = signed_mmr_request(&kp, h(10), [0u8; 32], Vec::new(), 0);

        let injected = proof_blob(MMR_MODE_HISTORICAL, &l.proof_for_empty());
        req.payload.extraData = Bytes::from(encode_solana_extra_data_mmr_proof(
            [0u8; 32], [0u8; 32], 0, &injected,
        ));

        let result = verify_solana_user_decrypt_signature(&req, CHAIN_ID);
        assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))));
    }

    // MULTI-HANDLE GUARD
    #[test]
    fn mmr_proof_multi_handle_rejected() {
        let two = [h(10), h(11)];
        let err = require_single_handle(&two)
            .expect_err("a two-handle MMR-proof request must be rejected");
        match err {
            ProcessingError::Irrecoverable(e) => {
                assert!(e.to_string().contains("exactly one handle"));
            }
            other => panic!("multi-handle MMR request must be Irrecoverable, got {other:?}"),
        }
        assert_eq!(require_single_handle(&[h(10)]).unwrap(), h(10));
        assert!(matches!(
            require_single_handle(&[]),
            Err(ProcessingError::Irrecoverable(_))
        ));
    }

    // (7) SIBLINGS CAP
    #[test]
    fn siblings_cap_rejected() {
        let kp = identity_kp(11);
        let oversized = MmrProof {
            leaf_index: 0,
            siblings: vec![[0u8; 32]; MAX_MMR_SIBLINGS + 1],
        };
        let blob = proof_blob(MMR_MODE_HISTORICAL, &oversized);
        let l = lineage(h(10), &[kp.public_key().as_ref().try_into().unwrap()]);
        let request = signed_mmr_request(&kp, h(10), l.value_key(), blob, 0);
        let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();

        let verifier = SolanaAclVerifier::new(HOST);
        let err = dispatch_solana_mmr_proof(&verifier, l.account, HOST, &l.acl, h(10), &auth)
            .expect_err("oversized sibling list must be rejected");
        assert!(matches!(err, ProcessingError::Irrecoverable(_)));
    }

    // Unknown mode byte
    #[test]
    fn unknown_mode_rejected() {
        let kp = identity_kp(11);
        let l = lineage(h(10), &[kp.public_key().as_ref().try_into().unwrap()]);
        let blob = proof_blob(0x09, &l.proof_for_empty());
        let request = signed_mmr_request(&kp, h(10), l.value_key(), blob, 0);
        let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();
        let verifier = SolanaAclVerifier::new(HOST);
        let err = dispatch_solana_mmr_proof(&verifier, l.account, HOST, &l.acl, h(10), &auth)
            .expect_err("unknown mode must be rejected");
        assert!(matches!(err, ProcessingError::Irrecoverable(_)));
    }
}
