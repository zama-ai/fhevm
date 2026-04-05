use bincode::deserialize;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed, set_return_data},
    program_error::ProgramError,
    pubkey::Pubkey as SolanaPubkey,
    rent::Rent,
    sysvar,
    sysvar::SysvarSerialize,
};
use solana_slot_hashes::SlotHashes;
use solana_system_interface::{instruction as system_instruction, program as system_program};
use std::io::{Cursor, Write};

use crate::events::HostEvent;
use crate::instructions::HostProgramConfig;
use crate::program::{HostProgramSession, HostProgramState};
use crate::secp256k1_verifier::Secp256k1ProofVerifier;
use crate::types::Pubkey;
use crate::{HostInstruction, SESSION_PDA_SEED, STATE_PDA_SEED};

pub use crate::{find_session_pda, find_state_pda, OnchainInstruction};

const STATE_ACCOUNT_DISCRIMINATOR: [u8; 8] = *b"FHEHOST0";
const STATE_ACCOUNT_LAYOUT_VERSION: u32 = 1;
const SESSION_ACCOUNT_DISCRIMINATOR: [u8; 8] = *b"FHESESS0";
const SESSION_ACCOUNT_LAYOUT_VERSION: u32 = 1;
const MAX_BATCH_INSTRUCTIONS: usize = 64;
const BORSH_EMPTY_RESULT_VEC: [u8; 4] = 0u32.to_le_bytes();
// Inner-instruction account reallocations on Solana are capped at 10 KiB, so
// reserve room up front but keep the initial allocation below that limit.
const STATE_ACCOUNT_RESERVE_BYTES: usize = 8 * 1024;
const SESSION_ACCOUNT_RESERVE_BYTES: usize = 2 * 1024;

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
struct StoredHostProgramState {
    discriminator: [u8; 8],
    layout_version: u32,
    state: HostProgramState,
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
struct StoredHostProgramSession {
    discriminator: [u8; 8],
    layout_version: u32,
    caller: Pubkey,
    recent_blockhash: [u8; 32],
    session_nonce: u64,
    session: HostProgramSession,
}

impl StoredHostProgramState {
    fn new(state: HostProgramState) -> Self {
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
            return Err(SolanaHostProgramError::InvalidStateLayout.into());
        }
        Ok(())
    }

    fn serialized_len(state: &HostProgramState) -> Result<usize, ProgramError> {
        serialized_len(&Self::new(state.clone()))
    }
}

impl StoredHostProgramSession {
    fn new(
        caller: Pubkey,
        recent_blockhash: [u8; 32],
        session_nonce: u64,
        session: HostProgramSession,
    ) -> Self {
        Self {
            discriminator: SESSION_ACCOUNT_DISCRIMINATOR,
            layout_version: SESSION_ACCOUNT_LAYOUT_VERSION,
            caller,
            recent_blockhash,
            session_nonce,
            session,
        }
    }

    fn validate(&self) -> ProgramResult {
        if self.discriminator != SESSION_ACCOUNT_DISCRIMINATOR
            || self.layout_version != SESSION_ACCOUNT_LAYOUT_VERSION
        {
            return Err(SolanaHostProgramError::InvalidSessionAccount.into());
        }
        Ok(())
    }

    fn reset(&mut self, caller: Pubkey, recent_blockhash: [u8; 32], session_nonce: u64) {
        self.caller = caller;
        self.recent_blockhash = recent_blockhash;
        self.session_nonce = session_nonce;
        self.session.reset();
    }
}

#[repr(u32)]
enum SolanaHostProgramError {
    InvalidInstructionData = 1,
    MissingRequiredSignature = 2,
    InvalidStateAccount = 3,
    StateAlreadyInitialized = 4,
    StateNotInitialized = 5,
    SerializationFailure = 6,
    InvalidClockSysvar = 7,
    InvalidStateLayout = 8,
    InvalidInitializer = 9,
    InvalidBatchSize = 10,
    MissingSystemProgram = 11,
    MissingRentSysvar = 12,
    MissingSlotHashesSysvar = 13,
    UnavailableRecentBlockhash = 14,
    InvalidSessionAccount = 15,
    InvalidSlotHashesKey = 16,
    InvalidSlotHashesData = 17,
}

