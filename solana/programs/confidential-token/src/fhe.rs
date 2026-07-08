//! Token-local FHE helper functions.
//!
//! The confidential token program keeps ZamaHost CPI assembly in this module
//! so business logic can build typed eval frames and receive host-verified
//! output handles.

use anchor_lang::{prelude::*, AccountDeserialize};
use zama_host::{
    cpi, cpi::accounts::AllowForDecryption as HostAllowForDecryption, program::ZamaHost, HostConfig,
};

use crate::{
    compute_signer_address, hcu_authority_address, token_account_address,
    total_supply_authority_address, ConfidentialTokenAccount, ConfidentialTokenError,
};

/// A durable eval output account bound to the exact slot it is allowed to create.
pub(crate) struct DurableOutput<'info> {
    acl_record: AccountInfo<'info>,
    output: Box<zama_fhe::DurableOutput>,
}

impl<'info> DurableOutput<'info> {
    pub(crate) fn new(
        acl_record: AccountInfo<'info>,
        slot: zama_fhe::DurableSlot,
        access: zama_fhe::AccessPolicy,
    ) -> Result<Self> {
        require_keys_eq!(
            acl_record.key(),
            slot.address(),
            ConfidentialTokenError::AmountAclMismatch
        );
        require_keys_eq!(
            *acl_record.owner,
            System::id(),
            ConfidentialTokenError::AmountAclMismatch
        );
        require!(
            acl_record.data_is_empty(),
            ConfidentialTokenError::AmountAclMismatch
        );
        require!(
            !acl_record.executable,
            ConfidentialTokenError::AmountAclMismatch
        );
        let output = zama_fhe::DurableOutput::new(slot, access);
        output.birth().map_err(|error| {
            msg!("invalid durable FHE output: {:?}", error);
            error!(ConfidentialTokenError::InvalidFheEvalPlan)
        })?;
        Ok(Self {
            acl_record,
            output: Box::new(output),
        })
    }

    pub(crate) fn output(&self) -> zama_fhe::Output {
        zama_fhe::Output::durable_output((*self.output).clone())
    }

    pub(crate) fn handle(&self) -> Result<[u8; 32]> {
        let birth = self.birth()?;
        require_keys_eq!(
            self.acl_record.key(),
            birth.acl_record(),
            ConfidentialTokenError::CurrentAclRecordMismatch
        );
        require_keys_eq!(
            *self.acl_record.owner,
            zama_host::ID,
            ConfidentialTokenError::CurrentAclRecordMismatch
        );
        require!(
            self.acl_record.data_len() == 8 + zama_host::AclRecord::SPACE,
            ConfidentialTokenError::CurrentAclRecordMismatch
        );
        let data = self.acl_record.try_borrow_data()?;
        let mut data_slice: &[u8] = &data;
        let record = zama_host::AclRecord::try_deserialize(&mut data_slice)?;
        require!(
            zama_host::acl_record_subject_slots_are_canonical(&record),
            ConfidentialTokenError::CurrentAclRecordMismatch
        );
        require!(
            record.nonce_key == birth.nonce_key()
                && record.nonce_sequence == birth.sequence()
                && record.acl_domain_key == birth.acl_domain_key()
                && record.app_account == birth.app_account()
                && record.encrypted_value_label == birth.encrypted_value_label()
                && record.public_decrypt == birth.public_decrypt()
                && record.subject_count as usize == birth.subjects().len()
                && record.overflow_subject_count == 0
                && record.bump
                    == zama_host::acl_record_address(birth.nonce_key(), birth.sequence()).1,
            ConfidentialTokenError::CurrentAclRecordMismatch
        );
        for (index, subject) in birth.subjects().iter().enumerate() {
            require!(
                subject.matches_record_entry(record.subjects[index], record.subject_roles[index]),
                ConfidentialTokenError::CurrentAclRecordMismatch
            );
        }
        Ok(record.handle)
    }

    pub(crate) fn account_info(&self) -> AccountInfo<'info> {
        self.acl_record.clone()
    }

    fn birth(&self) -> Result<zama_fhe::DurableOutputBirth> {
        self.output.birth().map_err(|error| {
            msg!("invalid durable FHE output: {:?}", error);
            error!(ConfidentialTokenError::InvalidFheEvalPlan)
        })
    }
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

/// Mint-scoped HCU authority PDA: the identity the host's HCU block cap meters and trusts for
/// this mint's evals. Program-signed via CPI seeds so no other caller can spend this mint's
/// meter or claim its trusted bypass.
#[derive(Clone)]
pub(crate) struct HcuAuthority<'info> {
    account: AccountInfo<'info>,
    mint: Pubkey,
    bump: u8,
}

