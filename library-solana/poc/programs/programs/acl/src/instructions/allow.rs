use crate::{
    constants::{self, MAX_SUBJECTS},
    error::AclError,
    event::Allowed,
    state::{
        Config, HandlerPermissions, HandlerState,
    },
    types::Handle,
};
use anchor_lang::prelude::*;


/// Grants `context_key` access to the handle stored at the PDA identified
/// by `(initial_key, output_index)`.
///
/// The two pubkeys play different roles and are intentionally distinct:
/// - `initial_key`: PDA seed — identifies *which* permission list to mutate.
///   Must match the `initial_key` that was passed to `init_handle` when the
///   list was created.
/// - `context_key`: the account being added to that list's `allowed_accounts`.
///   This is the grantee.
#[event_cpi]
#[derive(Accounts)]
#[instruction(handle: Handle, context_key: Pubkey, initial_key: Pubkey, output_index: u128)]
pub struct Allow<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [
            constants::PERMISSION_LIST,
            initial_key.as_ref(), 
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

/// In current implementation we decided not to do pagination/extension of handle PDAs 
/// and limited them to 8 accounts per handle
/// We can make it for v1 if we will decide it is needed.
pub fn allow(ctx: Context<Allow>, handle: Handle, context_key: Pubkey, _initial_key: Pubkey, _output_index: u128) -> Result<()> {
    let permission_list = &mut ctx.accounts.permission_list;
    let config = &ctx.accounts.acl_config;
    let authority = &ctx.accounts.authority;
    require!(context_key != Pubkey::default(), AclError::DefaultKeyAllow);
    require!(
        config.authorize(authority.key),
        AclError::UnauthorizedAccess
    );
    require!(permission_list.state == HandlerState::Bound, AclError::HandleNotReady);
    require!(handle == permission_list.handle, AclError::HandleMismatch);
    let subject_count = permission_list.subject_count as usize;
    require!(subject_count < MAX_SUBJECTS, AclError::HandleOverflow);

    if permission_list.allowed_accounts.contains(&context_key) {
        return Ok(())
    }
    
    permission_list.allowed_accounts[subject_count] = context_key;
    permission_list.subject_count += 1;
    emit_cpi!(Allowed {
        handle,
        context_key
    });
    Ok(())
}
