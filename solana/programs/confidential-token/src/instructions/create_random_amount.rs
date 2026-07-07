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
    /// Owner's confidential token account carrying the amount nonce allocator.
    #[account(mut)]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub amount_acl_record: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to create the random handle.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
    /// CHECK: validated against the canonical `["hcu-authority", mint]` PDA and program-signed
    /// into the host CPI as this mint's HCU metering identity.
    pub hcu_authority: UncheckedAccount<'info>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it against the
    /// canonical `["hcu-block-meter", hcu_authority]` PDA. Supplied by an untrusted mint under a
    /// metering-band cap; omitted when the mint is trusted or the cap is unrestricted.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it against the
    /// canonical `["hcu-trusted", hcu_authority]` PDA. Present + valid bypasses the cap; absent
    /// means the mint is metered.
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
}

/// Creates a token-scoped random encrypted amount for transfer or burn flows.
pub fn create_random_amount(
    ctx: Context<CreateRandomAmount>,
    amount_kind: ConfidentialAmountKind,
) -> Result<()> {
    create_random_amount_inner(ctx, amount_kind, None)
}

/// Creates a token-scoped bounded random encrypted amount for transfer or burn flows.
pub fn create_random_bounded_amount(
    ctx: Context<CreateRandomAmount>,
    amount_kind: ConfidentialAmountKind,
    upper_bound: [u8; 32],
) -> Result<()> {
    create_random_amount_inner(ctx, amount_kind, Some(upper_bound))
}

fn create_random_amount_inner(
    ctx: Context<CreateRandomAmount>,
    amount_kind: ConfidentialAmountKind,
    upper_bound: Option<[u8; 32]>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    let mint_key = ctx.accounts.mint.key();
    let owner = ctx.accounts.owner.key();
    let token_account_key = ctx.accounts.token_account.key();
    let nonce_sequence = ctx.accounts.token_account.next_amount_nonce_sequence;
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

    let encrypted_value_label = amount_kind.encrypted_value_label();
    let amount_output = fhe::DurableOutput::new(
        ctx.accounts.amount_acl_record.to_account_info(),
        durable_slot(mint_key, owner, encrypted_value_label, nonce_sequence),
        zama_fhe::AccessPolicy::for_compute(ctx.accounts.compute_signer.key())
            .map_err(invalid_eval_plan)?,
    )?;
    let context_id = transfer_eval_context(
        b"random-amount",
        mint_key,
        owner,
        owner,
        encrypted_value_label,
        nonce_sequence,
        nonce_sequence,
    )?;
    let mut builder =
        zama_fhe::EvalBuilder::new(context_id, zama_fhe::EvalAppAuthority::new(owner));
    match upper_bound {
        Some(upper_bound) => builder
            .rand_bounded_u64(
                zama_fhe::BoundedU64UpperBound::from_be_bytes(upper_bound)
                    .map_err(invalid_eval_plan)?,
                amount_output.output(),
            )
            .map_err(invalid_eval_plan)?,
        None => builder
            .rand_u64(amount_output.output())
            .map_err(invalid_eval_plan)?,
    };
    let plan = builder.finish().map_err(invalid_eval_plan)?;
    let compute_authority = fhe::ComputeAuthority::for_mint(
        &ctx.accounts.compute_signer,
        mint_key,
        ctx.bumps.compute_signer,
    )?;
    let eval_accounts = fhe::EvalAccountSet::for_plan(
        &plan,
        [amount_output.account_info()],
        [fhe::OutputAuthority::transaction_signer(
            &ctx.accounts.owner,
        )],
    )?;
    fhe::eval(fhe::Eval {
        context: fhe::EvalContext {
            payer: &ctx.accounts.owner,
            event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            compute_authority,
            system_program: &ctx.accounts.system_program,
            hcu_authority: fhe::HcuAuthority::for_mint(&ctx.accounts.hcu_authority, mint_key)?,
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
    let handle = amount_output.handle()?;
    ctx.accounts.token_account.next_amount_nonce_sequence = nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
    emit_cpi!(RandomAmountCreatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner,
        token_account: token_account_key,
        amount_kind,
        bounded: upper_bound.is_some(),
        upper_bound: upper_bound.unwrap_or([0; 32]),
        handle,
        acl_record: ctx.accounts.amount_acl_record.key(),
        nonce_sequence,
    });
    Ok(())
}
