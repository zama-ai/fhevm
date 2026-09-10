use anchor_lang::prelude::Instructions;
use anchor_lang::solana_program::program_error::ProgramError;
use anchor_lang::{AccountDeserialize, InstructionData};
use mollusk_svm::result::types::{TransactionProgramResult, TransactionResult};
use solana_sdk::sysvar::SysvarId;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use zama_host as host;
use zama_solana_test_kit::{
    anchor_ix, empty_system_account, funded_system_account, host_svm, serialized_account,
    system_program_account,
};

struct Fixture {
    payer: Pubkey,
    authority: Pubkey,
    state: Pubkey,
    scratch: Pubkey,
    accounts: Vec<(Pubkey, Account)>,
}

impl Fixture {
    fn new(prefund: u64) -> Self {
        let payer = Pubkey::new_unique();
        let program = Pubkey::new_unique();
        let (authority, _) = Pubkey::find_program_address(&[b"authority"], &program);
        let scope = [3; 32];
        let (state, bump) = host::encrypted_state_address(program, authority, scope);
        let (scratch, _) = host::transient_address(state);
        let mut scratch_account = empty_system_account();
        scratch_account.lamports = prefund;
        Self {
            payer,
            authority,
            state,
            scratch,
            accounts: vec![
                (payer, funded_system_account()),
                (authority, empty_system_account()),
                (
                    state,
                    Account {
                        lamports: 10_000_000,
                        owner: host::ID,
                        data: serialized_account(host::EncryptedState {
                            program,
                            authority,
                            scope,
                            slots: vec![],
                            leaf_count: 0,
                            peaks: vec![],
                            bump,
                        }),
                        ..Account::default()
                    },
                ),
                (scratch, scratch_account),
                (
                    anchor_lang::prelude::system_program::ID,
                    system_program_account(),
                ),
            ],
        }
    }

    fn open(&self) -> Instruction {
        anchor_ix(
            host::ID,
            host::accounts::OpenScratch {
                payer: self.payer,
                authority: self.authority,
                encrypted_state: self.state,
                scratch: self.scratch,
                instructions: Instructions::id(),
                system_program: anchor_lang::prelude::system_program::ID,
            },
            host::instruction::OpenScratch {},
        )
    }

    fn close(&self) -> Instruction {
        Instruction {
            program_id: host::ID,
            data: host::instruction::CloseScratch {}.data(),
            accounts: vec![
                AccountMeta::new_readonly(Instructions::id(), false),
                AccountMeta::new(self.scratch, false),
                AccountMeta::new(self.payer, false),
            ],
        }
    }
}

#[test]
fn scratch_is_created_and_closed_atomically_including_prefunded_addresses() {
    for donation in [0, 1_000_000] {
        let fixture = Fixture::new(donation);
        let result = host_svm().process_transaction_instructions(
            &[fixture.open(), fixture.close()],
            &fixture.accounts,
        );
        assert!(result.raw_result.is_ok(), "{:?}", result.raw_result);
        let account = |key| {
            &result
                .resulting_accounts
                .iter()
                .find(|(k, _)| *k == key)
                .unwrap()
                .1
        };
        assert_eq!(account(fixture.scratch).lamports, 0);
        assert!(account(fixture.scratch).data.is_empty());
        let initial_payer = fixture
            .accounts
            .iter()
            .find(|(key, _)| *key == fixture.payer)
            .unwrap()
            .1
            .lamports;
        assert_eq!(account(fixture.payer).lamports, initial_payer + donation);
        // The same canonical address can be reused, but no grants survive the prior transaction.
        let again = host_svm().process_transaction_instructions(
            &[fixture.open(), fixture.close()],
            &result.resulting_accounts,
        );
        assert!(again.raw_result.is_ok(), "{:?}", again.raw_result);
    }
}

