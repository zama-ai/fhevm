//! Confidential token app used by the Solana FHEVM PoC.
//!
//! This program is intentionally small: it demonstrates how an app can keep
//! token-specific semantics locally while delegating FHE handle creation,
//! compute ACL checks, and protocol event emission to `zama-host`.

// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]
#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

mod fhe;

use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::{get_return_data, invoke, invoke_signed, set_return_data},
    system_instruction,
};
use anchor_lang::{prelude::*, AccountDeserialize};
use anchor_spl::{
    associated_token::get_associated_token_address_with_program_id,
    token::{self as spl_token, Mint as SplMint, Token, TokenAccount, TransferChecked},
};
pub use confidential_token_receiver_sdk::{
    transfer_receiver_return_data, TransferReceiverReturn, TRANSFER_RECEIVER_RETURN_FIELD_COUNT,
    TRANSFER_RECEIVER_RETURN_LEN, TRANSFER_RECEIVER_RETURN_MAGIC,
};
use solana_instructions_sysvar::{
    load_current_index_checked, load_instruction_at_checked, ID as INSTRUCTIONS_SYSVAR_ID,
};
use zama_host::{self, program::ZamaHost, AclSubjectEntry};

declare_id!("5GKzUSfqBSNjoVW83w3xPtTnAe84srZcDTBstpSoBCR4");

const BALANCE_FHE_TYPE: u8 = 5;
const APP_EVENT_VERSION: u8 = 0;
pub const CALLBACK_SETTLEMENT_PREPARED: u8 = 1;
pub const CALLBACK_SETTLEMENT_FINALIZED: u8 = 2;
const DISCLOSURE_PROOF_DOMAIN_SEPARATOR: &[u8] = b"zama-confidential-token-disclosure-v1";
const ED25519_SIGNATURE_OFFSETS_SERIALIZED_SIZE: usize = 14;
const ED25519_SIGNATURE_OFFSETS_START: usize = 2;
const ED25519_PUBKEY_SERIALIZED_SIZE: usize = 32;
const ED25519_SIGNATURE_SERIALIZED_SIZE: usize = 64;
const ED25519_PROGRAM_ID: Pubkey =
    anchor_lang::pubkey!("Ed25519SigVerify111111111111111111111111111");

