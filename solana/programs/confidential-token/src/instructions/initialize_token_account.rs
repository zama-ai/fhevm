//! Initializes confidential token accounts and their initial balance handles.

use super::*;

/// Accounts for initializing a confidential token account.
#[derive(Accounts)]
#[event_cpi]
pub struct InitializeTokenAccount<'info> {
    /// Account owner and rent payer.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint this account belongs to.
    pub mint: Account<'info, ConfidentialMint>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + ConfidentialTokenAccount::SPACE,
        seeds = [b"token-account", mint.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub token_account: Account<'info, ConfidentialTokenAccount>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub acl_record: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to create the initial balance handle.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Account<'info, zama_host::HostConfig>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Initializes a token account and creates its initial confidential balance handle.
pub fn initialize_token_account(
    ctx: Context<InitializeTokenAccount>,
    initial_balance: u64,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    require!(
        initial_balance == 0,
        ConfidentialTokenError::NonZeroInitialBalanceUnsupported
    );
    let token_account = &mut ctx.accounts.token_account;
    token_account.owner = ctx.accounts.owner.key();
    token_account.mint = ctx.accounts.mint.key();
    token_account.balance_handle = [0; 32];
    token_account.balance_acl_record = Pubkey::default();
    token_account.next_balance_nonce_sequence = 1;
    token_account.next_amount_nonce_sequence = 0;
    token_account.bump = ctx.bumps.token_account;
    require_keys_eq!(
        ctx.accounts.mint.acl_domain_key,
        ctx.accounts.mint.key(),
        ConfidentialTokenError::AclDomainKeyMismatch
    );
    require_keys_eq!(
        ctx.accounts.compute_signer.key(),
        ctx.accounts.mint.compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    let acl_record = ctx.accounts.acl_record.key();
    let balance_handle = trivial_encrypt_balance_acl(
        &ctx.accounts.owner,
        &ctx.accounts.mint,
        &ctx.accounts.compute_signer,
        &ctx.accounts.token_account,
        ctx.accounts.acl_record.to_account_info(),
        &ctx.accounts.zama_event_authority,
        &ctx.accounts.zama_program,
        &ctx.accounts.host_config,
        &ctx.accounts.system_program,
        ctx.bumps.compute_signer,
        0,
        initial_balance,
    )?;
    let token_account = &mut ctx.accounts.token_account;
    token_account.balance_handle = balance_handle;
    token_account.balance_acl_record = acl_record;
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: ctx.accounts.mint.key(),
        owner: ctx.accounts.owner.key(),
        token_account: token_account.key(),
        old_handle: [0; 32],
        old_acl_record: Pubkey::default(),
        new_handle: balance_handle,
        new_acl_record: acl_record,
        reason: BalanceHandleUpdateReason::Initialize,
    });
    Ok(())
}