#[test]
fn missing_close_and_unsigned_open_leave_all_accounts_unchanged() {
    let fixture = Fixture::new(0);
    let result = host_svm().process_transaction_instructions(&[fixture.open()], &fixture.accounts);
    assert_eq!(
        result.program_result,
        TransactionProgramResult::Failure(
            0,
            ProgramError::Custom(6000 + host::ZamaHostError::TransientCloseMissing as u32)
        )
    );
    assert_eq!(result.resulting_accounts, fixture.accounts);
    let mut open = fixture.open();
    open.accounts[1].is_signer = false;
    let result =
        host_svm().process_transaction_instructions(&[open, fixture.close()], &fixture.accounts);
    assert_eq!(
        result.program_result,
        TransactionProgramResult::Failure(
            0,
            ProgramError::Custom(anchor_lang::error::ErrorCode::AccountNotSigner as u32)
        )
    );
    assert_eq!(result.resulting_accounts, fixture.accounts);
}

#[test]
fn scratch_cannot_close_before_the_final_instruction() {
    let fixture = Fixture::new(0);
    let result = host_svm().process_transaction_instructions(
        &[fixture.open(), fixture.close(), fixture.close()],
        &fixture.accounts,
    );
    assert_eq!(
        result.program_result,
        TransactionProgramResult::Failure(
            1,
            ProgramError::Custom(6000 + host::ZamaHostError::TransientCloseMissing as u32)
        )
    );
    assert_eq!(result.resulting_accounts, fixture.accounts);
}

#[test]
fn nested_scratch_close_rolls_back_the_whole_transaction() {
    let fixture = Fixture::new(0);
    let mut svm = host_svm();
    svm.add_program(&delegator_vault::ID, "delegator_vault");
    let mut nested_close = anchor_ix(
        delegator_vault::ID,
        delegator_vault::accounts::CloseScratchViaCpi {
            instructions: Instructions::id(),
            zama_host: host::ID,
        },
        delegator_vault::instruction::CloseScratchViaCpi {},
    );
    nested_close.accounts.extend([
        AccountMeta::new(fixture.scratch, false),
        AccountMeta::new(fixture.payer, false),
    ]);
    // The valid final close lets open succeed; the intervening CPI must still be rejected.
    let result = svm.process_transaction_instructions(
        &[fixture.open(), nested_close, fixture.close()],
        &fixture.accounts,
    );
    assert_eq!(
        result.program_result,
        TransactionProgramResult::Failure(
            1,
            ProgramError::Custom(6000 + host::ZamaHostError::TransientCloseMissing as u32)
        )
    );
    assert_eq!(result.resulting_accounts, fixture.accounts);
}

#[test]
fn active_workspace_cannot_be_reopened() {
    let fixture = Fixture::new(0);
    let result = host_svm().process_transaction_instructions(
        &[fixture.open(), fixture.open(), fixture.close()],
        &fixture.accounts,
    );
    assert_eq!(
        result.program_result,
        TransactionProgramResult::Failure(
            1,
            ProgramError::Custom(
                6000 + host::ZamaHostError::FheExecuteOutputAlreadyInitialized as u32
            )
        )
    );
    assert_eq!(result.resulting_accounts, fixture.accounts);
}

#[test]
fn refund_substitution_and_duplicate_close_are_rejected() {
    let fixture = Fixture::new(0);
    let mut close = fixture.close();
    close.accounts[2].pubkey = fixture.authority;
    let result =
        host_svm().process_transaction_instructions(&[fixture.open(), close], &fixture.accounts);
    assert_eq!(
        result.program_result,
        TransactionProgramResult::Failure(
            0,
            ProgramError::Custom(6000 + host::ZamaHostError::TransientCloseMissing as u32)
        )
    );
    assert_eq!(result.resulting_accounts, fixture.accounts);
    let mut close = fixture.close();
    close.accounts.extend_from_within(1..3);
    let result =
        host_svm().process_transaction_instructions(&[fixture.open(), close], &fixture.accounts);
    assert_eq!(
        result.program_result,
        TransactionProgramResult::Failure(
            0,
            ProgramError::Custom(6000 + host::ZamaHostError::TransientCloseMissing as u32)
        )
    );
    assert_eq!(result.resulting_accounts, fixture.accounts);
}

