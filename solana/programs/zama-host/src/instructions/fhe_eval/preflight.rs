use super::*;

pub(super) fn preflight_eval_frame<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    args: &FheEvalArgs,
) -> Result<()> {
    assert_frame_pins_or_persists_under_cap(args, ctx.accounts.host_config.hcu_block_cap_per_app)?;
    preflight_eval_frame_accounts(
        ctx.remaining_accounts,
        args,
        ctx.accounts.app_account_authority.key(),
        ctx.accounts.host_config.grant_deny_list_enabled,
    )
}

/// Rejects the value-less persist-nothing frame class under a finite block cap (fhevm-internal#1744).
///
/// The per-slot HCU meter keys on the signed `compute_subject`. A durable input requires the
/// subject to be an allowed ACL subject, and a verified input requires it to equal the attested
/// contract, so either operand pins the subject — the caller cannot swap in a fresh key without
/// losing input access. A frame with neither pinning operand and no durable output persists nothing
/// and verifies nothing: `compute_subject` is a free variable AND the frame produces nothing of
/// value (its transient outputs create no ACL leaf and are undecryptable). That class is what the
/// keypair-rotation bypass rode, so it is rejected before compute.
///
/// A durable OUTPUT is allowed through here, but note it does NOT pin the subject: output binding
/// authorizes against `app_account_authority`, never `compute_subject`. So a throwaway-lineage
/// create/supersede still lets a caller rotate the subject for a fresh per-slot meter — that vector
/// remains open, but is rent-bounded (~one `HcuBlockMeter` PDA rent per rotation) rather than free,
/// and closing it fully needs a registered app identity (the issue's Option 2, deferred). The
/// allowance is kept because it is also the legitimate trivial-encrypt/`Rand` -> durable-output
/// bootstrap/mint path. The deactivated cap (`u64::MAX`, the ship default) short-circuits, so
/// behavior is unchanged wherever a finite cap is not deployed.
fn assert_frame_pins_or_persists_under_cap(
    args: &FheEvalArgs,
    hcu_block_cap_per_app: u64,
) -> Result<()> {
    if hcu_block_cap_per_app == u64::MAX {
        return Ok(());
    }
    require!(
        args.steps.iter().any(step_pins_or_persists),
        ZamaHostError::FheEvalUnanchoredUnderBlockCap
    );
    Ok(())
}

/// True for an operand that pins `compute_subject`: a durable ACL input (the subject must be an
/// allowed subject) or a verified input (the subject must equal the attested contract). Exhaustive
/// so a future operand variant must classify itself rather than default to "does not pin".
fn operand_pins_subject(operand: &FheEvalOperand) -> bool {
    match operand {
        FheEvalOperand::AllowedDurable { .. } | FheEvalOperand::VerifiedInput { .. } => true,
        FheEvalOperand::AllowedLocal { .. } | FheEvalOperand::Scalar(_) => false,
    }
}

/// True for an output that persists durable state. Exhaustive so a future output variant must
/// classify itself rather than default to "does not persist".
fn output_persists(output: &FheEvalOutput) -> bool {
    match output {
        FheEvalOutput::AllowedDurable { .. } => true,
        FheEvalOutput::AllowedLocal => false,
    }
}

/// True when a step carries a subject-pinning operand or a durable output.
fn step_pins_or_persists(step: &FheEvalStep) -> bool {
    let (output, operand_pins) = match step {
        FheEvalStep::Binary {
            lhs, rhs, output, ..
        } => (
            output,
            operand_pins_subject(lhs) || operand_pins_subject(rhs),
        ),
        FheEvalStep::Ternary {
            control,
            if_true,
            if_false,
            output,
            ..
        } => (
            output,
            operand_pins_subject(control)
                || operand_pins_subject(if_true)
                || operand_pins_subject(if_false),
        ),
        FheEvalStep::TrivialEncrypt { output, .. }
        | FheEvalStep::Rand { output, .. }
        | FheEvalStep::RandBounded { output, .. } => (output, false),
        FheEvalStep::Unary {
            operand, output, ..
        } => (output, operand_pins_subject(operand)),
        FheEvalStep::Sum {
            operands, output, ..
        } => (output, operands.iter().any(operand_pins_subject)),
        FheEvalStep::IsIn {
            value, set, output, ..
        } => (
            output,
            operand_pins_subject(value) || set.iter().any(operand_pins_subject),
        ),
        FheEvalStep::MulDiv {
            factor1,
            factor2,
            output,
            ..
        } => (
            output,
            operand_pins_subject(factor1) || operand_pins_subject(factor2),
        ),
    };
    operand_pins || output_persists(output)
}

