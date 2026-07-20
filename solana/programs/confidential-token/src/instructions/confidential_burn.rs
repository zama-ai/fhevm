//! Burns encrypted token balances and rotates confidential supply state.

use super::*;

/// Accounts for confidential balance burn.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialBurn<'info> {
    /// Token owner and burn authority.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint whose encrypted total supply is decreased.
    #[account(mut)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token account whose balance is decreased.
    #[account(mut)]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: Mint-scoped app authority for total-supply handles.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Stable balance lineage; read for the current handle and superseded by this eval.
    #[account(mut, address = token_account.balance_encrypted_value)]
    pub balance_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// Stable total-supply lineage; read for the current handle and superseded by this eval.
    #[account(mut, address = mint.total_supply_encrypted_value)]
    pub total_supply_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// CHECK: stable `burned_amount` lineage for `token_account`; created on the
    /// account's first burn, superseded in place thereafter to each burn's own
    /// delta. Each burn makes its own delta handle publicly decryptable at burn
    /// (ERC-7984 `unwrap` parity), so every burn stays permanently redeemable
    /// even after a later burn supersedes this lineage (DD-036 / Vector 2 closed).
    #[account(mut, address = encrypted_value_address(mint.key(), token_account.key(), burned_amount_label()).0)]
    pub burned_amount_value: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it against the
    /// canonical `["hcu-block-meter", compute_signer]` PDA. Supplied by an untrusted mint under a
    /// metering-band cap; omitted when the mint is trusted or the cap is unrestricted.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it against the
    /// canonical `["hcu-trusted", compute_signer]` PDA. Present + valid bypasses the cap; absent
    /// means the mint is metered.
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
}

impl<'info> ConfidentialBurn<'info> {
    fn as_burn_accounts<'a>(
        &'a self,
        remaining_accounts: &'a [AccountInfo<'info>],
    ) -> BurnAccounts<'a, 'info> {
        BurnAccounts {
            payer: &self.owner,
            burn_authority: self.owner.key(),
            mint: &self.mint,
            token_account: &self.token_account,
            compute_signer: &self.compute_signer,
            total_supply_authority: &self.total_supply_authority,
            balance_value: self.balance_value.to_account_info(),
            total_supply_value: self.total_supply_value.to_account_info(),
            burned_amount_value: self.burned_amount_value.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            deny_subject_records: remaining_accounts,
            system_program: &self.system_program,
            hcu_block_meter: self
                .hcu_block_meter
                .as_ref()
                .map(|account| account.to_account_info()),
            hcu_trusted_app_record: self
                .hcu_trusted_app_record
                .as_ref()
                .map(|account| account.to_account_info()),
        }
    }
}

/// Burns an encrypted amount by rotating the account balance and encrypted total supply.
pub fn confidential_burn<'info>(
    ctx: Context<'info, ConfidentialBurn<'info>>,
    amount_attestation: zama_host::CoprocessorInputAttestation,
) -> Result<()> {
    let outcome = execute_burn(
        ctx.accounts.as_burn_accounts(ctx.remaining_accounts),
        ctx.bumps.compute_signer,
        BurnAmountSource::Attested(amount_attestation),
    )?;
    emit_cpi!(ConfidentialBurnEvent {
        version: APP_EVENT_VERSION,
        mint: outcome.mint,
        owner: outcome.owner,
        token_account: outcome.token_account,
        burned_handle: outcome.burned_handle,
        burned_encrypted_value: outcome.burned_encrypted_value,
    });
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: outcome.mint,
        owner: outcome.owner,
        token_account: outcome.token_account,
        old_handle: outcome.old_balance_handle,
        old_encrypted_value: outcome.balance_encrypted_value,
        new_handle: outcome.new_balance_handle,
        new_encrypted_value: outcome.balance_encrypted_value,
        reason: BalanceHandleUpdateReason::BurnDebit,
    });
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: outcome.mint,
        old_handle: outcome.old_total_supply_handle,
        old_encrypted_value: outcome.total_supply_encrypted_value,
        new_handle: outcome.new_total_supply_handle,
        new_encrypted_value: outcome.total_supply_encrypted_value,
        reason: TotalSupplyUpdateReason::Burn,
    });
    Ok(())
}

