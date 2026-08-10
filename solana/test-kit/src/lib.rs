//! Shared Mollusk test fixtures for programs that build on `zama-host`.
//!
//! The kit holds only what is program-agnostic or `zama-host`-generic: the Mollusk environment,
//! Anchor instruction/account plumbing, the host's fixture accounts (`HostConfig`, `KmsContext`,
//! `EncryptedValue`, deny records), the coprocessor/KMS signature minting, the cleartext oracle
//! that replays `fhe_execute` CPIs, and the rolling cost snapshots. Program-specific fixtures
//! (a token's mints, a batcher's batches) stay with their suites.
//!
//! The kit deliberately depends on no program crate other than `zama-host`: each suite registers
//! its own programs on the `Mollusk` it gets from [`svm`], so a new consumer program can use the
//! kit without pulling in every program in the workspace.

pub mod cost_snapshot;
pub mod oracle;
pub mod signing;

pub mod contracts;

use std::collections::HashMap;
use std::path::PathBuf;

use anchor_lang::{
    prelude::system_program, AccountDeserialize, AccountSerialize, AnchorDeserialize,
    Discriminator, InstructionData, ToAccountMetas,
};
use anchor_spl::token::spl_token;
use mollusk_svm::{result::Check, Mollusk, MolluskContext};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
};
use zama_host as host;

/// The stateful Mollusk context every suite drives: an account store keyed by address.
pub type Ctx = MolluskContext<HashMap<Pubkey, Account>>;

// ---------------------------------------------------------------------------
// Fixture constants shared by every suite
// ---------------------------------------------------------------------------

/// The gateway chain the fixtures' host config points at.
pub const GATEWAY_CHAIN_ID: u64 = 31337;
/// EVM address of the fixtures' `InputVerification` contract (attestation domain separator).
pub const INPUT_VERIFICATION_CONTRACT: [u8; 20] = [0xCDu8; 20];
/// EVM address of the fixtures' `Decryption` contract (KMS cert domain separator).
pub const DECRYPTION_CONTRACT: [u8; 20] = [0xDEu8; 20];
/// FHE type tag of a `euint64` balance — the type every amount fixture uses.
pub const BALANCE_FHE_TYPE: u8 = 5;
/// Decimals of every fixture SPL mint.
pub const DECIMALS: u8 = 6;

// ---------------------------------------------------------------------------
// Mollusk environment
// ---------------------------------------------------------------------------

/// Boots a Mollusk with `program` registered, resolving SBF artifacts from
/// `solana/target/deploy` (build them with `bash scripts/check-zama-host-idl.sh` first; a stale
/// `.so` fails on an unrelated missing account rather than on anything a test asserts).
///
/// Suites register further programs on the returned value and raise the compute budget where
/// their instructions need it.
pub fn svm(program_id: &Pubkey, program_name: &str) -> Mollusk {
    // The kit sits directly under `solana/`, so `../target/deploy` from its manifest is the
    // workspace deploy dir; moving the crate deeper would silently break this path.
    static SET_SBF_OUT_DIR: std::sync::Once = std::sync::Once::new();
    SET_SBF_OUT_DIR.call_once(|| {
        let deploy_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy");
        // SAFETY: guarded by the Once, so no other thread can be mutating the environment
        // through this path concurrently; test binaries have no other env writers.
        unsafe {
            std::env::set_var("SBF_OUT_DIR", deploy_dir);
        }
    });
    Mollusk::new(program_id, program_name)
}

/// Runs the Mollusk at a nonzero slot with a `SlotHashes` entry below it, like a real
/// validator. `fhe_execute` derives handle entropy from the previous bank hash, so every suite
/// that executes it needs this; a suite with no host involvement skips it.
pub fn set_previous_bank_hash_sysvars(mollusk: &mut Mollusk) {
    mollusk.sysvars.clock.slot = 100;
    mollusk.sysvars.slot_hashes =
        solana_sdk::slot_hashes::SlotHashes::new(&[(99, solana_sdk::hash::Hash::new_unique())]);
}

/// [`svm`] for `zama-host` alone, with the previous-bank-hash sysvars in place.
pub fn host_svm() -> Mollusk {
    let mut mollusk = svm(&host::id(), "zama_host");
    set_previous_bank_hash_sysvars(&mut mollusk);
    mollusk
}

/// [`host_svm`] with an empty `SlotHashes` — for asserting the missing-previous-bank-hash
/// failure path.
pub fn host_svm_without_previous_bank_hash() -> Mollusk {
    let mut mollusk = host_svm();
    mollusk.sysvars.slot_hashes = solana_sdk::slot_hashes::SlotHashes::default();
    mollusk
}

