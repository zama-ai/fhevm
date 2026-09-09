//! Minimal host consumer: a per-owner counter stored in an encrypted dictionary.
//! The counter PDA signs state creation and FHE calls; each new count is decryptable by its owner.
//! The specimen assumes the PoC's disabled deny list and unrestricted HCU configuration.

// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]

/// Program-specific error codes.
pub mod errors;
/// Counter state and address derivations.
pub mod state;

// Re-export errors and state for generated clients and tests.
pub use errors::*;
pub use state::*;

use anchor_lang::prelude::*;
use zama_fhe::{
    ExecutionAuthority, ExecutionCpiAccounts, FheExecution, Output, Scalar, State, Uint,
};
use zama_host::program::ZamaHost;

declare_id!("6zEiFjcGjYaVDmVETVPRQB2p6vk9zj6aPbXCKGVuS8wj");

#[program]
pub mod encrypted_counter {
    use super::*;

    /// Creates the counter state and initializes its count slot to zero.
    pub fn initialize<'info>(ctx: Context<'info, Initialize<'info>>) -> Result<()> {
        let counter = ctx.accounts.counter.key();
        ctx.accounts.counter.set_inner(Counter {
            bump: ctx.bumps.counter,
            authority_bump: ctx.bumps.counter_authority,
        });
        // The owner user-decrypts their count; the counter authority reads it as the next
        // increment's operand by signing, so it needs no allow.
        let bump = [ctx.bumps.counter_authority];
        let authority_seeds: &[&[u8]] = &[COUNTER_AUTHORITY_SEED, counter.as_ref(), &bump];
        zama_host::cpi::create_encrypted_state(
            CpiContext::new_with_signer(
                ctx.accounts.zama_program.key(),
                zama_host::cpi::accounts::CreateEncryptedState {
                    payer: ctx.accounts.owner.to_account_info(),
                    authority: ctx.accounts.counter_authority.to_account_info(),
                    encrypted_state: ctx.accounts.encrypted_state.to_account_info(),
                    host_config: ctx.accounts.host_config.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
                &[authority_seeds],
            ),
            zama_host::instructions::CreateEncryptedStateArgs {
                program: crate::ID,
                scope: counter.to_bytes(),
                authority_seeds: authority_seeds.iter().map(|seed| seed.to_vec()).collect(),
            },
        )?;
        let info = ctx.accounts.encrypted_state.to_account_info();
        let account =
            zama_host::EncryptedState::try_deserialize(&mut &info.try_borrow_data()?[..])?;
        let state = State::new(&account);
        let output = state.set(count_key()).allow(ctx.accounts.owner.key());
        let execution = FheExecution::build(
            ExecutionAuthority::new(ctx.accounts.counter_authority.key()),
            |builder| {
                builder.trivial_encrypt_u64(0, Output::state(output))?;
                Ok(())
            },
        )
        .map_err(invalid_execution)?;
        let resolved = execution
            .resolve_accounts(
                [ctx.accounts.encrypted_state.to_account_info()],
                [ctx.accounts.counter_authority.to_account_info()],
            )
            .map_err(invalid_execution_accounts)?;
        execution.invoke(
            ExecutionCpiAccounts {
                payer: ctx.accounts.owner.to_account_info(),
                authority: ctx.accounts.counter_authority.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                deny_scope_records: ctx.remaining_accounts.to_vec(),
                system_program: ctx.accounts.system_program.to_account_info(),
                hcu_block_meter: None,
                hcu_trusted_app_record: None,
                rand_nonce: None,
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
        let state = State::new(&ctx.accounts.encrypted_state);
        let operand = state
            .get::<Uint<64>>(count_key())
            .map_err(invalid_execution)?;
        let output = state.set(count_key()).allow(ctx.accounts.owner.key());
        let execution = FheExecution::build_returning(
            ExecutionAuthority::new(ctx.accounts.counter_authority.key()),
            |builder| {
                builder.add(
                    operand,
                    Scalar::<Uint<64>>::u64(amount),
                    Output::state(output),
                )
            },
        )
        .map_err(invalid_execution)?;
        let resolved = execution
            .execution()
            .resolve_accounts(
                [ctx.accounts.encrypted_state.to_account_info()],
                [ctx.accounts.counter_authority.to_account_info()],
            )
            .map_err(invalid_execution_accounts)?;
        let bump = [ctx.accounts.counter.authority_bump];
        let authority_seeds: &[&[u8]] = &[COUNTER_AUTHORITY_SEED, counter.as_ref(), &bump];
        let handle = execution.invoke(
            ExecutionCpiAccounts {
                payer: ctx.accounts.owner.to_account_info(),
                authority: ctx.accounts.counter_authority.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                deny_scope_records: ctx.remaining_accounts.to_vec(),
                system_program: ctx.accounts.system_program.to_account_info(),
                hcu_block_meter: None,
                hcu_trusted_app_record: None,
                rand_nonce: None,
                event_authority: ctx.accounts.zama_event_authority.to_account_info(),
                program: ctx.accounts.zama_program.to_account_info(),
            },
            &resolved,
            &[authority_seeds],
        )?;
        anchor_lang::solana_program::program::set_return_data(&handle);
        Ok(())
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
    /// CHECK: this program PDA signs host calls.
    #[account(seeds = [COUNTER_AUTHORITY_SEED, counter.key().as_ref()], bump)]
    pub counter_authority: UncheckedAccount<'info>,
    /// CHECK: created by the host at the counter's canonical state address.
    #[account(mut, address = counter_state_id(counter.key()).address() @ CounterError::CountValueInvalid)]
    pub encrypted_state: UncheckedAccount<'info>,
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
    /// CHECK: this program PDA signs host calls.
    #[account(seeds = [COUNTER_AUTHORITY_SEED, counter.key().as_ref()], bump = counter.authority_bump)]
    pub counter_authority: UncheckedAccount<'info>,
    /// Host-owned dictionary holding the current count and decrypt history.
    #[account(mut, address = counter_state_id(counter.key()).address() @ CounterError::CountValueInvalid)]
    pub encrypted_state: Box<Account<'info, zama_host::EncryptedState>>,
    /// CHECK: ZamaHost config PDA; validated by the host program.
    pub host_config: UncheckedAccount<'info>,
    /// CHECK: ZamaHost event-CPI authority; validated by the host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}
