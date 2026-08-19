//! Specimen `zama-host` consumer: one encrypted u64 whose updates are LONG dependent step chains.
//!
//! Where `encrypted-counter` is the smallest complete consumer, this program is the load-smoke
//! shape — the Solana analog of the EVM suite's `SlowLaneContention.sol`. Every `extend` packs a
//! dependent chain into ONE `fhe_execute`, each step's operand being the previous step's transient
//! result, so the coprocessor cannot parallelize any of it: the whole chain sits in the slow lane
//! and must be computed strictly in order before the tail handle becomes decryptable. Its Mollusk
//! test (`runtime-tests/tests/dep_chain_mollusk.rs`) proves the kit's cleartext oracle replays a
//! chain at this program's full depth; the live load-smoke scenario drives the same dependent-step
//! shape through the typed `fhe_execute` client against the running coprocessor.
//!
//! This program's depth is the host's one step ceiling, `MAX_FHE_EXECUTION_STEPS` (32), which
//! makes it the at-cap specimen: a CPI-composing program builds its execution on the SBF
//! entrypoint's fixed 32 KB bump heap (DD-046: the heap is fixed and cannot be raised), and its
//! Mollusk test extending at full depth is what proves a maximum execution — build, packet,
//! account resolution and Anchor's own deserialization — actually fits that heap under SBF, on
//! top of `zama-fhe`'s host-side byte count in `heap_budget.rs`.
//!
//! Executions assume `grant_deny_list_enabled = false` and no binding HCU cap: `hcu_block_meter`
//! and `hcu_trusted_app_record` are hardcoded `None` (the PoC host fixtures never enable them),
//! and deny-list records ride in as the (empty) remaining accounts.

// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]

/// Program-specific error codes.
pub mod errors;
/// Chain state account and PDA/label derivations.
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

declare_id!("7Sz3qA6Wm84uWjfiFJR4ww1Trx7DZ4MMb4iB4sn5vqaV");

/// The deepest chain one `extend` can carry: the host's `MAX_FHE_EXECUTION_STEPS`, the one step
/// ceiling for executions built on-chain and off (see the module docs — extending at this depth
/// is what verifies the maximum execution fits the fixed program heap).
pub const MAX_CHAIN_LINKS: u8 = 32;

/// `extend` promises exactly the host's ceiling; if the host's cap moves, this program's
/// contract (and its error message) must move with it consciously.
const _: () = assert!(MAX_CHAIN_LINKS as usize == zama_host::MAX_FHE_EXECUTION_STEPS);

#[program]
pub mod dep_chain {
    use super::*;

