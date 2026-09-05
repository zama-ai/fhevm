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
use crate::{errors::ZamaHostError, state::*};

/// Accounts for creating or updating an HCU trust-registry record.
#[derive(Accounts)]
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
    /// CHECK: created or overwritten after canonical ("hcu-trusted", program, scope) PDA validation.
    #[account(mut)]
    pub hcu_trusted_app_record: UncheckedAccount<'info>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Creates or updates the trust state for the application `(program, scope)`.
pub fn set_hcu_app_trusted(
    ctx: Context<SetHcuAppTrusted>,
    program: Pubkey,
    scope: [u8; 32],
    trusted: bool,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, &ctx.accounts.admin)?;
    let app = AppScope { program, scope };
    let (expected, bump) = hcu_trusted_app_address(app);
    require_keys_eq!(
        expected,
        ctx.accounts.hcu_trusted_app_record.key(),
        ZamaHostError::HcuTrustedAppRecordMismatch
    );

    let info = ctx.accounts.hcu_trusted_app_record.to_account_info();
    let current = current_trusted_status(&info, app, bump)?;
    if current.unwrap_or(false) == trusted {
        return Ok(());
    }

    create_pda_if_needed(
        &ctx.accounts.payer.to_account_info(),
        &info,
        &ctx.accounts.system_program.to_account_info(),
        8 + HcuTrustedAppRecord::SPACE,
        &[HCU_TRUSTED_APP_SEED, program.as_ref(), &scope, &[bump]],
    )?;

    write_account(
        &info,
        &HcuTrustedAppRecord {
            program,
            scope,
            trusted,
            bump,
        },
    )?;
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

/// Reads the current trust flag for `app`: `None` when the record is absent (system-owned + empty),
/// otherwise the stored `trusted` after validating owner / length / PDA identity / bump.
fn current_trusted_status(info: &AccountInfo, app: AppScope, bump: u8) -> Result<Option<bool>> {
    if is_uninitialized_pda_account(info, ZamaHostError::HcuTrustedAppRecordMismatch)? {
        return Ok(None);
    }
    require_keys_eq!(
        *info.owner,
        crate::ID,
        ZamaHostError::HcuTrustedAppRecordMismatch
    );
    require!(
        info.data_len() == 8 + HcuTrustedAppRecord::SPACE,
        ZamaHostError::HcuTrustedAppRecordMismatch
    );
    let data = info.try_borrow_data()?;
    let mut data_slice: &[u8] = &data;
    let record = HcuTrustedAppRecord::try_deserialize(&mut data_slice)?;
    require!(
        record.program == app.program && record.scope == app.scope && record.bump == bump,
        ZamaHostError::HcuTrustedAppRecordMismatch
    );
    Ok(Some(record.trusted))
}
