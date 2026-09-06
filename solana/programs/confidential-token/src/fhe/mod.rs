//! Token-local FHE helper functions.
//!
//! The confidential token program keeps ZamaHost CPI assembly in this module
//! so business logic can build typed executions and receive host-verified
//! output handles.
//!
//! The token program is one application per mint to the host: every value it writes belongs to
//! `(confidential_token::ID, mint)` ([`crate::token_app`]), and every value is controlled by a
//! PDA of this program — the token account for holder-scoped values, the `total-supply` PDA for
//! the encrypted total supply. Reading a value into a computation is admitted by that PDA's
//! signature, so the token program signs for every value an execution touches and no "compute
//! signer" identity exists. Who may decrypt a handle is said on the write that produces it
//! (`PersistentOutput::allow`); the owner is allowed on every holder-scoped write.

use anchor_lang::{prelude::*, AccountDeserialize};
use zama_host::{program::ZamaHost, EncryptedValue, HostConfig};

use crate::{
    token_account_address, token_app, total_supply_authority_address, ConfidentialTokenAccount,
    ConfidentialTokenError,
};

mod verify_public_decrypt;
pub(crate) use verify_public_decrypt::*;

/// A persistent execution output account bound to the exact `EncryptedValue` encrypted value
/// account it is allowed to create or update.
pub(crate) struct PersistentOutput<'info> {
    encrypted_value: AccountInfo<'info>,
    output: Box<zama_fhe::PersistentOutput>,
}

impl<'info> PersistentOutput<'info> {
    /// Binds `encrypted_value` as the output of a persistent execution step: creates the
    /// encrypted value account's first handle if the PDA does not exist yet (carrying
    /// `authority`'s seeds so the host can prove the value belongs to this program), or updates
    /// it (echoing the current handle read off the on-chain account) if it does. `allows` are
    /// the keys that may decrypt the handle this write produces.
    pub(crate) fn new(
        encrypted_value: AccountInfo<'info>,
        key: zama_fhe::EncryptedValueId,
        authority: &ValueAuthority<'info>,
        allows: impl IntoIterator<Item = Pubkey>,
    ) -> Result<Self> {
        Self::new_inner(encrypted_value, key, authority, allows, false)
    }

    /// Like [`new`](Self::new), but binds the output created publicly decryptable: the host
    /// seals a public-decrypt leaf for the new handle inside the same execution CPI
    /// (EVM `unwrap` parity; DD-036). Used by `confidential_burn` for the burned
    /// delta so every burn stays permanently redeemable with no second CPI.
    pub(crate) fn new_public(
        encrypted_value: AccountInfo<'info>,
        key: zama_fhe::EncryptedValueId,
        authority: &ValueAuthority<'info>,
        allows: impl IntoIterator<Item = Pubkey>,
    ) -> Result<Self> {
        Self::new_inner(encrypted_value, key, authority, allows, true)
    }

    fn new_inner(
        encrypted_value: AccountInfo<'info>,
        key: zama_fhe::EncryptedValueId,
        authority: &ValueAuthority<'info>,
        allows: impl IntoIterator<Item = Pubkey>,
        make_public: bool,
    ) -> Result<Self> {
        require_keys_eq!(
            encrypted_value.key(),
            key.address(),
            ConfidentialTokenError::CurrentEncryptedValueMismatch
        );
        require_keys_eq!(
            authority.key(),
            key.encrypted_value_account_authority(),
            ConfidentialTokenError::EncryptedValueAuthorityMismatch
        );
        let mut output = if *encrypted_value.owner == System::id() {
            require!(
                encrypted_value.data_is_empty() && !encrypted_value.executable,
                ConfidentialTokenError::InvalidFheExecution
            );
            zama_fhe::PersistentOutput::create(key, authority.signer.seeds().as_slice())
        } else {
            let value = read_encrypted_value(&encrypted_value)?;
            zama_fhe::PersistentOutput::update(key, value.current_handle)
        };
        for allowed in allows {
            output = output.allow(allowed);
        }
        if make_public {
            output = output.make_public();
        }
        output.validate().map_err(|error| {
            msg!("invalid persistent FHE output: {:?}", error);
            error!(ConfidentialTokenError::InvalidFheExecution)
        })?;
        Ok(Self {
            encrypted_value,
            output: Box::new(output),
        })
    }