/// Accounts for a confidential burn that spends an existing on-chain `EncryptedValue` as the amount,
/// instead of a freshly attested client-side encryption.
///
/// This is the burn-side analog of [`ConfidentialTransferFromValue`]: the 190-byte attestation
/// argument is gone and one account is added — `amount_value`, the encrypted amount to burn. It is
/// read-only (the durable operand the eval reads) and is never superseded or consumed; only the
/// balance, total-supply, and burned-amount lineages change, exactly as in [`ConfidentialBurn`].
/// The batcher path uses this to burn a computed batch total (a handle produced by summing joins)
/// whose owner is a program PDA that authorizes the burn via `invoke_signed`.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialBurnFromValue<'info> {
    /// Token owner and burn authority. Must be in `amount_value`'s subject set (the spend gate).
    /// Not `mut`: rent for the burned-amount lineage's first bind is paid by `payer`, so the owner
    /// may be a program PDA (the batcher) that only authorizes the burn via `invoke_signed`.
    pub owner: Signer<'info>,
    /// Pays rent for the burned-amount lineage on its first bind.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Confidential mint whose encrypted total supply is decreased.
    #[account(mut)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token account whose balance is decreased.
    #[account(mut)]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: Mint-scoped app authority for total-supply handles.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Stable balance lineage; read for the current handle and superseded by this eval.
    #[account(mut, address = token_account.balance_encrypted_value)]
    pub balance_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// Stable total-supply lineage; read for the current handle and superseded by this eval.
    #[account(mut, address = mint.total_supply_encrypted_value)]
    pub total_supply_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// CHECK: stable `burned_amount` lineage for `token_account`, born publicly decryptable exactly
    /// as in [`ConfidentialBurn`]; created on the account's first burn, superseded in place
    /// thereafter to each burn's own delta (DD-036 / Vector 2). This is the same output shape
    /// `redeem_burned_amount` later consumes — only where the amount comes from differs.
    #[account(mut, address = encrypted_value_address(mint.key(), token_account.key(), burned_amount_label()).0)]
    pub burned_amount_value: UncheckedAccount<'info>,
    /// The existing encrypted amount to burn: a computed or received `euint64` handle. Read-only
    /// durable operand — never superseded, never consumed. Its address is the canonical PDA of its
    /// own `(acl_domain_key, app_account, encrypted_value_label)` fields, so a lineage from any app
    /// may be passed here once its owner has granted the mint's compute subject via `allow_subjects`.
    pub amount_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it against the
    /// canonical `["hcu-block-meter", compute_signer]` PDA. Supplied by an untrusted mint under a
    /// metering-band cap; omitted when the mint is trusted or the cap is unrestricted.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it against the
    /// canonical `["hcu-trusted", compute_signer]` PDA. Present + valid bypasses the cap; absent
    /// means the mint is metered.
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
}

impl<'info> ConfidentialBurnFromValue<'info> {
    fn as_burn_accounts<'a>(
        &'a self,
        remaining_accounts: &'a [AccountInfo<'info>],
    ) -> BurnAccounts<'a, 'info> {
        BurnAccounts {
            payer: &self.payer,
            burn_authority: self.owner.key(),
            mint: &self.mint,
            token_account: &self.token_account,
            compute_signer: &self.compute_signer,
            total_supply_authority: &self.total_supply_authority,
            balance_value: self.balance_value.to_account_info(),
            total_supply_value: self.total_supply_value.to_account_info(),
            burned_amount_value: self.burned_amount_value.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            deny_subject_records: remaining_accounts,
            system_program: &self.system_program,
            hcu_block_meter: self
                .hcu_block_meter
                .as_ref()
                .map(|account| account.to_account_info()),
            hcu_trusted_app_record: self
                .hcu_trusted_app_record
                .as_ref()
                .map(|account| account.to_account_info()),
        }
    }
}

