//! Mint-authority wrappers for total-supply encrypted value account subject management.
//!
//! The host requires ACL mutation to be signed by the encrypted value account authority. For the
//! encrypted total supply that authority is the `total-supply` PDA, so this program verifies the
//! existing `ConfidentialMint.authority` and signs the host CPI with that PDA.

use super::*;
use zama_host::cpi;

/// Accounts for granting subjects on a mint's encrypted total supply.
#[derive(Accounts)]
pub struct AllowTotalSupplySubjects<'info> {
    /// Pays for encrypted value account growth.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Existing confidential mint authority. Governance may own this key later.
    pub authority: Signer<'info>,
    #[account(has_one = authority @ ConfidentialTokenError::MintAuthorityMismatch)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// CHECK: encrypted total-supply authority PDA; signs the host CPI.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Encrypted total-supply value whose subjects are changed.
    #[account(mut, address = mint.total_supply_encrypted_value)]
    pub total_supply_value: Box<Account<'info, zama_host::EncryptedValue>>,
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// CHECK: canonical deny record for the PDA authority when grant denial is enabled.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}

/// Accounts for removing one subject from a mint's encrypted total supply.
#[derive(Accounts)]
pub struct RemoveTotalSupplySubject<'info> {
    /// Existing confidential mint authority. Governance may own this key later.
    pub authority: Signer<'info>,
    #[account(has_one = authority @ ConfidentialTokenError::MintAuthorityMismatch)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// CHECK: encrypted total-supply authority PDA; signs the host CPI.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Encrypted total-supply value whose subjects are changed.
    #[account(mut, address = mint.total_supply_encrypted_value)]
    pub total_supply_value: Box<Account<'info, zama_host::EncryptedValue>>,
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    pub zama_program: Program<'info, ZamaHost>,
}

/// Accounts for mint-authority public sealing of encrypted total supply.
#[derive(Accounts)]
pub struct MakeTotalSupplyHandlePublic<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    #[account(has_one = authority @ ConfidentialTokenError::MintAuthorityMismatch)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// CHECK: encrypted total-supply authority PDA; signs the host CPI.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Encrypted total-supply value whose current handle is sealed.
    #[account(mut, address = mint.total_supply_encrypted_value)]
    pub total_supply_value: Box<Account<'info, zama_host::EncryptedValue>>,
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}

fn assert_total_supply_authority(
    value: &Account<zama_host::EncryptedValue>,
    mint: Pubkey,
    authority: Pubkey,
) -> Result<()> {
    require_keys_eq!(
        authority,
        total_supply_authority_address(mint).0,
        ConfidentialTokenError::TotalSupplyAuthorityMismatch
    );
    require_keys_eq!(value.domain, mint, ConfidentialTokenError::DomainMismatch);
    require_keys_eq!(
        value.encrypted_value_account_authority,
        authority,
        ConfidentialTokenError::TotalSupplyAuthorityMismatch
    );
    require!(
        value.label == encrypted_total_supply_label(),
        ConfidentialTokenError::TotalSupplyValueMismatch
    );
    require_keys_eq!(
        value.key(),
        encrypted_value_address(mint, authority, encrypted_total_supply_label()).0,
        ConfidentialTokenError::TotalSupplyValueMismatch
    );
    Ok(())
}

/// Grants decrypt subjects on the encrypted total supply.
pub fn allow_total_supply_subjects<'info>(
    ctx: Context<'info, AllowTotalSupplySubjects<'info>>,
    subjects: Vec<Pubkey>,
) -> Result<()> {
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    let mint = ctx.accounts.mint.key();
    assert_total_supply_authority(
        &ctx.accounts.total_supply_value,
        mint,
        ctx.accounts.total_supply_authority.key(),
    )?;
    if !ctx.accounts.host_config.grant_deny_list_enabled {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
    }

    let bump = [ctx.bumps.total_supply_authority];
    let seeds: &[&[u8]] = &[b"total-supply", mint.as_ref(), &bump];
    let deny_record = ctx
        .accounts
        .deny_subject_record
        .as_ref()
        .map(|account| account.to_account_info());
    cpi::allow_subjects(
        CpiContext::new_with_signer(
            ctx.accounts.zama_program.key(),
            cpi::accounts::AllowEncryptedValueSubjects {
                payer: ctx.accounts.payer.to_account_info(),
                authority: ctx.accounts.total_supply_authority.to_account_info(),
                encrypted_value: ctx.accounts.total_supply_value.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                deny_subject_record: deny_record,
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &[seeds],
        )
        .with_remaining_accounts(ctx.remaining_accounts.to_vec()),
        subjects,
    )
}

/// Removes one decrypt subject from the encrypted total supply.
pub fn remove_total_supply_subject(
    ctx: Context<RemoveTotalSupplySubject>,
    subject: Pubkey,
) -> Result<()> {
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    let mint = ctx.accounts.mint.key();
    assert_total_supply_authority(
        &ctx.accounts.total_supply_value,
        mint,
        ctx.accounts.total_supply_authority.key(),
    )?;
    assert_no_remaining_accounts(ctx.remaining_accounts)?;

    let bump = [ctx.bumps.total_supply_authority];
    let seeds: &[&[u8]] = &[b"total-supply", mint.as_ref(), &bump];
    cpi::remove_subject(
        CpiContext::new_with_signer(
            ctx.accounts.zama_program.key(),
            cpi::accounts::RemoveEncryptedValueSubject {
                authority: ctx.accounts.total_supply_authority.to_account_info(),
                encrypted_value: ctx.accounts.total_supply_value.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
            },
            &[seeds],
        ),
        subject,
    )
}

/// Seals the encrypted total supply's current handle as publicly decryptable.
pub fn make_total_supply_handle_public(
    ctx: Context<MakeTotalSupplyHandlePublic>,
    handle: [u8; 32],
) -> Result<()> {
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let mint = ctx.accounts.mint.key();
    assert_total_supply_authority(
        &ctx.accounts.total_supply_value,
        mint,
        ctx.accounts.total_supply_authority.key(),
    )?;

    let bump = [ctx.bumps.total_supply_authority];
    let seeds: &[&[u8]] = &[b"total-supply", mint.as_ref(), &bump];
    cpi::make_handle_public(
        CpiContext::new_with_signer(
            ctx.accounts.zama_program.key(),
            cpi::accounts::MakeEncryptedValueHandlePublic {
                payer: ctx.accounts.payer.to_account_info(),
                authority: ctx.accounts.total_supply_authority.to_account_info(),
                encrypted_value: ctx.accounts.total_supply_value.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &[seeds],
        ),
        handle,
    )
}
