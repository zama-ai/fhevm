use borsh::{BorshDeserialize, BorshSerialize};
use solana_encrypted_erc20_core::{
    evm_address_from_solana_pubkey, find_state_pda, AllowanceEntry, BalanceEntry,
    EncryptedErc20ExecutionResult, EncryptedErc20Instruction, EncryptedErc20State,
    ENCRYPTED_ERC20_STATE_PDA_SEED,
};
use solana_host_contracts_core::{
    find_session_pda as find_host_session_pda, find_state_pda as find_host_state_pda,
    BinaryOperand, ContextUserInputs, FheType, Handle, HostInstruction,
    InstructionResult as HostInstructionResult, OnchainInstruction as HostOnchainInstruction,
    Operator, Pubkey as HostPubkey,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{get_return_data, invoke_signed, set_return_data},
    program_error::ProgramError,
    pubkey::Pubkey as SolanaPubkey,
    rent::Rent,
    sysvar,
    sysvar::SysvarSerialize,
};
use solana_system_interface::{instruction as system_instruction, program as system_program};

const STATE_ACCOUNT_DISCRIMINATOR: [u8; 8] = *b"EERC2000";
const STATE_ACCOUNT_LAYOUT_VERSION: u32 = 1;
const SESSION_NONCE: u64 = 1;

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
struct StoredEncryptedErc20State {
    discriminator: [u8; 8],
    layout_version: u32,
    state: EncryptedErc20State,
}

impl StoredEncryptedErc20State {
    fn new(state: EncryptedErc20State) -> Self {
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
            return Err(EncryptedErc20ProgramError::InvalidStateLayout.into());
        }
        Ok(())
    }
}

#[repr(u32)]
enum EncryptedErc20ProgramError {
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
    CapacityExceeded = 16,
    Unauthorized = 17,
}

impl From<EncryptedErc20ProgramError> for ProgramError {
    fn from(value: EncryptedErc20ProgramError) -> Self {
        ProgramError::Custom(value as u32)
    }
}

pub fn id() -> SolanaPubkey {
    solana_program::pubkey!("Cjb3AVoxxKmG4TGWX5gzSjCNwtxN6gneVsWB7f9i8Csx")
}

pub fn process_instruction<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = EncryptedErc20Instruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::from(EncryptedErc20ProgramError::InvalidInstructionData))?;

    match instruction {
        EncryptedErc20Instruction::InitializePda {
            owner,
            host_program,
            name,
            symbol,
            max_balance_entries,
            max_allowance_entries,
        } => initialize_pda(
            program_id,
            accounts,
            owner,
            host_program,
            name,
            symbol,
            max_balance_entries,
            max_allowance_entries,
        ),
        EncryptedErc20Instruction::Mint { minted_amount } => mint(program_id, accounts, minted_amount),
        EncryptedErc20Instruction::Transfer {
            to,
            input_handle,
            input_proof,
        } => transfer(program_id, accounts, to, input_handle, input_proof),
        EncryptedErc20Instruction::Approve {
            spender,
            input_handle,
            input_proof,
        } => approve(program_id, accounts, spender, input_handle, input_proof),
        EncryptedErc20Instruction::TransferFrom {
            from,
            to,
            input_handle,
            input_proof,
        } => transfer_from(program_id, accounts, from, to, input_handle, input_proof),
        EncryptedErc20Instruction::BalanceOf { wallet } => balance_of(program_id, accounts, wallet),
        EncryptedErc20Instruction::Allowance { owner, spender } => {
            allowance(program_id, accounts, owner, spender)
        }
        EncryptedErc20Instruction::TotalSupply => total_supply(program_id, accounts),
    }
}