/// Burns an encrypted amount taken from an existing on-chain `EncryptedValue` (a computed or
/// received handle), rotating the account balance and encrypted total supply. The amount value is
/// spent read-only, and the burned-amount output is born publicly decryptable exactly as in the
/// attestation path, so `redeem_burned_amount` consumes it unchanged.
pub fn confidential_burn_from_value<'info>(
    ctx: Context<'info, ConfidentialBurnFromValue<'info>>,
) -> Result<()> {
    let amount_value = &ctx.accounts.amount_value;
    // Reject a non-euint64 amount early for a clear error, before the eval builder / host would
    // reject the same handle deeper in the CPI (the host's binary type validation still covers it).
    require!(
        zama_host::handle_fhe_type(amount_value.current_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    // Token-level spend gate — EVM `FHE.isAllowed(amount, msg.sender)` parity: the signing owner
    // must be in the amount value's subject set. App-level by design; the host stays role-blind.
    require!(
        amount_value.has_subject(ctx.accounts.owner.key()),
        ConfidentialTokenError::AmountSpendSubjectMismatch
    );
    let amount_handle = amount_value.current_handle;
    let amount_value_info = amount_value.to_account_info();
    let outcome = execute_burn(
        ctx.accounts.as_burn_accounts(ctx.remaining_accounts),
        ctx.bumps.compute_signer,
        BurnAmountSource::ExistingValue {
            amount_value: amount_value_info,
            amount_handle,
        },
    )?;
    emit_cpi!(ConfidentialBurnEvent {
        version: APP_EVENT_VERSION,
        mint: outcome.mint,
        owner: outcome.owner,
        token_account: outcome.token_account,
        burned_handle: outcome.burned_handle,
        burned_encrypted_value: outcome.burned_encrypted_value,
    });
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: outcome.mint,
        owner: outcome.owner,
        token_account: outcome.token_account,
        old_handle: outcome.old_balance_handle,
        old_encrypted_value: outcome.balance_encrypted_value,
        new_handle: outcome.new_balance_handle,
        new_encrypted_value: outcome.balance_encrypted_value,
        reason: BalanceHandleUpdateReason::BurnDebit,
    });
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: outcome.mint,
        old_handle: outcome.old_total_supply_handle,
        old_encrypted_value: outcome.total_supply_encrypted_value,
        new_handle: outcome.new_total_supply_handle,
        new_encrypted_value: outcome.total_supply_encrypted_value,
        reason: TotalSupplyUpdateReason::Burn,
    });
    Ok(())
}

/// Where a burn's amount comes from. The `ge -> sub -> select` debit, the born-public `burned` delta,
/// and the total-supply decrement are identical for both arms; only how the amount operand enters the
/// eval frame differs. Mirrors [`TransferAmountSource`].
enum BurnAmountSource<'info> {
    /// EVM `FHE.fromExternal` parity: a coprocessor-attested fresh client-side encryption, verified
    /// in-frame and transient-allowed for this eval (no durable amount account).
    Attested(zama_host::CoprocessorInputAttestation),
    /// EVM computed/received `euint64` parity: an existing on-chain `EncryptedValue` lineage, spent
    /// as a read-only durable operand at its current handle. It is never superseded and never
    /// consumed. The token spend gate (signing owner in the value's subject set) and euint64 type
    /// check run in the instruction handler before this reaches the eval builder; the host re-checks
    /// the handle is current and that the mint's compute subject is allowed on the value, in-frame.
    ExistingValue {
        amount_value: AccountInfo<'info>,
        amount_handle: [u8; 32],
    },
}

impl BurnAmountSource<'_> {
    fn amount_handle(&self) -> [u8; 32] {
        match self {
            Self::Attested(attestation) => attestation.input_handle,
            Self::ExistingValue { amount_handle, .. } => *amount_handle,
        }
    }

    /// Domain-separates the eval context id per arm so the two amount sources never derive colliding
    /// handles for the same (mint, token account, amount handle) tuple. The attested arm keeps the
    /// original `burn-balance` tag so its handle derivation is byte-for-byte unchanged.
    fn context_tag(&self) -> &'static [u8] {
        match self {
            Self::Attested(_) => b"burn-balance",
            Self::ExistingValue { .. } => b"burn-from-value",
        }
    }
}

/// Fixed ZamaHost CPI accounts and burn operands shared by the attested and existing-value arms.
struct BurnAccounts<'a, 'info> {
    /// Rent payer for the burned-amount lineage's first bind (the owner in the attested arm, an
    /// independent signer in the from-value arm so the owner may be a PDA).
    payer: &'a Signer<'info>,
    /// Token owner and burn authority (the signing owner's key).
    burn_authority: Pubkey,
    mint: &'a Account<'info, ConfidentialMint>,
    token_account: &'a Account<'info, ConfidentialTokenAccount>,
    compute_signer: &'a UncheckedAccount<'info>,
    total_supply_authority: &'a UncheckedAccount<'info>,
    /// Stable balance lineage: read for the current handle, then superseded in place as the output.
    balance_value: AccountInfo<'info>,
    /// Stable total-supply lineage: read for the current handle, then superseded in place.
    total_supply_value: AccountInfo<'info>,
    /// Stable burned-amount lineage: superseded to this burn's born-public delta.
    burned_amount_value: AccountInfo<'info>,
    zama_event_authority: &'a UncheckedAccount<'info>,
    zama_program: &'a Program<'info, ZamaHost>,
    host_config: &'a Account<'info, zama_host::HostConfig>,
    deny_subject_records: &'a [AccountInfo<'info>],
    system_program: &'a Program<'info, System>,
    hcu_block_meter: Option<AccountInfo<'info>>,
    hcu_trusted_app_record: Option<AccountInfo<'info>>,
}

