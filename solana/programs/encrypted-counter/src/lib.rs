//! Specimen `zama-host` consumer: one encrypted u64 counter per owner.
//!
//! This program exists to be copied. It is the smallest complete shape of an app computing over
//! encrypted state through the host — one state account, one PDA acting with one identity as
//! both the execution's `compute_subject` and its `encrypted_value_account_authority` (the
//! confidential-batcher pattern), and two instructions covering the create and the update form
//! of a persistent output. Its Mollusk test (`runtime-tests/tests/counter_mollusk.rs`) is the
//! matching specimen for testing a consumer with `zama-solana-test-kit`.
//!
//! Executions assume `grant_deny_list_enabled = false` and no binding HCU cap: `hcu_block_meter`
//! and `hcu_trusted_app_record` are hardcoded `None` (the PoC host fixtures never enable them),
//! and deny-list records ride in as the (empty) remaining accounts.

// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]

/// Program-specific error codes.
pub mod errors;
/// Counter state account and PDA/label derivations.
pub mod state;

// Re-export errors and state for generated clients and tests.
pub use errors::*;
pub use state::*;

use anchor_lang::prelude::*;
use zama_fhe::{
    Domain, EncryptedValueId, EncryptedValueLabel, ExecutionCpiAccounts,
    ExecutionEncryptedValueAccountAuthority, FheExecution, Output, PersistentOutput, Scalar, Uint,
    Uint64Handle,
};
use zama_host::program::ZamaHost;

declare_id!("4tjvykteKMKDVmRGbfVSkANmBnm91NfD6DAaw6pykfF3");

#[program]
pub mod encrypted_counter {
    use super::*;

    /// Creates the counter and its encrypted value at zero.
    pub fn initialize<'info>(ctx: Context<'info, Initialize<'info>>) -> Result<()> {
        let counter = ctx.accounts.counter.key();
        ctx.accounts.counter.set_inner(Counter {
            bump: ctx.bumps.counter,
            authority_bump: ctx.bumps.counter_authority,
        });
        // The owner user-decrypts their count; the counter authority re-reads it as the next
        // increment's operand.
        let output = PersistentOutput::create(
            count_encrypted_value_id(counter),
            vec![
                ctx.accounts.owner.key(),
                ctx.accounts.counter_authority.key(),
            ],
        );
        let execution = FheExecution::build(
            ExecutionEncryptedValueAccountAuthority::new(ctx.accounts.counter_authority.key()),
            |builder| {
                builder.trivial_encrypt_u64(0, Output::persistent(output))?;
                Ok(())
            },
        )
        .map_err(invalid_execution)?;
        let resolved = execution
            .resolve_accounts(
                [ctx.accounts.count_value.to_account_info()],
                [ctx.accounts.counter_authority.to_account_info()],
            )
            .map_err(invalid_execution_accounts)?;
        let bump = [ctx.bumps.counter_authority];
        let authority_seeds: &[&[u8]] = &[COUNTER_AUTHORITY_SEED, counter.as_ref(), &bump];
        execution.invoke(
            ExecutionCpiAccounts {
                payer: ctx.accounts.owner.to_account_info(),
                compute_subject: ctx.accounts.counter_authority.to_account_info(),
                encrypted_value_account_authority: ctx.accounts.counter_authority.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                deny_subject_records: ctx.remaining_accounts,
                system_program: ctx.accounts.system_program.to_account_info(),
                hcu_block_meter: None,
                hcu_trusted_app_record: None,
                event_authority: ctx.accounts.zama_event_authority.to_account_info(),
                program: ctx.accounts.zama_program.to_account_info(),
            },
            &resolved,
            &[authority_seeds],
        )
    }

    /// Adds a plaintext amount to the encrypted count.
    pub fn increment<'info>(ctx: Context<'info, Increment<'info>>, amount: u64) -> Result<()> {
        let counter = ctx.accounts.counter.key();
        let count_value = &ctx.accounts.count_value;
        // Operand from the encrypted value account's own canonical fields, so the operand slot
        // always matches the account the host re-validates.
        let operand = Uint64Handle::persistent(
            count_value.current_handle,
            EncryptedValueId::new(
                Domain::new(count_value.domain),
                count_value.encrypted_value_account_authority,
                EncryptedValueLabel::new(count_value.label),
            ),
        )
        .map_err(invalid_execution)?;
        // Same audience as the create: an update replaces the stored subjects wholesale.
        let output = PersistentOutput::update(
            count_encrypted_value_id(counter),
            vec![
                ctx.accounts.owner.key(),
                ctx.accounts.counter_authority.key(),
            ],
            count_value,
        );
        let execution = FheExecution::build(
            ExecutionEncryptedValueAccountAuthority::new(ctx.accounts.counter_authority.key()),
            |builder| {
                builder.add(
                    operand,
                    Scalar::<Uint<64>>::u64(amount),
                    Output::persistent(output),
                )?;
                Ok(())
            },
        )
        .map_err(invalid_execution)?;
        let resolved = execution
            .resolve_accounts(
                [ctx.accounts.count_value.to_account_info()],
                [ctx.accounts.counter_authority.to_account_info()],
            )
            .map_err(invalid_execution_accounts)?;
        let bump = [ctx.accounts.counter.authority_bump];
        let authority_seeds: &[&[u8]] = &[COUNTER_AUTHORITY_SEED, counter.as_ref(), &bump];
        execution.invoke(
            ExecutionCpiAccounts {
                payer: ctx.accounts.owner.to_account_info(),
                compute_subject: ctx.accounts.counter_authority.to_account_info(),
                encrypted_value_account_authority: ctx.accounts.counter_authority.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                deny_subject_records: ctx.remaining_accounts,
                system_program: ctx.accounts.system_program.to_account_info(),
                hcu_block_meter: None,
                hcu_trusted_app_record: None,
                event_authority: ctx.accounts.zama_event_authority.to_account_info(),
                program: ctx.accounts.zama_program.to_account_info(),
            },
            &resolved,
            &[authority_seeds],
        )
    }
}

