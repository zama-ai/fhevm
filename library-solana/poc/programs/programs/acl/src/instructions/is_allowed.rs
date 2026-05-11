use anchor_lang::prelude::*;

use crate::{error::AclError, state::HandlerPermissions, types::Handle};

#[derive(Accounts)]
#[instruction(handle: Handle)]
pub struct IsAllowed<'info> {
    pub payer: Signer<'info>,

    pub permission_list: Account<'info, HandlerPermissions>,
}

pub fn is_allowed(ctx: Context<IsAllowed>, handle: Handle) -> Result<bool> {
    let permission_list = &ctx.accounts.permission_list;
    let payer = &ctx.accounts.payer;
    require!(permission_list.handle == handle, AclError::HandleMismatch);
    Ok(permission_list.allowed_accounts.contains(payer.key))
}