    pub(crate) fn output(&self) -> zama_fhe::Output {
        zama_fhe::Output::persistent((*self.output).clone())
    }

    /// Reads the handle the host bound into `encrypted_value` by this execution CPI.
    /// Call only after the CPI that carries this output has executed.
    pub(crate) fn handle(&self) -> Result<[u8; 32]> {
        let value = read_encrypted_value(&self.encrypted_value)?;
        Ok(value.current_handle)
    }

    pub(crate) fn account_info(&self) -> AccountInfo<'info> {
        self.encrypted_value.clone()
    }
}

/// Decodes a canonical, host-owned `EncryptedValue` account.
pub(crate) fn read_encrypted_value(info: &AccountInfo) -> Result<EncryptedValue> {
    require_keys_eq!(
        *info.owner,
        zama_host::ID,
        ConfidentialTokenError::CurrentEncryptedValueMismatch
    );
    let data = info.try_borrow_data()?;
    let mut slice: &[u8] = &data;
    EncryptedValue::try_deserialize(&mut slice)
}

/// A euint64 operand read from a stored value's own canonical fields, so the operand slot
/// always matches the account the host re-validates.
pub(crate) fn uint64_operand(value: &EncryptedValue) -> Result<zama_fhe::Uint64Handle> {
    zama_fhe::Uint64Handle::persistent(
        value.current_handle,
        zama_fhe::EncryptedValueId::from_value(value),
    )
    .map_err(|error| {
        msg!("invalid FHE execution: {:?}", error);
        error!(ConfidentialTokenError::InvalidFheExecution)
    })
}

/// The application's deny record witness for one `fhe_execute` / `make_handle_public` CPI: the
/// single remaining account while the host's deny list is enabled, none otherwise. The host
/// re-derives the PDA; checking it here turns a wrong witness into this program's error.
pub(crate) fn deny_scope_record<'info>(
    host_config: &HostConfig,
    remaining_accounts: &[AccountInfo<'info>],
    mint: Pubkey,
) -> Result<Option<AccountInfo<'info>>> {
    if !host_config.grant_deny_list_enabled {
        require!(
            remaining_accounts.is_empty(),
            ConfidentialTokenError::UnexpectedRemainingAccounts
        );
        return Ok(None);
    }
    let [record] = remaining_accounts else {
        return err!(ConfidentialTokenError::UnexpectedRemainingAccounts);
    };
    require_keys_eq!(
        record.key(),
        zama_host::deny_scope_address(token_app(mint)).0,
        ConfidentialTokenError::UnexpectedRemainingAccounts
    );
    Ok(Some(record.clone()))
}

/// Signer model for a value authority required by an execution.
#[derive(Clone)]
pub(crate) enum ValueAuthoritySigner {
    TokenAccount {
        mint: Pubkey,
        owner: Pubkey,
        bump: u8,
    },
    TotalSupply {
        mint: Pubkey,
        bump: u8,
    },
    /// Another program's PDA whose signature the transaction already carries (through that
    /// program's `invoke_signed`). `seeds` prove it to the host on a create; this program never
    /// signs with them.
    Foreign {
        seeds: Vec<Vec<u8>>,
    },
}

impl ValueAuthoritySigner {
    pub(crate) fn token_account(account: &Account<'_, ConfidentialTokenAccount>) -> Self {
        Self::TokenAccount {
            mint: account.mint,
            owner: account.owner,
            bump: account.bump,
        }
    }

    pub(crate) fn total_supply(mint: Pubkey, bump: u8) -> Self {
        Self::TotalSupply { mint, bump }
    }

