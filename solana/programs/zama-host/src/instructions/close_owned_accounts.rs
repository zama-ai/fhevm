//! Closes program-owned accounts so a preview deployment can be wiped between trials (DD-051).
//!
//! Only the owner program can zero an account's lamports or reassign it. Targets are untyped
//! remaining accounts, so an account left by an older layout can still be closed, and a target
//! the program does not own is skipped rather than failing the instruction. Gated on the upgrade
//! authority rather than `HostConfig.admin` because a half-initialized deployment has no config.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::bpf_loader_upgradeable;

use crate::errors::ZamaHostError;

include!("../../../close_program_owned.rs");

/// Accounts for closing program-owned accounts. Targets follow as remaining accounts.
#[derive(Accounts)]
pub struct CloseOwnedAccounts<'info> {
    /// The program's upgrade authority; receives the rent of every closed account.
    #[account(mut)]
    pub admin: Signer<'info>,
    /// The program's `ProgramData`, which names the upgrade authority.
    #[account(
        address = bpf_loader_upgradeable::get_program_data_address(&crate::ID),
        constraint = program_data.upgrade_authority_address == Some(admin.key()) @ ZamaHostError::HostConfigAdminMismatch
    )]
    pub program_data: Account<'info, ProgramData>,
}

/// Closes each program-owned remaining account; a foreign or already closed account is skipped.
pub fn close_owned_accounts<'info>(ctx: Context<'info, CloseOwnedAccounts<'info>>) -> Result<()> {
    close_program_owned(
        &ctx.accounts.admin.to_account_info(),
        ctx.remaining_accounts,
    )
}
