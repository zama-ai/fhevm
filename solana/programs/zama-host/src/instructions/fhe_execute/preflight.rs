//! Whole-execution checks that run before any state mutates: every remaining account and every
//! dictionary entry is referenced, every persistent operand's authority signed, every persistent
//! value the default authority controls belongs to one application, and the deny record of every
//! application the execution touches is present when the deny list is on. Returns the application
//! identity the rest of the execution keys on and the applications to deny-check.
//!
//! A value admitted by an additional signing authority belongs to that authority's own
//! application and does not fold: the signature is that program's consent, given through
//! `invoke_signed`, for this execution to read the value or write it. That is how programs
//! compose on Solana: the token can spend an amount in a JoinRecord's contribution State when
//! the batcher signs as that JoinRecord. The meter stays on the default authority's application.
//! States read, written or used to initiate a grant are deny-checked independently of that meter;
//! a grant's consumer State is deny-checked when the grant is consumed.

use super::account_table::ExecutionAccountTable;
use super::*;

pub(super) fn preflight_execution<'info>(
    table: &mut ExecutionAccountTable<'_, 'info>,
    ctx: &Context<'info, FheExecute<'info>>,
    args: &FheExecuteArgs,
) -> Result<PreflightOutcome> {
    let mut preflight = Preflight {
        table,
        dictionary: &args.dictionary,
        dictionary_used: vec![false; args.dictionary.len()],
        authority: ctx.accounts.authority.key(),
        app: None,
        touched_apps: Vec::new(),
        slots_written: Vec::with_capacity(MAX_FHE_EXECUTION_STEPS),
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
    for app in &preflight.touched_apps {
        preflight
            .table
            .mark_deny_record(ctx.accounts.host_config.grant_deny_list_enabled, *app)?;
    }
    preflight.table.assert_all_used()?;
    Ok(PreflightOutcome {
        app: preflight.app,
        touched_apps: preflight.touched_apps,
    })
}

#[derive(Debug)]
pub(super) struct PreflightOutcome {
    /// The application the default authority's persistent values belong to: what the execution
    /// is metered and rand-seeded as. `None` when it controls none.
    pub(super) app: Option<AppScope>,
    /// Every distinct application whose persistent value the execution reads or writes, under
    /// any signing authority. Each is deny-checked.
    pub(super) touched_apps: Vec<AppScope>,
}

/// Marks every account the execution references into the shared table so
/// [`ExecutionAccountTable::assert_all_used`] can reject dangling accounts before
/// any pass mutates state, and folds the application identity.
struct Preflight<'t, 'a, 'info> {
    table: &'t mut ExecutionAccountTable<'a, 'info>,
    dictionary: &'t [[u8; 32]],
    dictionary_used: Vec<bool>,
    authority: Pubkey,
    /// The `(program, scope)` of every persistent value the default authority controls; a second
    /// one is an error.
    app: Option<AppScope>,
    /// Every distinct application touched, the default authority's included, in first-seen order.
    touched_apps: Vec<AppScope>,
    /// State slots written by completed earlier steps. Operands are checked
    /// before the current step's output is recorded, so read-then-update in one
    /// step remains valid.
    slots_written: Vec<(u8, [u8; 32])>,
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

    fn admit_state(&mut self, index: u8) -> Result<()> {
        self.table.mark(index.into())?;
        let state = self.table.state(index.into())?;
        let authority = state.authority;
        let app = AppScope {
            program: state.program,
            scope: state.scope,
        };
        self.table.mark_signer(authority, self.authority)?;
        self.fold_app(authority, app)
    }

    /// Records one persistent value's application for the deny check and folds it into the
    /// execution's when the default authority controls it: the first one is adopted, every later
    /// one must match (one execution, one meter). A value under an additional signing authority
    /// is that program's own.
    fn fold_app(&mut self, authority: Pubkey, app: AppScope) -> Result<()> {
        if !self.touched_apps.contains(&app) {
            self.touched_apps.push(app);
        }
        if authority != self.authority {
            return Ok(());
        }
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
        FheExecuteOperand::StateSlot {
            handle_index,
            state_index,
            key_index,
        } => {
            preflight.mark_dictionary(*handle_index)?;
            let key = preflight.mark_dictionary(*key_index)?;
            require!(
                !preflight.slots_written.contains(&(*state_index, key)),
                ZamaHostError::FheExecutePersistentOperandWrittenEarlier
            );
            preflight.admit_state(*state_index)?;
        }
        FheExecuteOperand::TransientResult {
            handle_index,
            scratch_index,
            consumer_state_index,
        } => {
            preflight.mark_dictionary(*handle_index)?;
            preflight.table.mark((*scratch_index).into())?;
            preflight.admit_state(*consumer_state_index)?;
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
        FheExecuteOutput::State {
            state_index,
            slot,
            allow_indexes,
            grants,
            ..
        } => {
            preflight.admit_state(*state_index)?;
            if let Some(slot) = slot {
                let key = preflight.mark_dictionary(slot.key_index)?;
                require!(
                    !preflight.slots_written.contains(&(*state_index, key)),
                    ZamaHostError::InvalidFheExecuteAccount
                );
                preflight.slots_written.push((*state_index, key));
                if let Some(previous) = slot.previous_handle_index {
                    preflight.mark_dictionary(previous)?;
                }
            }
            for index in allow_indexes {
                preflight.mark_dictionary(*index)?;
            }
            require!(
                grants.len() <= MAX_TRANSIENT_GRANTS,
                ZamaHostError::TransientCapacityExceeded
            );
            for grant in grants {
                preflight.table.mark(grant.scratch_index.into())?;
                preflight.admit_state(grant.initiating_state_index)?;
                preflight.table.mark(grant.consumer_state_index.into())?;
                preflight.table.state(grant.consumer_state_index.into())?;
            }
        }
        FheExecuteOutput::Transient => {}
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

    fn encrypted_state(app: AppScope, authority: Pubkey, key: [u8; 32]) -> AccountInfo<'static> {
        let mut state = EncryptedState {
            program: app.program,
            authority,
            scope: app.scope,
            slots: vec![EncryptedSlot {
                key,
                handle: [1; 32],
            }],
            leaf_count: 0,
            peaks: Vec::new(),
            bump: 0,
        };
        let (address, bump) = state.canonical_address();
        state.bump = bump;
        let mut data = Vec::new();
        state.try_serialize(&mut data).unwrap();
        AccountInfo::new(
            Box::leak(Box::new(address)),
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

    fn read_step(state_index: u8, key_index: u8) -> FheExecuteStep {
        FheExecuteStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheExecuteOperand::StateSlot {
                handle_index: 0,
                state_index,
                key_index,
            },
            rhs: FheExecuteOperand::Scalar { value_index: 1 },
            output_fhe_type: 5,
            output: FheExecuteOutput::Transient,
        }
    }

    fn execution(steps: Vec<FheExecuteStep>) -> FheExecuteArgs {
        FheExecuteArgs {
            returned_results: Vec::new(),
            account_count: 0,
            dictionary: vec![[1; 32], [2; 32], [9; 32]],
            steps,
        }
    }

    /// Runs the account half of preflight with `default_authority` as the context signer.
    fn run(
        accounts: &[AccountInfo<'static>],
        args: &FheExecuteArgs,
        default_authority: Pubkey,
    ) -> Result<PreflightOutcome> {
        let mut table = ExecutionAccountTable::new(accounts).unwrap();
        let mut preflight = Preflight {
            table: &mut table,
            dictionary: &args.dictionary,
            dictionary_used: vec![false; args.dictionary.len()],
            authority: default_authority,
            app: None,
            touched_apps: Vec::new(),

            slots_written: Vec::new(),
        };
        for (index, step) in args.steps.iter().enumerate() {
            preflight_step(step, index, &mut preflight)?;
        }
        preflight.table.assert_all_used()?;
        Ok(PreflightOutcome {
            app: preflight.app,
            touched_apps: preflight.touched_apps,
        })
    }

    #[test]
    fn unused_remaining_account_fails_preflight() {
        // One State-slot operand, two passed accounts: the dangling second account
        // must fail the whole-execution all-used check.
        let authority = Pubkey::new_unique();
        let accounts = vec![
            encrypted_state(app(1), authority, [9; 32]),
            test_account(Pubkey::new_unique()),
        ];
        assert!(run(&accounts, &execution(vec![read_step(0, 2)]), authority).is_err());
    }

    #[test]
    fn state_slot_cannot_be_read_after_an_earlier_step_writes_it() {
        let authority = Pubkey::new_unique();
        let args = execution(vec![
            FheExecuteStep::TrivialEncrypt {
                plaintext: [0; 32],
                fhe_type: 5,
                output: state_output(0, 2),
            },
            read_step(0, 2),
        ]);
        let accounts = vec![encrypted_state(app(1), authority, [9; 32])];
        assert!(run(&accounts, &args, authority).is_err());
    }

    #[test]
    fn state_slot_may_be_read_and_updated_in_the_same_step() {
        let authority = Pubkey::new_unique();
        let args = execution(vec![FheExecuteStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheExecuteOperand::StateSlot {
                handle_index: 0,
                state_index: 0,
                key_index: 2,
            },
            rhs: FheExecuteOperand::Scalar { value_index: 1 },
            output_fhe_type: 5,
            output: state_output(0, 2),
        }]);
        let accounts = vec![encrypted_state(app(1), authority, [9; 32])];
        assert!(run(&accounts, &args, authority).is_ok());
    }

    /// Reading a value needs its authority's signature: the default context signer admits it,
    /// so does a signing remaining account, and nothing else does.
    #[test]
    fn state_slot_is_admitted_by_its_authority_signature_only() {
        let authority = Pubkey::new_unique();
        let args = execution(vec![read_step(0, 2)]);

        let by_default_signer = vec![encrypted_state(app(1), authority, [9; 32])];
        assert_eq!(
            run(&by_default_signer, &args, authority).unwrap().app,
            Some(app(1))
        );

        let by_remaining_signer = vec![
            encrypted_state(app(1), authority, [9; 32]),
            signer_account(authority, true),
        ];
        assert!(run(&by_remaining_signer, &args, Pubkey::new_unique()).is_ok());

        let authority_present_but_not_signing = vec![
            encrypted_state(app(1), authority, [9; 32]),
            signer_account(authority, false),
        ];
        assert_eq!(
            run(
                &authority_present_but_not_signing,
                &args,
                Pubkey::new_unique()
            )
            .unwrap_err(),
            error!(ZamaHostError::EncryptedStateAccountAuthorityMismatch)
        );

        let nobody = vec![encrypted_state(app(1), authority, [9; 32])];
        assert_eq!(
            run(&nobody, &args, Pubkey::new_unique()).unwrap_err(),
            error!(ZamaHostError::EncryptedStateAccountAuthorityMismatch)
        );
    }

    /// One execution under its default authority cannot mix States from two applications.
    #[test]
    fn values_of_two_scopes_cannot_share_an_execution() {
        let authority = Pubkey::new_unique();
        let same_scope = vec![encrypted_state(app(1), authority, [9; 32])];
        assert_eq!(
            run(
                &same_scope,
                &execution(vec![read_step(0, 2), read_step(0, 2)]),
                authority,
            )
            .unwrap()
            .app,
            Some(app(1))
        );

        let mixed = vec![
            encrypted_state(app(1), authority, [9; 32]),
            encrypted_state(app(2), authority, [9; 32]),
        ];
        assert_eq!(
            run(
                &mixed,
                &execution(vec![read_step(0, 2), read_step(1, 2)]),
                authority,
            )
            .unwrap_err(),
            error!(ZamaHostError::FheExecuteMixedScopes)
        );
    }

    /// A value under an additional signing authority is that program's own: reading or writing
    /// it neither folds into nor conflicts with the execution's application, but its application
    /// is still one the execution touches, so it is deny-checked.
    #[test]
    fn values_of_another_signing_authority_keep_their_own_application() {
        let authority = Pubkey::new_unique();
        let foreign_authority = Pubkey::new_unique();
        let two_reads = execution(vec![read_step(0, 2), read_step(1, 2)]);
        let accounts = vec![
            encrypted_state(app(1), authority, [9; 32]),
            encrypted_state(app(2), foreign_authority, [9; 32]),
            signer_account(foreign_authority, true),
        ];
        let outcome = run(&accounts, &two_reads, authority).unwrap();
        assert_eq!(outcome.app, Some(app(1)));
        assert_eq!(outcome.touched_apps, vec![app(1), app(2)]);
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
            returned_results: Vec::new(),
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
            authority: Pubkey::new_unique(),
            app: None,
            touched_apps: Vec::new(),

            slots_written: Vec::new(),
        };
        for (index, step) in args.steps.iter().enumerate() {
            preflight_step(step, index, &mut preflight).unwrap();
        }
        assert_eq!(preflight.app, None);
    }

    fn state_output(state_index: u8, key_index: u8) -> FheExecuteOutput {
        FheExecuteOutput::State {
            state_index,
            previous_leaf_count: 0,
            slot: Some(SlotWrite {
                key_index,
                previous_handle_index: Some(0),
            }),
            allow_indexes: Vec::new(),
            make_public: false,
            grants: Vec::new(),
        }
    }
}
