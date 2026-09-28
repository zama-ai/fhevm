//! Pre-check for the host-generic Solana arm of v3 user decryption: the permit signature, then
//! the deployment the permit names.
//!
//! On the EVM arms the gateway contract is a backstop the relayer's pre-check merely anticipates:
//! the contract verifies the EIP-712 signature inside the transaction, so a bad one reverts
//! before the fee is collected. The host-generic entry has no such backstop. The permit and its
//! ed25519 signature ride inside `solanaRequest`, a field the gateway never reads; the gateway
//! checks the validity window and nothing else about the permit. Only the KMS connectors verify
//! the signature, each independently, and that check stays mandatory and authoritative.
//!
//! The same holds for the deployment the permit is signed for. A permit names a host chain and a
//! zama-host program, and every handle embeds the chain it was written on. The connector refuses
//! a request whose three values disagree — before it reads any state — but by then the gateway
//! transaction is sent and the fee is spent, and the refusal never reaches the relayer, so the
//! job dies by timeout. On the EVM arm the same mismatch cannot get past the pre-check: the
//! signature is verified under a domain built from the handles' chain, so a permit for another
//! chain is a bad signature. This stage gives the Solana arm the same door: a permit for a chain
//! or a program this relayer does not serve, or handles from another chain than the permit
//! names, is refused here, field by field, at the cost of no transaction.
//!
//! What this stage buys is what the EIP-712 pre-check buys: a detectably bad request is refused
//! here, with a specific error, instead of costing a readiness round and surfacing later as an
//! opaque failure.
//!
//! The permit checks run on the canonical request bytes the relayer is about to forward, through
//! the connector's own decode path (`decode_solana_request`, then `PermitFields::decode`, then
//! `verify_signature`), so what is verified is exactly what will be read downstream. The handle
//! checks run on the request's typed `ct_handles`: the handles the gateway transaction carries,
//! 32 bytes by type, and the ones the connector holds the blob's entries equal to.

use crate::host::handle_chain_id::extract_chain_id_from_u256;
use crate::host::signature_prechecker::{PreCheckSigner, SigPreCheckError};
use alloy::primitives::U256;
use std::collections::HashMap;
use tracing::warn;
use zama_solana_permit::{verify_signature, PermitFields, Signature, SIGNATURE_LEN};
use zama_solana_request::decode_solana_request;

/// The zama-host program this relayer serves on each configured Solana host chain, keyed by
/// chain id: the `acl_address` of the chain's `host_chains` entry.
pub type SolanaDeployments = HashMap<u64, [u8; 32]>;

/// Verifies the permit signature carried inside a canonical `solanaRequest` blob, then that the
/// permit names a deployment this relayer serves and handles from that deployment's chain.
///
/// Pure: no state beyond `deployments`, no I/O. A wrong signature — wrong length, or one that
/// does not verify over the envelope rebuilt from the permit fields — is
/// [`SigPreCheckError::Invalid`]. A permit for a chain or program not in `deployments`, or a
/// handle embedding another chain than the permit names, is [`SigPreCheckError::Deployment`],
/// keyed by the payload field at fault. Both are the caller's fault.
///
/// A blob or permit that cannot be read passes with a warning. Admission produced these bytes a
/// moment ago through the same crates that read them here, so a re-decode failure is this
/// relayer's own defect, and an advisory check does not refuse users on its own defects. The host
/// ACL pre-check applies the same policy to the same blob.
pub fn verify_solana_permit(
    solana_request: &[u8],
    ct_handles: &[U256],
    deployments: &SolanaDeployments,
) -> Result<(), SigPreCheckError> {
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
    })?;

    // The deployment, in the connector's order: the chain, the program on it, then the handles
    // against the chain. Checked after the signature so a forged permit is reported as forged,
    // not as misaddressed.
    let chain_id = permit.chain_id();
    let Some(program_id) = deployments.get(&chain_id) else {
        return Err(SigPreCheckError::Deployment {
            field: "chainId".to_string(),
            issue: format!("{chain_id} is not a Solana host chain this relayer serves"),
        });
    };
    let signed_program_id = permit.verifying_program_id().as_bytes();
    if signed_program_id != program_id {
        return Err(SigPreCheckError::Deployment {
            field: "verifyingProgramId".to_string(),
            issue: format!(
                "names program {}, the zama-host program on chain {chain_id} is {}",
                solana_pubkey::Pubkey::new_from_array(*signed_program_id),
                solana_pubkey::Pubkey::new_from_array(*program_id),
            ),
        });
    }
    for (index, handle) in ct_handles.iter().enumerate() {
        let embedded_chain_id = extract_chain_id_from_u256(handle);
        if embedded_chain_id != chain_id {
            return Err(SigPreCheckError::Deployment {
                field: format!("handles[{index}].handle"),
                issue: format!(
                    "belongs to host chain {embedded_chain_id}, the permit names chain {chain_id}"
                ),
            });
        }
    }

    Ok(())
}
