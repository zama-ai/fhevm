//! Stops host areas at once: any enabled pauser, like EVM `ACL.pause` for a `PauserSet` member.

use anchor_lang::prelude::*;

use super::common::*;
use crate::errors::ZamaHostError;
use crate::state::{HostConfig, PauseFlags, PauserRecord, HOST_CONFIG_SEED, PAUSER_SEED};

/// Accounts for setting pause flags.
#[derive(Accounts)]
#[event_cpi]
pub struct Pause<'info> {
    /// A key with an enabled pauser record.
    pub pauser: Signer<'info>,
    /// The signer's pauser record.
    #[account(
        seeds = [PAUSER_SEED, pauser.key().as_ref()],
        bump = pauser_record.bump,
        constraint = pauser_record.enabled @ ZamaHostError::NotPauser,
    )]
    pub pauser_record: Account<'info, PauserRecord>,
    /// Singleton config PDA.
    #[account(mut, seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
}

/// Sets the pause flag of every area `areas` names. Areas already paused stay paused, so
/// pausers acting at the same time all succeed.
pub fn pause(ctx: Context<Pause>, areas: PauseFlags) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    require_top_level_unless_pda(
        &ctx.accounts.pauser.key(),
        ZamaHostError::WalletPauseThroughCpi,
    )?;
    let config = &mut ctx.accounts.host_config;
    let paused = config.paused.with(areas);
    if paused == config.paused {
        return Ok(());
    }
    config.paused = paused;
    config.updated_slot = Clock::get()?.slot;
    emit_config_updated(
        config,
        ctx.accounts.pauser.key(),
        &ctx.accounts.event_authority,
    )
}
