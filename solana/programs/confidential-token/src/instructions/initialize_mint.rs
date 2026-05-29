use super::*;

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