pub fn required_state_account_len(
    owner: HostPubkey,
    host_program: HostPubkey,
    name: &str,
    symbol: &str,
    max_balance_entries: u16,
    max_allowance_entries: u16,
) -> Result<usize, ProgramError> {
    let state = EncryptedErc20State {
        owner,
        host_program,
        name: name.to_owned(),
        symbol: symbol.to_owned(),
        total_supply: u64::MAX,
        max_balance_entries,
        max_allowance_entries,
        balances: vec![
            BalanceEntry {
                wallet: HostPubkey::from([0_u8; 32]),
                handle: Handle::from([0_u8; 32]),
            };
            max_balance_entries as usize
        ],
        allowances: vec![
            AllowanceEntry {
                owner: HostPubkey::from([0_u8; 32]),
                spender: HostPubkey::from([0_u8; 32]),
                handle: Handle::from([0_u8; 32]),
            };
            max_allowance_entries as usize
        ],
    };
    borsh::to_vec(&StoredEncryptedErc20State::new(state))
        .map(|bytes| bytes.len())
        .map_err(|_| ProgramError::from(EncryptedErc20ProgramError::SerializationFailure))
}

fn initialize_pda<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
    owner: HostPubkey,
    host_program: HostPubkey,
    name: String,
    symbol: String,
    max_balance_entries: u16,
    max_allowance_entries: u16,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let authority = next_account_info(account_iter)?;
    let state_account = next_account_info(account_iter)?;
    let system_program_account = account_iter.next();
    let rent_sysvar_account = account_iter.next();

    ensure_signer(authority)?;
    if HostPubkey::from(authority.key) != owner {
        return Err(EncryptedErc20ProgramError::InvalidInitializer.into());
    }

    let (expected_pda, _) = find_state_pda(program_id);
    if state_account.key != &expected_pda {
        return Err(EncryptedErc20ProgramError::InvalidStateAccount.into());
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
            &name,
            &symbol,
            max_balance_entries,
            max_allowance_entries,
        )?;
    } else {
        return Err(EncryptedErc20ProgramError::InvalidStateAccount.into());
    }

    if load_state_account(state_account)?.is_some() {
        return Err(EncryptedErc20ProgramError::StateAlreadyInitialized.into());
    }

    save_state_account(
        state_account,
        &StoredEncryptedErc20State::new(EncryptedErc20State {
            owner,
            host_program,
            name,
            symbol,
            total_supply: 0,
            max_balance_entries,
            max_allowance_entries,
            balances: vec![],
            allowances: vec![],
        }),
    )?;
    set_return_data(&[]);
    Ok(())
}

fn mint<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
    minted_amount: u64,
) -> ProgramResult {
    let execution = parse_execution_accounts(program_id, accounts)?;
    let mut state = execution.load_state()?;
    if state.owner != HostPubkey::from(execution.authority.key) {
        return Err(EncryptedErc20ProgramError::Unauthorized.into());
    }

    let owner = HostPubkey::from(execution.authority.key);
    let current_balance = ensure_balance_handle(&execution, &mut state, owner)?;
    let new_balance = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::BinaryOp {
            op: Operator::FheAdd,
            lhs: current_balance,
            rhs: BinaryOperand::Scalar(scalar_word_from_u64(minted_amount)),
            result_type: FheType::Uint64,
            charge_hcu: false,
        }],
    )?)?;

    persist_balance_handle(&execution, owner, new_balance)?;
    set_balance_handle(&mut state, owner, new_balance)?;
    state.total_supply = state.total_supply.saturating_add(minted_amount);
    execution.save_state(&state)?;
    clean_host_transients(&execution)?;
    write_result_data(vec![new_balance], Some(state.total_supply))
}

