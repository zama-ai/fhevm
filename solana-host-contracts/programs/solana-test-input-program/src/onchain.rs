use borsh::{BorshDeserialize, BorshSerialize};
use solana_host_contracts_core::{
    find_session_pda as find_host_session_pda, find_state_pda as find_host_state_pda,
    BinaryOperand, ContextUserInputs, FheType, Handle, HostInstruction, InstructionResult as HostInstructionResult,
    OnchainInstruction as HostOnchainInstruction, Pubkey as HostPubkey,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{get_return_data, invoke, invoke_signed, set_return_data},
    program_error::ProgramError,
    pubkey::Pubkey as SolanaPubkey,
    rent::Rent,
    sysvar,
    sysvar::SysvarSerialize,
};
use solana_system_interface::{instruction as system_instruction, program as system_program};
use solana_test_input_core::{
    find_state_pda, TestInputExecutionResult, TestInputInstruction, TestInputState,
    TEST_INPUT_STATE_PDA_SEED,
};

const STATE_ACCOUNT_DISCRIMINATOR: [u8; 8] = *b"TINPUT00";
const STATE_ACCOUNT_LAYOUT_VERSION: u32 = 1;
const MIXED_PUBLIC_ADDRESS: [u8; 20] = [
    0xfc, 0x43, 0x82, 0xc0, 0x84, 0xfc, 0xa3, 0xf4, 0xfb, 0x07, 0xc3, 0xbc, 0xda, 0x90, 0x6c, 0x01,
    0x79, 0x75, 0x95, 0xa8,
];
const USER_DECRYPT_ADDRESS: [u8; 20] = [
    0x8b, 0xa1, 0xf1, 0x09, 0x55, 0x1b, 0xd4, 0x32, 0x80, 0x30, 0x12, 0x64, 0x5a, 0xc1, 0x36, 0xdd,
    0xd6, 0x4d, 0xba, 0x72,
];
const USER_DECRYPT_UINT256: [u8; 32] = [
    0xa4, 0x3c, 0x19, 0xc9, 0xc1, 0x9f, 0xe2, 0x13, 0x5e, 0x77, 0x13, 0x3e, 0x55, 0x17, 0x4f, 0xcb,
    0x10, 0x05, 0x21, 0x11, 0x6f, 0xca, 0xd4, 0xe3, 0xa2, 0xe9, 0x77, 0xa6, 0x6c, 0x41, 0xff, 0x11,
];

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
struct StoredTestInputState {
    discriminator: [u8; 8],
    layout_version: u32,
    state: TestInputState,
}

impl StoredTestInputState {
    fn new(state: TestInputState) -> Self {
        Self {
            discriminator: STATE_ACCOUNT_DISCRIMINATOR,
            layout_version: STATE_ACCOUNT_LAYOUT_VERSION,
            state,
        }
    }

    fn validate(&self) -> ProgramResult {
        if self.discriminator != STATE_ACCOUNT_DISCRIMINATOR
            || self.layout_version != STATE_ACCOUNT_LAYOUT_VERSION
        {
            return Err(TestInputProgramError::InvalidStateLayout.into());
        }
        Ok(())
    }
}

#[repr(u32)]
enum TestInputProgramError {
    InvalidInstructionData = 1,
    MissingRequiredSignature = 2,
    InvalidStateAccount = 3,
    StateAlreadyInitialized = 4,
    StateNotInitialized = 5,
    SerializationFailure = 6,
    InvalidStateLayout = 7,
    InvalidInitializer = 8,
    MissingSystemProgram = 9,
    MissingRentSysvar = 10,
    InvalidHostProgram = 11,
    InvalidHostStateAccount = 12,
    InvalidHostSessionAccount = 13,
    InvalidHostReturnData = 14,
    MissingHostReturnData = 15,
}

impl From<TestInputProgramError> for ProgramError {
    fn from(value: TestInputProgramError) -> Self {
        ProgramError::Custom(value as u32)
    }
}

pub fn id() -> SolanaPubkey {
    solana_program::pubkey!("5MaDNrtMTmYccr1ASgE1i2LZgbnyBPeDR7tN8Q8ewXTv")
}

