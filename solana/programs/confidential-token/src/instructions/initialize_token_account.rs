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
    /// CHECK: balance encrypted-value ACL lineage; created via the Zama host CPI.
    #[account(mut)]
    pub balance_value_acl: UncheckedAccount<'info>,
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
    {
        let token_account = &mut ctx.accounts.token_account;
        token_account.owner = ctx.accounts.owner.key();
        token_account.mint = ctx.accounts.mint.key();
        token_account.balance_handle = [0; 32];
        token_account.next_amount_nonce_sequence = 0;
        token_account.bump = ctx.bumps.token_account;
    }
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
    let mint_key = ctx.accounts.mint.key();
    let owner = ctx.accounts.owner.key();
    let compute_signer = ctx.accounts.compute_signer.key();
    let token_account_key = ctx.accounts.token_account.key();
    let context_id = transfer_eval_context(
        b"initialize-balance",
        mint_key,
        token_account_key,
        token_account_key,
        [0; 32],
        0,
        0,
    )?;
    // The balance is a transient eval output (no per-rotation ACL record); its
    // computed handle returns via fhe_eval set_return_data and is bound into the
    // balance encrypted-value ACL lineage below.
    let mut builder = zama_fhe::EvalBuilder::new(
        context_id,
        zama_fhe::EvalAppAuthority::new(token_account_key),
    );
    let balance = builder
        .trivial_encrypt_u64(initial_balance, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let balance_index = balance
        .producer_index()
        .ok_or(error!(ConfidentialTokenError::InvalidFheEvalPlan))?;
    let plan = builder.finish().map_err(invalid_eval_plan)?;
    let compute_authority = fhe::ComputeAuthority::for_mint(
        &ctx.accounts.compute_signer,
        mint_key,
        ctx.bumps.compute_signer,
    )?;
    fhe::eval_transient(
        fhe::EvalContext {
            payer: &ctx.accounts.owner,
            event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            compute_authority,
            system_program: &ctx.accounts.system_program,
        },
        fhe::OutputAuthority::token_account(&ctx.accounts.token_account)?,
        [],
        plan,
    )?;
    let balance_handle = fhe::read_eval_output_handle(balance_index)?;
    upsert_value_acl(
        &LineageCpi {
            zama_program: ctx.accounts.zama_program.to_account_info(),
            encrypted_value_acl: ctx.accounts.balance_value_acl.to_account_info(),
            payer: ctx.accounts.owner.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        LineageAuthority::balance(&ctx.accounts.token_account),
        mint_key,
        balance_handle,
        vec![owner, compute_signer],
    )?;
    let token_account = &mut ctx.accounts.token_account;
    token_account.balance_handle = balance_handle;
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: ctx.accounts.mint.key(),
        owner: ctx.accounts.owner.key(),
        token_account: token_account.key(),
        old_handle: [0; 32],
        new_handle: balance_handle,
        reason: BalanceHandleUpdateReason::Initialize,
    });
    Ok(())
}