    /// Signer seeds borrowed straight from the stored key material; assembling
    /// them allocates nothing on the never-freeing program heap.
    fn seeds(&self) -> ValueAuthoritySeeds<'_> {
        match self {
            Self::TokenAccount { mint, owner, bump } => ValueAuthoritySeeds {
                seeds: [
                    b"token-account",
                    mint.as_ref(),
                    owner.as_ref(),
                    std::slice::from_ref(bump),
                ],
                len: 4,
            },
            Self::TotalSupply { mint, bump } => ValueAuthoritySeeds {
                seeds: [
                    b"total-supply",
                    mint.as_ref(),
                    std::slice::from_ref(bump),
                    &[],
                ],
                len: 3,
            },
            Self::Foreign { seeds } => {
                let mut slots: [&[u8]; 4] = [&[], &[], &[], &[]];
                for (slot, seed) in slots.iter_mut().zip(seeds) {
                    *slot = seed;
                }
                ValueAuthoritySeeds {
                    seeds: slots,
                    len: seeds.len(),
                }
            }
        }
    }

    /// Whether this program signs for the authority in the host CPI.
    fn signs_here(&self) -> bool {
        !matches!(self, Self::Foreign { .. })
    }
}

/// Seed slices for one value authority. The array is sized for the widest
/// signer variant; `len` says how many slots the variant fills.
struct ValueAuthoritySeeds<'a> {
    seeds: [&'a [u8]; 4],
    len: usize,
}

impl<'a> ValueAuthoritySeeds<'a> {
    fn as_slice(&self) -> &[&'a [u8]] {
        &self.seeds[..self.len]
    }
}

/// A value authority account plus the signer model that authorizes it.
#[derive(Clone)]
pub(crate) struct ValueAuthority<'info> {
    account: AccountInfo<'info>,
    signer: Box<ValueAuthoritySigner>,
}

impl<'info> ValueAuthority<'info> {
    pub(crate) fn token_account(
        account: &Account<'info, ConfidentialTokenAccount>,
    ) -> Result<Self> {
        let (expected, expected_bump) = token_account_address(account.mint, account.owner);
        require_keys_eq!(
            account.key(),
            expected,
            ConfidentialTokenError::TokenAccountMismatch
        );
        require!(
            account.bump == expected_bump,
            ConfidentialTokenError::TokenAccountMismatch
        );
        Ok(Self {
            account: account.to_account_info(),
            signer: Box::new(ValueAuthoritySigner::token_account(account)),
        })
    }

    pub(crate) fn total_supply(
        account: &UncheckedAccount<'info>,
        mint: Pubkey,
        bump: u8,
    ) -> Result<Self> {
        let (expected, expected_bump) = total_supply_authority_address(mint);
        require_keys_eq!(
            account.key(),
            expected,
            ConfidentialTokenError::TotalSupplyAuthorityMismatch
        );
        require!(
            bump == expected_bump,
            ConfidentialTokenError::TotalSupplyAuthorityMismatch
        );
        Ok(Self {
            account: account.to_account_info(),
            signer: Box::new(ValueAuthoritySigner::total_supply(mint, bump)),
        })
    }

    /// An authority whose signature the transaction already carries and that never creates a
    /// value here (the spend gate's amount authority).
    pub(crate) fn external(account: AccountInfo<'info>) -> Self {
        Self {
            account,
            signer: Box::new(ValueAuthoritySigner::Foreign { seeds: Vec::new() }),
        }
    }

    /// Another program's signing PDA that a transfer writes a receipt for; `seeds` (bump last)
    /// derive it under that program and let the host prove the value is that program's.
    pub(crate) fn foreign(account: AccountInfo<'info>, seeds: Vec<Vec<u8>>) -> Result<Self> {
        require!(
            seeds.len() <= 4,
            ConfidentialTokenError::InvalidFheExecution
        );
        Ok(Self {
            account,
            signer: Box::new(ValueAuthoritySigner::Foreign { seeds }),
        })
    }