pub fn process_instruction<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = TestInputInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::from(TestInputProgramError::InvalidInstructionData))?;

    match instruction {
        TestInputInstruction::InitializePda {
            owner,
            host_program,
        } => initialize_pda(program_id, accounts, owner, host_program),
        TestInputInstruction::RequestUint64NonTrivial {
            input_handle,
            input_proof,
            user_id,
        } => request_uint64_non_trivial(
            program_id,
            accounts,
            input_handle,
            input_proof,
            user_id,
        ),
        TestInputInstruction::Add42ToInput64 {
            input_handle,
            input_proof,
            user_id,
        } => add_42_to_input_64(
            program_id,
            accounts,
            input_handle,
            input_proof,
            user_id,
        ),
        TestInputInstruction::CreateUserDecryptFixture {
            fixture_index,
            user_id,
        } => create_user_decrypt_fixture(program_id, accounts, fixture_index, user_id),
        TestInputInstruction::CreateUserDecryptFixtures { user_id } => {
            create_user_decrypt_fixtures(program_id, accounts, user_id)
        }
        TestInputInstruction::CreateUserDecryptFixturesChunk {
            start_fixture_index,
            fixture_count,
            user_id,
        } => create_user_decrypt_fixtures_chunk(
            program_id,
            accounts,
            start_fixture_index,
            fixture_count,
            user_id,
        ),
        TestInputInstruction::CreatePublicEbool => create_public_ebool(program_id, accounts),
        TestInputInstruction::CreatePublicMixed => create_public_mixed(program_id, accounts),
    }
}

pub fn required_state_account_len(
    owner: HostPubkey,
    host_program: HostPubkey,
) -> Result<usize, ProgramError> {
    let state = TestInputState {
        owner,
        host_program,
        res_uint64: Some(Handle::from([0_u8; 32])),
        next_session_nonce: u64::MAX,
    };
    borsh::to_vec(&StoredTestInputState::new(state))
        .map(|bytes| bytes.len())
        .map_err(|_| ProgramError::from(TestInputProgramError::SerializationFailure))
}

fn initialize_pda<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
    owner: HostPubkey,
    host_program: HostPubkey,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let authority = next_account_info(account_iter)?;
    let state_account = next_account_info(account_iter)?;
    let system_program_account = account_iter.next();
    let rent_sysvar_account = account_iter.next();

    ensure_signer(authority)?;
    if HostPubkey::from(authority.key) != owner {
        return Err(TestInputProgramError::InvalidInitializer.into());
    }

    let (expected_pda, _) = find_state_pda(program_id);
    if state_account.key != &expected_pda {
        return Err(TestInputProgramError::InvalidStateAccount.into());
    }

    ensure_state_account_writable(state_account)?;
    if state_account.owner == program_id {
        ensure_state_account_owner(program_id, state_account)?;
    } else if system_program::check_id(state_account.owner)
        && state_account.lamports() == 0
        && state_account.data_is_empty()
    {
        create_pda_state_account(
            program_id,
            authority,
            state_account,
            system_program_account,
            rent_sysvar_account,
            owner,
            host_program,
        )?;
    } else {
        return Err(TestInputProgramError::InvalidStateAccount.into());
    }

    if load_state_account(state_account)?.is_some() {
        return Err(TestInputProgramError::StateAlreadyInitialized.into());
    }

    save_state_account(
        state_account,
        &StoredTestInputState::new(TestInputState {
            owner,
            host_program,
            res_uint64: None,
            next_session_nonce: 1,
        }),
    )?;
    set_return_data(&[]);
    Ok(())
}

fn request_uint64_non_trivial<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
    input_handle: Handle,
    input_proof: Vec<u8>,
    user_id: HostPubkey,
) -> ProgramResult {
    let execution = parse_execution_accounts(program_id, accounts)?;
    let mut state = execution.load_state()?;
    let session_nonce = reserve_session_nonce(&mut state);
    let app_contract = HostPubkey::from(execution.app_state.key);

    invoke_host_batch(
        &execution,
        session_nonce,
        vec![
            HostInstruction::VerifyInput {
                context: ContextUserInputs {
                    user_id,
                    contract_id: HostPubkey::from(execution.app_state.key),
                },
                input_handle,
                input_proof,
            },
            HostInstruction::Allow {
                handle: input_handle,
                account: app_contract,
            },
            HostInstruction::CleanTransientStorage,
        ],
    )?;

    state.res_uint64 = Some(input_handle);
    execution.save_state(&state)?;
    write_result_data(vec![input_handle])
}

