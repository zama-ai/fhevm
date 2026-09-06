//! Shared confidential-token instruction helpers.
//!
//! This module holds cross-instruction account shape checks, FHE CPI builders,
//! and deterministic labels used by the token handlers.

use super::*;

pub(crate) fn assert_no_remaining_accounts(remaining_accounts: &[AccountInfo]) -> Result<()> {
    require!(
        remaining_accounts.is_empty(),
        ConfidentialTokenError::UnexpectedRemainingAccounts
    );
    Ok(())
}

pub(crate) struct TransferAccounts<'a, 'info> {
    pub(crate) payer: &'a Signer<'info>,
    /// The sender's signing owner: the transfer authority, and the value authority of an
    /// existing amount it controls itself.
    pub(crate) transfer_authority: &'a Signer<'info>,
    pub(crate) mint: &'a Account<'info, ConfidentialMint>,
    pub(crate) from_account: &'a Account<'info, ConfidentialTokenAccount>,
    pub(crate) to_account: &'a Account<'info, ConfidentialTokenAccount>,
    /// Sender's stable balance encrypted value account: read for the current handle, then
    /// replaced in place as the output.
    pub(crate) from_balance_value: AccountInfo<'info>,
    /// Recipient's stable balance encrypted value account: read for the current handle, then
    /// replaced in place as the output.
    pub(crate) to_balance_value: AccountInfo<'info>,
    /// Sender's stable transferred-amount encrypted value account, replaced every transfer.
    pub(crate) transferred_amount_value: AccountInfo<'info>,
    pub(crate) zama_event_authority: &'a UncheckedAccount<'info>,
    pub(crate) zama_program: &'a Program<'info, ZamaHost>,
    pub(crate) host_config: &'a Account<'info, zama_host::HostConfig>,
    /// The instruction's remaining accounts: the mint's deny record while the deny list is on.
    pub(crate) remaining_accounts: &'a [AccountInfo<'info>],
    pub(crate) system_program: &'a Program<'info, System>,
    /// Per-mint HCU block meter forwarded into the host `fhe_execute` CPI (`None` = untrusted,
    /// no meter). The host keys the meter on the application `(program, mint)`.
    pub(crate) hcu_block_meter: Option<AccountInfo<'info>>,
    /// HCU trust witness forwarded into the host `fhe_execute` CPI (`None` = untrusted).
    pub(crate) hcu_trusted_app_record: Option<AccountInfo<'info>>,
    /// Underlying SPL mint whose owner is the token program. Freeze checks read this account.
    pub(crate) underlying_mint: AccountInfo<'info>,
    /// ATA of `from_account.owner` on `underlying_mint`.
    pub(crate) from_ata: AccountInfo<'info>,
    /// ATA of `to_account.owner` on `underlying_mint`. May alias `from_ata` on self-transfer.
    pub(crate) to_ata: AccountInfo<'info>,
    /// A receipt of the transferred amount written for the recipient program, when it asked.
    pub(crate) receipt: Option<ReceiptAccounts<'info>>,
}

/// How a program receiving a transfer learns the amount it received: the transfer's execution
/// also writes `receipt = receipt + transferred` (first time `transferred + 0`) into a value of
/// that program, controlled by the PDA it signs the CPI with. A value's authority is the only
/// key that may compute on it, so the recipient cannot read `transferred_amount` itself; the
/// token program writes the sum into the recipient's own value instead, in the same execution
/// that computes it. EVM analog: `FHE.allow(transferred, recipient)` followed by the recipient's
/// own `add`.
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct TransferReceipt {
    /// The recipient program.
    pub program: Pubkey,
    /// The recipient program's scope for the value.
    pub scope: [u8; 32],
    /// The value's label under `(program, receipt_authority, scope)`.
    pub label: [u8; 32],
    /// Seeds (bump last) deriving `receipt_authority` under `program`; proven by the host on
    /// the first write.
    pub authority_seeds: Vec<Vec<u8>>,
    /// Keys allowed to decrypt the receipt's new handle.
    pub allows: Vec<Pubkey>,
}

