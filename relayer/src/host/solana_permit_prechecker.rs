//! Signature pre-check for the host-generic Solana arm of v3 user decryption.
//!
//! On the EVM arms the gateway contract is a backstop the relayer's pre-check merely anticipates:
//! the contract verifies the EIP-712 signature inside the transaction, so a bad one reverts
//! before the fee is collected. The host-generic entry has no such backstop. The permit and its
//! ed25519 signature ride inside `solanaRequest`, a field the gateway never reads; the gateway
//! checks the validity window and nothing else about the permit. Only the KMS connectors verify
//! the signature, each independently, and that check stays mandatory and authoritative.
//!
//! What this stage buys is what the EIP-712 pre-check buys: a detectably bad signature is refused
//! here, with a specific error, instead of costing a readiness round and surfacing later as an
//! opaque failure.
//!
//! The check runs on the canonical request bytes the relayer is about to forward, through the
//! connector's own decode path (`decode_solana_request`, then `PermitFields::decode`, then
//! `verify_signature`), so what is verified is exactly what will be read downstream.

use crate::host::signature_prechecker::{PreCheckSigner, SigPreCheckError};
use tracing::warn;
use zama_solana_permit::{verify_signature, PermitFields, Signature, SIGNATURE_LEN};
use zama_solana_request::decode_solana_request;

/// Verifies the permit signature carried inside a canonical `solanaRequest` blob.
///
/// Pure: no state, no I/O. A wrong signature — wrong length, or one that does not verify over the
/// envelope rebuilt from the permit fields — is [`SigPreCheckError::Invalid`], the caller's fault.
///
/// A blob or permit that cannot be read passes with a warning. Admission produced these bytes a
/// moment ago through the same crates that read them here, so a re-decode failure is this
/// relayer's own defect, and an advisory check does not refuse users on its own defects. The host
/// ACL pre-check applies the same policy to the same blob.
pub fn verify_solana_permit(solana_request: &[u8]) -> Result<(), SigPreCheckError> {
    let wire = match decode_solana_request(solana_request) {
        Ok(wire) => wire,
        Err(error) => {
            warn!(
                error = %error,
                "Solana signature pre-check could not re-decode the request blob; passing"
            );
            return Ok(());
        }
    };
    let permit = match PermitFields::decode(&wire.permit) {
        Ok(permit) => permit,
        Err(error) => {
            warn!(
                error = %error,
                "Solana signature pre-check could not re-decode the permit fields; passing"
            );
            return Ok(());
        }
    };
    let signer = PreCheckSigner::Solana(solana_pubkey::Pubkey::new_from_array(
        *permit.user_pubkey().as_bytes(),
    ));

    let signature_bytes: [u8; SIGNATURE_LEN] = match wire.signature.as_slice().try_into() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Err(SigPreCheckError::Invalid {
                signer,
                reason: format!(
                    "ed25519 signature is {} bytes, expected {SIGNATURE_LEN}",
                    wire.signature.len()
                ),
            });
        }
    };

    // `verify_signature` rebuilds the envelope from the typed fields, so what is checked is the
    // text the wallet displayed. It reports a structurally unusable pubkey (non-canonical or
    // small-order) as its own failure; both are refusals of this signature.
    verify_signature(&permit, &Signature::new(signature_bytes)).map_err(|error| {
        SigPreCheckError::Invalid {
            signer,
            reason: error.to_string(),
        }
    })
}
