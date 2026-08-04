//! Token-local FHE helper functions.
//!
//! The confidential token program keeps ZamaHost CPI assembly in this module
//! so business logic can build typed executions and receive host-verified
//! output handles.

use anchor_lang::{prelude::*, AccountDeserialize};
use zama_host::{program::ZamaHost, EncryptedValue, HostConfig};

use crate::{
    compute_signer_address, token_account_address, total_supply_authority_address,
    ConfidentialTokenAccount, ConfidentialTokenError,
};

mod verify_public_decrypt;
pub(crate) use verify_public_decrypt::*;

/// Audience for a confidential-token persistent output.
///
/// Holder-scoped encrypted value accounts (balances, `transferred_amount`, `burned_amount`) must
/// always grant the holder's owner key and the mint compute-signer PDA: the owner
/// keeps decrypt access to their own value, and the compute signer gates the next
/// eval that reads it. [`PersistentAudience::for_owner`] takes both as required
/// parameters, so a holder output can never be built missing either; extra owners
/// (the recipient phase of a `transferred_amount` update) are additive via
/// [`PersistentAudience::with_owner`]. Mint-scoped encrypted value accounts with no single holder
/// (total supply, freshly minted random amounts) use
/// [`PersistentAudience::compute_only`], the one owner-less path.
///
/// This is the only way token instructions produce persistent-output subjects:
/// [`PersistentOutput::new`]/[`PersistentOutput::new_public`] accept a `PersistentAudience`
/// rather than a raw subject vector, so the owner+compute invariant holds
/// by construction rather than by convention at each call site.
pub(crate) struct PersistentAudience {
    owner: Option<Pubkey>,
    extra_owners: Vec<Pubkey>,
    compute: Pubkey,
}

impl PersistentAudience {
    /// Holder-scoped audience granting `owner` and the mint `compute` signer.
    pub(crate) fn for_owner(owner: Pubkey, compute: Pubkey) -> Self {
        Self {
            owner: Some(owner),
            extra_owners: Vec::new(),
            compute,
        }
    }

    /// Mint-scoped audience with no holder, granting only the `compute` signer.
    pub(crate) fn compute_only(compute: Pubkey) -> Self {
        Self {
            owner: None,
            extra_owners: Vec::new(),
            compute,
        }
    }

    /// Adds an extra owner subject (the recipient of a `transferred_amount` phase).
    pub(crate) fn with_owner(mut self, owner: Pubkey) -> Self {
        self.extra_owners.push(owner);
        self
    }

    /// Renders the audience into host output subjects, ordered owner(s) then
    /// compute signer.
    fn into_subjects(self) -> Vec<Pubkey> {
        let mut subjects = Vec::with_capacity(2 + self.extra_owners.len());
        subjects.extend(self.owner);
        subjects.extend(self.extra_owners);
        subjects.push(self.compute);
        subjects
    }
}

/// A persistent eval output account bound to the exact `EncryptedValue` encrypted value account
/// it is allowed to create or update.
pub(crate) struct PersistentOutput<'info> {
    encrypted_value: AccountInfo<'info>,
    output: Box<zama_fhe::PersistentOutput>,
}

impl<'info> PersistentOutput<'info> {
    /// Binds `encrypted_value` as the output of a persistent eval step: creates the
    /// encrypted value account's first handle if the PDA does not exist yet, or updates it
    /// (reading `previous_handle`/`previous_subjects` off the on-chain account)
    /// if it does. Either way the eval CPI's attestation matches exactly what
    /// the host will verify.
    pub(crate) fn new(
        encrypted_value: AccountInfo<'info>,
        key: zama_fhe::EncryptedValueId,
        audience: PersistentAudience,
    ) -> Result<Self> {
        Self::new_inner(encrypted_value, key, audience, false)
    }

    /// Like [`new`], but binds the output created publicly decryptable: the host
    /// seals a public-decrypt leaf for the new handle inside the same eval CPI
    /// (EVM `unwrap` parity; DD-036). Used by `confidential_burn` for the burned
    /// delta so every burn stays permanently redeemable with no second CPI.
    pub(crate) fn new_public(
        encrypted_value: AccountInfo<'info>,
        key: zama_fhe::EncryptedValueId,
        audience: PersistentAudience,
    ) -> Result<Self> {
        Self::new_inner(encrypted_value, key, audience, true)
    }