fn transfer<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
    to: HostPubkey,
    input_handle: Handle,
    input_proof: Vec<u8>,
) -> ProgramResult {
    let execution = parse_execution_accounts(program_id, accounts)?;
    let mut state = execution.load_state()?;
    let from = HostPubkey::from(execution.authority.key);

    let verified_amount = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::VerifyInput {
            context: ContextUserInputs {
                user_address: evm_address_from_solana_pubkey(execution.authority.key),
                contract_address: evm_address_from_solana_pubkey(execution.app_state.key),
            },
            input_handle,
            input_proof,
        }],
    )?)?;

    let current_from = ensure_balance_handle(&execution, &mut state, from)?;
    let current_to = ensure_balance_handle(&execution, &mut state, to)?;
    let zero = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(0),
            to_type: FheType::Uint64,
            charge_hcu: false,
        }],
    )?)?;

    let can_transfer = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::BinaryOp {
            op: Operator::FheLe,
            lhs: verified_amount,
            rhs: BinaryOperand::Handle(current_from),
            result_type: FheType::Bool,
            charge_hcu: false,
        }],
    )?)?;

    let transfer_value = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::TernaryOp {
            op: Operator::FheIfThenElse,
            control: can_transfer,
            if_true: verified_amount,
            if_false: zero,
            charge_hcu: false,
        }],
    )?)?;

    let new_balance_to = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::BinaryOp {
            op: Operator::FheAdd,
            lhs: current_to,
            rhs: BinaryOperand::Handle(transfer_value),
            result_type: FheType::Uint64,
            charge_hcu: false,
        }],
    )?)?;

    let new_balance_from = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::BinaryOp {
            op: Operator::FheSub,
            lhs: current_from,
            rhs: BinaryOperand::Handle(transfer_value),
            result_type: FheType::Uint64,
            charge_hcu: false,
        }],
    )?)?;

    persist_balance_handles(&execution, &[(from, new_balance_from), (to, new_balance_to)])?;
    set_balance_handle(&mut state, from, new_balance_from)?;
    set_balance_handle(&mut state, to, new_balance_to)?;
    execution.save_state(&state)?;
    clean_host_transients(&execution)?;
    write_result_data(vec![new_balance_from, new_balance_to], Some(state.total_supply))
}

fn approve<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
    spender: HostPubkey,
    input_handle: Handle,
    input_proof: Vec<u8>,
) -> ProgramResult {
    let execution = parse_execution_accounts(program_id, accounts)?;
    let mut state = execution.load_state()?;
    let owner = HostPubkey::from(execution.authority.key);

    let verified_amount = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::VerifyInput {
            context: ContextUserInputs {
                user_address: evm_address_from_solana_pubkey(execution.authority.key),
                contract_address: evm_address_from_solana_pubkey(execution.app_state.key),
            },
            input_handle,
            input_proof,
        }],
    )?)?;

    persist_allowance_handle(&execution, owner, spender, verified_amount)?;
    set_allowance_handle(&mut state, owner, spender, verified_amount)?;
    execution.save_state(&state)?;
    clean_host_transients(&execution)?;
    write_result_data(vec![verified_amount], Some(state.total_supply))
}

