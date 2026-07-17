//! Stateless public-decrypt verifier (fhevm-internal#1704).
//!
//! `verify_public_decrypt` is a pull-oracle: a caller (usually via CPI) brings a KMS
//! `PublicDecryptVerification` certificate plus an MMR public-leaf inclusion proof in its own
//! transaction, this instruction verifies both against on-chain state, and returns the proven
//! `(handle, cleartext)` through `return_data`. It creates nothing, mutates nothing, emits nothing,
//! and needs no signer — everything it asserts is signed off-chain (the cert) or committed on-chain
//! (the sealed public-decrypt leaf). Act-once and timeout live in the consuming app's own state
//! machine, exactly as an EVM app tracks its decryption callbacks.
//!
//! The cert is verified against the CURRENT `KmsContext` (`host_config.current_kms_context_id`), not
//! a request-time pin: a rotated-out — possibly compromised — context can produce no accepted cert,
//! at the cost of in-flight certs dying on rotation (the KMS re-signs under the new context). Fail
//! closed. This preserves the context-rotation-rejection property one layer below the app.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::set_return_data;

use super::common::read_canonical_encrypted_value;
use crate::{eip712, errors::ZamaHostError, state::*};

/// Anchor-native mirror of `zama_solana_acl::MmrProof` for use as an instruction argument. The
/// shared ACL crate is deliberately Anchor-free (pure `borsh`) so it cannot derive Anchor IDL
/// metadata; this local type carries the identical wire shape and converts into the shared proof.
/// Defined in the host program so CPI callers depend only on the host IDL, never on token types.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct MmrInclusionProof {
    /// Index of the proven leaf within the lineage's MMR.
    pub leaf_index: u64,
    /// Authentication path from the leaf up to its mountain peak.
    pub siblings: Vec<[u8; 32]>,
}

impl From<MmrInclusionProof> for zama_solana_acl::MmrProof {
    fn from(proof: MmrInclusionProof) -> Self {
        zama_solana_acl::MmrProof {
            leaf_index: proof.leaf_index,
            siblings: proof.siblings,
        }
    }
}

/// Accounts for `verify_public_decrypt`. All read-only: a pure verifier reads state and returns a
/// value, so it takes no payer, no signer, and no system program.
#[derive(Accounts)]
pub struct VerifyPublicDecrypt<'info> {
    /// Canonical singleton host config: source of the current KMS context id and the gateway
    /// EIP-712 domain (chain id + `Decryption` verifying contract).
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// KMS context PDA; must be the canonical PDA for `host_config.current_kms_context_id`.
    pub kms_context: Account<'info, KmsContext>,
    /// The lineage whose peaks the inclusion proof is checked against.
    /// CHECK: layout, ownership, and canonical PDA are validated in the handler via `read_canonical_encrypted_value`.
    pub encrypted_value: UncheckedAccount<'info>,
}

/// Verifies a KMS public-decrypt certificate against the current context plus an MMR public-leaf
/// inclusion proof, and returns `(handle ++ cleartext)` via `return_data`.
///
/// `cleartext` is the 32-byte big-endian `uint256` the KMS signs over (today's decrypted result
/// fits in 32 bytes); `return_data` is 64 bytes, well under the 1024-byte limit.
pub fn verify_public_decrypt(
    ctx: Context<VerifyPublicDecrypt>,
    handle: [u8; 32],
    cleartext: [u8; 32],
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
    proof: MmrInclusionProof,
) -> Result<()> {
    let host_config = &ctx.accounts.host_config;
    let kms_context = &ctx.accounts.kms_context;
    let current_context_id = host_config.current_kms_context_id;

    require!(
        host_config.decryption_contract != [0u8; 20] && current_context_id != 0,
        ZamaHostError::GatewayVerifierConfigUnset
    );
    require!(!kms_context.destroyed, ZamaHostError::InvalidKmsContext);
    // The passed context account must be the canonical PDA for the CURRENT context id.
    require!(
        kms_context.context_id == current_context_id
            && kms_context.key() == kms_context_address(current_context_id).0,
        ZamaHostError::InvalidKmsContext
    );
    // The id the certificate commits to (via signed extra_data, EVM `_extractContextId` parity) must
    // equal the current id: a cert minted under a rotated-out context cannot be presented here.
    let cert_context_id = eip712::extract_kms_context_id(&extra_data, current_context_id)
        .ok_or(ZamaHostError::InvalidKmsContext)?;
    require!(
        cert_context_id == current_context_id,
        ZamaHostError::InvalidKmsContext
    );

    let verifier = eip712::Eip712VerifierConfig {
        gateway_chain_id: host_config.gateway_chain_id,
        verifying_contract: host_config.decryption_contract,
        signers: &kms_context.signers,
        threshold: kms_context.thresholds.public_decryption,
    };
    require!(
        eip712::verify_kms_public_decrypt(
            &verifier,
            &[handle],
            &cleartext,
            &extra_data,
            &signatures,
        ),
        ZamaHostError::InvalidKmsCertificate
    );

    // Exact-handle public-decrypt proof against the lineage's current peaks (no roll-forward): a
    // handle sealed public stays provable after later supersedes move the peaks.
    let info = ctx.accounts.encrypted_value.to_account_info();
    let value = read_canonical_encrypted_value(&info)?;
    zama_solana_acl::authorize_public(
        info.key().to_bytes(),
        &value.to_shared(),
        handle,
        &proof.into(),
    )
    .map_err(|_| error!(ZamaHostError::PublicDecryptProofInvalid))?;

    let mut return_data = [0u8; 64];
    return_data[..32].copy_from_slice(&handle);
    return_data[32..].copy_from_slice(&cleartext);
    set_return_data(&return_data);
    Ok(())
}