    fn new_inner(
        encrypted_value: AccountInfo<'info>,
        key: zama_fhe::EncryptedValueId,
        audience: PersistentAudience,
        make_public: bool,
    ) -> Result<Self> {
        require_keys_eq!(
            encrypted_value.key(),
            key.address(),
            ConfidentialTokenError::CurrentEncryptedValueMismatch
        );
        let subjects = audience.into_subjects();
        let output = if *encrypted_value.owner == System::id() {
            require!(
                encrypted_value.data_is_empty() && !encrypted_value.executable,
                ConfidentialTokenError::InvalidFheExecution
            );
            zama_fhe::PersistentOutput::create(key, subjects)
        } else {
            let value = read_encrypted_value(&encrypted_value)?;
            zama_fhe::PersistentOutput::update(key, subjects, &value)
        }
        .with_make_public(make_public);
        output.binding().map_err(|error| {
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

    /// Reads the handle the host bound into `encrypted_value` by this eval CPI.
    /// Call only after the CPI that carries this output has executed.
    pub(crate) fn handle(&self) -> Result<[u8; 32]> {
        let binding = self.binding()?;
        require_keys_eq!(
            self.encrypted_value.key(),
            binding.encrypted_value(),
            ConfidentialTokenError::CurrentEncryptedValueMismatch
        );
        let value = read_encrypted_value(&self.encrypted_value)?;
        Ok(value.current_handle)
    }

    pub(crate) fn account_info(&self) -> AccountInfo<'info> {
        self.encrypted_value.clone()
    }

    fn binding(&self) -> Result<zama_fhe::PersistentOutputBinding> {
        self.output.binding().map_err(|error| {
            msg!("invalid persistent FHE output: {:?}", error);
            error!(ConfidentialTokenError::InvalidFheExecution)
        })
    }
}

/// Decodes a canonical, program-owned `EncryptedValue` account.
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

/// Program-controlled compute signer PDA plus the ACL domain it signs for.
#[derive(Clone)]
pub(crate) struct ComputeAuthority<'info> {
    account: AccountInfo<'info>,
    domain: Pubkey,
    bump: u8,
}

impl<'info> ComputeAuthority<'info> {
    pub(crate) fn for_mint(
        account: &UncheckedAccount<'info>,
        mint: Pubkey,
        bump: u8,
    ) -> Result<Self> {
        let (expected, expected_bump) = compute_signer_address(mint);
        require_keys_eq!(
            account.key(),
            expected,
            ConfidentialTokenError::ComputeSignerMismatch
        );
        require!(
            bump == expected_bump,
            ConfidentialTokenError::ComputeSignerMismatch
        );
        Ok(Self {
            account: account.to_account_info(),
            domain: mint,
            bump,
        })
    }

    fn account_info(&self) -> AccountInfo<'info> {
        self.account.clone()
    }

    fn signer_seeds<'a>(&'a self, bump: &'a [u8; 1]) -> [&'a [u8]; 3] {
        [b"fhe-compute", self.domain.as_ref(), bump]
    }
}

/// Signer model for a persistent output authority required by an execution.
#[derive(Clone)]
pub(crate) enum OutputAuthoritySigner {
    // Only constructed by the `poc`-gated create_random_amount helpers.
    #[cfg_attr(not(feature = "poc"), allow(dead_code))]
    Transaction,
    TokenAccount {
        mint: Pubkey,
        owner: Pubkey,
        bump: u8,
    },
    TotalSupply {
        mint: Pubkey,
        bump: u8,
    },
}

impl OutputAuthoritySigner {
    #[cfg_attr(not(feature = "poc"), allow(dead_code))]
    pub(crate) fn transaction_signer() -> Self {
        Self::Transaction
    }

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

    fn seed_bytes(&self) -> Vec<Vec<u8>> {
        match self {
            Self::Transaction => Vec::new(),
            Self::TokenAccount { mint, owner, bump } => vec![
                b"token-account".to_vec(),
                mint.to_bytes().to_vec(),
                owner.to_bytes().to_vec(),
                vec![*bump],
            ],
            Self::TotalSupply { mint, bump } => vec![
                b"total-supply".to_vec(),
                mint.to_bytes().to_vec(),
                vec![*bump],
            ],
        }
    }
}

