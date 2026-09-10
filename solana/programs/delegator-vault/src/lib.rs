//! Test-only program-controlled delegator: a vault-like PDA that grants and revokes
//! user-decryption delegations via CPI.
//!
//! The smallest shape of the multisig model the delegation design targets: the delegator is not
//! a wallet but a PDA of another program, and the host's `delegator: Signer` requirement is
//! satisfied by `invoke_signed` — exactly what a Squads vault does when a proposal executes.
//! This program exists for the Mollusk CPI cases in
//! `runtime-tests/tests/user_decryption_delegation_mollusk.rs` and is deployed nowhere.
//!
//! The CPI is assembled by hand — accounts struct, `Instruction`, `invoke_signed` — following
//! `zama-fhe/src/cpi.rs`, the pattern the production consumers use.

// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]

use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed},
    InstructionData, ToAccountInfos,
};
use zama_host::program::ZamaHost;

declare_id!("ANFJWue3SKz6F7chTeeLfbZxqEwaGdhHb7muY6pkvFa3");

/// Seed of the vault PDA that acts as the delegator.
pub const VAULT_SEED: &[u8] = b"vault";

/// The vault PDA an executor acts through.
pub fn vault_address(executor: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[VAULT_SEED, executor.as_ref()], &crate::ID)
}

#[program]
pub mod delegator_vault {
    use super::*;

    /// Exercises the host's rejection of nested scratch closure. Test-only; deployed nowhere.
    pub fn close_scratch_via_cpi<'info>(
        ctx: Context<'info, CloseScratchViaCpi<'info>>,
    ) -> Result<()> {
        zama_host::cpi::close_scratch(
            CpiContext::new(
                ctx.accounts.zama_host.key(),
                zama_host::cpi::accounts::CloseScratch {
                    instructions: ctx.accounts.instructions.to_account_info(),
                    scratch: ctx.accounts.scratch.to_account_info(),
                    refund: ctx.accounts.refund.to_account_info(),
                },
            )
            .with_remaining_accounts(ctx.remaining_accounts.to_vec()),
        )
    }

    /// Checks return data immediately after CPI, before the transaction's final
    /// scratch close overwrites the runtime return channel.
    pub fn check_cpi_return<'info>(
        ctx: Context<'info, CheckCpiReturn<'info>>,
        instruction_data: Vec<u8>,
        expected: Vec<u8>,
    ) -> Result<()> {
        let instruction = Instruction {
            program_id: ctx.accounts.callee.key(),
            accounts: ctx
                .remaining_accounts
                .iter()
                .map(|info| {
                    if info.is_writable {
                        AccountMeta::new(info.key(), info.is_signer)
                    } else {
                        AccountMeta::new_readonly(info.key(), info.is_signer)
                    }
                })
                .collect(),
            data: instruction_data,
        };
        anchor_lang::solana_program::program::invoke(&instruction, ctx.remaining_accounts)?;
        match anchor_lang::solana_program::program::get_return_data() {
            Some((program, returned)) => {
                require_keys_eq!(program, ctx.accounts.callee.key());
                require!(
                    returned == expected,
                    anchor_lang::error::ErrorCode::RequireEqViolated
                );
            }
            None => require!(
                expected.is_empty(),
                anchor_lang::error::ErrorCode::RequireEqViolated
            ),
        }
        Ok(())
    }

    /// Grants a user-decryption delegation with the executor's vault PDA as the delegator.
    pub fn grant_via_vault(
        ctx: Context<VaultDelegation>,
        delegate: Pubkey,
        authority: Pubkey,
        expiration_slot: u64,
    ) -> Result<()> {
        let cpi_accounts = zama_host::cpi::accounts::DelegateForUserDecryption {
            payer: ctx.accounts.executor.to_account_info(),
            delegator: ctx.accounts.vault.to_account_info(),
            host_config: ctx.accounts.host_config.to_account_info(),
            delegation_record: ctx.accounts.delegation_record.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let instruction = Instruction {
            program_id: ctx.accounts.zama_host.key(),
            accounts: cpi_accounts.to_account_metas(None),
            data: zama_host::instruction::DelegateForUserDecryption {
                delegate,
                authority,
                expiration_slot,
            }
            .data(),
        };
        // The callee's own program account rides along: `invoke_signed` resolves the target
        // through the caller's account infos, not only through the instruction's metas.
        let mut infos = cpi_accounts.to_account_infos();
        infos.push(ctx.accounts.zama_host.to_account_info());
        let executor = ctx.accounts.executor.key();
        let bump = [ctx.bumps.vault];
        let seeds: &[&[u8]] = &[VAULT_SEED, executor.as_ref(), &bump];
        invoke_signed(&instruction, &infos, &[seeds])?;
        Ok(())
    }

    /// Revokes a delegation the executor's vault PDA granted.
    pub fn revoke_via_vault(ctx: Context<VaultDelegation>) -> Result<()> {
        let cpi_accounts = zama_host::cpi::accounts::RevokeDelegationForUserDecryption {
            delegator: ctx.accounts.vault.to_account_info(),
            host_config: ctx.accounts.host_config.to_account_info(),
            delegation_record: ctx.accounts.delegation_record.to_account_info(),
        };
        let instruction = Instruction {
            program_id: ctx.accounts.zama_host.key(),
            accounts: cpi_accounts.to_account_metas(None),
            data: zama_host::instruction::RevokeDelegationForUserDecryption {}.data(),
        };
        // See the grant: the callee's program account has to be among the infos.
        let mut infos = cpi_accounts.to_account_infos();
        infos.push(ctx.accounts.zama_host.to_account_info());
        let executor = ctx.accounts.executor.key();
        let bump = [ctx.bumps.vault];
        let seeds: &[&[u8]] = &[VAULT_SEED, executor.as_ref(), &bump];
        invoke_signed(&instruction, &infos, &[seeds])?;
        Ok(())
    }
}

/// One account set serves both instructions; the revoke simply ignores the system program.
#[derive(Accounts)]
pub struct VaultDelegation<'info> {
    /// Executes the "proposal" and pays rent on a grant.
    #[account(mut)]
    pub executor: Signer<'info>,
    /// CHECK: the executor's own vault PDA, held to its seeds here; it signs the CPI, which is
    /// the whole point — the host sees a delegator that is not a wallet.
    #[account(seeds = [VAULT_SEED, executor.key().as_ref()], bump)]
    pub vault: UncheckedAccount<'info>,
    /// CHECK: validated by the host program against its own seeds.
    pub host_config: UncheckedAccount<'info>,
    /// CHECK: validated by the host program against the canonical delegation PDA.
    #[account(mut)]
    pub delegation_record: UncheckedAccount<'info>,
    pub zama_host: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}

/// Accounts forwarded to the host in the nested-close negative test.
#[derive(Accounts)]
pub struct CloseScratchViaCpi<'info> {
    /// CHECK: test forwards the scratch to ZamaHost.
    #[account(mut)]
    pub scratch: UncheckedAccount<'info>,
    /// CHECK: ZamaHost checks the recorded refund destination.
    #[account(mut)]
    pub refund: UncheckedAccount<'info>,
    /// CHECK: validated by the host; this proxy deliberately adds no authorization.
    pub instructions: UncheckedAccount<'info>,
    pub zama_host: Program<'info, ZamaHost>,
}

/// The remaining accounts are the exact callee account list.
#[derive(Accounts)]
pub struct CheckCpiReturn<'info> {
    /// CHECK: test probe intentionally invokes the supplied executable.
    #[account(executable)]
    pub callee: UncheckedAccount<'info>,
}
