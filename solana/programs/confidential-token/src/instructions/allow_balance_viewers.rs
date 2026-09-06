//! Owner-authorized allow and public-seal of token-account state.
//!
//! Who may decrypt a handle is decided on the write that produces it, so granting a viewer
//! (an auditor, a second wallet) access to a balance is a re-write: one execution copies the
//! balance onto a fresh handle allowed to the owner and the viewers. The grant lasts for that
//! handle; the next transfer or wrap produces a new one allowed to the owner alone, exactly as
//! an EVM `FHE.allow` covers one handle. Sealing a handle publicly stays a standalone host CPI
//! signed by the token-account PDA, the value's authority.

use super::*;
use zama_host::cpi;

/// Accounts for re-writing a balance allowed to extra viewers.
#[derive(Accounts)]
#[event_cpi]
pub struct AllowBalanceViewers<'info> {
    /// Pays for encrypted value account growth on the host.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Token account owner authorizing the grant.
    pub owner: Signer<'info>,
    pub mint: Box<Account<'info, ConfidentialMint>>,
    #[account(
        seeds = [b"token-account", mint.key().as_ref(), owner.key().as_ref()],
        bump = token_account.bump,
        has_one = owner @ ConfidentialTokenError::OwnerMismatch,
        has_one = mint @ ConfidentialTokenError::MintMismatch,
    )]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Stable balance encrypted value account; read for the current handle and replaced.
    #[account(mut, address = token_account.balance_encrypted_value)]
    pub balance_value: Box<Account<'info, zama_host::EncryptedValue>>,
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    pub zama_program: Program<'info, ZamaHost>,
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

/// Accounts for owner-authorized public sealing of a token-account state field.
#[derive(Accounts)]
pub struct MakeTokenAccountHandlePublic<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub mint: Box<Account<'info, ConfidentialMint>>,
    #[account(
        seeds = [b"token-account", mint.key().as_ref(), owner.key().as_ref()],
        bump = token_account.bump,
        has_one = owner @ ConfidentialTokenError::OwnerMismatch,
        has_one = mint @ ConfidentialTokenError::MintMismatch,
    )]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: canonical host account and exact token state binding are validated in the handler.
    #[account(mut)]
    pub encrypted_value: UncheckedAccount<'info>,
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}

/// Re-writes the balance onto a handle allowed to the owner and `viewers`.
pub fn allow_balance_viewers<'info>(
    ctx: Context<'info, AllowBalanceViewers<'info>>,
    viewers: Vec<Pubkey>,
) -> Result<()> {
    let mint_key = ctx.accounts.mint.key();
    let owner = ctx.accounts.owner.key();
    let token_account = &ctx.accounts.token_account;
    let balance_value = &ctx.accounts.balance_value;
    assert_token_value(
        balance_value,
        mint_key,
        token_account.key(),
        encrypted_balance_label(),
    )?;
    let deny_scope_record =
        fhe::deny_scope_record(&ctx.accounts.host_config, ctx.remaining_accounts, mint_key)?;
    let old_balance_handle = balance_value.current_handle;
    let balance = fhe::uint64_operand(balance_value)?;
    let authority = fhe::ValueAuthority::token_account(token_account)?;
    let balance_output = fhe::PersistentOutput::new(
        balance_value.to_account_info(),
        balance_encrypted_value_id(mint_key, token_account.key()),
        &authority,
        std::iter::once(owner).chain(viewers),
    )?;
    let execution = zama_fhe::FheExecution::build(
        zama_fhe::ExecutionEncryptedValueAccountAuthority::new(token_account.key()),
        |builder| {
            builder.add(
                balance,
                zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(0),
                balance_output.output(),
            )?;
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
            deny_scope_record,
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
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner,
        token_account: token_account.key(),
        old_handle: old_balance_handle,
        old_encrypted_value: balance_value.key(),
        new_handle: balance_output.handle()?,
        new_encrypted_value: balance_value.key(),
        reason: BalanceHandleUpdateReason::AllowViewers,
    });
    Ok(())
}

/// Seals the current handle of one exact token-account state field as publicly decryptable.
pub fn make_token_account_handle_public<'info>(
    ctx: Context<'info, MakeTokenAccountHandlePublic<'info>>,
    kind: DisclosedValueKind,
    handle: [u8; 32],
) -> Result<()> {
    let mint = ctx.accounts.mint.key();
    let token_account = ctx.accounts.token_account.key();
    let label = match kind {
        DisclosedValueKind::Balance => encrypted_balance_label(),
        DisclosedValueKind::TransferredAmount => encrypted_transferred_amount_label(),
        DisclosedValueKind::BurnedAmount => encrypted_burned_amount_label(),
        DisclosedValueKind::TotalSupply => {
            return err!(ConfidentialTokenError::DisclosedValueBindingMismatch);
        }
    };
    let value = fhe::read_encrypted_value(&ctx.accounts.encrypted_value.to_account_info())?;
    assert_token_value(&value, mint, token_account, label)
        .map_err(|_| error!(ConfidentialTokenError::DisclosedValueBindingMismatch))?;
    require_keys_eq!(
        ctx.accounts.encrypted_value.key(),
        encrypted_value_address(mint, token_account, label).0,
        ConfidentialTokenError::DisclosedValueBindingMismatch
    );
    let deny_scope_record =
        fhe::deny_scope_record(&ctx.accounts.host_config, ctx.remaining_accounts, mint)?;

    let owner = ctx.accounts.owner.key();
    let bump = [ctx.accounts.token_account.bump];
    let seeds: &[&[u8]] = &[
        b"token-account",
        mint.as_ref(),
        owner.as_ref(),
        bump.as_ref(),
    ];
    cpi::make_handle_public(
        CpiContext::new_with_signer(
            ctx.accounts.zama_program.key(),
            cpi::accounts::MakeEncryptedValueHandlePublic {
                payer: ctx.accounts.payer.to_account_info(),
                authority: ctx.accounts.token_account.to_account_info(),
                encrypted_value: ctx.accounts.encrypted_value.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                deny_scope_record,
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &[seeds],
        ),
        handle,
    )
}