fn transfer_from<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
    from: HostPubkey,
    to: HostPubkey,
    input_handle: Handle,
    input_proof: Vec<u8>,
) -> ProgramResult {
    let execution = parse_execution_accounts(program_id, accounts)?;
    let mut state = execution.load_state()?;
    let spender = HostPubkey::from(execution.authority.key);

    let verified_amount = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::VerifyInput {
            context: ContextUserInputs {
                user_address: evm_address_from_solana_pubkey(execution.authority.key),
                contract_address: evm_address_from_solana_pubkey(execution.app_state.key),
            },
            input_handle,
            input_proof,
        }],
    )?)?;

    let current_allowance = ensure_allowance_handle(&execution, &mut state, from, spender)?;
    let current_from = ensure_balance_handle(&execution, &mut state, from)?;
    let current_to = ensure_balance_handle(&execution, &mut state, to)?;
    let zero = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(0),
            to_type: FheType::Uint64,
            charge_hcu: false,
        }],
    )?)?;

    let allowed_transfer = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::BinaryOp {
            op: Operator::FheLe,
            lhs: verified_amount,
            rhs: BinaryOperand::Handle(current_allowance),
            result_type: FheType::Bool,
            charge_hcu: false,
        }],
    )?)?;

    let can_transfer = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::BinaryOp {
            op: Operator::FheLe,
            lhs: verified_amount,
            rhs: BinaryOperand::Handle(current_from),
            result_type: FheType::Bool,
            charge_hcu: false,
        }],
    )?)?;

    let is_transferable = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::BinaryOp {
            op: Operator::FheBitAnd,
            lhs: can_transfer,
            rhs: BinaryOperand::Handle(allowed_transfer),
            result_type: FheType::Bool,
            charge_hcu: false,
        }],
    )?)?;

    let decreased_allowance = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::BinaryOp {
            op: Operator::FheSub,
            lhs: current_allowance,
            rhs: BinaryOperand::Handle(verified_amount),
            result_type: FheType::Uint64,
            charge_hcu: false,
        }],
    )?)?;

    let next_allowance = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::TernaryOp {
            op: Operator::FheIfThenElse,
            control: is_transferable,
            if_true: decreased_allowance,
            if_false: current_allowance,
            charge_hcu: false,
        }],
    )?)?;

    let transfer_value = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::TernaryOp {
            op: Operator::FheIfThenElse,
            control: is_transferable,
            if_true: verified_amount,
            if_false: zero,
            charge_hcu: false,
        }],
    )?)?;

    let new_balance_to = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::BinaryOp {
            op: Operator::FheAdd,
            lhs: current_to,
            rhs: BinaryOperand::Handle(transfer_value),
            result_type: FheType::Uint64,
            charge_hcu: false,
        }],
    )?)?;

    let new_balance_from = single_returned_handle(invoke_host_batch(
        &execution,
        vec![HostInstruction::BinaryOp {
            op: Operator::FheSub,
            lhs: current_from,
            rhs: BinaryOperand::Handle(transfer_value),
            result_type: FheType::Uint64,
            charge_hcu: false,
        }],
    )?)?;

    persist_allowance_handle(&execution, from, spender, next_allowance)?;
    persist_balance_handles(&execution, &[(from, new_balance_from), (to, new_balance_to)])?;
    set_allowance_handle(&mut state, from, spender, next_allowance)?;
    set_balance_handle(&mut state, from, new_balance_from)?;
    set_balance_handle(&mut state, to, new_balance_to)?;
    execution.save_state(&state)?;
    clean_host_transients(&execution)?;
    write_result_data(
        vec![new_balance_from, new_balance_to, next_allowance],
        Some(state.total_supply),
    )
}

fn balance_of<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
    wallet: HostPubkey,
) -> ProgramResult {
    let execution = parse_execution_accounts(program_id, accounts)?;
    let mut state = execution.load_state()?;
    let handle = ensure_balance_handle(&execution, &mut state, wallet)?;
    execution.save_state(&state)?;
    clean_host_transients(&execution)?;
    write_result_data(vec![handle], Some(state.total_supply))
}

fn allowance<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
    owner: HostPubkey,
    spender: HostPubkey,
) -> ProgramResult {
    let execution = parse_execution_accounts(program_id, accounts)?;
    let mut state = execution.load_state()?;
    let handle = ensure_allowance_handle(&execution, &mut state, owner, spender)?;
    execution.save_state(&state)?;
    clean_host_transients(&execution)?;
    write_result_data(vec![handle], Some(state.total_supply))
}

fn total_supply<'a>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let execution = parse_execution_accounts(program_id, accounts)?;
    let state = execution.load_state()?;
    write_result_data(vec![], Some(state.total_supply))
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
    app_state_bump: u8,
}

impl<'a> ExecutionAccounts<'a> {
    fn load_state(&self) -> Result<EncryptedErc20State, ProgramError> {
        let stored = load_state_account(&self.app_state)?
            .ok_or_else(|| ProgramError::from(EncryptedErc20ProgramError::StateNotInitialized))?;
        if SolanaPubkey::from(stored.state.host_program) != *self.host_program.key {
            return Err(EncryptedErc20ProgramError::InvalidHostProgram.into());
        }
        if self.host_state.key != &find_host_state_pda(self.host_program.key).0 {
            return Err(EncryptedErc20ProgramError::InvalidHostStateAccount.into());
        }
        if self.host_session.key != &find_host_session_pda(self.host_program.key, self.app_state.key).0
        {
            return Err(EncryptedErc20ProgramError::InvalidHostSessionAccount.into());
        }
        Ok(stored.state)
    }

    fn save_state(&self, state: &EncryptedErc20State) -> ProgramResult {
        save_state_account(&self.app_state, &StoredEncryptedErc20State::new(state.clone()))
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

    let (expected_pda, bump) = find_state_pda(program_id);
    if app_state.key != &expected_pda {
        return Err(EncryptedErc20ProgramError::InvalidStateAccount.into());
    }

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
        app_state_bump: bump,
    })
}