/// Persistent output authority account plus the signer model that authorizes it.
#[derive(Clone)]
pub(crate) struct OutputAuthority<'info> {
    account: AccountInfo<'info>,
    signer: Box<OutputAuthoritySigner>,
}

impl<'info> OutputAuthority<'info> {
    #[cfg_attr(not(feature = "poc"), allow(dead_code))]
    pub(crate) fn transaction_signer(account: &Signer<'info>) -> Self {
        Self {
            account: account.to_account_info(),
            signer: Box::new(OutputAuthoritySigner::transaction_signer()),
        }
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
            signer: Box::new(OutputAuthoritySigner::token_account(account)),
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
            signer: Box::new(OutputAuthoritySigner::total_supply(mint, bump)),
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
    output_authorities: Vec<OutputAuthority<'info>>,
}

impl<'info> ExecutionAccountSet<'info> {
    pub(crate) fn for_execution(
        execution: &zama_fhe::FheExecution,
        available_accounts: impl IntoIterator<Item = AccountInfo<'info>>,
        output_authorities: impl IntoIterator<Item = OutputAuthority<'info>>,
    ) -> Result<Self> {
        let output_authorities = output_authorities.into_iter().collect::<Vec<_>>();
        let output_authority_accounts = output_authorities
            .iter()
            .map(OutputAuthority::account_info)
            .collect::<Vec<_>>();
        let accounts = execution
            .resolve_accounts(available_accounts, output_authority_accounts)
            .map_err(map_execution_account_resolution_error)?;

        Ok(Self {
            accounts,
            output_authorities,
        })
    }

    fn output_authority(&self, pubkey: Pubkey) -> Option<OutputAuthority<'info>> {
        self.output_authorities
            .iter()
            .find(|authority| authority.key() == pubkey)
            .cloned()
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
        zama_fhe::ExecutionAccountResolutionError::DuplicateOutputAuthority { .. } => {
            error!(ConfidentialTokenError::DuplicateFheOutputAuthority)
        }
        zama_fhe::ExecutionAccountResolutionError::UnexpectedOutputAuthority { .. } => {
            error!(ConfidentialTokenError::UnexpectedFheOutputAuthority)
        }
        zama_fhe::ExecutionAccountResolutionError::MissingOutputAuthority { .. } => {
            error!(ConfidentialTokenError::MissingFheOutputAuthority)
        }
    }
}

/// Inputs required to evaluate an instruction-local FHE execution.
pub(crate) struct ExecuteContext<'a, 'info> {
    /// Transaction payer and rent payer for any persistent output ACL records.
    pub payer: &'a Signer<'info>,
    /// Anchor event CPI authority for ZamaHost.
    pub event_authority: &'a UncheckedAccount<'info>,
    /// ZamaHost program account.
    pub zama_program: &'a Program<'info, ZamaHost>,
    /// Host config used for chain-id-aware handle derivation.
    pub host_config: &'a Account<'info, HostConfig>,
    /// Canonical deny-record PDA witnesses supplied as instruction remaining accounts.
    pub deny_subject_records: &'a [AccountInfo<'info>],
    /// Program-controlled compute signer PDA and its ACL domain.
    pub compute_authority: ComputeAuthority<'info>,
    /// System program used for output ACL creation.
    pub system_program: &'a Program<'info, System>,
    /// Per-`compute_subject` HCU block meter forwarded into the host `fhe_execute` CPI (`None` unless
    /// the caller threads it; behavior-neutral while the host cap is unrestricted). The host keys
    /// the meter on `compute_subject` — here the mint's compute signer PDA — so metering stays
    /// per-mint automatically, with no separate HCU authority account.
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