#[test]
fn one_final_close_handles_two_independent_workspaces() {
    let first = Fixture::new(0);
    let second = Fixture::new(0);
    let mut accounts = first.accounts.clone();
    accounts.extend(
        second
            .accounts
            .iter()
            .filter(|(key, _)| !first.accounts.iter().any(|(existing, _)| existing == key))
            .cloned(),
    );
    let mut close = first.close();
    close
        .accounts
        .extend_from_slice(&second.close().accounts[1..]);
    let result = host_svm()
        .process_transaction_instructions(&[first.open(), second.open(), close], &accounts);
    assert!(result.raw_result.is_ok(), "{:?}", result.raw_result);
    for scratch in [first.scratch, second.scratch] {
        let account = &result
            .resulting_accounts
            .iter()
            .find(|(key, _)| *key == scratch)
            .unwrap()
            .1;
        assert_eq!(account.lamports, 0);
        assert!(host::TransientState::try_deserialize(&mut account.data.as_slice()).is_err());
    }
}

#[test]
fn two_slots_share_history_and_stale_slot_writes_roll_back() {
    let fixture = Fixture::new(0);
    let (config, config_account) = zama_solana_test_kit::host_config_account(
        &zama_solana_test_kit::HostConfigParams::new(Pubkey::new_unique()),
    );
    let event_authority = zama_solana_test_kit::event_authority(host::ID);
    let mut accounts = fixture.accounts.clone();
    accounts.extend([
        (config, config_account),
        (event_authority, empty_system_account()),
    ]);
    let output = |key_index| host::FheExecuteOutput::State {
        state_index: 0,
        previous_leaf_count: key_index as u64,
        slot: Some(host::SlotWrite {
            key_index,
            previous_handle_index: None,
        }),
        allow_indexes: vec![2],
        make_public: false,
        grants: vec![],
    };
    let mut one = [0; 32];
    one[31] = 1;
    let mut two = [0; 32];
    two[31] = 2;
    let args = host::FheExecuteArgs {
        returned_results: vec![
            host::ExecutionResultRef {
                step_index: 0,
                output_index: 0,
            },
            host::ExecutionResultRef {
                step_index: 1,
                output_index: 0,
            },
        ],
        account_count: 1,
        dictionary: vec![[11; 32], [12; 32], fixture.payer.to_bytes()],
        steps: vec![
            host::FheExecuteStep::TrivialEncrypt {
                plaintext: one,
                fhe_type: 5,
                output: output(0),
            },
            host::FheExecuteStep::TrivialEncrypt {
                plaintext: two,
                fhe_type: 5,
                output: output(1),
            },
        ],
    };
    let mut instruction = anchor_ix(
        host::ID,
        host::accounts::FheExecute {
            payer: fixture.payer,
            authority: fixture.authority,
            host_config: config,
            system_program: anchor_lang::prelude::system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            rand_nonce: None,
            event_authority,
            program: host::ID,
        },
        host::instruction::FheExecute { args: args.clone() },
    );
    instruction
        .accounts
        .push(AccountMeta::new(fixture.state, false));
    let svm = host_svm();
    let result = svm.process_transaction_instructions(&[instruction.clone()], &accounts);
    assert!(result.raw_result.is_ok(), "{:?}", result.raw_result);
    let state_account = &result
        .resulting_accounts
        .iter()
        .find(|(key, _)| *key == fixture.state)
        .unwrap()
        .1;
    let state = zama_solana_acl::decode_encrypted_state(&state_account.data).unwrap();
    assert_eq!(state.slots.len(), 2);
    assert_eq!(state.leaf_count, 2);
    let handles = [state.get(&[11; 32]).unwrap(), state.get(&[12; 32]).unwrap()];
    assert_ne!(handles[0], handles[1]);
    let leaves: Vec<_> = handles
        .iter()
        .enumerate()
        .map(|(index, handle)| {
            zama_solana_acl::historical_access_leaf_commitment(
                fixture.state.to_bytes(),
                index as u64,
                *handle,
                fixture.payer.to_bytes(),
            )
        })
        .collect();
    assert_eq!(state.peaks, zama_solana_acl::mmr_peaks_from_leaves(&leaves));
    assert_eq!(result.return_data, handles.concat());
    // Selection order is independent of execution order; omitted results stay off the channel.
    for selected in [
        vec![],
        vec![1],
        vec![1, 0],
        vec![1, 1],
        vec![1; host::MAX_RETURNED_HANDLES],
    ] {
        let mut selected_args = args.clone();
        selected_args.returned_results = selected
            .iter()
            .map(|&step_index| host::ExecutionResultRef {
                step_index,
                output_index: 0,
            })
            .collect();
        let mut selected_ix = instruction.clone();
        selected_ix.data = host::instruction::FheExecute {
            args: selected_args,
        }
        .data();
        let selected_result = svm.process_transaction_instructions(&[selected_ix], &accounts);
        assert!(
            selected_result.raw_result.is_ok(),
            "{:?}",
            selected_result.raw_result
        );
        let expected: Vec<u8> = selected.iter().flat_map(|&i| handles[i as usize]).collect();
        assert_eq!(selected_result.return_data, expected);
        assert_eq!(
            selected_result.resulting_accounts,
            result.resulting_accounts
        );
    }
    for invalid in [
        vec![host::ExecutionResultRef {
            step_index: 2,
            output_index: 0,
        }],
        vec![host::ExecutionResultRef {
            step_index: 0,
            output_index: 1,
        }],
        vec![
            host::ExecutionResultRef {
                step_index: 0,
                output_index: 0
            };
            host::MAX_RETURNED_HANDLES + 1
        ],
    ] {
        let mut invalid_args = args.clone();
        invalid_args.returned_results = invalid;
        let mut invalid_ix = instruction.clone();
        invalid_ix.data = host::instruction::FheExecute { args: invalid_args }.data();
        let invalid_result = svm.process_transaction_instructions(&[invalid_ix], &accounts);
        assert_eq!(
            invalid_result.program_result,
            TransactionProgramResult::Failure(
                0,
                ProgramError::Custom(6000 + host::ZamaHostError::InvalidReturnSelection as u32)
            )
        );
        assert_eq!(invalid_result.resulting_accounts, accounts);
    }

    let stale = svm.process_transaction_instructions(&[instruction], &result.resulting_accounts);
    assert_eq!(
        stale.program_result,
        TransactionProgramResult::Failure(
            0,
            ProgramError::Custom(6000 + host::ZamaHostError::PreviousStateMismatch as u32)
        )
    );
    assert_eq!(stale.resulting_accounts, result.resulting_accounts);
}

