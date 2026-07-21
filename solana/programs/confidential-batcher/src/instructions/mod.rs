//! Instruction account contexts and handlers for the confidential batcher.
//!
//! The public Anchor entrypoints in `lib.rs` delegate into these modules so
//! account contexts, validation, and handler logic stay out of the crate root.

pub mod claim;
pub mod dispatch;
pub mod initialize_batcher;
pub mod join;
pub mod open_batch;
pub mod quit;
pub mod settle;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint as SplMint, Token, TokenAccount};
use confidential_token::{self as ct, program::ConfidentialToken};
use demo_vault::program::DemoVault;
use zama_host::program::ZamaHost;

use crate::{constants::*, errors::*, events::*, fhe, state::*};

pub use claim::*;
pub use dispatch::*;
pub use initialize_batcher::*;
pub use join::*;
pub use open_batch::*;
pub use quit::*;
pub use settle::*;

/// Moves lamports from the transaction payer to the batch authority PDA, which
/// pays the rent that token CPIs charge to the account owner (token-account
/// creation at open, the redemption marker and eval growth at settle).
/// Unspent lamports stay on the PDA and are unrecoverable by design in this
/// PoC — there is no sweep instruction.
pub(crate) fn fund_batch_authority<'info>(
    payer: &Signer<'info>,
    batch_authority: &UncheckedAccount<'info>,
    system_program: &Program<'info, System>,
    lamports: u64,
) -> Result<()> {
    if lamports == 0 {
        return Ok(());
    }
    anchor_lang::system_program::transfer(
        CpiContext::new(
            system_program.key(),
            anchor_lang::system_program::Transfer {
                from: payer.to_account_info(),
                to: batch_authority.to_account_info(),
            },
        ),
        lamports,
    )
}

/// Signer-seed bytes for a batch authority PDA.
pub(crate) struct BatchAuthoritySeeds {
    batch: Pubkey,
    bump: [u8; 1],
}

impl BatchAuthoritySeeds {
    pub(crate) fn new(batch: Pubkey, bump: u8) -> Self {
        Self {
            batch,
            bump: [bump],
        }
    }

    pub(crate) fn seeds(&self) -> [&[u8]; 3] {
        [BATCH_AUTHORITY_SEED, self.batch.as_ref(), &self.bump]
    }
}
