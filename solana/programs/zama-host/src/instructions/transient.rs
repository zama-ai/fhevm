use super::common::{assert_no_remaining_accounts, create_pda_strict};
use crate::{errors::ZamaHostError, state::*};
use anchor_lang::solana_program::instruction::{
    get_stack_height, Instruction, TRANSACTION_LEVEL_STACK_HEIGHT,
};
use anchor_lang::{prelude::*, AccountsExit, Discriminator};
use solana_instructions_sysvar::{load_current_index_checked, load_instruction_at_checked};

#[derive(Accounts)]
pub struct OpenScratch<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: payer-derived host PDA, strictly created below.
    #[account(mut)]
    pub scratch: UncheckedAccount<'info>,
    /// CHECK: only the runtime's Instructions sysvar is accepted.
    #[account(address = solana_instructions_sysvar::ID)]
    pub instructions: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseScratch<'info> {
    /// CHECK: only the runtime's Instructions sysvar is accepted.
    #[account(address = solana_instructions_sysvar::ID)]
    pub instructions: UncheckedAccount<'info>,
    #[account(mut, close = refund)]
    pub scratch: AccountLoader<'info, TransientState>,
    /// CHECK: must match the rent payer recorded by OpenScratch.
    #[account(mut)]
    pub refund: UncheckedAccount<'info>,
}

pub fn open_scratch<'info>(ctx: Context<'info, OpenScratch<'info>>) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    require!(
        get_stack_height() == TRANSACTION_LEVEL_STACK_HEIGHT,
        ZamaHostError::TransientCloseMissing
    );
    let payer = ctx.accounts.payer.key();
    let (address, bump) = transient_address(payer);
    require_keys_eq!(
        ctx.accounts.scratch.key(),
        address,
        ZamaHostError::TransientAccountInvalid
    );
    assert_final_close(address, payer, &ctx.accounts.instructions)?;
    create_pda_strict(
        &ctx.accounts.payer.to_account_info(),
        &ctx.accounts.scratch.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        TransientState::SPACE,
        &[TRANSIENT_SEED, payer.as_ref(), &[bump]],
    )?;
    let loader =
        AccountLoader::<TransientState>::try_from_unchecked(&crate::ID, &ctx.accounts.scratch)?;
    {
        let mut scratch = loader.load_init()?;
        scratch.payer = payer;
        scratch.bump = bump;
    }
    loader.exit(&crate::ID)
}

pub fn close_scratch(ctx: Context<CloseScratch>) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    require!(
        get_stack_height() == TRANSACTION_LEVEL_STACK_HEIGHT,
        ZamaHostError::TransientCloseMissing
    );
    let instructions = ctx.accounts.instructions.to_account_info();
    let (last_index, _) = final_close(&instructions)?;
    require!(
        load_current_index_checked(&instructions)? == last_index,
        ZamaHostError::TransientCloseMissing
    );
    require!(
        ctx.accounts.scratch.to_account_info().data_len() == TransientState::SPACE,
        ZamaHostError::TransientAccountInvalid
    );
    let scratch = ctx.accounts.scratch.load()?;
    scratch.validate(ctx.accounts.scratch.key())?;
    require_keys_eq!(
        ctx.accounts.refund.key(),
        scratch.payer,
        ZamaHostError::TransientAccountInvalid
    );
    require_keys_neq!(
        ctx.accounts.scratch.key(),
        ctx.accounts.refund.key(),
        ZamaHostError::TransientAccountInvalid
    );
    Ok(())
}

/// Every host execution must use the single scratch closed by the final
/// instruction. Strict top-level creation and final-only closure prevent resets.
pub(super) fn assert_final_close(
    scratch: Pubkey,
    payer: Pubkey,
    instructions: &AccountInfo,
) -> Result<()> {
    let (last, close) = final_close(instructions)?;
    require!(
        load_current_index_checked(instructions)? < last,
        ZamaHostError::TransientCloseMissing
    );
    require_keys_eq!(
        close.accounts[1].pubkey,
        scratch,
        ZamaHostError::TransientCloseMissing
    );
    require_keys_eq!(
        close.accounts[2].pubkey,
        payer,
        ZamaHostError::TransientCloseMissing
    );
    Ok(())
}

fn final_close(instructions: &AccountInfo) -> Result<(u16, Instruction)> {
    let data = instructions.try_borrow_data()?;
    let count = data.get(..2).ok_or(ZamaHostError::TransientCloseMissing)?;
    let count = u16::from_le_bytes([count[0], count[1]]);
    let last = count
        .checked_sub(1)
        .ok_or(ZamaHostError::TransientCloseMissing)?;
    drop(data);
    let close = load_instruction_at_checked(last as usize, instructions)?;
    require_keys_eq!(
        close.program_id,
        crate::ID,
        ZamaHostError::TransientCloseMissing
    );
    require!(
        close.data == crate::instruction::CloseScratch::DISCRIMINATOR
            && close.accounts.len() == 3
            && close.accounts[1].is_writable
            && close.accounts[2].is_writable,
        ZamaHostError::TransientCloseMissing
    );
    require_keys_eq!(
        close.accounts[0].pubkey,
        solana_instructions_sysvar::ID,
        ZamaHostError::TransientCloseMissing
    );
    Ok((last, close))
}
