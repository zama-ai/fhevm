use anchor_lang::prelude::*;
use anchor_spl::token::{self as spl_token, Mint as SplMint, Token, TokenAccount, TransferChecked};
use zama_fhe as fhe;
use zama_host::{self, program::ZamaHost};

use crate::{
    balance_acl_subjects, balance_label, balance_nonce_key,
    constant::{
        CONFIDENTIAL_MINT, CONFIDENTIAL_TOKEN_ACCOUNT, FHE_COMPUTE, FHE_RAND_COUNTER,
        VAULT_AUTHORITY,
    },
    durable_acl_handle, fhe_context, token_app_account, withdrawal_label, withdrawal_nonce_key,
    BalanceHandleUpdateReason, BalanceHandleUpdatedEvent, ConfidentialMint,
    ConfidentialTokenAccount, ConfidentialTokenError, WithdrawalRequestedEvent, APP_EVENT_VERSION,
    BALANCE_FHE_TYPE,
};

pub fn request_unwrap_usdc(ctx: Context<RequestUnwrapUsdc>, amount: u64) -> Result<()> {
    let token_account = &ctx.accounts.token_account;
    let mint = &ctx.accounts.mint;
    let compute_signer = mint.compute_signer;
    let nonce_sequence = token_account.next_balance_nonce_sequence;

    require_keys_eq!(
        token_account.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );

    require_keys_eq!(
        token_account.mint,
        mint.key(),
        ConfidentialTokenError::MintMismatch
    );

    require_keys_eq!(
        mint.underlying_mint,
        ctx.accounts.underlying_mint.key(),
        ConfidentialTokenError::UnderlyingMintMismatch
    );
    require_keys_eq!(
        ctx.accounts.compute_signer.key(),
        compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    require_keys_eq!(
        ctx.accounts.current_compute_acl.key(),
        token_account.balance_acl_record,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );

    require!(
        token_account.pending_withdrawal_handle == [0u8; 32],
        ConfidentialTokenError::PendingWithdrawal
    );
    require_keys_eq!(
        Pubkey::default(),
        token_account.pending_withdrawal_acl_record,
        ConfidentialTokenError::PendingWithdrawal
    );

    fhe::execute(
        fhe_context(
            &ctx.accounts.owner,
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.compute_signer,
            &ctx.accounts.zama_rand_counter,
            mint.key(),
            ctx.bumps.compute_signer,
            &ctx.accounts.system_program,
        ),
        |fhe| {
            let current_balance = fhe.encrypted(fhe::EncryptedValue {
                handle: token_account.balance_handle,
                acl_record: ctx.accounts.current_compute_acl.to_account_info(),
            })?;
            let amount = fhe.trivial_encrypt_u64(amount, BALANCE_FHE_TYPE)?;
            let zero = fhe.trivial_encrypt_u64(0, BALANCE_FHE_TYPE)?;
            let unwrap_allowed = fhe.ge(current_balance, amount.clone())?;
            let allowed_withdrawal =
                fhe.if_then_else(unwrap_allowed, amount, zero, BALANCE_FHE_TYPE)?;
            fhe.allow(
                &allowed_withdrawal,
                fhe::DurableAllow {
                    acl_record: ctx.accounts.output_acl.to_account_info(),
                    app_account: token_app_account(token_account.to_account_info()),
                    nonce_key: withdrawal_nonce_key(mint.key(), token_account.key()),
                    nonce_sequence,
                    encrypted_value_label: withdrawal_label(),
                    subjects: balance_acl_subjects(token_account.owner, compute_signer),
                    public_decrypt: true,
                },
            )?;
            Ok(())
        },
    )?;
    let withdrawal_handle = durable_acl_handle(&ctx.accounts.output_acl.to_account_info())?;

    let token_account = &mut ctx.accounts.token_account;
    // Record the approved withdrawal as pending; the balance is left untouched
    // here and is only debited in `finalize_unwrap_usdc`, once the coprocessor
    // has publicly decrypted this handle.
    token_account.pending_withdrawal_handle = withdrawal_handle;
    token_account.pending_withdrawal_acl_record = ctx.accounts.output_acl.key();
    // PoC simplicity: the withdrawal ACL reuses the balance nonce counter. This
    // can be split into a dedicated withdrawal nonce space after the PoC.
    token_account.next_balance_nonce_sequence = nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
    emit_cpi!(WithdrawalRequestedEvent {
        version: APP_EVENT_VERSION,
        mint: mint.key(),
        owner: token_account.owner,
        token_account: token_account.key(),
        withdrawal_handle,
        withdrawal_acl_record: ctx.accounts.output_acl.key(),
        nonce_sequence,
    });

    Ok(())
}

#[derive(Accounts)]
#[event_cpi]
pub struct RequestUnwrapUsdc<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(seeds = [CONFIDENTIAL_MINT, underlying_mint.key().as_ref()], bump)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    #[account(mut, seeds = [CONFIDENTIAL_TOKEN_ACCOUNT, mint.key().as_ref(), owner.key().as_ref()], bump)]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// spl mints does not have derivation scheme
    pub underlying_mint: Box<Account<'info, SplMint>>,
    /// CHECK: PDA authority for the underlying-token vault.
    #[account(seeds = [VAULT_AUTHORITY, mint.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [FHE_COMPUTE, mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    pub current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub output_acl: UncheckedAccount<'info>,
    /// CHECK: global Zama host rand counter PDA.
    #[account(
        mut,
        seeds = [FHE_RAND_COUNTER],
        bump,
        seeds::program = zama_program.key()
    )]
    pub zama_rand_counter: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    pub zama_program: Program<'info, ZamaHost>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

