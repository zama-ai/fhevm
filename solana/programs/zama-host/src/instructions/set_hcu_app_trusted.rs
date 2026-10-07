//! Creates and updates HCU trust-registry records (per-application block-cap bypass).
//!
//! Mirrors `set_deny_scope`, but inverted: absence means "untrusted" (metered), and only an
//! admin-created, program-owned record with `trusted == true` bypasses the cap. The application is
//! the `(program, scope)` the block cap meters. An application cannot self-trust — the write is
//! admin-gated.

use anchor_lang::prelude::*;

use super::common::*;
use crate::event_cpi::emit_event_cpi;
use crate::events::HcuAppTrustUpdatedEvent;
use crate::state::*;

/// Accounts for creating or updating an HCU trust-registry record.
#[derive(Accounts)]
#[instruction(app_program: Pubkey, scope: Pubkey)]
#[event_cpi]
pub struct SetHcuAppTrusted<'info> {
    /// Pays rent if the trust-registry PDA must be created.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Configured host admin.
    pub admin: Signer<'info>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// The application's trust-registry record, created on first use.
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + HcuTrustedAppRecord::SPACE,
        seeds = [HCU_TRUSTED_APP_SEED, app_program.as_ref(), scope.as_ref()],
        bump,
    )]
    pub hcu_trusted_app_record: Account<'info, HcuTrustedAppRecord>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Creates or updates the trust state for the application `(program, scope)`.
pub fn set_hcu_app_trusted(
    ctx: Context<SetHcuAppTrusted>,
    program: Pubkey,
    scope: Pubkey,
    trusted: bool,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, &ctx.accounts.admin)?;

    // A record created by this call is zeroed: `trusted == false` reads as an absent record does.
    // The identity is written before the unchanged-state return, so a zeroed record never persists.
    let record = &mut ctx.accounts.hcu_trusted_app_record;
    record.program = program;
    record.scope = scope;
    record.bump = ctx.bumps.hcu_trusted_app_record;
    if record.trusted == trusted {
        return Ok(());
    }
    record.trusted = trusted;

    emit_event_cpi(
        &ctx.accounts.event_authority,
        &HcuAppTrustUpdatedEvent {
            version: EVENT_VERSION,
            hcu_trusted_app_record: ctx.accounts.hcu_trusted_app_record.key(),
            program,
            scope,
            trusted,
            updated_slot: Clock::get()?.slot,
        },
    )?;
    Ok(())
}
