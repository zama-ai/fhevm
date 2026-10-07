//! Creates and updates application deny-list records.

use anchor_lang::prelude::*;

use super::common::*;
use crate::event_cpi::emit_event_cpi;
use crate::events::DenyScopeUpdatedEvent;
use crate::state::*;

/// Accounts for creating or updating a deny-list record.
#[derive(Accounts)]
#[instruction(app_program: Pubkey, scope: Pubkey)]
#[event_cpi]
pub struct SetDenyScope<'info> {
    /// Pays rent if the deny-list PDA must be created.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Configured host admin.
    pub admin: Signer<'info>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// The application's deny-list record, created on first use.
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + DenyScopeRecord::SPACE,
        seeds = [DENY_SCOPE_SEED, app_program.as_ref(), scope.as_ref()],
        bump,
    )]
    pub deny_scope_record: Account<'info, DenyScopeRecord>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Creates or updates the deny-list state for the application `(program, scope)`.
pub fn set_deny_scope(
    ctx: Context<SetDenyScope>,
    program: Pubkey,
    scope: Pubkey,
    denied: bool,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, &ctx.accounts.admin)?;

    // A record created by this call is zeroed: `denied == false` reads as an absent record does.
    // The identity is written before the unchanged-state return, so a zeroed record never persists.
    let record = &mut ctx.accounts.deny_scope_record;
    record.program = program;
    record.scope = scope;
    record.bump = ctx.bumps.deny_scope_record;
    if record.denied == denied {
        return Ok(());
    }
    record.denied = denied;

    emit_event_cpi(
        &ctx.accounts.event_authority,
        &DenyScopeUpdatedEvent {
            version: EVENT_VERSION,
            deny_scope_record: ctx.accounts.deny_scope_record.key(),
            program,
            scope,
            denied,
            updated_slot: Clock::get()?.slot,
        },
    )?;
    Ok(())
}