impl From<SolanaHostProgramError> for ProgramError {
    fn from(value: SolanaHostProgramError) -> Self {
        ProgramError::Custom(value as u32)
    }
}

fn program_error_from_host(value: crate::HostContractError) -> ProgramError {
    msg!("host logic error: {}", value);
    ProgramError::Custom(SolanaHostProgramError::InvalidInstructionData as u32)
}

pub fn id() -> SolanaPubkey {
    SolanaPubkey::new_from_array([0x42; 32])
}

pub fn process_instruction<'a, 'b>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'b>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = OnchainInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::from(SolanaHostProgramError::InvalidInstructionData))?;

    match instruction {
        OnchainInstruction::Initialize { config } => initialize(program_id, accounts, config),
        OnchainInstruction::InitializePda { config } => {
            initialize_pda(program_id, accounts, config)
        }
        OnchainInstruction::Execute {
            instruction,
            session_nonce,
            recent_blockhash: _,
        } => execute_batch(program_id, accounts, vec![instruction], session_nonce),
        OnchainInstruction::ExecuteBatch {
            instructions,
            session_nonce,
            recent_blockhash: _,
        } => execute_batch(program_id, accounts, instructions, session_nonce),
    }
}

pub fn encode_instruction(instruction: &OnchainInstruction) -> Result<Vec<u8>, ProgramError> {
    borsh::to_vec(instruction)
        .map_err(|_| ProgramError::from(SolanaHostProgramError::SerializationFailure))
}

pub fn decode_state(data: &[u8]) -> Result<HostProgramState, ProgramError> {
    let mut slice = data;
    let stored = StoredHostProgramState::deserialize(&mut slice)
        .map_err(|_| ProgramError::from(SolanaHostProgramError::SerializationFailure))?;
    stored.validate()?;
    Ok(stored.state)
}

pub fn required_state_account_len(config: &HostProgramConfig) -> Result<usize, ProgramError> {
    let state = HostProgramState::new(config.clone()).map_err(program_error_from_host)?;
    StoredHostProgramState::serialized_len(&state)
}

pub fn required_session_account_len() -> Result<usize, ProgramError> {
    serialized_len(&StoredHostProgramSession::new(
        Pubkey::ZERO,
        [0; 32],
        0,
        HostProgramSession::default(),
    ))
}

pub fn state_account_len_with_reserve(
    config: &HostProgramConfig,
    reserve_bytes: usize,
) -> Result<usize, ProgramError> {
    Ok(required_state_account_len(config)? + reserve_bytes)
}

fn initialize<'a, 'b>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'b>],
    config: HostProgramConfig,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let authority = next_account_info(account_iter)?;
    let state_account = next_account_info(account_iter)?;
    let system_program_account = account_iter.next();
    let rent_sysvar_account = account_iter.next();

    ensure_signer(authority)?;
    ensure_state_account_owner(program_id, state_account)?;

    if Pubkey::from(authority.key) != config.owner {
        return Err(SolanaHostProgramError::InvalidInitializer.into());
    }

    initialize_state_account(
        state_account,
        config,
        Some(authority),
        system_program_account,
        rent_sysvar_account,
    )
}

