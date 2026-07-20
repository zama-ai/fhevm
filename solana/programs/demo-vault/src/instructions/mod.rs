//! Instruction account contexts and handlers for the demo-vault program.
//!
//! The public Anchor entrypoints in `lib.rs` delegate into these modules so
//! account contexts, validation, and handler logic stay out of the crate root.

pub mod deposit;
pub mod harvest;
pub mod initialize_vault;
pub mod withdraw;

use anchor_lang::prelude::*;
use anchor_spl::token::{
    self as spl_token, Burn, Mint, MintTo, Token, TokenAccount, TransferChecked,
};

use crate::constants::*;
use crate::errors::*;
use crate::events::*;
use crate::state::*;

pub use deposit::*;
pub use harvest::*;
pub use initialize_vault::*;
pub use withdraw::*;