/// Anchor entrypoint module for the confidential token PoC.
#[program]
pub mod confidential_token {
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
        let token_account = &mut ctx.accounts.token_account;
        token_account.owner = ctx.accounts.owner.key();
        token_account.mint = ctx.accounts.mint.key();
        token_account.balance_handle = [0; 32];
        token_account.balance_acl_record = Pubkey::default();
        token_account.next_balance_nonce_sequence = 1;
        token_account.next_amount_nonce_sequence = 0;
        token_account.bump = ctx.bumps.token_account;
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
        let acl_record = ctx.accounts.acl_record.key();
        let balance_handle = trivial_encrypt_balance_acl(
            &ctx.accounts.owner,
            &ctx.accounts.mint,
            &ctx.accounts.compute_signer,
            &ctx.accounts.token_account,
            ctx.accounts.acl_record.to_account_info(),
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.host_config,
            &ctx.accounts.system_program,
            ctx.bumps.compute_signer,
            0,
            initial_balance,
        )?;
        let token_account = &mut ctx.accounts.token_account;
        token_account.balance_handle = balance_handle;
        token_account.balance_acl_record = acl_record;
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: ctx.accounts.mint.key(),
            owner: ctx.accounts.owner.key(),
            token_account: token_account.key(),
            old_handle: [0; 32],
            old_acl_record: Pubkey::default(),
            new_handle: balance_handle,
            new_acl_record: acl_record,
            reason: BalanceHandleUpdateReason::Initialize,
        });
        Ok(())
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

    /// Escrows public USDC and rotates the confidential balance by `amount`.
    pub fn wrap_usdc(ctx: Context<WrapUsdc>, amount: u64) -> Result<()> {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
        assert_confidential_mint_shape(&ctx.accounts.mint)?;
        let mint_key = ctx.accounts.mint.key();
        let decimals = ctx.accounts.mint.decimals;
        let compute_signer = ctx.accounts.mint.compute_signer;
        let total_supply_authority = ctx.accounts.total_supply_authority.key();
        let old_total_supply_handle = ctx.accounts.mint.total_supply_handle;
        let old_total_supply_acl_record = ctx.accounts.mint.total_supply_acl_record;
        let total_supply_nonce_sequence = ctx.accounts.mint.next_total_supply_nonce_sequence;
        let token_account = ctx.accounts.token_account.as_ref();
        let nonce_sequence = token_account.next_balance_nonce_sequence;
        let old_balance_handle = token_account.balance_handle;
        let old_balance_acl_record = token_account.balance_acl_record;

        require_keys_eq!(
            token_account.owner,
            ctx.accounts.owner.key(),
            ConfidentialTokenError::OwnerMismatch
        );
        require_keys_eq!(
            token_account.mint,
            mint_key,
            ConfidentialTokenError::MintMismatch
        );
        assert_confidential_token_account_shape(token_account, mint_key, ctx.accounts.owner.key())?;
        require_keys_eq!(
            ctx.accounts.mint.underlying_mint,
            ctx.accounts.underlying_mint.key(),
            ConfidentialTokenError::UnderlyingMintMismatch
        );
        assert_canonical_vault_token_account(
            ctx.accounts.vault_usdc.key(),
            ctx.accounts.vault_authority.key(),
            ctx.accounts.underlying_mint.key(),
        )?;
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
        require_keys_eq!(
            total_supply_authority,
            total_supply_authority_address(mint_key).0,
            ConfidentialTokenError::TotalSupplyAuthorityMismatch
        );
        assert_current_total_supply_acl(
            &ctx.accounts.current_total_supply_acl,
            ctx.accounts.current_total_supply_acl.key(),
            ctx.accounts.mint.as_ref(),
            mint_key,
            total_supply_authority,
        )?;

        spl_token::transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.key(),
                TransferChecked {
                    from: ctx.accounts.user_usdc.to_account_info(),
                    mint: ctx.accounts.underlying_mint.to_account_info(),
                    to: ctx.accounts.vault_usdc.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
            decimals,
        )?;

        let amount_handle = fhe::trivial_encrypt_u64(fhe::TrivialEncryptU64 {
            payer: &ctx.accounts.owner,
            event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            compute_signer: &ctx.accounts.compute_signer,
            app_account_authority: token_account,
            output_acl_record: ctx.accounts.amount_compute_acl.to_account_info(),
            acl_domain_key: mint_key,
            compute_signer_bump: ctx.bumps.compute_signer,
            system_program: &ctx.accounts.system_program,
            output_nonce_key: nonce_key(mint_key, token_account.key(), wrap_amount_label()),
            output_nonce_sequence: nonce_sequence,
            output_encrypted_value_label: wrap_amount_label(),
            plaintext: amount,
            fhe_type: BALANCE_FHE_TYPE,
            output_subjects: compute_acl_subject(compute_signer),
            output_public_decrypt: false,
        })?;

        let new_balance_handle = add_balance(
            &ctx.accounts.owner,
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.host_config,
            &ctx.accounts.compute_signer,
            &ctx.accounts.token_account,
            ctx.accounts.current_compute_acl.to_account_info(),
            token_account.balance_handle,
            ctx.accounts.amount_compute_acl.to_account_info(),
            amount_handle,
            ctx.accounts.output_acl.to_account_info(),
            mint_key,
            ctx.bumps.compute_signer,
            &ctx.accounts.system_program,
            nonce_sequence,
        )?;
        let total_supply_authority_bump = [ctx.bumps.total_supply_authority];
        let total_supply_authority_seeds: &[&[u8]] = &[
            b"total-supply",
            mint_key.as_ref(),
            &total_supply_authority_bump,
        ];
        let new_total_supply_handle = fhe::add_with_app_pda(fhe::BinaryOpWithAppPda {
            payer: &ctx.accounts.owner,
            event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            compute_signer: &ctx.accounts.compute_signer,
            app_account_authority: &ctx.accounts.total_supply_authority,
            app_signer_seeds: total_supply_authority_seeds,
            output_app_account: total_supply_authority,
            lhs_acl_record: ctx.accounts.current_total_supply_acl.to_account_info(),
            lhs: old_total_supply_handle,
            rhs_acl_record: ctx.accounts.amount_compute_acl.to_account_info(),
            rhs: amount_handle,
            scalar: false,
            output_acl_record: ctx.accounts.total_supply_output_acl.to_account_info(),
            output_fhe_type: BALANCE_FHE_TYPE,
            acl_domain_key: mint_key,
            compute_signer_bump: ctx.bumps.compute_signer,
            system_program: &ctx.accounts.system_program,
            output_nonce_key: total_supply_nonce_key(mint_key, total_supply_authority),
            output_nonce_sequence: total_supply_nonce_sequence,
            output_encrypted_value_label: total_supply_label(),
            output_subjects: compute_acl_subject(compute_signer),
            output_public_decrypt: false,
        })?;

        let token_account = &mut ctx.accounts.token_account;
        token_account.balance_handle = new_balance_handle;
        token_account.balance_acl_record = ctx.accounts.output_acl.key();
        token_account.next_balance_nonce_sequence = nonce_sequence
            .checked_add(1)
            .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
        let mint = &mut ctx.accounts.mint;
        mint.total_supply_handle = new_total_supply_handle;
        mint.total_supply_acl_record = ctx.accounts.total_supply_output_acl.key();
        mint.next_total_supply_nonce_sequence = total_supply_nonce_sequence
            .checked_add(1)
            .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: mint_key,
            owner: token_account.owner,
            token_account: token_account.key(),
            old_handle: old_balance_handle,
            old_acl_record: old_balance_acl_record,
            new_handle: new_balance_handle,
            new_acl_record: ctx.accounts.output_acl.key(),
            reason: BalanceHandleUpdateReason::Wrap,
        });
        emit_cpi!(TotalSupplyHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: mint_key,
            old_handle: old_total_supply_handle,
            old_acl_record: old_total_supply_acl_record,
            new_handle: new_total_supply_handle,
            new_acl_record: ctx.accounts.total_supply_output_acl.key(),
            reason: TotalSupplyUpdateReason::Wrap,
        });
        Ok(())
    }

    /// Burns an encrypted amount by rotating the account balance and encrypted total supply.
    pub fn confidential_burn(
        ctx: Context<ConfidentialBurn>,
        amount_handle: [u8; 32],
    ) -> Result<()> {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
        assert_confidential_mint_shape(&ctx.accounts.mint)?;
        let mint_key = ctx.accounts.mint.key();
        let compute_signer = ctx.accounts.mint.compute_signer;
        let total_supply_authority = ctx.accounts.total_supply_authority.key();
        let token_account = ctx.accounts.token_account.as_ref();
        let owner = token_account.owner;
        let token_account_key = token_account.key();
        let balance_nonce_sequence = token_account.next_balance_nonce_sequence;
        let old_balance_handle = token_account.balance_handle;
        let old_balance_acl_record = token_account.balance_acl_record;
        let total_supply_nonce_sequence = ctx.accounts.mint.next_total_supply_nonce_sequence;
        let old_total_supply_handle = ctx.accounts.mint.total_supply_handle;
        let old_total_supply_acl_record = ctx.accounts.mint.total_supply_acl_record;

        require_keys_eq!(
            owner,
            ctx.accounts.owner.key(),
            ConfidentialTokenError::OwnerMismatch
        );
        require_keys_eq!(
            token_account.mint,
            mint_key,
            ConfidentialTokenError::MintMismatch
        );
        assert_confidential_token_account_shape(token_account, mint_key, owner)?;
        require_keys_eq!(
            ctx.accounts.compute_signer.key(),
            compute_signer,
            ConfidentialTokenError::ComputeSignerMismatch
        );
        assert_current_balance_acl(
            &ctx.accounts.current_compute_acl,
            ctx.accounts.current_compute_acl.key(),
            token_account,
            mint_key,
        )?;
        require_keys_eq!(
            total_supply_authority,
            total_supply_authority_address(mint_key).0,
            ConfidentialTokenError::TotalSupplyAuthorityMismatch
        );
        assert_current_total_supply_acl(
            &ctx.accounts.current_total_supply_acl,
            ctx.accounts.current_total_supply_acl.key(),
            ctx.accounts.mint.as_ref(),
            mint_key,
            total_supply_authority,
        )?;
        assert_burn_amount_acl(
            &ctx.accounts.amount_compute_acl,
            amount_handle,
            mint_key,
            owner,
            compute_signer,
        )?;

        let burn_success_handle = ge_balance(
            &ctx.accounts.owner,
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.host_config,
            &ctx.accounts.compute_signer,
            token_account,
            ctx.accounts.current_compute_acl.to_account_info(),
            old_balance_handle,
            ctx.accounts.amount_compute_acl.to_account_info(),
            amount_handle,
            ctx.accounts.burn_success_acl.to_account_info(),
            mint_key,
            ctx.bumps.compute_signer,
            &ctx.accounts.system_program,
            balance_nonce_sequence,
            burn_success_label(),
        )?;
        let debit_candidate_handle = compute_balance_scratch(
            fhe::sub,
            BalanceScratch {
                payer: &ctx.accounts.owner,
                zama_event_authority: &ctx.accounts.zama_event_authority,
                zama_program: &ctx.accounts.zama_program,
                host_config: &ctx.accounts.host_config,
                compute_signer: &ctx.accounts.compute_signer,
                token_account,
                lhs_acl_record: ctx.accounts.current_compute_acl.to_account_info(),
                lhs: old_balance_handle,
                rhs_acl_record: ctx.accounts.amount_compute_acl.to_account_info(),
                rhs: amount_handle,
                output_acl_record: ctx.accounts.debit_candidate_acl.to_account_info(),
                mint: mint_key,
                compute_signer_bump: ctx.bumps.compute_signer,
                system_program: &ctx.accounts.system_program,
                output_nonce_sequence: balance_nonce_sequence,
                output_encrypted_value_label: burn_debit_candidate_label(),
                output_subjects: compute_acl_subject(ctx.accounts.compute_signer.key()),
            },
        )?;
        let new_balance_handle = select_balance(
            &ctx.accounts.owner,
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.host_config,
            &ctx.accounts.compute_signer,
            token_account,
            ctx.accounts.burn_success_acl.to_account_info(),
            burn_success_handle,
            ctx.accounts.debit_candidate_acl.to_account_info(),
            debit_candidate_handle,
            ctx.accounts.current_compute_acl.to_account_info(),
            old_balance_handle,
            ctx.accounts.output_acl.to_account_info(),
            mint_key,
            ctx.bumps.compute_signer,
            &ctx.accounts.system_program,
            balance_nonce_sequence,
        )?;
        let burned_handle = compute_balance_scratch(
            fhe::sub,
            BalanceScratch {
                payer: &ctx.accounts.owner,
                zama_event_authority: &ctx.accounts.zama_event_authority,
                zama_program: &ctx.accounts.zama_program,
                host_config: &ctx.accounts.host_config,
                compute_signer: &ctx.accounts.compute_signer,
                token_account,
                lhs_acl_record: ctx.accounts.current_compute_acl.to_account_info(),
                lhs: old_balance_handle,
                rhs_acl_record: ctx.accounts.output_acl.to_account_info(),
                rhs: new_balance_handle,
                output_acl_record: ctx.accounts.burned_amount_acl.to_account_info(),
                mint: mint_key,
                compute_signer_bump: ctx.bumps.compute_signer,
                system_program: &ctx.accounts.system_program,
                output_nonce_sequence: balance_nonce_sequence,
                output_encrypted_value_label: burned_amount_label(),
                output_subjects: burned_amount_acl_subjects(
                    owner,
                    ctx.accounts.compute_signer.key(),
                ),
            },
        )?;

        let total_supply_authority_bump = [ctx.bumps.total_supply_authority];
        let total_supply_authority_seeds: &[&[u8]] = &[
            b"total-supply",
            mint_key.as_ref(),
            &total_supply_authority_bump,
        ];
        let new_total_supply_handle = fhe::sub_with_app_pda(fhe::BinaryOpWithAppPda {
            payer: &ctx.accounts.owner,
            event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            compute_signer: &ctx.accounts.compute_signer,
            app_account_authority: &ctx.accounts.total_supply_authority,
            app_signer_seeds: total_supply_authority_seeds,
            output_app_account: total_supply_authority,
            lhs_acl_record: ctx.accounts.current_total_supply_acl.to_account_info(),
            lhs: old_total_supply_handle,
            rhs_acl_record: ctx.accounts.burned_amount_acl.to_account_info(),
            rhs: burned_handle,
            scalar: false,
            output_acl_record: ctx.accounts.total_supply_output_acl.to_account_info(),
            output_fhe_type: BALANCE_FHE_TYPE,
            acl_domain_key: mint_key,
            compute_signer_bump: ctx.bumps.compute_signer,
            system_program: &ctx.accounts.system_program,
            output_nonce_key: total_supply_nonce_key(mint_key, total_supply_authority),
            output_nonce_sequence: total_supply_nonce_sequence,
            output_encrypted_value_label: total_supply_label(),
            output_subjects: compute_acl_subject(ctx.accounts.compute_signer.key()),
            output_public_decrypt: false,
        })?;

        let token_account = &mut ctx.accounts.token_account;
        token_account.balance_handle = new_balance_handle;
        token_account.balance_acl_record = ctx.accounts.output_acl.key();
        token_account.next_balance_nonce_sequence = balance_nonce_sequence
            .checked_add(1)
            .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
        let mint = &mut ctx.accounts.mint;
        mint.total_supply_handle = new_total_supply_handle;
        mint.total_supply_acl_record = ctx.accounts.total_supply_output_acl.key();
        mint.next_total_supply_nonce_sequence = total_supply_nonce_sequence
            .checked_add(1)
            .ok_or(ConfidentialTokenError::AclNonceOverflow)?;

        emit_cpi!(ConfidentialBurnEvent {
            version: APP_EVENT_VERSION,
            mint: mint_key,
            owner,
            token_account: token_account_key,
            burned_handle,
            burned_acl_record: ctx.accounts.burned_amount_acl.key(),
        });
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: mint_key,
            owner,
            token_account: token_account_key,
            old_handle: old_balance_handle,
            old_acl_record: old_balance_acl_record,
            new_handle: new_balance_handle,
            new_acl_record: ctx.accounts.output_acl.key(),
            reason: BalanceHandleUpdateReason::BurnDebit,
        });
        emit_cpi!(TotalSupplyHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: mint_key,
            old_handle: old_total_supply_handle,
            old_acl_record: old_total_supply_acl_record,
            new_handle: new_total_supply_handle,
            new_acl_record: ctx.accounts.total_supply_output_acl.key(),
            reason: TotalSupplyUpdateReason::Burn,
        });
        Ok(())
    }

    /// Transfers an encrypted amount by rotating the sender and recipient balance handles.
    pub fn confidential_transfer(
        ctx: Context<ConfidentialTransfer>,
        amount_handle: [u8; 32],
    ) -> Result<()> {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
        require_keys_eq!(
            ctx.accounts.from_account.owner,
            ctx.accounts.owner.key(),
            ConfidentialTokenError::OwnerMismatch
        );
        let outcome = execute_transfer(
            ctx.accounts.as_transfer_accounts(),
            ctx.bumps.compute_signer,
            amount_handle,
        )?;
        if let Some(outcome) = outcome {
            emit_cpi!(ConfidentialTransferEvent {
                version: APP_EVENT_VERSION,
                mint: outcome.mint,
                from_owner: outcome.from_owner,
                from_token_account: outcome.from_token_account,
                to_owner: outcome.to_owner,
                to_token_account: outcome.to_token_account,
                transferred_handle: outcome.transferred_handle,
                transferred_acl_record: outcome.transferred_acl_record,
            });
            emit_cpi!(BalanceHandleUpdatedEvent {
                version: APP_EVENT_VERSION,
                mint: outcome.mint,
                owner: outcome.from_owner,
                token_account: outcome.from_token_account,
                old_handle: outcome.old_from_handle,
                old_acl_record: outcome.old_from_acl_record,
                new_handle: outcome.new_from_handle,
                new_acl_record: outcome.new_from_acl_record,
                reason: BalanceHandleUpdateReason::TransferDebit,
            });
            emit_cpi!(BalanceHandleUpdatedEvent {
                version: APP_EVENT_VERSION,
                mint: outcome.mint,
                owner: outcome.to_owner,
                token_account: outcome.to_token_account,
                old_handle: outcome.old_to_handle,
                old_acl_record: outcome.old_to_acl_record,
                new_handle: outcome.new_to_handle,
                new_acl_record: outcome.new_to_acl_record,
                reason: BalanceHandleUpdateReason::TransferCredit,
            });
        }
        Ok(())
    }

    /// Calls an arbitrary receiver hook and verifies its encrypted callback-success result.
    pub fn confidential_call_transfer_receiver<'info>(
        ctx: Context<'info, ConfidentialCallTransferReceiver<'info>>,
        sent_handle: [u8; 32],
        callback_success_handle: [u8; 32],
        receiver_instruction_data: Vec<u8>,
    ) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.caller.key(),
            ctx.accounts.from_account.owner,
            ConfidentialTokenError::OwnerMismatch
        );
        call_transfer_receiver_hook(
            &ctx.accounts.mint,
            &ctx.accounts.from_account,
            &ctx.accounts.to_account,
            &ctx.accounts.compute_signer,
            &ctx.accounts.sent_amount_acl,
            &ctx.accounts.callback_success_acl,
            &ctx.accounts.receiver_program,
            &ctx.accounts.instructions_sysvar.to_account_info(),
            ctx.remaining_accounts,
            sent_handle,
            callback_success_handle,
            receiver_instruction_data,
        )?;
        write_transfer_receiver_hook_call(
            &mut ctx.accounts.hook_record,
            ctx.accounts.mint.key(),
            ctx.accounts.from_account.key(),
            ctx.accounts.to_account.key(),
            sent_handle,
            ctx.accounts.sent_amount_acl.key(),
            callback_success_handle,
            ctx.accounts.callback_success_acl.key(),
            ctx.accounts.receiver_program.key(),
            ctx.accounts.caller.key(),
            ctx.bumps.hook_record,
        );
        Ok(())
    }

    /// Calls a receiver hook after an operator-driven confidential transfer.
    pub fn confidential_call_transfer_receiver_from<'info>(
        ctx: Context<'info, ConfidentialCallTransferReceiverFrom<'info>>,
        sent_handle: [u8; 32],
        callback_success_handle: [u8; 32],
        receiver_instruction_data: Vec<u8>,
    ) -> Result<()> {
        assert_active_operator_record(
            &ctx.accounts.operator_record,
            &ctx.accounts.from_account,
            ctx.accounts.operator.key(),
        )?;
        call_transfer_receiver_hook(
            &ctx.accounts.mint,
            &ctx.accounts.from_account,
            &ctx.accounts.to_account,
            &ctx.accounts.compute_signer,
            &ctx.accounts.sent_amount_acl,
            &ctx.accounts.callback_success_acl,
            &ctx.accounts.receiver_program,
            &ctx.accounts.instructions_sysvar.to_account_info(),
            ctx.remaining_accounts,
            sent_handle,
            callback_success_handle,
            receiver_instruction_data,
        )?;
        write_transfer_receiver_hook_call(
            &mut ctx.accounts.hook_record,
            ctx.accounts.mint.key(),
            ctx.accounts.from_account.key(),
            ctx.accounts.to_account.key(),
            sent_handle,
            ctx.accounts.sent_amount_acl.key(),
            callback_success_handle,
            ctx.accounts.callback_success_acl.key(),
            ctx.accounts.receiver_program.key(),
            ctx.accounts.operator.key(),
            ctx.bumps.hook_record,
        );
        Ok(())
    }

    /// Sets or revokes an operator for this confidential token account.
    pub fn set_operator(
        ctx: Context<SetOperator>,
        operator: Pubkey,
        expiration_slot: u64,
    ) -> Result<()> {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
        assert_confidential_mint_shape(&ctx.accounts.mint)?;
        require_keys_eq!(
            ctx.accounts.token_account.owner,
            ctx.accounts.owner.key(),
            ConfidentialTokenError::OwnerMismatch
        );
        require_keys_eq!(
            ctx.accounts.token_account.mint,
            ctx.accounts.mint.key(),
            ConfidentialTokenError::MintMismatch
        );
        assert_confidential_token_account_shape(
            &ctx.accounts.token_account,
            ctx.accounts.mint.key(),
            ctx.accounts.owner.key(),
        )?;
        let (expected, bump) = operator_record_address(ctx.accounts.token_account.key(), operator);
        require_keys_eq!(
            ctx.accounts.operator_record.key(),
            expected,
            ConfidentialTokenError::OperatorRecordMismatch
        );
        create_operator_record_if_needed(
            &ctx.accounts.owner.to_account_info(),
            &ctx.accounts.operator_record.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
            ctx.accounts.token_account.key(),
            ctx.accounts.owner.key(),
            operator,
            bump,
        )?;
        write_operator_record(
            &ctx.accounts.operator_record.to_account_info(),
            &ConfidentialOperator {
                token_account: ctx.accounts.token_account.key(),
                owner: ctx.accounts.owner.key(),
                operator,
                expiration_slot,
                bump,
            },
        )?;
        emit_cpi!(OperatorSetEvent {
            version: APP_EVENT_VERSION,
            mint: ctx.accounts.mint.key(),
            token_account: ctx.accounts.token_account.key(),
            owner: ctx.accounts.owner.key(),
            operator,
            expiration_slot,
        });
        Ok(())
    }

    /// Closes a revoked or expired operator row and refunds rent to the token owner.
    pub fn close_operator(ctx: Context<CloseOperator>, operator: Pubkey) -> Result<()> {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
        assert_confidential_mint_shape(&ctx.accounts.mint)?;
        require_keys_eq!(
            ctx.accounts.token_account.owner,
            ctx.accounts.operator_record.owner,
            ConfidentialTokenError::OperatorRecordMismatch
        );
        require_keys_eq!(
            ctx.accounts.token_account.mint,
            ctx.accounts.mint.key(),
            ConfidentialTokenError::MintMismatch
        );
        assert_confidential_token_account_shape(
            &ctx.accounts.token_account,
            ctx.accounts.mint.key(),
            ctx.accounts.token_account.owner,
        )?;
        assert_operator_record_shape(
            &ctx.accounts.operator_record,
            ctx.accounts.token_account.key(),
            ctx.accounts.token_account.owner,
            operator,
        )?;
        require_keys_eq!(
            ctx.accounts.refund_recipient.key(),
            ctx.accounts.operator_record.owner,
            ConfidentialTokenError::OwnerMismatch
        );

        let slot = Clock::get()?.slot;
        let operator_active = ctx.accounts.operator_record.expiration_slot != 0
            && ctx.accounts.operator_record.expiration_slot >= slot;
        if operator_active {
            let owner = ctx
                .accounts
                .owner
                .as_ref()
                .ok_or(ConfidentialTokenError::OwnerMismatch)?;
            require_keys_eq!(
                owner.key(),
                ctx.accounts.operator_record.owner,
                ConfidentialTokenError::OwnerMismatch
            );
        } else if let Some(owner) = ctx.accounts.owner.as_ref() {
            require_keys_eq!(
                owner.key(),
                ctx.accounts.operator_record.owner,
                ConfidentialTokenError::OwnerMismatch
            );
        }

        emit_cpi!(OperatorClosedEvent {
            version: APP_EVENT_VERSION,
            mint: ctx.accounts.mint.key(),
            token_account: ctx.accounts.token_account.key(),
            owner: ctx.accounts.operator_record.owner,
            operator,
            closed_while_active: operator_active,
        });
        Ok(())
    }

    /// Transfers an encrypted amount from a holder through an active operator.
    pub fn confidential_transfer_from(
        ctx: Context<ConfidentialTransferFrom>,
        amount_handle: [u8; 32],
    ) -> Result<()> {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
        assert_active_operator_record(
            &ctx.accounts.operator_record,
            &ctx.accounts.from_account,
            ctx.accounts.operator.key(),
        )?;
        let outcome = execute_transfer(
            ctx.accounts.as_transfer_accounts(),
            ctx.bumps.compute_signer,
            amount_handle,
        )?;
        if let Some(outcome) = outcome {
            emit_cpi!(ConfidentialTransferEvent {
                version: APP_EVENT_VERSION,
                mint: outcome.mint,
                from_owner: outcome.from_owner,
                from_token_account: outcome.from_token_account,
                to_owner: outcome.to_owner,
                to_token_account: outcome.to_token_account,
                transferred_handle: outcome.transferred_handle,
                transferred_acl_record: outcome.transferred_acl_record,
            });
            emit_cpi!(BalanceHandleUpdatedEvent {
                version: APP_EVENT_VERSION,
                mint: outcome.mint,
                owner: outcome.from_owner,
                token_account: outcome.from_token_account,
                old_handle: outcome.old_from_handle,
                old_acl_record: outcome.old_from_acl_record,
                new_handle: outcome.new_from_handle,
                new_acl_record: outcome.new_from_acl_record,
                reason: BalanceHandleUpdateReason::TransferDebit,
            });
            emit_cpi!(BalanceHandleUpdatedEvent {
                version: APP_EVENT_VERSION,
                mint: outcome.mint,
                owner: outcome.to_owner,
                token_account: outcome.to_token_account,
                old_handle: outcome.old_to_handle,
                old_acl_record: outcome.old_to_acl_record,
                new_handle: outcome.new_to_handle,
                new_acl_record: outcome.new_to_acl_record,
                reason: BalanceHandleUpdateReason::TransferCredit,
            });
        }
        Ok(())
    }

    /// Prepares receiver callback settlement by computing the encrypted refund.
    pub fn confidential_prepare_transfer_callback(
        ctx: Context<ConfidentialPrepareTransferCallback>,
        sent_handle: [u8; 32],
        callback_success_handle: [u8; 32],
    ) -> Result<()> {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
        let outcome = prepare_transfer_callback_settlement(
            ctx.accounts.as_prepare_callback_accounts(),
            ctx.bumps.compute_signer,
            ctx.bumps.settlement_record,
            sent_handle,
            callback_success_handle,
        )?;
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            owner: outcome.to_owner,
            token_account: outcome.to_token_account,
            old_handle: outcome.old_to_handle,
            old_acl_record: outcome.old_to_acl_record,
            new_handle: outcome.new_to_handle,
            new_acl_record: outcome.new_to_acl_record,
            reason: BalanceHandleUpdateReason::TransferCallbackRefundDebit,
        });
        Ok(())
    }

    /// Finalizes a prepared callback settlement by crediting refund and recording final transfer.
    pub fn confidential_finalize_transfer_callback(
        ctx: Context<ConfidentialFinalizeTransferCallback>,
    ) -> Result<()> {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
        let outcome = finalize_transfer_callback_settlement(
            ctx.accounts.as_finalize_callback_accounts(),
            ctx.bumps.compute_signer,
        )?;
        emit_cpi!(ConfidentialTransferEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            from_owner: outcome.to_owner,
            from_token_account: outcome.to_token_account,
            to_owner: outcome.from_owner,
            to_token_account: outcome.from_token_account,
            transferred_handle: outcome.refund_handle,
            transferred_acl_record: outcome.refund_acl_record,
        });
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            owner: outcome.from_owner,
            token_account: outcome.from_token_account,
            old_handle: outcome.old_from_handle,
            old_acl_record: outcome.old_from_acl_record,
            new_handle: outcome.new_from_handle,
            new_acl_record: outcome.new_from_acl_record,
            reason: BalanceHandleUpdateReason::TransferCallbackRefundCredit,
        });
        Ok(())
    }

    /// Test-only receiver endpoint that returns the supplied callback-success witness.
    pub fn test_receiver_return_callback(
        ctx: Context<TestReceiverReturnCallback>,
        mint: Pubkey,
        from_token_account: Pubkey,
        to_token_account: Pubkey,
        sent_handle: [u8; 32],
        sent_acl_record: Pubkey,
        callback_success_handle: [u8; 32],
        callback_success_acl_record: Pubkey,
    ) -> Result<()> {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
        set_return_data(&transfer_receiver_return_data(
            mint,
            from_token_account,
            to_token_account,
            sent_handle,
            sent_acl_record,
            callback_success_handle,
            callback_success_acl_record,
        ));
        Ok(())
    }

    /// Requests public disclosure for the current confidential balance handle.
    pub fn request_disclose_balance(ctx: Context<RequestDiscloseBalance>) -> Result<()> {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
        assert_confidential_mint_shape(&ctx.accounts.mint)?;
        require_keys_eq!(
            ctx.accounts.token_account.owner,
            ctx.accounts.owner.key(),
            ConfidentialTokenError::OwnerMismatch
        );
        require_keys_eq!(
            ctx.accounts.token_account.mint,
            ctx.accounts.mint.key(),
            ConfidentialTokenError::MintMismatch
        );
        assert_confidential_token_account_shape(
            &ctx.accounts.token_account,
            ctx.accounts.mint.key(),
            ctx.accounts.owner.key(),
        )?;
        assert_current_balance_acl(
            &ctx.accounts.balance_acl_record,
            ctx.accounts.balance_acl_record.key(),
            &ctx.accounts.token_account,
            ctx.accounts.mint.key(),
        )?;

        let handle = ctx.accounts.token_account.balance_handle;
        let acl_record = ctx.accounts.balance_acl_record.key();
        fhe::allow_public_decrypt(fhe::AllowPublicDecrypt {
            authority: &ctx.accounts.owner,
            authority_permission_record: ctx
                .accounts
                .authority_permission_record
                .as_ref()
                .map(|account| account.to_account_info()),
            acl_record: ctx.accounts.balance_acl_record.to_account_info(),
            host_config: &ctx.accounts.host_config,
            deny_subject_record: ctx
                .accounts
                .deny_subject_record
                .as_ref()
                .map(|account| account.to_account_info()),
            event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            handle,
        })?;
        emit_cpi!(BalanceDisclosureRequestedEvent {
            version: APP_EVENT_VERSION,
            mint: ctx.accounts.mint.key(),
            owner: ctx.accounts.owner.key(),
            token_account: ctx.accounts.token_account.key(),
            handle,
            acl_record,
        });
        Ok(())
    }

    /// Requests public disclosure for any token-scoped encrypted amount handle.
    pub fn request_disclose_amount(
        ctx: Context<RequestDiscloseAmount>,
        amount_handle: [u8; 32],
    ) -> Result<()> {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
        assert_confidential_mint_shape(&ctx.accounts.mint)?;
        assert_token_amount_acl(
            &ctx.accounts.amount_acl_record,
            amount_handle,
            ctx.accounts.mint.key(),
            ctx.accounts.mint.compute_signer,
        )?;

        fhe::allow_public_decrypt(fhe::AllowPublicDecrypt {
            authority: &ctx.accounts.requester,
            authority_permission_record: ctx
                .accounts
                .authority_permission_record
                .as_ref()
                .map(|account| account.to_account_info()),
            acl_record: ctx.accounts.amount_acl_record.to_account_info(),
            host_config: &ctx.accounts.host_config,
            deny_subject_record: ctx
                .accounts
                .deny_subject_record
                .as_ref()
                .map(|account| account.to_account_info()),
            event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            handle: amount_handle,
        })?;

        emit_cpi!(AmountDisclosureRequestedEvent {
            version: APP_EVENT_VERSION,
            mint: ctx.accounts.mint.key(),
            requester: ctx.accounts.requester.key(),
            handle: amount_handle,
            acl_record: ctx.accounts.amount_acl_record.key(),
        });
        Ok(())
    }

    /// Emits a KMS-certified cleartext for the current balance handle.
    pub fn disclose_balance(ctx: Context<DiscloseBalance>, cleartext_amount: u64) -> Result<()> {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
        assert_confidential_mint_shape(&ctx.accounts.mint)?;
        require_keys_eq!(
            ctx.accounts.token_account.mint,
            ctx.accounts.mint.key(),
            ConfidentialTokenError::MintMismatch
        );
        assert_confidential_token_account_shape(
            &ctx.accounts.token_account,
            ctx.accounts.mint.key(),
            ctx.accounts.token_account.owner,
        )?;
        assert_current_balance_acl(
            &ctx.accounts.balance_acl_record,
            ctx.accounts.balance_acl_record.key(),
            &ctx.accounts.token_account,
            ctx.accounts.mint.key(),
        )?;
        let handle = ctx.accounts.token_account.balance_handle;
        assert_material_commitment(
            &ctx.accounts.balance_material_commitment,
            ctx.accounts.balance_material_commitment.key(),
            &ctx.accounts.balance_acl_record,
            handle,
        )?;
        assert_public_decrypt_released(&ctx.accounts.balance_acl_record)?;
        assert_disclosure_signature(
            &ctx.accounts.instructions_sysvar.to_account_info(),
            ctx.accounts.mint.kms_verifier_authority,
            ctx.accounts.mint.key(),
            handle,
            cleartext_amount,
        )?;

        emit_cpi!(BalanceDisclosedEvent {
            version: APP_EVENT_VERSION,
            mint: ctx.accounts.mint.key(),
            owner: ctx.accounts.token_account.owner,
            token_account: ctx.accounts.token_account.key(),
            handle,
            cleartext_amount,
        });
        Ok(())
    }

    /// Emits a KMS-certified cleartext for any token-scoped encrypted amount.
    pub fn disclose_amount(
        ctx: Context<DiscloseAmount>,
        amount_handle: [u8; 32],
        cleartext_amount: u64,
    ) -> Result<()> {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
        assert_confidential_mint_shape(&ctx.accounts.mint)?;
        assert_token_amount_acl(
            &ctx.accounts.amount_acl_record,
            amount_handle,
            ctx.accounts.mint.key(),
            ctx.accounts.mint.compute_signer,
        )?;
        assert_material_commitment(
            &ctx.accounts.amount_material_commitment,
            ctx.accounts.amount_material_commitment.key(),
            &ctx.accounts.amount_acl_record,
            amount_handle,
        )?;
        assert_public_decrypt_released(&ctx.accounts.amount_acl_record)?;
        assert_disclosure_signature(
            &ctx.accounts.instructions_sysvar.to_account_info(),
            ctx.accounts.mint.kms_verifier_authority,
            ctx.accounts.mint.key(),
            amount_handle,
            cleartext_amount,
        )?;

        emit_cpi!(AmountDisclosedEvent {
            version: APP_EVENT_VERSION,
            mint: ctx.accounts.mint.key(),
            handle: amount_handle,
            cleartext_amount,
        });
        Ok(())
    }

    /// Redeems a previously burned encrypted amount from the underlying-token vault.
    pub fn redeem_burned_amount(
        ctx: Context<RedeemBurnedAmount>,
        burned_handle: [u8; 32],
        cleartext_amount: u64,
    ) -> Result<()> {
        assert_no_remaining_accounts(ctx.remaining_accounts)?;
        assert_confidential_mint_shape(&ctx.accounts.mint)?;
        let mint_key = ctx.accounts.mint.key();
        let token_account_key = ctx.accounts.token_account.key();
        require_keys_eq!(
            ctx.accounts.mint.underlying_mint,
            ctx.accounts.underlying_mint.key(),
            ConfidentialTokenError::UnderlyingMintMismatch
        );
        assert_canonical_vault_token_account(
            ctx.accounts.vault_usdc.key(),
            ctx.accounts.vault_authority.key(),
            ctx.accounts.underlying_mint.key(),
        )?;
        require_keys_eq!(
            ctx.accounts.token_account.owner,
            ctx.accounts.owner.key(),
            ConfidentialTokenError::OwnerMismatch
        );
        require_keys_eq!(
            ctx.accounts.token_account.mint,
            mint_key,
            ConfidentialTokenError::MintMismatch
        );
        assert_confidential_token_account_shape(
            &ctx.accounts.token_account,
            mint_key,
            ctx.accounts.owner.key(),
        )?;
        assert_burned_amount_acl(
            &ctx.accounts.burned_amount_acl,
            burned_handle,
            mint_key,
            token_account_key,
            ctx.accounts.owner.key(),
            ctx.accounts.mint.compute_signer,
        )?;
        assert_material_commitment(
            &ctx.accounts.burned_material_commitment,
            ctx.accounts.burned_material_commitment.key(),
            &ctx.accounts.burned_amount_acl,
            burned_handle,
        )?;
        assert_public_decrypt_released(&ctx.accounts.burned_amount_acl)?;
        assert_disclosure_signature(
            &ctx.accounts.instructions_sysvar.to_account_info(),
            ctx.accounts.mint.kms_verifier_authority,
            mint_key,
            burned_handle,
            cleartext_amount,
        )?;

        let vault_authority_bump = [ctx.bumps.vault_authority];
        let vault_authority_seeds: &[&[u8]] =
            &[b"vault-authority", mint_key.as_ref(), &vault_authority_bump];
        spl_token::transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.key(),
                TransferChecked {
                    from: ctx.accounts.vault_usdc.to_account_info(),
                    mint: ctx.accounts.underlying_mint.to_account_info(),
                    to: ctx.accounts.destination_usdc.to_account_info(),
                    authority: ctx.accounts.vault_authority.to_account_info(),
                },
                &[vault_authority_seeds],
            ),
            cleartext_amount,
            ctx.accounts.mint.decimals,
        )?;

        let redemption = &mut ctx.accounts.redemption_record;
        redemption.mint = mint_key;
        redemption.owner = ctx.accounts.owner.key();
        redemption.token_account = token_account_key;
        redemption.burned_handle = burned_handle;
        redemption.burned_acl_record = ctx.accounts.burned_amount_acl.key();
        redemption.cleartext_amount = cleartext_amount;
        redemption.bump = ctx.bumps.redemption_record;

        emit_cpi!(BurnRedeemedEvent {
            version: APP_EVENT_VERSION,
            mint: mint_key,
            owner: ctx.accounts.owner.key(),
            token_account: token_account_key,
            burned_handle,
            burned_acl_record: ctx.accounts.burned_amount_acl.key(),
            destination_usdc: ctx.accounts.destination_usdc.key(),
            cleartext_amount,
        });
        Ok(())
    }
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
    pub acl_record: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to create the initial balance handle.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Account<'info, zama_host::HostConfig>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

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
}

