//! Migrates legacy confidential mints to split disclosure/redemption verifier sets.

use super::*;
use anchor_lang::solana_program::system_instruction;

/// Accounts for migrating a legacy confidential mint layout.
#[derive(Accounts)]
#[event_cpi]
pub struct MigrateMintVerifierSets<'info> {
    /// Rent payer for any account-size top-up needed by realloc.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Legacy mint authority.
    pub authority: Signer<'info>,
    /// CHECK: legacy confidential mint account; handler verifies owner, discriminator, size, and fields.
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// Active threshold verifier set for disclosure responses.
    pub disclosure_verifier_set: Box<Account<'info, zama_host::VerifierSet>>,
    /// Active threshold verifier set for burn-redemption responses.
    pub redemption_verifier_set: Box<Account<'info, zama_host::VerifierSet>>,
    /// System program used for rent top-up before realloc.
    pub system_program: Program<'info, System>,
}

/// Migrates a legacy mint account from the single KMS verifier authority layout.
///
/// The legacy verifier authority is intentionally not mapped into either new
/// verifier set. Operators must provide active token-scoped verifier sets whose
/// membership has already been bootstrapped in ZamaHost.
pub fn migrate_mint_verifier_sets(ctx: Context<MigrateMintVerifierSets>) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let mint_info = ctx.accounts.mint.to_account_info();
    let mint_key = ctx.accounts.mint.key();
    require_keys_eq!(
        *mint_info.owner,
        crate::ID,
        ConfidentialTokenError::MintAccountMismatch
    );
    require!(
        !mint_info.executable,
        ConfidentialTokenError::MintAccountMismatch
    );

    let legacy = {
        let data = mint_info.try_borrow_data()?;
        require!(
            data.len() == 8 + LegacyConfidentialMintV1::SPACE
                && data[..8] == ConfidentialMint::DISCRIMINATOR[..],
            ConfidentialTokenError::MintAccountMismatch
        );
        LegacyConfidentialMintV1::try_from_slice(&data[8..])
            .map_err(|_| error!(ConfidentialTokenError::MintAccountMismatch))?
    };
    require_keys_eq!(
        legacy.authority,
        ctx.accounts.authority.key(),
        ConfidentialTokenError::MintAuthorityMismatch
    );
    require_keys_eq!(
        legacy.acl_domain_key,
        mint_key,
        ConfidentialTokenError::AclDomainKeyMismatch
    );
    require_keys_eq!(
        legacy.compute_signer,
        compute_signer_address(mint_key).0,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    assert_active_verifier_set(
        &ctx.accounts.disclosure_verifier_set,
        ctx.accounts.disclosure_verifier_set.key(),
        zama_host::VERIFIER_SET_KIND_TOKEN_DISCLOSURE,
        mint_key,
    )?;
    assert_active_verifier_set(
        &ctx.accounts.redemption_verifier_set,
        ctx.accounts.redemption_verifier_set.key(),
        zama_host::VERIFIER_SET_KIND_TOKEN_REDEMPTION,
        mint_key,
    )?;

    let required_lamports = Rent::get()?.minimum_balance(8 + ConfidentialMint::SPACE);
    let current_lamports = mint_info.lamports();
    if current_lamports < required_lamports {
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.payer.key(),
                &mint_key,
                required_lamports - current_lamports,
            ),
            &[
                ctx.accounts.payer.to_account_info(),
                mint_info.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    }

    mint_info.resize(8 + ConfidentialMint::SPACE)?;
    let migrated = ConfidentialMint {
        authority: legacy.authority,
        acl_domain_key: legacy.acl_domain_key,
        compute_signer: legacy.compute_signer,
        underlying_mint: legacy.underlying_mint,
        disclosure_verifier_set: ctx.accounts.disclosure_verifier_set.key(),
        redemption_verifier_set: ctx.accounts.redemption_verifier_set.key(),
        decimals: legacy.decimals,
        total_supply_handle: legacy.total_supply_handle,
        total_supply_acl_record: legacy.total_supply_acl_record,
        next_total_supply_nonce_sequence: legacy.next_total_supply_nonce_sequence,
    };
    {
        let mut data = mint_info.try_borrow_mut_data()?;
        let mut writer: &mut [u8] = &mut data[..];
        migrated.try_serialize(&mut writer)?;
    }

    emit_cpi!(ConfidentialMintMigratedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        authority: ctx.accounts.authority.key(),
        legacy_kms_verifier_authority: legacy.kms_verifier_authority,
        disclosure_verifier_set: ctx.accounts.disclosure_verifier_set.key(),
        redemption_verifier_set: ctx.accounts.redemption_verifier_set.key(),
        disclosure_verifier_set_version: ctx.accounts.disclosure_verifier_set.version,
        redemption_verifier_set_version: ctx.accounts.redemption_verifier_set.version,
        migrated_slot: Clock::get()?.slot,
    });
    Ok(())
}