fn preflight_eval_frame_accounts(
    remaining_accounts: &[AccountInfo],
    args: &FheEvalArgs,
    app_account_authority: Pubkey,
    deny_list_enabled: bool,
) -> Result<()> {
    assert_unique_remaining_accounts(remaining_accounts)?;
    let mut preflight =
        EvalPreflight::new(remaining_accounts, app_account_authority, deny_list_enabled);

    for (index, step) in args.steps.iter().enumerate() {
        preflight_eval_step(step, index, &mut preflight)?;
    }
    preflight.finish()
}

struct EvalPreflight<'a, 'info> {
    remaining_accounts: &'a [AccountInfo<'info>],
    remaining_accounts_used: Vec<bool>,
    app_account_authority: Pubkey,
    deny_list_enabled: bool,
}

impl<'a, 'info> EvalPreflight<'a, 'info> {
    /// Tracks static frame requirements before any account mutation happens.
    fn new(
        remaining_accounts: &'a [AccountInfo<'info>],
        app_account_authority: Pubkey,
        deny_list_enabled: bool,
    ) -> Self {
        Self {
            remaining_accounts,
            remaining_accounts_used: vec![false; remaining_accounts.len()],
            app_account_authority,
            deny_list_enabled,
        }
    }

    fn account(&self, index: u16) -> Result<&AccountInfo<'info>> {
        self.remaining_accounts
            .get(index as usize)
            .ok_or_else(|| error!(ZamaHostError::InvalidFheEvalAccount))
    }

    fn mark_account(&mut self, index: u16) -> Result<()> {
        let account_index = index as usize;
        let used = self
            .remaining_accounts_used
            .get_mut(account_index)
            .ok_or_else(|| error!(ZamaHostError::InvalidFheEvalAccount))?;
        *used = true;
        Ok(())
    }

    fn mark_output_authority(&mut self, authority_index: Option<u16>) -> Result<Pubkey> {
        match authority_index {
            Some(index) => {
                let authority = self.account(index)?.key();
                self.mark_account(index)?;
                Ok(authority)
            }
            None => Ok(self.app_account_authority),
        }
    }

    fn mark_deny_record(&mut self, authority: Pubkey) -> Result<()> {
        if !self.deny_list_enabled {
            return Ok(());
        }
        let (expected, _) = deny_subject_address(authority);
        let Some(index) = self
            .remaining_accounts
            .iter()
            .position(|account| account.key() == expected)
        else {
            return Err(error!(ZamaHostError::AclDenyRecordMissing));
        };
        self.remaining_accounts_used[index] = true;
        Ok(())
    }

    fn finish(self) -> Result<()> {
        require!(
            self.remaining_accounts_used.iter().all(|used| *used),
            ZamaHostError::InvalidFheEvalAccount
        );
        Ok(())
    }
}

fn assert_unique_remaining_accounts(remaining_accounts: &[AccountInfo]) -> Result<()> {
    for (index, account) in remaining_accounts.iter().enumerate() {
        require!(
            !remaining_accounts[index + 1..]
                .iter()
                .any(|later| later.key() == account.key()),
            ZamaHostError::InvalidFheEvalAccount
        );
    }
    Ok(())
}