pub(crate) struct ReceiptAccounts<'info> {
    pub(crate) value: AccountInfo<'info>,
    pub(crate) authority: fhe::ValueAuthority<'info>,
    pub(crate) key: zama_fhe::EncryptedValueId,
    pub(crate) allows: Vec<Pubkey>,
}

impl<'info> ReceiptAccounts<'info> {
    /// Binds the optional receipt: both accounts and the descriptor together, or none of them.
    pub(crate) fn bind(
        receipt: Option<TransferReceipt>,
        value: Option<&UncheckedAccount<'info>>,
        authority: Option<&Signer<'info>>,
    ) -> Result<Option<Self>> {
        match (receipt, value, authority) {
            (None, None, None) => Ok(None),
            (Some(receipt), Some(value), Some(authority)) => {
                let key = zama_fhe::EncryptedValueId::new(
                    zama_fhe::AppScope {
                        program: receipt.program,
                        scope: receipt.scope,
                    },
                    authority.key(),
                    zama_fhe::EncryptedValueLabel::new(receipt.label),
                );
                Ok(Some(Self {
                    value: value.to_account_info(),
                    authority: fhe::ValueAuthority::foreign(
                        authority.to_account_info(),
                        receipt.authority_seeds,
                    )?,
                    key,
                    allows: receipt.allows,
                }))
            }
            _ => err!(ConfidentialTokenError::TransferReceiptMismatch),
        }
    }
}

/// Where a transfer's amount comes from. The `ge -> sub -> select` debit and `add` credit that
/// move the two balance encrypted value accounts are identical for both arms; only how the amount operand enters
/// the execution differs.
pub(crate) enum TransferAmountSource<'info> {
    /// EVM `FHE.fromExternal` parity: a coprocessor-attested fresh client-side encryption,
    /// verified in-execution and transient-allowed for this execution (no persistent amount account).
    Attested(zama_host::CoprocessorInputAttestation),
    /// EVM computed `euint64` parity: an existing on-chain `EncryptedValue` account, spent as a
    /// read-only persistent operand at its current handle. It is never replaced and never
    /// consumed — only the two balance encrypted value accounts change. The token's spend gate
    /// (the signing owner controls the value, or owns the token account that does) and euint64
    /// type check run in the instruction handler before this reaches the execution builder; the
    /// host re-checks the handle is current and that the value's authority signed, in-execution.
    ExistingValue { amount_value: AccountInfo<'info> },
}

pub(crate) struct TransferOutcome {
    pub(crate) mint: Pubkey,
    pub(crate) from_owner: Pubkey,
    pub(crate) from_token_account: Pubkey,
    pub(crate) old_from_handle: [u8; 32],
    pub(crate) new_from_handle: [u8; 32],
    pub(crate) from_encrypted_value: Pubkey,
    pub(crate) transferred_handle: [u8; 32],
    pub(crate) transferred_encrypted_value: Pubkey,
    pub(crate) to_owner: Pubkey,
    pub(crate) to_token_account: Pubkey,
    pub(crate) old_to_handle: [u8; 32],
    pub(crate) new_to_handle: [u8; 32],
    pub(crate) to_encrypted_value: Pubkey,
}

