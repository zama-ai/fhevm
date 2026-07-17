//! On-chain account data for `HostConfig`.

use super::*;

/// Singleton host configuration and authority surface.
///
/// `HostConfig` is the runtime switchboard for this PoC. Production-shaped
/// instructions reject while paused.
#[account]
pub struct HostConfig {
    /// Program administrator allowed to update config flags.
    pub admin: Pubkey,
    /// Host-chain id included in handle derivation.
    pub chain_id: u64,
    /// EVM gateway chain id used in the coprocessor/KMS EIP-712 domain separators.
    pub gateway_chain_id: u64,
    /// EVM `InputVerification` contract address: the EIP-712 verifying contract for
    /// coprocessor `CiphertextVerification` input attestations.
    pub input_verification_contract: [u8; 20],
    /// Registered coprocessor EVM signer set for input attestations (EVM `InputVerifier`
    /// parity). Fixed-capacity so `HostConfig` keeps a pinned byte layout; only the first
    /// `coprocessor_signer_count` entries are active, the rest are zero padding.
    pub coprocessor_signers: [[u8; 20]; Self::MAX_COPROCESSOR_SIGNERS],
    /// Number of active entries in `coprocessor_signers`.
    pub coprocessor_signer_count: u8,
    /// Minimum distinct valid signatures (n-of-m) required over an input attestation;
    /// `1 <= coprocessor_threshold <= coprocessor_signer_count`.
    pub coprocessor_threshold: u8,
    /// EVM `Decryption` contract address: the EIP-712 verifying contract for KMS
    /// `PublicDecryptVerification` certificates (disclose/redeem).
    pub decryption_contract: [u8; 20],
    /// Active KMS context id (mirrors `ProtocolConfig.getCurrentKmsContextId`). The
    /// signer set + thresholds live in the `KmsContext` PDA at this id; 0 means none
    /// defined yet. Updated by `define_kms_context`.
    pub current_kms_context_id: u64,
    /// Pauses production-shaped host instructions when true.
    pub paused: bool,
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
    /// Upper bound on registered coprocessor signers. A hard cap keeps the singleton's byte
    /// layout pinned (the array serializes to `MAX_COPROCESSOR_SIGNERS * 20` bytes regardless of
    /// how many signers are active) and bounds the per-attestation recovery cost.
    pub const MAX_COPROCESSOR_SIGNERS: usize = 8;
    pub const SPACE: usize = 32
        + 8
        + 8
        + 20
        + (Self::MAX_COPROCESSOR_SIGNERS * 20)
        + 1
        + 1
        + 20
        + 8
        + 1
        + 1
        + 8
        + 8
        + 8
        + 8
        + 1;

    /// Active coprocessor signer set (the first `coprocessor_signer_count` entries).
    pub fn active_coprocessor_signers(&self) -> &[[u8; 20]] {
        &self.coprocessor_signers[..self.coprocessor_signer_count as usize]
    }
}

/// Zero-pads a coprocessor signer slice into the fixed-capacity array stored in `HostConfig`.
/// Entries beyond `MAX_COPROCESSOR_SIGNERS` are ignored; callers validate the length first.
pub fn pack_coprocessor_signers(
    signers: &[[u8; 20]],
) -> [[u8; 20]; HostConfig::MAX_COPROCESSOR_SIGNERS] {
    let mut out = [[0u8; 20]; HostConfig::MAX_COPROCESSOR_SIGNERS];
    for (slot, signer) in out.iter_mut().zip(signers.iter()) {
        *slot = *signer;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::AccountSerialize;

    // The serialized account must be exactly `8 + SPACE`; a short SPACE would truncate the
    // singleton. The coprocessor signer set is a fixed-cap array (`MAX_COPROCESSOR_SIGNERS * 20`
    // bytes) plus a `count` and a `threshold` byte, so the layout stays pinned regardless of how
    // many signers are registered.
    #[test]
    fn host_config_space_matches_serialized_len() {
        // 151 (single-signer layout) - 20 (old `coprocessor_signer`) + 160 (8 * 20 signer array)
        // + 1 (count) + 1 (threshold) = 293.
        assert_eq!(HostConfig::SPACE, 293);

        let cfg = HostConfig {
            admin: Pubkey::new_unique(),
            chain_id: 1,
            gateway_chain_id: 0,
            input_verification_contract: [0u8; 20],
            coprocessor_signers: [[0u8; 20]; HostConfig::MAX_COPROCESSOR_SIGNERS],
            coprocessor_signer_count: 0,
            coprocessor_threshold: 0,
            decryption_contract: [0u8; 20],
            current_kms_context_id: 0,
            paused: false,
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