fn add_42_to_input_64<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
    input_handle: Handle,
    input_proof: Vec<u8>,
    user_id: HostPubkey,
) -> ProgramResult {
    let execution = parse_execution_accounts(program_id, accounts)?;
    let mut state = execution.load_state()?;
    let session_nonce = reserve_session_nonce(&mut state);
    let app_contract = HostPubkey::from(execution.app_state.key);

    invoke_host_batch(
        &execution,
        session_nonce,
        vec![HostInstruction::VerifyInput {
            context: ContextUserInputs {
                user_id,
                contract_id: HostPubkey::from(execution.app_state.key),
            },
            input_handle,
            input_proof,
        }],
    )?;

    let trivial_42 = single_returned_handle(invoke_host_batch(
        &execution,
        session_nonce,
        vec![HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(42),
            to_type: FheType::Uint64,
            charge_hcu: false,
        }],
    )?)?;

    let result = single_returned_handle(invoke_host_batch(
        &execution,
        session_nonce,
        vec![HostInstruction::BinaryOp {
            op: solana_host_contracts_core::Operator::FheAdd,
            lhs: input_handle,
            rhs: BinaryOperand::Handle(trivial_42),
            result_type: FheType::Uint64,
            charge_hcu: false,
        }],
    )?)?;

    persist_allow_pairs(
        &execution,
        session_nonce,
        result,
        &[
            app_contract,
            user_id,
        ],
    )?;
    invoke_host_batch(
        &execution,
        session_nonce,
        vec![
            HostInstruction::AllowForDecryption {
                handles: vec![result],
            },
            HostInstruction::CleanTransientStorage,
        ],
    )?;

    state.res_uint64 = Some(result);
    execution.save_state(&state)?;
    write_result_data(vec![result])
}

fn create_public_ebool<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let execution = parse_execution_accounts(program_id, accounts)?;
    let mut state = execution.load_state()?;
    let session_nonce = reserve_session_nonce(&mut state);
    let app_contract = HostPubkey::from(execution.app_state.key);
    let handle = single_returned_handle(invoke_host_batch(
        &execution,
        session_nonce,
        vec![HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(1),
            to_type: FheType::Bool,
            charge_hcu: false,
        }],
    )?)?;

    persist_allow_pairs(
        &execution,
        session_nonce,
        handle,
        &[app_contract],
    )?;
    invoke_host_batch(
        &execution,
        session_nonce,
        vec![
            HostInstruction::AllowForDecryption {
                handles: vec![handle],
            },
            HostInstruction::CleanTransientStorage,
        ],
    )?;

    execution.save_state(&state)?;
    write_result_data(vec![handle])
}

fn create_user_decrypt_fixtures<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
    user_id: HostPubkey,
) -> ProgramResult {
    let execution = parse_execution_accounts(program_id, accounts)?;
    let mut state = execution.load_state()?;
    let session_nonce = reserve_session_nonce(&mut state);
    let app_contract = HostPubkey::from(execution.app_state.key);

    let trivial_encrypts = [
        HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(1),
            to_type: FheType::Bool,
            charge_hcu: false,
        },
        HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(42),
            to_type: FheType::Uint8,
            charge_hcu: false,
        },
        HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(16),
            to_type: FheType::Uint16,
            charge_hcu: false,
        },
        HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(32),
            to_type: FheType::Uint32,
            charge_hcu: false,
        },
        HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(18_446_744_073_709_551_600),
            to_type: FheType::Uint64,
            charge_hcu: false,
        },
        HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u128(145_275_933_516_363_203_950_142_179_850_024_740_765),
            to_type: FheType::Uint128,
            charge_hcu: false,
        },
        HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_evm_address(USER_DECRYPT_ADDRESS),
            to_type: FheType::Uint160,
            charge_hcu: false,
        },
        HostInstruction::TrivialEncrypt {
            plaintext: USER_DECRYPT_UINT256,
            to_type: FheType::Uint256,
            charge_hcu: false,
        },
    ];

    let mut handles = Vec::with_capacity(trivial_encrypts.len());
    for instruction in trivial_encrypts {
        handles.push(single_returned_handle(invoke_host_batch(
            &execution,
            session_nonce,
            vec![instruction],
        )?)?);
    }

    for handle in &handles {
        persist_allow_pairs(
            &execution,
            session_nonce,
            *handle,
            &[
                app_contract,
                user_id,
            ],
        )?;
    }
    invoke_host_batch(
        &execution,
        session_nonce,
        vec![HostInstruction::CleanTransientStorage],
    )?;

    execution.save_state(&state)?;
    write_result_data(handles)
}

