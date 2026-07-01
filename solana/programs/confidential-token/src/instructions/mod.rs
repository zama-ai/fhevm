//! Instruction account contexts and handlers for the confidential-token program.
//!
//! The public Anchor entrypoints in `lib.rs` delegate into these modules so
//! account contexts, validation, and handler logic stay out of the crate root.

pub mod close_consumed_burn_redemption_request;
pub mod close_consumed_disclosure_request;
pub mod close_expired_burn_redemption_request;
pub mod close_expired_disclosure_request;
pub mod common;
pub mod confidential_burn;
pub mod confidential_transfer;
pub mod create_random_amount;
pub mod disclose_amount_secp;
pub mod disclose_balance_secp;
pub mod initialize_mint;
pub mod initialize_token_account;
pub mod redeem_burned_amount_secp;
pub mod request_burn_redemption;
pub mod request_disclose_amount;
pub mod request_disclose_balance;
pub mod wrap_usdc;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::AccountMeta;
use anchor_spl::{
    associated_token::get_associated_token_address_with_program_id,
    token::{self as spl_token, Mint as SplMint, Token, TokenAccount, TransferChecked},
};
use zama_host::{self, program::ZamaHost};

use crate::{errors::*, events::*, fhe, state::*};

pub use close_consumed_burn_redemption_request::*;
pub use close_consumed_disclosure_request::*;
pub use close_expired_burn_redemption_request::*;
pub use close_expired_disclosure_request::*;
pub(crate) use common::*;
pub use confidential_burn::*;
pub use confidential_transfer::*;
pub use create_random_amount::*;
pub use disclose_amount_secp::*;
pub use disclose_balance_secp::*;
pub use initialize_mint::*;
pub use initialize_token_account::*;
pub use redeem_burned_amount_secp::*;
pub use request_burn_redemption::*;
pub use request_disclose_amount::*;
pub use request_disclose_balance::*;
pub use wrap_usdc::*;