fn invoke_host_batch(
    execution: &ExecutionAccounts<'_>,
    instructions: Vec<HostInstruction>,
) -> Result<Vec<HostInstructionResult>, ProgramError> {
    let host_ix = HostOnchainInstruction::ExecuteBatch {
        instructions,
        session_nonce: SESSION_NONCE,
        recent_blockhash: [0; 32],
    };

    let instruction = Instruction {
        program_id: *execution.host_program.key,
        accounts: vec![
            AccountMeta::new(*execution.app_state.key, true),
            AccountMeta::new(*execution.authority.key, true),
            AccountMeta::new(*execution.host_state.key, false),
            AccountMeta::new(*execution.host_session.key, false),
            AccountMeta::new_readonly(*execution.clock.key, false),
            AccountMeta::new_readonly(*execution.slot_hashes.key, false),
            AccountMeta::new_readonly(*execution.system_program.key, false),
            AccountMeta::new_readonly(*execution.rent.key, false),
        ],
        data: borsh::to_vec(&host_ix)
            .map_err(|_| ProgramError::from(EncryptedErc20ProgramError::SerializationFailure))?,
    };
    let bump_seed = [execution.app_state_bump];
    let signer_seeds: &[&[u8]] = &[ENCRYPTED_ERC20_STATE_PDA_SEED, &bump_seed];

    invoke_signed(
        &instruction,
        &[
            execution.host_program.clone(),
            execution.app_state.clone(),
            execution.authority.clone(),
            execution.host_state.clone(),
            execution.host_session.clone(),
            execution.clock.clone(),
            execution.slot_hashes.clone(),
            execution.system_program.clone(),
            execution.rent.clone(),
        ],
        &[signer_seeds],
    )?;

    let Some((returning_program, return_data)) = get_return_data() else {
        return Err(EncryptedErc20ProgramError::MissingHostReturnData.into());
    };
    if returning_program != *execution.host_program.key {
        return Err(EncryptedErc20ProgramError::InvalidHostReturnData.into());
    }

    Vec::<HostInstructionResult>::try_from_slice(&return_data)
        .map_err(|_| ProgramError::from(EncryptedErc20ProgramError::InvalidHostReturnData))
}

fn ensure_balance_handle(
    execution: &ExecutionAccounts<'_>,
    state: &mut EncryptedErc20State,
    wallet: HostPubkey,
) -> Result<Handle, ProgramError> {
    if let Some(handle) = state
        .balances
        .iter()
        .find(|entry| entry.wallet == wallet)
        .map(|entry| entry.handle)
    {
        return Ok(handle);
    }
    if state.balances.len() >= state.max_balance_entries as usize {
        return Err(EncryptedErc20ProgramError::CapacityExceeded.into());
    }
    let zero = single_returned_handle(invoke_host_batch(
        execution,
        vec![HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(0),
            to_type: FheType::Uint64,
            charge_hcu: false,
        }],
    )?)?;
    persist_balance_handle(execution, wallet, zero)?;
    state.balances.push(BalanceEntry { wallet, handle: zero });
    Ok(zero)
}

fn ensure_allowance_handle(
    execution: &ExecutionAccounts<'_>,
    state: &mut EncryptedErc20State,
    owner: HostPubkey,
    spender: HostPubkey,
) -> Result<Handle, ProgramError> {
    if let Some(handle) = state
        .allowances
        .iter()
        .find(|entry| entry.owner == owner && entry.spender == spender)
        .map(|entry| entry.handle)
    {
        return Ok(handle);
    }
    if state.allowances.len() >= state.max_allowance_entries as usize {
        return Err(EncryptedErc20ProgramError::CapacityExceeded.into());
    }
    let zero = single_returned_handle(invoke_host_batch(
        execution,
        vec![HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(0),
            to_type: FheType::Uint64,
            charge_hcu: false,
        }],
    )?)?;
    persist_allowance_handle(execution, owner, spender, zero)?;
    state.allowances.push(AllowanceEntry {
        owner,
        spender,
        handle: zero,
    });
    Ok(zero)
}

