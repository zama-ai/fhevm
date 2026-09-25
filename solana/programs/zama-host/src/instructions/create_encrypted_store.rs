use anchor_lang::prelude::*;

use super::common::{
    assert_no_remaining_accounts, assert_not_paused, create_pda_strict, write_account,
};
use crate::{errors::ZamaHostError, state::*};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateEncryptedStoreArgs {
    pub program: Pubkey,
    pub authority_seeds: Vec<Vec<u8>>,
}

#[derive(Accounts)]
pub struct CreateEncryptedStore<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    /// CHECK: only its key and owner are read; the owner must be the store's program.
    pub scope: UncheckedAccount<'info>,
    /// CHECK: canonical PDA and uninitialized ownership are checked before creation.
    #[account(mut)]
    pub encrypted_store: UncheckedAccount<'info>,
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    pub system_program: Program<'info, System>,
}

pub fn create_encrypted_store(
    ctx: Context<CreateEncryptedStore>,
    args: CreateEncryptedStoreArgs,
) -> Result<()> {
    assert_not_paused(
        &ctx.accounts.host_config,
        |paused| paused.acl_writes,
        ZamaHostError::AclWritesPaused,
    )?;
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let authority = ctx.accounts.authority.key();
    let seeds: Vec<&[u8]> = args.authority_seeds.iter().map(Vec::as_slice).collect();
    let derived = Pubkey::create_program_address(&seeds, &args.program)
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
    let scope = ctx.accounts.scope.key().to_bytes();
    require_keys_eq!(
        *ctx.accounts.scope.owner,
        args.program,
        ZamaHostError::EncryptedStoreScopeNotProgramAccount
    );
    let (address, bump) = encrypted_store_address(args.program, authority, scope);
    let info = ctx.accounts.encrypted_store.to_account_info();
    require_keys_eq!(
        address,
        info.key(),
        ZamaHostError::EncryptedStorePdaMismatch
    );
    create_pda_strict(
        &ctx.accounts.payer.to_account_info(),
        &info,
        &ctx.accounts.system_program.to_account_info(),
        zama_solana_acl::EncryptedStore::account_size(0, 0),
        &[
            ENCRYPTED_STORE_SEED,
            args.program.as_ref(),
            authority.as_ref(),
            &scope,
            &[bump],
        ],
    )?;
    write_account(
        &info,
        &EncryptedStore {
            program: args.program,
            authority,
            scope,
            slots: Vec::new(),
            leaf_count: 0,
            peaks: Vec::new(),
            bump,
        },
    )
}
