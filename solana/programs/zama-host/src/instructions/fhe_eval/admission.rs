use super::super::common::{
    assert_output_acl_metadata, assert_transient_grant, read_transient_capability_for_eval,
    read_transient_session,
};
use super::handles::{
    expected_binary_eval_result, expected_rand_eval_seed, expected_ternary_eval_result,
    expected_trivial_eval_result, EvalHandleContext,
};
use super::*;

#[allow(clippy::too_many_arguments)]
pub(super) fn admit_eval_frame<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    args: &FheEvalArgs,
    subject: Pubkey,
    session_authority: Pubkey,
    current_slot: u64,
    handle_context: &EvalHandleContext<'_>,
    instructions_sysvar: Option<&AccountInfo<'info>>,
) -> Result<()> {
    let mut admission = AdmissionState::new(args.steps.len());
    for (index, step) in args.steps.iter().enumerate() {
        match step {
            FheEvalStep::Binary {
                op,
                lhs,
                rhs,
                output_fhe_type,
                output,
            } => {
                let lhs = admission.resolve_lhs_operand(
                    ctx.remaining_accounts,
                    lhs,
                    subject,
                    session_authority,
                    handle_context.chain_id,
                    current_slot,
                    instructions_sysvar,
                )?;
                let rhs = admission.resolve_rhs_operand(
                    ctx.remaining_accounts,
                    rhs,
                    subject,
                    session_authority,
                    handle_context.chain_id,
                    current_slot,
                    instructions_sysvar,
                )?;
                assert_binary_operand_types(
                    *op,
                    lhs.handle,
                    rhs.handle,
                    rhs.scalar,
                    *output_fhe_type,
                )?;
                let result = expected_binary_eval_result(
                    *op,
                    lhs.handle,
                    rhs.handle,
                    rhs.scalar,
                    *output_fhe_type,
                    handle_context,
                    index as u16,
                    output,
                );
                admission.accept_output(
                    ctx,
                    result,
                    output,
                    input_session_policies(&lhs, &rhs),
                    inputs_allow_public_decrypt(&lhs, &rhs),
                    true,
                    current_slot,
                )?;
            }
            FheEvalStep::Ternary {
                op,
                control,
                if_true,
                if_false,
                output_fhe_type,
                output,
            } => {
                let control = admission.resolve_encrypted_operand(
                    ctx.remaining_accounts,
                    control,
                    subject,
                    session_authority,
                    handle_context.chain_id,
                    current_slot,
                    instructions_sysvar,
                )?;
                let if_true = admission.resolve_encrypted_operand(
                    ctx.remaining_accounts,
                    if_true,
                    subject,
                    session_authority,
                    handle_context.chain_id,
                    current_slot,
                    instructions_sysvar,
                )?;
                let if_false = admission.resolve_encrypted_operand(
                    ctx.remaining_accounts,
                    if_false,
                    subject,
                    session_authority,
                    handle_context.chain_id,
                    current_slot,
                    instructions_sysvar,
                )?;
                assert_ternary_operand_types(
                    control.handle,
                    if_true.handle,
                    if_false.handle,
                    *output_fhe_type,
                )?;
                let result = expected_ternary_eval_result(
                    *op,
                    control.handle,
                    if_true.handle,
                    if_false.handle,
                    *output_fhe_type,
                    handle_context,
                    index as u16,
                    output,
                );
                admission.accept_output(
                    ctx,
                    result,
                    output,
                    input_session_policies3(&control, &if_true, &if_false),
                    inputs3_allow_public_decrypt(&control, &if_true, &if_false),
                    true,
                    current_slot,
                )?;
            }
            FheEvalStep::TrivialEncrypt {
                plaintext,
                fhe_type,
                output,
            } => {
                assert_supported_fhe_type(*fhe_type)?;
                let result = expected_trivial_eval_result(
                    *plaintext,
                    *fhe_type,
                    handle_context,
                    index as u16,
                    output,
                );
                admission.accept_output(
                    ctx,
                    result,
                    output,
                    Vec::new(),
                    false,
                    false,
                    current_slot,
                )?;
            }
            FheEvalStep::Rand { fhe_type, output } => {
                assert_supported_rand_type(*fhe_type)?;
                let seed = expected_rand_eval_seed(handle_context, index as u16, output);
                let result = computed_rand_handle(seed, *fhe_type, handle_context.chain_id);
                admission.accept_output(
                    ctx,
                    result,
                    output,
                    Vec::new(),
                    false,
                    false,
                    current_slot,
                )?;
            }
        }
    }
    Ok(())
}

