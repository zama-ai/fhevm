//! Reference **app-driven** deposit vault for the Solana FHEVM PoC.
//!
//! This program demonstrates the Solana-native answer to EVM `confidentialTransferAndCall`
//! (issue #1593): instead of the token calling the app back (the ported 4-leg callback flow this
//! PoC removed), the *receiving app* drives the whole thing with a single `deposit` instruction.
//!
//! The depositor signs **once**. That signature propagates through the CPI to
//! `confidential_token::confidential_transfer`, which moves the depositor's encrypted amount into
//! the vault's confidential token account atomically. Because a confidential transfer is all-or-zero
//! (insufficient balance ⇒ transfers 0), there is no success-bit, no refund leg, and no standing
//! operator/approval — the transfer itself is the accept signal.
//!
//! The amount is a coprocessor-attested external input (fromExternal): the depositor builds it bound
//! to `(user = depositor, contract = the mint's compute-signer PDA)`, exactly as for a direct
//! transfer; this app is a pure composition layer and just forwards it.

// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]
#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

use anchor_lang::prelude::*;
use confidential_token::cpi::accounts::ConfidentialTransfer as CtConfidentialTransfer;
use confidential_token::program::ConfidentialToken;
use zama_host::{program::ZamaHost, CoprocessorInputAttestation};

declare_id!("8JdZ2wLbRJtc969bcY6rqSRUKFovxHeKhWPmQaLhHojd");

#[program]
pub mod confidential_deposit_app {
    use super::*;

    /// Creates the vault record for a confidential token account. The vault's token account is a
    /// normal `confidential_token` account whose owner is `vault_authority` (a caller-chosen PDA);
    /// receiving transfers never requires the recipient to sign, so the vault authority is only a
    /// bookkeeping identity here.
    pub fn initialize_vault(ctx: Context<InitializeVault>, vault_authority: Pubkey) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.vault_authority = vault_authority;
        vault.vault_token_account = ctx.accounts.vault_token_account.key();
        vault.deposit_count = 0;
        Ok(())
    }

    /// Deposits a coprocessor-attested encrypted amount from the depositor into the vault by
    /// composing `confidential_token::confidential_transfer` via CPI. One user signature; no
    /// operator, no callback, no refund leg.
    pub fn deposit(
        ctx: Context<Deposit>,
        amount_attestation: CoprocessorInputAttestation,
    ) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.vault_token_account.key(),
            ctx.accounts.vault.vault_token_account,
            DepositError::VaultTokenAccountMismatch
        );

        // The depositor's signature on this instruction propagates through the CPI as the transfer
        // authority (`owner`) — no `invoke_signed`, no operator authority. This is the crux of the
        // app-driven pattern: authority flows down the call stack, so the app never needs the token
        // to call it back.
        let cpi_accounts = CtConfidentialTransfer {
            owner: ctx.accounts.depositor.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            from_account: ctx.accounts.depositor_token_account.to_account_info(),
            to_account: ctx.accounts.vault_token_account.to_account_info(),
            compute_signer: ctx.accounts.compute_signer.to_account_info(),
            from_current_compute_acl: ctx.accounts.from_current_compute_acl.to_account_info(),
            to_current_compute_acl: ctx.accounts.to_current_compute_acl.to_account_info(),
            from_output_acl: ctx.accounts.from_output_acl.to_account_info(),
            transferred_amount_acl: ctx.accounts.transferred_amount_acl.to_account_info(),
            to_output_acl: ctx.accounts.to_output_acl.to_account_info(),
            zama_event_authority: ctx.accounts.zama_event_authority.to_account_info(),
            zama_program: ctx.accounts.zama_program.to_account_info(),
            host_config: ctx.accounts.host_config.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            event_authority: ctx
                .accounts
                .confidential_token_event_authority
                .to_account_info(),
            program: ctx.accounts.confidential_token_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.confidential_token_program.key(), cpi_accounts);
        confidential_token::cpi::confidential_transfer(cpi_ctx, amount_attestation)?;

        // App-side accounting. The deposited amount is confidential — the vault only ever sees its
        // own rotated encrypted balance — so this reference records deposit provenance and a count
        // rather than a plaintext share. A production vault would rotate a per-depositor *encrypted*
        // position here via its own `fhe_eval` (the same primitives the token uses), still with no
        // callback.
        let vault = &mut ctx.accounts.vault;
        vault.deposit_count = vault
            .deposit_count
            .checked_add(1)
            .ok_or(DepositError::DepositCountOverflow)?;

        emit!(DepositMade {
            vault: vault.key(),
            depositor: ctx.accounts.depositor.key(),
            mint: ctx.accounts.mint.key(),
            deposit_index: vault.deposit_count,
        });
        Ok(())
    }
}

