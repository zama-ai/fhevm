use anchor_lang::prelude::*;

use super::common::{
    assert_no_remaining_accounts, assert_not_paused, create_pda_strict, write_account,
};
use crate::{errors::ZamaHostError, state::*};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateEncryptedStateArgs {
    pub program: Pubkey,
    pub scope: [u8; 32],
    pub authority_seeds: Vec<Vec<u8>>,
}

#[derive(Accounts)]
pub struct CreateEncryptedState<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    /// CHECK: canonical PDA and uninitialized ownership are checked before creation.
    #[account(mut)]
    pub encrypted_state: UncheckedAccount<'info>,
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    pub system_program: Program<'info, System>,
}

pub fn create_encrypted_state(
    ctx: Context<CreateEncryptedState>,
    args: CreateEncryptedStateArgs,
) -> Result<()> {
    assert_not_paused(&ctx.accounts.host_config)?;
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let authority = ctx.accounts.authority.key();
    let seeds: Vec<&[u8]> = args.authority_seeds.iter().map(Vec::as_slice).collect();
    let derived = Pubkey::create_program_address(&seeds, &args.program)
        .map_err(|_| error!(ZamaHostError::EncryptedValueAuthorityNotProgramPda))?;
    require_keys_eq!(
        derived,
        authority,
        ZamaHostError::EncryptedValueAuthorityNotProgramPda
    );
    let (address, bump) = encrypted_state_address(args.program, authority, args.scope);
    let info = ctx.accounts.encrypted_state.to_account_info();
    require_keys_eq!(
        address,
        info.key(),
        ZamaHostError::EncryptedValuePdaMismatch
    );
    create_pda_strict(
        &ctx.accounts.payer.to_account_info(),
        &info,
        &ctx.accounts.system_program.to_account_info(),
        zama_solana_acl::EncryptedState::account_size(0, 0),
        &[
            ENCRYPTED_STATE_SEED,
            args.program.as_ref(),
            authority.as_ref(),
            &args.scope,
            &[bump],
        ],
    )?;
    write_account(
        &info,
        &EncryptedState {
            program: args.program,
            authority,
            scope: args.scope,
            slots: Vec::new(),
            leaf_count: 0,
            peaks: Vec::new(),
            bump,
        },
    )
}