fn create_user_decrypt_fixtures_chunk<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
    start_fixture_index: u8,
    fixture_count: u8,
    user_id: HostPubkey,
) -> ProgramResult {
    if fixture_count == 0 {
        return write_result_data(Vec::new());
    }

    let end_fixture_index = start_fixture_index
        .checked_add(fixture_count)
        .ok_or(ProgramError::InvalidInstructionData)?;
    if end_fixture_index > 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let execution = parse_execution_accounts(program_id, accounts)?;
    let mut state = execution.load_state()?;
    let session_nonce = reserve_session_nonce(&mut state);
    let app_contract = HostPubkey::from(execution.app_state.key);

    let mut handles = Vec::with_capacity(fixture_count as usize);
    for fixture_index in start_fixture_index..end_fixture_index {
        handles.push(single_returned_handle(invoke_host_batch(
            &execution,
            session_nonce,
            vec![user_decrypt_fixture_instruction(fixture_index)?],
        )?)?);
    }

    for handle in &handles {
        persist_allow_pairs(
            &execution,
            session_nonce,
            *handle,
            &[
                app_contract,
                user_id,
            ],
        )?;
    }
    invoke_host_batch(
        &execution,
        session_nonce,
        vec![HostInstruction::CleanTransientStorage],
    )?;

    execution.save_state(&state)?;
    write_result_data(handles)
}

fn create_user_decrypt_fixture<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
    fixture_index: u8,
    user_id: HostPubkey,
) -> ProgramResult {
    let execution = parse_execution_accounts(program_id, accounts)?;
    let mut state = execution.load_state()?;
    let session_nonce = reserve_session_nonce(&mut state);
    let app_contract = HostPubkey::from(execution.app_state.key);
    let handle = single_returned_handle(invoke_host_batch(
        &execution,
        session_nonce,
        vec![user_decrypt_fixture_instruction(fixture_index)?],
    )?)?;

    persist_allow_pairs(
        &execution,
        session_nonce,
        handle,
        &[
            app_contract,
            user_id,
        ],
    )?;
    invoke_host_batch(
        &execution,
        session_nonce,
        vec![HostInstruction::CleanTransientStorage],
    )?;

    execution.save_state(&state)?;
    write_result_data(vec![handle])
}

fn create_public_mixed<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let execution = parse_execution_accounts(program_id, accounts)?;
    let mut state = execution.load_state()?;
    let session_nonce = reserve_session_nonce(&mut state);
    let app_contract = HostPubkey::from(execution.app_state.key);
    let bool_handle = single_returned_handle(invoke_host_batch(
        &execution,
        session_nonce,
        vec![HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(1),
            to_type: FheType::Bool,
            charge_hcu: false,
        }],
    )?)?;
    let uint32_handle = single_returned_handle(invoke_host_batch(
        &execution,
        session_nonce,
        vec![HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(242),
            to_type: FheType::Uint32,
            charge_hcu: false,
        }],
    )?)?;
    let address_handle = single_returned_handle(invoke_host_batch(
        &execution,
        session_nonce,
        vec![HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_evm_address(MIXED_PUBLIC_ADDRESS),
            to_type: FheType::Uint160,
            charge_hcu: false,
        }],
    )?)?;

    persist_many_allow_pairs(
        &execution,
        session_nonce,
        &[
            (bool_handle, &[app_contract]),
            (uint32_handle, &[app_contract]),
            (address_handle, &[app_contract]),
        ],
    )?;
    invoke_host_batch(
        &execution,
        session_nonce,
        vec![
            HostInstruction::AllowForDecryption {
                handles: vec![bool_handle, uint32_handle, address_handle],
            },
            HostInstruction::CleanTransientStorage,
        ],
    )?;

    execution.save_state(&state)?;
    write_result_data(vec![bool_handle, uint32_handle, address_handle])
}

