use anchor_lang::solana_program::instruction::{
    get_stack_height, Instruction, TRANSACTION_LEVEL_STACK_HEIGHT,
};
use anchor_lang::{prelude::*, Discriminator};
use solana_instructions_sysvar::{load_current_index_checked, load_instruction_at_checked};

use super::common::{assert_no_remaining_accounts, create_pda_strict, write_account};
use crate::{errors::ZamaHostError, state::*};

#[derive(Accounts)]
pub struct OpenScratch<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(address = encrypted_state.authority)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [ENCRYPTED_STATE_SEED, encrypted_state.program.as_ref(), encrypted_state.authority.as_ref(), &encrypted_state.scope],
        bump = encrypted_state.bump,
    )]
    pub encrypted_state: Account<'info, EncryptedState>,
    /// CHECK: derived from the canonical initiating state and created empty below.
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
    // Remaining accounts are writable (scratch, recorded refund) pairs.
}

pub fn open_scratch(ctx: Context<OpenScratch>) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let instructions = ctx.accounts.instructions.to_account_info();
    let (last_index, close) = final_close(&instructions)?;
    require!(
        load_current_index_checked(&instructions)? < last_index,
        ZamaHostError::TransientCloseMissing
    );
    let state = ctx.accounts.encrypted_state.key();
    let (address, bump) = transient_address(state);
    let scratch = ctx.accounts.scratch.to_account_info();
    require_keys_eq!(
        scratch.key(),
        address,
        ZamaHostError::TransientAccountInvalid
    );
    let mut matches = close.accounts[1..]
        .chunks_exact(2)
        .filter(|pair| pair[0].pubkey == address);
    let pair = matches.next().ok_or(ZamaHostError::TransientCloseMissing)?;
    require!(
        matches.next().is_none(),
        ZamaHostError::TransientCloseMissing
    );
    require!(
        pair[0].is_writable && pair[1].is_writable,
        ZamaHostError::TransientCloseMissing
    );
    require_keys_eq!(
        pair[1].pubkey,
        ctx.accounts.payer.key(),
        ZamaHostError::TransientCloseMissing
    );
    create_pda_strict(
        &ctx.accounts.payer.to_account_info(),
        &scratch,
        &ctx.accounts.system_program.to_account_info(),
        TransientState::SPACE,
        &[TRANSIENT_SEED, state.as_ref(), &[bump]],
    )?;
    write_account(
        &scratch,
        &TransientState {
            initiating_state: state,
            refund: ctx.accounts.payer.key(),
            grants: Vec::new(),
            bump,
        },
    )
}

pub fn close_scratch(ctx: Context<CloseScratch>) -> Result<()> {
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
    let accounts = ctx.remaining_accounts;
    require!(
        !accounts.is_empty() && accounts.len().is_multiple_of(2),
        ZamaHostError::TransientAccountInvalid
    );
    for (index, pair) in accounts.chunks_exact(2).enumerate() {
        let scratch = &pair[0];
        let refund = &pair[1];
        require!(
            scratch.is_writable && refund.is_writable,
            ZamaHostError::TransientAccountInvalid
        );
        require!(
            !accounts
                .chunks_exact(2)
                .any(|other| other[0].key() == refund.key()),
            ZamaHostError::TransientAccountInvalid
        );
        require!(
            !accounts[..index * 2]
                .chunks_exact(2)
                .any(|other| other[0].key() == scratch.key()),
            ZamaHostError::TransientAccountInvalid
        );
        require_keys_eq!(
            *scratch.owner,
            crate::ID,
            ZamaHostError::TransientAccountInvalid
        );
        require!(
            scratch.data_len() == TransientState::SPACE,
            ZamaHostError::TransientAccountInvalid
        );
        let data = TransientState::try_deserialize(&mut &scratch.try_borrow_data()?[..])
            .map_err(|_| error!(ZamaHostError::TransientAccountInvalid))?;
        let (address, bump) = transient_address(data.initiating_state);
        require_keys_eq!(
            scratch.key(),
            address,
            ZamaHostError::TransientAccountInvalid
        );
        require!(data.bump == bump, ZamaHostError::TransientAccountInvalid);
        require_keys_eq!(
            refund.key(),
            data.refund,
            ZamaHostError::TransientAccountInvalid
        );
        refund.add_lamports(scratch.lamports())?;
        **scratch.try_borrow_mut_lamports()? = 0;
        scratch.assign(&System::id());
        scratch.resize(0)?;
    }
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
        close.data == crate::instruction::CloseScratch::DISCRIMINATOR,
        ZamaHostError::TransientCloseMissing
    );
    require!(
        close.accounts.len() >= 3 && close.accounts.len() % 2 == 1,
        ZamaHostError::TransientCloseMissing
    );
    require_keys_eq!(
        close.accounts[0].pubkey,
        solana_instructions_sysvar::ID,
        ZamaHostError::TransientCloseMissing
    );
    Ok((last, close))
}