fn set_balance_handle(
    state: &mut EncryptedErc20State,
    wallet: HostPubkey,
    handle: Handle,
) -> Result<(), ProgramError> {
    if let Some(entry) = state.balances.iter_mut().find(|entry| entry.wallet == wallet) {
        entry.handle = handle;
        return Ok(());
    }
    if state.balances.len() >= state.max_balance_entries as usize {
        return Err(EncryptedErc20ProgramError::CapacityExceeded.into());
    }
    state.balances.push(BalanceEntry { wallet, handle });
    Ok(())
}

fn set_allowance_handle(
    state: &mut EncryptedErc20State,
    owner: HostPubkey,
    spender: HostPubkey,
    handle: Handle,
) -> Result<(), ProgramError> {
    if let Some(entry) = state
        .allowances
        .iter_mut()
        .find(|entry| entry.owner == owner && entry.spender == spender)
    {
        entry.handle = handle;
        return Ok(());
    }
    if state.allowances.len() >= state.max_allowance_entries as usize {
        return Err(EncryptedErc20ProgramError::CapacityExceeded.into());
    }
    state.allowances.push(AllowanceEntry {
        owner,
        spender,
        handle,
    });
    Ok(())
}

fn persist_balance_handle(
    execution: &ExecutionAccounts<'_>,
    wallet: HostPubkey,
    handle: Handle,
) -> ProgramResult {
    persist_handle_to_accounts(
        execution,
        handle,
        &[
            HostPubkey::from(execution.app_state.key),
            wallet,
        ],
    )
}

fn persist_balance_handles(
    execution: &ExecutionAccounts<'_>,
    entries: &[(HostPubkey, Handle)],
) -> ProgramResult {
    let mut instructions = Vec::with_capacity(entries.len() * 2);
    for (wallet, handle) in entries {
        instructions.push(HostInstruction::Allow {
            handle: *handle,
            account: HostPubkey::from(execution.app_state.key),
        });
        instructions.push(HostInstruction::Allow {
            handle: *handle,
            account: *wallet,
        });
    }
    let _ = invoke_host_batch(execution, instructions)?;
    Ok(())
}

fn persist_allowance_handle(
    execution: &ExecutionAccounts<'_>,
    owner: HostPubkey,
    spender: HostPubkey,
    handle: Handle,
) -> ProgramResult {
    persist_handle_to_accounts(
        execution,
        handle,
        &[
            HostPubkey::from(execution.app_state.key),
            owner,
            spender,
        ],
    )
}

fn persist_handle_to_accounts(
    execution: &ExecutionAccounts<'_>,
    handle: Handle,
    accounts: &[HostPubkey],
) -> ProgramResult {
    let mut instructions = Vec::with_capacity(accounts.len());
    for account in accounts {
        instructions.push(HostInstruction::Allow {
            handle,
            account: *account,
        });
    }
    let _ = invoke_host_batch(execution, instructions)?;
    Ok(())
}

fn clean_host_transients(execution: &ExecutionAccounts<'_>) -> ProgramResult {
    let _ = invoke_host_batch(execution, vec![HostInstruction::CleanTransientStorage])?;
    Ok(())
}

fn single_returned_handle(results: Vec<HostInstructionResult>) -> Result<Handle, ProgramError> {
    results
        .into_iter()
        .find_map(|result| result.returned_handle)
        .ok_or_else(|| ProgramError::from(EncryptedErc20ProgramError::InvalidHostReturnData))
}

fn write_result_data(handles: Vec<Handle>, total_supply: Option<u64>) -> ProgramResult {
    let encoded = borsh::to_vec(&EncryptedErc20ExecutionResult {
        returned_handles: handles,
        total_supply,
    })
    .map_err(|_| ProgramError::from(EncryptedErc20ProgramError::SerializationFailure))?;
    set_return_data(&encoded);
    Ok(())
}

