//! Shared constants and PDA seed bytes for the confidential-token program.

pub(crate) const BALANCE_FHE_TYPE: u8 = 5;
pub const APP_EVENT_VERSION: u8 = 1;
pub const CALLBACK_SETTLEMENT_PREPARED: u8 = 1;
pub const CALLBACK_SETTLEMENT_FINALIZED: u8 = 2;
pub const MAX_RECEIVER_HOOK_DATA_LEN: usize = 1024;
pub const MAX_RECEIVER_HOOK_ACCOUNTS: usize = 32;
