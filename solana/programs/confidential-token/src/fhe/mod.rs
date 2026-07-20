//! Token-local FHE helper functions.
//!
//! The confidential token program keeps ZamaHost CPI assembly in this module
//! so business logic can build typed eval frames and receive host-verified
//! output handles.

use anchor_lang::{prelude::*, AccountDeserialize};
use zama_host::{program::ZamaHost, EncryptedValue, HostConfig};

use crate::{
    compute_signer_address, token_account_address, total_supply_authority_address,
    ConfidentialTokenAccount, ConfidentialTokenError,
};

mod verify_public_decrypt;
pub(crate) use verify_public_decrypt::*;

/// Audience for a confidential-token durable output.
///
/// Holder-scoped lineages (balances, `transferred_amount`, `burned_amount`) must
/// always grant the holder's owner key and the mint compute-signer PDA: the owner
/// keeps decrypt access to their own value, and the compute signer gates the next
/// eval that reads it. [`DurableAudience::for_owner`] takes both as required
/// parameters, so a holder output can never be built missing either; extra owners
/// (the recipient leg of a `transferred_amount` rotation) are additive via
/// [`DurableAudience::with_owner`]. Mint-scoped lineages with no single holder
/// (total supply, freshly minted random amounts) use
/// [`DurableAudience::compute_only`], the one owner-less path.
///
/// This is the only way token instructions produce durable-output policies:
/// [`DurableOutput::new`]/[`DurableOutput::new_public`] accept a `DurableAudience`
/// and not a raw [`zama_fhe::AccessPolicy`], so the owner+compute invariant holds
/// by construction rather than by convention at each call site.
pub(crate) struct DurableAudience {
    owner: Option<Pubkey>,
    extra_owners: Vec<Pubkey>,
    compute: Pubkey,
}

impl DurableAudience {
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

    /// Adds an extra owner subject (the recipient of a `transferred_amount` leg).
    pub(crate) fn with_owner(mut self, owner: Pubkey) -> Self {
        self.extra_owners.push(owner);
        self
    }

    /// Renders the audience into host output subjects, ordered owner(s) then
    /// compute signer.
    fn into_policy(self) -> zama_fhe::Result<zama_fhe::AccessPolicy> {
        let mut subjects = Vec::with_capacity(2 + self.extra_owners.len());
        subjects.extend(self.owner.map(zama_fhe::AccessSubject::owner));
        subjects.extend(
            self.extra_owners
                .into_iter()
                .map(zama_fhe::AccessSubject::owner),
        );
        subjects.push(zama_fhe::AccessSubject::compute(self.compute));
        zama_fhe::AccessPolicy::from_subjects(subjects)
    }
}

/// A durable eval output account bound to the exact `EncryptedValue` lineage
/// it is allowed to create or supersede.
pub(crate) struct DurableOutput<'info> {
    encrypted_value: AccountInfo<'info>,
    output: Box<zama_fhe::DurableOutput>,
}

impl<'info> DurableOutput<'info> {
    /// Binds `encrypted_value` as the output of a durable eval step: creates the
    /// lineage's first handle if the PDA does not exist yet, or supersedes it
    /// (reading `previous_handle`/`previous_subjects` off the on-chain account)
    /// if it does. Either way the eval CPI's attestation matches exactly what
    /// the host will verify.
    pub(crate) fn new(
        encrypted_value: AccountInfo<'info>,
        slot: zama_fhe::DurableSlot,
        audience: DurableAudience,
    ) -> Result<Self> {
        Self::new_inner(encrypted_value, slot, audience, false)
    }

    /// Like [`new`], but binds the output born publicly decryptable: the host
    /// seals a public-decrypt leaf for the new handle inside the same eval CPI
    /// (EVM `unwrap` parity; DD-036). Used by `confidential_burn` for the burned
    /// delta so every burn stays permanently redeemable with no second CPI.
    pub(crate) fn new_public(
        encrypted_value: AccountInfo<'info>,
        slot: zama_fhe::DurableSlot,
        audience: DurableAudience,
    ) -> Result<Self> {
        Self::new_inner(encrypted_value, slot, audience, true)
    }