#[test]
fn maximum_result_grants_fit_one_execution_and_leave_no_account() {
    let fixture = Fixture::new(0);
    let (config, config_account) = zama_solana_test_kit::host_config_account(
        &zama_solana_test_kit::HostConfigParams::new(Pubkey::new_unique()),
    );
    let event_authority = zama_solana_test_kit::event_authority(host::ID);
    let mut accounts = fixture.accounts.clone();
    accounts.extend([
        (config, config_account),
        (event_authority, empty_system_account()),
    ]);
    let args = host::FheExecuteArgs {
        returned_results: Vec::new(),
        account_count: 2,
        dictionary: vec![],
        steps: (0..host::MAX_TRANSIENT_GRANTS)
            .map(|index| {
                let mut plaintext = [0; 32];
                plaintext[31] = index as u8;
                host::FheExecuteStep::TrivialEncrypt {
                    plaintext,
                    fhe_type: 5,
                    output: host::FheExecuteOutput::State {
                        state_index: 0,
                        previous_leaf_count: 0,
                        slot: None,
                        allow_indexes: vec![],
                        make_public: false,
                        grants: vec![host::ResultGrant {
                            scratch_index: 1,
                            initiating_state_index: 0,
                            consumer_state_index: 0,
                        }],
                    },
                }
            })
            .collect(),
    };
    let mut execute = anchor_ix(
        host::ID,
        host::accounts::FheExecute {
            payer: fixture.payer,
            authority: fixture.authority,
            host_config: config,
            system_program: anchor_lang::prelude::system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            rand_nonce: None,
            event_authority,
            program: host::ID,
        },
        host::instruction::FheExecute { args },
    );
    execute.accounts.extend([
        AccountMeta::new_readonly(fixture.state, false),
        AccountMeta::new(fixture.scratch, false),
    ]);
    let result = host_svm()
        .process_transaction_instructions(&[fixture.open(), execute, fixture.close()], &accounts);
    assert!(result.raw_result.is_ok(), "{:?}", result.raw_result);
    let scratch = &result
        .resulting_accounts
        .iter()
        .find(|(key, _)| *key == fixture.scratch)
        .unwrap()
        .1;
    assert!(scratch.data.is_empty());
    assert_eq!(scratch.lamports, 0);
    let state = &result
        .resulting_accounts
        .iter()
        .find(|(key, _)| *key == fixture.state)
        .unwrap()
        .1;
    let state = zama_solana_acl::decode_encrypted_state(&state.data).unwrap();
    assert_eq!(state.leaf_count, 0);
    assert!(state.peaks.is_empty());
    assert!(state.slots.is_empty());
}