/// Accounts for wrapping public USDC into a confidential balance.
#[derive(Accounts)]
#[event_cpi]
pub struct WrapUsdc<'info> {
    /// Token owner and transfer authority.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint.
    #[account(mut)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account whose balance is increased.
    #[account(mut)]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Underlying SPL mint.
    pub underlying_mint: Box<Account<'info, SplMint>>,
    /// Owner's source USDC token account.
    #[account(
        mut,
        constraint = user_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = user_usdc.owner == owner.key() @ ConfidentialTokenError::OwnerMismatch
    )]
    pub user_usdc: Box<Account<'info, TokenAccount>>,
    /// Program vault USDC token account.
    #[account(
        mut,
        constraint = vault_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = vault_usdc.owner == vault_authority.key() @ ConfidentialTokenError::VaultAuthorityMismatch
    )]
    pub vault_usdc: Box<Account<'info, TokenAccount>>,
    /// CHECK: PDA authority for the underlying-token vault.
    #[account(seeds = [b"vault-authority", mint.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: Mint-scoped app authority for total-supply handles.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Current balance ACL record used as the left-hand operand.
    pub current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Current total-supply ACL record used as the left-hand operand.
    pub current_total_supply_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub amount_compute_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub total_supply_output_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// SPL token program.
    pub token_program: Program<'info, Token>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

/// Accounts for confidential balance burn.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialBurn<'info> {
    /// Token owner and burn authority.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint whose encrypted total supply is decreased.
    #[account(mut)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token account whose balance is decreased.
    #[account(mut)]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: Mint-scoped app authority for total-supply handles.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Current balance ACL record used as the left-hand operand.
    pub current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Current total-supply ACL record used as the left-hand operand.
    pub current_total_supply_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Encrypted burn amount ACL record.
    pub amount_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub burn_success_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub debit_candidate_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub burned_amount_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub total_supply_output_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

/// Accounts for confidential balance transfer.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialTransfer<'info> {
    /// Sender and transfer authority.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Sender token account.
    #[account(mut)]
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    // Anchor 1 rejects duplicate mutable Account<T> values unless the account opts in.
    // A self-transfer is a supported no-op, so from_account and to_account may be equal.
    #[account(mut, dup)]
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// Sender current balance ACL record.
    pub from_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Recipient current balance ACL record.
    pub to_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Encrypted amount ACL record.
    pub amount_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub transfer_success_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub debit_candidate_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub from_output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub transferred_amount_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub to_output_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

impl<'info> ConfidentialTransfer<'info> {
    fn as_transfer_accounts(&mut self) -> TransferAccounts<'_, 'info> {
        TransferAccounts {
            payer: &self.owner,
            mint: &self.mint,
            from_account: &mut self.from_account,
            to_account: &mut self.to_account,
            compute_signer: &self.compute_signer,
            from_current_compute_acl: self.from_current_compute_acl.as_ref(),
            to_current_compute_acl: self.to_current_compute_acl.as_ref(),
            amount_compute_acl: &self.amount_compute_acl,
            transfer_success_acl: self.transfer_success_acl.to_account_info(),
            debit_candidate_acl: self.debit_candidate_acl.to_account_info(),
            from_output_acl: self.from_output_acl.to_account_info(),
            transferred_amount_acl: self.transferred_amount_acl.to_account_info(),
            to_output_acl: self.to_output_acl.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            system_program: &self.system_program,
        }
    }
}

/// Accounts for calling a receiver hook after a confidential transfer.
#[derive(Accounts)]
#[instruction(sent_handle: [u8; 32])]
pub struct ConfidentialCallTransferReceiver<'info> {
    /// Original sender owner and rent payer for the hook invocation transaction.
    #[account(mut)]
    pub caller: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Original sender token account.
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Original recipient token account.
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// ACL record for the prior transfer's all-or-zero sent amount.
    pub sent_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// ACL record for the receiver-produced encrypted callback success bit.
    pub callback_success_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: Receiver hook program invoked with the remaining accounts.
    pub receiver_program: UncheckedAccount<'info>,
    /// CHECK: Solana instructions sysvar used to prove same-transaction transfer intent.
    #[account(address = INSTRUCTIONS_SYSVAR_ID)]
    pub instructions_sysvar: UncheckedAccount<'info>,
    /// One-shot marker for this receiver hook invocation.
    #[account(
        init,
        payer = caller,
        space = 8 + TransferReceiverHookCall::SPACE,
        seeds = [b"transfer-hook", mint.key().as_ref(), sent_handle.as_ref()],
        bump
    )]
    pub hook_record: Account<'info, TransferReceiverHookCall>,
    /// System program used to create the one-shot hook marker.
    pub system_program: Program<'info, System>,
}

/// Accounts for calling a receiver hook after an operator-driven confidential transfer.
#[derive(Accounts)]
#[instruction(sent_handle: [u8; 32])]
pub struct ConfidentialCallTransferReceiverFrom<'info> {
    /// Active operator that initiated or is authorized to continue the split transfer-and-call flow.
    #[account(mut)]
    pub operator: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Original sender token account controlled by the operator row.
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Original recipient token account.
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Operator authorization row for `(from_account, operator)`.
    #[account(
        seeds = [b"operator", from_account.key().as_ref(), operator.key().as_ref()],
        bump = operator_record.bump
    )]
    pub operator_record: Account<'info, ConfidentialOperator>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// ACL record for the prior transfer's all-or-zero sent amount.
    pub sent_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// ACL record for the receiver-produced encrypted callback success bit.
    pub callback_success_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: Receiver hook program invoked with the remaining accounts.
    pub receiver_program: UncheckedAccount<'info>,
    /// CHECK: Solana instructions sysvar used to prove same-transaction transfer intent.
    #[account(address = INSTRUCTIONS_SYSVAR_ID)]
    pub instructions_sysvar: UncheckedAccount<'info>,
    /// One-shot marker for this receiver hook invocation.
    #[account(
        init,
        payer = operator,
        space = 8 + TransferReceiverHookCall::SPACE,
        seeds = [b"transfer-hook", mint.key().as_ref(), sent_handle.as_ref()],
        bump
    )]
    pub hook_record: Account<'info, TransferReceiverHookCall>,
    /// System program used to create the one-shot hook marker.
    pub system_program: Program<'info, System>,
}

/// Accounts for setting or revoking an operator.
#[derive(Accounts)]
#[event_cpi]
pub struct SetOperator<'info> {
    /// Token account owner and rent payer.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint.
    pub mint: Account<'info, ConfidentialMint>,
    /// Token account whose operator row is being changed.
    pub token_account: Account<'info, ConfidentialTokenAccount>,
    /// CHECK: Canonical operator PDA created or overwritten by this instruction.
    #[account(mut)]
    pub operator_record: UncheckedAccount<'info>,
    /// System program used for operator PDA creation.
    pub system_program: Program<'info, System>,
}

/// Accounts for closing an operator row.
#[derive(Accounts)]
#[event_cpi]
#[instruction(operator: Pubkey)]
pub struct CloseOperator<'info> {
    /// Optional token owner. Required when closing an active operator row.
    pub owner: Option<Signer<'info>>,
    /// Confidential mint.
    pub mint: Account<'info, ConfidentialMint>,
    /// Token account controlled by the operator row.
    pub token_account: Account<'info, ConfidentialTokenAccount>,
    /// Operator authorization row to close.
    #[account(
        mut,
        seeds = [b"operator", token_account.key().as_ref(), operator.as_ref()],
        bump = operator_record.bump,
        close = refund_recipient
    )]
    pub operator_record: Account<'info, ConfidentialOperator>,
    /// CHECK: Must be the stored token owner and receives the rent refund.
    #[account(mut)]
    pub refund_recipient: UncheckedAccount<'info>,
}

/// Accounts for confidential operator transfer.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialTransferFrom<'info> {
    /// Active operator and rent payer for output ACL records.
    #[account(mut)]
    pub operator: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Sender token account controlled by the operator row.
    #[account(mut)]
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    // Anchor 1 rejects duplicate mutable Account<T> values unless the account opts in.
    // A self-transfer is a supported no-op, so from_account and to_account may be equal.
    #[account(mut, dup)]
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Operator authorization row for `(from_account, operator)`.
    #[account(
        seeds = [b"operator", from_account.key().as_ref(), operator.key().as_ref()],
        bump = operator_record.bump
    )]
    pub operator_record: Account<'info, ConfidentialOperator>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// Sender current balance ACL record.
    pub from_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Recipient current balance ACL record.
    pub to_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Encrypted amount ACL record.
    pub amount_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub transfer_success_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub debit_candidate_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub from_output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub transferred_amount_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub to_output_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

impl<'info> ConfidentialTransferFrom<'info> {
    fn as_transfer_accounts(&mut self) -> TransferAccounts<'_, 'info> {
        TransferAccounts {
            payer: &self.operator,
            mint: &self.mint,
            from_account: &mut self.from_account,
            to_account: &mut self.to_account,
            compute_signer: &self.compute_signer,
            from_current_compute_acl: self.from_current_compute_acl.as_ref(),
            to_current_compute_acl: self.to_current_compute_acl.as_ref(),
            amount_compute_acl: &self.amount_compute_acl,
            transfer_success_acl: self.transfer_success_acl.to_account_info(),
            debit_candidate_acl: self.debit_candidate_acl.to_account_info(),
            from_output_acl: self.from_output_acl.to_account_info(),
            transferred_amount_acl: self.transferred_amount_acl.to_account_info(),
            to_output_acl: self.to_output_acl.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            system_program: &self.system_program,
        }
    }
}

/// Accounts for preparing receiver callback settlement and debiting any refund.
#[derive(Accounts)]
#[event_cpi]
#[instruction(sent_handle: [u8; 32])]
pub struct ConfidentialPrepareTransferCallback<'info> {
    /// Rent payer for callback-settlement output accounts.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: receiver-side authority key that produced the encrypted callback result.
    ///
    /// This authority is already enforced by the callback-success ACL record; it
    /// does not sign settlement so a failed callback can be refunded without
    /// recipient cooperation after the hook.
    pub callback_authority: UncheckedAccount<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Original sender token account.
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Original recipient token account; pays any best-effort refund in this prepare step.
    #[account(mut)]
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// Original recipient current balance ACL record.
    pub to_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// ACL record for the prior transfer's all-or-zero sent amount.
    pub sent_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// ACL record for the encrypted callback success bit.
    pub callback_success_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Verified receiver-hook invocation for this sent amount.
    #[account(
        seeds = [b"transfer-hook", mint.key().as_ref(), sent_handle.as_ref()],
        bump = hook_record.bump
    )]
    pub hook_record: Account<'info, TransferReceiverHookCall>,
    /// Replay marker for this callback settlement.
    #[account(
        init,
        payer = payer,
        space = 8 + TransferCallbackSettlement::SPACE,
        seeds = [b"transfer-callback", mint.key().as_ref(), sent_handle.as_ref()],
        bump
    )]
    pub settlement_record: Account<'info, TransferCallbackSettlement>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub callback_zero_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub requested_refund_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub refund_success_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub refund_debit_candidate_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub to_output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub refund_amount_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL and replay-marker account creation.
    pub system_program: Program<'info, System>,
}

impl<'info> ConfidentialPrepareTransferCallback<'info> {
    fn as_prepare_callback_accounts(&mut self) -> PrepareTransferCallbackAccounts<'_, 'info> {
        PrepareTransferCallbackAccounts {
            payer: &self.payer,
            callback_authority: &self.callback_authority,
            mint: &self.mint,
            from_account: self.from_account.as_ref(),
            to_account: &mut self.to_account,
            compute_signer: &self.compute_signer,
            to_current_compute_acl: self.to_current_compute_acl.as_ref(),
            sent_amount_acl: &self.sent_amount_acl,
            callback_success_acl: &self.callback_success_acl,
            hook_record: &self.hook_record,
            settlement_record: &mut self.settlement_record,
            callback_zero_acl: self.callback_zero_acl.to_account_info(),
            requested_refund_acl: self.requested_refund_acl.to_account_info(),
            refund_success_acl: self.refund_success_acl.to_account_info(),
            refund_debit_candidate_acl: self.refund_debit_candidate_acl.to_account_info(),
            to_output_acl: self.to_output_acl.to_account_info(),
            refund_amount_acl: self.refund_amount_acl.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            system_program: &self.system_program,
        }
    }
}

/// Accounts for finalizing a prepared callback settlement and crediting any refund.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialFinalizeTransferCallback<'info> {
    /// Rent payer for final callback-settlement output accounts.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Original sender token account; receives any best-effort refund in this finalize step.
    #[account(mut)]
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Original recipient token account; must match the prepared settlement.
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// Original sender current balance ACL record.
    pub from_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// ACL record for the prior transfer's all-or-zero sent amount.
    pub sent_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Prepared callback settlement.
    #[account(mut)]
    pub settlement_record: Account<'info, TransferCallbackSettlement>,
    /// ACL record for the prepared refund amount.
    pub refund_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub from_output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub transferred_amount_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

impl<'info> ConfidentialFinalizeTransferCallback<'info> {
    fn as_finalize_callback_accounts(&mut self) -> FinalizeTransferCallbackAccounts<'_, 'info> {
        FinalizeTransferCallbackAccounts {
            payer: &self.payer,
            mint: &self.mint,
            from_account: &mut self.from_account,
            to_account: self.to_account.as_ref(),
            compute_signer: &self.compute_signer,
            from_current_compute_acl: self.from_current_compute_acl.as_ref(),
            sent_amount_acl: &self.sent_amount_acl,
            settlement_record: &mut self.settlement_record,
            refund_amount_acl: &self.refund_amount_acl,
            from_output_acl: self.from_output_acl.to_account_info(),
            transferred_amount_acl: self.transferred_amount_acl.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            system_program: &self.system_program,
        }
    }
}

/// Empty account set for the test receiver hook endpoint.
#[derive(Accounts)]
pub struct TestReceiverReturnCallback {}

/// Accounts for requesting public disclosure of the current balance handle.
#[derive(Accounts)]
#[event_cpi]
pub struct RequestDiscloseBalance<'info> {
    /// Token account owner and disclosure authority.
    pub owner: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account whose current balance is disclosed.
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Current balance ACL record. Updated by ZamaHost CPI.
    #[account(mut)]
    pub balance_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: optional overflow permission witness for the owner authority.
    pub authority_permission_record: Option<UncheckedAccount<'info>>,
    /// CHECK: optional deny-list witness when host deny-lists are enabled.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to update the ACL record.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for pause and deny-list checks.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
}

/// Accounts for requesting public disclosure of a token-scoped encrypted amount.
#[derive(Accounts)]
#[event_cpi]
pub struct RequestDiscloseAmount<'info> {
    /// Requester that must have `ACL_ROLE_PUBLIC_DECRYPT` on the amount ACL.
    pub requester: Signer<'info>,
    /// Confidential mint that scopes the encrypted amount.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token-scoped amount ACL record. Updated by ZamaHost CPI.
    #[account(mut)]
    pub amount_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: optional overflow permission witness for the requester authority.
    pub authority_permission_record: Option<UncheckedAccount<'info>>,
    /// CHECK: optional deny-list witness when host deny-lists are enabled.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to update the ACL record.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for pause and deny-list checks.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
}