#[inline(never)]
pub(crate) fn execute_transfer<'info>(
    accounts: TransferAccounts<'_, 'info>,
    amount_source: TransferAmountSource<'info>,
) -> Result<Option<TransferOutcome>> {
    assert_confidential_mint_shape(accounts.mint)?;
    let mint_key = accounts.mint.key();
    let from = accounts.from_account;
    let to = accounts.to_account;

    if let TransferAmountSource::Attested(amount_attestation) = &amount_source {
        // EVM `fromExternal` parity for the amount: the attested input must be authored by the
        // sender (user) and bound to this program (the `msg.sender`/contract analog the host
        // re-checks against the execution's application). The coprocessor signature over both is
        // verified in-execution. The `ExistingValue` arm is gated instead by the token spend gate
        // and euint64 type check in its instruction handler.
        assert_amount_attestation_binding(amount_attestation, accounts.transfer_authority.key())?;
    }
    require_keys_eq!(from.mint, mint_key, ConfidentialTokenError::MintMismatch);
    require_keys_eq!(to.mint, mint_key, ConfidentialTokenError::MintMismatch);
    assert_confidential_token_account_shape(from, mint_key, from.owner)?;
    assert_confidential_token_account_shape(to, mint_key, to.owner)?;
    check_underlying_ata_not_frozen(
        accounts.mint,
        from.owner,
        &accounts.underlying_mint,
        &accounts.from_ata,
    )?;
    check_underlying_ata_not_frozen(
        accounts.mint,
        to.owner,
        &accounts.underlying_mint,
        &accounts.to_ata,
    )?;
    require_keys_eq!(
        accounts.from_balance_value.key(),
        from.balance_encrypted_value,
        ConfidentialTokenError::CurrentEncryptedValueMismatch
    );
    require_keys_eq!(
        accounts.to_balance_value.key(),
        to.balance_encrypted_value,
        ConfidentialTokenError::CurrentEncryptedValueMismatch
    );
    let from_key = from.key();
    let to_key = to.key();
    let from_owner = from.owner;
    let to_owner = to.owner;
    let from_encrypted_value = accounts.from_balance_value.key();
    let to_encrypted_value = accounts.to_balance_value.key();
    if from_key == to_key {
        assert_no_remaining_accounts(accounts.remaining_accounts)?;
        return Ok(None);
    }

    let from_balance = fhe::read_encrypted_value(&accounts.from_balance_value)?;
    let to_balance = fhe::read_encrypted_value(&accounts.to_balance_value)?;
    let old_from_handle = from_balance.current_handle;
    let old_to_handle = to_balance.current_handle;

    let (new_from_handle, transferred_handle, new_to_handle) = compute_transfer_handles(
        &accounts,
        &amount_source,
        mint_key,
        &from_balance,
        &to_balance,
    )?;

    let transferred_encrypted_value = accounts.transferred_amount_value.key();

    Ok(Some(TransferOutcome {
        mint: mint_key,
        from_owner,
        from_token_account: from_key,
        old_from_handle,
        new_from_handle,
        from_encrypted_value,
        transferred_handle,
        transferred_encrypted_value,
        to_owner,
        to_token_account: to_key,
        old_to_handle,
        new_to_handle,
        to_encrypted_value,
    }))
}