    fn key(&self) -> Pubkey {
        self.account.key()
    }

    fn account_info(&self) -> AccountInfo<'info> {
        self.account.clone()
    }
}

/// Pubkey-indexed accounts and authorities available to satisfy an execution.
pub(crate) struct ExecutionAccountSet<'info> {
    accounts: zama_fhe::ResolvedExecutionAccounts<'info>,
    value_authorities: Vec<ValueAuthority<'info>>,
}

impl<'info> ExecutionAccountSet<'info> {
    pub(crate) fn for_execution(
        execution: &zama_fhe::FheExecution,
        available_accounts: impl IntoIterator<Item = AccountInfo<'info>>,
        value_authorities: impl IntoIterator<Item = ValueAuthority<'info>>,
    ) -> Result<Self> {
        let value_authorities = value_authorities.into_iter().collect::<Vec<_>>();
        let value_authority_accounts = value_authorities
            .iter()
            .map(ValueAuthority::account_info)
            .collect::<Vec<_>>();
        let accounts = execution
            .resolve_accounts(available_accounts, value_authority_accounts)
            .map_err(map_execution_account_resolution_error)?;

        Ok(Self {
            accounts,
            value_authorities,
        })
    }

    fn value_authority(&self, pubkey: Pubkey) -> Result<ValueAuthority<'info>> {
        self.value_authorities
            .iter()
            .find(|authority| authority.key() == pubkey)
            .cloned()
            .ok_or_else(|| error!(ConfidentialTokenError::MissingFheOutputAuthority))
    }

    fn resolved_accounts(&self) -> &zama_fhe::ResolvedExecutionAccounts<'info> {
        &self.accounts
    }
}

fn map_execution_account_resolution_error(
    error: zama_fhe::ExecutionAccountResolutionError,
) -> Error {
    msg!("invalid fhe_execute account set: {:?}", error);
    match error {
        zama_fhe::ExecutionAccountResolutionError::DuplicateDynamicAccount { .. } => {
            error!(ConfidentialTokenError::DuplicateFheExecuteAccount)
        }
        zama_fhe::ExecutionAccountResolutionError::UnexpectedDynamicAccount { .. } => {
            error!(ConfidentialTokenError::UnexpectedFheExecuteAccount)
        }
        zama_fhe::ExecutionAccountResolutionError::MissingDynamicAccount { .. } => {
            error!(ConfidentialTokenError::MissingFheExecuteAccount)
        }
        zama_fhe::ExecutionAccountResolutionError::DynamicAccountNotWritable { .. } => {
            error!(ConfidentialTokenError::FheExecuteAccountNotWritable)
        }
        zama_fhe::ExecutionAccountResolutionError::DuplicateValueAuthority { .. } => {
            error!(ConfidentialTokenError::DuplicateFheOutputAuthority)
        }
        zama_fhe::ExecutionAccountResolutionError::UnexpectedValueAuthority { .. } => {
            error!(ConfidentialTokenError::UnexpectedFheOutputAuthority)
        }
        zama_fhe::ExecutionAccountResolutionError::MissingValueAuthority { .. } => {
            error!(ConfidentialTokenError::MissingFheOutputAuthority)
        }
    }
}

