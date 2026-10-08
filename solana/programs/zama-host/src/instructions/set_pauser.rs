//! Creates and updates pauser records: the admin manages the pauser set, like EVM
//! `PauserSet.addPauser` and `removePauser` (`onlyACLOwner`).

use anchor_lang::prelude::*;

use super::common::*;
use crate::event_cpi::emit_event_cpi;
use crate::events::PauserUpdatedEvent;
use crate::state::*;

/// Accounts for creating or updating a pauser record.
#[derive(Accounts)]
#[instruction(pauser: Pubkey)]
#[event_cpi]
pub struct SetPauser<'info> {
    /// Pays rent if the pauser PDA must be created.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Configured host admin.
    pub admin: Signer<'info>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// The pauser's record, created on first use.
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + PauserRecord::SPACE,
        seeds = [PAUSER_SEED, pauser.as_ref()],
        bump,
    )]
    pub pauser_record: Account<'info, PauserRecord>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Grants or withdraws `pauser`'s right to set pause flags.
pub fn set_pauser(ctx: Context<SetPauser>, pauser: Pubkey, enabled: bool) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, &ctx.accounts.admin)?;

    // A record created by this call is zeroed: `enabled == false` reads as an absent record does.
    // The identity is written before the unchanged-state return, so a zeroed record never persists.
    let record = &mut ctx.accounts.pauser_record;
    record.pauser = pauser;
    record.bump = ctx.bumps.pauser_record;
    if record.enabled == enabled {
        return Ok(());
    }
    record.enabled = enabled;

    emit_event_cpi(
        &ctx.accounts.event_authority,
        &PauserUpdatedEvent {
            version: EVENT_VERSION,
            pauser_record: ctx.accounts.pauser_record.key(),
            pauser,
            enabled,
            updated_slot: Clock::get()?.slot,
        },
    )?;
    Ok(())
}