/// Accounts for disclosing a KMS-certified current balance cleartext.
#[derive(Accounts)]
#[event_cpi]
pub struct DiscloseBalance<'info> {
    /// Confidential mint carrying the KMS verifier authority.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account whose current balance is disclosed.
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Current balance ACL record for the disclosed handle.
    pub balance_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// Material commitment witness for the disclosed handle.
    pub balance_material_commitment: Box<Account<'info, zama_host::HandleMaterialCommitment>>,
    /// CHECK: Solana instructions sysvar; handler verifies its address and previous Ed25519 ix.
    pub instructions_sysvar: UncheckedAccount<'info>,
}

/// Accounts for disclosing a KMS-certified token-scoped amount cleartext.
#[derive(Accounts)]
#[event_cpi]
pub struct DiscloseAmount<'info> {
    /// Confidential mint carrying the KMS verifier authority.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token-scoped amount ACL record for the disclosed handle.
    pub amount_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// Material commitment witness for the disclosed handle.
    pub amount_material_commitment: Box<Account<'info, zama_host::HandleMaterialCommitment>>,
    /// CHECK: Solana instructions sysvar; handler verifies its address and previous Ed25519 ix.
    pub instructions_sysvar: UncheckedAccount<'info>,
}

/// Accounts for redeeming a KMS-certified burned amount from the SPL vault.
#[derive(Accounts)]
#[instruction(burned_handle: [u8; 32], cleartext_amount: u64)]
#[event_cpi]
pub struct RedeemBurnedAmount<'info> {
    /// Token owner and redemption recipient.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint whose vault backs the redeemed burned amount.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account that produced the burned amount.
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Underlying SPL mint.
    pub underlying_mint: Box<Account<'info, SplMint>>,
    /// Program vault USDC token account.
    #[account(
        mut,
        constraint = vault_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = vault_usdc.owner == vault_authority.key() @ ConfidentialTokenError::VaultAuthorityMismatch
    )]
    pub vault_usdc: Box<Account<'info, TokenAccount>>,
    /// Owner's destination USDC token account.
    #[account(
        mut,
        constraint = destination_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = destination_usdc.owner == owner.key() @ ConfidentialTokenError::OwnerMismatch
    )]
    pub destination_usdc: Box<Account<'info, TokenAccount>>,
    /// CHECK: PDA authority for the underlying-token vault.
    #[account(seeds = [b"vault-authority", mint.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    /// Burned amount ACL record whose handle is redeemed.
    pub burned_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Material commitment witness for the burned handle.
    pub burned_material_commitment: Box<Account<'info, zama_host::HandleMaterialCommitment>>,
    /// Replay marker for this burned handle.
    #[account(
        init,
        payer = owner,
        space = 8 + BurnRedemption::SPACE,
        seeds = [b"burn-redemption", mint.key().as_ref(), burned_handle.as_ref()],
        bump
    )]
    pub redemption_record: Account<'info, BurnRedemption>,
    /// CHECK: Solana instructions sysvar; handler verifies its address and previous Ed25519 ix.
    pub instructions_sysvar: UncheckedAccount<'info>,
    /// SPL token program.
    pub token_program: Program<'info, Token>,
    /// System program used for the replay marker.
    pub system_program: Program<'info, System>,
}

/// Confidential mint state for the token PoC.
#[account]
pub struct ConfidentialMint {
    /// Admin/authority that created the mint.
    pub authority: Pubkey,
    /// ACL domain key, currently equal to the mint pubkey.
    pub acl_domain_key: Pubkey,
    /// Program-controlled compute signer PDA.
    pub compute_signer: Pubkey,
    /// Underlying SPL mint wrapped by this confidential mint.
    pub underlying_mint: Pubkey,
    /// Ed25519 authority accepted for KMS disclosure response certificates.
    pub kms_verifier_authority: Pubkey,
    /// Decimal precision inherited from the underlying mint.
    pub decimals: u8,
    /// Current encrypted total-supply handle.
    pub total_supply_handle: [u8; 32],
    /// Current ZamaHost ACL record for `total_supply_handle`.
    pub total_supply_acl_record: Pubkey,
    /// Next nonce sequence to use for a total-supply ACL record.
    pub next_total_supply_nonce_sequence: u64,
}

impl ConfidentialMint {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 32 + 1 + 32 + 32 + 8;
}

/// Confidential token account state.
#[account]
pub struct ConfidentialTokenAccount {
    /// Token account owner.
    pub owner: Pubkey,
    /// Confidential mint this account belongs to.
    pub mint: Pubkey,
    /// Current confidential balance handle.
    pub balance_handle: [u8; 32],
    /// Current ZamaHost ACL record for `balance_handle`.
    pub balance_acl_record: Pubkey,
    /// Next nonce sequence to use for a balance ACL record.
    pub next_balance_nonce_sequence: u64,
    /// Next nonce sequence to use for owner-scoped random amount ACL records.
    pub next_amount_nonce_sequence: u64,
    /// PDA bump for the token account.
    pub bump: u8,
}

/// Operator authorization for one confidential token account.
#[account]
pub struct ConfidentialOperator {
    /// Token account whose balance may be transferred by the operator.
    pub token_account: Pubkey,
    /// Token account owner that created the authorization.
    pub owner: Pubkey,
    /// Operator signer allowed until `expiration_slot`.
    pub operator: Pubkey,
    /// Last slot in which the operator remains active. Zero revokes the row.
    pub expiration_slot: u64,
    /// PDA bump for `(token_account, operator)`.
    pub bump: u8,
}

impl ConfidentialOperator {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 8 + 1;
}

/// Replay marker for a redeemed burned amount handle.
#[account]
pub struct BurnRedemption {
    /// Confidential mint whose vault paid the redemption.
    pub mint: Pubkey,
    /// Token owner that redeemed the burned amount.
    pub owner: Pubkey,
    /// Token account that produced the burned amount.
    pub token_account: Pubkey,
    /// Burned amount handle proven by KMS.
    pub burned_handle: [u8; 32],
    /// ACL record for `burned_handle`.
    pub burned_acl_record: Pubkey,
    /// KMS-certified cleartext amount released from the vault.
    pub cleartext_amount: u64,
    /// PDA bump for `(mint, burned_handle)`.
    pub bump: u8,
}

impl BurnRedemption {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 32 + 8 + 1;
}

/// One-shot marker for a receiver hook call.
#[account]
pub struct TransferReceiverHookCall {
    /// Confidential mint whose transfer reached the receiver hook.
    pub mint: Pubkey,
    /// Original sender token account.
    pub from_token_account: Pubkey,
    /// Original recipient token account.
    pub to_token_account: Pubkey,
    /// Prior transfer's encrypted all-or-zero sent handle.
    pub sent_handle: [u8; 32],
    /// ACL record for `sent_handle`.
    pub sent_acl_record: Pubkey,
    /// Encrypted receiver callback success bit.
    pub callback_success_handle: [u8; 32],
    /// ACL record for `callback_success_handle`.
    pub callback_success_acl_record: Pubkey,
    /// Receiver hook program that returned the callback witness.
    pub receiver_program: Pubkey,
    /// Sender or active operator that invoked the hook.
    pub caller: Pubkey,
    /// PDA bump for `(mint, sent_handle)`.
    pub bump: u8,
}

impl TransferReceiverHookCall {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = (32 * 9) + 1;
}

/// Replay marker for a transfer callback settlement.
#[account]
pub struct TransferCallbackSettlement {
    /// Confidential mint whose transfer was settled.
    pub mint: Pubkey,
    /// Original sender owner.
    pub from_owner: Pubkey,
    /// Original sender token account.
    pub from_token_account: Pubkey,
    /// Original recipient owner.
    pub to_owner: Pubkey,
    /// Original recipient token account.
    pub to_token_account: Pubkey,
    /// Prior transfer's encrypted all-or-zero sent handle.
    pub sent_handle: [u8; 32],
    /// ACL record for `sent_handle`.
    pub sent_acl_record: Pubkey,
    /// Encrypted receiver callback success bit.
    pub callback_success_handle: [u8; 32],
    /// ACL record for `callback_success_handle`.
    pub callback_success_acl_record: Pubkey,
    /// Encrypted refund requested by the callback result.
    pub requested_refund_handle: [u8; 32],
    /// ACL record for `requested_refund_handle`.
    pub requested_refund_acl_record: Pubkey,
    /// Encrypted amount actually refunded.
    pub refund_handle: [u8; 32],
    /// ACL record for `refund_handle`.
    pub refund_acl_record: Pubkey,
    /// Recipient balance handle after the prepared refund debit.
    pub to_balance_handle: [u8; 32],
    /// ACL record for `to_balance_handle`.
    pub to_balance_acl_record: Pubkey,
    /// Sender balance handle after the finalized refund credit.
    pub from_balance_handle: [u8; 32],
    /// ACL record for `from_balance_handle`.
    pub from_balance_acl_record: Pubkey,
    /// Encrypted amount that remains transferred after refund.
    pub transferred_handle: [u8; 32],
    /// ACL record for `transferred_handle`.
    pub transferred_acl_record: Pubkey,
    /// Settlement lifecycle status.
    pub status: u8,
    /// PDA bump for `(mint, sent_handle)`.
    pub bump: u8,
}

impl TransferCallbackSettlement {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = (32 * 19) + 2;
}

/// App-local balance history event.
///
/// This event is for frontend/app indexers. The generic coprocessor listener
/// consumes ZamaHost protocol events instead.
#[event]
pub struct BalanceHandleUpdatedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account owner.
    pub owner: Pubkey,
    /// Confidential token account.
    pub token_account: Pubkey,
    /// Previous balance handle.
    pub old_handle: [u8; 32],
    /// Previous ZamaHost ACL record.
    pub old_acl_record: Pubkey,
    /// New balance handle.
    pub new_handle: [u8; 32],
    /// New ZamaHost ACL record.
    pub new_acl_record: Pubkey,
    /// Reason this balance pointer changed.
    pub reason: BalanceHandleUpdateReason,
}

/// Reason code for [`BalanceHandleUpdatedEvent`].
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BalanceHandleUpdateReason {
    /// Initial account creation.
    Initialize,
    /// Public USDC was wrapped into this account.
    Wrap,
    /// Transfer debit from this account.
    TransferDebit,
    /// Transfer credit to this account.
    TransferCredit,
    /// Confidential burn debit from this account.
    BurnDebit,
    /// Receiver callback settlement debited a best-effort refund.
    TransferCallbackRefundDebit,
    /// Receiver callback settlement credited a best-effort refund.
    TransferCallbackRefundCredit,
}

/// App-local total-supply history event.
///
/// This mirrors ERC7984's encrypted `_totalSupply` pointer at the Solana mint
/// level. The generic coprocessor listener consumes ZamaHost protocol events;
/// this event is for token-aware indexers.
#[event]
pub struct TotalSupplyHandleUpdatedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Previous total-supply handle.
    pub old_handle: [u8; 32],
    /// Previous ZamaHost ACL record.
    pub old_acl_record: Pubkey,
    /// New total-supply handle.
    pub new_handle: [u8; 32],
    /// New ZamaHost ACL record.
    pub new_acl_record: Pubkey,
    /// Reason this total-supply pointer changed.
    pub reason: TotalSupplyUpdateReason,
}

/// Reason code for [`TotalSupplyHandleUpdatedEvent`].
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TotalSupplyUpdateReason {
    /// Initial mint creation.
    Initialize,
    /// Public USDC was wrapped into confidential supply.
    Wrap,
    /// Confidential supply was burned.
    Burn,
}

/// Token-scoped amount purpose used for amount-handle birth.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfidentialAmountKind {
    /// Amount intended for confidential transfers.
    Transfer,
    /// Amount intended for confidential burns.
    Burn,
}

impl ConfidentialAmountKind {
    fn encrypted_value_label(self) -> [u8; 32] {
        match self {
            ConfidentialAmountKind::Transfer => transfer_amount_label(),
            ConfidentialAmountKind::Burn => burn_amount_label(),
        }
    }
}

/// Emitted when the token program creates a token-scoped random amount.
#[event]
pub struct RandomAmountCreatedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account owner.
    pub owner: Pubkey,
    /// Confidential token account.
    pub token_account: Pubkey,
    /// Whether this amount is intended for transfer or burn.
    pub amount_kind: ConfidentialAmountKind,
    /// True when the random amount was bounded.
    pub bounded: bool,
    /// Bound supplied to ZamaHost for bounded random amounts, or zero bytes for unbounded amounts.
    pub upper_bound: [u8; 32],
    /// Newly created amount handle.
    pub handle: [u8; 32],
    /// ZamaHost ACL record initialized for the amount.
    pub acl_record: Pubkey,
    /// Nonce sequence used for the amount ACL record.
    pub nonce_sequence: u64,
}

/// Emitted when a holder changes a confidential-token operator row.
#[event]
pub struct OperatorSetEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account controlled by the operator row.
    pub token_account: Pubkey,
    /// Token account owner.
    pub owner: Pubkey,
    /// Operator signer.
    pub operator: Pubkey,
    /// Last active slot, or zero when revoked.
    pub expiration_slot: u64,
}

/// Emitted when an operator row is closed and its rent is refunded.
#[event]
pub struct OperatorClosedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account controlled by the closed row.
    pub token_account: Pubkey,
    /// Token account owner receiving the rent refund.
    pub owner: Pubkey,
    /// Closed operator signer.
    pub operator: Pubkey,
    /// True when the owner explicitly closed an active row.
    pub closed_while_active: bool,
}

/// Emitted when the owner requests public disclosure of the current balance.
#[event]
pub struct BalanceDisclosureRequestedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account owner.
    pub owner: Pubkey,
    /// Confidential token account.
    pub token_account: Pubkey,
    /// Publicly decryptable balance handle.
    pub handle: [u8; 32],
    /// ZamaHost ACL record updated by the request.
    pub acl_record: Pubkey,
}

/// Emitted when a requester asks to publicly disclose a token-scoped amount.
#[event]
pub struct AmountDisclosureRequestedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Requester authorized on the amount ACL.
    pub requester: Pubkey,
    /// Publicly decryptable amount handle.
    pub handle: [u8; 32],
    /// ZamaHost ACL record updated by the request.
    pub acl_record: Pubkey,
}

/// Emitted when a KMS certificate discloses the current balance cleartext.
#[event]
pub struct BalanceDisclosedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account owner.
    pub owner: Pubkey,
    /// Confidential token account.
    pub token_account: Pubkey,
    /// Disclosed balance handle.
    pub handle: [u8; 32],
    /// KMS-certified cleartext amount.
    pub cleartext_amount: u64,
}

/// Emitted when a KMS certificate discloses a token-scoped amount cleartext.
#[event]
pub struct AmountDisclosedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Disclosed encrypted amount handle.
    pub handle: [u8; 32],
    /// KMS-certified cleartext amount.
    pub cleartext_amount: u64,
}

/// Emitted when a KMS-certified burned amount is redeemed from the vault.
#[event]
pub struct BurnRedeemedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account owner.
    pub owner: Pubkey,
    /// Confidential token account that produced the burned amount.
    pub token_account: Pubkey,
    /// Burned amount handle proven by KMS.
    pub burned_handle: [u8; 32],
    /// ACL record for `burned_handle`.
    pub burned_acl_record: Pubkey,
    /// Underlying token destination account.
    pub destination_usdc: Pubkey,
    /// KMS-certified cleartext amount released from the vault.
    pub cleartext_amount: u64,
}

/// Emitted when a confidential burn computes the all-or-zero burned amount.
#[event]
pub struct ConfidentialBurnEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account owner.
    pub owner: Pubkey,
    /// Token account whose balance was debited.
    pub token_account: Pubkey,
    /// Encrypted amount actually burned.
    pub burned_handle: [u8; 32],
    /// ZamaHost ACL record for `burned_handle`.
    pub burned_acl_record: Pubkey,
}

/// Emitted when a confidential transfer computes the all-or-zero moved amount.
#[event]
pub struct ConfidentialTransferEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Sender token account owner.
    pub from_owner: Pubkey,
    /// Sender confidential token account.
    pub from_token_account: Pubkey,
    /// Recipient token account owner.
    pub to_owner: Pubkey,
    /// Recipient confidential token account.
    pub to_token_account: Pubkey,
    /// Encrypted amount actually transferred.
    pub transferred_handle: [u8; 32],
    /// ZamaHost ACL record for `transferred_handle`.
    pub transferred_acl_record: Pubkey,
}

impl ConfidentialTokenAccount {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 8 + 8 + 1;
}

/// Errors returned by the confidential token PoC.
#[error_code]
pub enum ConfidentialTokenError {
    /// Token owner did not match the required signer.
    #[msg("Token owner does not match signer")]
    OwnerMismatch,
    /// Token account mint did not match the supplied mint.
    #[msg("Token account mint does not match")]
    MintMismatch,
    /// Confidential mint account shape or self-domain metadata is invalid.
    #[msg("Confidential mint account is invalid")]
    MintAccountMismatch,
    /// Confidential mint profile fields are unusable.
    #[msg("Confidential mint config is invalid")]
    InvalidMintConfig,
    /// The instruction included undeclared trailing account metas.
    #[msg("instruction has unexpected remaining accounts")]
    UnexpectedRemainingAccounts,
    /// Token account was not the canonical owner/mint PDA.
    #[msg("Confidential token account is not canonical")]
    TokenAccountMismatch,
    /// Balance nonce sequence overflowed.
    #[msg("ACL nonce overflow")]
    AclNonceOverflow,
    /// Token account initialization cannot mint unbacked confidential supply.
    #[msg("nonzero initial confidential balances are unsupported")]
    NonZeroInitialBalanceUnsupported,
    /// Underlying SPL mint did not match the confidential mint metadata.
    #[msg("Underlying mint does not match confidential mint")]
    UnderlyingMintMismatch,
    /// Vault token account owner did not match the vault authority PDA.
    #[msg("Vault token account authority does not match vault authority PDA")]
    VaultAuthorityMismatch,
    /// Vault token account was not the mint's canonical associated token account.
    #[msg("Vault token account is not the canonical mint vault")]
    VaultAccountMismatch,
    /// Confidential mint ACL domain key was not the expected mint key.
    #[msg("Confidential mint ACL domain key is invalid")]
    AclDomainKeyMismatch,
    /// Compute signer PDA did not match the confidential mint metadata.
    #[msg("Compute signer does not match confidential mint")]
    ComputeSignerMismatch,
    /// Current ACL record account did not match token account state.
    #[msg("current ACL record does not match token account state")]
    CurrentAclRecordMismatch,
    /// Operator authorization row did not match the requested transfer.
    #[msg("operator record does not match")]
    OperatorRecordMismatch,
    /// Operator authorization is missing or expired.
    #[msg("operator authorization is expired")]
    OperatorExpired,
    /// Transfer amount handle does not carry the expected confidential balance type.
    #[msg("transfer amount handle type is invalid")]
    AmountHandleTypeMismatch,
    /// Transfer amount ACL record is not scoped to the sender token account.
    #[msg("transfer amount ACL record is invalid")]
    AmountAclMismatch,
    /// Total-supply authority PDA did not match the mint.
    #[msg("total supply authority does not match mint")]
    TotalSupplyAuthorityMismatch,
    /// Disclosure certificate was not signed by the mint KMS verifier authority.
    #[msg("disclosure proof signature is missing or invalid")]
    DisclosureProofSignatureMissing,
    /// Material commitment witness did not match the disclosed handle.
    #[msg("material commitment witness does not match")]
    MaterialCommitmentMismatch,
    /// The disclosed handle has not been released for public decrypt.
    #[msg("handle is not released for public decrypt")]
    PublicDecryptNotReleased,
    /// Transfer callback settlement accounts do not match the prior transfer.
    #[msg("transfer callback settlement accounts are invalid")]
    CallbackSettlementMismatch,
    /// Receiver hook did not return the expected callback witness.
    #[msg("receiver hook return data is invalid")]
    ReceiverHookMismatch,
}