struct AdmissionState {
    produced: Vec<ProducedValue>,
    durable_output_accounts: Vec<Pubkey>,
    session_appends: Vec<Pubkey>,
    session_consumes: Vec<(Pubkey, u16)>,
}

impl AdmissionState {
    fn new(step_count: usize) -> Self {
        Self {
            produced: Vec::with_capacity(step_count),
            durable_output_accounts: Vec::new(),
            session_appends: Vec::new(),
            session_consumes: Vec::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn resolve_lhs_operand<'info>(
        &mut self,
        remaining_accounts: &'info [AccountInfo<'info>],
        operand: &FheEvalOperand,
        subject: Pubkey,
        session_authority: Pubkey,
        chain_id: u64,
        current_slot: u64,
        instructions_sysvar: Option<&AccountInfo<'info>>,
    ) -> Result<ResolvedOperand> {
        match operand {
            FheEvalOperand::Scalar(_) => Err(error!(ZamaHostError::InvalidFheEvalAccount)),
            _ => self.resolve_encrypted_operand(
                remaining_accounts,
                operand,
                subject,
                session_authority,
                chain_id,
                current_slot,
                instructions_sysvar,
            ),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn resolve_rhs_operand<'info>(
        &mut self,
        remaining_accounts: &'info [AccountInfo<'info>],
        operand: &FheEvalOperand,
        subject: Pubkey,
        session_authority: Pubkey,
        chain_id: u64,
        current_slot: u64,
        instructions_sysvar: Option<&AccountInfo<'info>>,
    ) -> Result<ResolvedOperand> {
        match operand {
            FheEvalOperand::Scalar(value) => Ok(ResolvedOperand::scalar(*value)),
            _ => self.resolve_encrypted_operand(
                remaining_accounts,
                operand,
                subject,
                session_authority,
                chain_id,
                current_slot,
                instructions_sysvar,
            ),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn resolve_encrypted_operand<'info>(
        &mut self,
        remaining_accounts: &'info [AccountInfo<'info>],
        operand: &FheEvalOperand,
        subject: Pubkey,
        session_authority: Pubkey,
        chain_id: u64,
        current_slot: u64,
        instructions_sysvar: Option<&AccountInfo<'info>>,
    ) -> Result<ResolvedOperand> {
        match operand {
            FheEvalOperand::Durable {
                handle,
                acl_record_index,
                permission_index,
            } => {
                let record_info = account_at(remaining_accounts, *acl_record_index)?;
                let permission_info = permission_index
                    .map(|index| account_at(remaining_accounts, index))
                    .transpose()?;
                assert_unchecked_acl_record_subject_role(
                    record_info,
                    *handle,
                    chain_id,
                    subject,
                    ACL_ROLE_USE,
                    permission_info,
                )?;
                let public_decrypt_allowed = unchecked_acl_record_subject_has_role(
                    record_info,
                    *handle,
                    subject,
                    ACL_ROLE_PUBLIC_DECRYPT,
                    permission_info,
                )?;
                Ok(ResolvedOperand::encrypted(*handle, public_decrypt_allowed))
            }
            FheEvalOperand::Transient { producer_index } => self
                .produced
                .get(*producer_index as usize)
                .map(ResolvedOperand::from_produced)
                .ok_or_else(|| error!(ZamaHostError::FheEvalTransientMissing)),
            FheEvalOperand::TransientSession {
                handle,
                session_index,
                capability_index,
            } => {
                let session_info = account_at(remaining_accounts, *session_index)?;
                let capability = read_transient_capability_for_eval(
                    session_info,
                    session_authority,
                    current_slot,
                    *handle,
                    subject,
                    ACL_ROLE_USE,
                    *capability_index,
                    instructions_sysvar,
                )?;
                self.admit_transient_capability_consume(
                    session_info.key(),
                    *capability_index,
                    capability,
                )?;
                Ok(ResolvedOperand {
                    handle: *handle,
                    scalar: false,
                    public_decrypt_allowed: capability.grant.public_decrypt_allowed,
                    session_policies: vec![SessionPolicy {
                        subject: capability.grant.subject,
                        receiver_program: capability.grant.receiver_program,
                        role_flags: capability.grant.role_flags,
                        max_uses: capability.grant.max_uses,
                        durable_output_allowed: capability.grant.durable_output_allowed,
                        public_decrypt_allowed: capability.grant.public_decrypt_allowed,
                        acl_domain_key: capability.grant.acl_domain_key,
                        app_account: capability.grant.app_account,
                    }],
                })
            }
            FheEvalOperand::Scalar(_) => Err(error!(ZamaHostError::InvalidFheEvalAccount)),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn accept_output<'info>(
        &mut self,
        ctx: &Context<'info, FheEval<'info>>,
        result: [u8; 32],
        output: &FheEvalOutput,
        output_policies: Vec<SessionPolicy>,
        output_public_decrypt_allowed: bool,
        enforce_public_decrypt_role_propagation: bool,
        current_slot: u64,
    ) -> Result<()> {
        require!(
            !self.produced.iter().any(|value| value.handle == result),
            ZamaHostError::FheEvalDuplicateHandle
        );

        match output {
            FheEvalOutput::Transient => {}
            FheEvalOutput::TransientSession {
                session_index,
                capability,
            } => {
                assert_session_policies_allow_transient_grant(&output_policies, *capability)?;
                self.admit_transient_session_append(
                    ctx.remaining_accounts,
                    *session_index,
                    ctx.accounts.app_account_authority.key(),
                    current_slot,
                    *capability,
                )?;
            }
            FheEvalOutput::Durable {
                output_acl_record_index,
                output_app_account_authority_index,
                output_nonce_key,
                output_nonce_sequence,
                output_acl_domain_key,
                output_app_account,
                output_encrypted_value_label,
                output_subjects,
                output_public_decrypt,
            } => {
                assert_session_policies_allow_output(
                    &output_policies,
                    *output_acl_domain_key,
                    *output_app_account,
                    output_subjects,
                    *output_public_decrypt,
                )?;
                let app_account_authority = admit_durable_output_authority(
                    ctx,
                    *output_app_account_authority_index,
                    *output_app_account,
                )?;
                if enforce_public_decrypt_role_propagation {
                    assert_derived_public_decrypt_roles_allowed(
                        output_subjects,
                        output_public_decrypt_allowed,
                        &app_account_authority,
                    )?;
                }
                assert_output_acl_metadata(
                    app_account_authority.key(),
                    *output_nonce_key,
                    *output_acl_domain_key,
                    *output_app_account,
                    *output_encrypted_value_label,
                    output_subjects,
                )?;
                assert_public_decrypt_not_set_at_birth(*output_public_decrypt)?;
                self.admit_durable_output_account(
                    ctx.remaining_accounts,
                    *output_acl_record_index,
                    *output_nonce_key,
                    *output_nonce_sequence,
                )?;
            }
        }

        self.produced.push(ProducedValue {
            handle: result,
            public_decrypt_allowed: output_public_decrypt_allowed,
            session_policies: output_policies,
        });
        Ok(())
    }

    fn admit_durable_output_account<'info>(
        &mut self,
        remaining_accounts: &'info [AccountInfo<'info>],
        output_acl_record_index: u16,
        output_nonce_key: [u8; 32],
        output_nonce_sequence: u64,
    ) -> Result<()> {
        let output_info = account_at(remaining_accounts, output_acl_record_index)?;
        let (expected, _) = acl_record_address(output_nonce_key, output_nonce_sequence);
        require_keys_eq!(
            output_info.key(),
            expected,
            ZamaHostError::AclRecordPdaMismatch
        );
        require!(
            !self
                .durable_output_accounts
                .iter()
                .any(|account| *account == output_info.key()),
            ZamaHostError::FheEvalOutputAlreadyInitialized
        );
        require!(
            output_info.is_writable,
            ZamaHostError::InvalidFheEvalAccount
        );
        require_keys_eq!(
            *output_info.owner,
            System::id(),
            ZamaHostError::FheEvalOutputAlreadyInitialized
        );
        require!(
            output_info.data_is_empty(),
            ZamaHostError::FheEvalOutputAlreadyInitialized
        );
        require!(
            !output_info.executable,
            ZamaHostError::FheEvalOutputAlreadyInitialized
        );
        self.durable_output_accounts.push(output_info.key());
        Ok(())
    }

    fn admit_transient_session_append<'info>(
        &mut self,
        remaining_accounts: &'info [AccountInfo<'info>],
        session_index: u16,
        authority: Pubkey,
        current_slot: u64,
        grant: TransientCapabilityGrant,
    ) -> Result<()> {
        let session_info = account_at(remaining_accounts, session_index)?;
        require!(
            session_info.is_writable,
            ZamaHostError::InvalidFheEvalAccount
        );
        let session = read_transient_session(session_info)?;
        require_keys_eq!(
            session.authority,
            authority,
            ZamaHostError::TransientSessionAuthorityMismatch
        );
        require!(
            session.state == TRANSIENT_SESSION_STATE_OPEN,
            ZamaHostError::TransientSessionStateInvalid
        );
        require!(
            current_slot <= session.expires_slot,
            ZamaHostError::TransientSessionExpired
        );
        let planned_appends = self
            .session_appends
            .iter()
            .filter(|session| **session == session_info.key())
            .count();
        require!(
            session.entries.len() + planned_appends < session.max_entries as usize,
            ZamaHostError::TransientSessionCapacityInvalid
        );
        assert_transient_grant(&session, grant)?;
        self.session_appends.push(session_info.key());
        Ok(())
    }

    fn admit_transient_capability_consume(
        &mut self,
        session_key: Pubkey,
        capability_index: u16,
        capability: TransientCapability,
    ) -> Result<()> {
        let planned_consumes = self
            .session_consumes
            .iter()
            .filter(|(session, index)| *session == session_key && *index == capability_index)
            .count();
        let remaining_uses = capability
            .grant
            .max_uses
            .saturating_sub(capability.used_count) as usize;
        require!(
            planned_consumes < remaining_uses,
            ZamaHostError::TransientCapabilityConsumed
        );
        self.session_consumes.push((session_key, capability_index));
        Ok(())
    }
}

fn admit_durable_output_authority<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    authority_index: Option<u16>,
    output_app_account: Pubkey,
) -> Result<AccountInfo<'info>> {
    match authority_index {
        Some(index) => {
            let authority = account_at(ctx.remaining_accounts, index)?;
            require!(authority.is_signer, ZamaHostError::InvalidFheEvalAccount);
            require_keys_eq!(
                authority.key(),
                output_app_account,
                ZamaHostError::AppAccountAuthorityMismatch
            );
            Ok(authority.clone())
        }
        None => Ok(ctx.accounts.app_account_authority.to_account_info()),
    }
}

fn account_at<'info>(
    remaining_accounts: &'info [AccountInfo<'info>],
    index: u16,
) -> Result<&'info AccountInfo<'info>> {
    remaining_accounts
        .get(index as usize)
        .ok_or_else(|| error!(ZamaHostError::InvalidFheEvalAccount))
}