fn preflight_eval_step(
    step: &FheEvalStep,
    step_index: usize,
    preflight: &mut EvalPreflight<'_, '_>,
) -> Result<()> {
    match step {
        FheEvalStep::Binary {
            lhs, rhs, output, ..
        } => {
            preflight_encrypted_operand(lhs, step_index, preflight)?;
            preflight_rhs_operand(rhs, step_index, preflight)?;
            preflight_output(output, preflight)?;
        }
        FheEvalStep::Ternary {
            control,
            if_true,
            if_false,
            output,
            ..
        } => {
            preflight_encrypted_operand(control, step_index, preflight)?;
            preflight_encrypted_operand(if_true, step_index, preflight)?;
            preflight_encrypted_operand(if_false, step_index, preflight)?;
            preflight_output(output, preflight)?;
        }
        FheEvalStep::TrivialEncrypt { output, .. } | FheEvalStep::Rand { output, .. } => {
            preflight_output(output, preflight)?;
        }
        FheEvalStep::Unary {
            operand, output, ..
        } => {
            preflight_encrypted_operand(operand, step_index, preflight)?;
            preflight_output(output, preflight)?;
        }
        FheEvalStep::RandBounded { output, .. } => {
            preflight_output(output, preflight)?;
        }
        FheEvalStep::Sum {
            operands, output, ..
        } => {
            for operand in operands {
                preflight_encrypted_operand(operand, step_index, preflight)?;
            }
            preflight_output(output, preflight)?;
        }
        FheEvalStep::IsIn {
            value, set, output, ..
        } => {
            preflight_encrypted_operand(value, step_index, preflight)?;
            for operand in set {
                preflight_encrypted_operand(operand, step_index, preflight)?;
            }
            preflight_output(output, preflight)?;
        }
        FheEvalStep::MulDiv {
            factor1,
            factor2,
            output,
            ..
        } => {
            preflight_encrypted_operand(factor1, step_index, preflight)?;
            preflight_rhs_operand(factor2, step_index, preflight)?;
            preflight_output(output, preflight)?;
        }
    }
    Ok(())
}

fn preflight_rhs_operand(
    operand: &FheEvalOperand,
    step_index: usize,
    preflight: &mut EvalPreflight<'_, '_>,
) -> Result<()> {
    match operand {
        FheEvalOperand::Scalar(_) => Ok(()),
        _ => preflight_encrypted_operand(operand, step_index, preflight),
    }
}

fn preflight_encrypted_operand(
    operand: &FheEvalOperand,
    step_index: usize,
    preflight: &mut EvalPreflight<'_, '_>,
) -> Result<()> {
    match operand {
        FheEvalOperand::AllowedDurable {
            encrypted_value_index,
            ..
        } => {
            preflight.mark_account(*encrypted_value_index)?;
        }
        FheEvalOperand::AllowedLocal { producer_index } => {
            require!(
                (*producer_index as usize) < step_index,
                ZamaHostError::FheEvalAllowedLocalMissing
            );
        }
        FheEvalOperand::VerifiedInput { .. } => {
            // No remaining account: the attestation is carried inline and verified in-frame.
        }
        FheEvalOperand::Scalar(_) => return Err(error!(ZamaHostError::InvalidFheEvalAccount)),
    }
    Ok(())
}

