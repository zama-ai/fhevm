use mollusk_svm::{result::Check, Mollusk};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_sdk_ids::system_program;

const PROGRAM_ID_STR: &str = "Fg6PaFpoGXkYsidMpWxTWqkZ4FK6s7vY8J3xA5rJQbSq";
const PROGRAM_ELF_NAME: &str = "zama_host_v0";
const REQUEST_ADD_DISC: [u8; 8] = [0xFE, 0xFA, 0xE0, 0x73, 0x76, 0x3D, 0x2B, 0x98];
const REQUEST_ADD_CPI_DISC: [u8; 8] = [0xAF, 0x9F, 0xDA, 0x12, 0x9C, 0x82, 0xB3, 0x68];

fn ensure_sbf_out_dir() {
    if std::env::var("SBF_OUT_DIR").is_err() {
        std::env::set_var("SBF_OUT_DIR", "../../target/deploy");
    }
}

#[test]
fn request_add_smoke_succeeds() {
    ensure_sbf_out_dir();

    let program_id: Pubkey = PROGRAM_ID_STR.parse().expect("parse program id");
    let caller = Pubkey::new_unique();
    let lhs = [7u8; 32];
    let rhs = [11u8; 32];
    let is_scalar = true;

    let mut data = Vec::with_capacity(8 + 32 + 32 + 1);
    data.extend_from_slice(&REQUEST_ADD_DISC);
    data.extend_from_slice(&lhs);
    data.extend_from_slice(&rhs);
    data.push(u8::from(is_scalar));

    let instruction = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(caller, true)],
        data,
    };
    let accounts = vec![(caller, Account::new(1_000_000_000, 0, &system_program::id()))];

    let mollusk = Mollusk::new(&program_id, PROGRAM_ELF_NAME);
    mollusk.process_and_validate_instruction(&instruction, &accounts, &[Check::success()]);
}

#[test]
#[ignore = "self-CPI path returns UnsupportedProgramId in current Mollusk setup; validated via localnet integration tier"]
fn request_add_cpi_emits_one_inner_instruction() {
    ensure_sbf_out_dir();

    let program_id: Pubkey = PROGRAM_ID_STR.parse().expect("parse program id");
    let caller = Pubkey::new_unique();
    let event_authority = Pubkey::find_program_address(&[b"__event_authority"], &program_id).0;
    let lhs = [17u8; 32];
    let rhs = [23u8; 32];
    let is_scalar = false;

    let mut data = Vec::with_capacity(8 + 32 + 32 + 1);
    data.extend_from_slice(&REQUEST_ADD_CPI_DISC);
    data.extend_from_slice(&lhs);
    data.extend_from_slice(&rhs);
    data.push(u8::from(is_scalar));

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(caller, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program_id, false),
        ],
        data,
    };
    let accounts = vec![
        (caller, Account::new(1_000_000_000, 0, &system_program::id())),
        (event_authority, Account::new(1_000_000_000, 0, &system_program::id())),
        (program_id, Account::new(1_000_000_000, 0, &system_program::id())),
    ];

    let mut mollusk = Mollusk::new(&program_id, PROGRAM_ELF_NAME);
    // Register the same program for self-CPI event emission path (`emit_cpi!`).
    mollusk.add_program(&program_id, PROGRAM_ELF_NAME);
    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::success(), Check::inner_instruction_count(1)],
    );
}
