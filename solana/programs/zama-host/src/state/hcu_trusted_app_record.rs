//! On-chain account data for `HcuTrustedAppRecord`.

use super::*;

/// Trust-registry record marking an app as bypassing the per-app HCU block cap.
///
/// This is the inverse of [`DenySubjectRecord`](super::DenySubjectRecord): the deny
/// list fails *closed* on presence, whereas absence here means "untrusted" (metered),
/// and only a present, program-owned record with `trusted == true` bypasses the cap.
/// Admin-only writer; long-lived. The direct analog of EVM's `blockHCUWhitelist`.
#[account]
pub struct HcuTrustedAppRecord {
    /// The `app_account_authority` this record governs.
    pub app: Pubkey,
    /// When true, `app` bypasses the per-app block cap entirely (no meter, no charge).
    pub trusted: bool,
    /// PDA bump for `PDA("hcu-trusted", app)`.
    pub bump: u8,
}

impl HcuTrustedAppRecord {
    /// Serialized size of the account body, excluding the Anchor discriminator.
    pub const SPACE: usize = 32 + 1 + 1;
}
