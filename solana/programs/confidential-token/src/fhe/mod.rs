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
use zama_host::{program::ZamaHost, HostConfig};

use crate::{
    token_account_address, token_app, total_supply_authority_address, ConfidentialTokenAccount,
    ConfidentialTokenError,
};

mod verify_public_decrypt;
pub(crate) use verify_public_decrypt::*;

pub(crate) struct SlotOutput<'info> {
    state: AccountInfo<'info>,
    key: [u8; 32],
    output: Box<zama_fhe::StateOutput>,
}

impl<'info> SlotOutput<'info> {
    pub(crate) fn new(
        state: AccountInfo<'info>,
        target: (zama_fhe::StateId, [u8; 32]),
        authority: &ValueAuthority<'info>,
        allows: impl IntoIterator<Item = Pubkey>,
    ) -> Result<Self> {
        require_keys_eq!(
            state.key(),
            target.0.address(),
            ConfidentialTokenError::CurrentEncryptedValueMismatch
        );
        require_keys_eq!(
            authority.key(),
            target.0.authority(),
            ConfidentialTokenError::EncryptedValueAuthorityMismatch
        );
        let account = read_state(&state)?;
        let mut output = zama_fhe::State::new(&account).set(target.1);
        for subject in allows {
            output = output.allow(subject);
        }
        Ok(Self {
            state,
            key: target.1,
            output: Box::new(output),
        })
    }

    pub(crate) fn new_public(
        state: AccountInfo<'info>,
        target: (zama_fhe::StateId, [u8; 32]),
        authority: &ValueAuthority<'info>,
        allows: impl IntoIterator<Item = Pubkey>,
    ) -> Result<Self> {
        let mut output = Self::new(state, target, authority, allows)?;
        output.output = Box::new((*output.output).make_public());
        Ok(output)
    }

    pub(crate) fn output(&self) -> zama_fhe::Output {
        zama_fhe::Output::state((*self.output).clone())
    }
    pub(crate) fn handle(&self) -> Result<[u8; 32]> {
        read_state(&self.state)?
            .get(&self.key)
            .ok_or_else(|| error!(ConfidentialTokenError::CurrentEncryptedValueMismatch))
    }
    pub(crate) fn account_info(&self) -> AccountInfo<'info> {
        self.state.clone()
    }
}

pub(crate) fn read_state(info: &AccountInfo) -> Result<zama_host::EncryptedState> {
    require_keys_eq!(
        *info.owner,
        zama_host::ID,
        ConfidentialTokenError::CurrentEncryptedValueMismatch
    );
    let state = zama_host::EncryptedState::try_deserialize(&mut &info.try_borrow_data()?[..])?;
    require_keys_eq!(
        state.canonical_address().0,
        info.key(),
        ConfidentialTokenError::CurrentEncryptedValueMismatch
    );
    Ok(state)
}

pub(crate) fn state_handle(state: &zama_host::EncryptedState, key: [u8; 32]) -> Result<[u8; 32]> {
    state.get(&key).ok_or_else(|| error!(ConfidentialTokenError::CurrentEncryptedValueMismatch))
}

pub(crate) fn uint64_operand(
    state: &zama_host::EncryptedState,
    key: [u8; 32],
) -> Result<zama_fhe::Uint64Handle> {
    zama_fhe::State::new(state)
        .get(key)
        .map_err(|_| error!(ConfidentialTokenError::InvalidFheExecution))
}

/// The deny record witnesses for one `fhe_execute` CPI: exactly one remaining account per
/// application the execution touches, in first-occurrence `apps` order, while the host's deny list is enabled;
/// none otherwise. The host re-derives the PDAs; checking them here turns a wrong witness into
/// this program's error.
pub(crate) fn deny_scope_records<'info>(
    host_config: &HostConfig,
    remaining_accounts: &[AccountInfo<'info>],
    apps: impl IntoIterator<Item = zama_fhe::AppScope>,
) -> Result<Vec<AccountInfo<'info>>> {
    if !host_config.grant_deny_list_enabled {
        require!(
            remaining_accounts.is_empty(),
            ConfidentialTokenError::UnexpectedRemainingAccounts
        );
        return Ok(Vec::new());
    }
    let mut unique_apps = Vec::new();
    for app in apps {
        if !unique_apps.contains(&app) {
            unique_apps.push(app);
        }
    }
    let mut apps = unique_apps.into_iter();
    let mut records = Vec::with_capacity(remaining_accounts.len());
    for record in remaining_accounts {
        let Some(app) = apps.next() else {
            return err!(ConfidentialTokenError::UnexpectedRemainingAccounts);
        };
        require_keys_eq!(
            record.key(),
            zama_host::deny_scope_address(app).0,
            ConfidentialTokenError::UnexpectedRemainingAccounts
        );
        records.push(record.clone());
    }
    require!(
        apps.next().is_none(),
        ConfidentialTokenError::UnexpectedRemainingAccounts
    );
    Ok(records)
}