/// Invokes one FHE execution under the current token account authority model.
pub(crate) fn execute<'info>(request: Execute<'_, 'info>) -> Result<()> {
    let app_authority_key = request.execution.app_authority().pubkey();
    let app_authority = request
        .accounts
        .output_authority(app_authority_key)
        .ok_or_else(|| error!(ConfidentialTokenError::MissingFheOutputAuthority))?;
    require_keys_eq!(
        app_authority.key(),
        app_authority_key,
        ConfidentialTokenError::MissingFheOutputAuthority
    );
    let compute_bump = [request.context.compute_authority.bump];
    let compute_signer_seeds = request
        .context
        .compute_authority
        .signer_seeds(&compute_bump);
    let app_authority_seed_bytes = app_authority.signer.seed_bytes();
    let app_authority_seeds: Vec<&[u8]> =
        app_authority_seed_bytes.iter().map(Vec::as_slice).collect();
    let mut additional_authorities = Vec::new();
    for authority in request.execution.additional_output_authorities() {
        if authority == app_authority.key() {
            continue;
        }
        if additional_authorities.contains(&authority) {
            continue;
        }
        additional_authorities.push(authority);
    }
    let extra_output_authorities: Vec<OutputAuthority<'info>> = additional_authorities
        .iter()
        .map(|authority| {
            let resolved = request
                .accounts
                .output_authority(*authority)
                .ok_or_else(|| error!(ConfidentialTokenError::MissingFheOutputAuthority))?;
            require_keys_eq!(
                resolved.key(),
                *authority,
                ConfidentialTokenError::MissingFheOutputAuthority
            );
            Ok(resolved)
        })
        .collect::<Result<Vec<_>>>()?;
    let extra_output_authority_seed_bytes: Vec<Vec<Vec<u8>>> = extra_output_authorities
        .iter()
        .map(|authority| authority.signer.seed_bytes())
        .collect();
    let extra_output_authority_seeds: Vec<Vec<&[u8]>> = extra_output_authority_seed_bytes
        .iter()
        .map(|seed_bytes| seed_bytes.iter().map(Vec::as_slice).collect())
        .collect();

    let mut signer_seed_vec: Vec<&[&[u8]]> = vec![compute_signer_seeds.as_slice()];
    if !app_authority_seeds.is_empty() {
        signer_seed_vec.push(app_authority_seeds.as_slice());
    }
    for seeds in &extra_output_authority_seeds {
        signer_seed_vec.push(seeds.as_slice());
    }
    validate_deny_subject_records_for_grant_subjects(
        request.context.host_config.grant_deny_list_enabled,
        request.context.deny_subject_records,
        app_authority.key(),
        &extra_output_authorities,
        &request.execution.newly_granted_subjects(),
    )?;

    request.execution.invoke(
        zama_fhe::ExecutionCpiAccounts {
            payer: request.context.payer.to_account_info(),
            compute_subject: request.context.compute_authority.account_info(),
            encrypted_value_account_authority: app_authority.account.clone(),
            host_config: request.context.host_config.to_account_info(),
            deny_subject_records: request.context.deny_subject_records,
            system_program: request.context.system_program.to_account_info(),
            hcu_block_meter: request.context.hcu_block_meter.clone(),
            hcu_trusted_app_record: request.context.hcu_trusted_app_record.clone(),
            event_authority: request.context.event_authority.to_account_info(),
            program: request.context.zama_program.to_account_info(),
        },
        request.accounts.resolved_accounts(),
        &signer_seed_vec,
    )?;
    Ok(())
}

