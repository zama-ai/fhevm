//! On-chain account data for `HostConfig`.
//!
//! Public API surface: `runtime-tests`' `host_mollusk` fixtures, which read the configured signer set
//! back out of a built `HostConfig` to assert what the program will accept.

use super::*;

/// Singleton host configuration and authority surface.
///
/// `HostConfig` is the runtime switchboard for this PoC. Each paused area rejects the
/// instructions it covers (`PauseFlags`).
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
    /// signer set + thresholds live in the `KmsContext` PDA at this id; `[0; 32]` means
    /// none defined yet. Updated by `define_kms_context`.
    pub current_kms_context_id: [u8; 32],
    /// Host areas currently stopped. A pauser sets them; only the admin clears them (DD-058).
    pub paused: PauseFlags,
    /// Enables the deny list: a denied application `(program, scope)` cannot compute, allow, or make a handle public.
    pub grant_deny_list_enabled: bool,
    /// Max total HCU summed over one `fhe_execute` execution. `u64::MAX` = unlimited
    /// (enforcement off); `0` is rejected at set time.
    pub max_hcu_per_tx: u64,
    /// Max critical-path (depth) HCU within one `fhe_execute` execution. `u64::MAX` =
    /// unlimited; `0` is rejected at set time.
    pub max_hcu_depth_per_tx: u64,
    /// Per-app HCU budget per slot, enforced in `fhe_execute`. `u64::MAX` = unrestricted (the ship
    /// default; short-circuits, touching no meter); `0` = ban untrusted apps (trusted still
    /// bypass) — the one knob where `0` is a real semantic; any other value is the metering band
    /// (must be `>= max_hcu_per_tx` unless that is unlimited).
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
        + 32
        + PauseFlags::SPACE
        + 1
        + 8
        + 8
        + 8
        + 8
        + 1;

    /// Active coprocessor signer set (the first `coprocessor_signer_count` entries).
    /// Count is write-validated (`≤ MAX`); clamp defends a corrupted singleton without panicking.
    pub fn active_coprocessor_signers(&self) -> &[[u8; 20]] {
        let count = (self.coprocessor_signer_count as usize).min(Self::MAX_COPROCESSOR_SIGNERS);
        &self.coprocessor_signers[..count]
    }
}

/// Zero-pads a coprocessor signer slice into the fixed-capacity array stored in `HostConfig`.
/// Panics if `signers.len()` exceeds [`HostConfig::MAX_COPROCESSOR_SIGNERS`] — callers must
/// validate first (`validate_and_pack_coprocessor_signers`); test fixtures must pass a legal set.
pub fn pack_coprocessor_signers(
    signers: &[[u8; 20]],
) -> [[u8; 20]; HostConfig::MAX_COPROCESSOR_SIGNERS] {
    assert!(
        signers.len() <= HostConfig::MAX_COPROCESSOR_SIGNERS,
        "coprocessor signer set exceeds HostConfig::MAX_COPROCESSOR_SIGNERS"
    );
    let mut out = [[0u8; 20]; HostConfig::MAX_COPROCESSOR_SIGNERS];
    for (slot, signer) in out.iter_mut().zip(signers.iter()) {
        *slot = *signer;
    }
    out
}

/// The host areas a pauser can stop, each matching an area EVM pauses on its own (DD-058). As
/// the argument of `pause` and `unpause`, a set field names an area to change.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PauseFlags {
    /// `fhe_execute`, with the allows, transient grants and public releases it writes (EVM ACL
    /// pause, which stops execution through `allowTransient`).
    pub execution: bool,
    /// `fhe_execute` steps that consume a coprocessor-attested input (Gateway
    /// `InputVerification` pause).
    pub verified_inputs: bool,
    /// ACL writes outside an execution: Store creation, `make_store_handle_public` and
    /// user-decryption delegation (EVM ACL pause).
    pub acl_writes: bool,
    /// `verify_public_decrypt` and the token instructions that accept a KMS certificate (Gateway
    /// `Decryption` pause).
    pub public_decrypt: bool,
}

impl PauseFlags {
    /// Serialized size.
    pub const SPACE: usize = 4;

    /// Every area.
    pub const ALL: Self = Self {
        execution: true,
        verified_inputs: true,
        acl_writes: true,
        public_decrypt: true,
    };

    /// These flags with every area `areas` names set.
    pub fn with(self, areas: Self) -> Self {
        Self {
            execution: self.execution || areas.execution,
            verified_inputs: self.verified_inputs || areas.verified_inputs,
            acl_writes: self.acl_writes || areas.acl_writes,
            public_decrypt: self.public_decrypt || areas.public_decrypt,
        }
    }

    /// These flags with every area `areas` names cleared.
    pub fn without(self, areas: Self) -> Self {
        Self {
            execution: self.execution && !areas.execution,
            verified_inputs: self.verified_inputs && !areas.verified_inputs,
            acl_writes: self.acl_writes && !areas.acl_writes,
            public_decrypt: self.public_decrypt && !areas.public_decrypt,
        }
    }

    /// Fails with the area's own pause error while `area` is paused.
    pub fn require_running(self, area: PauseArea) -> Result<()> {
        let (paused, error) = match area {
            PauseArea::Execution => (self.execution, ZamaHostError::ExecutionPaused),
            PauseArea::VerifiedInputs => {
                (self.verified_inputs, ZamaHostError::VerifiedInputsPaused)
            }
            PauseArea::AclWrites => (self.acl_writes, ZamaHostError::AclWritesPaused),
            PauseArea::PublicDecrypt => (self.public_decrypt, ZamaHostError::PublicDecryptPaused),
        };
        if paused {
            return Err(error.into());
        }
        Ok(())
    }
}

/// One area of `PauseFlags`, named by the instructions that refuse to run while it is paused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PauseArea {
    Execution,
    VerifiedInputs,
    AclWrites,
    PublicDecrypt,
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
        assert_eq!(HostConfig::SPACE, 320);

        let cfg = HostConfig {
            admin: Pubkey::new_unique(),
            chain_id: 1,
            gateway_chain_id: 0,
            input_verification_contract: [0u8; 20],
            coprocessor_signers: [[0u8; 20]; HostConfig::MAX_COPROCESSOR_SIGNERS],
            coprocessor_signer_count: 0,
            coprocessor_threshold: 0,
            decryption_contract: [0u8; 20],
            current_kms_context_id: [0u8; 32],
            paused: PauseFlags::default(),
            grant_deny_list_enabled: false,
            max_hcu_per_tx: u64::MAX,
            max_hcu_depth_per_tx: u64::MAX,
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