fn assert_no_remaining_accounts(remaining_accounts: &[AccountInfo]) -> Result<()> {
    require!(
        remaining_accounts.is_empty(),
        ConfidentialTokenError::UnexpectedRemainingAccounts
    );
    Ok(())
}

struct TransferAccounts<'a, 'info> {
    payer: &'a Signer<'info>,
    mint: &'a Account<'info, ConfidentialMint>,
    from_account: &'a mut Box<Account<'info, ConfidentialTokenAccount>>,
    to_account: &'a mut Box<Account<'info, ConfidentialTokenAccount>>,
    compute_signer: &'a UncheckedAccount<'info>,
    from_current_compute_acl: &'a Account<'info, zama_host::AclRecord>,
    to_current_compute_acl: &'a Account<'info, zama_host::AclRecord>,
    amount_compute_acl: &'a Account<'info, zama_host::AclRecord>,
    transfer_success_acl: AccountInfo<'info>,
    debit_candidate_acl: AccountInfo<'info>,
    from_output_acl: AccountInfo<'info>,
    transferred_amount_acl: AccountInfo<'info>,
    to_output_acl: AccountInfo<'info>,
    zama_event_authority: &'a UncheckedAccount<'info>,
    zama_program: &'a Program<'info, ZamaHost>,
    host_config: &'a Account<'info, zama_host::HostConfig>,
    system_program: &'a Program<'info, System>,
}

struct TransferOutcome {
    mint: Pubkey,
    from_owner: Pubkey,
    from_token_account: Pubkey,
    old_from_handle: [u8; 32],
    old_from_acl_record: Pubkey,
    new_from_handle: [u8; 32],
    new_from_acl_record: Pubkey,
    transferred_handle: [u8; 32],
    transferred_acl_record: Pubkey,
    to_owner: Pubkey,
    to_token_account: Pubkey,
    old_to_handle: [u8; 32],
    old_to_acl_record: Pubkey,
    new_to_handle: [u8; 32],
    new_to_acl_record: Pubkey,
}

struct PrepareTransferCallbackAccounts<'a, 'info> {
    payer: &'a Signer<'info>,
    callback_authority: &'a UncheckedAccount<'info>,
    mint: &'a Account<'info, ConfidentialMint>,
    from_account: &'a Account<'info, ConfidentialTokenAccount>,
    to_account: &'a mut Box<Account<'info, ConfidentialTokenAccount>>,
    compute_signer: &'a UncheckedAccount<'info>,
    to_current_compute_acl: &'a Account<'info, zama_host::AclRecord>,
    sent_amount_acl: &'a Account<'info, zama_host::AclRecord>,
    callback_success_acl: &'a Account<'info, zama_host::AclRecord>,
    hook_record: &'a Account<'info, TransferReceiverHookCall>,
    settlement_record: &'a mut Account<'info, TransferCallbackSettlement>,
    callback_zero_acl: AccountInfo<'info>,
    requested_refund_acl: AccountInfo<'info>,
    refund_success_acl: AccountInfo<'info>,
    refund_debit_candidate_acl: AccountInfo<'info>,
    to_output_acl: AccountInfo<'info>,
    refund_amount_acl: AccountInfo<'info>,
    zama_event_authority: &'a UncheckedAccount<'info>,
    zama_program: &'a Program<'info, ZamaHost>,
    host_config: &'a Account<'info, zama_host::HostConfig>,
    system_program: &'a Program<'info, System>,
}

struct PrepareTransferCallbackOutcome {
    mint: Pubkey,
    to_owner: Pubkey,
    to_token_account: Pubkey,
    old_to_handle: [u8; 32],
    old_to_acl_record: Pubkey,
    new_to_handle: [u8; 32],
    new_to_acl_record: Pubkey,
}

struct FinalizeTransferCallbackAccounts<'a, 'info> {
    payer: &'a Signer<'info>,
    mint: &'a Account<'info, ConfidentialMint>,
    from_account: &'a mut Box<Account<'info, ConfidentialTokenAccount>>,
    to_account: &'a Account<'info, ConfidentialTokenAccount>,
    compute_signer: &'a UncheckedAccount<'info>,
    from_current_compute_acl: &'a Account<'info, zama_host::AclRecord>,
    sent_amount_acl: &'a Account<'info, zama_host::AclRecord>,
    settlement_record: &'a mut Account<'info, TransferCallbackSettlement>,
    refund_amount_acl: &'a Account<'info, zama_host::AclRecord>,
    from_output_acl: AccountInfo<'info>,
    transferred_amount_acl: AccountInfo<'info>,
    zama_event_authority: &'a UncheckedAccount<'info>,
    zama_program: &'a Program<'info, ZamaHost>,
    host_config: &'a Account<'info, zama_host::HostConfig>,
    system_program: &'a Program<'info, System>,
}

struct FinalizeTransferCallbackOutcome {
    mint: Pubkey,
    from_owner: Pubkey,
    from_token_account: Pubkey,
    old_from_handle: [u8; 32],
    old_from_acl_record: Pubkey,
    new_from_handle: [u8; 32],
    new_from_acl_record: Pubkey,
    to_owner: Pubkey,
    to_token_account: Pubkey,
    refund_handle: [u8; 32],
    refund_acl_record: Pubkey,
}