    fn new_inner(
        encrypted_value: AccountInfo<'info>,
        slot: zama_fhe::DurableSlot,
        audience: DurableAudience,
        make_public: bool,
    ) -> Result<Self> {
        require_keys_eq!(
            encrypted_value.key(),
            slot.address(),
            ConfidentialTokenError::CurrentEncryptedValueMismatch
        );
        let access = audience.into_policy().map_err(|error| {
            msg!("invalid durable FHE output audience: {:?}", error);
            error!(ConfidentialTokenError::InvalidFheEvalPlan)
        })?;
        let output = if *encrypted_value.owner == System::id() {
            require!(
                encrypted_value.data_is_empty() && !encrypted_value.executable,
                ConfidentialTokenError::InvalidFheEvalPlan
            );
            zama_fhe::DurableOutput::create(slot, access)
        } else {
            let value = read_encrypted_value(&encrypted_value)?;
            zama_fhe::DurableOutput::supersede(
                slot,
                access,
                value.current_handle,
                value.subjects.clone(),
            )
        }
        .with_make_public(make_public);
        output.birth().map_err(|error| {
            msg!("invalid durable FHE output: {:?}", error);
            error!(ConfidentialTokenError::InvalidFheEvalPlan)
        })?;
        Ok(Self {
            encrypted_value,
            output: Box::new(output),
        })
    }

    pub(crate) fn output(&self) -> zama_fhe::Output {
        zama_fhe::Output::durable_output((*self.output).clone())
    }

    /// Reads the handle the host bound into `encrypted_value` by this eval CPI.
    /// Call only after the CPI that carries this output has executed.
    pub(crate) fn handle(&self) -> Result<[u8; 32]> {
        let birth = self.birth()?;
        require_keys_eq!(
            self.encrypted_value.key(),
            birth.encrypted_value(),
            ConfidentialTokenError::CurrentEncryptedValueMismatch
        );
        let value = read_encrypted_value(&self.encrypted_value)?;
        Ok(value.current_handle)
    }

    pub(crate) fn account_info(&self) -> AccountInfo<'info> {
        self.encrypted_value.clone()
    }

    fn birth(&self) -> Result<zama_fhe::DurableOutputBirth> {
        self.output.birth().map_err(|error| {
            msg!("invalid durable FHE output: {:?}", error);
            error!(ConfidentialTokenError::InvalidFheEvalPlan)
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
    acl_domain_key: Pubkey,
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
            acl_domain_key: mint,
            bump,
        })
    }

    fn account_info(&self) -> AccountInfo<'info> {
        self.account.clone()
    }

    fn signer_seeds<'a>(&'a self, bump: &'a [u8; 1]) -> [&'a [u8]; 3] {
        [b"fhe-compute", self.acl_domain_key.as_ref(), bump]
    }
}

/// Signer model for a durable output authority required by an eval frame.
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

/// Durable output authority account plus the signer model that authorizes it.
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

/// Pubkey-indexed accounts and authorities available to satisfy an eval plan.
pub(crate) struct EvalAccountSet<'info> {
    accounts: zama_fhe::ResolvedEvalAccounts<'info>,
    output_authorities: Vec<OutputAuthority<'info>>,
}

impl<'info> EvalAccountSet<'info> {
    pub(crate) fn for_plan(
        plan: &zama_fhe::EvalPlan,
        available_accounts: impl IntoIterator<Item = AccountInfo<'info>>,
        output_authorities: impl IntoIterator<Item = OutputAuthority<'info>>,
    ) -> Result<Self> {
        let output_authorities = output_authorities.into_iter().collect::<Vec<_>>();
        let output_authority_accounts = output_authorities
            .iter()
            .map(OutputAuthority::account_info)
            .collect::<Vec<_>>();
        let accounts = plan
            .resolve_accounts(available_accounts, output_authority_accounts)
            .map_err(map_eval_account_resolution_error)?;

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

    fn resolved_accounts(&self) -> &zama_fhe::ResolvedEvalAccounts<'info> {
        &self.accounts
    }
}

