//! On-chain account data for `HcuTrustedAppRecord`.

use super::*;

/// Trust-registry record marking an application as bypassing the per-app HCU block cap.
///
/// This is the inverse of [`DenyScopeRecord`](super::DenyScopeRecord): the deny list fails
/// *closed* on presence, whereas absence here means "untrusted" (metered), and only a present,
/// program-owned record with `trusted == true` bypasses the cap. Admin-only writer; long-lived.
/// The direct analog of EVM's `blockHCUWhitelist`.
#[account]
#[derive(InitSpace)]
pub struct HcuTrustedAppRecord {
    /// The application program this record governs.
    pub program: Pubkey,
    /// The scope this record governs: an account of `program`.
    pub scope: Pubkey,
    /// When true, the application bypasses the per-app block cap entirely (no meter, no charge).
    pub trusted: bool,
    /// PDA bump for `PDA("hcu-trusted", program, scope)`.
    pub bump: u8,
}
