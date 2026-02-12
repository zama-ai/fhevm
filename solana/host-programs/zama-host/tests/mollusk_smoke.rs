use anchor_lang::{AnchorSerialize, Discriminator, InstructionData};
use mollusk_svm::{result::Check, Mollusk};
use solana_account::Account;
use solana_instruction::{error::InstructionError, AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_sdk_ids::system_program;
use zama_host::instruction::RequestAdd as RequestAddIxData;

const PROGRAM_ELF_NAME: &str = "zama_host";
const HCU_TX_LIMIT: u64 = 20_000_000;
const HCU_DEPTH_LIMIT: u64 = 5_000_000;

const HOST_ERROR_INVALID_HCU_AUTHORITY: u32 = 6002;
const HOST_ERROR_HCU_TX_LIMIT_EXCEEDED: u32 = 6005;
const HOST_ERROR_HCU_DEPTH_LIMIT_EXCEEDED: u32 = 6006;
const HOST_ERROR_HCU_TRACKED_HANDLE_LIMIT_EXCEEDED: u32 = 6008;

fn program_id() -> Pubkey {
    zama_host::id()
        .to_string()
        .parse()
        .expect("parse zama_host program id")
}

fn caller_account() -> Account {
    Account::new(1_000_000_000, 0, &system_program::id())
}

fn meter_account(program_id: &Pubkey, meter_state: zama_host::HcuMeter) -> Account {
    let mut meter_data = zama_host::HcuMeter::DISCRIMINATOR.to_vec();
    meter_data.extend_from_slice(&meter_state.try_to_vec().expect("serialize meter"));
    let meter_space = 8 + 32 + 16 + 8 + 4 + (128 * (32 + 8));
    let mut meter_account = Account::new(1_000_000_000, meter_space, program_id);
    meter_account.data[..meter_data.len()].copy_from_slice(&meter_data);
    meter_account
}

fn meter_state(authority: Pubkey) -> zama_host::HcuMeter {
    zama_host::HcuMeter {
        authority: anchor_lang::prelude::Pubkey::new_from_array(authority.to_bytes()),
        meter_id: [1u8; 16],
        tx_total_hcu: 0,
        tracked_handles: vec![],
    }
}

fn request_add_instruction(
    program_id: Pubkey,
    caller: Pubkey,
    meter: Pubkey,
    lhs: [u8; 32],
    rhs: [u8; 32],
    is_scalar: bool,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(caller, true),
            AccountMeta::new(meter, false),
        ],
        data: RequestAddIxData {
            lhs,
            rhs,
            is_scalar,
        }
        .data(),
    }
}

fn default_lhs_rhs() -> ([u8; 32], [u8; 32]) {
    ([7u8; 32], [11u8; 32])
}

fn expected_failure(code: u32) -> mollusk_svm::result::ProgramResult {
    Err(InstructionError::Custom(code)).into()
}

#[test]
fn request_add_smoke_succeeds() {
    let sbf_out_dir = format!("{}/../target/deploy", env!("CARGO_MANIFEST_DIR"));
    std::env::set_var("SBF_OUT_DIR", &sbf_out_dir);

    let program_id = program_id();
    let caller = Pubkey::new_unique();
    let (lhs, rhs) = default_lhs_rhs();
    let is_scalar = true;
    let meter = Pubkey::new_unique();
    let meter_state = meter_state(caller);
    let instruction = request_add_instruction(program_id, caller, meter, lhs, rhs, is_scalar);
    let accounts = vec![
        (caller, caller_account()),
        (meter, meter_account(&program_id, meter_state)),
    ];

    let mollusk = Mollusk::new(&program_id, PROGRAM_ELF_NAME);
    mollusk.process_and_validate_instruction(&instruction, &accounts, &[Check::success()]);
}

