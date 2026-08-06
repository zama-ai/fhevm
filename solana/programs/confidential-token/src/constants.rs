//! Shared constants and PDA seed bytes for the confidential-token program.

pub(crate) const BALANCE_FHE_TYPE: u8 = 5;
pub const APP_EVENT_VERSION: u8 = 1;
/// PDA seed for the single pending burn scoped to one confidential token account.
pub const PENDING_BURN_SEED: &[u8] = b"pending-burn";
