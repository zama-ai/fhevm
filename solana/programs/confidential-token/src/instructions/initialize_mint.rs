//! Initializes confidential mint state and its zero encrypted total supply.

use super::*;

/// Accounts for initializing a confidential mint.
#[derive(Accounts)]
#[event_cpi]
pub struct InitializeMint<'info> {
    /// Mint authority and rent payer.
    #[account(mut)]
    pub authority: Signer<'info>,
    /// Confidential mint account created by this instruction.
    #[account(init, payer = authority, space = 8 + ConfidentialMint::INIT_SPACE)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Underlying SPL mint wrapped by this confidential mint.
    pub underlying_mint: Box<InterfaceAccount<'info, SplMint>>,
    /// Classic Token or Token-2022 program owning `underlying_mint`.
    pub token_program: Interface<'info, TokenInterface>,
    /// CHECK: Mint-scoped encrypted store authority for total-supply handles.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub total_supply_encrypted_store: UncheckedAccount<'info>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
    pub zama: ZamaHostCpi<'info>,
}

/// Initializes a confidential mint and creates its zero encrypted total supply, allowed to nobody
/// until the mint authority grants viewers.
pub fn initialize_mint<'info>(ctx: Context<'info, InitializeMint<'info>>) -> Result<()> {
    assert_supported_underlying_mint(&ctx.accounts.underlying_mint, &ctx.accounts.token_program)?;
    let mint_key = ctx.accounts.mint.key();
    let authority = fhe::StoreAuthority::total_supply(
        &ctx.accounts.total_supply_authority,
        mint_key,
        ctx.bumps.total_supply_authority,
    );
    authority.create_state(
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.total_supply_encrypted_store.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.zama.host_config.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;
    let total_supply_encrypted_store = ctx.accounts.total_supply_encrypted_store.key();
    let total_supply_output = fhe::SlotOutput::new(
        ctx.accounts.total_supply_encrypted_store.to_account_info(),
        total_supply_slot(mint_key),
        &authority,
        [],
    )?;
    let execution = zama_fhe::FheExecution::build(total_supply_slot(mint_key).0, |builder| {
        let new_total_supply = builder.trivial_encrypt_u64(0)?;
        builder.output(new_total_supply, total_supply_output.output())?;
        Ok(())
    })?;
    let execution_accounts = fhe::ExecutionAccountSet::for_execution(
        &execution,
        [total_supply_output.account_info()],
        [authority],
    )?;
    fhe::execute(fhe::Execute {
        context: fhe::ExecuteContext {
            payer: &ctx.accounts.authority,
            zama: &ctx.accounts.zama,
            deny_scope_records: fhe::deny_scope_records(
                &ctx.accounts.zama.host_config,
                ctx.remaining_accounts,
                [token_app(mint_key)],
            )?,
            system_program: &ctx.accounts.system_program,
        },
        accounts: &execution_accounts,
        execution,
    })?;
    let total_supply_handle = total_supply_output.handle()?;
    let mint = &mut ctx.accounts.mint;
    mint.authority = ctx.accounts.authority.key();
    mint.underlying_mint = ctx.accounts.underlying_mint.key();
    mint.decimals = ctx.accounts.underlying_mint.decimals;
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        old_handle: [0; 32],
        new_handle: total_supply_handle,
        encrypted_store: total_supply_encrypted_store,
        reason: TotalSupplyUpdateReason::Initialize,
    });
    Ok(())
}
