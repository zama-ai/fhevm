//! Whole-execution checks that run before any state mutates: every remaining account and every
//! dictionary entry is referenced, every persistent operand's authority signed, every persistent
//! value belongs to one application, and that application's deny record is present when the
//! deny list is on. Returns the application identity the rest of the execution keys on.

use super::account_table::ExecutionAccountTable;
use super::*;

pub(super) fn preflight_execution<'info>(
    table: &mut ExecutionAccountTable<'_, 'info>,
    ctx: &Context<'info, FheExecute<'info>>,
    args: &FheExecuteArgs,
) -> Result<Option<AppScope>> {
    let mut preflight = Preflight {
        table,
        dictionary: &args.dictionary,
        dictionary_used: vec![false; args.dictionary.len()],
        encrypted_value_account_authority: ctx.accounts.encrypted_value_account_authority.key(),
        app: None,
        persistent_outputs_written: Vec::with_capacity(MAX_FHE_EXECUTION_STEPS),
    };
    for (index, step) in args.steps.iter().enumerate() {
        preflight_step(step, index, &mut preflight)?;
    }
    // Whole-execution hygiene, mirroring the account table: every interned dictionary
    // entry must be referenced by some step, so an execution cannot carry dead bytes.
    require!(
        preflight.dictionary_used.iter().all(|used| *used),
        ZamaHostError::FheExecuteDictionaryEntryUnreferenced
    );
    if let Some(app) = preflight.app {
        preflight
            .table
            .mark_deny_record(ctx.accounts.host_config.grant_deny_list_enabled, app)?;
    }
    preflight.table.assert_all_used()?;
    Ok(preflight.app)
}

/// Marks every account the execution references into the shared table so
/// [`ExecutionAccountTable::assert_all_used`] can reject dangling accounts before
/// any pass mutates state, and folds the application identity.
struct Preflight<'t, 'a, 'info> {
    table: &'t mut ExecutionAccountTable<'a, 'info>,
    dictionary: &'t [[u8; 32]],
    dictionary_used: Vec<bool>,
    encrypted_value_account_authority: Pubkey,
    /// The `(program, scope)` of every persistent value seen so far; a second one is an error.
    app: Option<AppScope>,
    /// Persistent accounts written by completed earlier steps. Operands are checked
    /// before the current step's output is recorded, so read-then-update in one
    /// step remains valid.
    persistent_outputs_written: Vec<Pubkey>,
}

impl Preflight<'_, '_, '_> {
    /// Marks a dictionary reference used and returns its bytes; out-of-range fails the execution here,
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

    fn mark_output_authority(&mut self, authority_index: Option<u8>) -> Result<()> {
        if let Some(index) = authority_index {
            self.table.mark(u16::from(index))?;
        }
        Ok(())
    }

    /// Folds one persistent value's application into the execution's: the first one is adopted,
    /// every later one must match (one execution, one meter).
    fn fold_app(&mut self, app: AppScope) -> Result<()> {
        match self.app {
            None => self.app = Some(app),
            Some(current) => require!(current == app, ZamaHostError::FheExecuteMixedScopes),
        }
        Ok(())
    }
}

