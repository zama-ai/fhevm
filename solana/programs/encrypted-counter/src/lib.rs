//! Specimen `zama-host` consumer: one encrypted u64 counter per owner.
//!
//! This program exists to be copied. It is the smallest complete shape of an app computing over
//! encrypted state through the host — one state account, one PDA acting as both the execution's
//! `compute_subject` and its `encrypted_value_account_authority` (the batcher's single-identity
//! pattern), and two instructions covering the create and the update form of a persistent
//! output. Its Mollusk test (`runtime-tests/tests/counter_mollusk.rs`) is the matching specimen
//! for testing a consumer with `zama-solana-test-kit`.
//!
//! Public API surface: off-chain callers deriving the counter PDAs — `runtime-tests`'
//! `counter_mollusk` fixtures.

// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use errors::CounterError;
use zama_fhe::{
    Domain, EncryptedValueId, EncryptedValueLabel, ExecutionCpiAccounts,
    ExecutionEncryptedValueAccountAuthority, FheExecution, Output, PersistentOutput, Scalar, Uint,
    Uint64Handle,
};
use zama_host::program::ZamaHost;

/// Program-specific error codes.
pub mod errors;

declare_id!("4tjvykteKMKDVmRGbfVSkANmBnm91NfD6DAaw6pykfF3");

pub const COUNTER_SEED: &[u8] = b"counter";
pub const COUNTER_AUTHORITY_SEED: &[u8] = b"counter-authority";
/// Label of the counter's single encrypted value, zero-padded like every host label.
pub const COUNT_LABEL: [u8; 32] = *b"count\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

pub fn counter_address(owner: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[COUNTER_SEED, owner.as_ref()], &id())
}

pub fn counter_authority_address(counter: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[COUNTER_AUTHORITY_SEED, counter.as_ref()], &id())
}

/// The counter's encrypted value: domain is the counter account, authority its PDA.
pub fn count_encrypted_value_id(counter: Pubkey) -> EncryptedValueId {
    EncryptedValueId::new(
        Domain::new(counter),
        counter_authority_address(counter).0,
        EncryptedValueLabel::new(COUNT_LABEL),
    )
}

#[program]
pub mod encrypted_counter {
    use super::*;

    /// Creates the counter and its encrypted value at zero.
    pub fn initialize<'info>(ctx: Context<'info, Initialize<'info>>) -> Result<()> {
        let counter = ctx.accounts.counter.key();
        let output = PersistentOutput::create(
            count_encrypted_value_id(counter),
            vec![ctx.accounts.counter_authority.key()],
        );
        ctx.accounts.counter.set_inner(Counter {
            owner: ctx.accounts.owner.key(),
            count_value: ctx.accounts.count_value.key(),
            authority_bump: ctx.bumps.counter_authority,
        });
        execute_as_counter_authority(
            CounterAuthorityExecute {
                counter,
                authority_bump: ctx.bumps.counter_authority,
                counter_authority: ctx.accounts.counter_authority.to_account_info(),
                count_value: ctx.accounts.count_value.to_account_info(),
                payer: ctx.accounts.owner.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                zama_event_authority: ctx.accounts.zama_event_authority.to_account_info(),
                zama_program: ctx.accounts.zama_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                deny_subject_records: ctx.remaining_accounts,
            },
            |builder| {
                builder.trivial_encrypt_u64(0, Output::persistent(output))?;
                Ok(())
            },
        )
    }

    /// Adds a plaintext amount to the encrypted count.
    pub fn increment<'info>(ctx: Context<'info, Increment<'info>>, amount: u64) -> Result<()> {
        let counter = ctx.accounts.counter.key();
        let key = count_encrypted_value_id(counter);
        let current = read_encrypted_value(&ctx.accounts.count_value)?;
        let operand = Uint64Handle::persistent(current.current_handle, key.clone())
            .map_err(invalid_execution)?;
        let output =
            PersistentOutput::update(key, vec![ctx.accounts.counter_authority.key()], &current);
        execute_as_counter_authority(
            CounterAuthorityExecute {
                counter,
                authority_bump: ctx.accounts.counter.authority_bump,
                counter_authority: ctx.accounts.counter_authority.to_account_info(),
                count_value: ctx.accounts.count_value.to_account_info(),
                payer: ctx.accounts.owner.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                zama_event_authority: ctx.accounts.zama_event_authority.to_account_info(),
                zama_program: ctx.accounts.zama_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                deny_subject_records: ctx.remaining_accounts,
            },
            |builder| {
                builder.add(
                    operand,
                    Scalar::<Uint<64>>::u64(amount),
                    Output::persistent(output),
                )?;
                Ok(())
            },
        )
    }
}