fn map_eval_account_resolution_error(error: zama_fhe::EvalAccountResolutionError) -> Error {
    msg!("invalid FHE eval account set: {:?}", error);
    match error {
        zama_fhe::EvalAccountResolutionError::DuplicateDynamicAccount { .. } => {
            error!(ConfidentialTokenError::DuplicateFheEvalAccount)
        }
        zama_fhe::EvalAccountResolutionError::UnexpectedDynamicAccount { .. } => {
            error!(ConfidentialTokenError::UnexpectedFheEvalAccount)
        }
        zama_fhe::EvalAccountResolutionError::MissingDynamicAccount { .. } => {
            error!(ConfidentialTokenError::MissingFheEvalAccount)
        }
        zama_fhe::EvalAccountResolutionError::DynamicAccountNotWritable { .. } => {
            error!(ConfidentialTokenError::FheEvalAccountNotWritable)
        }
        zama_fhe::EvalAccountResolutionError::DuplicateOutputAuthority { .. } => {
            error!(ConfidentialTokenError::DuplicateFheOutputAuthority)
        }
        zama_fhe::EvalAccountResolutionError::UnexpectedOutputAuthority { .. } => {
            error!(ConfidentialTokenError::UnexpectedFheOutputAuthority)
        }
        zama_fhe::EvalAccountResolutionError::MissingOutputAuthority { .. } => {
            error!(ConfidentialTokenError::MissingFheOutputAuthority)
        }
    }
}

/// Inputs required to evaluate an instruction-local FHE plan.
pub(crate) struct EvalContext<'a, 'info> {
    /// Transaction payer and rent payer for any durable output ACL records.
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
    /// Per-`compute_subject` HCU block meter forwarded into the host `fhe_eval` CPI (`None` unless
    /// the caller threads it; behavior-neutral while the host cap is unrestricted). The host keys
    /// the meter on `compute_subject` — here the mint's compute signer PDA — so metering stays
    /// per-mint automatically, with no separate HCU authority account.
    pub hcu_block_meter: Option<AccountInfo<'info>>,
    /// HCU trust witness forwarded into the host `fhe_eval` CPI (`None` unless threaded).
    pub hcu_trusted_app_record: Option<AccountInfo<'info>>,
}

/// Inputs required to evaluate an instruction-local FHE plan.
pub(crate) struct Eval<'a, 'info> {
    /// Fixed ZamaHost CPI accounts shared by eval requests in this instruction.
    pub context: EvalContext<'a, 'info>,
    /// Typed resolver for dynamic accounts required by the eval plan.
    pub accounts: &'a EvalAccountSet<'info>,
    /// SDK-built host eval request and dynamic account role plan.
    pub plan: zama_fhe::EvalPlan,
}

