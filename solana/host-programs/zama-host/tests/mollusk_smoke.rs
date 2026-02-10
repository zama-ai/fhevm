use anchor_lang::InstructionData;
use mollusk_svm::{result::Check, Mollusk};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_sdk_ids::system_program;
use zama_host::instruction::RequestAdd as RequestAddIxData;

const PROGRAM_ELF_NAME: &str = "zama_host";

#[test]
fn request_add_smoke_succeeds() {
    let sbf_out_dir = format!("{}/../target/deploy", env!("CARGO_MANIFEST_DIR"));
    std::env::set_var("SBF_OUT_DIR", &sbf_out_dir);

    let program_id: Pubkey = zama_host::id()
        .to_string()
        .parse()
        .expect("parse zama_host program id");
    let caller = Pubkey::new_unique();
    let lhs = [7u8; 32];
    let rhs = [11u8; 32];
    let is_scalar = true;

    let instruction = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(caller, true)],
        data: RequestAddIxData {
            lhs,
            rhs,
            is_scalar,
        }
        .data(),
    };
    let accounts = vec![(
        caller,
        Account::new(1_000_000_000, 0, &system_program::id()),
    )];

    let mollusk = Mollusk::new(&program_id, PROGRAM_ELF_NAME);
    mollusk.process_and_validate_instruction(&instruction, &accounts, &[Check::success()]);
}