    /// Creates the chain and its encrypted tail value at zero.
    pub fn initialize<'info>(ctx: Context<'info, Initialize<'info>>) -> Result<()> {
        let chain = ctx.accounts.chain.key();
        ctx.accounts.chain.set_inner(Chain {
            bump: ctx.bumps.chain,
            authority_bump: ctx.bumps.chain_authority,
        });
        // The owner user-decrypts the tail; the chain authority re-reads it as the next
        // extension's first operand.
        let output = PersistentOutput::create(
            tail_encrypted_value_id(chain),
            vec![ctx.accounts.owner.key(), ctx.accounts.chain_authority.key()],
        );
        let execution = FheExecution::build(
            ExecutionEncryptedValueAccountAuthority::new(ctx.accounts.chain_authority.key()),
            |builder| {
                builder.trivial_encrypt_u64(0, Output::persistent(output))?;
                Ok(())
            },
        )
        .map_err(invalid_execution)?;
        let resolved = execution
            .resolve_accounts(
                [ctx.accounts.tail_value.to_account_info()],
                [ctx.accounts.chain_authority.to_account_info()],
            )
            .map_err(invalid_execution_accounts)?;
        let bump = [ctx.bumps.chain_authority];
        let authority_seeds: &[&[u8]] = &[CHAIN_AUTHORITY_SEED, chain.as_ref(), &bump];
        execution.invoke(
            ExecutionCpiAccounts {
                payer: ctx.accounts.owner.to_account_info(),
                compute_subject: ctx.accounts.chain_authority.to_account_info(),
                encrypted_value_account_authority: ctx.accounts.chain_authority.to_account_info(),
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

    /// Adds `amount` to the tail `links` times as one execution of `links` DEPENDENT steps:
    /// every add reads the previous add's transient result, and only the last one persists.
    /// The tail therefore grows by `links * amount`, but unlike `links` separate executions
    /// the coprocessor has to schedule every step after the one before it.
    pub fn extend<'info>(ctx: Context<'info, Extend<'info>>, links: u8, amount: u64) -> Result<()> {
        require!(
            (1..=MAX_CHAIN_LINKS).contains(&links),
            DepChainError::InvalidChainLength
        );
        let chain = ctx.accounts.chain.key();
        let tail_value = &ctx.accounts.tail_value;
        // Operand from the encrypted value account's own canonical fields, so the operand slot
        // always matches the account the host re-validates.
        let operand = Uint64Handle::persistent(
            tail_value.current_handle,
            EncryptedValueId::new(
                Domain::new(tail_value.domain),
                tail_value.encrypted_value_account_authority,
                EncryptedValueLabel::new(tail_value.label),
            ),
        )
        .map_err(invalid_execution)?;
        // Same audience as the create: an update replaces the stored subjects wholesale.
        let output = PersistentOutput::update(
            tail_encrypted_value_id(chain),
            vec![ctx.accounts.owner.key(), ctx.accounts.chain_authority.key()],
            tail_value,
        );
        let execution = FheExecution::build(
            ExecutionEncryptedValueAccountAuthority::new(ctx.accounts.chain_authority.key()),
            |builder| {
                if links == 1 {
                    builder.add(
                        operand,
                        Scalar::<Uint<64>>::u64(amount),
                        Output::persistent(output),
                    )?;
                    return Ok(());
                }
                let mut value = builder.add(
                    operand,
                    Scalar::<Uint<64>>::u64(amount),
                    Output::transient(),
                )?;
                for _ in 2..links {
                    value =
                        builder.add(value, Scalar::<Uint<64>>::u64(amount), Output::transient())?;
                }
                builder.add(
                    value,
                    Scalar::<Uint<64>>::u64(amount),
                    Output::persistent(output),
                )?;
                Ok(())
            },
        )
        .map_err(invalid_execution)?;
        let resolved = execution
            .resolve_accounts(
                [ctx.accounts.tail_value.to_account_info()],
                [ctx.accounts.chain_authority.to_account_info()],
            )
            .map_err(invalid_execution_accounts)?;
        let bump = [ctx.accounts.chain.authority_bump];
        let authority_seeds: &[&[u8]] = &[CHAIN_AUTHORITY_SEED, chain.as_ref(), &bump];
        execution.invoke(
            ExecutionCpiAccounts {
                payer: ctx.accounts.owner.to_account_info(),
                compute_subject: ctx.accounts.chain_authority.to_account_info(),
                encrypted_value_account_authority: ctx.accounts.chain_authority.to_account_info(),
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
    error!(DepChainError::InvalidFheExecution)
}

fn invalid_execution_accounts(
    error: zama_fhe::ExecutionAccountResolutionError,
) -> anchor_lang::error::Error {
    msg!("invalid dep-chain fhe_execute accounts: {:?}", error);
    error!(DepChainError::InvalidFheExecution)
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + Chain::SPACE,
        seeds = [CHAIN_SEED, owner.key().as_ref()],
        bump,
    )]
    pub chain: Account<'info, Chain>,
    /// CHECK: PDA signing the host CPI as compute subject and encrypted-value authority.
    #[account(seeds = [CHAIN_AUTHORITY_SEED, chain.key().as_ref()], bump)]
    pub chain_authority: UncheckedAccount<'info>,
    /// CHECK: created by the host CPI at the chain's canonical encrypted-value address.
    #[account(mut, address = tail_encrypted_value_id(chain.key()).address() @ DepChainError::TailValueInvalid)]
    pub tail_value: UncheckedAccount<'info>,
    /// CHECK: ZamaHost config PDA; validated by the host program.
    pub host_config: UncheckedAccount<'info>,
    /// CHECK: ZamaHost event-CPI authority; validated by the host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Extend<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(seeds = [CHAIN_SEED, owner.key().as_ref()], bump = chain.bump)]
    pub chain: Account<'info, Chain>,
    /// CHECK: PDA signing the host CPI as compute subject and encrypted-value authority.
    #[account(seeds = [CHAIN_AUTHORITY_SEED, chain.key().as_ref()], bump = chain.authority_bump)]
    pub chain_authority: UncheckedAccount<'info>,
    /// Stable tail encrypted value account; read for the current handle and replaced by this
    /// execution.
    #[account(mut, address = tail_encrypted_value_id(chain.key()).address() @ DepChainError::TailValueInvalid)]
    pub tail_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// CHECK: ZamaHost config PDA; validated by the host program.
    pub host_config: UncheckedAccount<'info>,
    /// CHECK: ZamaHost event-CPI authority; validated by the host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}