/// Evaluates an FHE plan using the current token account authority model.
pub(crate) fn eval<'info>(request: Eval<'_, 'info>) -> Result<()> {
    let app_authority_key = request.plan.app_authority().pubkey();
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
    for authority in request.plan.additional_output_authorities() {
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
        &request.plan.rotation_added_subjects(),
    )?;

    zama_fhe::invoke_eval_signed_resolved(
        &request.plan,
        zama_fhe::EvalCpiAccounts {
            payer: request.context.payer.to_account_info(),
            compute_subject: request.context.compute_authority.account_info(),
            app_account_authority: app_authority.account.clone(),
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
    rotation_added_subjects: &[Pubkey],
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
        // supersede grants for the first time (rotation-added) — the host deny-list-checks
        // both, so both may reach it through remaining accounts.
        require!(
            is_deny_record_for_authority(supplied.key(), app_authority)
                || extra_output_authorities
                    .iter()
                    .any(|authority| is_deny_record_for_authority(supplied.key(), authority.key()))
                || rotation_added_subjects
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

    fn audience_subjects(audience: DurableAudience) -> Vec<Pubkey> {
        audience
            .into_policy()
            .unwrap()
            .subjects()
            .iter()
            .map(|subject| subject.pubkey())
            .collect()
    }

    #[test]
    fn holder_audience_grants_owner_then_compute() {
        let owner = Pubkey::new_unique();
        let compute = Pubkey::new_unique();
        assert_eq!(
            audience_subjects(DurableAudience::for_owner(owner, compute)),
            vec![owner, compute]
        );
    }

    #[test]
    fn holder_audience_appends_extra_owner_before_compute() {
        let owner = Pubkey::new_unique();
        let recipient = Pubkey::new_unique();
        let compute = Pubkey::new_unique();
        assert_eq!(
            audience_subjects(DurableAudience::for_owner(owner, compute).with_owner(recipient)),
            vec![owner, recipient, compute]
        );
    }

    #[test]
    fn compute_only_audience_grants_compute_and_no_owner() {
        let compute = Pubkey::new_unique();
        assert_eq!(
            audience_subjects(DurableAudience::compute_only(compute)),
            vec![compute]
        );
    }

    #[test]
    fn duplicate_owner_and_compute_are_rejected() {
        let owner = Pubkey::new_unique();
        let compute = Pubkey::new_unique();
        // A holder audience whose extra owner repeats the compute signer would
        // render a duplicate subject; `into_policy` must reject it.
        assert!(DurableAudience::for_owner(owner, compute)
            .with_owner(compute)
            .into_policy()
            .is_err());
    }

    fn handle(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn balance_handle(tag: u8) -> [u8; 32] {
        let mut handle = [tag; 32];
        handle[30] = crate::BALANCE_FHE_TYPE;
        handle
    }

    fn context_id(tag: u8) -> zama_fhe::EvalContextId {
        zama_fhe::EvalContextId::new(handle(tag)).unwrap()
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

    fn durable_slot(account: Pubkey, label_tag: u8) -> zama_fhe::DurableSlot {
        zama_fhe::DurableSlot::new(
            Pubkey::new_unique(),
            account,
            zama_fhe::DurableLabel::new(handle(label_tag)),
        )
    }

    fn access_policy(subject: Pubkey) -> zama_fhe::AccessPolicy {
        zama_fhe::AccessPolicy::for_owner(subject).unwrap()
    }

    fn sample_plan() -> (zama_fhe::EvalPlan, Pubkey, Pubkey, Pubkey) {
        let authority = Pubkey::new_unique();
        let input_slot = durable_slot(authority, 1);
        let input_acl = input_slot.address();
        let output_slot = durable_slot(authority, 2);
        let output_acl = output_slot.address();
        let input = zama_fhe::Uint64Handle::durable(balance_handle(1), input_slot).unwrap();
        let mut builder =
            zama_fhe::EvalBuilder::new(context_id(9), zama_fhe::EvalAppAuthority::new(authority));
        builder
            .add(
                input,
                zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(1),
                zama_fhe::Output::durable(output_slot, access_policy(authority)),
            )
            .unwrap();
        (builder.finish().unwrap(), input_acl, output_acl, authority)
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
    fn eval_account_set_maps_dynamic_account_errors() {
        let (plan, input_acl, output_acl, authority) = sample_plan();

        let error = EvalAccountSet::for_plan(
            &plan,
            vec![
                account_info(input_acl, false),
                account_info(input_acl, false),
                account_info(output_acl, true),
            ],
            vec![output_authority(authority)],
        )
        .err()
        .unwrap();
        assert_token_error(error, ConfidentialTokenError::DuplicateFheEvalAccount);

        let error = EvalAccountSet::for_plan(
            &plan,
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
                account_info(Pubkey::new_unique(), false),
            ],
            vec![output_authority(authority)],
        )
        .err()
        .unwrap();
        assert_token_error(error, ConfidentialTokenError::UnexpectedFheEvalAccount);

        let error = EvalAccountSet::for_plan(
            &plan,
            vec![account_info(output_acl, true)],
            vec![output_authority(authority)],
        )
        .err()
        .unwrap();
        assert_token_error(error, ConfidentialTokenError::MissingFheEvalAccount);

        let error = EvalAccountSet::for_plan(
            &plan,
            vec![
                account_info(input_acl, false),
                account_info(output_acl, false),
            ],
            vec![output_authority(authority)],
        )
        .err()
        .unwrap();
        assert_token_error(error, ConfidentialTokenError::FheEvalAccountNotWritable);
    }

    #[test]
    fn eval_account_set_maps_output_authority_errors() {
        let (plan, input_acl, output_acl, authority) = sample_plan();

        let error = EvalAccountSet::for_plan(
            &plan,
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
            ],
            vec![output_authority(authority), output_authority(authority)],
        )
        .err()
        .unwrap();
        assert_token_error(error, ConfidentialTokenError::DuplicateFheOutputAuthority);

        let error = EvalAccountSet::for_plan(
            &plan,
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

        let error = EvalAccountSet::for_plan(
            &plan,
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