/// Inputs required to evaluate an instruction-local FHE execution.
pub(crate) struct ExecuteContext<'a, 'info> {
    /// Transaction payer and rent payer for any persistent output accounts.
    pub payer: &'a Signer<'info>,
    /// Anchor event CPI authority for ZamaHost.
    pub event_authority: &'a UncheckedAccount<'info>,
    /// ZamaHost program account.
    pub zama_program: &'a Program<'info, ZamaHost>,
    /// Host config used for chain-id-aware handle derivation.
    pub host_config: &'a Account<'info, HostConfig>,
    /// The mint's deny record witness, from [`deny_scope_record`].
    pub deny_scope_record: Option<AccountInfo<'info>>,
    /// System program used for output account creation.
    pub system_program: &'a Program<'info, System>,
    /// Per-mint HCU block meter forwarded into the host `fhe_execute` CPI (`None` unless the
    /// caller threads it; behavior-neutral while the host cap is unrestricted). The host keys
    /// the meter on the execution's application `(program, mint)`, so metering stays per-mint
    /// automatically.
    pub hcu_block_meter: Option<AccountInfo<'info>>,
    /// HCU trust witness forwarded into the host `fhe_execute` CPI (`None` unless threaded).
    pub hcu_trusted_app_record: Option<AccountInfo<'info>>,
}

/// Inputs for one instruction-local FHE execution.
pub(crate) struct Execute<'a, 'info> {
    /// Fixed ZamaHost CPI accounts shared by every execution in this instruction.
    pub context: ExecuteContext<'a, 'info>,
    /// Typed resolver for dynamic accounts required by the execution.
    pub accounts: &'a ExecutionAccountSet<'info>,
    /// SDK-built host execution request and dynamic account roles.
    pub execution: zama_fhe::FheExecution,
}

