use anchor_lang::prelude::*;

use crate::{
    constants,
    error::AclError,
    state::{Config, HandlerPermissions},
    types::Handle,
};

#[derive(Accounts)]
#[instruction(handle: Handle, subject_pubkey: Pubkey, initial_pubkey: Pubkey, output_index: u128)]
pub struct IsAllowed<'info> {
    #[account(
        seeds = [
            constants::PERMISSION_LIST,
            initial_pubkey.as_ref(),
            &output_index.to_le_bytes()
        ],
        bump = permission_list.bump
    )]
    pub permission_list: Account<'info, HandlerPermissions>,
    #[account(
        seeds = [constants::ACL_CONFIG],
        bump = acl_config.bump
    )]
    pub acl_config: Account<'info, Config>,
}

pub fn is_allowed(
    ctx: Context<IsAllowed>,
    handle: Handle,
    subject_pubkey: Pubkey,
    _initial_pubkey: Pubkey,
    _output_index: u128,
) -> Result<()> {
    let permission_list = &ctx.accounts.permission_list;
    require!(permission_list.handle == handle, AclError::HandleMismatch);
    require!(
        subject_pubkey != Pubkey::default(),
        AclError::DefaultKeyNotAllowed
    );
    require!(
        permission_list.allowed_accounts.contains(&subject_pubkey),
        AclError::HandleAuthorizationFailed
    );
    Ok(())
}
