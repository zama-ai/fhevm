//! App-local events for demo-vault.
//!
//! These events are for frontend/demo indexers. The generic coprocessor
//! host-listener does not consume them: the vault is the public half of the
//! confidential-vault design and emits no Zama host protocol events.

use anchor_lang::prelude::*;

/// Emitted when a vault is created.
#[event]
pub struct VaultInitialized {
    /// Event schema version.
    pub version: u8,
    /// Vault state account.
    pub vault: Pubkey,
    /// Underlying token mint the vault accepts.
    pub underlying_mint: Pubkey,
    /// Share mint the vault issues.
    pub share_mint: Pubkey,
    /// Token account holding the vault's underlying assets.
    pub vault_token_account: Pubkey,
    /// PDA authority owning the underlying token account and the share mint.
    pub vault_authority: Pubkey,
}

/// Emitted when underlying assets are deposited for freshly minted shares.
#[event]
pub struct Deposited {
    /// Event schema version.
    pub version: u8,
    /// Vault state account.
    pub vault: Pubkey,
    /// Depositor and transfer authority.
    pub depositor: Pubkey,
    /// Underlying assets moved into the vault.
    pub assets: u64,
    /// Shares minted to the depositor.
    pub shares: u64,
}

/// Emitted when shares are burned for underlying assets.
#[event]
pub struct Withdrawn {
    /// Event schema version.
    pub version: u8,
    /// Vault state account.
    pub vault: Pubkey,
    /// Share owner and burn authority.
    pub owner: Pubkey,
    /// Shares burned.
    pub shares: u64,
    /// Underlying assets returned to the owner.
    pub assets: u64,
}

/// Emitted when underlying assets are donated to the vault without minting shares.
///
/// This is the demo's simulated yield: the donation raises the share price for
/// every existing holder. A real vault would gate this behind a keeper/strategy
/// role; here it is permissionless.
#[event]
pub struct Harvested {
    /// Event schema version.
    pub version: u8,
    /// Vault state account.
    pub vault: Pubkey,
    /// Donor and transfer authority.
    pub donor: Pubkey,
    /// Underlying assets donated to the vault.
    pub assets: u64,
}
