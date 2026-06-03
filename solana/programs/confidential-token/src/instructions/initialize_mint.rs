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
    pub mint: Account<'info, ConfidentialMint>,
    /// Underlying SPL mint wrapped by this confidential mint.
    pub underlying_mint: Account<'info, SplMint>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: Mint-scoped app authority for total-supply handles.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// CHECK: Ed25519 authority whose KMS response certificates disclose cleartexts.
    pub kms_verifier_authority: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub total_supply_acl_record: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to create the initial total-supply handle.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Account<'info, zama_host::HostConfig>,
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
    require!(
        ctx.accounts.kms_verifier_authority.key() != Pubkey::default(),
        ConfidentialTokenError::InvalidMintConfig
    );
    let total_supply_authority = ctx.accounts.total_supply_authority.key();
    let total_supply_authority_bump = [ctx.bumps.total_supply_authority];
    let total_supply_authority_seeds: &[&[u8]] = &[
        b"total-supply",
        mint_key.as_ref(),
        &total_supply_authority_bump,
    ];
    let total_supply_acl_record = ctx.accounts.total_supply_acl_record.key();
    let total_supply_handle =
        fhe::trivial_encrypt_u64_with_app_pda(fhe::TrivialEncryptU64WithAppPda {
            payer: &ctx.accounts.authority,
            event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            compute_signer: &ctx.accounts.compute_signer,
            app_account_authority: &ctx.accounts.total_supply_authority,
            app_signer_seeds: total_supply_authority_seeds,
            output_app_account: total_supply_authority,
            output_acl_record: ctx.accounts.total_supply_acl_record.to_account_info(),
            acl_domain_key: mint_key,
            compute_signer_bump: ctx.bumps.compute_signer,
            system_program: &ctx.accounts.system_program,
            output_nonce_key: total_supply_nonce_key(mint_key, total_supply_authority),
            output_nonce_sequence: 0,
            output_encrypted_value_label: total_supply_label(),
            plaintext: 0,
            fhe_type: BALANCE_FHE_TYPE,
            output_subjects: compute_acl_subject(compute_signer),
            output_public_decrypt: false,
        })?;
    let mint = &mut ctx.accounts.mint;
    mint.authority = ctx.accounts.authority.key();
    mint.acl_domain_key = mint_key;
    mint.compute_signer = compute_signer;
    mint.underlying_mint = ctx.accounts.underlying_mint.key();
    mint.kms_verifier_authority = ctx.accounts.kms_verifier_authority.key();
    mint.decimals = ctx.accounts.underlying_mint.decimals;
    mint.total_supply_handle = total_supply_handle;
    mint.total_supply_acl_record = total_supply_acl_record;
    mint.next_total_supply_nonce_sequence = 1;
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        old_handle: [0; 32],
        old_acl_record: Pubkey::default(),
        new_handle: total_supply_handle,
        new_acl_record: total_supply_acl_record,
        reason: TotalSupplyUpdateReason::Initialize,
    });
    Ok(())
}
