//! Initializes confidential mint state and its host ACL domain.

use super::*;

/// Accounts for initializing a confidential mint.
#[derive(Accounts)]
#[event_cpi]
pub struct InitializeMint<'info> {
    /// Mint authority and rent payer.
    #[account(mut)]
    pub authority: Signer<'info>,
    /// Confidential mint account created by this instruction.
    #[account(init, payer = authority, space = 8 + ConfidentialMint::SPACE)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Underlying SPL mint wrapped by this confidential mint.
    pub underlying_mint: Box<Account<'info, SplMint>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: Mint-scoped app authority for total-supply handles.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub total_supply_encrypted_value: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to create the initial total-supply handle.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
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

/// Initializes a confidential mint and records its host ACL domain.
pub fn initialize_mint<'info>(ctx: Context<'info, InitializeMint<'info>>) -> Result<()> {
    let mint_key = ctx.accounts.mint.key();
    let compute_signer = compute_signer_address(mint_key).0;
    require_keys_eq!(
        ctx.accounts.compute_signer.key(),
        compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    let total_supply_authority = ctx.accounts.total_supply_authority.key();
    require_keys_eq!(
        total_supply_authority,
        total_supply_authority_address(mint_key).0,
        ConfidentialTokenError::TotalSupplyAuthorityMismatch
    );
    let total_supply_encrypted_value = ctx.accounts.total_supply_encrypted_value.key();
    let total_supply_output = fhe::DurableOutput::new(
        ctx.accounts.total_supply_encrypted_value.to_account_info(),
        durable_slot(mint_key, total_supply_authority, total_supply_label()),
        fhe::DurableAudience::compute_only(compute_signer),
    )?;
    let context_id = transfer_eval_context(
        b"initialize-total-supply",
        mint_key,
        total_supply_authority,
        total_supply_authority,
        [0; 32],
    )?;
    let mut builder = zama_fhe::EvalBuilder::new(
        context_id,
        zama_fhe::EvalAppAuthority::new(total_supply_authority),
    );
    builder
        .trivial_encrypt_u64(0, total_supply_output.output())
        .map_err(invalid_eval_plan)?;
    let plan = builder.finish().map_err(invalid_eval_plan)?;
    let compute_authority = fhe::ComputeAuthority::for_mint(
        &ctx.accounts.compute_signer,
        mint_key,
        ctx.bumps.compute_signer,
    )?;
    let total_supply_authority_bump = total_supply_authority_address(mint_key).1;
    let eval_accounts = fhe::EvalAccountSet::for_plan(
        &plan,
        [total_supply_output.account_info()],
        [fhe::OutputAuthority::total_supply(
            &ctx.accounts.total_supply_authority,
            mint_key,
            total_supply_authority_bump,
        )?],
    )?;
    fhe::eval(fhe::Eval {
        context: fhe::EvalContext {
            payer: &ctx.accounts.authority,
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
    let total_supply_handle = total_supply_output.handle()?;
    let mint = &mut ctx.accounts.mint;
    mint.authority = ctx.accounts.authority.key();
    mint.acl_domain_key = mint_key;
    mint.compute_signer = compute_signer;
    mint.underlying_mint = ctx.accounts.underlying_mint.key();
    mint.decimals = ctx.accounts.underlying_mint.decimals;
    mint.total_supply_encrypted_value = total_supply_encrypted_value;
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        old_handle: [0; 32],
        old_encrypted_value: Pubkey::default(),
        new_handle: total_supply_handle,
        new_encrypted_value: total_supply_encrypted_value,
        reason: TotalSupplyUpdateReason::Initialize,
    });
    Ok(())
}