fn initialize_pda<'a, 'b>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'b>],
    config: HostProgramConfig,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let authority = next_account_info(account_iter)?;
    let state_account = next_account_info(account_iter)?;
    let system_program_account = account_iter.next();
    let rent_sysvar_account = account_iter.next();

    ensure_signer(authority)?;

    if Pubkey::from(authority.key) != config.owner {
        return Err(SolanaHostProgramError::InvalidInitializer.into());
    }

    let (expected_pda, _) = find_state_pda(program_id);
    if state_account.key != &expected_pda {
        return Err(SolanaHostProgramError::InvalidStateAccount.into());
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
            &config,
        )?;
    } else {
        return Err(SolanaHostProgramError::InvalidStateAccount.into());
    }

    initialize_state_account(
        state_account,
        config,
        Some(authority),
        system_program_account,
        rent_sysvar_account,
    )
}

fn initialize_state_account<'a>(
    state_account: &AccountInfo<'a>,
    config: HostProgramConfig,
    payer: Option<&AccountInfo<'a>>,
    system_program_account: Option<&AccountInfo<'a>>,
    rent_sysvar_account: Option<&AccountInfo<'a>>,
) -> ProgramResult {
    if load_state_account(state_account)?.is_some() {
        return Err(SolanaHostProgramError::StateAlreadyInitialized.into());
    }

    let state = HostProgramState::new(config).map_err(program_error_from_host)?;
    save_state_account(
        state_account,
        &StoredHostProgramState::new(state),
        payer,
        system_program_account,
        rent_sysvar_account,
    )?;
    set_return_data(&[]);
    Ok(())
}

fn execute_batch<'a, 'b>(
    program_id: &SolanaPubkey,
    accounts: &'a [AccountInfo<'b>],
    instructions: Vec<HostInstruction>,
    session_nonce: u64,
) -> ProgramResult {
    if instructions.is_empty() || instructions.len() > MAX_BATCH_INSTRUCTIONS {
        return Err(SolanaHostProgramError::InvalidBatchSize.into());
    }
    if accounts.len() != 7 && accounts.len() != 8 {
        return Err(SolanaHostProgramError::InvalidInstructionData.into());
    }

    let account_iter = &mut accounts.iter();
    let authority = next_account_info(account_iter)?;
    let (fee_payer, state_account) = if accounts.len() == 8 {
        let fee_payer = next_account_info(account_iter)?;
        ensure_signer(fee_payer)?;
        (fee_payer, next_account_info(account_iter)?)
    } else {
        (authority, next_account_info(account_iter)?)
    };
    let session_account = next_account_info(account_iter)?;
    let clock_account = next_account_info(account_iter)?;
    let slot_hashes_account = account_iter
        .next()
        .ok_or_else(|| ProgramError::from(SolanaHostProgramError::MissingSlotHashesSysvar))?;
    let system_program_account = account_iter.next();
    let rent_sysvar_account = account_iter.next();

    ensure_signer(authority)?;
    ensure_state_account_owner(program_id, state_account)?;

    if clock_account.key != &sysvar::clock::id() {
        return Err(SolanaHostProgramError::InvalidClockSysvar.into());
    }

    let clock = Clock::from_account_info(clock_account)
        .map_err(|_| ProgramError::from(SolanaHostProgramError::InvalidClockSysvar))?;
    let recent_blockhash = load_recent_blockhash(slot_hashes_account, clock.slot)?;

    let mut stored_state = load_state_account(state_account)?
        .ok_or_else(|| ProgramError::from(SolanaHostProgramError::StateNotInitialized))?;

    let caller = Pubkey::from(authority.key);
    let mut stored_session = load_or_initialize_session_account(
        program_id,
        authority,
        fee_payer,
        session_account,
        recent_blockhash,
        session_nonce,
        system_program_account,
        rent_sysvar_account,
    )?;
    let mut session = HostProgramSession::default();
    std::mem::swap(&mut session, &mut stored_session.session);
    let mut results = Vec::with_capacity(instructions.len());
    let proof_verifier = Secp256k1ProofVerifier;

    for instruction in &instructions {
        let result = stored_state
            .state
            .process_instruction(
                instruction,
                crate::ProgramContext {
                    caller,
                    chain_id: stored_state.state.host_chain_id(),
                    slot: clock.slot,
                    timestamp: clock.unix_timestamp,
                    recent_blockhash,
                },
                &mut session,
                &proof_verifier,
                &proof_verifier,
            )
            .map_err(program_error_from_host)?;
        emit_events(&result.events)?;
        results.push(result);
    }

    save_state_account(
        state_account,
        &stored_state,
        Some(fee_payer),
        system_program_account,
        rent_sysvar_account,
    )?;
    stored_session.session = session;
    save_session_account(
        session_account,
        &stored_session,
        Some(fee_payer),
        system_program_account,
        rent_sysvar_account,
    )?;
    // App programs only read returned handles / verification bits from the
    // host return data. Re-serializing the full emitted event payloads here
    // duplicates the same data we already logged and can exhaust Solana's
    // tight heap budget on multi-instruction flows.
    if results
        .iter()
        .all(|result| result.returned_handle.is_none() && result.verified.is_none())
    {
        set_return_data(&BORSH_EMPTY_RESULT_VEC);
    } else {
        let compact_results: Vec<crate::InstructionResult> = results
            .into_iter()
            .map(|result| crate::InstructionResult {
                events: Vec::new(),
                returned_handle: result.returned_handle,
                verified: result.verified,
            })
            .collect();
        let return_data = borsh::to_vec(&compact_results)
            .map_err(|_| ProgramError::from(SolanaHostProgramError::SerializationFailure))?;
        set_return_data(&return_data);
    }
    Ok(())
}