fn compute_transfer_handles<'info>(
    accounts: &TransferAccounts<'_, 'info>,
    amount_source: &TransferAmountSource<'info>,
    mint_key: Pubkey,
    from_balance_value: &zama_host::EncryptedValue,
    to_balance_value: &zama_host::EncryptedValue,
) -> Result<([u8; 32], [u8; 32], [u8; 32])> {
    let from_key = accounts.from_account.key();
    let to_key = accounts.to_account.key();
    let from_owner = accounts.from_account.owner;
    let to_owner = accounts.to_account.owner;
    let from_balance = fhe::uint64_operand(from_balance_value)?;
    let to_balance = fhe::uint64_operand(to_balance_value)?;
    let from_authority = fhe::ValueAuthority::token_account(accounts.from_account)?;
    let to_authority = fhe::ValueAuthority::token_account(accounts.to_account)?;
    // Each write allows its holder on the new handle; the transferred amount is allowed to both
    // parties so the recipient can decrypt what they received.
    let from_output = fhe::PersistentOutput::new(
        accounts.from_balance_value.clone(),
        balance_encrypted_value_id(mint_key, from_key),
        &from_authority,
        [from_owner],
    )?;
    let transferred_output = fhe::PersistentOutput::new(
        accounts.transferred_amount_value.clone(),
        token_value_id(mint_key, from_key, encrypted_transferred_amount_label()),
        &from_authority,
        std::iter::once(from_owner).chain((to_owner != from_owner).then_some(to_owner)),
    )?;
    let to_output = fhe::PersistentOutput::new(
        accounts.to_balance_value.clone(),
        balance_encrypted_value_id(mint_key, to_key),
        &to_authority,
        [to_owner],
    )?;
    // The recipient program's receipt accumulates what this transfer moved; its previous total
    // is an operand only once the value exists.
    let receipt_output = accounts
        .receipt
        .as_ref()
        .map(|receipt| {
            let previous = (*receipt.value.owner != System::id())
                .then(|| fhe::read_encrypted_value(&receipt.value))
                .transpose()?
                .as_ref()
                .map(fhe::uint64_operand)
                .transpose()?;
            let output = fhe::PersistentOutput::new(
                receipt.value.clone(),
                receipt.key.clone(),
                &receipt.authority,
                receipt.allows.iter().copied(),
            )?;
            Ok::<_, Error>((output, previous))
        })
        .transpose()?;
    // Existing value: the amount is an on-chain encrypted value account's current handle, read as
    // a persistent operand named by the value's own canonical fields, so its PDA equals the passed
    // account; the host re-checks handle-is-current and the authority's signature. Read here
    // rather than inside the execution closure: a stored value belongs to no builder, and reading
    // the account is this program's error to report, not the builder's.
    let stored_amount = match amount_source {
        TransferAmountSource::Attested(_) => None,
        TransferAmountSource::ExistingValue { amount_value } => {
            Some(fhe::read_encrypted_value(amount_value)?)
        }
    };
    let stored_operand = stored_amount
        .as_ref()
        .map(fhe::uint64_operand)
        .transpose()?;
    let execution = zama_fhe::FheExecution::build(
        zama_fhe::ExecutionEncryptedValueAccountAuthority::new(from_key),
        |builder| {
            let amount = match (amount_source, stored_operand) {
                // fromExternal: the amount is a coprocessor-attested external input, verified
                // in-execution and transient-allowed for this execution (no persistent amount
                // handle / account).
                (TransferAmountSource::Attested(amount_attestation), _) => {
                    builder.verified_input(amount_attestation.clone())?
                }
                (_, Some(stored)) => stored.into(),
                (TransferAmountSource::ExistingValue { .. }, None) => {
                    unreachable!("an existing-value transfer always reads its stored amount above")
                }
            };
            let success = builder.ge(from_balance, amount, zama_fhe::Output::transient())?;
            let debit_candidate =
                builder.sub(from_balance, amount, zama_fhe::Output::transient())?;
            let new_from = builder.if_then_else(
                success,
                debit_candidate,
                from_balance,
                zama_fhe::Output::transient(),
            )?;
            let transferred = builder.sub(from_balance, new_from, transferred_output.output())?;
            builder.add(
                new_from,
                zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(0),
                from_output.output(),
            )?;
            builder.add(to_balance, transferred, to_output.output())?;
            if let Some((receipt, previous)) = &receipt_output {
                match previous {
                    Some(previous) => builder.add(*previous, transferred, receipt.output())?,
                    None => builder.add(
                        transferred,
                        zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(0),
                        receipt.output(),
                    )?,
                };
            }
            Ok(())
        },
    )
    .map_err(invalid_execution)?;
    // Persistent output accounts are the same for both arms; the existing-value arm adds the
    // amount encrypted value account as a read-only persistent input operand the execution now
    // requires, and its authority's signature when the signing owner controls it directly.
    let mut dynamic_accounts = vec![
        from_output.account_info(),
        transferred_output.account_info(),
        to_output.account_info(),
    ];
    let mut value_authorities = vec![from_authority, to_authority];
    if let (Some(receipt), Some((receipt_output, _))) = (&accounts.receipt, &receipt_output) {
        dynamic_accounts.push(receipt_output.account_info());
        value_authorities.push(receipt.authority.clone());
    }
    if let (TransferAmountSource::ExistingValue { amount_value }, Some(stored)) =
        (amount_source, &stored_amount)
    {
        // The amount encrypted value account can legitimately alias one of the output accounts
        // (spending the entire balance, or re-sending a transferred_amount that is also this
        // execution's output). The execution already merges those into one slot, so only add the
        // amount when it is a distinct account.
        if !dynamic_accounts
            .iter()
            .any(|account| account.key() == amount_value.key())
        {
            dynamic_accounts.push(amount_value.clone());
        }
        if stored.encrypted_value_account_authority == accounts.transfer_authority.key() {
            value_authorities.push(fhe::ValueAuthority::external(
                accounts.transfer_authority.to_account_info(),
            ));
        }
    }
    let execution_accounts =
        fhe::ExecutionAccountSet::for_execution(&execution, dynamic_accounts, value_authorities)?;

    fhe::execute(fhe::Execute {
        context: fhe::ExecuteContext {
            payer: accounts.payer,
            event_authority: accounts.zama_event_authority,
            zama_program: accounts.zama_program,
            host_config: accounts.host_config,
            deny_scope_records: fhe::deny_scope_records(
                accounts.host_config,
                accounts.remaining_accounts,
                std::iter::once(token_app(mint_key))
                    .chain(accounts.receipt.as_ref().map(|receipt| receipt.key.app())),
            )?,
            system_program: accounts.system_program,
            hcu_block_meter: accounts.hcu_block_meter.clone(),
            hcu_trusted_app_record: accounts.hcu_trusted_app_record.clone(),
        },
        accounts: &execution_accounts,
        execution,
    })?;

    Ok((
        from_output.handle()?,
        transferred_output.handle()?,
        to_output.handle()?,
    ))
}