// ---------------------------------------------------------------------------
// Anchor instruction and account plumbing
// ---------------------------------------------------------------------------

/// Builds an `Instruction` from an Anchor accounts struct and an Anchor args struct.
pub fn anchor_ix<A, D>(program_id: Pubkey, accounts: A, args: D) -> Instruction
where
    A: ToAccountMetas,
    D: InstructionData,
{
    Instruction {
        program_id,
        accounts: accounts.to_account_metas(None),
        data: args.data(),
    }
}

/// Serializes an Anchor account (discriminator + body) for direct account-map seeding.
pub fn serialized_account<T: AccountSerialize>(account: T) -> Vec<u8> {
    let mut data = Vec::new();
    account.try_serialize(&mut data).unwrap();
    data
}

/// `Check::err` for an Anchor `#[error_code]` variant: pass `error as u32` and the
/// `ERROR_CODE_OFFSET` is applied here. Anchor's own `ErrorCode` variants already carry their
/// absolute code — use [`anchor_framework_error_check`] for those.
pub fn anchor_error_check(error_code: u32) -> Check<'static> {
    Check::err(ProgramError::Custom(
        anchor_lang::error::ERROR_CODE_OFFSET + error_code,
    ))
}

/// `Check::err` for an `anchor_lang::error::ErrorCode` (framework-level, no offset).
pub fn anchor_framework_error_check(error: anchor_lang::error::ErrorCode) -> Check<'static> {
    Check::err(ProgramError::Custom(error as u32))
}

pub fn writable(pubkey: Pubkey) -> AccountMeta {
    AccountMeta::new(pubkey, false)
}

pub fn readonly(pubkey: Pubkey) -> AccountMeta {
    AccountMeta::new_readonly(pubkey, false)
}

pub fn readonly_signer(pubkey: Pubkey) -> AccountMeta {
    AccountMeta::new_readonly(pubkey, true)
}

/// The Anchor event-CPI authority PDA of a program.
pub fn event_authority(program_id: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"__event_authority"], &program_id).0
}

/// Decodes a self-CPI Anchor event from inner-instruction data.
pub fn decode_anchor_event<T>(data: &[u8]) -> Option<T>
where
    T: AnchorDeserialize + Discriminator,
{
    let event_prefix = anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(T::DISCRIMINATOR.iter().copied())
        .collect::<Vec<u8>>();
    let payload = data.strip_prefix(&event_prefix[..])?;
    T::deserialize(&mut &*payload).ok()
}

/// The 32-byte big-endian `uint256` encoding of a `u64` — scalar operands, plaintexts, and
/// KMS-signed cleartexts all carry this shape on the wire.
pub fn u256_be(value: u64) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[24..].copy_from_slice(&value.to_be_bytes());
    out
}

/// A left-aligned 32-byte label from a short name.
pub fn label(name: &str) -> [u8; 32] {
    let mut out = [0_u8; 32];
    let bytes = name.as_bytes();
    assert!(bytes.len() <= out.len());
    out[..bytes.len()].copy_from_slice(bytes);
    out
}

/// A canonical Solana handle: seed-filled, carrying the PoC chain id, FHE type, and version.
pub fn handle_for_chain(seed: u8, fhe_type: u8) -> [u8; 32] {
    let mut handle = [seed; 32];
    handle[21] = 0;
    handle[22..30].copy_from_slice(&host::SOLANA_POC_CHAIN_ID.to_be_bytes());
    handle[30] = fhe_type;
    handle[31] = host::HANDLE_VERSION;
    handle
}

// ---------------------------------------------------------------------------
// Plain accounts
// ---------------------------------------------------------------------------

/// A data-less system-owned account holding `lamports`.
pub fn system_account(lamports: u64) -> Account {
    Account {
        lamports,
        data: vec![],
        owner: system_program::ID,
        executable: false,
        rent_epoch: 0,
    }
}

/// [`system_account`] with generous funding for payer roles.
pub fn funded_system_account() -> Account {
    system_account(10_000_000_000)
}

/// [`system_account`] with zero lamports, for addresses an instruction will create.
pub fn empty_system_account() -> Account {
    system_account(0)
}

/// The system program's own executable account entry, for suites that seed it explicitly.
pub fn system_program_account() -> Account {
    Account {
        lamports: 1,
        data: b"system_program".to_vec(),
        owner: solana_sdk::native_loader::ID,
        executable: true,
        rent_epoch: 0,
    }
}