/// The mint's deny record witness for one `make_handle_public` CPI, which touches one application.
pub(crate) fn deny_scope_record<'info>(
    host_config: &HostConfig,
    remaining_accounts: &[AccountInfo<'info>],
    mint: Pubkey,
) -> Result<Option<AccountInfo<'info>>> {
    Ok(deny_scope_records(host_config, remaining_accounts, [token_app(mint)])?.pop())
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
    /// An authority already signed by the calling program or transaction.
    External,
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
            Self::External => ValueAuthoritySeeds { seeds: [&[]; 4], len: 0 },
        }
    }

    /// Whether this program signs for the authority in the host CPI.
    fn signs_here(&self) -> bool {
        !matches!(self, Self::External)
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
    pub(crate) fn create_state(
        &self,
        mint: Pubkey,
        state: AccountInfo<'info>,
        payer: AccountInfo<'info>,
        host_config: AccountInfo<'info>,
        system_program: AccountInfo<'info>,
    ) -> Result<()> {
        let seeds = self.signer.seeds();
        zama_host::cpi::create_encrypted_state(
            CpiContext::new_with_signer(
                zama_host::ID,
                zama_host::cpi::accounts::CreateEncryptedState {
                    payer,
                    authority: self.account_info(),
                    encrypted_state: state,
                    host_config,
                    system_program,
                },
                &[seeds.as_slice()],
            ),
            zama_host::instructions::CreateEncryptedStateArgs {
                program: crate::ID,
                scope: mint.to_bytes(),
                authority_seeds: seeds.as_slice().iter().map(|s| s.to_vec()).collect(),
            },
        )
    }

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
            signer: Box::new(ValueAuthoritySigner::External),
        }
    }

    pub(crate) fn key(&self) -> Pubkey {
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

    fn signing_authorities(&self, execution: &zama_fhe::FheExecution) -> Result<Vec<&ValueAuthority<'info>>> {
        execution.value_authorities().map(|key| self.value_authorities.iter().find(|authority| authority.key() == key).ok_or_else(|| error!(ConfidentialTokenError::MissingFheOutputAuthority))).collect()
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
    /// One deny record witness per application the execution touches, from [`deny_scope_records`].
    pub deny_scope_records: Vec<AccountInfo<'info>>,
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
pub(crate) struct Execute<'a, 'info, E = zama_fhe::FheExecution> {
    /// Fixed ZamaHost CPI accounts shared by every execution in this instruction.
    pub context: ExecuteContext<'a, 'info>,
    /// Typed resolver for dynamic accounts required by the execution.
    pub accounts: &'a ExecutionAccountSet<'info>,
    /// SDK-built host execution request and dynamic account roles.
    pub execution: E,
}

/// Invokes one FHE execution, signing for every value authority it requires that this program
/// controls; a foreign authority's signature is already on the transaction. Token executions
/// never draw randomness, so no rand nonce is passed.
pub(crate) fn execute<'info>(request: Execute<'_, 'info>) -> Result<()> {
    let authorities = request.accounts.signing_authorities(&request.execution)?;
    invoke_with_authorities(request.context, &authorities, |accounts, seeds| request.execution.invoke(accounts, request.accounts.resolved_accounts(), seeds))
}

pub(crate) fn execute_returning<'info>(request: Execute<'_, 'info, zama_fhe::ReturningFheExecution<zama_fhe::Uint<64>>>) -> Result<[u8; 32]> {
    let authorities = request.accounts.signing_authorities(request.execution.execution())?;
    invoke_with_authorities(request.context, &authorities, |accounts, seeds| request.execution.invoke(accounts, request.accounts.resolved_accounts(), seeds))
}

fn invoke_with_authorities<'info, R>(context: ExecuteContext<'_, 'info>, authorities: &[&ValueAuthority<'info>], invoke: impl FnOnce(zama_fhe::ExecutionCpiAccounts<'info>, &[&[&[u8]]]) -> Result<R>) -> Result<R> {
    let primary = authorities.first().ok_or(ConfidentialTokenError::MissingFheOutputAuthority)?;
    let authority_seeds: Vec<_> = authorities.iter().filter(|authority| authority.signer.signs_here()).map(|authority| authority.signer.seeds()).collect();
    let signer_seeds: Vec<_> = authority_seeds.iter().map(ValueAuthoritySeeds::as_slice).collect();
    invoke(zama_fhe::ExecutionCpiAccounts {
        payer: context.payer.to_account_info(),
        encrypted_value_account_authority: primary.account_info(),
        host_config: context.host_config.to_account_info(),
        deny_scope_records: context.deny_scope_records,
        system_program: context.system_program.to_account_info(),
        hcu_block_meter: context.hcu_block_meter,
        hcu_trusted_app_record: context.hcu_trusted_app_record,
        rand_nonce: None,
        event_authority: context.event_authority.to_account_info(),
        program: context.zama_program.to_account_info(),
    }, &signer_seeds)
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
