//! Instruction account contexts and handlers for the confidential-token program.
//!
//! The public Anchor entrypoints in `lib.rs` delegate into these modules so
//! account contexts, validation, and handler logic stay out of the crate root.

pub mod allow_token_account_subjects;
pub mod cancel_pending_burn;
pub mod common;
pub mod confidential_burn;
pub mod confidential_transfer;
pub mod disclose_secp;
pub mod initialize_mint;
pub mod initialize_token_account;
pub mod redeem_burned_amount;
pub mod total_supply_subjects;
pub mod wrap_usdc;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::AccountMeta,
    program::{invoke, invoke_signed},
    system_instruction,
};
use anchor_lang::AccountSerialize;
use anchor_spl::{
    associated_token::get_associated_token_address_with_program_id,
    token_interface::{
        self as spl_token, Mint as SplMint, TokenAccount, TokenInterface, TransferChecked,
    },
};
use zama_host::{self, program::ZamaHost};

use crate::{errors::*, events::*, fhe, state::*};

pub use allow_token_account_subjects::*;
pub use cancel_pending_burn::*;
pub(crate) use common::*;
pub use confidential_burn::*;
pub use confidential_transfer::*;
pub use disclose_secp::*;
pub use initialize_mint::*;
pub use initialize_token_account::*;
pub use redeem_burned_amount::*;
pub use total_supply_subjects::*;
pub use wrap_usdc::*;