fn load_state_account(
    account: &AccountInfo<'_>,
) -> Result<Option<StoredEncryptedErc20State>, ProgramError> {
    let data = account.try_borrow_data()?;
    if data.is_empty() || data.iter().all(|byte| *byte == 0) {
        return Ok(None);
    }

    let mut slice: &[u8] = &data;
    let stored = StoredEncryptedErc20State::deserialize(&mut slice)
        .map_err(|_| ProgramError::from(EncryptedErc20ProgramError::SerializationFailure))?;
    stored.validate()?;
    Ok(Some(stored))
}

fn save_state_account(
    account: &AccountInfo<'_>,
    state: &StoredEncryptedErc20State,
) -> ProgramResult {
    let serialized = borsh::to_vec(state)
        .map_err(|_| ProgramError::from(EncryptedErc20ProgramError::SerializationFailure))?;
    if serialized.len() > account.data_len() {
        return Err(EncryptedErc20ProgramError::InvalidStateAccount.into());
    }

    let mut data = account.try_borrow_mut_data()?;
    data.fill(0);
    data[..serialized.len()].copy_from_slice(&serialized);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn create_pda_state_account<'a>(
    program_id: &SolanaPubkey,
    payer: &AccountInfo<'a>,
    state_account: &AccountInfo<'a>,
    system_program_account: Option<&'a AccountInfo<'a>>,
    rent_sysvar_account: Option<&'a AccountInfo<'a>>,
    owner: HostPubkey,
    host_program: HostPubkey,
    name: &str,
    symbol: &str,
    max_balance_entries: u16,
    max_allowance_entries: u16,
) -> ProgramResult {
    let system_program_account = ensure_system_program_account(system_program_account)?;
    let required_len = required_state_account_len(
        owner,
        host_program,
        name,
        symbol,
        max_balance_entries,
        max_allowance_entries,
    )?;
    let lamports = load_rent(rent_sysvar_account)?.minimum_balance(required_len);
    let (_, bump) = find_state_pda(program_id);
    let bump_seed = [bump];
    let signer_seeds: &[&[u8]] = &[ENCRYPTED_ERC20_STATE_PDA_SEED, &bump_seed];

    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            state_account.key,
            lamports,
            required_len as u64,
            program_id,
        ),
        &[payer.clone(), state_account.clone(), system_program_account.clone()],
        &[signer_seeds],
    )
}

fn ensure_signer(account: &AccountInfo<'_>) -> ProgramResult {
    if !account.is_signer {
        return Err(EncryptedErc20ProgramError::MissingRequiredSignature.into());
    }
    Ok(())
}

fn ensure_state_account_owner(
    program_id: &SolanaPubkey,
    account: &AccountInfo<'_>,
) -> ProgramResult {
    if account.owner != program_id {
        return Err(EncryptedErc20ProgramError::InvalidStateAccount.into());
    }
    ensure_state_account_writable(account)
}

fn ensure_state_account_writable(account: &AccountInfo<'_>) -> ProgramResult {
    if !account.is_writable || account.executable {
        return Err(EncryptedErc20ProgramError::InvalidStateAccount.into());
    }
    Ok(())
}

fn ensure_system_program_account<'a>(
    account: Option<&'a AccountInfo<'a>>,
) -> Result<&'a AccountInfo<'a>, ProgramError> {
    let account =
        account.ok_or_else(|| ProgramError::from(EncryptedErc20ProgramError::MissingSystemProgram))?;
    if !system_program::check_id(account.key) {
        return Err(EncryptedErc20ProgramError::MissingSystemProgram.into());
    }
    Ok(account)
}

fn load_rent(account: Option<&AccountInfo<'_>>) -> Result<Rent, ProgramError> {
    let account =
        account.ok_or_else(|| ProgramError::from(EncryptedErc20ProgramError::MissingRentSysvar))?;
    if account.key != &sysvar::rent::id() {
        return Err(EncryptedErc20ProgramError::MissingRentSysvar.into());
    }
    Rent::from_account_info(account)
        .map_err(|_| ProgramError::from(EncryptedErc20ProgramError::MissingRentSysvar))
}

fn scalar_word_from_u64(value: u64) -> [u8; 32] {
    let mut output = [0_u8; 32];
    output[24..].copy_from_slice(&value.to_be_bytes());
    output
}
