use super::super::common::{
    assert_output_acl_metadata, assert_transient_grant, read_transient_capability_for_eval,
    read_transient_session,
};
use super::handles::EvalHandleContext;
use super::walk::{walk_eval_frame, EvalStepVisitor};
use super::*;

/// Validate-only first pass over the plan: re-uses the shared step walk and
/// operand resolvers (see [`walk`]) but performs no mutation. It tracks the
/// outputs and the planned transient consumes/appends in memory so the whole
/// frame is checked before execution touches any account.
pub(super) fn admit_eval_frame<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    args: &FheEvalArgs,
    subject: Pubkey,
    session_authority: Pubkey,
    current_slot: u64,
    handle_context: &EvalHandleContext<'_>,
    instructions_sysvar: Option<&AccountInfo<'info>>,
) -> Result<()> {
    let mut admission = AdmissionState::new(
        ctx.remaining_accounts,
        args.steps.len(),
        subject,
        session_authority,
        handle_context.chain_id,
        current_slot,
        instructions_sysvar,
    );
    walk_eval_frame(&mut admission, ctx, args, handle_context)
}

struct AdmissionState<'a, 'info> {
    remaining_accounts: &'a [AccountInfo<'info>],
    produced: Vec<ProducedValue>,
    durable_output_accounts: Vec<Pubkey>,
    session_appends: Vec<Pubkey>,
    session_consumes: Vec<(Pubkey, u16)>,
    subject: Pubkey,
    session_authority: Pubkey,
    chain_id: u64,
    current_slot: u64,
    instructions_sysvar: Option<&'a AccountInfo<'info>>,
}

impl<'a, 'info> AdmissionState<'a, 'info> {
    fn new(
        remaining_accounts: &'a [AccountInfo<'info>],
        step_count: usize,
        subject: Pubkey,
        session_authority: Pubkey,
        chain_id: u64,
        current_slot: u64,
        instructions_sysvar: Option<&'a AccountInfo<'info>>,
    ) -> Self {
        Self {
            remaining_accounts,
            produced: Vec::with_capacity(step_count),
            durable_output_accounts: Vec::new(),
            session_appends: Vec::new(),
            session_consumes: Vec::new(),
            subject,
            session_authority,
            chain_id,
            current_slot,
            instructions_sysvar,
        }
    }

    fn resolve_durable(
        &mut self,
        handle: [u8; 32],
        acl_record_index: u16,
        permission_index: Option<u16>,
    ) -> Result<ResolvedOperand> {
        let record_info = account_at(self.remaining_accounts, acl_record_index)?;
        let permission_info = permission_index
            .map(|index| account_at(self.remaining_accounts, index))
            .transpose()?;
        assert_unchecked_acl_record_subject_role(
            record_info,
            handle,
            self.chain_id,
            self.subject,
            ACL_ROLE_USE,
            permission_info,
        )?;
        let public_decrypt_allowed = unchecked_acl_record_subject_has_role(
            record_info,
            handle,
            self.subject,
            ACL_ROLE_PUBLIC_DECRYPT,
            permission_info,
        )?;
        Ok(ResolvedOperand::encrypted(handle, public_decrypt_allowed))
    }

    fn resolve_transient_session(
        &mut self,
        handle: [u8; 32],
        session_index: u16,
        capability_index: u16,
    ) -> Result<ResolvedOperand> {
        let session_info = account_at(self.remaining_accounts, session_index)?;
        let capability = read_transient_capability_for_eval(
            session_info,
            self.session_authority,
            self.current_slot,
            handle,
            self.subject,
            ACL_ROLE_USE,
            capability_index,
            self.instructions_sysvar,
        )?;
        self.admit_transient_capability_consume(session_info.key(), capability_index, capability)?;
        Ok(ResolvedOperand::transient_session(handle, capability.grant))
    }

    fn admit_durable_output_account(
        &mut self,
        remaining_accounts: &[AccountInfo],
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

    fn admit_transient_session_append(
        &mut self,
        remaining_accounts: &[AccountInfo],
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

impl EvalStepVisitor for AdmissionState<'_, '_> {
    fn subject(&self) -> Pubkey {
        self.subject
    }

    fn produced(&self) -> &[ProducedValue] {
        &self.produced
    }

    fn resolve_durable_operand(
        &mut self,
        handle: [u8; 32],
        acl_record_index: u16,
        permission_index: Option<u16>,
    ) -> Result<ResolvedOperand> {
        self.resolve_durable(handle, acl_record_index, permission_index)
    }

    fn resolve_transient_session_operand(
        &mut self,
        handle: [u8; 32],
        session_index: u16,
        capability_index: u16,
    ) -> Result<ResolvedOperand> {
        self.resolve_transient_session(handle, session_index, capability_index)
    }

    fn resolve_verified_input_operand(
        &mut self,
        attestation: &CoprocessorInputAttestation,
    ) -> Result<ResolvedOperand> {
        // Structural only — the handle is known from the operand; execution re-verifies the
        // attestation authoritatively (matches how transient-session consume is execution-gated).
        // The attested identities are still tracked so admission rejects an output that does not
        // bind them.
        Ok(ResolvedOperand::verified_input(
            attestation.input_handle,
            VerifiedInputBinding {
                user_address: Pubkey::new_from_array(attestation.user_address),
                contract_address: Pubkey::new_from_array(attestation.contract_address),
            },
        ))
    }

    fn record_op_event(&mut self, _event: EvalEvent) {}

    fn accept_output<'info>(
        &mut self,
        ctx: &Context<'info, FheEval<'info>>,
        result: [u8; 32],
        output: &FheEvalOutput,
        output_policies: Vec<SessionPolicy>,
        output_public_decrypt_allowed: bool,
        enforce_public_decrypt_role_propagation: bool,
        verified_input: Option<VerifiedInputBinding>,
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
                require!(
                    verified_input.is_none(),
                    ZamaHostError::InputBindTransientSessionUnsupported
                );
                assert_session_policies_allow_transient_grant(&output_policies, *capability)?;
                self.admit_transient_session_append(
                    ctx.remaining_accounts,
                    *session_index,
                    ctx.accounts.app_account_authority.key(),
                    self.current_slot,
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
                if let Some(binding) = verified_input {
                    assert_verified_input_output_binding(
                        &binding,
                        *output_acl_domain_key,
                        *output_app_account,
                        output_subjects,
                    )?;
                }
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
            verified_input,
        });
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

fn account_at<'a, 'info>(
    remaining_accounts: &'a [AccountInfo<'info>],
    index: u16,
) -> Result<&'a AccountInfo<'info>> {
    remaining_accounts
        .get(index as usize)
        .ok_or_else(|| error!(ZamaHostError::InvalidFheEvalAccount))
}
