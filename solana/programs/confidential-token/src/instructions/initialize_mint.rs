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
    /// CHECK: total-supply encrypted-value ACL lineage; created via the Zama host CPI.
    #[account(mut)]
    pub total_supply_value_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to create the initial total-supply handle.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Initializes a confidential mint and records its host ACL domain.
pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
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
    let total_supply_authority_bump = total_supply_authority_address(mint_key).1;
    // Total supply is a transient eval output (no per-rotation ACL record); its
    // computed handle returns via fhe_eval and is bound into the total-supply
    // encrypted-value ACL lineage below.
    let context_id = transfer_eval_context(
        b"initialize-total-supply",
        mint_key,
        total_supply_authority,
        total_supply_authority,
        [0; 32],
        0,
        0,
    )?;
    let mut builder = zama_fhe::EvalBuilder::new(
        context_id,
        zama_fhe::EvalAppAuthority::new(total_supply_authority),
    );
    let total_supply = builder
        .trivial_encrypt_u64(0, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let total_supply_index = total_supply
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
            payer: &ctx.accounts.authority,
            event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            compute_authority,
            system_program: &ctx.accounts.system_program,
        },
        fhe::OutputAuthority::total_supply(
            &ctx.accounts.total_supply_authority,
            mint_key,
            total_supply_authority_bump,
        )?,
        [],
        plan,
    )?;
    let total_supply_handle = fhe::read_eval_output_handle(total_supply_index)?;
    upsert_value_acl(
        &LineageCpi {
            zama_program: ctx.accounts.zama_program.to_account_info(),
            encrypted_value_acl: ctx.accounts.total_supply_value_acl.to_account_info(),
            payer: ctx.accounts.authority.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        LineageAuthority::total_supply(
            &ctx.accounts.total_supply_authority,
            mint_key,
            total_supply_authority_bump,
        ),
        mint_key,
        total_supply_handle,
        vec![compute_signer],
    )?;
    let mint = &mut ctx.accounts.mint;
    mint.authority = ctx.accounts.authority.key();
    mint.acl_domain_key = mint_key;
    mint.compute_signer = compute_signer;
    mint.underlying_mint = ctx.accounts.underlying_mint.key();
    mint.decimals = ctx.accounts.underlying_mint.decimals;
    mint.total_supply_handle = total_supply_handle;
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        old_handle: [0; 32],
        new_handle: total_supply_handle,
        reason: TotalSupplyUpdateReason::Initialize,
    });
    Ok(())
}
