use super::*;

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
    let nonce_key = nonce_key(mint_key, owner, encrypted_value_label);
    let request = fhe::RandU64 {
        payer: &ctx.accounts.owner,
        event_authority: &ctx.accounts.zama_event_authority,
        zama_program: &ctx.accounts.zama_program,
        host_config: &ctx.accounts.host_config,
        compute_signer: &ctx.accounts.compute_signer,
        app_account_authority: &ctx.accounts.owner,
        output_acl_record: ctx.accounts.amount_acl_record.to_account_info(),
        acl_domain_key: mint_key,
        compute_signer_bump: ctx.bumps.compute_signer,
        system_program: &ctx.accounts.system_program,
        output_nonce_key: nonce_key,
        output_nonce_sequence: nonce_sequence,
        output_encrypted_value_label: encrypted_value_label,
        output_subjects: compute_acl_subject(ctx.accounts.compute_signer.key()),
        output_public_decrypt: false,
    };

    let handle = match upper_bound {
        Some(upper_bound) => fhe::rand_bounded_u64(request, upper_bound)?,
        None => fhe::rand_u64(request)?,
    };
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
