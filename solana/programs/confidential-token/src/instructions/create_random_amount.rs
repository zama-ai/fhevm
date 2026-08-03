//! Creates token-scoped random encrypted amounts.

use super::*;

/// Accounts for creating a token-scoped random encrypted amount.
#[derive(Accounts)]
#[event_cpi]
pub struct CreateRandomAmount<'info> {
    /// Token account owner and rent payer.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint that scopes the encrypted amount.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Owner's confidential token account.
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: stable per-owner amount encrypted value account; created on first use, replaced thereafter.
    #[account(mut)]
    pub amount_value: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to create the random handle.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for encrypted value account creation/growth.
    pub system_program: Program<'info, System>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_execute` CPI, which validates it against the
    /// canonical `["hcu-block-meter", compute_signer]` PDA. Supplied by an untrusted mint under a
    /// metering-band cap; omitted when the mint is trusted or the cap is unrestricted.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_execute` CPI, which validates it against the
    /// canonical `["hcu-trusted", compute_signer]` PDA. Present + valid bypasses the cap; absent
    /// means the mint is metered.
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
}

/// Creates a token-scoped random encrypted amount for transfer or burn flows.
pub fn create_random_amount<'info>(
    ctx: Context<'info, CreateRandomAmount<'info>>,
    amount_kind: ConfidentialAmountKind,
) -> Result<()> {
    create_random_amount_inner(ctx, amount_kind, None)
}

/// Creates a token-scoped bounded random encrypted amount for transfer or burn flows.
pub fn create_random_bounded_amount<'info>(
    ctx: Context<'info, CreateRandomAmount<'info>>,
    amount_kind: ConfidentialAmountKind,
    upper_bound: [u8; 32],
) -> Result<()> {
    create_random_amount_inner(ctx, amount_kind, Some(upper_bound))
}

fn create_random_amount_inner<'info>(
    ctx: Context<'info, CreateRandomAmount<'info>>,
    amount_kind: ConfidentialAmountKind,
    upper_bound: Option<[u8; 32]>,
) -> Result<()> {
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    let mint_key = ctx.accounts.mint.key();
    let mint_domain = zama_fhe::Domain::new(mint_key);
    let owner = ctx.accounts.owner.key();
    let token_account_key = ctx.accounts.token_account.key();
    require_keys_eq!(
        ctx.accounts.token_account.owner,
        owner,
        ConfidentialTokenError::OwnerMismatch
    );
    require_keys_eq!(
        ctx.accounts.token_account.mint,
        mint_key,
        ConfidentialTokenError::MintMismatch
    );
    assert_confidential_token_account_shape(&ctx.accounts.token_account, mint_key, owner)?;
    require_keys_eq!(
        ctx.accounts.compute_signer.key(),
        ctx.accounts.mint.compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );

    let label = amount_kind.label();
    let amount_output = fhe::PersistentOutput::new(
        ctx.accounts.amount_value.to_account_info(),
        encrypted_value_id(mint_domain, owner, label),
        fhe::PersistentAudience::compute_only(ctx.accounts.compute_signer.key()),
    )?;
    let execution =
        zama_fhe::FheExecution::build(zama_fhe::ExecutionAppAuthority::new(owner), |builder| {
            match upper_bound {
                Some(upper_bound) => builder.rand_bounded_u64(
                    zama_fhe::BoundedU64UpperBound::from_be_bytes(upper_bound)?,
                    amount_output.output(),
                )?,
                None => builder.rand_u64(amount_output.output())?,
            };
            Ok(())
        })
        .map_err(invalid_execution)?;
    let compute_authority = fhe::ComputeAuthority::for_mint(
        &ctx.accounts.compute_signer,
        mint_key,
        ctx.bumps.compute_signer,
    )?;
    let execution_accounts = fhe::ExecutionAccountSet::for_execution(
        &execution,
        [amount_output.account_info()],
        [fhe::OutputAuthority::transaction_signer(
            &ctx.accounts.owner,
        )],
    )?;
    fhe::execute(fhe::Execute {
        context: fhe::ExecuteContext {
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
        accounts: &execution_accounts,
        execution,
    })?;
    let handle = amount_output.handle()?;
    emit_cpi!(RandomAmountCreatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner,
        token_account: token_account_key,
        amount_kind,
        handle,
        encrypted_value: ctx.accounts.amount_value.key(),
    });
    Ok(())
}
