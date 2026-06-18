//! On-chain account data for `HostConfig`.

use super::*;

/// Singleton host configuration and authority surface.
///
/// `HostConfig` is the runtime switchboard for this PoC. Production-shaped
/// instructions reject while paused, and mock/test-only instructions require
/// both the corresponding feature gate and configured signer.
#[account]
pub struct HostConfig {
    /// Program administrator allowed to update config flags.
    pub admin: Pubkey,
    /// Host-chain id included in handle derivation.
    pub chain_id: u64,
    /// Configured authority for input verification paths.
    pub input_verifier_authority: Pubkey,
    /// EVM gateway chain id used in the coprocessor/KMS EIP-712 domain separators.
    pub gateway_chain_id: u64,
    /// EVM `InputVerification` contract address: the EIP-712 verifying contract for
    /// coprocessor `CiphertextVerification` input attestations.
    pub input_verification_contract: [u8; 20],
    /// Authorized coprocessor EVM signer for input attestations (v0: single signer,
    /// threshold 1).
    pub coprocessor_signer: [u8; 20],
    /// EVM `Decryption` contract address: the EIP-712 verifying contract for KMS
    /// `PublicDecryptVerification` certificates (disclose/redeem).
    pub decryption_contract: [u8; 20],
    /// Active KMS context id (mirrors `ProtocolConfig.getCurrentKmsContextId`). The
    /// signer set + thresholds live in the `KmsContext` PDA at this id; 0 means none
    /// defined yet. Updated by `define_kms_context`.
    pub current_kms_context_id: u64,
    /// Configured authority for material-commitment paths.
    pub material_authority: Pubkey,
    /// Configured signer for `test_emit_*` shims.
    pub test_authority: Pubkey,
    /// Pauses production-shaped host instructions when true.
    pub paused: bool,
    /// Enables the mock encrypted-input bind instruction.
    pub mock_input_enabled: bool,
    /// Enables test event shim instructions.
    pub test_shims_enabled: bool,
    /// Enables deny-list checks for persistent grant authorities.
    pub grant_deny_list_enabled: bool,
    /// Slot in which the config was initialized or last changed.
    pub updated_slot: u64,
    /// PDA bump for `PDA("host-config")`.
    pub bump: u8,
}

impl HostConfig {
    pub const SPACE: usize = 32 + 8 + 32 + 8 + 20 + 20 + 20 + 8 + 32 + 32 + 1 + 1 + 1 + 1 + 8 + 1;

    /// True only for the local PoC sentinel chain id.
    ///
    /// Local-only relaxations are compiled only with the `poc` Cargo feature and
    /// are additionally confined to this sentinel chain id. Default builds
    /// reject the sentinel at initialization.
    pub fn is_local_poc_chain(&self) -> bool {
        POC_FEATURE_ENABLED && self.chain_id == SOLANA_POC_CHAIN_ID
    }

    /// True when PoC-only helpers may substitute zero birth entropy.
    ///
    /// The zero-hash fallback is confined to local PoC tests; production
    /// handle birth keeps using runtime entropy and fails closed without it.
    pub fn zero_birth_entropy_allowed(&self) -> bool {
        self.test_shims_enabled && self.is_local_poc_chain()
    }
}