/// Re-writes `value` onto a fresh handle of the same amount (`value + 0`) allowed to `allows`,
/// signed by `authority`: the one-step execution behind `allow_balance_viewers` and
/// `allow_total_supply_viewers`. Returns the new handle.
pub(crate) fn rewrite_allowing<'info>(
    context: fhe::ExecuteContext<'_, 'info>,
    value: &Account<'info, zama_host::EncryptedValue>,
    id: zama_fhe::EncryptedValueId,
    authority: fhe::ValueAuthority<'info>,
    allows: impl IntoIterator<Item = Pubkey>,
) -> Result<[u8; 32]> {
    let operand = fhe::uint64_operand(value)?;
    let output = fhe::PersistentOutput::new(value.to_account_info(), id, &authority, allows)?;
    let execution = zama_fhe::FheExecution::build(
        zama_fhe::ExecutionEncryptedValueAccountAuthority::new(
            value.encrypted_value_account_authority,
        ),
        |builder| {
            builder.add(
                operand,
                zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(0),
                output.output(),
            )?;
            Ok(())
        },
    )
    .map_err(invalid_execution)?;
    let accounts =
        fhe::ExecutionAccountSet::for_execution(&execution, [output.account_info()], [authority])?;
    fhe::execute(fhe::Execute {
        context,
        accounts: &accounts,
        execution,
    })?;
    output.handle()
}

pub(crate) fn invalid_execution(
    error: zama_fhe::FheExecutionBuildError,
) -> anchor_lang::error::Error {
    msg!("invalid FHE execution: {:?}", error);
    error!(ConfidentialTokenError::InvalidFheExecution)
}