/// Phase 2 of unwrap: completes a withdrawal whose approved amount has been
/// publicly decrypted by the coprocessor. Transfers the cleartext amount from
/// the vault to the user and debits the encrypted balance by the same handle.
///
/// PoC: `amount` is the cleartext of `pending_withdrawal_handle`, delivered by
/// the relayer. It is currently trusted (we only check public-decrypt
/// eligibility via the ACL record). Production MUST verify a KMS-signed
/// public-decrypt proof binding `(pending_withdrawal_handle, amount)` — the
/// analog of EVM `FHE.checkSignatures` — before releasing funds.
pub fn finalize_unwrap_usdc(ctx: Context<FinalizeUnwrapUsdc>, amount: u64) -> Result<()> {
    let token_account = &ctx.accounts.token_account;
    let mint = &ctx.accounts.mint;
    let compute_signer = mint.compute_signer;
    let nonce_sequence = token_account.next_balance_nonce_sequence;
    let old_balance_handle = token_account.balance_handle;
    let old_balance_acl_record = token_account.balance_acl_record;
    let pending_withdrawal_handle = token_account.pending_withdrawal_handle;

    require_keys_eq!(
        token_account.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    require_keys_eq!(
        token_account.mint,
        mint.key(),
        ConfidentialTokenError::MintMismatch
    );
    require_keys_eq!(
        mint.underlying_mint,
        ctx.accounts.underlying_mint.key(),
        ConfidentialTokenError::UnderlyingMintMismatch
    );
    require_keys_eq!(
        ctx.accounts.compute_signer.key(),
        compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    require_keys_eq!(
        ctx.accounts.current_compute_acl.key(),
        token_account.balance_acl_record,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );

    // There must be a pending withdrawal, and the supplied ACL record must be it.
    require!(
        pending_withdrawal_handle != [0u8; 32],
        ConfidentialTokenError::NoPendingWithdrawal
    );
    require_keys_eq!(
        ctx.accounts.withdrawal_acl.key(),
        token_account.pending_withdrawal_acl_record,
        ConfidentialTokenError::PendingWithdrawalMismatch
    );
    // Eligibility: the ACL record must bind the pending handle and be publicly
    // decryptable. (This is the eligibility half of the KMS check, mirroring
    // `kms_like_public_decrypt_check` — NOT authenticity of `amount`.)
    require!(
        ctx.accounts.withdrawal_acl.handle == pending_withdrawal_handle,
        ConfidentialTokenError::PendingWithdrawalMismatch
    );
    require!(
        ctx.accounts.withdrawal_acl.public_decrypt,
        ConfidentialTokenError::PendingWithdrawalMismatch
    );

    // TODO(PoC hardening): verify a KMS-signed public-decrypt proof binding
    // (pending_withdrawal_handle, amount) before releasing funds, e.g. by CPI
    // into a `zama_host::verify_public_decrypt(handle, amount, proof)` primitive.
    // Until then `amount` is trusted from the caller.

    // Transfer the approved amount from the vault to the user (vault PDA signs).
    let mint_key = mint.key();
    let vault_seeds: &[&[u8]] = &[
        VAULT_AUTHORITY,
        mint_key.as_ref(),
        &[ctx.bumps.vault_authority],
    ];
    spl_token::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            TransferChecked {
                from: ctx.accounts.vault_usdc.to_account_info(),
                mint: ctx.accounts.underlying_mint.to_account_info(),
                to: ctx.accounts.user_usdc.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            &[vault_seeds],
        ),
        amount,
        mint.decimals,
    )?;

    // Debit the encrypted balance by the same (now-revealed) approved withdrawal
    // handle, so the plaintext transferred equals the encrypted amount debited.
    fhe::execute(
        fhe_context(
            &ctx.accounts.owner,
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.compute_signer,
            &ctx.accounts.zama_rand_counter,
            mint.key(),
            ctx.bumps.compute_signer,
            &ctx.accounts.system_program,
        ),
        |fhe| {
            let current_balance = fhe.encrypted(fhe::EncryptedValue {
                handle: token_account.balance_handle,
                acl_record: ctx.accounts.current_compute_acl.to_account_info(),
            })?;
            let withdrawal = fhe.encrypted(fhe::EncryptedValue {
                handle: pending_withdrawal_handle,
                acl_record: ctx.accounts.withdrawal_acl.to_account_info(),
            })?;
            let new_balance = fhe.sub(current_balance, withdrawal, BALANCE_FHE_TYPE)?;
            fhe.allow(
                &new_balance,
                fhe::DurableAllow {
                    acl_record: ctx.accounts.output_acl.to_account_info(),
                    app_account: token_app_account(token_account.to_account_info()),
                    nonce_key: balance_nonce_key(mint.key(), token_account.key()),
                    nonce_sequence,
                    encrypted_value_label: balance_label(),
                    subjects: balance_acl_subjects(token_account.owner, compute_signer),
                    public_decrypt: false,
                },
            )?;
            Ok(())
        },
    )?;
    let new_balance_handle = durable_acl_handle(&ctx.accounts.output_acl.to_account_info())?;

    let token_account = &mut ctx.accounts.token_account;
    token_account.balance_handle = new_balance_handle;
    token_account.balance_acl_record = ctx.accounts.output_acl.key();
    token_account.next_balance_nonce_sequence = nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
    // Clear the pending withdrawal now that it has been finalized.
    token_account.pending_withdrawal_handle = [0; 32];
    token_account.pending_withdrawal_acl_record = Pubkey::default();
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint.key(),
        owner: token_account.owner,
        token_account: token_account.key(),
        old_handle: old_balance_handle,
        old_acl_record: old_balance_acl_record,
        new_handle: new_balance_handle,
        new_acl_record: ctx.accounts.output_acl.key(),
        reason: BalanceHandleUpdateReason::Unwrap,
    });

    Ok(())
}