/// Vault bookkeeping record for one confidential token account.
#[account]
#[derive(InitSpace)]
pub struct Vault {
    /// Bookkeeping authority that owns the vault's confidential token account.
    pub vault_authority: Pubkey,
    /// The confidential token account deposits are routed into.
    pub vault_token_account: Pubkey,
    /// Number of deposits routed through this vault.
    pub deposit_count: u64,
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    /// Pays for the vault record and signs its creation.
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: The vault's confidential token account; validated by `confidential_token` on deposit.
    pub vault_token_account: UncheckedAccount<'info>,
    /// The vault record, keyed by its confidential token account.
    #[account(
        init,
        payer = authority,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", vault_token_account.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

/// Mirrors `confidential_token::ConfidentialTransfer` (from → to) plus the vault record. Writability
/// mirrors the token instruction so the CPI's writable metas are honored at the outer level.
#[derive(Accounts)]
pub struct Deposit<'info> {
    /// Depositor and transfer authority; signs once for the whole composed deposit.
    pub depositor: Signer<'info>,
    /// Pays rent for the transfer's output ACL records.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Confidential mint; validated by `confidential_token`.
    pub mint: UncheckedAccount<'info>,
    /// CHECK: Depositor's confidential token account (transfer source); validated by the token CPI.
    #[account(mut)]
    pub depositor_token_account: UncheckedAccount<'info>,
    /// CHECK: Vault's confidential token account (transfer destination); validated by the token CPI
    /// and pinned to the vault record.
    #[account(mut)]
    pub vault_token_account: UncheckedAccount<'info>,
    /// CHECK: Mint compute-signer PDA; validated by `confidential_token`.
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: Depositor current balance ACL record; validated by the token CPI.
    pub from_current_compute_acl: UncheckedAccount<'info>,
    /// CHECK: Vault current balance ACL record; validated by the token CPI.
    pub to_current_compute_acl: UncheckedAccount<'info>,
    /// CHECK: New depositor balance ACL record (host-initialized in the CPI).
    #[account(mut)]
    pub from_output_acl: UncheckedAccount<'info>,
    /// CHECK: Transferred-amount ACL record (host-initialized in the CPI).
    #[account(mut)]
    pub transferred_amount_acl: UncheckedAccount<'info>,
    /// CHECK: New vault balance ACL record (host-initialized in the CPI).
    #[account(mut)]
    pub to_output_acl: UncheckedAccount<'info>,
    /// CHECK: ZamaHost event-CPI authority; validated by the host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program (FHE compute + ACL).
    pub zama_program: Program<'info, ZamaHost>,
    /// CHECK: ZamaHost config PDA; validated by the host program.
    pub host_config: UncheckedAccount<'info>,
    /// CHECK: confidential-token event-CPI authority; validated by the token program.
    pub confidential_token_event_authority: UncheckedAccount<'info>,
    /// confidential-token program composed via CPI.
    pub confidential_token_program: Program<'info, ConfidentialToken>,
    /// The vault record; deposit count is bumped on success.
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct DepositMade {
    /// Vault record the deposit was routed through.
    pub vault: Pubkey,
    /// Depositor (transfer authority).
    pub depositor: Pubkey,
    /// Confidential mint.
    pub mint: Pubkey,
    /// 1-based index of this deposit within the vault.
    pub deposit_index: u64,
}

#[error_code]
pub enum DepositError {
    #[msg("vault token account does not match the vault record")]
    VaultTokenAccountMismatch,
    #[msg("vault deposit count overflowed")]
    DepositCountOverflow,
}