/// Validates a coprocessor-attested transfer/burn amount (EVM `fromExternal` parity). The host
/// re-verifies the attestation signature and enforces `contract_address == program` in-execution;
/// the program binds the attested identities to this transaction: the input must be authored by
/// `expected_user` (the sender/burner) and bound to this program. The amount handle must be a
/// confidential balance.
pub(crate) fn assert_amount_attestation_binding(
    attestation: &zama_host::CoprocessorInputAttestation,
    expected_user: Pubkey,
) -> Result<()> {
    require!(
        zama_host::handle_fhe_type(attestation.input_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require_keys_eq!(
        Pubkey::new_from_array(attestation.user_address),
        expected_user,
        ConfidentialTokenError::AttestationUserMismatch
    );
    require_keys_eq!(
        Pubkey::new_from_array(attestation.contract_address),
        crate::ID,
        ConfidentialTokenError::AttestationContractMismatch
    );
    Ok(())
}

/// The token spend gate for an existing amount value — EVM `FHE.isAllowed(amount, msg.sender)`
/// parity, app-level by design. Computing on a value is admitted by its authority's signature,
/// so the spender must be that authority (another program's PDA signing through
/// `invoke_signed`), or own the token account whose values this program signs for. The amount
/// must be a confidential balance type.
pub(crate) fn assert_amount_value_spendable(
    amount_value: &zama_host::EncryptedValue,
    spender: Pubkey,
    spender_token_account: Pubkey,
) -> Result<()> {
    require!(
        zama_host::handle_fhe_type(amount_value.current_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        amount_value.encrypted_value_account_authority == spender
            || amount_value.encrypted_value_account_authority == spender_token_account,
        ConfidentialTokenError::AmountSpendAuthorityMismatch
    );
    Ok(())
}

/// Binds an encrypted value to one exact token state field: this program's application for
/// `mint`, the controlling PDA, and the field label.
pub(crate) fn assert_token_value(
    value: &zama_host::EncryptedValue,
    mint: Pubkey,
    authority: Pubkey,
    label: [u8; 32],
) -> Result<()> {
    require!(
        value.program == crate::ID
            && value.scope == mint.to_bytes()
            && value.encrypted_value_account_authority == authority
            && value.label == label,
        ConfidentialTokenError::TokenEncryptedValueMismatch
    );
    Ok(())
}

/// Encrypted value account checks for the redeem path: burned-amount handle type and the exact
/// token state field. The caller separately requires the pending handle to be current and proves
/// its publicness through the exact-handle MMR proof verified by `verify_public_decrypt`.
pub(crate) fn assert_burned_amount_value_account(
    amount_value: &Account<zama_host::EncryptedValue>,
    burned_handle: [u8; 32],
    mint: Pubkey,
    token_account: Pubkey,
) -> Result<()> {
    require!(
        zama_host::handle_fhe_type(burned_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    assert_token_value(
        amount_value,
        mint,
        token_account,
        encrypted_burned_amount_label(),
    )
    .map_err(|_| error!(ConfidentialTokenError::AmountAclMismatch))?;
    require_keys_eq!(
        amount_value.key(),
        encrypted_value_address(mint, token_account, encrypted_burned_amount_label()).0,
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

/// Encodes a u64 cleartext as the 32-byte big-endian (abi `uint256`) decrypted result
/// the KMS signs over in the `PublicDecryptVerification` certificate (cert-secp path).
pub(crate) fn kms_decrypted_result_bytes(cleartext_amount: u64) -> [u8; 32] {
    let mut decrypted = [0u8; 32];
    decrypted[24..].copy_from_slice(&cleartext_amount.to_be_bytes());
    decrypted
}

pub(crate) fn assert_host_config_allows_token_response(
    host_config: &Account<zama_host::HostConfig>,
) -> Result<()> {
    let (expected_key, expected_bump) = zama_host::host_config_address();
    require_keys_eq!(
        host_config.key(),
        expected_key,
        ConfidentialTokenError::RequestWitnessMismatch
    );
    require!(
        host_config.to_account_info().data_len() == 8 + zama_host::HostConfig::SPACE
            && host_config.bump == expected_bump
            && !host_config.paused,
        ConfidentialTokenError::RequestWitnessUnavailable
    );
    Ok(())
}

pub(crate) fn assert_canonical_vault_token_account(
    vault_usdc: Pubkey,
    vault_authority: Pubkey,
    underlying_mint: Pubkey,
    token_program: Pubkey,
) -> Result<()> {
    require_keys_eq!(
        vault_usdc,
        get_associated_token_address_with_program_id(
            &vault_authority,
            &underlying_mint,
            &token_program,
        ),
        ConfidentialTokenError::VaultAccountMismatch
    );
    Ok(())
}

/// Accepts the classic token program and extension-free Token-2022 mints. Extension behavior can
/// change transfer amounts or invoke external programs, so unsupported extensions fail closed.
pub(crate) fn assert_supported_underlying_mint(
    mint: &InterfaceAccount<SplMint>,
    token_program: &Interface<TokenInterface>,
) -> Result<()> {
    require_keys_eq!(
        *mint.to_account_info().owner,
        token_program.key(),
        ConfidentialTokenError::UnderlyingTokenProgramMismatch
    );
    if token_program.key() == anchor_spl::token_2022::ID {
        use anchor_spl::token_interface::spl_token_2022::extension::{
            BaseStateWithExtensions, StateWithExtensions,
        };
        let mint_info = mint.to_account_info();
        let data = mint_info.try_borrow_data()?;
        let state = StateWithExtensions::<
            anchor_spl::token_interface::spl_token_2022::state::Mint,
        >::unpack(&data)
        .map_err(|_| error!(ConfidentialTokenError::UnsupportedToken2022Extension))?;
        require!(
            state
                .get_extension_types()
                .map_err(|_| error!(ConfidentialTokenError::UnsupportedToken2022Extension))?
                .is_empty(),
            ConfidentialTokenError::UnsupportedToken2022Extension
        );
    }
    Ok(())
}

/// Validates an underlying token account against the selected token program. Token-2022 ATAs may
/// carry only `ImmutableOwner`; all amount-affecting or callback extensions are rejected by the
/// extension-free mint rule above and this account-side fail-closed check.
pub(crate) fn assert_supported_underlying_token_account(
    account: &InterfaceAccount<TokenAccount>,
    token_program: &Interface<TokenInterface>,
) -> Result<()> {
    require_keys_eq!(
        *account.to_account_info().owner,
        token_program.key(),
        ConfidentialTokenError::UnderlyingTokenProgramMismatch
    );
    require!(
        account.state != anchor_spl::token_interface::spl_token_2022::state::AccountState::Frozen,
        ConfidentialTokenError::UnderlyingTokenAccountFrozen
    );
    if token_program.key() == anchor_spl::token_2022::ID {
        use anchor_spl::token_interface::spl_token_2022::extension::{
            BaseStateWithExtensions, ExtensionType, StateWithExtensions,
        };
        let account_info = account.to_account_info();
        let data = account_info.try_borrow_data()?;
        let state = StateWithExtensions::<
            anchor_spl::token_interface::spl_token_2022::state::Account,
        >::unpack(&data)
        .map_err(|_| error!(ConfidentialTokenError::UnsupportedToken2022Extension))?;
        require!(
            state
                .get_extension_types()
                .map_err(|_| error!(ConfidentialTokenError::UnsupportedToken2022Extension))?
                .iter()
                .all(|extension| *extension == ExtensionType::ImmutableOwner),
            ConfidentialTokenError::UnsupportedToken2022Extension
        );
    }
    Ok(())
}

/// Issuer freeze (V3 `_requireNotBlocked` on the underlying).
///
/// `ata` must be the associated token account for `owner` on `mint.underlying_mint`
/// (`get_associated_token_address_with_program_id`, token program = `underlying_mint.owner`).
/// Uninitialized at that address (system-owned, empty) → not frozen: Circle/Tether freeze an
/// existing token account. Confidential transfer and burn use this; wrap and redeem freeze-check
/// the SPL accounts they move instead. Not the host grant deny-list.
pub(crate) fn check_underlying_ata_not_frozen(
    mint: &ConfidentialMint,
    owner: Pubkey,
    underlying_mint: &AccountInfo,
    ata: &AccountInfo,
) -> Result<()> {
    require_keys_eq!(
        underlying_mint.key(),
        mint.underlying_mint,
        ConfidentialTokenError::UnderlyingMintMismatch
    );
    let token_program = *underlying_mint.owner;
    require!(
        token_program == anchor_spl::token::ID || token_program == anchor_spl::token_2022::ID,
        ConfidentialTokenError::UnderlyingTokenProgramMismatch
    );
    require_keys_eq!(
        ata.key(),
        get_associated_token_address_with_program_id(&owner, &mint.underlying_mint, &token_program),
        ConfidentialTokenError::UnderlyingAssociatedAccountMismatch
    );
    if ata.owner == &System::id() && ata.data_is_empty() {
        require!(
            !ata.executable,
            ConfidentialTokenError::UnderlyingAssociatedAccountMismatch
        );
        return Ok(());
    }
    require_keys_eq!(
        *ata.owner,
        token_program,
        ConfidentialTokenError::UnderlyingTokenProgramMismatch
    );
    // Classic 165-byte accounts unpack through Token-2022 `StateWithExtensions`.
    use anchor_spl::token_interface::spl_token_2022::extension::StateWithExtensions;
    let data = ata.try_borrow_data()?;
    let state =
        StateWithExtensions::<anchor_spl::token_interface::spl_token_2022::state::Account>::unpack(
            &data,
        )
        .map_err(|_| error!(ConfidentialTokenError::UnderlyingAssociatedAccountMismatch))?;
    require_keys_eq!(
        state.base.mint,
        mint.underlying_mint,
        ConfidentialTokenError::UnderlyingMintMismatch
    );
    require_keys_eq!(
        state.base.owner,
        owner,
        ConfidentialTokenError::OwnerMismatch
    );
    require!(
        state.base.state
            != anchor_spl::token_interface::spl_token_2022::state::AccountState::Frozen,
        ConfidentialTokenError::UnderlyingTokenAccountFrozen
    );
    Ok(())
}

pub(crate) fn assert_confidential_token_account_key(
    token_account: Pubkey,
    mint: Pubkey,
    owner: Pubkey,
) -> Result<()> {
    require_keys_eq!(
        token_account,
        token_account_address(mint, owner).0,
        ConfidentialTokenError::TokenAccountMismatch
    );
    Ok(())
}

pub(crate) fn assert_confidential_mint_shape(mint: &Account<ConfidentialMint>) -> Result<()> {
    require!(
        mint.to_account_info().data_len() == 8 + ConfidentialMint::SPACE,
        ConfidentialTokenError::MintAccountMismatch
    );
    Ok(())
}

pub(crate) fn assert_confidential_token_account_shape(
    token_account: &Account<ConfidentialTokenAccount>,
    mint: Pubkey,
    owner: Pubkey,
) -> Result<()> {
    let expected_bump = token_account_address(mint, owner).1;
    assert_confidential_token_account_key(token_account.key(), mint, owner)?;
    require!(
        token_account.to_account_info().data_len() == 8 + ConfidentialTokenAccount::SPACE,
        ConfidentialTokenError::TokenAccountMismatch
    );
    require!(
        token_account.bump == expected_bump,
        ConfidentialTokenError::TokenAccountMismatch
    );
    require_keys_eq!(
        token_account.mint,
        mint,
        ConfidentialTokenError::MintMismatch
    );
    require_keys_eq!(
        token_account.owner,
        owner,
        ConfidentialTokenError::OwnerMismatch
    );
    Ok(())
}