fn invalid_execution(error: zama_fhe::FheExecutionBuildError) -> anchor_lang::error::Error {
    msg!("invalid FHE execution: {:?}", error);
    error!(CounterError::InvalidFheExecution)
}

fn invalid_execution_accounts(
    error: zama_fhe::ExecutionAccountResolutionError,
) -> anchor_lang::error::Error {
    msg!("invalid counter fhe_execute accounts: {:?}", error);
    error!(CounterError::InvalidFheExecution)
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + Counter::SPACE,
        seeds = [COUNTER_SEED, owner.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,
    /// CHECK: PDA signing the host CPI as compute subject and encrypted-value authority.
    #[account(seeds = [COUNTER_AUTHORITY_SEED, counter.key().as_ref()], bump)]
    pub counter_authority: UncheckedAccount<'info>,
    /// CHECK: created by the host CPI at the counter's canonical encrypted-value address.
    #[account(mut, address = count_encrypted_value_id(counter.key()).address() @ CounterError::CountValueInvalid)]
    pub count_value: UncheckedAccount<'info>,
    /// CHECK: ZamaHost config PDA; validated by the host program.
    pub host_config: UncheckedAccount<'info>,
    /// CHECK: ZamaHost event-CPI authority; validated by the host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(seeds = [COUNTER_SEED, owner.key().as_ref()], bump = counter.bump)]
    pub counter: Account<'info, Counter>,
    /// CHECK: PDA signing the host CPI as compute subject and encrypted-value authority.
    #[account(seeds = [COUNTER_AUTHORITY_SEED, counter.key().as_ref()], bump = counter.authority_bump)]
    pub counter_authority: UncheckedAccount<'info>,
    /// Stable count encrypted value account; read for the current handle and replaced by this
    /// execution.
    #[account(mut, address = count_encrypted_value_id(counter.key()).address() @ CounterError::CountValueInvalid)]
    pub count_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// CHECK: ZamaHost config PDA; validated by the host program.
    pub host_config: UncheckedAccount<'info>,
    /// CHECK: ZamaHost event-CPI authority; validated by the host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}
