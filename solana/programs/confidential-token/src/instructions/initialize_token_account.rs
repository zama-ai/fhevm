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
    pub balance_encrypted_value: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to create the initial balance handle.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Account<'info, zama_host::HostConfig>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it against the
    /// canonical `["hcu-block-meter", compute_signer]` PDA. Supplied by an untrusted mint under a
    /// metering-band cap; omitted when the mint is trusted or the cap is unrestricted.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it against the
    /// canonical `["hcu-trusted", compute_signer]` PDA. Present + valid bypasses the cap; absent
    /// means the mint is metered.
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
}

/// Initializes a token account and creates its initial confidential balance handle.
pub fn initialize_token_account<'info>(
    ctx: Context<'info, InitializeTokenAccount<'info>>,
    initial_balance: u64,
) -> Result<()> {
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    require!(
        initial_balance == 0,
        ConfidentialTokenError::NonZeroInitialBalanceUnsupported
    );
    {
        let token_account = &mut ctx.accounts.token_account;
        token_account.owner = ctx.accounts.owner.key();
        token_account.mint = ctx.accounts.mint.key();
        token_account.balance_encrypted_value = Pubkey::default();
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
    let balance_encrypted_value = ctx.accounts.balance_encrypted_value.key();
    let balance_output = fhe::DurableOutput::new(
        ctx.accounts.balance_encrypted_value.to_account_info(),
        durable_slot(mint_key, token_account_key, balance_label()),
        fhe::DurableAudience::for_owner(owner, compute_signer),
    )?;
    let context_id = transfer_eval_context(
        b"initialize-balance",
        mint_key,
        token_account_key,
        token_account_key,
        [0; 32],
    )?;
    let mut builder = zama_fhe::EvalBuilder::new(
        context_id,
        zama_fhe::EvalAppAuthority::new(token_account_key),
    );
    builder
        .trivial_encrypt_u64(initial_balance, balance_output.output())
        .map_err(invalid_eval_plan)?;
    let plan = builder.finish().map_err(invalid_eval_plan)?;
    let compute_authority = fhe::ComputeAuthority::for_mint(
        &ctx.accounts.compute_signer,
        mint_key,
        ctx.bumps.compute_signer,
    )?;
    let eval_accounts = fhe::EvalAccountSet::for_plan(
        &plan,
        [balance_output.account_info()],
        [fhe::OutputAuthority::token_account(
            &ctx.accounts.token_account,
        )?],
    )?;
    fhe::eval(fhe::Eval {
        context: fhe::EvalContext {
            payer: &ctx.accounts.owner,
            event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            deny_subject_records: ctx.remaining_accounts,
            compute_authority,
            system_program: &ctx.accounts.system_program,
            hcu_block_meter: ctx
                .accounts
                .hcu_block_meter
                .as_ref()
                .map(|account| account.to_account_info()),
            hcu_trusted_app_record: ctx
                .accounts
                .hcu_trusted_app_record
                .as_ref()
                .map(|account| account.to_account_info()),
        },
        accounts: &eval_accounts,
        plan,
    })?;
    let balance_handle = balance_output.handle()?;
    let token_account = &mut ctx.accounts.token_account;
    token_account.balance_encrypted_value = balance_encrypted_value;
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: ctx.accounts.mint.key(),
        owner: ctx.accounts.owner.key(),
        token_account: token_account.key(),
        old_handle: [0; 32],
        old_encrypted_value: Pubkey::default(),
        new_handle: balance_handle,
        new_encrypted_value: balance_encrypted_value,
        reason: BalanceHandleUpdateReason::Initialize,
    });
    Ok(())
}
