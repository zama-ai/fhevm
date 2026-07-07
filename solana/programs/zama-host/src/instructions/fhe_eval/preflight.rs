use super::event_budget::assert_eval_log_budget;
use super::*;

pub(super) fn preflight_eval_frame(
    remaining_accounts: &[AccountInfo],
    args: &FheEvalArgs,
) -> Result<()> {
    assert_unique_remaining_accounts(remaining_accounts)?;
    let mut preflight = EvalPreflight::new(remaining_accounts.len());

    for (index, step) in args.steps.iter().enumerate() {
        preflight_eval_step(step, index, &mut preflight)?;
    }
    assert_eval_log_budget(args)?;
    preflight.finish()
}

struct EvalPreflight {
    remaining_accounts_used: Vec<bool>,
}

impl EvalPreflight {
    /// Tracks static frame requirements before any account mutation happens.
    fn new(remaining_account_count: usize) -> Self {
        Self {
            remaining_accounts_used: vec![false; remaining_account_count],
        }
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
    preflight: &mut EvalPreflight,
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
        FheEvalStep::TrivialEncrypt { output, .. }
        | FheEvalStep::Rand { output, .. }
        | FheEvalStep::RandBounded { output, .. } => preflight_output(output, preflight)?,
    }
    Ok(())
}

fn preflight_rhs_operand(
    operand: &FheEvalOperand,
    step_index: usize,
    preflight: &mut EvalPreflight,
) -> Result<()> {
    match operand {
        FheEvalOperand::Scalar(_) => Ok(()),
        _ => preflight_encrypted_operand(operand, step_index, preflight),
    }
}

fn preflight_encrypted_operand(
    operand: &FheEvalOperand,
    step_index: usize,
    preflight: &mut EvalPreflight,
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

fn preflight_output(output: &FheEvalOutput, preflight: &mut EvalPreflight) -> Result<()> {
    match output {
        FheEvalOutput::AllowedLocal => {}
        FheEvalOutput::AllowedDurable {
            output_encrypted_value_index,
            output_app_account_authority_index,
            ..
        } => {
            preflight.mark_account(*output_encrypted_value_index)?;
            if let Some(index) = output_app_account_authority_index {
                preflight.mark_account(*index)?;
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

        assert!(preflight_eval_frame(&accounts, &args).is_err());
    }

    fn test_account(key: Pubkey) -> AccountInfo<'static> {
        let key = Box::leak(Box::new(key));
        let lamports = Box::leak(Box::new(0));
        let data = Box::leak(Vec::new().into_boxed_slice());
        let owner = Box::leak(Box::new(System::id()));
        AccountInfo::new(key, false, false, lamports, data, owner, false)
    }
}
