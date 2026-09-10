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
    #[account(init, payer = authority, space = 8 + ConfidentialMint::SPACE)]
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
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// CHECK: shared transaction transient store, validated by ZamaHost.
    #[account(mut)]
    pub transient_store: UncheckedAccount<'info>,
    /// CHECK: runtime Instructions sysvar, validated by ZamaHost.
    pub instructions: UncheckedAccount<'info>,
    /// ZamaHost program used to create the initial total-supply handle.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_execute` CPI, which validates it against the
    /// canonical `["hcu-block-meter", program, mint]` PDA. Supplied by an untrusted mint under a
    /// metering-band cap; omitted when the mint is trusted or the cap is unrestricted.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_execute` CPI, which validates it against the
    /// canonical `["hcu-trusted", program, mint]` PDA. Present + valid bypasses the cap; absent
    /// means the mint is metered.
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
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
    )?;
    authority.create_state(
        mint_key,
        ctx.accounts.total_supply_encrypted_store.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.host_config.to_account_info(),
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
    })
    .map_err(invalid_execution)?;
    let execution_accounts = fhe::ExecutionAccountSet::for_execution(
        &execution,
        [total_supply_output.account_info()],
        [authority],
    )?;
    fhe::execute(fhe::Execute {
        context: fhe::ExecuteContext {
            payer: &ctx.accounts.authority,
            event_authority: &ctx.accounts.zama_event_authority,
            transient_store: &ctx.accounts.transient_store,
            instructions: &ctx.accounts.instructions,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            deny_scope_records: fhe::deny_scope_records(
                &ctx.accounts.host_config,
                ctx.remaining_accounts,
                [token_app(mint_key)],
            )?,
            system_program: &ctx.accounts.system_program,
            hcu_block_meter: ctx
                .accounts
                .hcu_block_meter
                .as_ref()
                .map(|account| account.to_account_info()),
            hcu_trusted_app_record: ctx
                .accounts
                .hcu_trusted_app_record
                .as_ref()
                .map(|account| account.to_account_info()),
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
        old_encrypted_store: Pubkey::default(),
        new_handle: total_supply_handle,
        new_encrypted_store: total_supply_encrypted_store,
        reason: TotalSupplyUpdateReason::Initialize,
    });
    Ok(())
}