/// Invokes one FHE execution, signing for every value authority it requires that this program
/// controls; a foreign authority's signature is already on the transaction. Token executions
/// never draw randomness, so no rand nonce is passed.
pub(crate) fn execute<'info>(request: Execute<'_, 'info>) -> Result<()> {
    let encrypted_value_account_authority = request.accounts.value_authority(
        request
            .execution
            .encrypted_value_account_authority()
            .pubkey(),
    )?;
    let additional_authorities: Vec<ValueAuthority<'info>> = request
        .execution
        .additional_value_authorities()
        .map(|authority| request.accounts.value_authority(authority))
        .collect::<Result<_>>()?;
    let authority_seeds: Vec<ValueAuthoritySeeds> =
        std::iter::once(&encrypted_value_account_authority)
            .chain(&additional_authorities)
            .filter(|authority| authority.signer.signs_here())
            .map(|authority| authority.signer.seeds())
            .collect();
    let signer_seeds: Vec<&[&[u8]]> = authority_seeds
        .iter()
        .map(ValueAuthoritySeeds::as_slice)
        .collect();

    request.execution.invoke(
        zama_fhe::ExecutionCpiAccounts {
            payer: request.context.payer.to_account_info(),
            encrypted_value_account_authority: encrypted_value_account_authority.account_info(),
            host_config: request.context.host_config.to_account_info(),
            deny_scope_record: request.context.deny_scope_record,
            system_program: request.context.system_program.to_account_info(),
            hcu_block_meter: request.context.hcu_block_meter,
            hcu_trusted_app_record: request.context.hcu_trusted_app_record,
            rand_nonce: None,
            event_authority: request.context.event_authority.to_account_info(),
            program: request.context.zama_program.to_account_info(),
        },
        request.accounts.resolved_accounts(),
        &signer_seeds,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn handle(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn balance_handle(tag: u8) -> [u8; 32] {
        let mut handle = [tag; 32];
        handle[30] = crate::BALANCE_FHE_TYPE;
        handle
    }

    fn account_info(pubkey: Pubkey, is_writable: bool) -> AccountInfo<'static> {
        let key = Box::leak(Box::new(pubkey));
        let owner = Box::leak(Box::new(System::id()));
        let lamports = Box::leak(Box::new(0));
        let data = Box::leak(Vec::new().into_boxed_slice());
        AccountInfo::new(key, false, is_writable, lamports, data, owner, false)
    }

    // The signer model is irrelevant to these tests — they resolve authorities by address, and the
    // seeds are only used when actually signing a CPI, which a host unit test never does.
    fn value_authority(pubkey: Pubkey) -> ValueAuthority<'static> {
        ValueAuthority::external(account_info(pubkey, false))
    }

    fn encrypted_value_id(account: Pubkey, label_tag: u8) -> zama_fhe::EncryptedValueId {
        zama_fhe::EncryptedValueId::new(
            token_app(Pubkey::new_from_array([9; 32])),
            account,
            zama_fhe::EncryptedValueLabel::new(handle(label_tag)),
        )
    }

    fn sample_plan() -> (zama_fhe::FheExecution, Pubkey, Pubkey, Pubkey) {
        let authority = Pubkey::new_unique();
        let input_key = encrypted_value_id(authority, 1);
        let input_acl = input_key.address();
        let output_key = encrypted_value_id(authority, 2);
        let output_acl = output_key.address();
        let input = zama_fhe::Uint64Handle::persistent(balance_handle(1), input_key).unwrap();
        let execution = zama_fhe::FheExecution::build(
            zama_fhe::ExecutionEncryptedValueAccountAuthority::new(authority),
            |builder| {
                builder.add(
                    input,
                    zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(1),
                    zama_fhe::Output::persistent(
                        zama_fhe::PersistentOutput::create(output_key, &[]).allow(authority),
                    ),
                )?;
                Ok(())
            },
        )
        .unwrap();
        (execution, input_acl, output_acl, authority)
    }

    fn token_error_number(error: Error) -> u32 {
        match error {
            Error::AnchorError(error) => error.error_code_number,
            other => panic!("unexpected error: {other:?}"),
        }
    }

    fn assert_token_error(error: Error, expected: ConfidentialTokenError) {
        assert_eq!(
            token_error_number(error),
            token_error_number(error!(expected))
        );
    }

    #[test]
    fn batch_account_set_maps_dynamic_account_errors() {
        let (execution, input_acl, output_acl, authority) = sample_plan();

        let error = ExecutionAccountSet::for_execution(
            &execution,
            vec![
                account_info(input_acl, false),
                account_info(input_acl, false),
                account_info(output_acl, true),
            ],
            vec![value_authority(authority)],
        )
        .err()
        .unwrap();
        assert_token_error(error, ConfidentialTokenError::DuplicateFheExecuteAccount);

        let error = ExecutionAccountSet::for_execution(
            &execution,
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
                account_info(Pubkey::new_unique(), false),
            ],
            vec![value_authority(authority)],
        )
        .err()
        .unwrap();
        assert_token_error(error, ConfidentialTokenError::UnexpectedFheExecuteAccount);

        let error = ExecutionAccountSet::for_execution(
            &execution,
            vec![account_info(output_acl, true)],
            vec![value_authority(authority)],
        )
        .err()
        .unwrap();
        assert_token_error(error, ConfidentialTokenError::MissingFheExecuteAccount);

        let error = ExecutionAccountSet::for_execution(
            &execution,
            vec![
                account_info(input_acl, false),
                account_info(output_acl, false),
            ],
            vec![value_authority(authority)],
        )
        .err()
        .unwrap();
        assert_token_error(error, ConfidentialTokenError::FheExecuteAccountNotWritable);
    }

    #[test]
    fn batch_account_set_maps_value_authority_errors() {
        let (execution, input_acl, output_acl, authority) = sample_plan();

        let error = ExecutionAccountSet::for_execution(
            &execution,
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
            ],
            vec![value_authority(authority), value_authority(authority)],
        )
        .err()
        .unwrap();
        assert_token_error(error, ConfidentialTokenError::DuplicateFheOutputAuthority);

        let error = ExecutionAccountSet::for_execution(
            &execution,
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
            ],
            vec![
                value_authority(authority),
                value_authority(Pubkey::new_unique()),
            ],
        )
        .err()
        .unwrap();
        assert_token_error(error, ConfidentialTokenError::UnexpectedFheOutputAuthority);

        let error = ExecutionAccountSet::for_execution(
            &execution,
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
            ],
            Vec::new(),
        )
        .err()
        .unwrap();
        assert_token_error(error, ConfidentialTokenError::MissingFheOutputAuthority);
    }
}