fn execute_transfer<'info>(
    accounts: TransferAccounts<'_, 'info>,
    compute_signer_bump: u8,
    amount_handle: [u8; 32],
) -> Result<Option<TransferOutcome>> {
    assert_confidential_mint_shape(accounts.mint)?;
    let mint_key = accounts.mint.key();
    let compute_signer = accounts.mint.compute_signer;
    let from = accounts.from_account.as_ref();
    let to = accounts.to_account.as_ref();
    let from_nonce_sequence = from.next_balance_nonce_sequence;
    let to_nonce_sequence = to.next_balance_nonce_sequence;
    let old_from_handle = from.balance_handle;
    let old_from_acl_record = from.balance_acl_record;
    let old_to_handle = to.balance_handle;
    let old_to_acl_record = to.balance_acl_record;

    assert_transfer_amount_acl(
        accounts.amount_compute_acl,
        amount_handle,
        mint_key,
        accounts.payer.key(),
        compute_signer,
    )?;
    require_keys_eq!(from.mint, mint_key, ConfidentialTokenError::MintMismatch);
    require_keys_eq!(to.mint, mint_key, ConfidentialTokenError::MintMismatch);
    assert_confidential_token_account_shape(from, mint_key, from.owner)?;
    assert_confidential_token_account_shape(to, mint_key, to.owner)?;
    require_keys_eq!(
        accounts.compute_signer.key(),
        compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    assert_current_balance_acl(
        accounts.from_current_compute_acl,
        accounts.from_current_compute_acl.key(),
        from,
        mint_key,
    )?;
    assert_current_balance_acl(
        accounts.to_current_compute_acl,
        accounts.to_current_compute_acl.key(),
        to,
        mint_key,
    )?;
    if from.key() == to.key() {
        assert_self_transfer_output_accounts(&accounts, mint_key, from.key(), from_nonce_sequence)?;
        return Ok(None);
    }

    let transfer_success_handle = ge_balance(
        accounts.payer,
        accounts.zama_event_authority,
        accounts.zama_program,
        accounts.host_config,
        accounts.compute_signer,
        from,
        accounts.from_current_compute_acl.to_account_info(),
        from.balance_handle,
        accounts.amount_compute_acl.to_account_info(),
        amount_handle,
        accounts.transfer_success_acl.clone(),
        mint_key,
        compute_signer_bump,
        accounts.system_program,
        from_nonce_sequence,
        transfer_success_label(),
    )?;
    let debit_candidate_handle = compute_balance_scratch(
        fhe::sub,
        BalanceScratch {
            payer: accounts.payer,
            zama_event_authority: accounts.zama_event_authority,
            zama_program: accounts.zama_program,
            host_config: accounts.host_config,
            compute_signer: accounts.compute_signer,
            token_account: from,
            lhs_acl_record: accounts.from_current_compute_acl.to_account_info(),
            lhs: from.balance_handle,
            rhs_acl_record: accounts.amount_compute_acl.to_account_info(),
            rhs: amount_handle,
            output_acl_record: accounts.debit_candidate_acl.clone(),
            mint: mint_key,
            compute_signer_bump,
            system_program: accounts.system_program,
            output_nonce_sequence: from_nonce_sequence,
            output_encrypted_value_label: debit_candidate_label(),
            output_subjects: compute_acl_subject(accounts.compute_signer.key()),
        },
    )?;
    let new_from_handle = select_balance(
        accounts.payer,
        accounts.zama_event_authority,
        accounts.zama_program,
        accounts.host_config,
        accounts.compute_signer,
        from,
        accounts.transfer_success_acl.clone(),
        transfer_success_handle,
        accounts.debit_candidate_acl.clone(),
        debit_candidate_handle,
        accounts.from_current_compute_acl.to_account_info(),
        from.balance_handle,
        accounts.from_output_acl.clone(),
        mint_key,
        compute_signer_bump,
        accounts.system_program,
        from_nonce_sequence,
    )?;
    let transferred_handle = compute_balance_scratch(
        fhe::sub,
        BalanceScratch {
            payer: accounts.payer,
            zama_event_authority: accounts.zama_event_authority,
            zama_program: accounts.zama_program,
            host_config: accounts.host_config,
            compute_signer: accounts.compute_signer,
            token_account: from,
            lhs_acl_record: accounts.from_current_compute_acl.to_account_info(),
            lhs: from.balance_handle,
            rhs_acl_record: accounts.from_output_acl.clone(),
            rhs: new_from_handle,
            output_acl_record: accounts.transferred_amount_acl.clone(),
            mint: mint_key,
            compute_signer_bump,
            system_program: accounts.system_program,
            output_nonce_sequence: from_nonce_sequence,
            output_encrypted_value_label: transferred_amount_label(),
            output_subjects: transferred_amount_acl_subjects(
                from.owner,
                to.owner,
                accounts.compute_signer.key(),
            ),
        },
    )?;
    let new_to_handle = add_balance(
        accounts.payer,
        accounts.zama_event_authority,
        accounts.zama_program,
        accounts.host_config,
        accounts.compute_signer,
        to,
        accounts.to_current_compute_acl.to_account_info(),
        to.balance_handle,
        accounts.transferred_amount_acl.clone(),
        transferred_handle,
        accounts.to_output_acl.clone(),
        mint_key,
        compute_signer_bump,
        accounts.system_program,
        to_nonce_sequence,
    )?;

    let from = accounts.from_account.as_mut();
    from.balance_handle = new_from_handle;
    from.balance_acl_record = accounts.from_output_acl.key();
    from.next_balance_nonce_sequence = from_nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
    let from_owner = from.owner;
    let from_token_account = from.key();
    let new_from_acl_record = accounts.from_output_acl.key();

    let to = accounts.to_account.as_mut();
    to.balance_handle = new_to_handle;
    to.balance_acl_record = accounts.to_output_acl.key();
    to.next_balance_nonce_sequence = to_nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
    Ok(Some(TransferOutcome {
        mint: mint_key,
        from_owner,
        from_token_account,
        old_from_handle,
        old_from_acl_record,
        new_from_handle,
        new_from_acl_record,
        transferred_handle,
        transferred_acl_record: accounts.transferred_amount_acl.key(),
        to_owner: to.owner,
        to_token_account: to.key(),
        old_to_handle,
        old_to_acl_record,
        new_to_handle,
        new_to_acl_record: accounts.to_output_acl.key(),
    }))
}

fn assert_self_transfer_output_accounts(
    accounts: &TransferAccounts<'_, '_>,
    mint: Pubkey,
    token_account: Pubkey,
    nonce_sequence: u64,
) -> Result<()> {
    assert_unused_acl_target(
        &accounts.transfer_success_acl,
        acl_record_address_for(
            mint,
            token_account,
            transfer_success_label(),
            nonce_sequence,
        ),
    )?;
    assert_unused_acl_target(
        &accounts.debit_candidate_acl,
        acl_record_address_for(mint, token_account, debit_candidate_label(), nonce_sequence),
    )?;
    let balance_output =
        acl_record_address_for(mint, token_account, balance_label(), nonce_sequence);
    assert_unused_acl_target(&accounts.from_output_acl, balance_output)?;
    assert_unused_acl_target(&accounts.to_output_acl, balance_output)?;
    assert_unused_acl_target(
        &accounts.transferred_amount_acl,
        acl_record_address_for(
            mint,
            token_account,
            transferred_amount_label(),
            nonce_sequence,
        ),
    )?;
    Ok(())
}

fn assert_unused_acl_target(account: &AccountInfo, expected_key: Pubkey) -> Result<()> {
    require_keys_eq!(
        account.key(),
        expected_key,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        *account.owner,
        System::id(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        account.data_is_empty(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        !account.executable,
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

fn acl_record_address_for(
    mint: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    nonce_sequence: u64,
) -> Pubkey {
    zama_host::acl_record_address(
        nonce_key(mint, app_account, encrypted_value_label),
        nonce_sequence,
    )
    .0
}

fn prepare_transfer_callback_settlement<'info>(
    mut accounts: PrepareTransferCallbackAccounts<'_, 'info>,
    compute_signer_bump: u8,
    settlement_bump: u8,
    sent_handle: [u8; 32],
    callback_success_handle: [u8; 32],
) -> Result<PrepareTransferCallbackOutcome> {
    assert_confidential_mint_shape(accounts.mint)?;
    let mint_key = accounts.mint.key();
    let compute_signer = accounts.mint.compute_signer;
    let from = accounts.from_account;
    let to = accounts.to_account.as_ref();
    let from_token_account = from.key();
    let to_token_account = to.key();
    let from_owner = from.owner;
    let to_owner = to.owner;
    let to_nonce_sequence = to.next_balance_nonce_sequence;
    let old_to_handle = to.balance_handle;
    let old_to_acl_record = to.balance_acl_record;
    let amount_subjects = transferred_amount_acl_subjects(from_owner, to_owner, compute_signer);

    require!(
        from_token_account != to_token_account,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(from.mint, mint_key, ConfidentialTokenError::MintMismatch);
    require_keys_eq!(to.mint, mint_key, ConfidentialTokenError::MintMismatch);
    assert_confidential_token_account_shape(from, mint_key, from_owner)?;
    assert_confidential_token_account_shape(to, mint_key, to_owner)?;
    require_keys_eq!(
        accounts.callback_authority.key(),
        to_owner,
        ConfidentialTokenError::OwnerMismatch
    );
    require_keys_eq!(
        accounts.compute_signer.key(),
        compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    assert_current_balance_acl(
        accounts.to_current_compute_acl,
        old_to_acl_record,
        to,
        mint_key,
    )?;
    assert_transferred_amount_acl(
        accounts.sent_amount_acl,
        sent_handle,
        mint_key,
        from_token_account,
        from_owner,
        to_owner,
        compute_signer,
    )?;
    assert_callback_success_acl(
        accounts.callback_success_acl,
        callback_success_handle,
        mint_key,
        to_owner,
        compute_signer,
    )?;
    assert_transfer_receiver_hook_call_shape(
        accounts.hook_record,
        mint_key,
        from_token_account,
        to_token_account,
        sent_handle,
        accounts.sent_amount_acl.key(),
        callback_success_handle,
        accounts.callback_success_acl.key(),
    )?;

    let zero_handle = fhe::trivial_encrypt_u64(fhe::TrivialEncryptU64 {
        payer: accounts.payer,
        event_authority: accounts.zama_event_authority,
        zama_program: accounts.zama_program,
        host_config: accounts.host_config,
        compute_signer: accounts.compute_signer,
        app_account_authority: to,
        output_acl_record: accounts.callback_zero_acl.clone(),
        acl_domain_key: mint_key,
        compute_signer_bump,
        system_program: accounts.system_program,
        output_nonce_key: nonce_key(mint_key, to_token_account, callback_zero_label()),
        output_nonce_sequence: to_nonce_sequence,
        output_encrypted_value_label: callback_zero_label(),
        plaintext: 0,
        fhe_type: BALANCE_FHE_TYPE,
        output_subjects: compute_acl_subject(compute_signer),
        output_public_decrypt: false,
    })?;
    let requested_refund_handle = select_amount_scratch(TernaryScratch {
        payer: accounts.payer,
        zama_event_authority: accounts.zama_event_authority,
        zama_program: accounts.zama_program,
        host_config: accounts.host_config,
        compute_signer: accounts.compute_signer,
        token_account: to,
        control_acl_record: accounts.callback_success_acl.to_account_info(),
        control: callback_success_handle,
        if_true_acl_record: accounts.callback_zero_acl.clone(),
        if_true: zero_handle,
        if_false_acl_record: accounts.sent_amount_acl.to_account_info(),
        if_false: sent_handle,
        output_acl_record: accounts.requested_refund_acl.clone(),
        mint: mint_key,
        compute_signer_bump,
        system_program: accounts.system_program,
        output_nonce_sequence: to_nonce_sequence,
        output_encrypted_value_label: callback_refund_request_label(),
        output_subjects: amount_subjects.clone(),
    })?;
    let refund_success_handle = ge_balance(
        accounts.payer,
        accounts.zama_event_authority,
        accounts.zama_program,
        accounts.host_config,
        accounts.compute_signer,
        to,
        accounts.to_current_compute_acl.to_account_info(),
        old_to_handle,
        accounts.requested_refund_acl.clone(),
        requested_refund_handle,
        accounts.refund_success_acl.clone(),
        mint_key,
        compute_signer_bump,
        accounts.system_program,
        to_nonce_sequence,
        callback_refund_success_label(),
    )?;
    let refund_debit_candidate_handle = compute_balance_scratch(
        fhe::sub,
        BalanceScratch {
            payer: accounts.payer,
            zama_event_authority: accounts.zama_event_authority,
            zama_program: accounts.zama_program,
            host_config: accounts.host_config,
            compute_signer: accounts.compute_signer,
            token_account: to,
            lhs_acl_record: accounts.to_current_compute_acl.to_account_info(),
            lhs: old_to_handle,
            rhs_acl_record: accounts.requested_refund_acl.clone(),
            rhs: requested_refund_handle,
            output_acl_record: accounts.refund_debit_candidate_acl.clone(),
            mint: mint_key,
            compute_signer_bump,
            system_program: accounts.system_program,
            output_nonce_sequence: to_nonce_sequence,
            output_encrypted_value_label: callback_refund_debit_candidate_label(),
            output_subjects: compute_acl_subject(compute_signer),
        },
    )?;
    let new_to_handle = select_balance(
        accounts.payer,
        accounts.zama_event_authority,
        accounts.zama_program,
        accounts.host_config,
        accounts.compute_signer,
        to,
        accounts.refund_success_acl.clone(),
        refund_success_handle,
        accounts.refund_debit_candidate_acl.clone(),
        refund_debit_candidate_handle,
        accounts.to_current_compute_acl.to_account_info(),
        old_to_handle,
        accounts.to_output_acl.clone(),
        mint_key,
        compute_signer_bump,
        accounts.system_program,
        to_nonce_sequence,
    )?;
    let refund_handle = compute_balance_scratch(
        fhe::sub,
        BalanceScratch {
            payer: accounts.payer,
            zama_event_authority: accounts.zama_event_authority,
            zama_program: accounts.zama_program,
            host_config: accounts.host_config,
            compute_signer: accounts.compute_signer,
            token_account: to,
            lhs_acl_record: accounts.to_current_compute_acl.to_account_info(),
            lhs: old_to_handle,
            rhs_acl_record: accounts.to_output_acl.clone(),
            rhs: new_to_handle,
            output_acl_record: accounts.refund_amount_acl.clone(),
            mint: mint_key,
            compute_signer_bump,
            system_program: accounts.system_program,
            output_nonce_sequence: to_nonce_sequence,
            output_encrypted_value_label: callback_refund_amount_label(),
            output_subjects: amount_subjects.clone(),
        },
    )?;

    let new_to_acl_record = accounts.to_output_acl.key();
    let requested_refund_acl_record = accounts.requested_refund_acl.key();
    let refund_acl_record = accounts.refund_amount_acl.key();

    let to = accounts.to_account.as_mut();
    to.balance_handle = new_to_handle;
    to.balance_acl_record = new_to_acl_record;
    to.next_balance_nonce_sequence = to_nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;

    let settlement = &mut accounts.settlement_record;
    settlement.mint = mint_key;
    settlement.from_owner = from_owner;
    settlement.from_token_account = from_token_account;
    settlement.to_owner = to_owner;
    settlement.to_token_account = to_token_account;
    settlement.sent_handle = sent_handle;
    settlement.sent_acl_record = accounts.sent_amount_acl.key();
    settlement.callback_success_handle = callback_success_handle;
    settlement.callback_success_acl_record = accounts.callback_success_acl.key();
    settlement.requested_refund_handle = requested_refund_handle;
    settlement.requested_refund_acl_record = requested_refund_acl_record;
    settlement.refund_handle = refund_handle;
    settlement.refund_acl_record = refund_acl_record;
    settlement.to_balance_handle = new_to_handle;
    settlement.to_balance_acl_record = new_to_acl_record;
    settlement.from_balance_handle = [0; 32];
    settlement.from_balance_acl_record = Pubkey::default();
    settlement.transferred_handle = [0; 32];
    settlement.transferred_acl_record = Pubkey::default();
    settlement.status = CALLBACK_SETTLEMENT_PREPARED;
    settlement.bump = settlement_bump;

    Ok(PrepareTransferCallbackOutcome {
        mint: mint_key,
        to_owner,
        to_token_account,
        old_to_handle,
        old_to_acl_record,
        new_to_handle,
        new_to_acl_record,
    })
}

fn assert_active_operator_record(
    operator_record: &Account<ConfidentialOperator>,
    token_account: &Account<ConfidentialTokenAccount>,
    operator: Pubkey,
) -> Result<()> {
    assert_confidential_token_account_shape(
        token_account,
        token_account.mint,
        token_account.owner,
    )?;
    assert_operator_record_shape(
        operator_record,
        token_account.key(),
        token_account.owner,
        operator,
    )?;
    let slot = Clock::get()?.slot;
    require!(
        operator_record.expiration_slot != 0 && operator_record.expiration_slot >= slot,
        ConfidentialTokenError::OperatorExpired
    );
    Ok(())
}

fn assert_operator_record_shape(
    operator_record: &Account<ConfidentialOperator>,
    token_account: Pubkey,
    owner: Pubkey,
    operator: Pubkey,
) -> Result<()> {
    let (expected_key, expected_bump) = operator_record_address(token_account, operator);
    require_keys_eq!(
        operator_record.key(),
        expected_key,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        operator_record.to_account_info().data_len() == 8 + ConfidentialOperator::SPACE,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        operator_record.bump == expected_bump,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require_keys_eq!(
        operator_record.token_account,
        token_account,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require_keys_eq!(
        operator_record.owner,
        owner,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require_keys_eq!(
        operator_record.operator,
        operator,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    Ok(())
}

fn call_transfer_receiver_hook<'info>(
    mint: &Account<'info, ConfidentialMint>,
    from_account: &Account<'info, ConfidentialTokenAccount>,
    to_account: &Account<'info, ConfidentialTokenAccount>,
    compute_signer_account: &UncheckedAccount<'info>,
    sent_amount_acl: &Account<'info, zama_host::AclRecord>,
    callback_success_acl: &Account<'info, zama_host::AclRecord>,
    receiver_program_account: &UncheckedAccount<'info>,
    instructions_sysvar: &AccountInfo<'info>,
    remaining_accounts: &[AccountInfo<'info>],
    sent_handle: [u8; 32],
    callback_success_handle: [u8; 32],
    receiver_instruction_data: Vec<u8>,
) -> Result<()> {
    assert_confidential_mint_shape(mint)?;
    let mint_key = mint.key();
    let compute_signer = mint.compute_signer;
    let from = from_account;
    let to = to_account;
    let from_token_account = from.key();
    let to_token_account = to.key();
    let receiver_program = receiver_program_account.key();

    require!(
        receiver_program_account.executable,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require!(
        from_token_account != to_token_account,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(from.mint, mint_key, ConfidentialTokenError::MintMismatch);
    require_keys_eq!(to.mint, mint_key, ConfidentialTokenError::MintMismatch);
    assert_confidential_token_account_shape(from, mint_key, from.owner)?;
    assert_confidential_token_account_shape(to, mint_key, to.owner)?;
    assert_previous_transfer_for_receiver_hook(
        instructions_sysvar,
        mint_key,
        from_token_account,
        to_token_account,
        sent_amount_acl.key(),
    )?;
    require_keys_eq!(
        compute_signer_account.key(),
        compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    assert_transferred_amount_acl(
        sent_amount_acl,
        sent_handle,
        mint_key,
        from_token_account,
        from.owner,
        to.owner,
        compute_signer,
    )?;
    assert_callback_success_acl(
        callback_success_acl,
        callback_success_handle,
        mint_key,
        to.owner,
        compute_signer,
    )?;

    let metas = remaining_accounts
        .iter()
        .map(|account| {
            if account.is_writable {
                AccountMeta::new(*account.key, account.is_signer)
            } else {
                AccountMeta::new_readonly(*account.key, account.is_signer)
            }
        })
        .collect();
    set_return_data(&[]);
    invoke(
        &Instruction {
            program_id: receiver_program,
            accounts: metas,
            data: receiver_instruction_data,
        },
        remaining_accounts,
    )?;

    let Some((return_program, return_data)) = get_return_data() else {
        return err!(ConfidentialTokenError::ReceiverHookMismatch);
    };
    require_keys_eq!(
        return_program,
        receiver_program,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    let returned = TransferReceiverReturn::decode(&return_data)
        .map_err(|_| error!(ConfidentialTokenError::ReceiverHookMismatch))?;
    require_keys_eq!(
        returned.mint,
        mint_key,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        returned.from_token_account,
        from_token_account,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        returned.to_token_account,
        to_token_account,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require!(
        returned.sent_handle == sent_handle,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        returned.sent_acl_record,
        sent_amount_acl.key(),
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require!(
        returned.callback_success_handle == callback_success_handle,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        returned.callback_success_acl_record,
        callback_success_acl.key(),
        ConfidentialTokenError::ReceiverHookMismatch
    );

    Ok(())
}

fn assert_previous_transfer_for_receiver_hook(
    instructions_sysvar: &AccountInfo,
    mint: Pubkey,
    from_token_account: Pubkey,
    to_token_account: Pubkey,
    sent_acl_record: Pubkey,
) -> Result<()> {
    require_keys_eq!(
        instructions_sysvar.key(),
        INSTRUCTIONS_SYSVAR_ID,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    let current_index = load_current_index_checked(instructions_sysvar)
        .map_err(|_| error!(ConfidentialTokenError::ReceiverHookMismatch))?;
    let transfer_index = current_index
        .checked_sub(1)
        .ok_or(ConfidentialTokenError::ReceiverHookMismatch)?;
    let transfer_ix = load_instruction_at_checked(transfer_index as usize, instructions_sysvar)
        .map_err(|_| error!(ConfidentialTokenError::ReceiverHookMismatch))?;
    require_keys_eq!(
        transfer_ix.program_id,
        crate::ID,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require!(
        transfer_ix.data.len() >= 8,
        ConfidentialTokenError::ReceiverHookMismatch
    );

    let discriminator = &transfer_ix.data[..8];
    if discriminator == crate::instruction::ConfidentialTransfer::DISCRIMINATOR {
        assert_previous_transfer_accounts(
            &transfer_ix.accounts,
            PreviousTransferAccountIndexes {
                mint: 1,
                from_token_account: 2,
                to_token_account: 3,
                sent_acl_record: 11,
            },
            mint,
            from_token_account,
            to_token_account,
            sent_acl_record,
        )
    } else if discriminator == crate::instruction::ConfidentialTransferFrom::DISCRIMINATOR {
        assert_previous_transfer_accounts(
            &transfer_ix.accounts,
            PreviousTransferAccountIndexes {
                mint: 1,
                from_token_account: 2,
                to_token_account: 3,
                sent_acl_record: 12,
            },
            mint,
            from_token_account,
            to_token_account,
            sent_acl_record,
        )
    } else {
        err!(ConfidentialTokenError::ReceiverHookMismatch)
    }
}

struct PreviousTransferAccountIndexes {
    mint: usize,
    from_token_account: usize,
    to_token_account: usize,
    sent_acl_record: usize,
}

fn assert_previous_transfer_accounts(
    accounts: &[AccountMeta],
    indexes: PreviousTransferAccountIndexes,
    mint: Pubkey,
    from_token_account: Pubkey,
    to_token_account: Pubkey,
    sent_acl_record: Pubkey,
) -> Result<()> {
    let mint_meta = accounts
        .get(indexes.mint)
        .ok_or(ConfidentialTokenError::ReceiverHookMismatch)?;
    let from_meta = accounts
        .get(indexes.from_token_account)
        .ok_or(ConfidentialTokenError::ReceiverHookMismatch)?;
    let to_meta = accounts
        .get(indexes.to_token_account)
        .ok_or(ConfidentialTokenError::ReceiverHookMismatch)?;
    let sent_meta = accounts
        .get(indexes.sent_acl_record)
        .ok_or(ConfidentialTokenError::ReceiverHookMismatch)?;
    require_keys_eq!(
        mint_meta.pubkey,
        mint,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        from_meta.pubkey,
        from_token_account,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        to_meta.pubkey,
        to_token_account,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        sent_meta.pubkey,
        sent_acl_record,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    Ok(())
}

fn write_transfer_receiver_hook_call(
    hook_record: &mut Account<TransferReceiverHookCall>,
    mint: Pubkey,
    from_token_account: Pubkey,
    to_token_account: Pubkey,
    sent_handle: [u8; 32],
    sent_acl_record: Pubkey,
    callback_success_handle: [u8; 32],
    callback_success_acl_record: Pubkey,
    receiver_program: Pubkey,
    caller: Pubkey,
    bump: u8,
) {
    hook_record.mint = mint;
    hook_record.from_token_account = from_token_account;
    hook_record.to_token_account = to_token_account;
    hook_record.sent_handle = sent_handle;
    hook_record.sent_acl_record = sent_acl_record;
    hook_record.callback_success_handle = callback_success_handle;
    hook_record.callback_success_acl_record = callback_success_acl_record;
    hook_record.receiver_program = receiver_program;
    hook_record.caller = caller;
    hook_record.bump = bump;
}

fn finalize_transfer_callback_settlement<'info>(
    mut accounts: FinalizeTransferCallbackAccounts<'_, 'info>,
    compute_signer_bump: u8,
) -> Result<FinalizeTransferCallbackOutcome> {
    assert_confidential_mint_shape(accounts.mint)?;
    let mint_key = accounts.mint.key();
    let compute_signer = accounts.mint.compute_signer;
    let from = accounts.from_account.as_ref();
    let to = accounts.to_account;
    let from_nonce_sequence = from.next_balance_nonce_sequence;
    let old_from_handle = from.balance_handle;
    let old_from_acl_record = from.balance_acl_record;
    let from_owner = accounts.settlement_record.from_owner;
    let to_owner = accounts.settlement_record.to_owner;
    let from_token_account = accounts.settlement_record.from_token_account;
    let to_token_account = accounts.settlement_record.to_token_account;
    let sent_handle = accounts.settlement_record.sent_handle;
    let refund_handle = accounts.settlement_record.refund_handle;
    let refund_acl_record = accounts.settlement_record.refund_acl_record;
    let amount_subjects = transferred_amount_acl_subjects(from_owner, to_owner, compute_signer);

    assert_transfer_callback_settlement_shape(accounts.settlement_record, mint_key, sent_handle)?;
    require!(
        accounts.settlement_record.status == CALLBACK_SETTLEMENT_PREPARED,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        accounts.settlement_record.mint,
        mint_key,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        from.key(),
        from_token_account,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        to.key(),
        to_token_account,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        from.owner,
        from_owner,
        ConfidentialTokenError::OwnerMismatch
    );
    require_keys_eq!(to.owner, to_owner, ConfidentialTokenError::OwnerMismatch);
    require_keys_eq!(from.mint, mint_key, ConfidentialTokenError::MintMismatch);
    require_keys_eq!(to.mint, mint_key, ConfidentialTokenError::MintMismatch);
    assert_confidential_token_account_shape(from, mint_key, from_owner)?;
    assert_confidential_token_account_shape(to, mint_key, to_owner)?;
    require_keys_eq!(
        accounts.compute_signer.key(),
        compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    assert_current_balance_acl(
        accounts.from_current_compute_acl,
        old_from_acl_record,
        from,
        mint_key,
    )?;
    require_keys_eq!(
        to.balance_acl_record,
        accounts.settlement_record.to_balance_acl_record,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require!(
        to.balance_handle == accounts.settlement_record.to_balance_handle,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        accounts.sent_amount_acl.key(),
        accounts.settlement_record.sent_acl_record,
        ConfidentialTokenError::AmountAclMismatch
    );
    assert_transferred_amount_acl(
        accounts.sent_amount_acl,
        sent_handle,
        mint_key,
        from_token_account,
        from_owner,
        to_owner,
        compute_signer,
    )?;
    assert_callback_refund_acl(
        accounts.refund_amount_acl,
        refund_handle,
        mint_key,
        to_token_account,
        from_owner,
        to_owner,
        compute_signer,
    )?;
    require_keys_eq!(
        accounts.refund_amount_acl.key(),
        accounts.settlement_record.refund_acl_record,
        ConfidentialTokenError::AmountAclMismatch
    );

    let new_from_handle = add_balance(
        accounts.payer,
        accounts.zama_event_authority,
        accounts.zama_program,
        accounts.host_config,
        accounts.compute_signer,
        from,
        accounts.from_current_compute_acl.to_account_info(),
        old_from_handle,
        accounts.refund_amount_acl.to_account_info(),
        refund_handle,
        accounts.from_output_acl.clone(),
        mint_key,
        compute_signer_bump,
        accounts.system_program,
        from_nonce_sequence,
    )?;
    let final_transferred_handle = compute_balance_scratch(
        fhe::sub,
        BalanceScratch {
            payer: accounts.payer,
            zama_event_authority: accounts.zama_event_authority,
            zama_program: accounts.zama_program,
            host_config: accounts.host_config,
            compute_signer: accounts.compute_signer,
            token_account: from,
            lhs_acl_record: accounts.sent_amount_acl.to_account_info(),
            lhs: sent_handle,
            rhs_acl_record: accounts.refund_amount_acl.to_account_info(),
            rhs: refund_handle,
            output_acl_record: accounts.transferred_amount_acl.clone(),
            mint: mint_key,
            compute_signer_bump,
            system_program: accounts.system_program,
            output_nonce_sequence: from_nonce_sequence,
            output_encrypted_value_label: callback_final_transferred_label(),
            output_subjects: amount_subjects,
        },
    )?;

    let new_from_acl_record = accounts.from_output_acl.key();
    let transferred_acl_record = accounts.transferred_amount_acl.key();
    let from = accounts.from_account.as_mut();
    from.balance_handle = new_from_handle;
    from.balance_acl_record = new_from_acl_record;
    from.next_balance_nonce_sequence = from_nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;

    let settlement = &mut accounts.settlement_record;
    settlement.from_balance_handle = new_from_handle;
    settlement.from_balance_acl_record = new_from_acl_record;
    settlement.transferred_handle = final_transferred_handle;
    settlement.transferred_acl_record = transferred_acl_record;
    settlement.status = CALLBACK_SETTLEMENT_FINALIZED;

    Ok(FinalizeTransferCallbackOutcome {
        mint: mint_key,
        from_owner,
        from_token_account,
        old_from_handle,
        old_from_acl_record,
        new_from_handle,
        new_from_acl_record,
        to_owner,
        to_token_account,
        refund_handle,
        refund_acl_record,
    })
}

fn assert_transfer_callback_settlement_shape(
    settlement_record: &Account<TransferCallbackSettlement>,
    mint: Pubkey,
    sent_handle: [u8; 32],
) -> Result<()> {
    let (expected_key, expected_bump) = transfer_callback_settlement_address(mint, sent_handle);
    require_keys_eq!(
        settlement_record.key(),
        expected_key,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require!(
        settlement_record.to_account_info().data_len() == 8 + TransferCallbackSettlement::SPACE,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require!(
        settlement_record.bump == expected_bump,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    Ok(())
}

fn assert_transfer_receiver_hook_call_shape(
    hook_record: &Account<TransferReceiverHookCall>,
    mint: Pubkey,
    from_token_account: Pubkey,
    to_token_account: Pubkey,
    sent_handle: [u8; 32],
    sent_acl_record: Pubkey,
    callback_success_handle: [u8; 32],
    callback_success_acl_record: Pubkey,
) -> Result<()> {
    let (expected_key, expected_bump) = transfer_receiver_hook_address(mint, sent_handle);
    require_keys_eq!(
        hook_record.key(),
        expected_key,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require!(
        hook_record.to_account_info().data_len() == 8 + TransferReceiverHookCall::SPACE,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require!(
        hook_record.bump == expected_bump,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        hook_record.mint,
        mint,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        hook_record.from_token_account,
        from_token_account,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        hook_record.to_token_account,
        to_token_account,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require!(
        hook_record.sent_handle == sent_handle,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        hook_record.sent_acl_record,
        sent_acl_record,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require!(
        hook_record.callback_success_handle == callback_success_handle,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        hook_record.callback_success_acl_record,
        callback_success_acl_record,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    Ok(())
}

fn assert_transfer_amount_acl(
    amount_acl: &Account<zama_host::AclRecord>,
    amount_handle: [u8; 32],
    mint: Pubkey,
    transfer_authority: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    assert_amount_acl_record_shape(amount_acl)?;
    require!(
        zama_host::handle_fhe_type(amount_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        amount_acl.handle == amount_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.app_account,
        transfer_authority,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.encrypted_value_label == transfer_amount_label(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.nonce_key == nonce_key(mint, amount_acl.app_account, transfer_amount_label()),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

fn assert_burn_amount_acl(
    amount_acl: &Account<zama_host::AclRecord>,
    amount_handle: [u8; 32],
    mint: Pubkey,
    owner: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    assert_owner_amount_acl(
        amount_acl,
        amount_handle,
        mint,
        owner,
        compute_signer,
        burn_amount_label(),
    )
}

fn assert_transferred_amount_acl(
    amount_acl: &Account<zama_host::AclRecord>,
    amount_handle: [u8; 32],
    mint: Pubkey,
    from_token_account: Pubkey,
    from_owner: Pubkey,
    to_owner: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    assert_amount_acl_record_shape(amount_acl)?;
    require!(
        zama_host::handle_fhe_type(amount_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        amount_acl.handle == amount_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.app_account,
        from_token_account,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.encrypted_value_label == transferred_amount_label(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.nonce_key == nonce_key(mint, from_token_account, transferred_amount_label()),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(from_owner, zama_host::ACL_ROLE_PUBLIC_DECRYPT),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(to_owner, zama_host::ACL_ROLE_PUBLIC_DECRYPT),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

fn assert_callback_success_acl(
    success_acl: &Account<zama_host::AclRecord>,
    success_handle: [u8; 32],
    mint: Pubkey,
    callback_authority: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    assert_amount_acl_record_shape(success_acl)?;
    require!(
        zama_host::handle_fhe_type(success_handle) == 0,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        success_acl.handle == success_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        success_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        success_acl.app_account,
        callback_authority,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        success_acl.encrypted_value_label == callback_success_label(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        success_acl.nonce_key == nonce_key(mint, callback_authority, callback_success_label()),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        success_acl.inline_subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

fn assert_callback_refund_acl(
    refund_acl: &Account<zama_host::AclRecord>,
    refund_handle: [u8; 32],
    mint: Pubkey,
    to_token_account: Pubkey,
    from_owner: Pubkey,
    to_owner: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    assert_amount_acl_record_shape(refund_acl)?;
    require!(
        zama_host::handle_fhe_type(refund_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        refund_acl.handle == refund_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        refund_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        refund_acl.app_account,
        to_token_account,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        refund_acl.encrypted_value_label == callback_refund_amount_label(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        refund_acl.nonce_key == nonce_key(mint, to_token_account, callback_refund_amount_label()),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        refund_acl.inline_subject_has_role(from_owner, zama_host::ACL_ROLE_PUBLIC_DECRYPT),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        refund_acl.inline_subject_has_role(to_owner, zama_host::ACL_ROLE_PUBLIC_DECRYPT),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        refund_acl.inline_subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

fn assert_owner_amount_acl(
    amount_acl: &Account<zama_host::AclRecord>,
    amount_handle: [u8; 32],
    mint: Pubkey,
    owner: Pubkey,
    compute_signer: Pubkey,
    encrypted_value_label: [u8; 32],
) -> Result<()> {
    assert_amount_acl_record_shape(amount_acl)?;
    require!(
        zama_host::handle_fhe_type(amount_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        amount_acl.handle == amount_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.app_account,
        owner,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.encrypted_value_label == encrypted_value_label,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.nonce_key == nonce_key(mint, owner, encrypted_value_label),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

fn assert_token_amount_acl(
    amount_acl: &Account<zama_host::AclRecord>,
    amount_handle: [u8; 32],
    mint: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    assert_amount_acl_record_shape(amount_acl)?;
    require!(
        zama_host::handle_fhe_type(amount_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        amount_acl.handle == amount_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        is_token_amount_label(amount_acl.encrypted_value_label),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.nonce_key
            == nonce_key(
                mint,
                amount_acl.app_account,
                amount_acl.encrypted_value_label
            ),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

fn is_token_amount_label(encrypted_value_label: [u8; 32]) -> bool {
    encrypted_value_label == wrap_amount_label()
        || encrypted_value_label == burn_amount_label()
        || encrypted_value_label == transfer_amount_label()
        || encrypted_value_label == burned_amount_label()
        || encrypted_value_label == transferred_amount_label()
        || encrypted_value_label == callback_refund_amount_label()
}

fn assert_burned_amount_acl(
    amount_acl: &Account<zama_host::AclRecord>,
    burned_handle: [u8; 32],
    mint: Pubkey,
    token_account: Pubkey,
    owner: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    assert_amount_acl_record_shape(amount_acl)?;
    require!(
        zama_host::handle_fhe_type(burned_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        amount_acl.handle == burned_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.app_account,
        token_account,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.encrypted_value_label == burned_amount_label(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.nonce_key == nonce_key(mint, token_account, burned_amount_label()),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(owner, zama_host::ACL_ROLE_PUBLIC_DECRYPT),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

fn assert_material_commitment(
    material: &Account<zama_host::HandleMaterialCommitment>,
    material_key: Pubkey,
    acl_record: &Account<zama_host::AclRecord>,
    handle: [u8; 32],
) -> Result<()> {
    let acl_record_key = acl_record.key();
    let (expected_key, expected_bump) = zama_host::handle_material_address(acl_record_key);
    require_keys_eq!(
        material_key,
        expected_key,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require!(
        material.bump == expected_bump,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require!(
        material.to_account_info().data_len() == 8 + zama_host::HandleMaterialCommitment::SPACE,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require_keys_eq!(
        material.acl_record,
        acl_record_key,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require!(
        material.handle == handle,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require!(
        material.state == zama_host::HANDLE_MATERIAL_STATE_COMMITTED,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require!(
        material.material_commitment_hash
            == zama_host::handle_material_commitment_hash(
                material_key,
                acl_record_key,
                material.key_id,
                material.ciphertext_digest,
                material.sns_ciphertext_digest,
                material.coprocessor_set_digest,
            ),
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require_keys_eq!(
        acl_record.material_commitment,
        material_key,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require!(
        acl_record.material_commitment_hash == material.material_commitment_hash
            && acl_record.material_key_id == material.key_id,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    Ok(())
}

fn assert_public_decrypt_released(acl_record: &Account<zama_host::AclRecord>) -> Result<()> {
    assert_amount_acl_record_shape(acl_record)?;
    require!(
        acl_record.public_decrypt,
        ConfidentialTokenError::PublicDecryptNotReleased
    );
    Ok(())
}

fn assert_canonical_vault_token_account(
    vault_usdc: Pubkey,
    vault_authority: Pubkey,
    underlying_mint: Pubkey,
) -> Result<()> {
    require_keys_eq!(
        vault_usdc,
        get_associated_token_address_with_program_id(
            &vault_authority,
            &underlying_mint,
            &spl_token::ID,
        ),
        ConfidentialTokenError::VaultAccountMismatch
    );
    Ok(())
}

fn assert_confidential_token_account_key(
    token_account: Pubkey,
    mint: Pubkey,
    owner: Pubkey,
) -> Result<()> {
    require_keys_eq!(
        token_account,
        token_account_address(mint, owner).0,
        ConfidentialTokenError::TokenAccountMismatch
    );
    Ok(())
}

fn assert_confidential_mint_shape(mint: &Account<ConfidentialMint>) -> Result<()> {
    require!(
        mint.to_account_info().data_len() == 8 + ConfidentialMint::SPACE,
        ConfidentialTokenError::MintAccountMismatch
    );
    require_keys_eq!(
        mint.acl_domain_key,
        mint.key(),
        ConfidentialTokenError::AclDomainKeyMismatch
    );
    require_keys_eq!(
        mint.compute_signer,
        compute_signer_address(mint.key()).0,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    Ok(())
}

fn assert_confidential_token_account_shape(
    token_account: &Account<ConfidentialTokenAccount>,
    mint: Pubkey,
    owner: Pubkey,
) -> Result<()> {
    let expected_bump = token_account_address(mint, owner).1;
    assert_confidential_token_account_key(token_account.key(), mint, owner)?;
    require!(
        token_account.to_account_info().data_len() == 8 + ConfidentialTokenAccount::SPACE,
        ConfidentialTokenError::TokenAccountMismatch
    );
    require!(
        token_account.bump == expected_bump,
        ConfidentialTokenError::TokenAccountMismatch
    );
    require_keys_eq!(
        token_account.mint,
        mint,
        ConfidentialTokenError::MintMismatch
    );
    require_keys_eq!(
        token_account.owner,
        owner,
        ConfidentialTokenError::OwnerMismatch
    );
    Ok(())
}

fn assert_disclosure_signature(
    instructions_sysvar: &AccountInfo,
    verifier: Pubkey,
    mint: Pubkey,
    handle: [u8; 32],
    cleartext_amount: u64,
) -> Result<()> {
    require_keys_eq!(
        instructions_sysvar.key(),
        INSTRUCTIONS_SYSVAR_ID,
        ConfidentialTokenError::DisclosureProofSignatureMissing
    );
    let message = disclosure_proof_message(mint, handle, cleartext_amount, crate::ID);
    let current_index = load_current_index_checked(instructions_sysvar)
        .map_err(|_| error!(ConfidentialTokenError::DisclosureProofSignatureMissing))?;
    let verifier_index = current_index
        .checked_sub(1)
        .ok_or(ConfidentialTokenError::DisclosureProofSignatureMissing)?;
    let verifier_ix = load_instruction_at_checked(verifier_index as usize, instructions_sysvar)
        .map_err(|_| error!(ConfidentialTokenError::DisclosureProofSignatureMissing))?;
    require_keys_eq!(
        verifier_ix.program_id,
        ED25519_PROGRAM_ID,
        ConfidentialTokenError::DisclosureProofSignatureMissing
    );
    require!(
        ed25519_instruction_contains_message(&verifier_ix.data, verifier.as_ref(), &message),
        ConfidentialTokenError::DisclosureProofSignatureMissing
    );
    Ok(())
}

/// Builds the message that a KMS disclosure response signs for this token PoC.
pub fn disclosure_proof_message(
    mint: Pubkey,
    handle: [u8; 32],
    cleartext_amount: u64,
    program_id: Pubkey,
) -> Vec<u8> {
    let mut message = Vec::with_capacity(
        DISCLOSURE_PROOF_DOMAIN_SEPARATOR.len() + 32 + 32 + 32 + std::mem::size_of::<u64>(),
    );
    message.extend_from_slice(DISCLOSURE_PROOF_DOMAIN_SEPARATOR);
    message.extend_from_slice(program_id.as_ref());
    message.extend_from_slice(mint.as_ref());
    message.extend_from_slice(&handle);
    message.extend_from_slice(&cleartext_amount.to_le_bytes());
    message
}

fn ed25519_instruction_contains_message(
    data: &[u8],
    expected_pubkey: &[u8],
    expected_message: &[u8],
) -> bool {
    if data.len() < ED25519_SIGNATURE_OFFSETS_START {
        return false;
    }
    if data[1] != 0 {
        return false;
    }
    let signature_count = data[0] as usize;
    if signature_count == 0 {
        return false;
    }
    let expected_offsets_end = ED25519_SIGNATURE_OFFSETS_START
        .saturating_add(signature_count.saturating_mul(ED25519_SIGNATURE_OFFSETS_SERIALIZED_SIZE));
    if data.len() < expected_offsets_end {
        return false;
    }

    for signature_index in 0..signature_count {
        let start = ED25519_SIGNATURE_OFFSETS_START.saturating_add(
            signature_index.saturating_mul(ED25519_SIGNATURE_OFFSETS_SERIALIZED_SIZE),
        );
        let fields = &data[start..start + ED25519_SIGNATURE_OFFSETS_SERIALIZED_SIZE];
        let signature_offset = read_u16_le(fields, 0) as usize;
        let signature_instruction_index = read_u16_le(fields, 2);
        let public_key_offset = read_u16_le(fields, 4) as usize;
        let public_key_instruction_index = read_u16_le(fields, 6);
        let message_data_offset = read_u16_le(fields, 8) as usize;
        let message_data_size = read_u16_le(fields, 10) as usize;
        let message_instruction_index = read_u16_le(fields, 12);

        if signature_instruction_index != u16::MAX
            || public_key_instruction_index != u16::MAX
            || message_instruction_index != u16::MAX
        {
            continue;
        }
        let Some(signature_end) = signature_offset.checked_add(ED25519_SIGNATURE_SERIALIZED_SIZE)
        else {
            continue;
        };
        let Some(public_key_end) = public_key_offset.checked_add(ED25519_PUBKEY_SERIALIZED_SIZE)
        else {
            continue;
        };
        let Some(message_end) = message_data_offset.checked_add(message_data_size) else {
            continue;
        };
        if signature_end > data.len() || public_key_end > data.len() || message_end > data.len() {
            continue;
        }
        if &data[public_key_offset..public_key_end] != expected_pubkey {
            continue;
        }
        if &data[message_data_offset..message_end] == expected_message {
            return true;
        }
    }
    false
}

fn read_u16_le(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([data[offset], data[offset + 1]])
}

fn assert_current_balance_acl(
    balance_acl: &Account<zama_host::AclRecord>,
    balance_acl_key: Pubkey,
    token_account: &Account<ConfidentialTokenAccount>,
    mint: Pubkey,
) -> Result<()> {
    assert_current_acl_record_shape(balance_acl)?;
    require_keys_eq!(
        balance_acl_key,
        token_account.balance_acl_record,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        balance_acl.handle == token_account.balance_handle,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require_keys_eq!(
        balance_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require_keys_eq!(
        balance_acl.app_account,
        token_account.key(),
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        balance_acl.encrypted_value_label == balance_label(),
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        balance_acl.nonce_key == balance_nonce_key(mint, token_account.key()),
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    Ok(())
}

fn assert_current_total_supply_acl(
    supply_acl: &Account<zama_host::AclRecord>,
    supply_acl_key: Pubkey,
    mint: &Account<ConfidentialMint>,
    mint_key: Pubkey,
    total_supply_authority: Pubkey,
) -> Result<()> {
    assert_current_acl_record_shape(supply_acl)?;
    require_keys_eq!(
        supply_acl_key,
        mint.total_supply_acl_record,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        supply_acl.handle == mint.total_supply_handle,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require_keys_eq!(
        supply_acl.acl_domain_key,
        mint_key,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require_keys_eq!(
        supply_acl.app_account,
        total_supply_authority,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        supply_acl.encrypted_value_label == total_supply_label(),
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        supply_acl.nonce_key == total_supply_nonce_key(mint_key, total_supply_authority),
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    Ok(())
}

fn assert_current_acl_record_shape(acl_record: &Account<zama_host::AclRecord>) -> Result<()> {
    let (expected_key, expected_bump) =
        zama_host::acl_record_address(acl_record.nonce_key, acl_record.nonce_sequence);
    require_keys_eq!(
        acl_record.key(),
        expected_key,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        acl_record.to_account_info().data_len() == 8 + zama_host::AclRecord::SPACE,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        acl_record.bump == expected_bump,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        zama_host::acl_record_subject_slots_are_canonical(acl_record),
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    Ok(())
}

fn assert_amount_acl_record_shape(acl_record: &Account<zama_host::AclRecord>) -> Result<()> {
    let (expected_key, expected_bump) =
        zama_host::acl_record_address(acl_record.nonce_key, acl_record.nonce_sequence);
    require_keys_eq!(
        acl_record.key(),
        expected_key,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        acl_record.to_account_info().data_len() == 8 + zama_host::AclRecord::SPACE,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        acl_record.bump == expected_bump,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        zama_host::acl_record_subject_slots_are_canonical(acl_record),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

fn create_operator_record_if_needed<'info>(
    payer: &AccountInfo<'info>,
    operator_record: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    token_account: Pubkey,
    owner: Pubkey,
    operator: Pubkey,
    bump: u8,
) -> Result<()> {
    if operator_record.owner == &crate::ID {
        assert_existing_operator_record(operator_record, token_account, owner, operator, bump)?;
        return Ok(());
    }
    require_keys_eq!(
        *operator_record.owner,
        System::id(),
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        operator_record.data_is_empty(),
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        !operator_record.executable,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    let rent = Rent::get()?.minimum_balance(8 + ConfidentialOperator::SPACE);
    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            operator_record.key,
            rent,
            (8 + ConfidentialOperator::SPACE) as u64,
            &crate::ID,
        ),
        &[
            payer.clone(),
            operator_record.clone(),
            system_program.clone(),
        ],
        &[&[
            b"operator",
            token_account.as_ref(),
            operator.as_ref(),
            &[bump],
        ]],
    )?;
    require_keys_eq!(
        *operator_record.owner,
        crate::ID,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        !operator_record.executable,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        operator_record.data_len() == 8 + ConfidentialOperator::SPACE,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        operator_record.lamports() >= rent,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    Ok(())
}

fn assert_existing_operator_record(
    operator_record: &AccountInfo,
    token_account: Pubkey,
    owner: Pubkey,
    operator: Pubkey,
    bump: u8,
) -> Result<()> {
    require!(
        operator_record.data_len() == 8 + ConfidentialOperator::SPACE,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    let data = operator_record.try_borrow_data()?;
    let mut cursor = &data[..];
    let existing = ConfidentialOperator::try_deserialize(&mut cursor)
        .map_err(|_| error!(ConfidentialTokenError::OperatorRecordMismatch))?;
    require_keys_eq!(
        existing.token_account,
        token_account,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require_keys_eq!(
        existing.owner,
        owner,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require_keys_eq!(
        existing.operator,
        operator,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        existing.bump == bump,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    Ok(())
}

fn write_operator_record(info: &AccountInfo, record: &ConfidentialOperator) -> Result<()> {
    let mut data = info.try_borrow_mut_data()?;
    let mut cursor = &mut data[..];
    record.try_serialize(&mut cursor)?;
    Ok(())
}

fn add_balance<'info>(
    payer: &Signer<'info>,
    zama_event_authority: &UncheckedAccount<'info>,
    zama_program: &Program<'info, ZamaHost>,
    host_config: &Account<'info, zama_host::HostConfig>,
    compute_signer: &UncheckedAccount<'info>,
    token_account: &Account<'info, ConfidentialTokenAccount>,
    lhs_acl_record: AccountInfo<'info>,
    lhs: [u8; 32],
    rhs_acl_record: AccountInfo<'info>,
    rhs: [u8; 32],
    output_acl_record: AccountInfo<'info>,
    mint: Pubkey,
    compute_signer_bump: u8,
    system_program: &Program<'info, System>,
    output_nonce_sequence: u64,
) -> Result<[u8; 32]> {
    compute_balance_with(
        fhe::add,
        BalanceCompute {
            payer,
            zama_event_authority,
            zama_program,
            host_config,
            compute_signer,
            token_account,
            lhs_acl_record,
            lhs,
            rhs_acl_record,
            rhs,
            output_acl_record,
            mint,
            compute_signer_bump,
            system_program,
            output_nonce_sequence,
        },
    )
}

fn ge_balance<'info>(
    payer: &Signer<'info>,
    zama_event_authority: &UncheckedAccount<'info>,
    zama_program: &Program<'info, ZamaHost>,
    host_config: &Account<'info, zama_host::HostConfig>,
    compute_signer: &UncheckedAccount<'info>,
    token_account: &Account<'info, ConfidentialTokenAccount>,
    lhs_acl_record: AccountInfo<'info>,
    lhs: [u8; 32],
    rhs_acl_record: AccountInfo<'info>,
    rhs: [u8; 32],
    output_acl_record: AccountInfo<'info>,
    mint: Pubkey,
    compute_signer_bump: u8,
    system_program: &Program<'info, System>,
    output_nonce_sequence: u64,
    output_encrypted_value_label: [u8; 32],
) -> Result<[u8; 32]> {
    fhe::ge(fhe::BinaryOp {
        payer,
        event_authority: zama_event_authority,
        zama_program,
        host_config,
        compute_signer,
        app_account_authority: token_account,
        lhs_acl_record,
        lhs,
        rhs_acl_record,
        rhs,
        scalar: false,
        output_acl_record,
        output_fhe_type: 0,
        acl_domain_key: mint,
        compute_signer_bump,
        system_program,
        output_nonce_key: nonce_key(mint, token_account.key(), output_encrypted_value_label),
        output_nonce_sequence,
        output_encrypted_value_label,
        output_subjects: compute_acl_subject(compute_signer.key()),
        output_public_decrypt: false,
    })
}

fn select_balance<'info>(
    payer: &Signer<'info>,
    zama_event_authority: &UncheckedAccount<'info>,
    zama_program: &Program<'info, ZamaHost>,
    host_config: &Account<'info, zama_host::HostConfig>,
    compute_signer: &UncheckedAccount<'info>,
    token_account: &Account<'info, ConfidentialTokenAccount>,
    control_acl_record: AccountInfo<'info>,
    control: [u8; 32],
    if_true_acl_record: AccountInfo<'info>,
    if_true: [u8; 32],
    if_false_acl_record: AccountInfo<'info>,
    if_false: [u8; 32],
    output_acl_record: AccountInfo<'info>,
    mint: Pubkey,
    compute_signer_bump: u8,
    system_program: &Program<'info, System>,
    output_nonce_sequence: u64,
) -> Result<[u8; 32]> {
    fhe::if_then_else(fhe::TernaryOp {
        payer,
        event_authority: zama_event_authority,
        zama_program,
        host_config,
        compute_signer,
        app_account_authority: token_account,
        control_acl_record,
        control,
        if_true_acl_record,
        if_true,
        if_false_acl_record,
        if_false,
        output_acl_record,
        output_fhe_type: BALANCE_FHE_TYPE,
        acl_domain_key: mint,
        compute_signer_bump,
        system_program,
        output_nonce_key: balance_nonce_key(mint, token_account.key()),
        output_nonce_sequence,
        output_encrypted_value_label: balance_label(),
        output_subjects: balance_acl_subjects(token_account.owner, compute_signer.key()),
        output_public_decrypt: false,
    })
}

struct BalanceCompute<'a, 'info> {
    payer: &'a Signer<'info>,
    zama_event_authority: &'a UncheckedAccount<'info>,
    zama_program: &'a Program<'info, ZamaHost>,
    host_config: &'a Account<'info, zama_host::HostConfig>,
    compute_signer: &'a UncheckedAccount<'info>,
    token_account: &'a Account<'info, ConfidentialTokenAccount>,
    lhs_acl_record: AccountInfo<'info>,
    lhs: [u8; 32],
    rhs_acl_record: AccountInfo<'info>,
    rhs: [u8; 32],
    output_acl_record: AccountInfo<'info>,
    mint: Pubkey,
    compute_signer_bump: u8,
    system_program: &'a Program<'info, System>,
    output_nonce_sequence: u64,
}

struct BalanceScratch<'a, 'info> {
    payer: &'a Signer<'info>,
    zama_event_authority: &'a UncheckedAccount<'info>,
    zama_program: &'a Program<'info, ZamaHost>,
    host_config: &'a Account<'info, zama_host::HostConfig>,
    compute_signer: &'a UncheckedAccount<'info>,
    token_account: &'a Account<'info, ConfidentialTokenAccount>,
    lhs_acl_record: AccountInfo<'info>,
    lhs: [u8; 32],
    rhs_acl_record: AccountInfo<'info>,
    rhs: [u8; 32],
    output_acl_record: AccountInfo<'info>,
    mint: Pubkey,
    compute_signer_bump: u8,
    system_program: &'a Program<'info, System>,
    output_nonce_sequence: u64,
    output_encrypted_value_label: [u8; 32],
    output_subjects: Vec<AclSubjectEntry>,
}

struct TernaryScratch<'a, 'info> {
    payer: &'a Signer<'info>,
    zama_event_authority: &'a UncheckedAccount<'info>,
    zama_program: &'a Program<'info, ZamaHost>,
    host_config: &'a Account<'info, zama_host::HostConfig>,
    compute_signer: &'a UncheckedAccount<'info>,
    token_account: &'a Account<'info, ConfidentialTokenAccount>,
    control_acl_record: AccountInfo<'info>,
    control: [u8; 32],
    if_true_acl_record: AccountInfo<'info>,
    if_true: [u8; 32],
    if_false_acl_record: AccountInfo<'info>,
    if_false: [u8; 32],
    output_acl_record: AccountInfo<'info>,
    mint: Pubkey,
    compute_signer_bump: u8,
    system_program: &'a Program<'info, System>,
    output_nonce_sequence: u64,
    output_encrypted_value_label: [u8; 32],
    output_subjects: Vec<AclSubjectEntry>,
}

fn compute_balance_with<'info>(
    op: for<'a> fn(fhe::BinaryOp<'a, 'info>) -> Result<[u8; 32]>,
    request: BalanceCompute<'_, 'info>,
) -> Result<[u8; 32]> {
    let token_account_key = request.token_account.key();
    op(fhe::BinaryOp {
        payer: request.payer,
        event_authority: request.zama_event_authority,
        zama_program: request.zama_program,
        host_config: request.host_config,
        compute_signer: request.compute_signer,
        app_account_authority: request.token_account,
        lhs_acl_record: request.lhs_acl_record,
        lhs: request.lhs,
        rhs_acl_record: request.rhs_acl_record,
        rhs: request.rhs,
        scalar: false,
        output_acl_record: request.output_acl_record,
        output_fhe_type: BALANCE_FHE_TYPE,
        acl_domain_key: request.mint,
        compute_signer_bump: request.compute_signer_bump,
        system_program: request.system_program,
        output_nonce_key: balance_nonce_key(request.mint, token_account_key),
        output_nonce_sequence: request.output_nonce_sequence,
        output_encrypted_value_label: balance_label(),
        output_subjects: balance_acl_subjects(
            request.token_account.owner,
            request.compute_signer.key(),
        ),
        output_public_decrypt: false,
    })
}

fn compute_balance_scratch<'info>(
    op: for<'a> fn(fhe::BinaryOp<'a, 'info>) -> Result<[u8; 32]>,
    request: BalanceScratch<'_, 'info>,
) -> Result<[u8; 32]> {
    op(fhe::BinaryOp {
        payer: request.payer,
        event_authority: request.zama_event_authority,
        zama_program: request.zama_program,
        host_config: request.host_config,
        compute_signer: request.compute_signer,
        app_account_authority: request.token_account,
        lhs_acl_record: request.lhs_acl_record,
        lhs: request.lhs,
        rhs_acl_record: request.rhs_acl_record,
        rhs: request.rhs,
        scalar: false,
        output_acl_record: request.output_acl_record,
        output_fhe_type: BALANCE_FHE_TYPE,
        acl_domain_key: request.mint,
        compute_signer_bump: request.compute_signer_bump,
        system_program: request.system_program,
        output_nonce_key: nonce_key(
            request.mint,
            request.token_account.key(),
            request.output_encrypted_value_label,
        ),
        output_nonce_sequence: request.output_nonce_sequence,
        output_encrypted_value_label: request.output_encrypted_value_label,
        output_subjects: request.output_subjects,
        output_public_decrypt: false,
    })
}

fn select_amount_scratch<'info>(request: TernaryScratch<'_, 'info>) -> Result<[u8; 32]> {
    fhe::if_then_else(fhe::TernaryOp {
        payer: request.payer,
        event_authority: request.zama_event_authority,
        zama_program: request.zama_program,
        host_config: request.host_config,
        compute_signer: request.compute_signer,
        app_account_authority: request.token_account,
        control_acl_record: request.control_acl_record,
        control: request.control,
        if_true_acl_record: request.if_true_acl_record,
        if_true: request.if_true,
        if_false_acl_record: request.if_false_acl_record,
        if_false: request.if_false,
        output_acl_record: request.output_acl_record,
        output_fhe_type: BALANCE_FHE_TYPE,
        acl_domain_key: request.mint,
        compute_signer_bump: request.compute_signer_bump,
        system_program: request.system_program,
        output_nonce_key: nonce_key(
            request.mint,
            request.token_account.key(),
            request.output_encrypted_value_label,
        ),
        output_nonce_sequence: request.output_nonce_sequence,
        output_encrypted_value_label: request.output_encrypted_value_label,
        output_subjects: request.output_subjects,
        output_public_decrypt: false,
    })
}

fn trivial_encrypt_balance_acl<'info>(
    payer: &Signer<'info>,
    mint: &Account<'info, ConfidentialMint>,
    compute_signer: &UncheckedAccount<'info>,
    token_account: &Account<'info, ConfidentialTokenAccount>,
    acl_record: AccountInfo<'info>,
    zama_event_authority: &UncheckedAccount<'info>,
    zama_program: &Program<'info, ZamaHost>,
    host_config: &Account<'info, zama_host::HostConfig>,
    system_program: &Program<'info, System>,
    compute_signer_bump: u8,
    nonce_sequence: u64,
    plaintext: u64,
) -> Result<[u8; 32]> {
    fhe::trivial_encrypt_u64(fhe::TrivialEncryptU64 {
        payer,
        event_authority: zama_event_authority,
        zama_program,
        host_config,
        compute_signer,
        app_account_authority: token_account,
        output_acl_record: acl_record,
        acl_domain_key: mint.key(),
        compute_signer_bump,
        system_program,
        output_nonce_key: balance_nonce_key(mint.key(), token_account.key()),
        output_nonce_sequence: nonce_sequence,
        output_encrypted_value_label: balance_label(),
        plaintext,
        fhe_type: BALANCE_FHE_TYPE,
        output_subjects: balance_acl_subjects(token_account.owner, compute_signer.key()),
        output_public_decrypt: false,
    })
}

fn balance_acl_subjects(owner: Pubkey, compute_signer: Pubkey) -> Vec<AclSubjectEntry> {
    vec![
        AclSubjectEntry::user(owner),
        AclSubjectEntry::compute(compute_signer),
    ]
}

fn compute_acl_subject(compute_signer: Pubkey) -> Vec<AclSubjectEntry> {
    vec![AclSubjectEntry::compute(compute_signer)]
}

fn transferred_amount_acl_subjects(
    from_owner: Pubkey,
    to_owner: Pubkey,
    compute_signer: Pubkey,
) -> Vec<AclSubjectEntry> {
    let mut subjects = vec![AclSubjectEntry::user(from_owner)];
    if to_owner != from_owner {
        subjects.push(AclSubjectEntry::user(to_owner));
    }
    subjects.push(AclSubjectEntry::compute(compute_signer));
    subjects
}

fn burned_amount_acl_subjects(owner: Pubkey, compute_signer: Pubkey) -> Vec<AclSubjectEntry> {
    balance_acl_subjects(owner, compute_signer)
}

/// Returns the compute signer PDA for a confidential mint.
pub fn compute_signer_address(mint: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"fhe-compute", mint.as_ref()], &crate::ID)
}

/// Returns the mint-scoped app authority PDA for encrypted total supply.
pub fn total_supply_authority_address(mint: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"total-supply", mint.as_ref()], &crate::ID)
}

/// Returns the canonical confidential token account PDA for one owner and mint.
pub fn token_account_address(mint: Pubkey, owner: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"token-account", mint.as_ref(), owner.as_ref()],
        &crate::ID,
    )
}

/// Returns the PDA that owns the confidential mint's underlying-token vault.
pub fn vault_authority_address(mint: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"vault-authority", mint.as_ref()], &crate::ID)
}

/// Returns the canonical SPL token account used as the confidential mint's vault.
pub fn vault_token_account_address(mint: Pubkey, underlying_mint: Pubkey) -> Pubkey {
    get_associated_token_address_with_program_id(
        &vault_authority_address(mint).0,
        &underlying_mint,
        &spl_token::ID,
    )
}

/// Returns the operator authorization PDA for one token account and operator.
pub fn operator_record_address(token_account: Pubkey, operator: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"operator", token_account.as_ref(), operator.as_ref()],
        &crate::ID,
    )
}

/// Returns the replay-marker PDA for a redeemed burned amount handle.
pub fn burn_redemption_address(mint: Pubkey, burned_handle: [u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"burn-redemption", mint.as_ref(), burned_handle.as_ref()],
        &crate::ID,
    )
}

/// Returns the replay-marker PDA for a transfer callback settlement.
pub fn transfer_callback_settlement_address(mint: Pubkey, sent_handle: [u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"transfer-callback", mint.as_ref(), sent_handle.as_ref()],
        &crate::ID,
    )
}

/// Returns the one-shot marker PDA for a transfer receiver hook call.
pub fn transfer_receiver_hook_address(mint: Pubkey, sent_handle: [u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"transfer-hook", mint.as_ref(), sent_handle.as_ref()],
        &crate::ID,
    )
}

/// Returns the ZamaHost nonce key for a token balance field.
pub fn balance_nonce_key(acl_domain_key: Pubkey, app_account: Pubkey) -> [u8; 32] {
    nonce_key(acl_domain_key, app_account, balance_label())
}

/// Returns the ZamaHost nonce key for the encrypted total supply field.
pub fn total_supply_nonce_key(acl_domain_key: Pubkey, app_account: Pubkey) -> [u8; 32] {
    nonce_key(acl_domain_key, app_account, total_supply_label())
}

/// Fixed encrypted value label for confidential balances.
pub fn balance_label() -> [u8; 32] {
    *b"balance_________________________"
}

/// Fixed encrypted value label for the encrypted total supply.
pub fn total_supply_label() -> [u8; 32] {
    *b"total_supply____________________"
}

/// Fixed encrypted value label for public wrap amounts.
pub fn wrap_amount_label() -> [u8; 32] {
    *b"wrap_amount_____________________"
}

/// Fixed encrypted value label for externally verified burn amounts.
pub fn burn_amount_label() -> [u8; 32] {
    *b"burn_amount_____________________"
}

/// Fixed encrypted value label for externally verified transfer amounts.
pub fn transfer_amount_label() -> [u8; 32] {
    *b"transfer_amount_________________"
}

/// Fixed encrypted value label for burn success bits.
pub fn burn_success_label() -> [u8; 32] {
    *b"burn_success____________________"
}

/// Fixed encrypted value label for transfer success bits.
pub fn transfer_success_label() -> [u8; 32] {
    *b"transfer_success________________"
}

/// Fixed encrypted value label for unchecked burn debit candidates.
pub fn burn_debit_candidate_label() -> [u8; 32] {
    *b"burn_debit_candidate____________"
}

/// Fixed encrypted value label for unchecked debit candidates.
pub fn debit_candidate_label() -> [u8; 32] {
    *b"debit_candidate_________________"
}

/// Fixed encrypted value label for the all-or-zero burned amount.
pub fn burned_amount_label() -> [u8; 32] {
    *b"burned_amount___________________"
}

/// Fixed encrypted value label for the all-or-zero transferred amount.
pub fn transferred_amount_label() -> [u8; 32] {
    *b"transferred_amount______________"
}

/// Fixed encrypted value label for receiver callback success bits.
pub fn callback_success_label() -> [u8; 32] {
    *b"callback_success________________"
}

/// Fixed encrypted value label for callback-settlement zero constants.
pub fn callback_zero_label() -> [u8; 32] {
    *b"callback_zero___________________"
}

/// Fixed encrypted value label for callback-requested refunds.
pub fn callback_refund_request_label() -> [u8; 32] {
    *b"callback_refund_request_________"
}

/// Fixed encrypted value label for callback refund balance checks.
pub fn callback_refund_success_label() -> [u8; 32] {
    *b"callback_refund_success_________"
}

/// Fixed encrypted value label for callback refund debit candidates.
pub fn callback_refund_debit_candidate_label() -> [u8; 32] {
    *b"callback_refund_debit_candidate_"
}

/// Fixed encrypted value label for callback actual refunds.
pub fn callback_refund_amount_label() -> [u8; 32] {
    *b"callback_refund_amount__________"
}

/// Fixed encrypted value label for final transfer amounts after callback refunds.
pub fn callback_final_transferred_label() -> [u8; 32] {
    *b"callback_final_transferred______"
}

/// Delegates nonce-key derivation to ZamaHost so app and host agree exactly.
pub fn nonce_key(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
) -> [u8; 32] {
    zama_host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_receiver_return() -> TransferReceiverReturn {
        TransferReceiverReturn {
            mint: Pubkey::new_unique(),
            from_token_account: Pubkey::new_unique(),
            to_token_account: Pubkey::new_unique(),
            sent_handle: [1; 32],
            sent_acl_record: Pubkey::new_unique(),
            callback_success_handle: [2; 32],
            callback_success_acl_record: Pubkey::new_unique(),
        }
    }

    #[test]
    fn transfer_receiver_return_round_trips() {
        let payload = sample_receiver_return();
        let encoded = payload.encode();

        assert_eq!(encoded.len(), TRANSFER_RECEIVER_RETURN_LEN);
        assert_eq!(encoded.len(), TransferReceiverReturn::LEN);
        assert_eq!(TransferReceiverReturn::decode(&encoded).unwrap(), payload);
    }

    #[test]
    fn transfer_receiver_return_compatibility_encoder_matches_struct_encoder() {
        let payload = sample_receiver_return();

        assert_eq!(
            transfer_receiver_return_data(
                payload.mint,
                payload.from_token_account,
                payload.to_token_account,
                payload.sent_handle,
                payload.sent_acl_record,
                payload.callback_success_handle,
                payload.callback_success_acl_record,
            ),
            payload.encode()
        );
    }

    #[test]
    fn transfer_receiver_return_rejects_wrong_magic_or_length() {
        let mut encoded = sample_receiver_return().encode();
        encoded[0] ^= 0xff;
        assert!(TransferReceiverReturn::decode(&encoded).is_err());

        let mut truncated = sample_receiver_return().encode();
        truncated.pop();
        assert!(TransferReceiverReturn::decode(&truncated).is_err());
    }
}
