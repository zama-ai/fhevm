use super::account_table::EvalAccountTable;
use super::*;

pub(super) fn preflight_eval_frame<'info>(
    table: &mut EvalAccountTable<'_, 'info>,
    ctx: &Context<'info, FheExecute<'info>>,
    args: &FheExecuteArgs,
) -> Result<()> {
    assert_frame_pins_or_persists_under_cap(args, ctx.accounts.host_config.hcu_block_cap_per_app)?;
    assert_rand_steps_anchor_persistent_output(args)?;
    preflight_eval_frame_accounts(
        table,
        args,
        ctx.accounts.account_authority.key(),
        ctx.accounts.host_config.grant_deny_list_enabled,
    )
}

/// Rejects the value-less persist-nothing batch class under a finite block cap (fhevm-internal#1744).
///
/// The per-slot HCU meter keys on the signed `compute_subject`. A persistent input requires the
/// subject to be an allowed ACL subject, and a verified input requires it to equal the attested
/// contract, so either operand pins the subject — the caller cannot swap in a fresh key without
/// losing input access. A batch with neither pinning operand and no persistent output persists nothing
/// and verifies nothing: `compute_subject` is a free variable AND the batch produces nothing of
/// value (its transient outputs create no ACL leaf and are undecryptable). That class is what the
/// keypair-churn bypass rode, so it is rejected before compute.
///
/// A persistent OUTPUT is allowed through here, but note it does NOT pin the subject: output binding
/// authorizes against `account_authority`, never `compute_subject`. So a throwaway-encrypted value account
/// create/update still lets a caller swap the subject for a fresh per-slot meter — that vector
/// remains open, but is rent-bounded (~one `HcuBlockMeter` PDA rent per swap) rather than free,
/// and closing it fully needs a registered app identity (the issue's Option 2, deferred). The
/// allowance is kept because it is also the legitimate trivial-encrypt/`Rand` -> persistent-output
/// bootstrap/mint path. The deactivated cap (`u64::MAX`, the ship default) short-circuits, so
/// behavior is unchanged wherever a finite cap is not deployed.
fn assert_frame_pins_or_persists_under_cap(
    args: &FheExecuteArgs,
    hcu_block_cap_per_app: u64,
) -> Result<()> {
    if hcu_block_cap_per_app == u64::MAX {
        return Ok(());
    }
    require!(
        args.steps.iter().any(step_pins_or_persists),
        ZamaHostError::FheExecuteUnanchoredUnderBlockCap
    );
    Ok(())
}

/// Rejects a batch containing a rand step but no persistent output (fhevm-internal#1853 W4).
///
/// Rand seeds are anchored to the batch's persistent-write anchor — the live account identity
/// and version of every persistent output — so a rand step needs at least one
/// persistent output for the seed to be compulsorily fresh. The excluded class is provably
/// useless: an all-transient batch has no ACL records, no decrypt path, and no `make_public`,
/// so its randomness is unobservable by everyone including the author.
fn assert_rand_steps_anchor_persistent_output(args: &FheExecuteArgs) -> Result<()> {
    let has_rand = args.steps.iter().any(|step| {
        matches!(
            step,
            FheExecuteStep::Rand { .. } | FheExecuteStep::RandBounded { .. }
        )
    });
    if !has_rand {
        return Ok(());
    }
    require!(
        args.steps
            .iter()
            .any(|step| output_persists(super::step_output(step))),
        ZamaHostError::FheExecuteRandRequiresPersistentOutput
    );
    Ok(())
}

/// True for an operand that pins `compute_subject`: a persistent ACL input (the subject must be an
/// allowed subject) or a verified input (the subject must equal the attested contract). Exhaustive
/// so a future operand variant must classify itself rather than default to "does not pin".
fn operand_pins_subject(operand: &FheExecuteOperand) -> bool {
    match operand {
        FheExecuteOperand::AllowedPersistent { .. } | FheExecuteOperand::VerifiedInput { .. } => {
            true
        }
        FheExecuteOperand::AllowedLocal { .. } | FheExecuteOperand::Scalar { .. } => false,
    }
}

