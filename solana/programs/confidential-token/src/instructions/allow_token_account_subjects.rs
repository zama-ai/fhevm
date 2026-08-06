//! Owner-authorized host `allow_subjects` / `remove_subject` CPIs signed as the
//! token-account PDA (`EncryptedValue.encrypted_value_account_authority` for
//! balance-scoped values).
//!
//! Host subject-list mutation requires the signer to equal
//! `EncryptedValue.encrypted_value_account_authority` (fhevm-internal#1862 #13).
//! Wallet owners are decrypt/compute subjects, not ACL admins — auditor rotation
//! goes through these instructions so the program can `invoke_signed` as the
//! token-account PDA.

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
    /// Encrypted value whose encrypted value account authority must equal `token_account`.
    #[account(mut)]
    pub encrypted_value: Box<Account<'info, zama_host::EncryptedValue>>,
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
    /// Encrypted value whose encrypted value account authority must equal `token_account`.
    #[account(mut)]
    pub encrypted_value: Box<Account<'info, zama_host::EncryptedValue>>,
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    pub zama_program: Program<'info, ZamaHost>,
}

/// Accounts for owner-authorized public sealing of a token-account state field.
#[derive(Accounts)]
pub struct MakeTokenAccountHandlePublic<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub mint: Box<Account<'info, ConfidentialMint>>,
    #[account(
        seeds = [b"token-account", mint.key().as_ref(), owner.key().as_ref()],
        bump = token_account.bump,
        has_one = owner @ ConfidentialTokenError::OwnerMismatch,
        has_one = mint @ ConfidentialTokenError::MintMismatch,
    )]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: canonical host account and exact token state binding are validated in the handler.
    #[account(mut)]
    pub encrypted_value: UncheckedAccount<'info>,
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}

pub fn allow_token_account_subjects<'info>(
    ctx: Context<'info, AllowTokenAccountSubjects<'info>>,
    subjects: Vec<Pubkey>,
) -> Result<()> {
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    assert_encrypted_value_account_is_token_account(
        &ctx.accounts.encrypted_value,
        ctx.accounts.mint.key(),
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
        CpiContext::new_with_signer(ctx.accounts.zama_program.key(), cpi_accounts, &[seeds])
            .with_remaining_accounts(remaining),
        subjects,
    )
}

pub fn remove_token_account_subject(
    ctx: Context<RemoveTokenAccountSubject>,
    subject: Pubkey,
) -> Result<()> {
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_encrypted_value_account_is_token_account(
        &ctx.accounts.encrypted_value,
        ctx.accounts.mint.key(),
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

    let cpi_accounts = cpi::accounts::RemoveEncryptedValueSubject {
        authority: ctx.accounts.token_account.to_account_info(),
        encrypted_value: ctx.accounts.encrypted_value.to_account_info(),
        host_config: ctx.accounts.host_config.to_account_info(),
    };
    cpi::remove_subject(
        CpiContext::new_with_signer(ctx.accounts.zama_program.key(), cpi_accounts, &[seeds]),
        subject,
    )
}

/// Seals the current handle of one exact token-account state field as publicly decryptable.
pub fn make_token_account_handle_public(
    ctx: Context<MakeTokenAccountHandlePublic>,
    kind: DisclosedValueKind,
    handle: [u8; 32],
) -> Result<()> {
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let mint = ctx.accounts.mint.key();
    let token_account = ctx.accounts.token_account.key();
    let label = match kind {
        DisclosedValueKind::Balance => encrypted_balance_label(),
        DisclosedValueKind::TransferredAmount => encrypted_transferred_amount_label(),
        DisclosedValueKind::BurnedAmount => encrypted_burned_amount_label(),
        DisclosedValueKind::TotalSupply => {
            return err!(ConfidentialTokenError::DisclosedValueBindingMismatch);
        }
    };
    let value = fhe::read_encrypted_value(&ctx.accounts.encrypted_value.to_account_info())?;
    require_keys_eq!(value.domain, mint, ConfidentialTokenError::DomainMismatch);
    require_keys_eq!(
        value.encrypted_value_account_authority,
        token_account,
        ConfidentialTokenError::DisclosedValueBindingMismatch
    );
    require!(
        value.label == label,
        ConfidentialTokenError::DisclosedValueBindingMismatch
    );
    require_keys_eq!(
        ctx.accounts.encrypted_value.key(),
        encrypted_value_address(mint, token_account, label).0,
        ConfidentialTokenError::DisclosedValueBindingMismatch
    );

    let owner = ctx.accounts.owner.key();
    let bump = [ctx.accounts.token_account.bump];
    let seeds: &[&[u8]] = &[
        b"token-account",
        mint.as_ref(),
        owner.as_ref(),
        bump.as_ref(),
    ];
    cpi::make_handle_public(
        CpiContext::new_with_signer(
            ctx.accounts.zama_program.key(),
            cpi::accounts::MakeEncryptedValueHandlePublic {
                payer: ctx.accounts.payer.to_account_info(),
                authority: ctx.accounts.token_account.to_account_info(),
                encrypted_value: ctx.accounts.encrypted_value.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &[seeds],
        ),
        handle,
    )
}

fn assert_encrypted_value_account_is_token_account(
    encrypted_value: &Account<zama_host::EncryptedValue>,
    mint: Pubkey,
    token_account: Pubkey,
) -> Result<()> {
    require_keys_eq!(
        encrypted_value.encrypted_value_account_authority,
        token_account,
        ConfidentialTokenError::EncryptedValueAuthorityMismatch
    );
    require_keys_eq!(
        encrypted_value.domain,
        mint,
        ConfidentialTokenError::TokenEncryptedValueMismatch
    );
    require_keys_eq!(
        encrypted_value.key(),
        encrypted_value_address(mint, token_account, encrypted_value.label).0,
        ConfidentialTokenError::TokenEncryptedValueMismatch
    );
    Ok(())
}