/// An initialized SPL mint at [`DECIMALS`] with the given authority and supply.
pub fn spl_mint_account(mint_authority: Option<Pubkey>, supply: u64) -> Account {
    let mut data = vec![0u8; spl_token::state::Mint::LEN];
    spl_token::state::Mint::pack(
        spl_token::state::Mint {
            mint_authority: mint_authority.map(COption::Some).unwrap_or(COption::None),
            supply,
            decimals: DECIMALS,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        &mut data,
    )
    .unwrap();
    Account {
        lamports: 1_000_000_000,
        data,
        owner: spl_token::id(),
        executable: false,
        rent_epoch: 0,
    }
}

/// An initialized SPL token account holding `amount` of `mint`.
pub fn spl_token_account(mint: Pubkey, owner: Pubkey, amount: u64) -> Account {
    let mut data = vec![0u8; spl_token::state::Account::LEN];
    spl_token::state::Account::pack(
        spl_token::state::Account {
            mint,
            owner,
            amount,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        &mut data,
    )
    .unwrap();
    Account {
        lamports: 1_000_000_000,
        data,
        owner: spl_token::id(),
        executable: false,
        rent_epoch: 0,
    }
}

// ---------------------------------------------------------------------------
// Context store access
// ---------------------------------------------------------------------------

/// Reads and deserializes an Anchor account from the context store.
pub fn read_account<T: AccountDeserialize>(context: &Ctx, address: Pubkey) -> T {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing account in context store")
        .clone();
    T::try_deserialize(&mut account.data.as_slice()).expect("account should deserialize")
}

/// Reads the canonical `EncryptedValue` at `address` from the context store.
pub fn read_encrypted_value(context: &Ctx, address: Pubkey) -> host::EncryptedValue {
    read_account(context, address)
}

/// Reads the `EncryptedValue` at `address` out of a stateless instruction result.
pub fn read_encrypted_value_from_result(
    result: &mollusk_svm::result::InstructionResult,
    address: Pubkey,
) -> host::EncryptedValue {
    let account = result
        .resulting_accounts
        .iter()
        .find(|(key, _)| *key == address)
        .map(|(_, account)| account)
        .expect("encrypted value account present in result");
    let mut data: &[u8] = &account.data;
    host::EncryptedValue::try_deserialize(&mut data).expect("valid EncryptedValue account")
}

/// Amount held by the token account at `address` — classic SPL Token or Token-2022, decided by
/// the account's owner.
pub fn read_spl_amount(context: &Ctx, address: Pubkey) -> u64 {
    let store = context.account_store.borrow();
    let account = store.get(&address).expect("missing spl token account");
    if account.owner == spl_token::id() {
        spl_token::state::Account::unpack(&account.data)
            .expect("valid classic token account")
            .amount
    } else {
        use anchor_spl::token_2022::spl_token_2022;
        use spl_token_2022::extension::StateWithExtensions;
        StateWithExtensions::<spl_token_2022::state::Account>::unpack(&account.data)
            .expect("valid Token-2022 account")
            .base
            .amount
    }
}

/// Supply of the SPL mint at `address`.
pub fn read_mint_supply(context: &Ctx, address: Pubkey) -> u64 {
    let store = context.account_store.borrow();
    let account = store.get(&address).expect("missing spl mint");
    spl_token::state::Mint::unpack(&account.data)
        .expect("valid spl mint")
        .supply
}

/// Inserts fresh, empty system accounts for addresses an instruction will
/// create (Mollusk requires every referenced account to exist in the store).
pub fn ensure_system_accounts(context: &Ctx, addresses: &[Pubkey]) {
    let mut store = context.account_store.borrow_mut();
    for address in addresses {
        store.entry(*address).or_insert_with(empty_system_account);
    }
}

/// True when `address` is absent or a data-less system-owned account (a closed account).
pub fn account_is_system_owned_and_empty(context: &Ctx, address: Pubkey) -> bool {
    match context.account_store.borrow().get(&address) {
        None => true,
        Some(account) => account.owner == system_program::ID && account.data.is_empty(),
    }
}

// ---------------------------------------------------------------------------
// zama-host fixture accounts
// ---------------------------------------------------------------------------

/// Parameters for a fixture `HostConfig`. Defaults describe the common single-signer,
/// unrestricted-HCU config; suites override only what a test varies.
pub struct HostConfigParams {
    pub admin: Pubkey,
    pub coprocessor_signers: Vec<[u8; 20]>,
    pub coprocessor_threshold: u8,
    pub current_kms_context_id: u64,
    pub paused: bool,
    pub grant_deny_list_enabled: bool,
}

impl HostConfigParams {
    /// Defaults register [`signing::coprocessor_signing_key`]'s EVM address as the sole
    /// coprocessor signer, so a config built from these params accepts attestations minted by
    /// [`signing::amount_attestation_for`] without further setup.
    pub fn new(admin: Pubkey) -> Self {
        Self {
            admin,
            coprocessor_signers: vec![signing::secp_evm_address(
                &signing::coprocessor_signing_key(),
            )],
            coprocessor_threshold: 1,
            current_kms_context_id: 0,
            paused: false,
            grant_deny_list_enabled: false,
        }
    }
}

/// Builds the fixture `HostConfig` account at its canonical PDA.
pub fn host_config_account(params: &HostConfigParams) -> (Pubkey, Account) {
    let (host_config, bump) = host::host_config_address();
    (
        host_config,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(host::HostConfig {
                admin: params.admin,
                chain_id: host::SOLANA_POC_CHAIN_ID,
                gateway_chain_id: GATEWAY_CHAIN_ID,
                input_verification_contract: INPUT_VERIFICATION_CONTRACT,
                coprocessor_signers: host::pack_coprocessor_signers(&params.coprocessor_signers),
                coprocessor_signer_count: params.coprocessor_signers.len() as u8,
                coprocessor_threshold: params.coprocessor_threshold,
                decryption_contract: DECRYPTION_CONTRACT,
                current_kms_context_id: params.current_kms_context_id,
                paused: params.paused,
                grant_deny_list_enabled: params.grant_deny_list_enabled,
                max_hcu_per_tx: u64::MAX,
                max_hcu_depth_per_tx: u64::MAX,
                hcu_block_cap_per_app: u64::MAX,
                updated_slot: 0,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

/// Builds a fixture `KmsContext` account at its canonical PDA, with every threshold set to
/// `threshold`.
pub fn kms_context_account(
    context_id: u64,
    signers: Vec<[u8; 20]>,
    threshold: u8,
) -> (Pubkey, Account) {
    let (address, bump) = host::kms_context_address(context_id);
    (
        address,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(host::KmsContext {
                context_id,
                signers,
                thresholds: host::KmsThresholds {
                    public_decryption: threshold,
                    user_decryption: threshold,
                    kms_gen: threshold,
                    mpc: threshold,
                },
                destroyed: false,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

/// Builds a `DenySubjectRecord` account at its canonical PDA.
pub fn deny_subject_record_account(subject: Pubkey, denied: bool) -> (Pubkey, Account) {
    let (record, bump) = host::deny_subject_address(subject);
    (
        record,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(host::DenySubjectRecord {
                subject,
                denied,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

/// Builds a canonical `EncryptedValue` at the PDA derived from
/// `(domain, encrypted_value_account_authority, label)`.
pub fn new_encrypted_value(
    domain: Pubkey,
    encrypted_value_account_authority: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subjects: &[Pubkey],
) -> (Pubkey, host::EncryptedValue) {
    let encrypted_value_id = zama_solana_acl::derive_encrypted_value_id(
        domain.to_bytes(),
        encrypted_value_account_authority.to_bytes(),
        encrypted_value_label,
    );
    let (address, bump) = host::encrypted_value_address(encrypted_value_id);
    let value = host::EncryptedValue {
        domain,
        encrypted_value_account_authority,
        label: encrypted_value_label,
        current_handle: handle,
        subjects: subjects.to_vec(),
        leaf_count: 0,
        peaks: Vec::new(),
        bump,
    };
    (address, value)
}

/// Wraps an `EncryptedValue` into an account entry for direct account-map seeding.
pub fn encrypted_value_account(value: &host::EncryptedValue) -> Account {
    Account {
        lamports: 10_000_000_000,
        data: serialized_account(value.clone()),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

// ---------------------------------------------------------------------------
// fhe_execute wire decoding
// ---------------------------------------------------------------------------

/// Decodes `FheExecuteArgs` out of raw `fhe_execute` instruction data, `None` for any other
/// instruction.
pub(crate) fn decode_fhe_execute_args(data: &[u8]) -> Option<host::FheExecuteArgs> {
    let payload = data.strip_prefix(host::instruction::FheExecute::DISCRIMINATOR)?;
    host::FheExecuteArgs::deserialize(&mut &*payload).ok()
}

/// The output descriptor of any execution step variant.
pub(crate) fn execution_step_output(step: &host::FheExecuteStep) -> &host::FheExecuteOutput {
    match step {
        host::FheExecuteStep::Binary { output, .. }
        | host::FheExecuteStep::Ternary { output, .. }
        | host::FheExecuteStep::TrivialEncrypt { output, .. }
        | host::FheExecuteStep::Rand { output, .. }
        | host::FheExecuteStep::Unary { output, .. }
        | host::FheExecuteStep::RandBounded { output, .. }
        | host::FheExecuteStep::Sum { output, .. }
        | host::FheExecuteStep::IsIn { output, .. }
        | host::FheExecuteStep::MulDiv { output, .. } => output,
    }
}
