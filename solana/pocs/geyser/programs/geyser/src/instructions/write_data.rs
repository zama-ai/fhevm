use anchor_lang::prelude::*;

use crate::constants::DATA_SEED;
use crate::state::DataAccount;

#[derive(Accounts)]
pub struct WriteData<'info> {
    /// Pays for PDA creation and must be its authority.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// PDA that holds the data, derived from `[DATA_SEED, authority]`.
    /// Created on first call, reused on subsequent calls.
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + DataAccount::INIT_SPACE,
        seeds = [DATA_SEED, authority.key().as_ref()],
        bump,
    )]
    pub data_account: Account<'info, DataAccount>,

    pub system_program: Program<'info, System>,
}

/// Returned by `write_data`. Anchor borsh-serializes this and calls
/// `set_return_data`, so it shows up as the transaction's return data.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct WriteResult {
    /// The value that was persisted.
    pub value: u64,
    /// The PDA bump.
    pub bump: u8,
}

pub fn handler(ctx: Context<WriteData>, value: u64, message: String) -> Result<WriteResult> {
    let data = &mut ctx.accounts.data_account;

    data.authority = ctx.accounts.authority.key();
    data.value = value;
    data.message = message;
    data.bump = ctx.bumps.data_account;

    msg!(
        "Wrote value={} message=\"{}\" into PDA {}",
        data.value,
        data.message,
        data.key()
    );

    Ok(WriteResult {
        value: data.value,
        bump: data.bump,
    })
}
