//! Solana public-decryption ACL check, and the per-host handle it reads through.
//!
//! Public-ness has no live on-chain flag: it is provable only by a `PublicDecryptLeaf` MMR leaf
//! carried in the version-`0x03` `extraData` container. This module owns that check and the MMR
//! surface it reaches — the account fetch at `confirmed`, the proof-blob decoder, the mode byte,
//! and the verify-first failure classification.
//!
//! It was re-homed here out of the v0 user-decrypt module when that module was deleted: public
//! decrypt has no other authorization path, so its surface is relocated, never removed. The
//! external suite `kms-worker/tests/solana_public_decrypt_carrier.rs` pins it. User-decrypt
//! authorization no longer lives here at all — it goes through `core::solana::pipeline`.

use crate::core::{
    event_processor::ProcessingError,
    solana::deployment::DeploymentIdentity,
    solana::snapshot::RpcHostStateReader,
    solana_acl::{HandleBytes, SolanaAclVerifier, SolanaPubkeyBytes},
    solana_encrypted_value_acl::{
        DecodedEncryptedValueAcl, EncryptedValueTarget, decode_encrypted_value_acl,
        encrypted_value_acl_address,
    },
    solana_v2_fetcher::SolanaV2Fetcher,
};
use anyhow::anyhow;
use borsh::BorshDeserialize;
use connector_utils::types::solana_extra_data::parse_solana_mmr_proof_extra_data;
use solana_pubkey::Pubkey;
use zama_solana_acl::{EncryptedValue, MmrProof};

/// Transport-blob mode byte for a historical-access MMR proof.
pub const MMR_PROOF_MODE_HISTORICAL: u8 = 0x01;
/// Transport-blob mode byte for a public-decrypt MMR proof.
///
/// Part of the temporary public-decrypt proof carrier (see
/// `connector_utils::types::solana_extra_data::SOLANA_EXTRA_DATA_VERSION_MMR_PROOF` for its
/// ownership and removal condition).
pub const MMR_PROOF_MODE_PUBLIC: u8 = 0x02;
/// Upper bound on `MmrProof::siblings` accepted from an untrusted request, matching the MMR's
/// `u64` height ceiling (`mmr.rs` iterates heights `0..64`); bounds the decode-time allocation.
pub const MAX_MMR_SIBLINGS: usize = 64;

/// Per-chain Solana host: the deployment identity authorization is decided against, a
/// multi-account snapshot reader for the user-decrypt pipeline, and a single-account
/// `confirmed`-commitment fetcher for the public-decrypt path. Both readers hit the same host
/// RPC; the pipeline needs the atomic multi-account snapshot, public decrypt needs one account.
#[derive(Clone, Debug)]
pub struct SolanaHost {
    /// Which program and cluster this Connector authorizes against.
    pub deployment: DeploymentIdentity,
    /// The atomic `getMultipleAccounts` snapshot reader the user-decrypt pipeline drives.
    pub reader: RpcHostStateReader,
    /// The single-account `getAccountInfo` fetcher the public-decrypt path reads through.
    pub fetcher: SolanaV2Fetcher,
}

/// Enforces the single-handle scope of public decrypt: a `PublicDecryptLeaf` proof authorizes
/// exactly one handle. Pure so the rejection is unit-testable without a host.
pub fn require_single_handle(handles: &[HandleBytes]) -> Result<HandleBytes, ProcessingError> {
    match handles {
        [single] => Ok(*single),
        other => Err(ProcessingError::Irrecoverable(anyhow!(
            "Solana EncryptedValue public-decrypt requires exactly one handle per request, got {}",
            other.len()
        ))),
    }
}

/// Fetches the `EncryptedValue` account for `encrypted_value_id` at `confirmed` commitment and
/// decodes it. Never a snapshot: every call re-reads the live account, which is what lets the
/// public-decrypt proof verify against the LIVE peaks.
async fn fetch_encrypted_value_acl(
    host: &SolanaHost,
    encrypted_value_id: [u8; 32],
) -> Result<(SolanaPubkeyBytes, DecodedEncryptedValueAcl), ProcessingError> {
    let program_id = host.deployment.program_id();
    let (account_key, _bump) = encrypted_value_acl_address(program_id, encrypted_value_id);

    let account = host
        .fetcher
        .get_account(&account_key)
        .await
        .map_err(ProcessingError::Recoverable)?
        .ok_or_else(|| {
            ProcessingError::Recoverable(anyhow!(
                "Solana EncryptedValue encrypted value account {} not found at confirmed commitment",
                Pubkey::new_from_array(account_key),
            ))
        })?;

    if account.owner != program_id {
        return Err(ProcessingError::Irrecoverable(anyhow!(
            "Solana EncryptedValue encrypted value account {} is owned by {}, expected ZamaHost program {}",
            Pubkey::new_from_array(account_key),
            Pubkey::new_from_array(account.owner),
            Pubkey::new_from_array(program_id),
        )));
    }

    let acl = decode_encrypted_value_acl(&account.data).map_err(|e| {
        ProcessingError::Irrecoverable(anyhow!(
            "failed to decode EncryptedValue encrypted value account: {e}"
        ))
    })?;
    Ok((account_key, acl))
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
    if mode != MMR_PROOF_MODE_PUBLIC {
        return Err(ProcessingError::Irrecoverable(anyhow!(
            "Solana public decryption requires MMR proof mode {MMR_PROOF_MODE_PUBLIC:#04x}, got {mode:#04x}"
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
    if proof_invalid && proof_leaf_count == live_leaf_count {
        ProcessingError::Recoverable(anyhow!(
            "Solana MMR proof does not verify against an equal-count KMS confirmed view \
             (classification=confirmed_equal_count): proof leaf_count={proof_leaf_count}, live \
             confirmed leaf_count={live_leaf_count}; retrying within the configured decryption \
             attempt budget while confirmed views converge ({error})"
        ))
    } else if proof_invalid && proof_leaf_count > live_leaf_count {
        ProcessingError::Recoverable(anyhow!(
            "Solana MMR proof is ahead of the KMS confirmed view \
             (classification=confirmed_proof_ahead): proof leaf_count={proof_leaf_count}, live \
             confirmed leaf_count={live_leaf_count}; retrying within the configured decryption \
             attempt budget while the KMS view catches up ({error})"
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

/// Solana public-decryption ACL check. There is no live "is public" flag: public-ness is only
/// provable via a `PublicDecryptLeaf` MMR leaf carried in the `0x03` `extraData` container.
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

    let verifier = SolanaAclVerifier::new(host.deployment.program_id());
    let (account_key, decoded) = fetch_encrypted_value_acl(host, extra.acl_value_key).await?;
    dispatch_solana_public_mmr_proof(
        &verifier,
        account_key,
        host.deployment.program_id(),
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