fn emit_events(events: &[HostEvent]) -> ProgramResult {
    for event in events {
        let payload = borsh::to_vec(event)
            .map_err(|_| ProgramError::from(SolanaHostProgramError::SerializationFailure))?;
        // Emit a single canonical log line per host event. Duplicating the same
        // payload with both `sol_log_data` and `msg!` makes long Solana
        // transactions much more likely to hit log truncation, which can drop
        // trailing allow/decryption events before the listener sees them.
        msg!("HOST_EVENT:{}", encode_hex(&payload));
    }
    Ok(())
}

fn load_state_account(
    account: &AccountInfo<'_>,
) -> Result<Option<StoredHostProgramState>, ProgramError> {
    let data = account.try_borrow_data()?;
    if data.is_empty() || data.iter().all(|byte| *byte == 0) {
        return Ok(None);
    }

    let mut slice: &[u8] = &data;
    let stored = StoredHostProgramState::deserialize(&mut slice)
        .map_err(|_| ProgramError::from(SolanaHostProgramError::SerializationFailure))?;
    stored.validate()?;
    Ok(Some(stored))
}

fn load_session_account(
    account: &AccountInfo<'_>,
) -> Result<Option<StoredHostProgramSession>, ProgramError> {
    let data = account.try_borrow_data()?;
    if data.is_empty() || data.iter().all(|byte| *byte == 0) {
        return Ok(None);
    }

    let mut slice: &[u8] = &data;
    let stored = StoredHostProgramSession::deserialize(&mut slice)
        .map_err(|_| ProgramError::from(SolanaHostProgramError::SerializationFailure))?;
    stored.validate()?;
    Ok(Some(stored))
}

fn save_state_account<'a>(
    account: &AccountInfo<'a>,
    state: &StoredHostProgramState,
    payer: Option<&AccountInfo<'a>>,
    system_program_account: Option<&AccountInfo<'a>>,
    rent_sysvar_account: Option<&AccountInfo<'a>>,
) -> ProgramResult {
    let serialized_len = serialized_len(state)?;

    if serialized_len > account.data_len() {
        msg!("phase: resizing state account");
        resize_state_account(
            account,
            serialized_len.saturating_add(STATE_ACCOUNT_RESERVE_BYTES),
            payer,
            system_program_account,
            rent_sysvar_account,
        )?;
    }

    let mut data = account.try_borrow_mut_data()?;
    data.fill(0);
    state
        .serialize(&mut Cursor::new(&mut data[..serialized_len]))
        .map_err(|_| ProgramError::from(SolanaHostProgramError::SerializationFailure))?;
    Ok(())
}