/// Fixed ZamaHost CPI accounts for an execution signed by the counter authority PDA.
struct CounterAuthorityExecute<'a, 'info> {
    counter: Pubkey,
    authority_bump: u8,
    counter_authority: AccountInfo<'info>,
    count_value: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    host_config: AccountInfo<'info>,
    zama_event_authority: AccountInfo<'info>,
    zama_program: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    deny_subject_records: &'a [AccountInfo<'info>],
}

/// Builds and invokes one `fhe_execute` with the counter authority PDA signing as both compute
/// subject and encrypted-value authority.
fn execute_as_counter_authority<'info>(
    accounts: CounterAuthorityExecute<'_, 'info>,
    build: impl for<'id> FnOnce(&mut zama_fhe::FheExecutionBuilder<'id>) -> zama_fhe::Result<()>,
) -> Result<()> {
    let execution = FheExecution::build(
        ExecutionEncryptedValueAccountAuthority::new(accounts.counter_authority.key()),
        build,
    )
    .map_err(invalid_execution)?;
    let resolved = execution
        .resolve_accounts([accounts.count_value], [accounts.counter_authority.clone()])
        .map_err(|error| {
            msg!("invalid counter fhe_execute accounts: {:?}", error);
            error!(CounterError::InvalidFheExecution)
        })?;
    let bump = [accounts.authority_bump];
    let authority_seeds: &[&[u8]] = &[COUNTER_AUTHORITY_SEED, accounts.counter.as_ref(), &bump];
    execution.invoke(
        ExecutionCpiAccounts {
            payer: accounts.payer,
            compute_subject: accounts.counter_authority.clone(),
            encrypted_value_account_authority: accounts.counter_authority,
            host_config: accounts.host_config,
            deny_subject_records: accounts.deny_subject_records,
            system_program: accounts.system_program,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: accounts.zama_event_authority,
            program: accounts.zama_program,
        },
        &resolved,
        &[authority_seeds],
    )
}

/// Decodes a canonical, host-owned `EncryptedValue` account.
fn read_encrypted_value(info: &AccountInfo) -> Result<zama_host::EncryptedValue> {
    require_keys_eq!(*info.owner, zama_host::ID, CounterError::CountValueInvalid);
    let data = info.try_borrow_data()?;
    let mut slice: &[u8] = &data;
    zama_host::EncryptedValue::try_deserialize(&mut slice)
        .map_err(|_| CounterError::CountValueInvalid.into())
}

fn invalid_execution(error: zama_fhe::FheExecutionBuildError) -> anchor_lang::error::Error {
    msg!("invalid FHE execution: {:?}", error);
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
    #[account(seeds = [COUNTER_SEED, owner.key().as_ref()], bump)]
    pub counter: Account<'info, Counter>,
    /// CHECK: PDA signing the host CPI as compute subject and encrypted-value authority.
    #[account(seeds = [COUNTER_AUTHORITY_SEED, counter.key().as_ref()], bump = counter.authority_bump)]
    pub counter_authority: UncheckedAccount<'info>,
    /// CHECK: updated by the host CPI; pinned to the address recorded at initialize.
    #[account(mut, address = counter.count_value @ CounterError::CountValueInvalid)]
    pub count_value: UncheckedAccount<'info>,
    /// CHECK: ZamaHost config PDA; validated by the host program.
    pub host_config: UncheckedAccount<'info>,
    /// CHECK: ZamaHost event-CPI authority; validated by the host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub owner: Pubkey,
    pub count_value: Pubkey,
    pub authority_bump: u8,
}

impl Counter {
    pub const SPACE: usize = 32 + 32 + 1;
}
