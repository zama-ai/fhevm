//! Joins the pending batch with a coprocessor-attested encrypted amount of the
//! batcher's join token (confidential underlying for deposit batchers,
//! confidential shares for redeem batchers — the code is direction-free).
//!
use super::*;

/// Accounts for joining a batch.
#[derive(Accounts)]
pub struct Join<'info> {
    /// Joining user; transfer authority over their confidential balance.
    pub user: Signer<'info>,
    /// Pays join-record rent, transfer output rent, and the batcher execution's
    /// ACL rent.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Batcher config.
    pub batcher: Box<Account<'info, Batcher>>,
    /// The pending batch being joined.
    #[account(mut, constraint = batch.batcher == batcher.key() @ BatcherError::BatchBatcherMismatch)]
    pub batch: Box<Account<'info, Batch>>,
    /// CHECK: per-batch authority PDA; recipient owner of the transfer and authority of the
    /// receipt the token program writes.
    #[account(seeds = [BATCH_AUTHORITY_SEED, batch.key().as_ref()], bump = batch.authority_bump)]
    pub batch_authority: UncheckedAccount<'info>,
    /// The user's join record for this batch; created on first join.
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + JoinRecord::SPACE,
        seeds = [JOIN_RECORD_SEED, batch.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub join_record: Box<Account<'info, JoinRecord>>,
    /// Confidential mint users join batches with.
    pub join_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: underlying SPL mint wrapped by `join_confidential_mint`. Token program is its owner.
    pub join_underlying_mint: UncheckedAccount<'info>,
    /// CHECK: ATA of `user` on `join_underlying_mint`. Uninitialized → not frozen.
    pub user_ata: UncheckedAccount<'info>,
    /// CHECK: ATA of `batch_authority` on `join_underlying_mint`. Uninitialized → not frozen.
    pub batch_authority_ata: UncheckedAccount<'info>,
    /// CHECK: user's confidential token account (transfer source); validated
    /// by the token CPI.
    #[account(mut)]
    pub user_token_account: UncheckedAccount<'info>,
    /// CHECK: batch's confidential join token account (transfer destination);
    /// validated by the token CPI and pinned below.
    #[account(mut)]
    pub batch_join_token_account: UncheckedAccount<'info>,
    /// CHECK: user's stable balance encrypted State; replaced by the token CPI.
    #[account(mut)]
    pub user_balance_state: UncheckedAccount<'info>,
    /// CHECK: batch's stable balance encrypted State; replaced by the token CPI.
    #[account(mut)]
    pub batch_balance_state: UncheckedAccount<'info>,
    /// CHECK: canonical host state controlled by this JoinRecord.
    #[account(mut)]
    pub join_state: UncheckedAccount<'info>,
    /// CHECK: host validates the scratch derived from join_state.
    #[account(mut)]
    pub scratch: UncheckedAccount<'info>,
    /// CHECK: host validates the real Instructions sysvar and final close.
    pub instructions: UncheckedAccount<'info>,
    /// CHECK: ZamaHost event-CPI authority; validated by the host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program (FHE compute + ACL).
    pub zama_program: Program<'info, ZamaHost>,
    /// CHECK: ZamaHost config PDA; validated by the host program.
    pub host_config: UncheckedAccount<'info>,
    /// CHECK: confidential-token event-CPI authority; validated by the token program.
    pub confidential_token_event_authority: UncheckedAccount<'info>,
    /// confidential-token program composed via CPI.
    pub confidential_token_program: Program<'info, ConfidentialToken>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Transfers into the batch account, then adds the token's returned result to the
/// JoinRecord contribution slot using the scratch grant.
pub fn join<'info>(
    ctx: Context<'info, Join<'info>>,
    amount_attestation: zama_host::CoprocessorInputAttestation,
) -> Result<()> {
    require!(
        ctx.accounts.batch.status == BatchStatus::Pending,
        BatcherError::BatchNotPending
    );
    require_keys_eq!(
        ctx.accounts.join_confidential_mint.key(),
        ctx.accounts.batcher.join_confidential_mint,
        BatcherError::ConfidentialMintMismatch
    );
    let mint_key = ctx.accounts.join_confidential_mint.key();
    let batch_key = ctx.accounts.batch.key();
    let user = ctx.accounts.user.key();
    let batch_authority = ctx.accounts.batch_authority.key();
    require_keys_eq!(
        ctx.accounts.batch_join_token_account.key(),
        ct::token_account_address(mint_key, batch_authority).0,
        BatcherError::DerivedAccountMismatch
    );
    let record_key = ctx.accounts.join_record.key();
    let id = join_state_id(batch_key, record_key);
    require_keys_eq!(
        ctx.accounts.join_state.key(),
        id.address(),
        BatcherError::DerivedAccountMismatch
    );
    let bump = [ctx.bumps.join_record];
    let authority_seeds: &[&[u8]] = &[JOIN_RECORD_SEED, batch_key.as_ref(), user.as_ref(), &bump];
    if ctx.accounts.join_state.owner == &System::id() {
        zama_host::cpi::create_encrypted_state(
            CpiContext::new_with_signer(
                ctx.accounts.zama_program.key(),
                zama_host::cpi::accounts::CreateEncryptedState {
                    payer: ctx.accounts.payer.to_account_info(),
                    authority: ctx.accounts.join_record.to_account_info(),
                    encrypted_state: ctx.accounts.join_state.to_account_info(),
                    host_config: ctx.accounts.host_config.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
                &[authority_seeds],
            ),
            zama_host::instructions::CreateEncryptedStateArgs {
                program: crate::ID,
                scope: batch_key.to_bytes(),
                authority_seeds: authority_seeds.iter().map(|s| s.to_vec()).collect(),
            },
        )?;
    }
    zama_host::cpi::open_scratch(CpiContext::new_with_signer(
        ctx.accounts.zama_program.key(),
        zama_host::cpi::accounts::OpenScratch {
            payer: ctx.accounts.payer.to_account_info(),
            authority: ctx.accounts.join_record.to_account_info(),
            encrypted_state: ctx.accounts.join_state.to_account_info(),
            scratch: ctx.accounts.scratch.to_account_info(),
            instructions: ctx.accounts.instructions.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        &[authority_seeds],
    ))?;
    ct::cpi::confidential_transfer(
        CpiContext::new_with_signer(
            ctx.accounts.confidential_token_program.key(),
            ct::cpi::accounts::ConfidentialTransfer {
                owner: ctx.accounts.user.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                mint: ctx.accounts.join_confidential_mint.to_account_info(),
                underlying_mint: ctx.accounts.join_underlying_mint.to_account_info(),
                from_ata: ctx.accounts.user_ata.to_account_info(),
                to_ata: ctx.accounts.batch_authority_ata.to_account_info(),
                from_account: ctx.accounts.user_token_account.to_account_info(),
                to_account: ctx.accounts.batch_join_token_account.to_account_info(),
                from_state: ctx.accounts.user_balance_state.to_account_info(),
                to_state: ctx.accounts.batch_balance_state.to_account_info(),
                zama_event_authority: ctx.accounts.zama_event_authority.to_account_info(),
                zama_program: ctx.accounts.zama_program.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                hcu_block_meter: None,
                hcu_trusted_app_record: None,
                result_state: Some(ctx.accounts.join_state.to_account_info()),
                result_scratch: Some(ctx.accounts.scratch.to_account_info()),
                result_authority: Some(ctx.accounts.join_record.to_account_info()),
                event_authority: ctx
                    .accounts
                    .confidential_token_event_authority
                    .to_account_info(),
                program: ctx.accounts.confidential_token_program.to_account_info(),
            },
            &[authority_seeds],
        ),
        amount_attestation,
    )?;
    let (producer, returned) = anchor_lang::solana_program::program::get_return_data()
        .ok_or(BatcherError::InvalidFheExecution)?;
    require_keys_eq!(producer, ct::ID, BatcherError::InvalidFheExecution);
    let transferred: [u8; 32] = returned
        .try_into()
        .map_err(|_| error!(BatcherError::InvalidFheExecution))?;
    let account = fhe::read_state(&ctx.accounts.join_state)?;
    let state = zama_fhe::State::new(&account);
    let amount = state
        .granted::<zama_fhe::Uint<64>>(transferred, id)
        .map_err(fhe::invalid_execution)?;
    let previous = account
        .get(&joined_amount_key())
        .map(|_| state.get::<zama_fhe::Uint<64>>(joined_amount_key()))
        .transpose()
        .map_err(fhe::invalid_execution)?;
    let output = zama_fhe::Output::state(state.set(joined_amount_key()).allow(user));
    let execution = zama_fhe::FheExecution::build_returning(
        zama_fhe::ExecutionAuthority::new(record_key),
        |builder| match previous {
            Some(previous) => builder.add(previous, amount, output),
            None => builder.add(
                amount,
                zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(0),
                output,
            ),
        },
    )
    .map_err(fhe::invalid_execution)?;
    let joined_handle = fhe::JoinExecute {
        batch: batch_key,
        user,
        bump: ctx.bumps.join_record,
        record: ctx.accounts.join_record.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        host_config: ctx.accounts.host_config.to_account_info(),
        event_authority: ctx.accounts.zama_event_authority.to_account_info(),
        program: ctx.accounts.zama_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        deny_records: ctx.remaining_accounts,
    }
    .invoke(
        execution,
        vec![
            ctx.accounts.join_state.to_account_info(),
            ctx.accounts.scratch.to_account_info(),
        ],
    )?;

    let record = &mut ctx.accounts.join_record;
    record.batch = batch_key;
    record.user = user;
    record.bump = ctx.bumps.join_record;

    let batch = &mut ctx.accounts.batch;
    batch.join_count = batch.join_count.saturating_add(1);

    emit!(JoinedBatch {
        version: APP_EVENT_VERSION,
        batch: batch_key,
        user,
        joined_encrypted_state: ctx.accounts.join_state.key(),
        joined_handle,
    });
    Ok(())
}
