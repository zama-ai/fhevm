use anchor_lang::prelude::*;

use crate::{
    constants, error::AclError, event::NewHandle, state::{Config, DISCRIMINATOR, HandlerPermissions, HandlerState}, types::Handle
};

#[event_cpi]
#[derive(Accounts)]
#[instruction(handle: Handle, initial_key: Pubkey, output_index: u128)]
pub struct InitHandle<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        space = DISCRIMINATOR + HandlerPermissions::INIT_SPACE,
        seeds = [
            constants::PERMISSION_LIST, 
            initial_key.as_ref(), 
            &output_index.to_le_bytes()
        ],
        bump
    )]
    pub permission_list: Account<'info, HandlerPermissions>,

    #[account(
        seeds = [constants::ACL_CONFIG],
        bump = acl_config.bump
    )]
    pub acl_config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

pub fn init_handle(ctx: Context<InitHandle>, handle: Handle, initial_key: Pubkey, output_index: u128) -> Result<()> {
    let config = &ctx.accounts.acl_config;
    let authority = &ctx.accounts.authority;
    let permission_list = &mut ctx.accounts.permission_list;
    require!(
        config.authorize(authority.key),
        AclError::UnauthorizedAccess
    );
    if permission_list.state == HandlerState::Bound {
        require!(permission_list.handle == handle, AclError::HandleMismatch);
        return Ok(());
    }
    permission_list.handle = handle;
    permission_list.state = HandlerState::Bound;
    permission_list.bump = ctx.bumps.permission_list;
    emit_cpi!(NewHandle {
        handle,
        initial_key,
        output_index,
        config_key: config.key()
    });

    Ok(())
}