fn save_session_account<'a>(
    account: &AccountInfo<'a>,
    state: &StoredHostProgramSession,
    payer: Option<&AccountInfo<'a>>,
    system_program_account: Option<&AccountInfo<'a>>,
    rent_sysvar_account: Option<&AccountInfo<'a>>,
) -> ProgramResult {
    let serialized_len = serialized_len(state)?;

    if serialized_len > account.data_len() {
        msg!("phase: resizing session account");
        resize_session_account(
            account,
            serialized_len.saturating_add(SESSION_ACCOUNT_RESERVE_BYTES),
            payer,
            system_program_account,
            rent_sysvar_account,
        )?;
    }

    let mut data = account.try_borrow_mut_data()?;
    data.fill(0);
    state
        .serialize(&mut Cursor::new(&mut data[..serialized_len]))
        .map_err(|_| ProgramError::from(SolanaHostProgramError::SerializationFailure))?;
    Ok(())
}

#[derive(Default)]
struct CountingWriter {
    len: usize,
}

impl Write for CountingWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.len = self.len.saturating_add(buf.len());
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn serialized_len<T: BorshSerialize>(value: &T) -> Result<usize, ProgramError> {
    let mut writer = CountingWriter::default();
    value
        .serialize(&mut writer)
        .map_err(|_| ProgramError::from(SolanaHostProgramError::SerializationFailure))?;
    Ok(writer.len)
}

fn ensure_signer(account: &AccountInfo<'_>) -> ProgramResult {
    if !account.is_signer {
        return Err(SolanaHostProgramError::MissingRequiredSignature.into());
    }
    Ok(())
}

fn ensure_state_account_owner(
    program_id: &SolanaPubkey,
    account: &AccountInfo<'_>,
) -> ProgramResult {
    if account.owner != program_id {
        return Err(SolanaHostProgramError::InvalidStateAccount.into());
    }
    ensure_state_account_writable(account)
}

fn ensure_state_account_writable(account: &AccountInfo<'_>) -> ProgramResult {
    if !account.is_writable {
        return Err(SolanaHostProgramError::InvalidStateAccount.into());
    }
    if account.executable {
        return Err(SolanaHostProgramError::InvalidStateAccount.into());
    }
    Ok(())
}

fn ensure_session_account_owner(
    program_id: &SolanaPubkey,
    account: &AccountInfo<'_>,
) -> ProgramResult {
    if account.owner != program_id {
        return Err(SolanaHostProgramError::InvalidSessionAccount.into());
    }
    ensure_session_account_writable(account)
}

fn ensure_session_account_writable(account: &AccountInfo<'_>) -> ProgramResult {
    if !account.is_writable || account.executable {
        return Err(SolanaHostProgramError::InvalidSessionAccount.into());
    }
    Ok(())
}

fn ensure_system_program_account<'a, 'b>(
    account: Option<&'a AccountInfo<'b>>,
) -> Result<&'a AccountInfo<'b>, ProgramError> {
    let account =
        account.ok_or_else(|| ProgramError::from(SolanaHostProgramError::MissingSystemProgram))?;
    if !system_program::check_id(account.key) {
        return Err(SolanaHostProgramError::MissingSystemProgram.into());
    }
    Ok(account)
}

fn load_rent(rent_sysvar_account: Option<&AccountInfo<'_>>) -> Result<Rent, ProgramError> {
    let rent_sysvar_account = rent_sysvar_account
        .ok_or_else(|| ProgramError::from(SolanaHostProgramError::MissingRentSysvar))?;
    if rent_sysvar_account.key != &sysvar::rent::id() {
        return Err(SolanaHostProgramError::MissingRentSysvar.into());
    }
    Rent::from_account_info(rent_sysvar_account)
        .map_err(|_| ProgramError::from(SolanaHostProgramError::MissingRentSysvar))
}