fn preflight_output(output: &FheEvalOutput, preflight: &mut EvalPreflight<'_, '_>) -> Result<()> {
    match output {
        FheEvalOutput::AllowedLocal => {}
        FheEvalOutput::AllowedDurable {
            output_encrypted_value_index,
            output_app_account_authority_index,
            output_subjects,
            previous_subjects,
            ..
        } => {
            preflight.mark_account(*output_encrypted_value_index)?;
            let authority = preflight.mark_output_authority(*output_app_account_authority_index)?;
            preflight.mark_deny_record(authority)?;
            // A supersede that rotates the audience deny-checks each added subject in the bind
            // pass; mark their deny records here so finish() accounts for them. The added set is
            // `output_subjects \ previous_subjects` from instruction data alone — a lying
            // previous_subjects is rejected later with PreviousStateMismatch, so trusting it for
            // account-marking is safe. `None` previous is a create (no rotation, nothing added).
            if let Some(previous_subjects) = previous_subjects {
                for subject in output_subjects {
                    if !previous_subjects.contains(&subject.pubkey) {
                        preflight.mark_deny_record(subject.pubkey)?;
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preflight_rejects_duplicate_remaining_account_keys() {
        let duplicate = Pubkey::new_unique();
        let args = FheEvalArgs {
            context_id: [1; 32],
            steps: vec![FheEvalStep::Binary {
                op: FheBinaryOpCode::Add,
                lhs: FheEvalOperand::AllowedDurable {
                    handle: [1; 32],
                    encrypted_value_index: 0,
                },
                rhs: FheEvalOperand::AllowedDurable {
                    handle: [2; 32],
                    encrypted_value_index: 1,
                },
                output_fhe_type: 5,
                output: FheEvalOutput::AllowedLocal,
            }],
        };
        let accounts = vec![test_account(duplicate), test_account(duplicate)];

        assert!(
            preflight_eval_frame_accounts(&accounts, &args, Pubkey::new_unique(), false).is_err()
        );
    }

    fn test_account(key: Pubkey) -> AccountInfo<'static> {
        let key = Box::leak(Box::new(key));
        let lamports = Box::leak(Box::new(0));
        let data = Box::leak(Vec::new().into_boxed_slice());
        let owner = Box::leak(Box::new(System::id()));
        AccountInfo::new(key, false, false, lamports, data, owner, false)
    }

    fn frame(steps: Vec<FheEvalStep>) -> FheEvalArgs {
        FheEvalArgs {
            context_id: [1; 32],
            steps,
        }
    }

    fn trivial_local() -> FheEvalStep {
        FheEvalStep::TrivialEncrypt {
            plaintext: [0; 32],
            fhe_type: 5,
            output: FheEvalOutput::AllowedLocal,
        }
    }

    fn durable_output() -> FheEvalOutput {
        FheEvalOutput::AllowedDurable {
            output_encrypted_value_index: 0,
            output_app_account_authority_index: None,
            output_acl_domain_key: Pubkey::new_unique(),
            output_app_account: Pubkey::new_unique(),
            output_encrypted_value_label: [0; 32],
            output_subjects: Vec::new(),
            previous_handle: None,
            previous_subjects: None,
            make_public: false,
        }
    }

    fn verified_input() -> FheEvalOperand {
        FheEvalOperand::VerifiedInput {
            attestation: Box::new(CoprocessorInputAttestation {
                input_handle: [0; 32],
                ct_handles: Vec::new(),
                handle_index: 0,
                user_address: [0; 32],
                contract_address: [0; 32],
                contract_chain_id: 0,
                extra_data: Vec::new(),
                signatures: Vec::new(),
            }),
        }
    }

    const FINITE_CAP: u64 = 500_000;

    #[test]
    fn deactivated_cap_never_rejects_persist_nothing_frame() {
        // u64::MAX (ship default) short-circuits: behavior is unchanged where the cap is not deployed.
        assert!(
            assert_frame_pins_or_persists_under_cap(&frame(vec![trivial_local()]), u64::MAX)
                .is_ok()
        );
    }

    #[test]
    fn finite_cap_rejects_persist_nothing_frame() {
        assert!(
            assert_frame_pins_or_persists_under_cap(&frame(vec![trivial_local()]), FINITE_CAP)
                .is_err()
        );
    }

    #[test]
    fn finite_cap_accepts_durable_output_frame() {
        // The trivial-encrypt -> durable-output bootstrap/mint path stays legal.
        let step = FheEvalStep::TrivialEncrypt {
            plaintext: [0; 32],
            fhe_type: 5,
            output: durable_output(),
        };
        assert!(assert_frame_pins_or_persists_under_cap(&frame(vec![step]), FINITE_CAP).is_ok());
    }

    #[test]
    fn finite_cap_accepts_durable_input_frame() {
        let step = FheEvalStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheEvalOperand::AllowedDurable {
                handle: [1; 32],
                encrypted_value_index: 0,
            },
            rhs: FheEvalOperand::Scalar([2; 32]),
            output_fhe_type: 5,
            output: FheEvalOutput::AllowedLocal,
        };
        assert!(assert_frame_pins_or_persists_under_cap(&frame(vec![step]), FINITE_CAP).is_ok());
    }

    #[test]
    fn finite_cap_accepts_verified_input_transient_frame() {
        // A verified input pins the subject (attested contract must equal it), so a transient-output
        // frame that carries one is anchored and not persist-nothing.
        let step = FheEvalStep::Unary {
            op: FheUnaryOpCode::Not,
            operand: verified_input(),
            output_fhe_type: 5,
            output: FheEvalOutput::AllowedLocal,
        };
        assert!(assert_frame_pins_or_persists_under_cap(&frame(vec![step]), FINITE_CAP).is_ok());
    }
}
