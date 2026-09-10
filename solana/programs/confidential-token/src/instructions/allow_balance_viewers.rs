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
    /// Pays for encrypted State growth on the host.
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
    /// Stable balance encrypted State; read for the current handle and replaced.
    #[account(mut, address = encrypted_state_address(mint.key(), token_account.key()).0)]
    pub balance_state: Box<Account<'info, zama_host::EncryptedState>>,
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// CHECK: shared transaction transient store, validated by ZamaHost.
    #[account(mut)]
    pub transient_store: UncheckedAccount<'info>,
    /// CHECK: runtime Instructions sysvar, validated by ZamaHost.
    pub instructions: UncheckedAccount<'info>,
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
    pub encrypted_state: UncheckedAccount<'info>,
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
    let balance_state = &ctx.accounts.balance_state;
    assert_token_value(balance_state, mint_key, token_account.key(), balance_key())?;
    let old_balance_handle = fhe::state_handle(balance_state, balance_key())?;
    let new_balance_handle = rewrite_allowing(
        fhe::ExecuteContext {
            payer: &ctx.accounts.payer,
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
        balance_state,
        balance_slot(mint_key, token_account.key()),
        fhe::StateAuthority::token_account(token_account)?,
        std::iter::once(owner).chain(viewers),
    )?;
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner,
        token_account: token_account.key(),
        old_handle: old_balance_handle,
        old_encrypted_state: balance_state.key(),
        new_handle: new_balance_handle,
        new_encrypted_state: balance_state.key(),
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
        DisclosedValueKind::Balance => balance_key(),
        DisclosedValueKind::BurnedAmount => burned_amount_key(),
        DisclosedValueKind::TotalSupply => {
            return err!(ConfidentialTokenError::DisclosedValueBindingMismatch);
        }
    };
    let value = fhe::read_state(&ctx.accounts.encrypted_state.to_account_info())?;
    assert_token_value(&value, mint, token_account, label)
        .map_err(|_| error!(ConfidentialTokenError::DisclosedValueBindingMismatch))?;
    require_keys_eq!(
        ctx.accounts.encrypted_state.key(),
        encrypted_state_address(mint, token_account).0,
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
    cpi::make_state_handle_public(
        CpiContext::new_with_signer(
            ctx.accounts.zama_program.key(),
            cpi::accounts::MakeStateHandlePublic {
                payer: ctx.accounts.payer.to_account_info(),
                authority: ctx.accounts.token_account.to_account_info(),
                encrypted_state: ctx.accounts.encrypted_state.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                deny_scope_record,
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &[seeds],
        ),
        label,
        handle,
        value.leaf_count,
    )
}
