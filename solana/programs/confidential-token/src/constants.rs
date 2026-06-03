//! Shared constants and PDA seed bytes for the confidential-token program.

use anchor_lang::prelude::Pubkey;

pub(crate) const BALANCE_FHE_TYPE: u8 = 5;
pub const APP_EVENT_VERSION: u8 = 0;
pub const CALLBACK_SETTLEMENT_PREPARED: u8 = 1;
pub const CALLBACK_SETTLEMENT_FINALIZED: u8 = 2;
pub const MAX_RECEIVER_HOOK_DATA_LEN: usize = 1024;
pub const MAX_RECEIVER_HOOK_ACCOUNTS: usize = 32;
pub(crate) const DISCLOSURE_PROOF_DOMAIN_SEPARATOR: &[u8] =
    b"zama-confidential-token-disclosure-v1";
pub(crate) const ED25519_PROGRAM_ID: Pubkey =
    anchor_lang::pubkey!("Ed25519SigVerify111111111111111111111111111");