fn user_decrypt_fixture_instruction(fixture_index: u8) -> Result<HostInstruction, ProgramError> {
    let instruction = match fixture_index {
        0 => HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(1),
            to_type: FheType::Bool,
            charge_hcu: false,
        },
        1 => HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(42),
            to_type: FheType::Uint8,
            charge_hcu: false,
        },
        2 => HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(16),
            to_type: FheType::Uint16,
            charge_hcu: false,
        },
        3 => HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(32),
            to_type: FheType::Uint32,
            charge_hcu: false,
        },
        4 => HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(18_446_744_073_709_551_600),
            to_type: FheType::Uint64,
            charge_hcu: false,
        },
        5 => HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u128(145_275_933_516_363_203_950_142_179_850_024_740_765),
            to_type: FheType::Uint128,
            charge_hcu: false,
        },
        6 => HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_evm_address(USER_DECRYPT_ADDRESS),
            to_type: FheType::Uint160,
            charge_hcu: false,
        },
        7 => HostInstruction::TrivialEncrypt {
            plaintext: USER_DECRYPT_UINT256,
            to_type: FheType::Uint256,
            charge_hcu: false,
        },
        _ => return Err(TestInputProgramError::InvalidInstructionData.into()),
    };
    Ok(instruction)
}

struct ExecutionAccounts<'a> {
    authority: AccountInfo<'a>,
    app_state: AccountInfo<'a>,
    host_program: AccountInfo<'a>,
    host_state: AccountInfo<'a>,
    host_session: AccountInfo<'a>,
    clock: AccountInfo<'a>,
    slot_hashes: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    rent: AccountInfo<'a>,
}

impl<'a> ExecutionAccounts<'a> {
    fn load_state(&self) -> Result<TestInputState, ProgramError> {
        let stored = load_state_account(&self.app_state)?
            .ok_or_else(|| ProgramError::from(TestInputProgramError::StateNotInitialized))?;
        if SolanaPubkey::from(stored.state.host_program) != *self.host_program.key {
            return Err(TestInputProgramError::InvalidHostProgram.into());
        }
        if self.host_state.key != &find_host_state_pda(self.host_program.key).0 {
            return Err(TestInputProgramError::InvalidHostStateAccount.into());
        }
        if self.host_session.key
            != &find_host_session_pda(self.host_program.key, self.authority.key).0
        {
            return Err(TestInputProgramError::InvalidHostSessionAccount.into());
        }
        Ok(stored.state)
    }

    fn save_state(&self, state: &TestInputState) -> ProgramResult {
        save_state_account(&self.app_state, &StoredTestInputState::new(state.clone()))
    }
}

fn parse_execution_accounts<'a>(
    program_id: &SolanaPubkey,
    accounts: &[AccountInfo<'a>],
) -> Result<ExecutionAccounts<'a>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let authority = next_account_info(account_iter)?;
    let app_state = next_account_info(account_iter)?;
    let host_program = next_account_info(account_iter)?;
    let host_state = next_account_info(account_iter)?;
    let host_session = next_account_info(account_iter)?;
    let clock = next_account_info(account_iter)?;
    let slot_hashes = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    let rent = next_account_info(account_iter)?;

    ensure_signer(authority)?;
    ensure_state_account_owner(program_id, app_state)?;

    Ok(ExecutionAccounts {
        authority: authority.clone(),
        app_state: app_state.clone(),
        host_program: host_program.clone(),
        host_state: host_state.clone(),
        host_session: host_session.clone(),
        clock: clock.clone(),
        slot_hashes: slot_hashes.clone(),
        system_program: system_program.clone(),
        rent: rent.clone(),
    })
}

