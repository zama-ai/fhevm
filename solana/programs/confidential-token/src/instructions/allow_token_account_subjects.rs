//! Owner-authorized host `allow_subjects` / `remove_subject` CPIs signed as the
//! token-account PDA (`EncryptedValue.account` for balance-scoped values).
//!
//! Host subject-list mutation requires the signer to equal `EncryptedValue.account`
//! (fhevm-internal#1862 #13). Wallet owners are decrypt/compute subjects, not ACL
//! admins — auditor rotation goes through these instructions so the program can
//! `invoke_signed` as the token-account PDA.

use super::*;
use zama_host::cpi;

/// Accounts for granting additional subjects on a token-account-scoped encrypted value.
#[derive(Accounts)]
pub struct AllowTokenAccountSubjects<'info> {
    /// Pays for EncryptedValue realloc growth on the host.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Token account owner authorizing the grant.
    pub owner: Signer<'info>,
    pub mint: Box<Account<'info, ConfidentialMint>>,
    #[account(
        seeds = [b"token-account", mint.key().as_ref(), owner.key().as_ref()],
        bump = token_account.bump,
        has_one = owner @ ConfidentialTokenError::OwnerMismatch,
        has_one = mint @ ConfidentialTokenError::MintMismatch,
    )]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Encrypted value whose `account` field must equal `token_account`.
    #[account(mut)]
    pub encrypted_value: Box<Account<'info, zama_host::EncryptedValue>>,
    #[account(
        seeds = [zama_host::HOST_CONFIG_SEED],
        seeds::program = zama_host::ID,
        bump = host_config.bump,
    )]
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// CHECK: forwarded to host when the grant deny-list is enabled for the authority.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}

/// Accounts for removing one subject from a token-account-scoped encrypted value.
#[derive(Accounts)]
pub struct RemoveTokenAccountSubject<'info> {
    pub owner: Signer<'info>,
    pub mint: Box<Account<'info, ConfidentialMint>>,
    #[account(
        seeds = [b"token-account", mint.key().as_ref(), owner.key().as_ref()],
        bump = token_account.bump,
        has_one = owner @ ConfidentialTokenError::OwnerMismatch,
        has_one = mint @ ConfidentialTokenError::MintMismatch,
    )]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Encrypted value whose `account` field must equal `token_account`.
    #[account(mut)]
    pub encrypted_value: Box<Account<'info, zama_host::EncryptedValue>>,
    #[account(
        seeds = [zama_host::HOST_CONFIG_SEED],
        seeds::program = zama_host::ID,
        bump = host_config.bump,
    )]
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// CHECK: forwarded to host when the grant deny-list is enabled for the authority.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    pub zama_program: Program<'info, ZamaHost>,
}

pub fn allow_token_account_subjects<'info>(
    ctx: Context<'info, AllowTokenAccountSubjects<'info>>,
    subjects: Vec<Pubkey>,
) -> Result<()> {
    assert_encrypted_value_account_is_token_account(
        &ctx.accounts.encrypted_value,
        ctx.accounts.token_account.key(),
    )?;
    // Host `allow_subjects` requires one remaining-account deny witness per newly
    // granted subject when the grant deny-list is enabled. Forward those accounts
    // unchanged; when the deny-list is off, none may be supplied.
    if !ctx.accounts.host_config.grant_deny_list_enabled {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
    }

    let mint_key = ctx.accounts.mint.key();
    let owner_key = ctx.accounts.owner.key();
    let bump = [ctx.accounts.token_account.bump];
    let seeds: &[&[u8]] = &[
        b"token-account",
        mint_key.as_ref(),
        owner_key.as_ref(),
        bump.as_ref(),
    ];

    let deny_info = ctx
        .accounts
        .deny_subject_record
        .as_ref()
        .map(|account| account.to_account_info());
    let cpi_accounts = cpi::accounts::AllowEncryptedValueSubjects {
        payer: ctx.accounts.payer.to_account_info(),
        authority: ctx.accounts.token_account.to_account_info(),
        encrypted_value: ctx.accounts.encrypted_value.to_account_info(),
        host_config: ctx.accounts.host_config.to_account_info(),
        deny_subject_record: deny_info,
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let remaining = ctx.remaining_accounts.to_vec();
    cpi::allow_subjects(
        CpiContext::new_with_signer(
            ctx.accounts.zama_program.key(),
            cpi_accounts,
            &[seeds],
        )
        .with_remaining_accounts(remaining),
        subjects,
    )
}

pub fn remove_token_account_subject(
    ctx: Context<RemoveTokenAccountSubject>,
    subject: Pubkey,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_encrypted_value_account_is_token_account(
        &ctx.accounts.encrypted_value,
        ctx.accounts.token_account.key(),
    )?;

    let mint_key = ctx.accounts.mint.key();
    let owner_key = ctx.accounts.owner.key();
    let bump = [ctx.accounts.token_account.bump];
    let seeds: &[&[u8]] = &[
        b"token-account",
        mint_key.as_ref(),
        owner_key.as_ref(),
        bump.as_ref(),
    ];

    let deny_info = ctx
        .accounts
        .deny_subject_record
        .as_ref()
        .map(|account| account.to_account_info());
    let cpi_accounts = cpi::accounts::RemoveEncryptedValueSubject {
        authority: ctx.accounts.token_account.to_account_info(),
        encrypted_value: ctx.accounts.encrypted_value.to_account_info(),
        host_config: ctx.accounts.host_config.to_account_info(),
        deny_subject_record: deny_info,
    };
    cpi::remove_subject(
        CpiContext::new_with_signer(
            ctx.accounts.zama_program.key(),
            cpi_accounts,
            &[seeds],
        ),
        subject,
    )
}

fn assert_encrypted_value_account_is_token_account(
    encrypted_value: &Account<zama_host::EncryptedValue>,
    token_account: Pubkey,
) -> Result<()> {
    require_keys_eq!(
        encrypted_value.account,
        token_account,
        ConfidentialTokenError::CurrentEncryptedValueMismatch
    );
    Ok(())
}
