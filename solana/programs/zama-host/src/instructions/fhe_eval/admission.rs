use super::super::common::{
    assert_encrypted_value_subject_allowed, assert_output_acl_metadata,
    check_grant_not_denied_info, read_canonical_encrypted_value,
};
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
        encrypted_value_index: u16,
    ) -> Result<ResolvedOperand> {
        let value_info = account_at(self.remaining_accounts, encrypted_value_index)?;
        assert_encrypted_value_subject_allowed(value_info, handle, self.chain_id, self.subject)?;
        Ok(ResolvedOperand::encrypted(handle, false))
    }

    /// Admits one durable output lineage account: canonical PDA, writable,
    /// used at most once per frame, and either fresh (create) or an existing
    /// canonical lineage whose stored state matches the plan's previous_*.
    #[allow(clippy::too_many_arguments)]
    fn admit_durable_output_account(
        &mut self,
        remaining_accounts: &[AccountInfo],
        output_encrypted_value_index: u16,
        output_acl_domain_key: Pubkey,
        output_app_account: Pubkey,
        output_encrypted_value_label: [u8; 32],
        output_subjects: &[AclSubjectEntry],
        previous_handle: &Option<[u8; 32]>,
        previous_subjects: &Option<Vec<Pubkey>>,
    ) -> Result<()> {
        let output_info = account_at(remaining_accounts, output_encrypted_value_index)?;
        let value_key = zama_solana_acl::derive_value_key(
            output_acl_domain_key.to_bytes(),
            output_app_account.to_bytes(),
            output_encrypted_value_label,
        );
        let (expected, _) = encrypted_value_address(value_key);
        require_keys_eq!(
            output_info.key(),
            expected,
            ZamaHostError::EncryptedValuePdaMismatch
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
        if output_info.owner == &crate::ID {
            let value = read_canonical_encrypted_value(output_info)?;
            super::validate_durable_output_previous_state(
                &value,
                output_subjects,
                previous_handle,
                previous_subjects,
            )?;
        } else {
            require!(
                previous_handle.is_none() && previous_subjects.is_none(),
                ZamaHostError::PreviousStateMismatch
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
        }
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
        encrypted_value_index: u16,
    ) -> Result<ResolvedOperand> {
        self.resolve_durable(handle, encrypted_value_index)
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

    fn accept_output<'info>(
        &mut self,
        ctx: &Context<'info, FheEval<'info>>,
        _op_index: u16,
        result: [u8; 32],
        output: &FheEvalOutput,
        output_public_decrypt_allowed: bool,
    ) -> Result<()> {
        require!(
            !self.produced.iter().any(|value| value.handle == result),
            ZamaHostError::FheEvalDuplicateHandle
        );

        match output {
            FheEvalOutput::AllowedLocal => {}
            FheEvalOutput::AllowedDurable {
                output_encrypted_value_index,
                output_app_account_authority_index,
                output_acl_domain_key,
                output_app_account,
                output_encrypted_value_label,
                output_subjects,
                previous_handle,
                previous_subjects,
                make_public: _,
            } => {
                let app_account_authority = admit_durable_output_authority(
                    ctx,
                    *output_app_account_authority_index,
                    *output_app_account,
                )?;
                assert_output_acl_metadata(
                    app_account_authority.key(),
                    *output_app_account,
                    output_subjects,
                )?;
                self.admit_durable_output_account(
                    ctx.remaining_accounts,
                    *output_encrypted_value_index,
                    *output_acl_domain_key,
                    *output_app_account,
                    *output_encrypted_value_label,
                    output_subjects,
                    previous_handle,
                    previous_subjects,
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
    let authority = match authority_index {
        Some(index) => {
            let authority = account_at(ctx.remaining_accounts, index)?;
            require!(authority.is_signer, ZamaHostError::InvalidFheEvalAccount);
            require_keys_eq!(
                authority.key(),
                output_app_account,
                ZamaHostError::AppAccountAuthorityMismatch
            );
            authority.clone()
        }
        None => ctx.accounts.app_account_authority.to_account_info(),
    };
    let deny_record = super::deny_subject_record_for(
        &ctx.accounts.host_config,
        ctx.remaining_accounts,
        None,
        authority.key(),
    )?;
    check_grant_not_denied_info(&ctx.accounts.host_config, authority.key(), deny_record)?;
    Ok(authority)
}

fn account_at<'a, 'info>(
    remaining_accounts: &'a [AccountInfo<'info>],
    index: u16,
) -> Result<&'a AccountInfo<'info>> {
    remaining_accounts
        .get(index as usize)
        .ok_or_else(|| error!(ZamaHostError::InvalidFheEvalAccount))
}