/// Everything the burn handlers need to emit their app-local history events.
struct BurnOutcome {
    mint: Pubkey,
    owner: Pubkey,
    token_account: Pubkey,
    burned_handle: [u8; 32],
    burned_encrypted_value: Pubkey,
    old_balance_handle: [u8; 32],
    new_balance_handle: [u8; 32],
    balance_encrypted_value: Pubkey,
    old_total_supply_handle: [u8; 32],
    new_total_supply_handle: [u8; 32],
    total_supply_encrypted_value: Pubkey,
}

fn execute_burn<'info>(
    accounts: BurnAccounts<'_, 'info>,
    compute_signer_bump: u8,
    amount_source: BurnAmountSource<'info>,
) -> Result<BurnOutcome> {
    assert_confidential_mint_shape(accounts.mint)?;
    let mint_key = accounts.mint.key();
    let compute_signer = accounts.mint.compute_signer;
    let total_supply_authority = accounts.total_supply_authority.key();
    let token_account = accounts.token_account;
    let owner = token_account.owner;
    let token_account_key = token_account.key();
    let old_balance_handle = fhe::read_encrypted_value(&accounts.balance_value)?.current_handle;
    let old_total_supply_handle =
        fhe::read_encrypted_value(&accounts.total_supply_value)?.current_handle;

    require_keys_eq!(
        owner,
        accounts.burn_authority,
        ConfidentialTokenError::OwnerMismatch
    );
    require_keys_eq!(
        token_account.mint,
        mint_key,
        ConfidentialTokenError::MintMismatch
    );
    assert_confidential_token_account_shape(token_account, mint_key, owner)?;
    require_keys_eq!(
        accounts.compute_signer.key(),
        compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    require_keys_eq!(
        total_supply_authority,
        total_supply_authority_address(mint_key).0,
        ConfidentialTokenError::TotalSupplyAuthorityMismatch
    );

    if let BurnAmountSource::Attested(amount_attestation) = &amount_source {
        // fromExternal parity: the burn amount is a coprocessor-attested external input authored by
        // the owner and bound to the mint compute-signer PDA (see assert_amount_attestation_binding).
        // The `ExistingValue` arm is gated instead by the token spend gate and euint64 type check in
        // its instruction handler.
        assert_amount_attestation_binding(amount_attestation, owner, compute_signer)?;
    }

    let balance_output = fhe::DurableOutput::new(
        accounts.balance_value.clone(),
        durable_slot(mint_key, token_account_key, balance_label()),
        fhe::DurableAudience::for_owner(owner, compute_signer),
    )?;
    // ERC-7984 `unwrap` parity (`makePubliclyDecryptable(unwrapAmount)`): the burned delta is born
    // publicly decryptable inside this eval CPI, so the burn is permanently redeemable even after a
    // later burn supersedes this shared lineage (DD-036 / Vector 2) — with no second make-public CPI.
    let burned_output = fhe::DurableOutput::new_public(
        accounts.burned_amount_value.clone(),
        durable_slot(mint_key, token_account_key, burned_amount_label()),
        fhe::DurableAudience::for_owner(owner, compute_signer),
    )?;
    let total_supply_output = fhe::DurableOutput::new(
        accounts.total_supply_value.clone(),
        durable_slot(mint_key, total_supply_authority, total_supply_label()),
        fhe::DurableAudience::compute_only(compute_signer),
    )?;

    let balance = uint64_from_value(
        old_balance_handle,
        mint_key,
        token_account_key,
        balance_label(),
    )?;
    let total_supply = uint64_from_value(
        old_total_supply_handle,
        mint_key,
        total_supply_authority,
        total_supply_label(),
    )?;
    let context_id = transfer_eval_context(
        amount_source.context_tag(),
        mint_key,
        token_account_key,
        token_account_key,
        amount_source.amount_handle(),
    )?;
    let mut builder = zama_fhe::EvalBuilder::new(
        context_id,
        zama_fhe::EvalAppAuthority::new(token_account_key),
    );
    let amount: zama_fhe::Uint64Handle = match &amount_source {
        // fromExternal: the amount is a coprocessor-attested external input, verified in-frame and
        // transient-allowed for this eval (no durable amount handle / ACL account).
        BurnAmountSource::Attested(amount_attestation) => builder
            .verified_input(amount_attestation.clone())
            .map_err(invalid_eval_plan)?,
        // Existing value: the amount is an on-chain lineage's current handle, read as a durable
        // operand. The slot is derived from the value's own canonical fields, so its PDA equals the
        // passed account; the host re-checks handle-is-current and compute-subject membership.
        BurnAmountSource::ExistingValue { amount_value, .. } => {
            let value = fhe::read_encrypted_value(amount_value)?;
            uint64_from_value(
                value.current_handle,
                value.acl_domain_key,
                value.app_account,
                value.encrypted_value_label,
            )?
        }
    };
    let burn_success = builder
        .ge(balance, amount, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let debit_candidate = builder
        .sub(balance, amount, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let new_balance = builder
        .if_then_else(
            burn_success,
            debit_candidate,
            balance,
            balance_output.output(),
        )
        .map_err(invalid_eval_plan)?;
    let burned = builder
        .sub(balance, new_balance, burned_output.output())
        .map_err(invalid_eval_plan)?;
    builder
        .sub(total_supply, burned, total_supply_output.output())
        .map_err(invalid_eval_plan)?;
    let plan = builder.finish().map_err(invalid_eval_plan)?;
    let compute_authority =
        fhe::ComputeAuthority::for_mint(accounts.compute_signer, mint_key, compute_signer_bump)?;
    let total_supply_authority_bump = total_supply_authority_address(mint_key).1;
    // Durable output accounts are the same for both arms; the existing-value arm adds the amount
    // lineage as a read-only durable input operand the plan now requires.
    let mut dynamic_accounts = vec![
        balance_output.account_info(),
        burned_output.account_info(),
        total_supply_output.account_info(),
    ];
    if let BurnAmountSource::ExistingValue { amount_value, .. } = &amount_source {
        // The amount lineage can legitimately alias one of the output accounts (burning the entire
        // balance aliases the balance lineage; re-burning a burned_amount aliases the burned output).
        // The plan already merges those into one slot, so only add the amount when it is a distinct
        // account — pushing a duplicate would trip eval account resolution (the #3238 aliasing class).
        if !dynamic_accounts
            .iter()
            .any(|account| account.key() == amount_value.key())
        {
            dynamic_accounts.push(amount_value.clone());
        }
    }
    let eval_accounts = fhe::EvalAccountSet::for_plan(
        &plan,
        dynamic_accounts,
        [
            fhe::OutputAuthority::token_account(token_account)?,
            fhe::OutputAuthority::total_supply(
                accounts.total_supply_authority,
                mint_key,
                total_supply_authority_bump,
            )?,
        ],
    )?;
    fhe::eval(fhe::Eval {
        context: fhe::EvalContext {
            payer: accounts.payer,
            event_authority: accounts.zama_event_authority,
            zama_program: accounts.zama_program,
            host_config: accounts.host_config,
            deny_subject_records: accounts.deny_subject_records,
            compute_authority,
            system_program: accounts.system_program,
            hcu_block_meter: accounts.hcu_block_meter.clone(),
            hcu_trusted_app_record: accounts.hcu_trusted_app_record.clone(),
        },
        accounts: &eval_accounts,
        plan,
    })?;

    Ok(BurnOutcome {
        mint: mint_key,
        owner,
        token_account: token_account_key,
        burned_handle: burned_output.handle()?,
        burned_encrypted_value: accounts.burned_amount_value.key(),
        old_balance_handle,
        new_balance_handle: balance_output.handle()?,
        balance_encrypted_value: accounts.balance_value.key(),
        old_total_supply_handle,
        new_total_supply_handle: total_supply_output.handle()?,
        total_supply_encrypted_value: accounts.total_supply_value.key(),
    })
}
