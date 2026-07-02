use super::super::common::assert_output_acl_metadata;
use super::handles::EvalHandleContext;
use super::walk::{walk_eval_frame, EvalStepVisitor};
use super::*;

/// Validate-only first pass over the plan: re-uses the shared step walk and
/// operand resolvers (see [`walk`]) but performs no mutation. It tracks the
/// planned durable outputs in memory so the whole frame is checked before
/// execution touches any account.
///
/// Returns the per-frame HCU total computed by the walk, so the caller can run the read-only
/// block-cap `check` before execution.
pub(super) fn admit_eval_frame<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    args: &FheEvalArgs,
    subject: Pubkey,
    handle_context: &EvalHandleContext<'_>,
) -> Result<u64> {
    let mut admission = AdmissionState::new(
        ctx.remaining_accounts,
        args.steps.len(),
        subject,
        handle_context.chain_id,
    );
    walk_eval_frame(&mut admission, ctx, args, handle_context)
}

struct AdmissionState<'a, 'info> {
    remaining_accounts: &'a [AccountInfo<'info>],
    produced: Vec<ProducedValue>,
    durable_output_accounts: Vec<Pubkey>,
    subject: Pubkey,
    chain_id: u64,
}

impl<'a, 'info> AdmissionState<'a, 'info> {
    fn new(
        remaining_accounts: &'a [AccountInfo<'info>],
        step_count: usize,
        subject: Pubkey,
        chain_id: u64,
    ) -> Self {
        Self {
            remaining_accounts,
            produced: Vec::with_capacity(step_count),
            durable_output_accounts: Vec::new(),
            subject,
            chain_id,
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

    fn resolve_verified_input_operand(
        &mut self,
        attestation: &CoprocessorInputAttestation,
    ) -> Result<ResolvedOperand> {
        // Structural only — the handle is known from the operand; execution re-verifies the
        // attestation authoritatively (matches how transient-session consume is execution-gated).
        // The caller-is-contract gate runs in the shared `resolve_encrypted_operand`; derived
        // outputs are unconstrained (EVM `fromExternal` parity).
        Ok(ResolvedOperand::encrypted(attestation.input_handle, true))
    }

    fn record_op_event(&mut self, _event: EvalEvent) {}

    fn accept_output<'info>(
        &mut self,
        ctx: &Context<'info, FheEval<'info>>,
        result: [u8; 32],
        output: &FheEvalOutput,
        output_public_decrypt_allowed: bool,
        enforce_public_decrypt_role_propagation: bool,
    ) -> Result<()> {
        require!(
            !self.produced.iter().any(|value| value.handle == result),
            ZamaHostError::FheEvalDuplicateHandle
        );

        match output {
            FheEvalOutput::AllowedLocal => {}
            FheEvalOutput::AllowedDurable {
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