fn invoke_host_batch(
    execution: &ExecutionAccounts<'_>,
    session_nonce: u64,
    instructions: Vec<HostInstruction>,
) -> Result<Vec<HostInstructionResult>, ProgramError> {
    let host_ix = HostOnchainInstruction::ExecuteBatch {
        instructions,
        session_nonce,
        recent_blockhash: [0; 32],
    };

    let instruction = Instruction {
        program_id: *execution.host_program.key,
        accounts: vec![
            AccountMeta::new(*execution.authority.key, true),
            AccountMeta::new(*execution.host_state.key, false),
            AccountMeta::new(*execution.host_session.key, false),
            AccountMeta::new_readonly(*execution.clock.key, false),
            AccountMeta::new_readonly(*execution.slot_hashes.key, false),
            AccountMeta::new_readonly(*execution.system_program.key, false),
            AccountMeta::new_readonly(*execution.rent.key, false),
        ],
        data: borsh::to_vec(&host_ix)
            .map_err(|_| ProgramError::from(TestInputProgramError::SerializationFailure))?,
    };
    invoke(
        &instruction,
        &[
            execution.host_program.clone(),
            execution.authority.clone(),
            execution.host_state.clone(),
            execution.host_session.clone(),
            execution.clock.clone(),
            execution.slot_hashes.clone(),
            execution.system_program.clone(),
            execution.rent.clone(),
        ],
    )?;

    let Some((returning_program, return_data)) = get_return_data() else {
        return Err(TestInputProgramError::MissingHostReturnData.into());
    };
    if returning_program != *execution.host_program.key {
        return Err(TestInputProgramError::InvalidHostReturnData.into());
    }

    Vec::<HostInstructionResult>::try_from_slice(&return_data)
        .map_err(|_| ProgramError::from(TestInputProgramError::InvalidHostReturnData))
}

fn persist_allow_pairs(
    execution: &ExecutionAccounts<'_>,
    session_nonce: u64,
    handle: Handle,
    accounts: &[HostPubkey],
) -> Result<(), ProgramError> {
    let instructions = accounts
        .iter()
        .copied()
        .map(|account| HostInstruction::Allow { handle, account })
        .collect();
    invoke_host_batch(execution, session_nonce, instructions)?;
    Ok(())
}

fn persist_many_allow_pairs(
    execution: &ExecutionAccounts<'_>,
    session_nonce: u64,
    entries: &[(Handle, &[HostPubkey])],
) -> Result<(), ProgramError> {
    let mut instructions = Vec::new();
    for (handle, accounts) in entries {
        instructions.extend(
            accounts
                .iter()
                .copied()
                .map(|account| HostInstruction::Allow {
                    handle: *handle,
                    account,
                }),
        );
    }
    invoke_host_batch(execution, session_nonce, instructions)?;
    Ok(())
}

fn reserve_session_nonce(state: &mut TestInputState) -> u64 {
    let session_nonce = state.next_session_nonce;
    state.next_session_nonce = state.next_session_nonce.checked_add(1).unwrap_or(1);
    session_nonce
}

fn single_returned_handle(results: Vec<HostInstructionResult>) -> Result<Handle, ProgramError> {
    let handle = results
        .into_iter()
        .find_map(|result| result.returned_handle)
        .ok_or_else(|| ProgramError::from(TestInputProgramError::InvalidHostReturnData))?;
    Ok(handle)
}

fn write_result_data(handles: Vec<Handle>) -> ProgramResult {
    let encoded = borsh::to_vec(&TestInputExecutionResult {
        returned_handles: handles,
    })
    .map_err(|_| ProgramError::from(TestInputProgramError::SerializationFailure))?;
    set_return_data(&encoded);
    Ok(())
}

