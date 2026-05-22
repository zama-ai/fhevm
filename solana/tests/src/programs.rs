use std::path::PathBuf;

use anchor_litesvm::AnchorLiteSVM;
use litesvm::LiteSVM;
use solana_sdk::pubkey::Pubkey;

use crate::util::set_previous_slot_hash;
use crate::util::DEFAULT_TEST_PREVIOUS_BANK_HASH;

pub fn host_program_so_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy/zama_host.so")
}

pub fn token_program_so_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy/confidential_token.so")
}

pub fn svm_with_program(program_id: Pubkey, program_path: PathBuf) -> LiteSVM {
    svm_with_programs(&[(program_id, program_path)])
}

pub fn svm_with_programs(programs: &[(Pubkey, PathBuf)]) -> LiteSVM {
    for (_, path) in programs {
        assert!(
            path.exists(),
            "missing {}; run `cd solana && NO_DNA=1 anchor build --ignore-keys` before this runtime test",
            path.display()
        );
    }

    let program_bytes = programs
        .iter()
        .map(|(program_id, path)| (*program_id, std::fs::read(path).unwrap()))
        .collect::<Vec<_>>();
    let programs = program_bytes
        .iter()
        .map(|(program_id, bytes)| (*program_id, bytes.as_slice()))
        .collect::<Vec<_>>();
    let mut svm = AnchorLiteSVM::build_with_programs(&programs).svm;
    set_previous_slot_hash(&mut svm, DEFAULT_TEST_PREVIOUS_BANK_HASH);
    svm
}
