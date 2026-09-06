//! Mint-authority allow and public-seal of the encrypted total supply.
//!
//! The total supply is controlled by the `total-supply` PDA. Nobody is allowed on it by default
//! (wrap and burn write it with an empty allow list); the mint authority grants viewers by
//! re-writing it onto a handle allowed to them, and seals it publicly through the host CPI the
//! PDA signs. See `allow_balance_viewers` for why an allow is a write.

use super::*;
use zama_host::cpi;

/// Accounts for re-writing the encrypted total supply allowed to `viewers`.
#[derive(Accounts)]
#[event_cpi]
pub struct AllowTotalSupplyViewers<'info> {
    /// Pays for encrypted value account growth.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Existing confidential mint authority. Governance may own this key later.
    pub authority: Signer<'info>,
    #[account(has_one = authority @ ConfidentialTokenError::MintAuthorityMismatch)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// CHECK: encrypted total-supply authority PDA; signs the host CPI.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Encrypted total-supply value; read for the current handle and replaced.
    #[account(mut, address = mint.total_supply_encrypted_value)]
    pub total_supply_value: Box<Account<'info, zama_host::EncryptedValue>>,
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

/// Accounts for mint-authority public sealing of encrypted total supply.
#[derive(Accounts)]
pub struct MakeTotalSupplyHandlePublic<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    #[account(has_one = authority @ ConfidentialTokenError::MintAuthorityMismatch)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// CHECK: encrypted total-supply authority PDA; signs the host CPI.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Encrypted total-supply value whose current handle is sealed.
    #[account(mut, address = mint.total_supply_encrypted_value)]
    pub total_supply_value: Box<Account<'info, zama_host::EncryptedValue>>,
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}

/// Re-writes the encrypted total supply onto a handle allowed to `viewers`.
pub fn allow_total_supply_viewers<'info>(
    ctx: Context<'info, AllowTotalSupplyViewers<'info>>,
    viewers: Vec<Pubkey>,
) -> Result<()> {
    let mint = ctx.accounts.mint.key();
    let total_supply_authority = ctx.accounts.total_supply_authority.key();
    let total_supply_value = &ctx.accounts.total_supply_value;
    assert_token_value(
        total_supply_value,
        mint,
        total_supply_authority,
        encrypted_total_supply_label(),
    )?;
    let old_total_supply_handle = total_supply_value.current_handle;
    let new_total_supply_handle = rewrite_allowing(
        fhe::ExecuteContext {
            payer: &ctx.accounts.payer,
            event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            deny_scope_records: fhe::deny_scope_records(
                &ctx.accounts.host_config,
                ctx.remaining_accounts,
                [token_app(mint)],
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
        total_supply_value,
        total_supply_encrypted_value_id(mint),
        fhe::ValueAuthority::total_supply(
            &ctx.accounts.total_supply_authority,
            mint,
            ctx.bumps.total_supply_authority,
        )?,
        viewers,
    )?;
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint,
        old_handle: old_total_supply_handle,
        old_encrypted_value: total_supply_value.key(),
        new_handle: new_total_supply_handle,
        new_encrypted_value: total_supply_value.key(),
        reason: TotalSupplyUpdateReason::AllowViewers,
    });
    Ok(())
}

/// Seals the encrypted total supply's current handle as publicly decryptable.
pub fn make_total_supply_handle_public<'info>(
    ctx: Context<'info, MakeTotalSupplyHandlePublic<'info>>,
    handle: [u8; 32],
) -> Result<()> {
    let mint = ctx.accounts.mint.key();
    assert_token_value(
        &ctx.accounts.total_supply_value,
        mint,
        ctx.accounts.total_supply_authority.key(),
        encrypted_total_supply_label(),
    )?;
    let deny_scope_record =
        fhe::deny_scope_record(&ctx.accounts.host_config, ctx.remaining_accounts, mint)?;

    let bump = [ctx.bumps.total_supply_authority];
    let seeds: &[&[u8]] = &[b"total-supply", mint.as_ref(), &bump];
    cpi::make_handle_public(
        CpiContext::new_with_signer(
            ctx.accounts.zama_program.key(),
            cpi::accounts::MakeEncryptedValueHandlePublic {
                payer: ctx.accounts.payer.to_account_info(),
                authority: ctx.accounts.total_supply_authority.to_account_info(),
                encrypted_value: ctx.accounts.total_supply_value.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                deny_scope_record,
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &[seeds],
        ),
        handle,
    )
}
