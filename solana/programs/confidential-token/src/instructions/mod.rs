//! Instruction account contexts and handlers for the confidential-token program.
//!
//! The public Anchor entrypoints in `lib.rs` delegate into these modules so
//! account contexts, validation, and handler logic stay out of the crate root.

mod accounts;
mod close_operator;
mod common;
mod confidential_burn;
mod confidential_call_transfer_receiver;
mod confidential_call_transfer_receiver_from;
mod confidential_finalize_transfer_callback;
mod confidential_prepare_transfer_callback;
mod confidential_transfer;
mod confidential_transfer_from;
mod create_random_amount;
mod disclose_amount;
mod disclose_balance;
mod initialize_mint;
mod initialize_token_account;
mod redeem_burned_amount;
mod request_disclose_amount;
mod request_disclose_balance;
mod set_operator;
mod test_receiver_return_callback;
mod wrap_usdc;

use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::{get_return_data, invoke, invoke_signed, set_return_data},
    system_instruction,
};
use anchor_lang::{prelude::*, AccountDeserialize};
use anchor_spl::{
    associated_token::get_associated_token_address_with_program_id,
    token::{self as spl_token, Mint as SplMint, Token, TokenAccount, TransferChecked},
};
use solana_instructions_sysvar::{
    load_current_index_checked, load_instruction_at_checked, ID as INSTRUCTIONS_SYSVAR_ID,
};
use zama_host::{self, program::ZamaHost, AclSubjectEntry};

use crate::{
    errors::*, events::*, fhe, state::*, transfer_receiver_return_data, TransferReceiverReturn,
};

pub use accounts::*;
pub use close_operator::*;
pub use common::disclosure_proof_message;
pub(crate) use common::*;
pub use confidential_burn::*;
pub use confidential_call_transfer_receiver::*;
pub use confidential_call_transfer_receiver_from::*;
pub use confidential_finalize_transfer_callback::*;
pub use confidential_prepare_transfer_callback::*;
pub use confidential_transfer::*;
pub use confidential_transfer_from::*;
pub use create_random_amount::*;
pub use disclose_amount::*;
pub use disclose_balance::*;
pub use initialize_mint::*;
pub use initialize_token_account::*;
pub use redeem_burned_amount::*;
pub use request_disclose_amount::*;
pub use request_disclose_balance::*;
pub use set_operator::*;
pub use test_receiver_return_callback::*;
pub use wrap_usdc::*;
