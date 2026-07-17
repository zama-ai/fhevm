//! On-chain account data for `HcuBlockMeter`.

use super::*;

/// Per-`compute_subject` running HCU total for the current slot.
///
/// One meter per compute subject (keyed by the frame's `compute_subject`), reused across slots via
/// a lazy reset: when `last_seen_slot != clock.slot` the accumulated `used_hcu` is treated as `0`
/// for the new slot rather than carried over. Program-authority; lazy-created on the first
/// metered frame; permanent (no close / reclamation in v1, so close+reopen cannot reset the
/// counter mid-slot).
#[account]
pub struct HcuBlockMeter {
    /// The `compute_subject` this meter counts.
    pub app: Pubkey,
    /// Slot in which `used_hcu` was last written; a different current slot resets usage to 0.
    pub last_seen_slot: u64,
    /// HCU accumulated by this app so far in `last_seen_slot`.
    pub used_hcu: u64,
    /// PDA bump for `PDA("hcu-block-meter", app)`.
    pub bump: u8,
}

impl HcuBlockMeter {
    /// Serialized size of the account body, excluding the Anchor discriminator.
    pub const SPACE: usize = 32 + 8 + 8 + 1;
}
