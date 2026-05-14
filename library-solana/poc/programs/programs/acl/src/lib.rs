mod constants;
mod error;
mod event;
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

    pub fn allow(
        ctx: Context<Allow>,
        handle: Handle,
        context_key: Pubkey,
        initial_key: Pubkey,
        output_index: u128,
    ) -> Result<()> {
        instructions::allow(ctx, handle, context_key, initial_key, output_index)?;
        Ok(())
    }

    pub fn init_handle(
        ctx: Context<InitHandle>,
        handle: Handle,
        initial_key: Pubkey,
        output_index: u128,
    ) -> Result<()> {
        instructions::init_handle(ctx, handle, initial_key, output_index)?;
        Ok(())
    }

    pub fn is_allowed(
        ctx: Context<IsAllowed>,
        handle: Handle,
        subject_pubkey: Pubkey,
        initial_pubkey: Pubkey,
        output_index: u128,
    ) -> Result<()> {
        instructions::is_allowed(ctx, handle, subject_pubkey, initial_pubkey, output_index)?;
        Ok(())
    }
}