#[derive(Clone, Copy)]
enum GrantConsumptionCase {
    Valid,
    UngrantedHandle,
    WrongConsumer,
    UnsignedConsumer,
}

fn grant_then_consume(case: GrantConsumptionCase) -> TransactionResult {
    let initiating = Fixture::new(0);
    let consumer_program = Pubkey::new_unique();
    let (consumer_authority, _) = Pubkey::find_program_address(&[b"consumer"], &consumer_program);
    let consumer_scope = [9; 32];
    let (consumer_state, consumer_bump) =
        host::encrypted_state_address(consumer_program, consumer_authority, consumer_scope);
    let (config, config_account) = zama_solana_test_kit::host_config_account(
        &zama_solana_test_kit::HostConfigParams::new(Pubkey::new_unique()),
    );
    let event_authority = zama_solana_test_kit::event_authority(host::ID);
    let mut accounts = initiating.accounts.clone();
    accounts.extend([
        (consumer_authority, empty_system_account()),
        (
            consumer_state,
            Account {
                lamports: 10_000_000,
                owner: host::ID,
                data: serialized_account(host::EncryptedState {
                    program: consumer_program,
                    authority: consumer_authority,
                    scope: consumer_scope,
                    slots: vec![],
                    leaf_count: 0,
                    peaks: vec![],
                    bump: consumer_bump,
                }),
                ..Account::default()
            },
        ),
        (config, config_account),
        (event_authority, empty_system_account()),
    ]);

    let svm = host_svm();
    let slot = svm.sysvars.clock.slot;
    let previous_bank_hash = svm
        .sysvars
        .slot_hashes
        .iter()
        .find(|(candidate, _)| *candidate < slot)
        .unwrap()
        .1
        .to_bytes();
    let handle_context = host::HandleDerivationContext {
        chain_id: host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp: svm.sysvars.clock.unix_timestamp,
    };
    let plaintext = [7; 32];
    let granted_handle = host::computed_eval_trivial_handle(plaintext, 5, &handle_context);
    let operand_handle = match case {
        GrantConsumptionCase::UngrantedHandle => {
            host::computed_eval_trivial_handle([8; 32], 5, &handle_context)
        }
        _ => granted_handle,
    };
    let operand_consumer_index = match case {
        GrantConsumptionCase::WrongConsumer => 0,
        _ => 2,
    };
    let args = host::FheExecuteArgs {
        returned_results: Vec::new(),
        account_count: if matches!(case, GrantConsumptionCase::WrongConsumer) {
            3
        } else {
            4
        },
        dictionary: vec![operand_handle, [0; 32]],
        steps: vec![
            host::FheExecuteStep::TrivialEncrypt {
                plaintext,
                fhe_type: 5,
                output: host::FheExecuteOutput::State {
                    state_index: 0,
                    previous_leaf_count: 0,
                    slot: None,
                    allow_indexes: vec![],
                    make_public: false,
                    grants: vec![host::ResultGrant {
                        initiating_state_index: 0,
                        consumer_state_index: 2,
                        scratch_index: 1,
                    }],
                },
            },
            host::FheExecuteStep::Binary {
                op: host::FheBinaryOpCode::Add,
                lhs: host::FheExecuteOperand::TransientResult {
                    consumer_state_index: operand_consumer_index,
                    scratch_index: 1,
                    handle_index: 0,
                },
                rhs: host::FheExecuteOperand::Scalar { value_index: 1 },
                output_fhe_type: 5,
                output: host::FheExecuteOutput::Transient,
            },
        ],
    };
    let mut execute = anchor_ix(
        host::ID,
        host::accounts::FheExecute {
            payer: initiating.payer,
            authority: initiating.authority,
            host_config: config,
            system_program: anchor_lang::prelude::system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            rand_nonce: None,
            event_authority,
            program: host::ID,
        },
        host::instruction::FheExecute { args },
    );
    execute.accounts.extend([
        AccountMeta::new_readonly(initiating.state, false),
        AccountMeta::new(initiating.scratch, false),
        AccountMeta::new_readonly(consumer_state, false),
    ]);
    if !matches!(case, GrantConsumptionCase::WrongConsumer) {
        execute.accounts.push(AccountMeta::new_readonly(
            consumer_authority,
            !matches!(case, GrantConsumptionCase::UnsignedConsumer),
        ));
    }
    svm.process_transaction_instructions(
        &[initiating.open(), execute, initiating.close()],
        &accounts,
    )
}

