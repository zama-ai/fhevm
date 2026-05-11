mod constants;
mod error;
mod instructions;
pub mod state;
mod types;

use instructions::*;

use crate::types::Handle;
use anchor_lang::prelude::*;

declare_id!("GorNJjGrLdmZmxLvVvdNdAcdomhfErpsqoCeY9F7ZBYH");

#[program]
pub mod acl {

    use super::*;

    pub fn init(
        ctx: Context<Init>,
        fhe_authority: Pubkey,
        external_input_authority: Pubkey,
    ) -> Result<()> {
        instructions::init_acl(ctx, fhe_authority, external_input_authority)?;
        Ok(())
    }

    pub fn allow(ctx: Context<Allow>, handle: Handle, context_key: Pubkey) -> Result<()> {
        instructions::allow(ctx, handle, context_key)?;
        Ok(())
    }

    pub fn init_handle(ctx: Context<InitHandle>, handle: Handle) -> Result<()> {
        instructions::init_handle(ctx, handle)?;
        Ok(())
    }

    pub fn is_allowed(ctx: Context<IsAllowed>, handle: Handle) -> Result<bool> {
        let res = instructions::is_allowed(ctx, handle)?;
        Ok(res)
    }
}
