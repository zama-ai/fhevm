//! Opens a batch: the `Batch` account, its per-batch authority, its own
//! confidential token accounts (join and payout side), and its plain SPL
//! accounts for settle's redeem/vault/wrap phases. `initialize_batcher` opens
//! batch 0 and `dispatch` opens the next batch (INVARIANTS #70).
//!
//! One batch, one token account: the burned/revealed total is exactly this
//! batch's sum, so dust from an earlier batch can never leak into it (the EVM
//! batcher documents the inter-batch dust leak this prevents).
//!
//! Direction-free: the join/payout roles are fixed by the batcher config, so
//! deposit and redeem batches open identically.

use super::*;

/// The accounts that open a batch, borrowed from `initialize_batcher` or `dispatch`. Anchor has
/// already created `batch` at the batcher's next index and both plain SPL accounts.
pub(crate) struct OpenBatch<'a, 'info> {
    pub payer: &'a Signer<'info>,
    pub batcher: &'a mut Account<'info, Batcher>,
    pub batch: &'a mut Account<'info, Batch>,
    pub batch_authority: &'a UncheckedAccount<'info>,
    pub batch_authority_bump: u8,
    pub join_confidential_mint: &'a Account<'info, ct::ConfidentialMint>,
    pub join_underlying_mint: &'a Account<'info, SplMint>,
    pub batch_join_token_account: &'a UncheckedAccount<'info>,
    pub batch_join_balance_store: &'a UncheckedAccount<'info>,
    pub join_mint_hcu_block_meter: &'a Option<UncheckedAccount<'info>>,
    pub join_mint_hcu_trusted_app_record: &'a Option<UncheckedAccount<'info>>,
    pub payout_confidential_mint: &'a Account<'info, ct::ConfidentialMint>,
    pub payout_underlying_mint: &'a Account<'info, SplMint>,
    pub batch_payout_token_account: &'a UncheckedAccount<'info>,
    pub batch_payout_balance_store: &'a UncheckedAccount<'info>,
    pub payout_mint_hcu_block_meter: &'a Option<UncheckedAccount<'info>>,
    pub payout_mint_hcu_trusted_app_record: &'a Option<UncheckedAccount<'info>>,
    pub zama_event_authority: &'a UncheckedAccount<'info>,
    pub transient_store: &'a UncheckedAccount<'info>,
    pub instructions: &'a UncheckedAccount<'info>,
    pub zama_program: &'a Program<'info, ZamaHost>,
    pub host_config: &'a Account<'info, zama_host::HostConfig>,
    pub confidential_token_event_authority: &'a UncheckedAccount<'info>,
    pub confidential_token_program: &'a Program<'info, ConfidentialToken>,
    pub system_program: &'a Program<'info, System>,
}

/// Funds the batch authority, creates both batch token accounts unless they already exist, marks the
/// batch Pending and advances the batcher's next index. Deny records: `(token, join mint)`, then
/// `(token, payout mint)`.
pub(crate) fn open_batch<'info>(
    accounts: OpenBatch<'_, 'info>,
    authority_funding_lamports: u64,
    [join_deny_records, payout_deny_records]: [&[AccountInfo<'info>]; 2],
) -> Result<()> {
    require_keys_eq!(
        accounts.join_confidential_mint.key(),
        accounts.batcher.join_confidential_mint,
        BatcherError::ConfidentialMintMismatch
    );
    require_keys_eq!(
        accounts.payout_confidential_mint.key(),
        accounts.batcher.payout_confidential_mint,
        BatcherError::ConfidentialMintMismatch
    );
    require_keys_eq!(
        accounts.join_underlying_mint.key(),
        accounts.join_confidential_mint.underlying_mint,
        BatcherError::JoinMintVaultMismatch
    );
    require_keys_eq!(
        accounts.payout_underlying_mint.key(),
        accounts.payout_confidential_mint.underlying_mint,
        BatcherError::PayoutMintVaultMismatch
    );

    fund_batch_authority(
        accounts.payer,
        accounts.batch_authority,
        accounts.system_program,
        authority_funding_lamports,
    )?;

    let batch_key = accounts.batch.key();
    let authority = BatchAuthoritySeeds::new(batch_key, accounts.batch_authority_bump);
    let authority_seeds = authority.seeds();
    for (mint, token_account, balance_store, records, hcu_block_meter, hcu_trusted_app_record) in [
        (
            accounts.join_confidential_mint,
            accounts.batch_join_token_account,
            accounts.batch_join_balance_store,
            join_deny_records,
            accounts.join_mint_hcu_block_meter,
            accounts.join_mint_hcu_trusted_app_record,
        ),
        (
            accounts.payout_confidential_mint,
            accounts.batch_payout_token_account,
            accounts.batch_payout_balance_store,
            payout_deny_records,
            accounts.payout_mint_hcu_block_meter,
            accounts.payout_mint_hcu_trusted_app_record,
        ),
    ] {
        ct::cpi::initialize_token_account(
            CpiContext::new_with_signer(
                accounts.confidential_token_program.key(),
                ct::cpi::accounts::InitializeTokenAccount {
                    payer: accounts.batch_authority.to_account_info(),
                    owner: accounts.batch_authority.to_account_info(),
                    mint: mint.to_account_info(),
                    token_account: token_account.to_account_info(),
                    balance_encrypted_store: balance_store.to_account_info(),
                    zama_event_authority: accounts.zama_event_authority.to_account_info(),
                    transient_store: accounts.transient_store.to_account_info(),
                    instructions: accounts.instructions.to_account_info(),
                    zama_program: accounts.zama_program.to_account_info(),
                    host_config: accounts.host_config.to_account_info(),
                    system_program: accounts.system_program.to_account_info(),
                    hcu_block_meter: hcu_block_meter
                        .as_ref()
                        .map(|account| account.to_account_info()),
                    hcu_trusted_app_record: hcu_trusted_app_record
                        .as_ref()
                        .map(|account| account.to_account_info()),
                    event_authority: accounts
                        .confidential_token_event_authority
                        .to_account_info(),
                    program: accounts.confidential_token_program.to_account_info(),
                },
                &[&authority_seeds],
            )
            .with_remaining_accounts(records.to_vec()),
        )?;
    }

    let index = accounts.batcher.next_batch_index;
    let opened_at = Clock::get()?.unix_timestamp;
    let batch = accounts.batch;
    batch.batcher = accounts.batcher.key();
    batch.index = index;
    batch.status = BatchStatus::Pending;
    // Bounded by `MAX_MIN_BATCH_AGE_SECS`, so the cast cannot wrap.
    batch.dispatchable_at = opened_at.saturating_add(accounts.batcher.min_batch_age_secs as i64);
    batch.settle_deadline_secs = accounts.batcher.settle_deadline_secs;
    batch.settle_deadline = 0;
    batch.join_count = 0;
    batch.authority_bump = accounts.batch_authority_bump;
    batch.burned_total_handle = [0; 32];
    batch.total_joined = 0;
    batch.payout_received = 0;

    let batcher = accounts.batcher;
    batcher.next_batch_index = index
        .checked_add(1)
        .ok_or(BatcherError::BatchIndexOverflow)?;

    emit!(BatchOpened {
        batcher: batcher.key(),
        batch: batch_key,
        index,
        batch_authority: accounts.batch_authority.key(),
        opened_at,
        min_batch_age_secs: batcher.min_batch_age_secs,
        settle_deadline_secs: batch.settle_deadline_secs,
    });
    Ok(())
}
