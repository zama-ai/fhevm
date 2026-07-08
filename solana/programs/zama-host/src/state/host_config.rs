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
    /// Max total HCU summed over one `fhe_eval` plan. `0` = unlimited (enforcement off).
    pub max_hcu_per_tx: u64,
    /// Max critical-path (depth) HCU within one `fhe_eval` plan. `0` = unlimited.
    pub max_hcu_depth_per_tx: u64,
    /// Per-app HCU budget per slot, enforced in `fhe_eval`. `u64::MAX` = unrestricted (the ship
    /// default; short-circuits, touching no meter); `0` = ban untrusted apps (trusted still bypass);
    /// any other value is the metering band (must be `>= max_hcu_per_tx` unless that is `0`).
    pub hcu_block_cap_per_app: u64,
    /// Slot in which the config was initialized or last changed.
    pub updated_slot: u64,
    /// PDA bump for `PDA("host-config")`.
    pub bump: u8,
}

impl HostConfig {
    // + max_hcu_per_tx (8) + max_hcu_depth_per_tx (8): 225 -> 241.
    // + hcu_block_cap_per_app (8): 241 -> 249.
    pub const SPACE: usize =
        32 + 8 + 32 + 8 + 20 + 20 + 20 + 8 + 32 + 32 + 1 + 1 + 1 + 1 + 8 + 8 + 8 + 8 + 1;

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

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::AccountSerialize;

    // Adding the per-app block cap grows `HostConfig` by exactly one u64 (8 bytes): SPACE goes
    // 241 -> 249. There is no separate enable flag — the cap value itself encodes "unrestricted"
    // and "ban", so no extra bool byte. A serialized account must be exactly `8 + SPACE`; a short
    // SPACE would truncate the singleton.
    #[test]
    fn host_config_space_grows_to_249_for_block_cap() {
        // The prior SPACE was 241 (all fields through the two per-frame HCU limits).
        const PRIOR_SPACE: usize = 241;
        assert_eq!(HostConfig::SPACE, PRIOR_SPACE + 8);
        assert_eq!(HostConfig::SPACE, 249);

        let cfg = HostConfig {
            admin: Pubkey::new_unique(),
            chain_id: 1,
            input_verifier_authority: Pubkey::new_unique(),
            gateway_chain_id: 0,
            input_verification_contract: [0u8; 20],
            coprocessor_signer: [0u8; 20],
            decryption_contract: [0u8; 20],
            current_kms_context_id: 0,
            material_authority: Pubkey::new_unique(),
            test_authority: Pubkey::new_unique(),
            paused: false,
            mock_input_enabled: false,
            test_shims_enabled: false,
            grant_deny_list_enabled: false,
            max_hcu_per_tx: 0,
            max_hcu_depth_per_tx: 0,
            // Ships unrestricted (u64::MAX). A `0` default would instead ban every untrusted app
            // on deploy — the strictest state, not a neutral one.
            hcu_block_cap_per_app: u64::MAX,
            updated_slot: 0,
            bump: 0,
        };
        let mut buf = Vec::new();
        cfg.try_serialize(&mut buf).unwrap();
        assert_eq!(buf.len(), 8 + HostConfig::SPACE);
    }
}
