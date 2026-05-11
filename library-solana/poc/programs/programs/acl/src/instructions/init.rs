use anchor_lang::prelude::*;

use crate::{constants, state::Config};

#[derive(Accounts)]
#[instruction(fhe_authority: Pubkey, external_input_authority: Pubkey)]
pub struct Init<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + Config::INIT_SPACE,
        seeds = [constants::ACL_CONFIG],
        bump
    )]
    pub config_pda: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

pub fn init_acl(
    ctx: Context<Init>,
    fhe_authority: Pubkey,
    external_input_authority: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.config_pda;
    config.fhe_authority = fhe_authority;
    config.external_input_authority = external_input_authority;
    Ok(())
}


