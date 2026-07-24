//! CPI wrapper around the host `verify_public_decrypt` instruction.

use anchor_lang::{prelude::*, solana_program::program::get_return_data};
use zama_host::{cpi, program::ZamaHost, HostConfig};

use crate::ConfidentialTokenError;

/// Inputs required to consume the stateless host public-decrypt verifier.
pub struct VerifyPublicDecrypt<'a, 'info> {
    /// Handle the caller pinned; the handle the host proves public must equal this.
    pub expected_handle: [u8; 32],
    /// 32-byte big-endian `uint256` cleartext the KMS certificate signs over.
    pub cleartext: [u8; 32],
    /// KMS threshold signatures over the `PublicDecryptVerification` certificate.
    pub signatures: Vec<[u8; 65]>,
    /// Signed `extra_data` committing the KMS context id (EVM `_extractContextId` parity).
    pub extra_data: Vec<u8>,
    /// MMR public-leaf inclusion proof for `expected_handle` against the encrypted value account's current peaks.
    pub proof: zama_host::instructions::MmrInclusionProof,
    /// ValueAccount whose peaks the inclusion proof is checked against.
    pub encrypted_value: AccountInfo<'info>,
    /// Host config carrying the current KMS context id and gateway EIP-712 domain.
    pub host_config: &'a Account<'info, HostConfig>,
    /// KMS context PDA for the id the certificate commits to (any live context).
    pub kms_context: AccountInfo<'info>,
    /// ZamaHost program account.
    pub zama_program: &'a Program<'info, ZamaHost>,
}

/// CPIs the stateless host `verify_public_decrypt`, then reads the `(handle, cleartext, context_id)`
/// it wrote to `return_data`, asserting the return came from ZamaHost and that the proven handle
/// equals the caller-pinned `expected_handle`. Returns the certified 32-byte cleartext. The host
/// verifies the KMS certificate against the context the cert names (any live, non-destroyed context)
/// and the MMR proof against the encrypted value account's peaks; this wrapper adds only the return-data integrity +
/// pinned-handle checks. It deliberately does NOT constrain the returned context id: token
/// disclosure and redemption accept any live context, matching EVM's valid-until-destroyed rotation
/// grace (`destroy_kms_context` is the revocation lever). The verified id is available at
/// `return_data[64..72]` (little-endian u64) for a consumer that wants a current-only policy.
pub(crate) fn verify_public_decrypt(request: VerifyPublicDecrypt<'_, '_>) -> Result<[u8; 32]> {
    let expected_handle = request.expected_handle;
    cpi::verify_public_decrypt(
        CpiContext::new(
            request.zama_program.key(),
            cpi::accounts::VerifyPublicDecrypt {
                host_config: request.host_config.to_account_info(),
                kms_context: request.kms_context,
                encrypted_value: request.encrypted_value,
            },
        ),
        expected_handle,
        request.cleartext,
        request.signatures,
        request.extra_data,
        request.proof,
    )?;

    let (program_id, data) =
        get_return_data().ok_or(ConfidentialTokenError::VerifierReturnDataInvalid)?;
    require_keys_eq!(
        program_id,
        zama_host::ID,
        ConfidentialTokenError::VerifierReturnDataInvalid
    );
    require!(
        data.len() == 72,
        ConfidentialTokenError::VerifierReturnDataInvalid
    );
    let mut returned_handle = [0u8; 32];
    returned_handle.copy_from_slice(&data[..32]);
    require!(
        returned_handle == expected_handle,
        ConfidentialTokenError::DisclosedHandleMismatch
    );
    let mut returned_cleartext = [0u8; 32];
    returned_cleartext.copy_from_slice(&data[32..64]);
    // `data[64..72]` is the verified context id (little-endian u64); token disclosure and redemption
    // accept any live context (EVM parity), so it is not constrained here.
    Ok(returned_cleartext)
}