impl<'info> HcuAuthority<'info> {
    pub(crate) fn for_mint(account: &UncheckedAccount<'info>, mint: Pubkey) -> Result<Self> {
        let (expected, bump) = hcu_authority_address(mint);
        require_keys_eq!(
            account.key(),
            expected,
            ConfidentialTokenError::HcuAuthorityMismatch
        );
        Ok(Self {
            account: account.to_account_info(),
            mint,
            bump,
        })
    }

    fn account_info(&self) -> AccountInfo<'info> {
        self.account.clone()
    }

    fn signer_seeds<'a>(&'a self, bump: &'a [u8; 1]) -> [&'a [u8]; 3] {
        [b"hcu-authority", self.mint.as_ref(), bump]
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
    /// Program-controlled compute signer PDA and its ACL domain.
    pub compute_authority: ComputeAuthority<'info>,
    /// System program used for output ACL creation.
    pub system_program: &'a Program<'info, System>,
    /// Mint-scoped HCU authority signed into the host `fhe_eval` CPI — the identity the block
    /// cap meters and trusts. Mandatory on every eval, matching the host account shape.
    pub hcu_authority: HcuAuthority<'info>,
    /// Per-app HCU block meter forwarded into the host `fhe_eval` CPI (`None` unless the caller
    /// threads it; behavior-neutral while the host cap is unrestricted).
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

    let hcu_authority_bump = [request.context.hcu_authority.bump];
    let hcu_authority_seeds = request
        .context
        .hcu_authority
        .signer_seeds(&hcu_authority_bump);

    let mut signer_seed_vec: Vec<&[&[u8]]> = vec![compute_signer_seeds.as_slice()];
    if !app_authority_seeds.is_empty() {
        signer_seed_vec.push(app_authority_seeds.as_slice());
    }
    for seeds in &extra_output_authority_seeds {
        signer_seed_vec.push(seeds.as_slice());
    }
    signer_seed_vec.push(hcu_authority_seeds.as_slice());

    zama_fhe::invoke_eval_signed_resolved(
        &request.plan,
        zama_fhe::EvalCpiAccounts {
            payer: request.context.payer.to_account_info(),
            compute_subject: request.context.compute_authority.account_info(),
            app_account_authority: app_authority.account.clone(),
            host_config: request.context.host_config.to_account_info(),
            system_program: request.context.system_program.to_account_info(),
            hcu_authority: request.context.hcu_authority.account_info(),
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

/// Inputs required to mark a host handle publicly decryptable.
pub struct AllowPublicDecrypt<'a, 'info> {
    /// Subject that already has `ACL_ROLE_PUBLIC_DECRYPT` on the ACL record.
    pub authority: &'a Signer<'info>,
    /// Optional overflow permission witness when the authority is not inline.
    pub authority_permission_record: Option<AccountInfo<'info>>,
    /// ACL record whose public-decrypt flag is updated.
    pub acl_record: AccountInfo<'info>,
    /// ZamaHost config account.
    pub host_config: &'a Account<'info, HostConfig>,
    /// Optional deny-list witness when grant deny-lists are enabled.
    pub deny_subject_record: Option<AccountInfo<'info>>,
    /// Anchor event CPI authority for ZamaHost.
    pub event_authority: &'a UncheckedAccount<'info>,
    /// ZamaHost program account.
    pub zama_program: &'a Program<'info, ZamaHost>,
    /// Handle stored in `acl_record`.
    pub handle: [u8; 32],
}

/// Delegates public-decrypt authorization to ZamaHost.
pub fn allow_public_decrypt<'info>(request: AllowPublicDecrypt<'_, 'info>) -> Result<()> {
    cpi::allow_for_decryption(
        CpiContext::new(
            request.zama_program.key(),
            HostAllowForDecryption {
                authority: request.authority.to_account_info(),
                authority_permission_record: request.authority_permission_record,
                acl_record: request.acl_record,
                host_config: request.host_config.to_account_info(),
                deny_subject_record: request.deny_subject_record,
                event_authority: request.event_authority.to_account_info(),
                program: request.zama_program.to_account_info(),
            },
        ),
        request.handle,
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

    fn durable_slot(account: Pubkey, sequence: u64) -> zama_fhe::DurableSlot {
        zama_fhe::DurableSlot::new(
            Pubkey::new_unique(),
            account,
            zama_fhe::DurableLabel::new(handle(5)),
            sequence,
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
