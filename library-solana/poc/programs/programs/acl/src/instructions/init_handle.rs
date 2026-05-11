use anchor_lang::prelude::*;

use crate::{
    constants,
    error::AclError,
    state::{Config, HandlerPermissions, DISCRIMINATOR},
    types::Handle,
};

#[derive(Accounts)]
#[instruction(handle: Handle)]
pub struct InitHandle<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = DISCRIMINATOR + HandlerPermissions::INIT_SPACE
    )]
    pub permission_list: Account<'info, HandlerPermissions>,

    #[account(
        seeds = [constants::ACL_CONFIG],
        bump
    )]
    pub acl_config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

pub fn init_handle(ctx: Context<InitHandle>, handle: Handle) -> Result<()> {
    let config = &ctx.accounts.acl_config;
    let authority = &ctx.accounts.authority;
    let permission_list = &mut ctx.accounts.permission_list;
    require!(
        config.authorize(authority.key),
        AclError::UnauthorizedAccess
    );
    permission_list.handle = handle;

    Ok(())
}
