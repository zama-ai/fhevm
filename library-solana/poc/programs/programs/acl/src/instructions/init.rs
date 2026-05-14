use anchor_lang::prelude::*;

use crate::{
    constants,
    program::Acl,
    state::{Config, DISCRIMINATOR},
};

/// Currently Init can be called only by the upgrade authority of ACL
/// It can be changed later but I leave it like this for the POC
#[derive(Accounts)]
#[instruction(fhe_authority: Pubkey, external_input_authority: Pubkey)]
pub struct Init<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = DISCRIMINATOR + Config::INIT_SPACE,
        seeds = [constants::ACL_CONFIG],
        bump
    )]
    pub config_pda: Account<'info, Config>,

    #[account(constraint = acl.programdata_address()? == Some(program_data.key()))]
    pub acl: Program<'info, Acl>,

    #[account(constraint = program_data.upgrade_authority_address == Some(payer.key()))]
    pub program_data: Account<'info, ProgramData>,

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
    config.bump = ctx.bumps.config_pda;
    Ok(())
}