fn load_recent_blockhash(
    slot_hashes_account: &AccountInfo<'_>,
    current_slot: u64,
) -> Result<[u8; 32], ProgramError> {
    if slot_hashes_account.key != &sysvar::slot_hashes::id() {
        return Err(SolanaHostProgramError::InvalidSlotHashesKey.into());
    }

    let data = slot_hashes_account.try_borrow_data()?;
    if data.len() < 8 {
        return Err(SolanaHostProgramError::InvalidSlotHashesData.into());
    }
    let count_bytes: [u8; 8] = data[..8]
        .try_into()
        .map_err(|_| ProgramError::from(SolanaHostProgramError::InvalidSlotHashesData))?;
    let entry_count = u64::from_le_bytes(count_bytes) as usize;
    let entries = data
        .get(8..)
        .ok_or_else(|| ProgramError::from(SolanaHostProgramError::InvalidSlotHashesData))?;

    for chunk in entries.chunks_exact(40).take(entry_count) {
        let slot = u64::from_le_bytes(
            chunk[..8]
                .try_into()
                .map_err(|_| ProgramError::from(SolanaHostProgramError::InvalidSlotHashesData))?,
        );
        if slot < current_slot {
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&chunk[8..40]);
            return Ok(hash);
        }
    }

    // Fallback for non-runtime test fixtures that still use bincode-serialized SlotHashes.
    let slot_hashes: SlotHashes = deserialize(data.as_ref())
        .map_err(|_| ProgramError::from(SolanaHostProgramError::InvalidSlotHashesData))?;
    slot_hashes
        .iter()
        .find(|(slot, _)| *slot < current_slot)
        .map(|(_, hash)| hash.to_bytes())
        .ok_or_else(|| ProgramError::from(SolanaHostProgramError::UnavailableRecentBlockhash))
}

fn load_or_initialize_session_account<'a>(
    program_id: &SolanaPubkey,
    authority: &AccountInfo<'a>,
    fee_payer: &AccountInfo<'a>,
    session_account: &AccountInfo<'a>,
    recent_blockhash: [u8; 32],
    session_nonce: u64,
    system_program_account: Option<&AccountInfo<'a>>,
    rent_sysvar_account: Option<&AccountInfo<'a>>,
) -> Result<StoredHostProgramSession, ProgramError> {
    let (expected_pda, _) = find_session_pda(program_id, authority.key);
    if session_account.key != &expected_pda {
        return Err(SolanaHostProgramError::InvalidSessionAccount.into());
    }
    ensure_session_account_writable(session_account)?;

    let caller = Pubkey::from(authority.key);
    let mut stored = if session_account.owner == program_id {
        ensure_session_account_owner(program_id, session_account)?;
        load_session_account(session_account)?.unwrap_or_else(|| {
            StoredHostProgramSession::new(
                caller,
                recent_blockhash,
                session_nonce,
                HostProgramSession::default(),
            )
        })
    } else if system_program::check_id(session_account.owner)
        && session_account.lamports() == 0
        && session_account.data_is_empty()
    {
        create_pda_session_account(
            program_id,
            authority,
            fee_payer,
            session_account,
            system_program_account,
            rent_sysvar_account,
        )?;
        StoredHostProgramSession::new(
            caller,
            recent_blockhash,
            session_nonce,
            HostProgramSession::default(),
        )
    } else {
        return Err(SolanaHostProgramError::InvalidSessionAccount.into());
    };

    if stored.caller != caller
        || stored.recent_blockhash != recent_blockhash
        || stored.session_nonce != session_nonce
    {
        stored.reset(caller, recent_blockhash, session_nonce);
    }

    Ok(stored)
}

