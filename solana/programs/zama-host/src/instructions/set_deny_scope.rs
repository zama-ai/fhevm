//! Creates and updates application deny-list records.

use anchor_lang::prelude::*;

use super::common::*;
use crate::event_cpi::emit_event_cpi;
use crate::events::DenyScopeUpdatedEvent;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for creating or updating a deny-list record.
#[derive(Accounts)]
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
    /// CHECK: created or overwritten after canonical deny-list PDA validation.
    #[account(mut)]
    pub deny_scope_record: UncheckedAccount<'info>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Creates or updates the deny-list state for the application `(program, scope)`.
pub fn set_deny_scope(
    ctx: Context<SetDenyScope>,
    program: Pubkey,
    scope: [u8; 32],
    denied: bool,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, &ctx.accounts.admin)?;
    let app = AppScope { program, scope };
    let (expected, bump) = deny_scope_address(app);
    require_keys_eq!(
        expected,
        ctx.accounts.deny_scope_record.key(),
        ZamaHostError::DenyRecordMismatch
    );

    let info = ctx.accounts.deny_scope_record.to_account_info();
    let current = current_deny_status(&info, app, bump)?;
    if current.unwrap_or(false) == denied {
        return Ok(());
    }

    create_pda_if_needed(
        &ctx.accounts.payer.to_account_info(),
        &info,
        &ctx.accounts.system_program.to_account_info(),
        8 + DenyScopeRecord::SPACE,
        &[DENY_SCOPE_SEED, program.as_ref(), &scope, &[bump]],
    )?;

    write_account(
        &info,
        &DenyScopeRecord {
            program,
            scope,
            denied,
            bump,
        },
    )?;
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

fn current_deny_status(info: &AccountInfo, app: AppScope, bump: u8) -> Result<Option<bool>> {
    if is_uninitialized_pda_account(info, ZamaHostError::DenyRecordMismatch)? {
        return Ok(None);
    }
    require_keys_eq!(*info.owner, crate::ID, ZamaHostError::DenyRecordMismatch);
    require!(
        info.data_len() == 8 + DenyScopeRecord::SPACE,
        ZamaHostError::DenyRecordMismatch
    );
    let data = info.try_borrow_data()?;
    let mut data_slice: &[u8] = &data;
    let record = DenyScopeRecord::try_deserialize(&mut data_slice)?;
    require!(
        record.program == app.program && record.scope == app.scope && record.bump == bump,
        ZamaHostError::DenyRecordMismatch
    );
    Ok(Some(record.denied))
}