fn preflight_step(
    step: &FheExecuteStep,
    step_index: usize,
    preflight: &mut Preflight<'_, '_, '_>,
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
        FheExecuteStep::TrivialEncrypt { output, .. }
        | FheExecuteStep::Rand { output, .. }
        | FheExecuteStep::RandBounded { output, .. } => {
            preflight_output(output, preflight)?;
        }
        FheExecuteStep::Unary {
            operand, output, ..
        } => {
            preflight_encrypted_operand(operand, step_index, preflight)?;
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
    preflight: &mut Preflight<'_, '_, '_>,
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
    preflight: &mut Preflight<'_, '_, '_>,
) -> Result<()> {
    match operand {
        FheExecuteOperand::StoredValue {
            handle_index,
            encrypted_value_index,
        } => {
            preflight.mark_dictionary(*handle_index)?;
            let index = u16::from(*encrypted_value_index);
            let key = preflight.table.account(index)?.key();
            require!(
                !preflight.persistent_outputs_written.contains(&key),
                ZamaHostError::FheExecutePersistentOperandWrittenEarlier
            );
            preflight.table.mark(index)?;
            // Reading a value is admitted by its authority's signature, and the value carries the
            // application it belongs to. Decoded once here; the walk reuses the cached decode.
            let value = preflight.table.canonical_encrypted_value(index)?;
            let authority = value.encrypted_value_account_authority;
            let app = AppScope {
                program: value.program,
                scope: value.scope,
            };
            preflight
                .table
                .mark_signer(authority, preflight.encrypted_value_account_authority)?;
            preflight.fold_app(app)?;
        }
        FheExecuteOperand::EarlierStep { producer_index } => {
            require!(
                (*producer_index as usize) < step_index,
                ZamaHostError::FheExecuteEarlierStepMissing
            );
        }
        FheExecuteOperand::VerifiedInput { .. } => {
            // No remaining account: the attestation is carried inline and verified in-execution.
        }
        FheExecuteOperand::Scalar { .. } => {
            return Err(error!(ZamaHostError::InvalidFheExecuteAccount))
        }
    }
    Ok(())
}

fn preflight_output(
    output: &FheExecuteOutput,
    preflight: &mut Preflight<'_, '_, '_>,
) -> Result<()> {
    match output {
        FheExecuteOutput::Transient => {}
        FheExecuteOutput::StoredValue {
            output_encrypted_value_index,
            output_authority_index,
            output_program_index,
            output_authority_key_index,
            output_scope_index,
            output_label_index,
            output_authority_seeds,
            output_allow_indexes,
            previous_handle_index,
            ..
        } => {
            let index = u16::from(*output_encrypted_value_index);
            let output_key = preflight.table.account(index)?.key();
            preflight.table.mark(index)?;
            let program = Pubkey::new_from_array(preflight.mark_dictionary(*output_program_index)?);
            preflight.mark_dictionary(*output_authority_key_index)?;
            let scope = preflight.mark_dictionary(*output_scope_index)?;
            preflight.mark_dictionary(*output_label_index)?;
            preflight.mark_output_authority(*output_authority_index)?;
            for seed in output_authority_seeds {
                if let PdaSeed::Interned { index } = seed {
                    preflight.mark_dictionary(*index)?;
                }
            }
            for allow_index in output_allow_indexes {
                preflight.mark_dictionary(*allow_index)?;
            }
            if let Some(index) = previous_handle_index {
                preflight.mark_dictionary(*index)?;
            }
            preflight.fold_app(AppScope { program, scope })?;
            preflight.persistent_outputs_written.push(output_key);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::AccountSerialize;

    fn test_account(key: Pubkey) -> AccountInfo<'static> {
        signer_account(key, false)
    }

    fn signer_account(key: Pubkey, is_signer: bool) -> AccountInfo<'static> {
        let key = Box::leak(Box::new(key));
        let lamports = Box::leak(Box::new(0));
        let data = Box::leak(Vec::new().into_boxed_slice());
        let owner = Box::leak(Box::new(System::id()));
        AccountInfo::new(key, is_signer, false, lamports, data, owner, false)
    }

    /// A canonical stored value of `app` controlled by `authority`, with `[1; 32]` as its handle.
    fn stored_value(app: AppScope, authority: Pubkey) -> AccountInfo<'static> {
        let mut value = EncryptedValue {
            program: app.program,
            encrypted_value_account_authority: authority,
            scope: app.scope,
            label: [9; 32],
            current_handle: [1; 32],
            leaf_count: 0,
            peaks: Vec::new(),
            bump: 0,
        };
        let (key, bump) = value.canonical_address();
        value.bump = bump;
        let mut data = Vec::new();
        value.try_serialize(&mut data).unwrap();
        AccountInfo::new(
            Box::leak(Box::new(key)),
            false,
            true,
            Box::leak(Box::new(0)),
            Box::leak(data.into_boxed_slice()),
            Box::leak(Box::new(crate::ID)),
            false,
        )
    }

    fn app(tag: u8) -> AppScope {
        AppScope {
            program: Pubkey::new_from_array([tag; 32]),
            scope: [tag; 32],
        }
    }

    fn read_step(encrypted_value_index: u8) -> FheExecuteStep {
        FheExecuteStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheExecuteOperand::StoredValue {
                handle_index: 0,
                encrypted_value_index,
            },
            rhs: FheExecuteOperand::Scalar { value_index: 1 },
            output_fhe_type: 5,
            output: FheExecuteOutput::Transient,
        }
    }

    fn execution(steps: Vec<FheExecuteStep>) -> FheExecuteArgs {
        FheExecuteArgs {
            account_count: 0,
            dictionary: vec![[1; 32], [2; 32]],
            steps,
        }
    }

    /// Runs the account half of preflight with `default_authority` as the context signer.
    fn run(
        accounts: &[AccountInfo<'static>],
        args: &FheExecuteArgs,
        default_authority: Pubkey,
    ) -> Result<Option<AppScope>> {
        let mut table = ExecutionAccountTable::new(accounts).unwrap();
        let mut preflight = Preflight {
            table: &mut table,
            dictionary: &args.dictionary,
            dictionary_used: vec![false; args.dictionary.len()],
            encrypted_value_account_authority: default_authority,
            app: None,
            persistent_outputs_written: Vec::new(),
        };
        for (index, step) in args.steps.iter().enumerate() {
            preflight_step(step, index, &mut preflight)?;
        }
        let app = preflight.app;
        preflight.table.assert_all_used()?;
        Ok(app)
    }

    #[test]
    fn unused_remaining_account_fails_preflight() {
        // One persistent operand, two passed accounts: the dangling second account
        // must fail the whole-execution all-used check.
        let authority = Pubkey::new_unique();
        let accounts = vec![
            stored_value(app(1), authority),
            test_account(Pubkey::new_unique()),
        ];
        assert!(run(&accounts, &execution(vec![read_step(0)]), authority).is_err());
    }

    #[test]
    fn persistent_operand_cannot_alias_an_account_written_by_an_earlier_step() {
        let authority = Pubkey::new_unique();
        let args = execution(vec![
            FheExecuteStep::TrivialEncrypt {
                plaintext: [0; 32],
                fhe_type: 5,
                output: persistent_output(),
            },
            read_step(0),
        ]);
        let accounts = vec![stored_value(app(1), authority)];
        assert!(run(&accounts, &args, authority).is_err());
    }

    #[test]
    fn persistent_operand_may_update_its_account_in_the_same_step() {
        let authority = Pubkey::new_unique();
        let args = execution(vec![FheExecuteStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheExecuteOperand::StoredValue {
                handle_index: 0,
                encrypted_value_index: 0,
            },
            rhs: FheExecuteOperand::Scalar { value_index: 1 },
            output_fhe_type: 5,
            output: persistent_output(),
        }]);
        let accounts = vec![stored_value(app(1), authority)];
        assert!(run(&accounts, &args, authority).is_ok());
    }

    /// Reading a value needs its authority's signature: the default context signer admits it,
    /// so does a signing remaining account, and nothing else does.
    #[test]
    fn persistent_operand_is_admitted_by_its_authority_signature_only() {
        let authority = Pubkey::new_unique();
        let args = execution(vec![read_step(0)]);

        let by_default_signer = vec![stored_value(app(1), authority)];
        assert_eq!(
            run(&by_default_signer, &args, authority).unwrap(),
            Some(app(1))
        );

        let by_remaining_signer = vec![
            stored_value(app(1), authority),
            signer_account(authority, true),
        ];
        assert!(run(&by_remaining_signer, &args, Pubkey::new_unique()).is_ok());

        let authority_present_but_not_signing = vec![
            stored_value(app(1), authority),
            signer_account(authority, false),
        ];
        assert_eq!(
            run(
                &authority_present_but_not_signing,
                &args,
                Pubkey::new_unique()
            )
            .unwrap_err(),
            error!(ZamaHostError::EncryptedValueAccountAuthorityMismatch)
        );

        let nobody = vec![stored_value(app(1), authority)];
        assert_eq!(
            run(&nobody, &args, Pubkey::new_unique()).unwrap_err(),
            error!(ZamaHostError::EncryptedValueAccountAuthorityMismatch)
        );
    }

    /// One execution, one application: values of two scopes cannot be mixed, whether the second
    /// comes from another operand or from a declared output.
    #[test]
    fn values_of_two_scopes_cannot_share_an_execution() {
        let authority = Pubkey::new_unique();
        let two_reads = execution(vec![read_step(0), read_step(1)]);
        let same_scope = vec![
            stored_value(app(1), authority),
            stored_value(app(1), Pubkey::new_unique()),
        ];
        // The second value's authority did not sign, so use one authority for both.
        let same_scope = vec![same_scope.into_iter().next().unwrap(), {
            let mut value = EncryptedValue {
                program: app(1).program,
                encrypted_value_account_authority: authority,
                scope: app(1).scope,
                label: [8; 32],
                current_handle: [1; 32],
                leaf_count: 0,
                peaks: Vec::new(),
                bump: 0,
            };
            let (key, bump) = value.canonical_address();
            value.bump = bump;
            let mut data = Vec::new();
            value.try_serialize(&mut data).unwrap();
            AccountInfo::new(
                Box::leak(Box::new(key)),
                false,
                true,
                Box::leak(Box::new(0)),
                Box::leak(data.into_boxed_slice()),
                Box::leak(Box::new(crate::ID)),
                false,
            )
        }];
        assert_eq!(
            run(&same_scope, &two_reads, authority).unwrap(),
            Some(app(1))
        );

        let mixed = vec![
            stored_value(app(1), authority),
            stored_value(app(2), authority),
        ];
        assert_eq!(
            run(&mixed, &two_reads, authority).unwrap_err(),
            error!(ZamaHostError::FheExecuteMixedScopes)
        );

        // Output declared for scope 2 while reading scope 1: dictionary entry 2 is the program
        // and scope of the output.
        let mut args = execution(vec![FheExecuteStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheExecuteOperand::StoredValue {
                handle_index: 0,
                encrypted_value_index: 0,
            },
            rhs: FheExecuteOperand::Scalar { value_index: 1 },
            output_fhe_type: 5,
            output: FheExecuteOutput::StoredValue {
                output_encrypted_value_index: 1,
                output_authority_index: None,
                output_program_index: 2,
                output_authority_key_index: 2,
                output_scope_index: 2,
                output_label_index: 1,
                output_authority_seeds: Vec::new(),
                output_allow_indexes: Vec::new(),
                previous_handle_index: None,
                make_public: false,
            },
        }]);
        args.dictionary.push([2; 32]);
        let accounts = vec![
            stored_value(app(1), authority),
            test_account(Pubkey::new_unique()),
        ];
        assert_eq!(
            run(&accounts, &args, authority).unwrap_err(),
            error!(ZamaHostError::FheExecuteMixedScopes)
        );
    }

    #[test]
    fn transient_only_execution_has_no_application() {
        let args = execution(vec![FheExecuteStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheExecuteOperand::EarlierStep { producer_index: 0 },
            rhs: FheExecuteOperand::Scalar { value_index: 1 },
            output_fhe_type: 5,
            output: FheExecuteOutput::Transient,
        }]);
        // Step 0 referencing itself as an earlier step is rejected; use a trivial first.
        let args = FheExecuteArgs {
            steps: vec![
                FheExecuteStep::TrivialEncrypt {
                    plaintext: [0; 32],
                    fhe_type: 5,
                    output: FheExecuteOutput::Transient,
                },
                args.steps[0].clone(),
            ],
            dictionary: vec![[1; 32], [2; 32]],
            account_count: 0,
        };
        // Dictionary entry 0 is unreferenced here; only the account half is under test.
        let mut table = ExecutionAccountTable::new(&[]).unwrap();
        let mut preflight = Preflight {
            table: &mut table,
            dictionary: &args.dictionary,
            dictionary_used: vec![false; 2],
            encrypted_value_account_authority: Pubkey::new_unique(),
            app: None,
            persistent_outputs_written: Vec::new(),
        };
        for (index, step) in args.steps.iter().enumerate() {
            preflight_step(step, index, &mut preflight).unwrap();
        }
        assert_eq!(preflight.app, None);
    }

    fn persistent_output() -> FheExecuteOutput {
        FheExecuteOutput::StoredValue {
            output_encrypted_value_index: 0,
            output_authority_index: None,
            output_program_index: 0,
            output_authority_key_index: 0,
            output_scope_index: 0,
            output_label_index: 1,
            output_authority_seeds: Vec::new(),
            output_allow_indexes: Vec::new(),
            previous_handle_index: None,
            make_public: false,
        }
    }
}