/// True for an output that persists persistent state. Exhaustive so a future output variant must
/// classify itself rather than default to "does not persist".
fn output_persists(output: &FheExecuteOutput) -> bool {
    match output {
        FheExecuteOutput::AllowedPersistent { .. } => true,
        FheExecuteOutput::AllowedLocal => false,
    }
}

/// True when a step carries a subject-pinning operand or a persistent output.
fn step_pins_or_persists(step: &FheExecuteStep) -> bool {
    let (output, operand_pins) = match step {
        FheExecuteStep::Binary {
            lhs, rhs, output, ..
        } => (
            output,
            operand_pins_subject(lhs) || operand_pins_subject(rhs),
        ),
        FheExecuteStep::Ternary {
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
        FheExecuteStep::TrivialEncrypt { output, .. }
        | FheExecuteStep::Rand { output, .. }
        | FheExecuteStep::RandBounded { output, .. } => (output, false),
        FheExecuteStep::Unary {
            operand, output, ..
        } => (output, operand_pins_subject(operand)),
        FheExecuteStep::Sum {
            operands, output, ..
        } => (output, operands.iter().any(operand_pins_subject)),
        FheExecuteStep::IsIn {
            value, set, output, ..
        } => (
            output,
            operand_pins_subject(value) || set.iter().any(operand_pins_subject),
        ),
        FheExecuteStep::MulDiv {
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
    table: &mut EvalAccountTable<'_, '_>,
    args: &FheExecuteArgs,
    account_authority: Pubkey,
    deny_list_enabled: bool,
) -> Result<()> {
    let mut preflight = EvalPreflight {
        table,
        dictionary: &args.dictionary,
        dictionary_used: vec![false; args.dictionary.len()],
        account_authority,
        deny_list_enabled,
        persistent_outputs_written: Vec::with_capacity(MAX_FHE_BATCH_OPS),
    };
    for (index, step) in args.steps.iter().enumerate() {
        preflight_eval_step(step, index, &mut preflight)?;
    }
    // Whole-batch hygiene, mirroring the account table: every interned dictionary
    // entry must be referenced by some step, so a batch cannot carry dead bytes.
    require!(
        preflight.dictionary_used.iter().all(|used| *used),
        ZamaHostError::FheExecuteDictionaryEntryUnreferenced
    );
    preflight.table.assert_all_used()
}

/// Marks every account the batch references into the shared table so
/// [`EvalAccountTable::assert_all_used`] can reject dangling accounts before
/// any pass mutates state.
struct EvalPreflight<'t, 'a, 'info> {
    table: &'t mut EvalAccountTable<'a, 'info>,
    dictionary: &'t [[u8; 32]],
    dictionary_used: Vec<bool>,
    account_authority: Pubkey,
    deny_list_enabled: bool,
    /// Persistent accounts written by completed earlier steps. Operands are checked
    /// before the current step's output is recorded, so read-then-update in one
    /// step remains valid.
    persistent_outputs_written: Vec<Pubkey>,
}

impl EvalPreflight<'_, '_, '_> {
    /// Marks a dictionary reference used and returns its bytes; out-of-range fails the batch here,
    /// before execution resolves anything.
    fn mark_dictionary(&mut self, index: u8) -> Result<[u8; 32]> {
        let entry = self
            .dictionary
            .get(index as usize)
            .copied()
            .ok_or_else(|| error!(ZamaHostError::FheExecuteDictionaryIndexOutOfBounds))?;
        self.dictionary_used[index as usize] = true;
        Ok(entry)
    }

    fn mark_output_authority(&mut self, authority_index: Option<u8>) -> Result<Pubkey> {
        match authority_index {
            Some(index) => {
                let authority = self.table.account(u16::from(index))?.key();
                self.table.mark(u16::from(index))?;
                Ok(authority)
            }
            None => Ok(self.account_authority),
        }
    }

    fn mark_deny_record(&mut self, subject: Pubkey) -> Result<()> {
        self.table.mark_deny_record(self.deny_list_enabled, subject)
    }
}

fn preflight_eval_step(
    step: &FheExecuteStep,
    step_index: usize,
    preflight: &mut EvalPreflight<'_, '_, '_>,
) -> Result<()> {
    match step {
        FheExecuteStep::Binary {
            lhs, rhs, output, ..
        } => {
            preflight_encrypted_operand(lhs, step_index, preflight)?;
            preflight_rhs_operand(rhs, step_index, preflight)?;
            preflight_output(output, preflight)?;
        }
        FheExecuteStep::Ternary {
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
        FheExecuteStep::TrivialEncrypt { output, .. } | FheExecuteStep::Rand { output, .. } => {
            preflight_output(output, preflight)?;
        }
        FheExecuteStep::Unary {
            operand, output, ..
        } => {
            preflight_encrypted_operand(operand, step_index, preflight)?;
            preflight_output(output, preflight)?;
        }
        FheExecuteStep::RandBounded { output, .. } => {
            preflight_output(output, preflight)?;
        }
        FheExecuteStep::Sum {
            operands, output, ..
        } => {
            for operand in operands {
                preflight_encrypted_operand(operand, step_index, preflight)?;
            }
            preflight_output(output, preflight)?;
        }
        FheExecuteStep::IsIn {
            value, set, output, ..
        } => {
            preflight_encrypted_operand(value, step_index, preflight)?;
            for operand in set {
                preflight_encrypted_operand(operand, step_index, preflight)?;
            }
            preflight_output(output, preflight)?;
        }
        FheExecuteStep::MulDiv {
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
    operand: &FheExecuteOperand,
    step_index: usize,
    preflight: &mut EvalPreflight<'_, '_, '_>,
) -> Result<()> {
    match operand {
        FheExecuteOperand::Scalar { value_index } => {
            preflight.mark_dictionary(*value_index)?;
            Ok(())
        }
        _ => preflight_encrypted_operand(operand, step_index, preflight),
    }
}

fn preflight_encrypted_operand(
    operand: &FheExecuteOperand,
    step_index: usize,
    preflight: &mut EvalPreflight<'_, '_, '_>,
) -> Result<()> {
    match operand {
        FheExecuteOperand::AllowedPersistent {
            handle_index,
            encrypted_value_index,
        } => {
            preflight.mark_dictionary(*handle_index)?;
            let key = preflight
                .table
                .account(u16::from(*encrypted_value_index))?
                .key();
            require!(
                !preflight.persistent_outputs_written.contains(&key),
                ZamaHostError::FheExecutePersistentOperandWrittenEarlier
            );
            preflight.table.mark(u16::from(*encrypted_value_index))?;
        }
        FheExecuteOperand::AllowedLocal { producer_index } => {
            require!(
                (*producer_index as usize) < step_index,
                ZamaHostError::FheExecuteAllowedLocalMissing
            );
        }
        FheExecuteOperand::VerifiedInput { .. } => {
            // No remaining account: the attestation is carried inline and verified in-batch.
        }
        FheExecuteOperand::Scalar { .. } => {
            return Err(error!(ZamaHostError::InvalidFheExecuteAccount))
        }
    }
    Ok(())
}

fn preflight_output(
    output: &FheExecuteOutput,
    preflight: &mut EvalPreflight<'_, '_, '_>,
) -> Result<()> {
    match output {
        FheExecuteOutput::AllowedLocal => {}
        FheExecuteOutput::AllowedPersistent {
            output_encrypted_value_index,
            output_account_authority_index,
            output_domain_index,
            output_account_index,
            output_label_index,
            output_subject_indexes,
            previous_subjects,
            ..
        } => {
            let output_key = preflight
                .table
                .account(u16::from(*output_encrypted_value_index))?
                .key();
            preflight
                .table
                .mark(u16::from(*output_encrypted_value_index))?;
            preflight.mark_dictionary(*output_domain_index)?;
            preflight.mark_dictionary(*output_account_index)?;
            preflight.mark_dictionary(*output_label_index)?;
            let authority = preflight.mark_output_authority(*output_account_authority_index)?;
            preflight.mark_deny_record(authority)?;
            // Every newly granted subject is deny-checked in the bind pass; mark
            // their deny records here so finish() accounts for them. On a update
            // the new set is `output_subjects \ previous_subjects` from instruction
            // data alone — a lying previous_subjects is rejected later with
            // PreviousStateMismatch, so trusting it for account-marking is safe. On
            // a create (`None` previous) every output subject is a new grant.
            for subject_index in output_subject_indexes {
                let subject = Pubkey::new_from_array(preflight.mark_dictionary(*subject_index)?);
                let is_new_grant = match previous_subjects {
                    Some(previous_subjects) => !previous_subjects.contains(&subject),
                    None => true,
                };
                if is_new_grant {
                    preflight.mark_deny_record(subject)?;
                }
            }
            preflight.persistent_outputs_written.push(output_key);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unused_remaining_account_fails_preflight() {
        // One persistent operand, two passed accounts: the dangling second account
        // must fail the whole-batch all-used check.
        let args = batch(vec![FheExecuteStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheExecuteOperand::AllowedPersistent {
                handle_index: 0,
                encrypted_value_index: 0,
            },
            rhs: FheExecuteOperand::Scalar { value_index: 1 },
            output_fhe_type: 5,
            output: FheExecuteOutput::AllowedLocal,
        }]);
        let accounts = vec![
            test_account(Pubkey::new_unique()),
            test_account(Pubkey::new_unique()),
        ];
        let mut table = EvalAccountTable::new(&accounts).unwrap();

        assert!(
            preflight_eval_frame_accounts(&mut table, &args, Pubkey::new_unique(), false).is_err()
        );
    }

    #[test]
    fn persistent_operand_cannot_alias_an_account_written_by_an_earlier_step() {
        let args = batch(vec![
            FheExecuteStep::TrivialEncrypt {
                plaintext: [0; 32],
                fhe_type: 5,
                output: persistent_output(),
            },
            FheExecuteStep::Binary {
                op: FheBinaryOpCode::Add,
                lhs: FheExecuteOperand::AllowedPersistent {
                    handle_index: 0,
                    encrypted_value_index: 0,
                },
                rhs: FheExecuteOperand::Scalar { value_index: 1 },
                output_fhe_type: 5,
                output: FheExecuteOutput::AllowedLocal,
            },
        ]);
        let accounts = vec![test_account(Pubkey::new_unique())];
        let mut table = EvalAccountTable::new(&accounts).unwrap();

        assert!(
            preflight_eval_frame_accounts(&mut table, &args, Pubkey::new_unique(), false).is_err()
        );
    }

    #[test]
    fn persistent_operand_may_update_its_account_in_the_same_step() {
        let args = batch(vec![FheExecuteStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheExecuteOperand::AllowedPersistent {
                handle_index: 0,
                encrypted_value_index: 0,
            },
            rhs: FheExecuteOperand::Scalar { value_index: 1 },
            output_fhe_type: 5,
            output: persistent_output(),
        }]);
        let accounts = vec![test_account(Pubkey::new_unique())];
        let mut table = EvalAccountTable::new(&accounts).unwrap();

        assert!(
            preflight_eval_frame_accounts(&mut table, &args, Pubkey::new_unique(), false).is_ok()
        );
    }

    fn test_account(key: Pubkey) -> AccountInfo<'static> {
        let key = Box::leak(Box::new(key));
        let lamports = Box::leak(Box::new(0));
        let data = Box::leak(Vec::new().into_boxed_slice());
        let owner = Box::leak(Box::new(System::id()));
        AccountInfo::new(key, false, false, lamports, data, owner, false)
    }

    fn batch(steps: Vec<FheExecuteStep>) -> FheExecuteArgs {
        FheExecuteArgs {
            account_count: 0,
            dictionary: vec![[1; 32], [2; 32]],
            steps,
        }
    }

    fn trivial_local() -> FheExecuteStep {
        FheExecuteStep::TrivialEncrypt {
            plaintext: [0; 32],
            fhe_type: 5,
            output: FheExecuteOutput::AllowedLocal,
        }
    }

    fn persistent_output() -> FheExecuteOutput {
        FheExecuteOutput::AllowedPersistent {
            output_encrypted_value_index: 0,
            output_account_authority_index: None,
            output_domain_index: 0,
            output_account_index: 0,
            output_label_index: 1,
            output_subject_indexes: Vec::new(),
            previous_handle: None,
            previous_subjects: None,
            make_public: false,
        }
    }

    fn verified_input() -> FheExecuteOperand {
        FheExecuteOperand::VerifiedInput {
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

    #[test]
    fn rand_step_without_persistent_output_is_rejected() {
        // Unconditional (unlike the cap-gated persist-nothing rule): the rand seed is anchored
        // to the batch's persistent writes, so an all-transient rand batch has no seed anchor.
        let args = batch(vec![FheExecuteStep::Rand {
            fhe_type: 5,
            output: FheExecuteOutput::AllowedLocal,
        }]);
        assert!(assert_rand_steps_anchor_persistent_output(&args).is_err());
    }

    #[test]
    fn rand_step_with_persistent_output_anywhere_in_frame_is_accepted() {
        // The persistent output may be the rand step's own or any other step's.
        let own = batch(vec![FheExecuteStep::Rand {
            fhe_type: 5,
            output: persistent_output(),
        }]);
        assert!(assert_rand_steps_anchor_persistent_output(&own).is_ok());

        let elsewhere = batch(vec![
            FheExecuteStep::Rand {
                fhe_type: 5,
                output: FheExecuteOutput::AllowedLocal,
            },
            FheExecuteStep::TrivialEncrypt {
                plaintext: [0; 32],
                fhe_type: 5,
                output: persistent_output(),
            },
        ]);
        assert!(assert_rand_steps_anchor_persistent_output(&elsewhere).is_ok());
    }

    #[test]
    fn frame_without_rand_needs_no_persistent_output() {
        assert!(assert_rand_steps_anchor_persistent_output(&batch(vec![trivial_local()])).is_ok());
    }

    const FINITE_CAP: u64 = 500_000;

    #[test]
    fn deactivated_cap_never_rejects_persist_nothing_frame() {
        // u64::MAX (ship default) short-circuits: behavior is unchanged where the cap is not deployed.
        assert!(
            assert_frame_pins_or_persists_under_cap(&batch(vec![trivial_local()]), u64::MAX)
                .is_ok()
        );
    }

    #[test]
    fn finite_cap_rejects_persist_nothing_frame() {
        assert!(
            assert_frame_pins_or_persists_under_cap(&batch(vec![trivial_local()]), FINITE_CAP)
                .is_err()
        );
    }

    #[test]
    fn finite_cap_accepts_persistent_output_frame() {
        // The trivial-encrypt -> persistent-output bootstrap/mint path stays legal.
        let step = FheExecuteStep::TrivialEncrypt {
            plaintext: [0; 32],
            fhe_type: 5,
            output: persistent_output(),
        };
        assert!(assert_frame_pins_or_persists_under_cap(&batch(vec![step]), FINITE_CAP).is_ok());
    }

    #[test]
    fn finite_cap_accepts_persistent_input_frame() {
        let step = FheExecuteStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheExecuteOperand::AllowedPersistent {
                handle_index: 0,
                encrypted_value_index: 0,
            },
            rhs: FheExecuteOperand::Scalar { value_index: 1 },
            output_fhe_type: 5,
            output: FheExecuteOutput::AllowedLocal,
        };
        assert!(assert_frame_pins_or_persists_under_cap(&batch(vec![step]), FINITE_CAP).is_ok());
    }

    #[test]
    fn finite_cap_accepts_verified_input_transient_frame() {
        // A verified input pins the subject (attested contract must equal it), so a transient-output
        // batch that carries one is anchored and not persist-nothing.
        let step = FheExecuteStep::Unary {
            op: FheUnaryOpCode::Not,
            operand: verified_input(),
            output_fhe_type: 5,
            output: FheExecuteOutput::AllowedLocal,
        };
        assert!(assert_frame_pins_or_persists_under_cap(&batch(vec![step]), FINITE_CAP).is_ok());
    }
}