#[test]
fn request_add_reverts_on_invalid_hcu_authority() {
    let sbf_out_dir = format!("{}/../target/deploy", env!("CARGO_MANIFEST_DIR"));
    std::env::set_var("SBF_OUT_DIR", &sbf_out_dir);

    let program_id = program_id();
    let caller = Pubkey::new_unique();
    let meter = Pubkey::new_unique();
    let (lhs, rhs) = default_lhs_rhs();
    let instruction = request_add_instruction(program_id, caller, meter, lhs, rhs, true);

    let accounts = vec![
        (caller, caller_account()),
        (
            meter,
            meter_account(&program_id, meter_state(Pubkey::new_unique())),
        ),
    ];
    let mollusk = Mollusk::new(&program_id, PROGRAM_ELF_NAME);
    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::program_result(expected_failure(
            HOST_ERROR_INVALID_HCU_AUTHORITY,
        ))],
    );
}

#[test]
fn request_add_reverts_on_hcu_global_tx_limit() {
    let sbf_out_dir = format!("{}/../target/deploy", env!("CARGO_MANIFEST_DIR"));
    std::env::set_var("SBF_OUT_DIR", &sbf_out_dir);

    let program_id = program_id();
    let caller = Pubkey::new_unique();
    let meter = Pubkey::new_unique();
    let (lhs, rhs) = default_lhs_rhs();
    let instruction = request_add_instruction(program_id, caller, meter, lhs, rhs, true);

    let mut state = meter_state(caller);
    state.tx_total_hcu = HCU_TX_LIMIT - 10_000;

    let accounts = vec![
        (caller, caller_account()),
        (meter, meter_account(&program_id, state)),
    ];
    let mollusk = Mollusk::new(&program_id, PROGRAM_ELF_NAME);
    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::program_result(expected_failure(
            HOST_ERROR_HCU_TX_LIMIT_EXCEEDED,
        ))],
    );
}

#[test]
fn request_add_reverts_on_hcu_depth_limit() {
    let sbf_out_dir = format!("{}/../target/deploy", env!("CARGO_MANIFEST_DIR"));
    std::env::set_var("SBF_OUT_DIR", &sbf_out_dir);

    let program_id = program_id();
    let caller = Pubkey::new_unique();
    let meter = Pubkey::new_unique();
    let (lhs, rhs) = default_lhs_rhs();
    let instruction = request_add_instruction(program_id, caller, meter, lhs, rhs, true);

    let mut state = meter_state(caller);
    state.tracked_handles.push(zama_host::HandleDepth {
        handle: lhs,
        depth_hcu: HCU_DEPTH_LIMIT - 10_000,
    });

    let accounts = vec![
        (caller, caller_account()),
        (meter, meter_account(&program_id, state)),
    ];
    let mollusk = Mollusk::new(&program_id, PROGRAM_ELF_NAME);
    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::program_result(expected_failure(
            HOST_ERROR_HCU_DEPTH_LIMIT_EXCEEDED,
        ))],
    );
}

#[test]
fn request_add_reverts_on_tracked_handle_capacity_limit() {
    let sbf_out_dir = format!("{}/../target/deploy", env!("CARGO_MANIFEST_DIR"));
    std::env::set_var("SBF_OUT_DIR", &sbf_out_dir);

    let program_id = program_id();
    let caller = Pubkey::new_unique();
    let meter = Pubkey::new_unique();
    let (lhs, rhs) = default_lhs_rhs();
    let instruction = request_add_instruction(program_id, caller, meter, lhs, rhs, true);

    let mut state = meter_state(caller);
    state.tracked_handles = (0..128)
        .map(|i| {
            let mut handle = [0u8; 32];
            handle[0] = i as u8;
            zama_host::HandleDepth {
                handle,
                depth_hcu: 1,
            }
        })
        .collect();

    let accounts = vec![
        (caller, caller_account()),
        (meter, meter_account(&program_id, state)),
    ];
    let mollusk = Mollusk::new(&program_id, PROGRAM_ELF_NAME);
    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::program_result(expected_failure(
            HOST_ERROR_HCU_TRACKED_HANDLE_LIMIT_EXCEEDED,
        ))],
    );
}