fn validate_deny_subject_records_for_grant_subjects<'info>(
    deny_list_enabled: bool,
    supplied_records: &[AccountInfo<'info>],
    app_authority: Pubkey,
    extra_output_authorities: &[OutputAuthority<'info>],
    newly_granted_subjects: &[Pubkey],
) -> Result<()> {
    if !deny_list_enabled {
        require!(
            supplied_records.is_empty(),
            ConfidentialTokenError::UnexpectedRemainingAccounts
        );
        return Ok(());
    }

    for (index, supplied) in supplied_records.iter().enumerate() {
        require!(
            !supplied_records[index + 1..]
                .iter()
                .any(|later| later.key() == supplied.key()),
            ConfidentialTokenError::UnexpectedRemainingAccounts
        );
        // A supplied deny record must witness either an output authority or a subject a
        // persistent output grants for the first time (created or update-added) — the host
        // deny-list-checks both, so both may reach it through remaining accounts.
        require!(
            is_deny_record_for_authority(supplied.key(), app_authority)
                || extra_output_authorities
                    .iter()
                    .any(|authority| is_deny_record_for_authority(supplied.key(), authority.key()))
                || newly_granted_subjects
                    .iter()
                    .any(|subject| is_deny_record_for_authority(supplied.key(), *subject)),
            ConfidentialTokenError::UnexpectedRemainingAccounts
        );
    }
    Ok(())
}

fn is_deny_record_for_authority(record: Pubkey, authority: Pubkey) -> bool {
    zama_host::deny_subject_address(authority).0 == record
}

#[cfg(test)]
mod tests {
    use super::*;

    fn audience_subjects(audience: PersistentAudience) -> Vec<Pubkey> {
        audience.into_subjects()
    }

    #[test]
    fn holder_audience_grants_owner_then_compute() {
        let owner = Pubkey::new_unique();
        let compute = Pubkey::new_unique();
        assert_eq!(
            audience_subjects(PersistentAudience::for_owner(owner, compute)),
            vec![owner, compute]
        );
    }

    #[test]
    fn holder_audience_appends_extra_owner_before_compute() {
        let owner = Pubkey::new_unique();
        let recipient = Pubkey::new_unique();
        let compute = Pubkey::new_unique();
        assert_eq!(
            audience_subjects(PersistentAudience::for_owner(owner, compute).with_owner(recipient)),
            vec![owner, recipient, compute]
        );
    }

    #[test]
    fn compute_only_audience_grants_compute_and_no_owner() {
        let compute = Pubkey::new_unique();
        assert_eq!(
            audience_subjects(PersistentAudience::compute_only(compute)),
            vec![compute]
        );
    }

    #[test]
    fn duplicate_owner_and_compute_are_rejected() {
        let owner = Pubkey::new_unique();
        let compute = Pubkey::new_unique();
        // A holder audience whose extra owner repeats the compute signer
        // renders a duplicate subject; the persistent output rejects it.
        let output = zama_fhe::PersistentOutput::create(
            zama_fhe::EncryptedValueId::new(
                zama_fhe::Domain::new(Pubkey::new_unique()),
                Pubkey::new_unique(),
                zama_fhe::EncryptedValueLabel::new([1; 32]),
            ),
            PersistentAudience::for_owner(owner, compute)
                .with_owner(compute)
                .into_subjects(),
        );
        assert!(output.binding().is_err());
    }

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

    fn output_authority(pubkey: Pubkey) -> OutputAuthority<'static> {
        OutputAuthority {
            account: account_info(pubkey, false),
            signer: Box::new(OutputAuthoritySigner::transaction_signer()),
        }
    }

    fn encrypted_value_id(account: Pubkey, label_tag: u8) -> zama_fhe::EncryptedValueId {
        zama_fhe::EncryptedValueId::new(
            zama_fhe::Domain::new(Pubkey::new_unique()),
            account,
            zama_fhe::EncryptedValueLabel::new(handle(label_tag)),
        )
    }

    fn subjects(subject: Pubkey) -> Vec<Pubkey> {
        vec![subject]
    }

    fn sample_plan() -> (zama_fhe::FheExecution, Pubkey, Pubkey, Pubkey) {
        let authority = Pubkey::new_unique();
        let input_key = encrypted_value_id(authority, 1);
        let input_acl = input_key.address();
        let output_key = encrypted_value_id(authority, 2);
        let output_acl = output_key.address();
        let input = zama_fhe::Uint64Handle::persistent(balance_handle(1), input_key).unwrap();
        let execution = zama_fhe::FheExecution::build(
            zama_fhe::ExecutionAppAuthority::new(authority),
            |builder| {
                builder.add(
                    input,
                    zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(1),
                    zama_fhe::Output::persistent(zama_fhe::PersistentOutput::create(
                        output_key,
                        subjects(authority),
                    )),
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
            vec![output_authority(authority)],
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
            vec![output_authority(authority)],
        )
        .err()
        .unwrap();
        assert_token_error(error, ConfidentialTokenError::UnexpectedFheExecuteAccount);

        let error = ExecutionAccountSet::for_execution(
            &execution,
            vec![account_info(output_acl, true)],
            vec![output_authority(authority)],
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
            vec![output_authority(authority)],
        )
        .err()
        .unwrap();
        assert_token_error(error, ConfidentialTokenError::FheExecuteAccountNotWritable);
    }

    #[test]
    fn batch_account_set_maps_output_authority_errors() {
        let (execution, input_acl, output_acl, authority) = sample_plan();

        let error = ExecutionAccountSet::for_execution(
            &execution,
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
            ],
            vec![output_authority(authority), output_authority(authority)],
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
                output_authority(authority),
                output_authority(Pubkey::new_unique()),
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
