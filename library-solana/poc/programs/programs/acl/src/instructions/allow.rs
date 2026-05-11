use crate::{
    constants,
    error::AclError,
    state::{Config, HandlerPermissions, CHUNK, DISCRIMINATOR, HANDLE_SIZE, PUBKEY_SIZE, VEC_PREFIX},
    types::Handle,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(handle: u128, context_key: Pubkey)]
pub struct Allow<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    #[account(
        mut,
        realloc = {
            let data_len = permission_list.to_account_info().data_len();
            let header = DISCRIMINATOR + HANDLE_SIZE + VEC_PREFIX;
            let current_capacity = (data_len - header) / PUBKEY_SIZE;
            let chunks = (current_capacity + 1).div_ceil(CHUNK);
            DISCRIMINATOR + HandlerPermissions::space_for_chunks(chunks)
        },
        realloc::payer = payer,
        realloc::zero = true,
    )]
    pub permission_list: Account<'info, HandlerPermissions>,
    #[account(
        seeds = [constants::ACL_CONFIG],
        bump
    )]
    pub acl_config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

pub fn allow(ctx: Context<Allow>, handle: Handle, context_key: Pubkey) -> Result<()> {
    let permission_list = &mut ctx.accounts.permission_list;
    let config = &ctx.accounts.acl_config;
    let authority = &ctx.accounts.authority;
    require!(handle == permission_list.handle, AclError::HandleMismatch);
    require!(config.authorize(authority.key), AclError::UnauthorizedAccess);
    permission_list.allowed_accounts.push(context_key);
    Ok(())
}
