use crate::{
    onchain::{
        decode_state, encode_instruction, find_session_pda, find_state_pda, id as program_id,
        OnchainInstruction,
    },
    EvmAddress, FheType, HcuConfig, HostInstruction, HostProgramConfig, Pubkey,
    VerifierContextConfig,
};
use solana_instruction::{AccountMeta, Instruction};
use solana_program_test::ProgramTest;
use solana_signer::Signer;
use solana_system_interface::program as system_program;
use solana_transaction::Transaction;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

fn custom_pubkey(byte: u8) -> Pubkey {
    Pubkey::new([byte; 32])
}

fn evm(byte: u8) -> EvmAddress {
    EvmAddress::new([byte; 20])
}

fn host_program_config(owner: Pubkey) -> HostProgramConfig {
    HostProgramConfig {
        owner,
        upgrade_authority: custom_pubkey(2),
        acl_program: custom_pubkey(3),
        host_chain_id: 777,
        input_verifier: VerifierContextConfig {
            source_contract: evm(20),
            source_chain_id: 54321,
            signers: vec![evm(21), evm(22)],
            threshold: 1,
        },
        kms_verifier: VerifierContextConfig {
            source_contract: evm(30),
            source_chain_id: 54321,
            signers: vec![evm(31), evm(32)],
            threshold: 1,
        },
        hcu: HcuConfig {
            hcu_cap_per_block: 10_000_000,
            max_hcu_depth_per_tx: 8_000_000,
            max_hcu_per_tx: 9_000_000,
        },
    }
}

#[tokio::test]
async fn program_test_initialize_and_execute_rand() {
    let program_id = program_id();
    let sbf_out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy");
    let sbf_program = sbf_out_dir.join("solana_host_contracts.so");
    if !sbf_program.exists() {
        eprintln!(
            "Skipping BPF-backed ProgramTest because {} is missing. Run `cargo build-sbf --manifest-path {}` first.",
            sbf_program.display(),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml").display()
        );
        return;
    }
    if sbf_program_is_stale(&sbf_program) {
        eprintln!(
            "Skipping BPF-backed ProgramTest because {} is older than the current program sources. Run `cargo build-sbf --manifest-path {}` first.",
            sbf_program.display(),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml").display()
        );
        return;
    }

    std::env::set_var("SBF_OUT_DIR", &sbf_out_dir);
    let mut program_test = ProgramTest::new("solana_host_contracts", program_id, None);
    program_test.prefer_bpf(true);
    // The BPF-backed path includes PDA creation, dynamic account resize, and full host-program
    // dispatch, which is above the default ProgramTest compute cap on current Agave releases.
    program_test.set_compute_max_units(1_000_000);

    let context = program_test.start_with_context().await;
    let owner = Pubkey::from(context.payer.pubkey());
    let config = host_program_config(owner);
    let (state_pda, _) = find_state_pda(&program_id);
    let (session_pda, _) = find_session_pda(&program_id, &context.payer.pubkey());
    let init_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(context.payer.pubkey(), true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ],
        data: encode_instruction(&OnchainInstruction::InitializePda { config }).unwrap(),
    };
    let tx = Transaction::new_signed_with_payer(
        &[init_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction_with_preflight(tx)
        .await
        .unwrap();

    let execute_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(context.payer.pubkey(), true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new(session_pda, false),
            AccountMeta::new_readonly(solana_program::sysvar::clock::id(), false),
            AccountMeta::new_readonly(solana_program::sysvar::slot_hashes::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ],
        data: encode_instruction(&OnchainInstruction::Execute {
            instruction: HostInstruction::FheRand {
                rand_type: FheType::Uint8,
                charge_hcu: false,
            },
            session_nonce: 1,
            recent_blockhash: [0xAB; 32],
        })
        .unwrap(),
    };
    let tx = Transaction::new_signed_with_payer(
        &[execute_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction_with_preflight(tx)
        .await
        .unwrap();

    let account = context
        .banks_client
        .get_account(state_pda)
        .await
        .unwrap()
        .unwrap();
    let state = decode_state(&account.data).unwrap();
    assert_eq!(state.owner(), owner);
    assert_eq!(state.executor().counter_rand(), 1);

    std::env::remove_var("SBF_OUT_DIR");
}

fn sbf_program_is_stale(sbf_program: &PathBuf) -> bool {
    let artifact_mtime = match fs::metadata(sbf_program).and_then(|metadata| metadata.modified()) {
        Ok(mtime) => mtime,
        Err(_) => return true,
    };
    newest_source_mtime(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")) > artifact_mtime
}

fn newest_source_mtime(dir: PathBuf) -> SystemTime {
    let mut newest = SystemTime::UNIX_EPOCH;
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return newest,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            newest = newest.max(newest_source_mtime(path));
        } else if let Ok(mtime) = entry.metadata().and_then(|metadata| metadata.modified()) {
            newest = newest.max(mtime);
        }
    }
    newest
}
