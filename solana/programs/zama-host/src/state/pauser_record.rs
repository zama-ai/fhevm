//! On-chain account data for `PauserRecord`.

use super::*;

/// Grants one key the right to set pause flags (EVM `PauserSet` membership). The admin creates
/// and toggles it with `set_pauser`; only the admin clears a flag (DD-058).
#[account]
pub struct PauserRecord {
    /// The key this record grants.
    pub pauser: Pubkey,
    /// Whether `pauser` may currently set pause flags.
    pub enabled: bool,
    /// PDA bump for `PDA("pauser", pauser)`.
    pub bump: u8,
}

impl PauserRecord {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 1 + 1;
}
