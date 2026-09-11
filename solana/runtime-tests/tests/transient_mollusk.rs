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
    transient_store: Pubkey,
    accounts: Vec<(Pubkey, Account)>,
}

impl Fixture {
    fn new(prefund: u64) -> Self {
        let payer = Pubkey::new_unique();
        let program = Pubkey::new_unique();
        let (authority, _) = Pubkey::find_program_address(&[b"authority"], &program);
        let scope = [3; 32];
        let (state, bump) = host::encrypted_store_address(program, authority, scope);
        let (transient_store, _) = host::transient_store_address(payer);
        let mut transient_store_account = empty_system_account();
        transient_store_account.lamports = prefund;
        Self {
            payer,
            authority,
            state,
            transient_store,
            accounts: vec![
                (payer, funded_system_account()),
                (authority, empty_system_account()),
                (
                    state,
                    Account {
                        lamports: 10_000_000,
                        owner: host::ID,
                        data: serialized_account(host::EncryptedStore {
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
                (transient_store, transient_store_account),
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
            host::accounts::OpenTransientStore {
                payer: self.payer,
                transient_store: self.transient_store,
                instructions: Instructions::id(),
                system_program: anchor_lang::prelude::system_program::ID,
            },
            host::instruction::OpenTransientStore {},
        )
    }

    fn close(&self) -> Instruction {
        Instruction {
            program_id: host::ID,
            data: host::instruction::CloseTransientStore {}.data(),
            accounts: vec![
                AccountMeta::new_readonly(Instructions::id(), false),
                AccountMeta::new(self.transient_store, false),
                AccountMeta::new(self.payer, false),
            ],
        }
    }
}

#[test]
fn transient_store_is_created_and_closed_atomically_including_prefunded_addresses() {
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
        assert_eq!(account(fixture.transient_store).lamports, 0);
        assert!(account(fixture.transient_store).data.is_empty());
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
    open.accounts[0].is_signer = false;
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
fn transient_store_cannot_close_before_the_final_instruction() {
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
fn nested_transient_store_close_rolls_back_the_whole_transaction() {
    let fixture = Fixture::new(0);
    let mut svm = host_svm();
    svm.add_program(&delegator_vault::ID, "delegator_vault");
    let nested_close = anchor_ix(
        delegator_vault::ID,
        delegator_vault::accounts::CloseTransientStoreViaCpi {
            transient_store: fixture.transient_store,
            refund: fixture.payer,
            instructions: Instructions::id(),
            zama_host: host::ID,
        },
        delegator_vault::instruction::CloseTransientStoreViaCpi {},
    );
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
fn one_transaction_cannot_open_two_transient_store_accounts() {
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
    let result = host_svm()
        .process_transaction_instructions(&[first.open(), second.open(), first.close()], &accounts);
    assert_eq!(
        result.program_result,
        TransactionProgramResult::Failure(
            1,
            ProgramError::Custom(6000 + host::ZamaHostError::TransientCloseMissing as u32)
        )
    );
    assert_eq!(result.resulting_accounts, accounts);
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
    let output = |key_index| host::FheExecuteEffect {
        result: host::ExecutionResultRef {
            step_index: key_index,
            output_index: 0,
        },
        store_index: 0,
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
        execution_store_index: 0,
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
        effects: vec![output(0), output(1)],
        steps: vec![
            host::FheExecuteStep::TrivialEncrypt {
                plaintext: one,
                fhe_type: 5,
            },
            host::FheExecuteStep::TrivialEncrypt {
                plaintext: two,
                fhe_type: 5,
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
            transient_store: fixture.transient_store,
            instructions: Instructions::id(),
            event_authority,
            program: host::ID,
        },
        host::instruction::FheExecute { args: args.clone() },
    );
    instruction
        .accounts
        .push(AccountMeta::new(fixture.state, false));
    let mut svm = host_svm();
    svm.add_program(&delegator_vault::ID, "delegator_vault");
    let result = svm.process_transaction_instructions(
        &[fixture.open(), instruction.clone(), fixture.close()],
        &accounts,
    );
    assert!(result.raw_result.is_ok(), "{:?}", result.raw_result);
    let state_account = &result
        .resulting_accounts
        .iter()
        .find(|(key, _)| *key == fixture.state)
        .unwrap()
        .1;
    let state = zama_solana_acl::decode_encrypted_store(&state_account.data).unwrap();
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
            args: selected_args.clone(),
        }
        .data();
        let expected: Vec<u8> = selected.iter().flat_map(|&i| handles[i as usize]).collect();
        let mut probe = anchor_ix(
            delegator_vault::ID,
            delegator_vault::accounts::CheckCpiReturn { callee: host::ID },
            delegator_vault::instruction::CheckCpiReturn {
                instruction_data: selected_ix.data.clone(),
                expected,
            },
        );
        probe.accounts.extend(selected_ix.accounts);
        let selected_result = svm
            .process_transaction_instructions(&[fixture.open(), probe, fixture.close()], &accounts);
        assert!(
            selected_result.raw_result.is_ok(),
            "{:?}",
            selected_result.raw_result
        );

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
        let invalid_result = svm.process_transaction_instructions(
            &[fixture.open(), invalid_ix, fixture.close()],
            &accounts,
        );
        assert_eq!(
            invalid_result.program_result,
            TransactionProgramResult::Failure(
                1,
                ProgramError::Custom(6000 + host::ZamaHostError::InvalidReturnSelection as u32)
            )
        );
        assert_eq!(invalid_result.resulting_accounts, accounts);
    }

    let stale = svm.process_transaction_instructions(
        &[fixture.open(), instruction, fixture.close()],
        &result.resulting_accounts,
    );
    assert_eq!(
        stale.program_result,
        TransactionProgramResult::Failure(
            1,
            ProgramError::Custom(6000 + host::ZamaHostError::PreviousStoreMismatch as u32)
        )
    );
    assert_eq!(stale.resulting_accounts, result.resulting_accounts);
}

#[test]
fn maximum_result_grants_fit_one_execution_and_leave_no_account() {
    let fixture = Fixture::new(0);
    let consumer = Fixture::new(0);
    let (config, config_account) = zama_solana_test_kit::host_config_account(
        &zama_solana_test_kit::HostConfigParams::new(Pubkey::new_unique()),
    );
    let mut accounts = fixture.accounts.clone();
    accounts.push(
        consumer
            .accounts
            .iter()
            .find(|(key, _)| *key == consumer.state)
            .unwrap()
            .clone(),
    );
    accounts.extend([
        (config, config_account),
        (
            zama_solana_test_kit::event_authority(host::ID),
            empty_system_account(),
        ),
    ]);
    let args = host::FheExecuteArgs {
        execution_store_index: 0,
        returned_results: vec![],
        account_count: 2,
        dictionary: vec![],
        steps: (0..host::MAX_TRANSIENT_GRANTS)
            .map(|index| {
                let mut plaintext = [0; 32];
                plaintext[31] = index as u8;
                host::FheExecuteStep::TrivialEncrypt {
                    plaintext,
                    fhe_type: 5,
                }
            })
            .collect(),
        effects: (0..host::MAX_TRANSIENT_GRANTS)
            .map(|index| host::FheExecuteEffect {
                result: host::ExecutionResultRef {
                    step_index: index as u8,
                    output_index: 0,
                },
                store_index: 0,
                previous_leaf_count: 0,
                slot: None,
                allow_indexes: vec![],
                make_public: false,
                grants: vec![host::ResultGrant {
                    consumer_store_index: 1,
                }],
            })
            .collect(),
    };
    let first = execute(
        &fixture,
        fixture.authority,
        config,
        args.clone(),
        vec![
            AccountMeta::new_readonly(fixture.state, false),
            AccountMeta::new_readonly(consumer.state, false),
        ],
    );
    let result = host_svm().process_transaction_instructions(
        &[fixture.open(), first.clone(), fixture.close()],
        &accounts,
    );
    assert!(result.raw_result.is_ok(), "{:?}", result.raw_result);
    let transient_store = &result
        .resulting_accounts
        .iter()
        .find(|(key, _)| *key == fixture.transient_store)
        .unwrap()
        .1;
    assert!(transient_store.data.is_empty());
    assert_eq!(transient_store.lamports, 0);
    let state = &result
        .resulting_accounts
        .iter()
        .find(|(key, _)| *key == fixture.state)
        .unwrap()
        .1;
    let state = zama_solana_acl::decode_encrypted_store(&state.data).unwrap();
    assert_eq!(state.leaf_count, 0);
    assert!(state.peaks.is_empty());
    assert!(state.slots.is_empty());
    // Repeating a grant in a later call does not use another entry. A new handle does.
    for (plaintext, succeeds) in [(0, true), (host::MAX_TRANSIENT_GRANTS as u8, false)] {
        let mut next = args.clone();
        let mut value = [0; 32];
        value[31] = plaintext;
        next.steps = vec![host::FheExecuteStep::TrivialEncrypt {
            plaintext: value,
            fhe_type: 5,
        }];
        next.effects.truncate(1);
        let second = execute(
            &fixture,
            fixture.authority,
            config,
            next,
            vec![
                AccountMeta::new_readonly(fixture.state, false),
                AccountMeta::new_readonly(consumer.state, false),
            ],
        );
        let next_result = host_svm().process_transaction_instructions(
            &[fixture.open(), first.clone(), second, fixture.close()],
            &accounts,
        );
        if succeeds {
            assert!(
                next_result.raw_result.is_ok(),
                "{:?}",
                next_result.raw_result
            );
        } else {
            assert_eq!(
                next_result.program_result,
                TransactionProgramResult::Failure(
                    2,
                    ProgramError::Custom(
                        6000 + host::ZamaHostError::TransientCapacityExceeded as u32
                    ),
                )
            );
            assert_eq!(next_result.resulting_accounts, accounts);
        }
    }
}

fn execute(
    fixture: &Fixture,
    authority: Pubkey,
    config: Pubkey,
    args: host::FheExecuteArgs,
    remaining: Vec<AccountMeta>,
) -> Instruction {
    let mut ix = anchor_ix(
        host::ID,
        host::accounts::FheExecute {
            payer: fixture.payer,
            authority,
            host_config: config,
            system_program: anchor_lang::prelude::system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            rand_nonce: None,
            transient_store: fixture.transient_store,
            instructions: Instructions::id(),
            event_authority: zama_solana_test_kit::event_authority(host::ID),
            program: host::ID,
        },
        host::instruction::FheExecute { args },
    );
    ix.accounts.extend(remaining);
    ix
}

#[derive(Clone, Copy)]
enum GrantConsumptionCase {
    Valid,
    UngrantedHandle,
    WrongConsumer,
    WrongTransientStore,
    UnsignedConsumer,
    NoGrant,
    TotalLimit,
    DepthLimit,
    BlockMeter,
    DenyEnabled,
    MissingConsumerDeny,
}

fn grant_then_consume(case: GrantConsumptionCase) -> TransactionResult {
    let producer = Fixture::new(0);
    let consumer = Fixture::new(0);
    let (config, mut config_account) = zama_solana_test_kit::host_config_account(
        &zama_solana_test_kit::HostConfigParams::new(Pubkey::new_unique()),
    );
    let mut settings =
        host::HostConfig::try_deserialize(&mut config_account.data.as_slice()).unwrap();
    match case {
        GrantConsumptionCase::TotalLimit => settings.max_hcu_per_tx = 133_031,
        GrantConsumptionCase::DepthLimit => settings.max_hcu_depth_per_tx = 133_031,
        GrantConsumptionCase::BlockMeter => settings.hcu_block_cap_per_app = 133_000,
        GrantConsumptionCase::DenyEnabled | GrantConsumptionCase::MissingConsumerDeny => {
            settings.grant_deny_list_enabled = true
        }
        _ => {}
    }
    config_account.data = serialized_account(settings);
    let mut accounts = producer.accounts.clone();
    accounts.extend(
        consumer
            .accounts
            .iter()
            .filter(|(key, _)| *key == consumer.state || *key == consumer.authority)
            .cloned(),
    );
    accounts.extend([
        (config, config_account),
        (
            zama_solana_test_kit::event_authority(host::ID),
            empty_system_account(),
        ),
    ]);
    let svm = host_svm();
    let context = host::HandleDerivationContext {
        chain_id: host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash: svm
            .sysvars
            .slot_hashes
            .iter()
            .find(|(slot, _)| *slot < svm.sysvars.clock.slot)
            .unwrap()
            .1
            .to_bytes(),
        unix_timestamp: svm.sysvars.clock.unix_timestamp,
    };
    let handle = host::computed_eval_trivial_handle([7; 32], 5, &context);
    let grant_to = if matches!(case, GrantConsumptionCase::WrongConsumer) {
        producer.state
    } else {
        consumer.state
    };
    let has_grant = !matches!(case, GrantConsumptionCase::NoGrant);
    let grant_is_foreign = has_grant && grant_to != producer.state;
    let mut produce_args = host::FheExecuteArgs {
        execution_store_index: 0,
        account_count: if grant_is_foreign { 2 } else { 1 },
        dictionary: vec![],
        steps: vec![host::FheExecuteStep::TrivialEncrypt {
            plaintext: [7; 32],
            fhe_type: 5,
        }],
        effects: if has_grant {
            vec![host::FheExecuteEffect {
                result: host::ExecutionResultRef {
                    step_index: 0,
                    output_index: 0,
                },
                store_index: 0,
                previous_leaf_count: 0,
                slot: None,
                allow_indexes: vec![],
                make_public: false,
                grants: vec![host::ResultGrant {
                    consumer_store_index: if grant_is_foreign { 1 } else { 0 },
                }],
            }]
        } else {
            vec![]
        },
        returned_results: vec![],
    };
    let app = |fixture: &Fixture| {
        let state_account = &fixture
            .accounts
            .iter()
            .find(|(key, _)| *key == fixture.state)
            .unwrap()
            .1;
        let state =
            host::EncryptedStore::try_deserialize(&mut state_account.data.as_slice()).unwrap();
        host::AppScope {
            program: state.program,
            scope: state.scope,
        }
    };
    let deny_enabled = matches!(
        case,
        GrantConsumptionCase::DenyEnabled | GrantConsumptionCase::MissingConsumerDeny
    );
    let producer_deny = zama_solana_test_kit::deny_scope_record_account(app(&producer), false);
    let consumer_deny = zama_solana_test_kit::deny_scope_record_account(app(&consumer), false);
    if deny_enabled {
        produce_args.account_count += 1;
    }
    let mut producer_accounts = vec![AccountMeta::new_readonly(producer.state, false)];
    if grant_is_foreign {
        producer_accounts.push(AccountMeta::new_readonly(consumer.state, false));
    }
    if deny_enabled {
        producer_accounts.push(AccountMeta::new_readonly(producer_deny.0, false));
        accounts.push(producer_deny);
    }
    let mut produce = execute(
        &producer,
        producer.authority,
        config,
        produce_args,
        producer_accounts,
    );
    let mut consume_args = host::FheExecuteArgs {
        execution_store_index: 0,
        account_count: 1,
        dictionary: vec![
            if matches!(case, GrantConsumptionCase::UngrantedHandle) {
                host::computed_eval_trivial_handle([8; 32], 5, &context)
            } else {
                handle
            },
            [0; 32],
        ],
        steps: vec![host::FheExecuteStep::Binary {
            op: host::FheBinaryOpCode::Add,
            lhs: host::FheExecuteOperand::TransientResult {
                handle_index: 0,
                consumer_store_index: 0,
            },
            rhs: host::FheExecuteOperand::Scalar { value_index: 1 },
            output_fhe_type: 5,
        }],
        effects: vec![],
        returned_results: vec![],
    };
    let mut consumer_accounts = vec![AccountMeta::new_readonly(consumer.state, false)];
    if matches!(case, GrantConsumptionCase::DenyEnabled) {
        consume_args.account_count += 1;
        consumer_accounts.push(AccountMeta::new_readonly(consumer_deny.0, false));
        accounts.push(consumer_deny);
    }
    let mut consume = execute(
        &producer,
        consumer.authority,
        config,
        consume_args,
        consumer_accounts,
    );
    if matches!(case, GrantConsumptionCase::BlockMeter) {
        for (ix, app) in [
            (&mut produce, app(&producer)),
            (&mut consume, app(&consumer)),
        ] {
            let meter = host::hcu_block_meter_address(app).0;
            ix.accounts[4] = AccountMeta::new(meter, false);
            accounts.push((meter, empty_system_account()));
        }
    }
    if matches!(case, GrantConsumptionCase::WrongTransientStore) {
        consume.accounts[7].pubkey = consumer.transient_store;
        accounts.push((consumer.transient_store, empty_system_account()));
    }
    if matches!(case, GrantConsumptionCase::UnsignedConsumer) {
        consume.accounts[1].is_signer = false;
    }
    svm.process_transaction_instructions(
        &[producer.open(), produce, consume, producer.close()],
        &accounts,
    )
}

#[test]
fn transient_result_accepts_the_exact_granted_handle_and_consumer_store() {
    let valid = grant_then_consume(GrantConsumptionCase::Valid);
    assert!(valid.raw_result.is_ok(), "{:?}", valid.raw_result);
}

#[test]
fn transient_result_rejects_missing_grants_and_wrong_handle_or_consumer() {
    for case in [
        GrantConsumptionCase::UngrantedHandle,
        GrantConsumptionCase::WrongConsumer,
        GrantConsumptionCase::NoGrant,
    ] {
        assert_eq!(
            grant_then_consume(case).program_result,
            TransactionProgramResult::Failure(
                2,
                ProgramError::Custom(6000 + host::ZamaHostError::TransientAccountInvalid as u32)
            )
        );
    }
}

#[test]
fn execution_cannot_substitute_an_unopened_transient_store_account() {
    assert_eq!(
        grant_then_consume(GrantConsumptionCase::WrongTransientStore).program_result,
        TransactionProgramResult::Failure(
            2,
            ProgramError::Custom(anchor_lang::error::ErrorCode::AccountOwnedByWrongProgram as u32)
        )
    );
}

#[test]
fn transient_result_requires_the_consumer_store_authority_signature() {
    assert_eq!(
        grant_then_consume(GrantConsumptionCase::UnsignedConsumer).program_result,
        TransactionProgramResult::Failure(
            2,
            ProgramError::Custom(anchor_lang::error::ErrorCode::AccountNotSigner as u32)
        )
    );
}

#[test]
fn cross_application_transaction_limits_accumulate_and_block_meters_remain_per_app() {
    for (case, error) in [
        (
            GrantConsumptionCase::TotalLimit,
            host::ZamaHostError::HcuTransactionLimitExceeded,
        ),
        (
            GrantConsumptionCase::DepthLimit,
            host::ZamaHostError::HcuTransactionDepthLimitExceeded,
        ),
    ] {
        assert_eq!(
            grant_then_consume(case).program_result,
            TransactionProgramResult::Failure(2, ProgramError::Custom(6000 + error as u32))
        );
    }
    let result = grant_then_consume(GrantConsumptionCase::BlockMeter);
    assert!(result.raw_result.is_ok(), "{:?}", result.raw_result);
    let mut used: Vec<_> = result
        .resulting_accounts
        .iter()
        .filter_map(|(_, account)| {
            host::HcuBlockMeter::try_deserialize(&mut account.data.as_slice()).ok()
        })
        .map(|meter| meter.used_hcu)
        .collect();
    used.sort_unstable();
    assert_eq!(used, [32, 133_000]);
}

#[test]
fn grant_creation_checks_producer_scope_and_consumption_checks_consumer_scope() {
    let result = grant_then_consume(GrantConsumptionCase::DenyEnabled);
    assert!(result.raw_result.is_ok(), "{:?}", result.raw_result);
    assert_eq!(
        grant_then_consume(GrantConsumptionCase::MissingConsumerDeny).program_result,
        TransactionProgramResult::Failure(
            2,
            ProgramError::Custom(6000 + host::ZamaHostError::DenyRecordMissing as u32)
        )
    );
}

/// SBF account-capacity check. Application CPI packet and outer transaction wire
/// limits are measured separately by the boundary suite.
#[test]
fn result_journal_capacity_is_shared_across_calls_and_fails_atomically() {
    for count in [host::MAX_TRANSIENT_RESULTS, host::MAX_TRANSIENT_RESULTS + 1] {
        let fixture = Fixture::new(0);
        let (config, config_account) = zama_solana_test_kit::host_config_account(
            &zama_solana_test_kit::HostConfigParams::new(Pubkey::new_unique()),
        );
        let mut accounts = fixture.accounts.clone();
        accounts.extend([
            (config, config_account),
            (
                zama_solana_test_kit::event_authority(host::ID),
                empty_system_account(),
            ),
        ]);
        let mut instructions = vec![fixture.open()];
        for start in (0..count).step_by(host::MAX_FHE_EXECUTION_STEPS) {
            let args = host::FheExecuteArgs {
                execution_store_index: 0,
                account_count: 1,
                dictionary: vec![],
                returned_results: vec![],
                effects: vec![],
                // Repeated handles must consume occurrence capacity too.
                steps: vec![
                    host::FheExecuteStep::TrivialEncrypt {
                        plaintext: [7; 32],
                        fhe_type: 5
                    };
                    (count - start).min(host::MAX_FHE_EXECUTION_STEPS)
                ],
            };
            instructions.push(execute(
                &fixture,
                fixture.authority,
                config,
                args,
                vec![AccountMeta::new_readonly(fixture.state, false)],
            ));
        }
        instructions.push(fixture.close());
        let result = host_svm().process_transaction_instructions(&instructions, &accounts);
        if count == host::MAX_TRANSIENT_RESULTS {
            assert!(result.raw_result.is_ok(), "{:?}", result.raw_result);
            println!(
                "{count} journal occurrences: {} CU including open/close",
                result.compute_units_consumed
            );
            assert!(result
                .get_account(&fixture.transient_store)
                .unwrap()
                .data
                .is_empty());
        } else {
            assert_eq!(
                result.program_result,
                TransactionProgramResult::Failure(
                    4,
                    ProgramError::Custom(
                        6000 + host::ZamaHostError::TransientCapacityExceeded as u32
                    )
                )
            );
            assert_eq!(result.resulting_accounts, accounts);
        }
    }
}

#[test]
fn producer_reuses_its_result_across_calls_with_transaction_origin_and_depth() {
    for reload_slot in [false, true] {
        for max_depth in [133_031, 133_032] {
            let fixture = Fixture::new(0);
            let (config, mut config_account) = zama_solana_test_kit::host_config_account(
                &zama_solana_test_kit::HostConfigParams::new(fixture.payer),
            );
            let mut settings =
                host::HostConfig::try_deserialize(&mut config_account.data.as_slice()).unwrap();
            settings.max_hcu_depth_per_tx = max_depth;
            config_account.data = serialized_account(settings);
            let svm = host_svm();
            let context = host::HandleDerivationContext {
                chain_id: host::SOLANA_POC_CHAIN_ID,
                previous_bank_hash: svm
                    .sysvars
                    .slot_hashes
                    .iter()
                    .find(|(slot, _)| *slot < svm.sysvars.clock.slot)
                    .unwrap()
                    .1
                    .to_bytes(),
                unix_timestamp: svm.sysvars.clock.unix_timestamp,
            };
            let first = host::computed_eval_trivial_handle([7; 32], 5, &context);
            let effect = |key_index| host::FheExecuteEffect {
                result: host::ExecutionResultRef {
                    step_index: 0,
                    output_index: 0,
                },
                store_index: 0,
                previous_leaf_count: 0,
                slot: Some(host::SlotWrite {
                    key_index,
                    previous_handle_index: None,
                }),
                allow_indexes: vec![],
                make_public: false,
                grants: vec![],
            };
            let produce_args = host::FheExecuteArgs {
                execution_store_index: 0,
                account_count: 1,
                dictionary: if reload_slot { vec![[1; 32]] } else { vec![] },
                steps: vec![host::FheExecuteStep::TrivialEncrypt {
                    plaintext: [7; 32],
                    fhe_type: 5,
                }],
                effects: if reload_slot { vec![effect(0)] } else { vec![] },
                returned_results: vec![],
            };
            let consume_args = host::FheExecuteArgs {
                execution_store_index: 0,
                account_count: 1,
                dictionary: if reload_slot {
                    vec![first, [0; 32], [2; 32], [1; 32]]
                } else {
                    vec![first, [0; 32], [2; 32]]
                },
                steps: vec![host::FheExecuteStep::Binary {
                    op: host::FheBinaryOpCode::Add,
                    lhs: if reload_slot {
                        host::FheExecuteOperand::StoreSlot {
                            handle_index: 0,
                            store_index: 0,
                            key_index: 3,
                        }
                    } else {
                        host::FheExecuteOperand::TransientResult {
                            handle_index: 0,
                            consumer_store_index: 0,
                        }
                    },
                    rhs: host::FheExecuteOperand::Scalar { value_index: 1 },
                    output_fhe_type: 5,
                }],
                effects: vec![effect(2)],
                returned_results: vec![],
            };
            let state_meta = vec![AccountMeta::new(fixture.state, false)];
            let produce = execute(
                &fixture,
                fixture.authority,
                config,
                produce_args,
                state_meta.clone(),
            );
            let consume = execute(
                &fixture,
                fixture.authority,
                config,
                consume_args.clone(),
                state_meta.clone(),
            );
            let mut accounts = fixture.accounts.clone();
            accounts.extend([
                (config, config_account),
                (
                    zama_solana_test_kit::event_authority(host::ID),
                    empty_system_account(),
                ),
            ]);
            let result = svm.process_transaction_instructions(
                &[fixture.open(), produce, consume, fixture.close()],
                &accounts,
            );
            if max_depth == 133_031 {
                assert_eq!(
                    result.program_result,
                    TransactionProgramResult::Failure(
                        2,
                        ProgramError::Custom(
                            6000 + host::ZamaHostError::HcuTransactionDepthLimitExceeded as u32
                        )
                    )
                );
                continue;
            }
            assert!(result.raw_result.is_ok(), "{:?}", result.raw_result);
            let read = |result: &TransactionResult| {
                host::EncryptedStore::try_deserialize(
                    &mut result.get_account(&fixture.state).unwrap().data.as_slice(),
                )
                .unwrap()
            };
            let expected = host::computed_eval_handle(
                host::FheBinaryOpCode::Add,
                first,
                [0; 32],
                true,
                5,
                [0; 32],
                &context,
            );
            assert_eq!(read(&result).get(&[2; 32]), Some(expected));
            if reload_slot {
                // Same block entropy, new transaction: reading the persisted first result is boundary input.
                let mut next_args = consume_args;
                next_args.dictionary.push(expected);
                next_args.effects[0]
                    .slot
                    .as_mut()
                    .unwrap()
                    .previous_handle_index = Some(4);
                let next = execute(&fixture, fixture.authority, config, next_args, state_meta);
                let next_result = svm.process_transaction_instructions(
                    &[fixture.open(), next, fixture.close()],
                    &result.resulting_accounts,
                );
                assert!(
                    next_result.raw_result.is_ok(),
                    "{:?}",
                    next_result.raw_result
                );
                let expected = host::computed_eval_handle(
                    host::FheBinaryOpCode::Add,
                    first,
                    [0; 32],
                    true,
                    5,
                    host::operand_boundary_mask([true, false]).unwrap(),
                    &context,
                );
                assert_eq!(read(&next_result).get(&[2; 32]), Some(expected));
            }
        }
    }
}

#[test]
fn oracle_recovers_unstored_random_results_from_their_own_cpi_seed_event() {
    let fixture = Fixture::new(0);
    let (config, config_account) = zama_solana_test_kit::host_config_account(
        &zama_solana_test_kit::HostConfigParams::new(fixture.payer),
    );
    let (nonce, nonce_account) = zama_solana_test_kit::rand_nonce_account(7);
    let args = host::FheExecuteArgs {
        execution_store_index: 0,
        account_count: 1,
        dictionary: vec![],
        steps: vec![
            host::FheExecuteStep::Rand { fhe_type: 5 },
            host::FheExecuteStep::RandBounded {
                upper_bound: zama_solana_test_kit::u256_be(16),
                fhe_type: 5,
            },
        ],
        effects: vec![],
        returned_results: vec![],
    };
    let mut instruction = execute(
        &fixture,
        fixture.authority,
        config,
        args,
        vec![AccountMeta::new_readonly(fixture.state, false)],
    );
    instruction.accounts[6] = AccountMeta::new(nonce, false);
    let mut probe = anchor_ix(
        delegator_vault::ID,
        delegator_vault::accounts::CheckCpiReturn { callee: host::ID },
        delegator_vault::instruction::CheckCpiReturn {
            instruction_data: instruction.data,
            expected: vec![],
        },
    );
    probe.accounts.extend(instruction.accounts);
    let mut svm = host_svm();
    svm.add_program(&delegator_vault::ID, "delegator_vault");
    let mut accounts: std::collections::HashMap<_, _> = fixture.accounts.into_iter().collect();
    accounts.extend([
        (config, config_account),
        (nonce, nonce_account),
        (
            zama_solana_test_kit::event_authority(host::ID),
            empty_system_account(),
        ),
    ]);
    let context = svm.with_context(accounts);
    let result = zama_solana_test_kit::transaction::process_fhe_instruction(
        &context,
        fixture.payer,
        &probe,
        &[mollusk_svm::result::Check::success()],
    );
    let event: host::FheExecuteRandomSeedsEvent = result
        .inner_instructions
        .iter()
        .find_map(|inner| zama_solana_test_kit::decode_anchor_event(&inner.instruction.data))
        .unwrap();
    assert_eq!(event.seeds.len(), 2);
    let mut ledger = zama_solana_test_kit::oracle::CleartextLedger::default();
    let replay = ledger.replay_fhe_cpis(&context, &result);
    assert_eq!(replay.executions, 1);
    assert_eq!(replay.persistent_outputs, 0);
    let first = host::computed_rand_handle(event.seeds[0].seed, 5, host::SOLANA_POC_CHAIN_ID);
    let second = host::computed_rand_bounded_handle(
        zama_solana_test_kit::u256_be(16),
        event.seeds[1].seed,
        5,
        host::SOLANA_POC_CHAIN_ID,
    );
    ledger.u64_for_handle(first);
    assert!(ledger.u64_for_handle(second) < 16);
    assert!(
        zama_solana_test_kit::read_encrypted_store(&context, fixture.state)
            .slots
            .is_empty()
    );
}

/// Cross-pinned by sdk/js-sdk/src/solana/fheTransaction.test.ts.
#[test]
fn sdk_transient_store_fixture_matches_host_address_and_lifecycle_bytes() {
    let payer = Pubkey::new_from_array([0x44; 32]);
    assert_eq!(
        host::transient_store_address(payer).0.to_string(),
        "7HVhfpvm7TiBwHw8vFNeEqkMCDTU2cWpruEweWsRziAW"
    );
    assert_eq!(
        host::instruction::OpenTransientStore {}.data(),
        [54, 100, 76, 213, 84, 233, 196, 94]
    );
    assert_eq!(
        host::instruction::CloseTransientStore {}.data(),
        [107, 197, 28, 166, 51, 173, 83, 189]
    );
}
