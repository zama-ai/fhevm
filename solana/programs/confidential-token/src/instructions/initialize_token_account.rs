//! Initializes confidential token accounts and their zero-balance handles.

use super::*;

/// Accounts for initializing a confidential token account.
#[derive(Accounts)]
#[event_cpi]
pub struct InitializeTokenAccount<'info> {
    /// Pays rent for the token account and its initial encrypted balance.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Account owner. The canonical token-account PDA, stored owner, and the key allowed to
    /// decrypt the balance are all derived from this address.
    /// CHECK: This account is used only as a public key.
    pub owner: UncheckedAccount<'info>,
    /// Confidential mint this account belongs to.
    pub mint: Account<'info, ConfidentialMint>,
    #[account(
        init,
        payer = payer,
        space = 8 + ConfidentialTokenAccount::SPACE,
        seeds = [b"token-account", mint.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub token_account: Account<'info, ConfidentialTokenAccount>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub balance_encrypted_value: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to create the initial balance handle.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Account<'info, zama_host::HostConfig>,
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

/// Initializes a token account and creates its zero confidential balance handle.
pub fn initialize_token_account<'info>(
    ctx: Context<'info, InitializeTokenAccount<'info>>,
) -> Result<()> {
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    {
        let token_account = &mut ctx.accounts.token_account;
        token_account.owner = ctx.accounts.owner.key();
        token_account.mint = ctx.accounts.mint.key();
        token_account.balance_encrypted_value = Pubkey::default();
        token_account.bump = ctx.bumps.token_account;
    }
    let mint_key = ctx.accounts.mint.key();
    let owner = ctx.accounts.owner.key();
    let token_account_key = ctx.accounts.token_account.key();
    let balance_encrypted_value = ctx.accounts.balance_encrypted_value.key();
    let authority = fhe::ValueAuthority::token_account(&ctx.accounts.token_account)?;
    let balance_output = fhe::PersistentOutput::new(
        ctx.accounts.balance_encrypted_value.to_account_info(),
        balance_encrypted_value_id(mint_key, token_account_key),
        &authority,
        [owner],
    )?;
    let execution = zama_fhe::FheExecution::build(
        zama_fhe::ExecutionEncryptedValueAccountAuthority::new(token_account_key),
        |builder| {
            builder.trivial_encrypt_u64(0, balance_output.output())?;
            Ok(())
        },
    )
    .map_err(invalid_execution)?;
    let execution_accounts = fhe::ExecutionAccountSet::for_execution(
        &execution,
        [balance_output.account_info()],
        [authority],
    )?;
    fhe::execute(fhe::Execute {
        context: fhe::ExecuteContext {
            payer: &ctx.accounts.payer,
            event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            deny_scope_record: fhe::deny_scope_record(
                &ctx.accounts.host_config,
                ctx.remaining_accounts,
                mint_key,
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
    let balance_handle = balance_output.handle()?;
    let token_account = &mut ctx.accounts.token_account;
    token_account.balance_encrypted_value = balance_encrypted_value;
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: ctx.accounts.mint.key(),
        owner: ctx.accounts.owner.key(),
        token_account: token_account.key(),
        old_handle: [0; 32],
        old_encrypted_value: Pubkey::default(),
        new_handle: balance_handle,
        new_encrypted_value: balance_encrypted_value,
        reason: BalanceHandleUpdateReason::Initialize,
    });
    Ok(())
}
