use anchor_lang::prelude::*;

use super::common::{
    assert_no_remaining_accounts, assert_not_paused, create_pda_strict, write_account,
};
use crate::{errors::ZamaHostError, state::*};

#[derive(Accounts)]
#[instruction(program: Pubkey)]
pub struct CreateEncryptedStore<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    /// CHECK: only its key and owner are read; the owner must be the store's program.
    pub scope: UncheckedAccount<'info>,
    /// CHECK: canonical PDA and uninitialized ownership are checked before creation.
    #[account(mut, seeds = [ENCRYPTED_STORE_SEED, program.as_ref(), authority.key().as_ref(), scope.key().as_ref()], bump)]
    pub encrypted_store: UncheckedAccount<'info>,
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    pub system_program: Program<'info, System>,
}

pub fn create_encrypted_store(
    ctx: Context<CreateEncryptedStore>,
    program: Pubkey,
    authority_seeds: Vec<Vec<u8>>,
) -> Result<()> {
    assert_not_paused(&ctx.accounts.host_config, PauseArea::AclWrites)?;
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let authority = ctx.accounts.authority.key();
    let seeds: Vec<&[u8]> = authority_seeds.iter().map(Vec::as_slice).collect();
    let derived = Pubkey::create_program_address(&seeds, &program)
        .map_err(|_| error!(ZamaHostError::EncryptedStoreAuthorityNotProgramPda))?;
    require_keys_eq!(
        derived,
        authority,
        ZamaHostError::EncryptedStoreAuthorityNotProgramPda
    );
    // The scope names an account of the store's program, so two programs cannot pick the same
    // application and a program id is never a scope (its owner is the loader). This also keeps the
    // wildcard sentinel out: nothing can live there, and an absent account is System-owned, while
    // `program` signed through the authority PDA above, which the System program never does.
    let scope = ctx.accounts.scope.key();
    require_keys_eq!(
        *ctx.accounts.scope.owner,
        program,
        ZamaHostError::EncryptedStoreScopeNotProgramAccount
    );
    let bump = ctx.bumps.encrypted_store;
    let info = ctx.accounts.encrypted_store.to_account_info();
    // The cleartext build allocates the largest shape up front so its plaintext section sits at a
    // fixed offset no later Borsh write reaches.
    #[cfg(not(feature = "cleartext"))]
    let space = zama_solana_acl::EncryptedStore::account_size(0, 0);
    #[cfg(feature = "cleartext")]
    let space = crate::cleartext::layout::STORE_ACCOUNT_SIZE;
    create_pda_strict(
        &ctx.accounts.payer.to_account_info(),
        &info,
        &ctx.accounts.system_program.to_account_info(),
        space,
        &[
            ENCRYPTED_STORE_SEED,
            program.as_ref(),
            authority.as_ref(),
            scope.as_ref(),
            &[bump],
        ],
    )?;
    write_account(
        &info,
        &EncryptedStore {
            program,
            authority,
            scope,
            slots: Vec::new(),
            leaf_count: 0,
            peaks: Vec::new(),
            bump,
        },
    )?;
    #[cfg(feature = "cleartext")]
    crate::cleartext::layout::init_store_section(&mut info.try_borrow_mut_data()?)?;
    Ok(())
}