fn load_state_account(
    account: &AccountInfo<'_>,
) -> Result<Option<StoredTestInputState>, ProgramError> {
    let data = account.try_borrow_data()?;
    if data.is_empty() || data.iter().all(|byte| *byte == 0) {
        return Ok(None);
    }

    let mut slice: &[u8] = &data;
    let stored = StoredTestInputState::deserialize(&mut slice)
        .map_err(|_| ProgramError::from(TestInputProgramError::SerializationFailure))?;
    stored.validate()?;
    Ok(Some(stored))
}

fn save_state_account(account: &AccountInfo<'_>, state: &StoredTestInputState) -> ProgramResult {
    let serialized = borsh::to_vec(state)
        .map_err(|_| ProgramError::from(TestInputProgramError::SerializationFailure))?;
    if serialized.len() > account.data_len() {
        return Err(TestInputProgramError::InvalidStateAccount.into());
    }

    let mut data = account.try_borrow_mut_data()?;
    data.fill(0);
    data[..serialized.len()].copy_from_slice(&serialized);
    Ok(())
}

fn create_pda_state_account<'a>(
    program_id: &SolanaPubkey,
    payer: &AccountInfo<'a>,
    state_account: &AccountInfo<'a>,
    system_program_account: Option<&'a AccountInfo<'a>>,
    rent_sysvar_account: Option<&'a AccountInfo<'a>>,
    owner: HostPubkey,
    host_program: HostPubkey,
) -> ProgramResult {
    let system_program_account = ensure_system_program_account(system_program_account)?;
    let required_len = required_state_account_len(owner, host_program)?;
    let lamports = load_rent(rent_sysvar_account)?.minimum_balance(required_len);
    let (_, bump) = find_state_pda(program_id);
    let bump_seed = [bump];
    let signer_seeds: &[&[u8]] = &[TEST_INPUT_STATE_PDA_SEED, &bump_seed];

    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            state_account.key,
            lamports,
            required_len as u64,
            program_id,
        ),
        &[
            payer.clone(),
            state_account.clone(),
            system_program_account.clone(),
        ],
        &[signer_seeds],
    )
}

fn ensure_signer(account: &AccountInfo<'_>) -> ProgramResult {
    if !account.is_signer {
        return Err(TestInputProgramError::MissingRequiredSignature.into());
    }
    Ok(())
}

fn ensure_state_account_owner(
    program_id: &SolanaPubkey,
    account: &AccountInfo<'_>,
) -> ProgramResult {
    if account.owner != program_id {
        return Err(TestInputProgramError::InvalidStateAccount.into());
    }
    ensure_state_account_writable(account)
}

fn ensure_state_account_writable(account: &AccountInfo<'_>) -> ProgramResult {
    if !account.is_writable || account.executable {
        return Err(TestInputProgramError::InvalidStateAccount.into());
    }
    Ok(())
}

fn ensure_system_program_account<'a>(
    account: Option<&'a AccountInfo<'a>>,
) -> Result<&'a AccountInfo<'a>, ProgramError> {
    let account =
        account.ok_or_else(|| ProgramError::from(TestInputProgramError::MissingSystemProgram))?;
    if !system_program::check_id(account.key) {
        return Err(TestInputProgramError::MissingSystemProgram.into());
    }
    Ok(account)
}

fn load_rent(account: Option<&AccountInfo<'_>>) -> Result<Rent, ProgramError> {
    let account =
        account.ok_or_else(|| ProgramError::from(TestInputProgramError::MissingRentSysvar))?;
    if account.key != &sysvar::rent::id() {
        return Err(TestInputProgramError::MissingRentSysvar.into());
    }
    Rent::from_account_info(account)
        .map_err(|_| ProgramError::from(TestInputProgramError::MissingRentSysvar))
}

fn scalar_word_from_u64(value: u64) -> [u8; 32] {
    let mut output = [0_u8; 32];
    output[24..].copy_from_slice(&value.to_be_bytes());
    output
}

fn scalar_word_from_u128(value: u128) -> [u8; 32] {
    let mut output = [0_u8; 32];
    output[16..].copy_from_slice(&value.to_be_bytes());
    output
}

fn scalar_word_from_evm_address(address: [u8; 20]) -> [u8; 32] {
    let mut output = [0_u8; 32];
    output[12..].copy_from_slice(&address);
    output
}