#[test]
fn transient_result_accepts_the_exact_granted_handle_and_consumer_state() {
    let valid = grant_then_consume(GrantConsumptionCase::Valid);
    assert!(valid.raw_result.is_ok(), "{:?}", valid.raw_result);
}

#[test]
fn transient_result_rejects_an_ungranted_handle() {
    assert_eq!(
        grant_then_consume(GrantConsumptionCase::UngrantedHandle).program_result,
        TransactionProgramResult::Failure(
            1,
            ProgramError::Custom(6000 + host::ZamaHostError::TransientAccountInvalid as u32)
        )
    );
}

#[test]
fn transient_result_rejects_the_wrong_consumer_state() {
    assert_eq!(
        grant_then_consume(GrantConsumptionCase::WrongConsumer).program_result,
        TransactionProgramResult::Failure(
            1,
            ProgramError::Custom(6000 + host::ZamaHostError::TransientAccountInvalid as u32)
        )
    );
}

#[test]
fn transient_result_requires_the_consumer_state_authority_signature() {
    assert_eq!(
        grant_then_consume(GrantConsumptionCase::UnsignedConsumer).program_result,
        TransactionProgramResult::Failure(
            1,
            ProgramError::Custom(
                6000 + host::ZamaHostError::EncryptedStateAccountAuthorityMismatch as u32
            )
        )
    );
}

#[test]
fn result_grant_rejects_a_scratch_opened_for_another_initiating_state() {
    let initiating = Fixture::new(0);
    let other = Fixture::new(0);
    let (config, config_account) = zama_solana_test_kit::host_config_account(
        &zama_solana_test_kit::HostConfigParams::new(Pubkey::new_unique()),
    );
    let event_authority = zama_solana_test_kit::event_authority(host::ID);
    let mut accounts = initiating.accounts.clone();
    let other_accounts: Vec<_> = other
        .accounts
        .iter()
        .filter(|(key, _)| !accounts.iter().any(|(existing, _)| existing == key))
        .cloned()
        .collect();
    accounts.extend(other_accounts);
    accounts.extend([
        (config, config_account),
        (event_authority, empty_system_account()),
    ]);
    let args = host::FheExecuteArgs {
        returned_results: Vec::new(),
        account_count: 2,
        dictionary: vec![],
        steps: vec![host::FheExecuteStep::TrivialEncrypt {
            plaintext: [7; 32],
            fhe_type: 5,
            output: host::FheExecuteOutput::State {
                state_index: 0,
                previous_leaf_count: 0,
                slot: None,
                allow_indexes: vec![],
                make_public: false,
                grants: vec![host::ResultGrant {
                    initiating_state_index: 0,
                    consumer_state_index: 0,
                    scratch_index: 1,
                }],
            },
        }],
    };
    let mut execute = anchor_ix(
        host::ID,
        host::accounts::FheExecute {
            payer: initiating.payer,
            authority: initiating.authority,
            host_config: config,
            system_program: anchor_lang::prelude::system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            rand_nonce: None,
            event_authority,
            program: host::ID,
        },
        host::instruction::FheExecute { args },
    );
    execute.accounts.extend([
        AccountMeta::new_readonly(initiating.state, false),
        AccountMeta::new(other.scratch, false),
    ]);
    let result = host_svm()
        .process_transaction_instructions(&[other.open(), execute, other.close()], &accounts);
    assert_eq!(
        result.program_result,
        TransactionProgramResult::Failure(
            1,
            ProgramError::Custom(6000 + host::ZamaHostError::TransientAccountInvalid as u32)
        )
    );
}
