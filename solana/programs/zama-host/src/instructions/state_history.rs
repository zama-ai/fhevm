//! Public sealing of a current State slot.

use super::common::*;
use crate::{errors::ZamaHostError, state::*};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

#[derive(Accounts)]
pub struct MakeStateHandlePublic<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub encrypted_state: Account<'info, EncryptedState>,
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// CHECK: the canonical application deny witness is checked by the handler.
    pub deny_scope_record: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
}

pub fn make_state_handle_public(
    ctx: Context<MakeStateHandlePublic>,
    key: [u8; 32],
    handle: [u8; 32],
    previous_leaf_count: u64,
) -> Result<()> {
    assert_not_paused(&ctx.accounts.host_config)?;
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let state = &mut ctx.accounts.encrypted_state;
    state.validate(state.key())?;
    require_keys_eq!(
        ctx.accounts.authority.key(),
        state.authority,
        ZamaHostError::EncryptedStateAccountAuthorityMismatch
    );
    require!(
        state.get(&key) == Some(handle),
        ZamaHostError::EncryptedStatePublicHandleMismatch
    );
    require!(
        state.leaf_count == previous_leaf_count,
        ZamaHostError::EncryptedStateMmrInconsistent
    );
    check_scope_not_denied(
        &ctx.accounts.host_config,
        AppScope {
            program: state.program,
            scope: state.scope,
        },
        ctx.accounts.deny_scope_record.as_ref(),
    )?;
    let commitment = zama_solana_acl::public_decrypt_leaf_commitment(
        state.key().to_bytes(),
        state.leaf_count,
        handle,
    );
    let state_data: &mut EncryptedState = state;
    zama_solana_acl::mmr_append(
        &mut state_data.peaks,
        &mut state_data.leaf_count,
        commitment,
    )
    .map_err(map_mmr_append_error)?;
    let space = zama_solana_acl::EncryptedState::account_size(state.slots.len(), state.peaks.len());
    grow_account_if_needed(
        &ctx.accounts.payer.to_account_info(),
        &state.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        space,
    )?;
    Ok(())
}

fn map_mmr_append_error(error: zama_solana_acl::AclError) -> anchor_lang::error::Error {
    match error {
        zama_solana_acl::AclError::MmrPeakCapacityExceeded => {
            error!(ZamaHostError::EncryptedStateMmrPeakCapacityExceeded)
        }
        _ => error!(ZamaHostError::EncryptedStateMmrInconsistent),
    }
}

/// Reallocs the account and tops up rent when `target_space` grows past the
/// account's current data length. Never shrinks — the leaf count is monotonic.
pub(super) fn grow_account_if_needed<'info>(
    payer: &AccountInfo<'info>,
    account: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    target_space: usize,
) -> Result<()> {
    if account.data_len() >= target_space {
        return Ok(());
    }
    let rent = Rent::get()?.minimum_balance(target_space);
    if account.lamports() < rent {
        let top_up = rent - account.lamports();
        invoke_signed(
            &system_instruction::transfer(payer.key, account.key, top_up),
            &[payer.clone(), account.clone(), system_program.clone()],
            &[],
        )?;
    }
    account.resize(target_space)?;
    Ok(())
}