fn create_pda_state_account<'a>(
    program_id: &SolanaPubkey,
    payer: &AccountInfo<'a>,
    state_account: &AccountInfo<'a>,
    system_program_account: Option<&AccountInfo<'a>>,
    rent_sysvar_account: Option<&AccountInfo<'a>>,
    config: &HostProgramConfig,
) -> ProgramResult {
    let system_program_account = ensure_system_program_account(system_program_account)?;
    let required_len = state_account_len_with_reserve(config, STATE_ACCOUNT_RESERVE_BYTES)?;
    let lamports = load_rent(rent_sysvar_account)?.minimum_balance(required_len);
    let (_, bump) = find_state_pda(program_id);
    let bump_seed = [bump];
    let signer_seeds: &[&[u8]] = &[STATE_PDA_SEED, &bump_seed];

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

fn create_pda_session_account<'a>(
    program_id: &SolanaPubkey,
    caller: &AccountInfo<'a>,
    fee_payer: &AccountInfo<'a>,
    session_account: &AccountInfo<'a>,
    system_program_account: Option<&AccountInfo<'a>>,
    rent_sysvar_account: Option<&AccountInfo<'a>>,
) -> ProgramResult {
    let system_program_account = ensure_system_program_account(system_program_account)?;
    let required_len =
        required_session_account_len()?.saturating_add(SESSION_ACCOUNT_RESERVE_BYTES);
    let lamports = load_rent(rent_sysvar_account)?.minimum_balance(required_len);
    let (_, bump) = find_session_pda(program_id, caller.key);
    let bump_seed = [bump];
    let signer_seeds: &[&[u8]] = &[SESSION_PDA_SEED, caller.key.as_ref(), &bump_seed];

    invoke_signed(
        &system_instruction::create_account(
            fee_payer.key,
            session_account.key,
            lamports,
            required_len as u64,
            program_id,
        ),
        &[
            fee_payer.clone(),
            session_account.clone(),
            system_program_account.clone(),
        ],
        &[signer_seeds],
    )
}

fn resize_state_account<'a>(
    state_account: &AccountInfo<'a>,
    new_len: usize,
    payer: Option<&AccountInfo<'a>>,
    system_program_account: Option<&AccountInfo<'a>>,
    rent_sysvar_account: Option<&AccountInfo<'a>>,
) -> ProgramResult {
    let payer =
        payer.ok_or_else(|| ProgramError::from(SolanaHostProgramError::InvalidStateAccount))?;
    let system_program_account = ensure_system_program_account(system_program_account)?;
    let rent = load_rent(rent_sysvar_account)?;
    let required_lamports = rent
        .minimum_balance(new_len)
        .saturating_sub(state_account.lamports());

    if required_lamports > 0 {
        invoke(
            &system_instruction::transfer(payer.key, state_account.key, required_lamports),
            &[
                payer.clone(),
                state_account.clone(),
                system_program_account.clone(),
            ],
        )?;
    }

    state_account.resize(new_len)
}

fn resize_session_account<'a>(
    session_account: &AccountInfo<'a>,
    new_len: usize,
    payer: Option<&AccountInfo<'a>>,
    system_program_account: Option<&AccountInfo<'a>>,
    rent_sysvar_account: Option<&AccountInfo<'a>>,
) -> ProgramResult {
    let payer =
        payer.ok_or_else(|| ProgramError::from(SolanaHostProgramError::InvalidSessionAccount))?;
    let system_program_account = ensure_system_program_account(system_program_account)?;
    let rent = load_rent(rent_sysvar_account)?;
    let required_lamports = rent
        .minimum_balance(new_len)
        .saturating_sub(session_account.lamports());

    if required_lamports > 0 {
        invoke(
            &system_instruction::transfer(payer.key, session_account.key, required_lamports),
            &[
                payer.clone(),
                session_account.clone(),
                system_program_account.clone(),
            ],
        )?;
    }

    session_account.resize(new_len)
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}
