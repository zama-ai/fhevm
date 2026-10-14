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
        init_if_needed,
        payer = payer,
        space = 8 + ConfidentialTokenAccount::INIT_SPACE,
        seeds = [b"token-account", mint.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub token_account: Account<'info, ConfidentialTokenAccount>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub balance_encrypted_store: UncheckedAccount<'info>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
    pub zama: ZamaHostCpi<'info>,
}

/// Initializes a token account and creates its zero confidential balance handle, or leaves an
/// existing one unchanged.
pub fn initialize_token_account<'info>(
    ctx: Context<'info, InitializeTokenAccount<'info>>,
) -> Result<()> {
    // An account this call creates is zeroed; one that existed holds its mint. The owner need not
    // sign, so anyone can create an account first: the batcher's `dispatch` must still succeed, and
    // the existing balance must not be replaced by a new zero.
    if ctx.accounts.token_account.mint == ctx.accounts.mint.key() {
        return Ok(());
    }
    {
        let token_account = &mut ctx.accounts.token_account;
        token_account.owner = ctx.accounts.owner.key();
        token_account.mint = ctx.accounts.mint.key();
        token_account.bump = ctx.bumps.token_account;
    }
    let mint_key = ctx.accounts.mint.key();
    let owner = ctx.accounts.owner.key();
    let token_account_key = ctx.accounts.token_account.key();
    let balance_encrypted_store = ctx.accounts.balance_encrypted_store.key();
    let authority = fhe::StoreAuthority::token_account(&ctx.accounts.token_account);
    authority.create_state(
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.balance_encrypted_store.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.zama.host_config.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;
    let balance_output = fhe::SlotOutput::new(
        ctx.accounts.balance_encrypted_store.to_account_info(),
        balance_slot(mint_key, token_account_key),
        &authority,
        [owner],
    )?;
    let execution =
        zama_fhe::FheExecution::build(balance_slot(mint_key, token_account_key).0, |builder| {
            let new_balance = builder.trivial_encrypt_u64(0)?;
            builder.output(new_balance, balance_output.output())?;
            Ok(())
        })?;
    let execution_accounts = fhe::ExecutionAccountSet::for_execution(
        &execution,
        [balance_output.account_info()],
        [authority],
    )?;
    fhe::execute(fhe::Execute {
        context: fhe::ExecuteContext {
            payer: &ctx.accounts.payer,
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
    let balance_handle = balance_output.handle()?;
    let token_account = &mut ctx.accounts.token_account;
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: ctx.accounts.mint.key(),
        owner: ctx.accounts.owner.key(),
        token_account: token_account.key(),
        old_handle: [0; 32],
        new_handle: balance_handle,
        encrypted_store: balance_encrypted_store,
        reason: BalanceHandleUpdateReason::Initialize,
    });
    Ok(())
}