#[derive(Accounts)]
#[event_cpi]
pub struct FinalizeUnwrapUsdc<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(seeds = [CONFIDENTIAL_MINT, underlying_mint.key().as_ref()], bump)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    #[account(mut, seeds = [CONFIDENTIAL_TOKEN_ACCOUNT, mint.key().as_ref(), owner.key().as_ref()], bump)]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// spl mints does not have derivation scheme
    pub underlying_mint: Box<Account<'info, SplMint>>,
    #[account(
        mut,
        constraint = user_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = user_usdc.owner == owner.key() @ ConfidentialTokenError::OwnerMismatch
    )]
    pub user_usdc: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = vault_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = vault_usdc.owner == vault_authority.key() @ ConfidentialTokenError::VaultAuthorityMismatch
    )]
    pub vault_usdc: Box<Account<'info, TokenAccount>>,
    /// CHECK: PDA authority for the underlying-token vault.
    #[account(seeds = [VAULT_AUTHORITY, mint.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [FHE_COMPUTE, mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    pub current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// The pending withdrawal ACL record (allowed for public decryption),
    /// consumed by this instruction.
    pub withdrawal_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub output_acl: UncheckedAccount<'info>,
    /// CHECK: global Zama host rand counter PDA.
    #[account(
        mut,
        seeds = [FHE_RAND_COUNTER],
        bump,
        seeds::program = zama_program.key()
    )]
    pub zama_rand_counter: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    pub zama_program: Program<'info, ZamaHost>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
