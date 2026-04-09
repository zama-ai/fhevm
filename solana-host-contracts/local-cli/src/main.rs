use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use borsh::{to_vec, BorshDeserialize};
use k256::ecdsa::SigningKey;
use serde_json::{json, Value};
use sha3::{Digest, Keccak256};
use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_confidential_token_core::{
    find_state_pda as find_confidential_token_state_pda, ConfidentialTokenExecutionResult,
    ConfidentialTokenInstruction,
};
use solana_host_contracts_core::{
    find_session_pda, find_state_pda, BinaryOperand, ContextUserInputs, EvmAddress, FheType,
    HcuConfig, HostInstruction, HostProgramConfig, HostProgramSession, HostProgramState,
    InstructionResult, OnchainInstruction, Operator, ProgramContext, Pubkey as HostPubkey,
    VerifierContextConfig,
};
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::{read_keypair_file, Keypair};
use solana_pubkey::Pubkey;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, sysvar};
use solana_signer::Signer;
use solana_system_interface::program as system_program;
use solana_test_input_core::{
    find_state_pda as find_test_input_state_pda, TestInputExecutionResult, TestInputInstruction,
};
use solana_transaction::Transaction;
use std::{
    collections::HashMap,
    env,
    error::Error,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
    thread,
    time::{Duration, Instant},
};

type DynResult<T> = Result<T, Box<dyn Error>>;

const DEFAULT_LOCAL_RPC_URL: &str = "http://127.0.0.1:18999";
const DEFAULT_LOCAL_WS_URL: &str = "ws://127.0.0.1:19000";
const DEFAULT_HOST_PROGRAM_ID: &str = "5TeWSsjg2gbxCyWVniXeCmwM7UtHTCK7svzJr5xYJzHf";
const DEFAULT_TEST_INPUT_PROGRAM_ID: &str = "5MaDNrtMTmYccr1ASgE1i2LZgbnyBPeDR7tN8Q8ewXTv";
const DEFAULT_CONFIDENTIAL_TOKEN_PROGRAM_ID: &str = "Cjb3AVoxxKmG4TGWX5gzSjCNwtxN6gneVsWB7f9i8Csx";
const LOCAL_TRANSACTION_COMPUTE_UNIT_LIMIT: u32 = 1_400_000;
const LOCAL_TRANSACTION_HEAP_FRAME_BYTES: u32 = 256 * 1024;
const DEFAULT_TOKEN_NAME: &str = "Naraggara";
const DEFAULT_TOKEN_SYMBOL: &str = "NARA";
const DEFAULT_TOKEN_MINT_AMOUNT: u64 = 10_000;
const DEFAULT_TOKEN_TRANSFER_AMOUNT: u64 = 1_337;
const DEFAULT_LOCALNET_AIRDROP_SOL: u64 = 20;
const DEFAULT_TOKEN_RECIPIENT_AIRDROP_SOL: u64 = 5;
const DEFAULT_LOCALNET_BOOTSTRAP_TIMEOUT_SECS: u64 = 180;
const ENV_OVERRIDE_KEYS: &[&str] = &[
    "SOLANA_HOST_CHAIN_ID",
    "CHAIN_ID_GATEWAY",
    "INPUT_VERIFICATION_ADDRESS",
    "DECRYPTION_ADDRESS",
    "NUM_KMS_NODES",
    "PUBLIC_DECRYPTION_THRESHOLD",
    "NUM_COPROCESSORS",
    "COPROCESSOR_THRESHOLD",
    "HCU_CAP_PER_BLOCK",
    "MAX_HCU_DEPTH_PER_TX",
    "MAX_HCU_PER_TX",
    "SOLANA_HOST_OUTPUT_RPC_URL",
    "SOLANA_HOST_OUTPUT_WS_URL",
    "SOLANA_HOST_AIRDROP_SOL",
];

fn main() -> DynResult<()> {
    let mut args = env::args().skip(1);
    let command = args.next().ok_or("missing command")?;
    let rest: Vec<String> = args.collect();

    match command.as_str() {
        "init-local" => init_local(parse_options(&rest)?),
        "bootstrap-localnet" => bootstrap_localnet(parse_options(&rest)?),
        "smoke-rand" => smoke_rand(parse_options(&rest)?),
        "show-pdas" => show_pdas(parse_options(&rest)?),
        "trivial-encrypt" => trivial_encrypt_cmd(parse_options(&rest)?),
        "binary-op" => binary_op_cmd(parse_options(&rest)?),
        "cast" => cast_cmd(parse_options(&rest)?),
        "allow" => allow_cmd(parse_options(&rest)?),
        "allow-for-decryption" => allow_for_decryption_cmd(parse_options(&rest)?),
        "verify-input" => verify_input_cmd(parse_options(&rest)?),
        "scenario-basic" => scenario_basic(parse_options(&rest)?),
        "scenario-scalar" => scenario_scalar(parse_options(&rest)?),
        "scenario-cast" => scenario_cast(parse_options(&rest)?),
        "scenario-batch" => scenario_batch(parse_options(&rest)?),
        "scenario-public-ebool" => scenario_public_ebool(parse_options(&rest)?),
        "scenario-public-mixed" => scenario_public_mixed(parse_options(&rest)?),
        "scenario-user-decrypt" => scenario_user_decrypt(parse_options(&rest)?),
        "runtime-identities" => runtime_identities(parse_options(&rest)?),
        "runtime-state" => runtime_state(parse_options(&rest)?),
        "scenario-input-proof" => scenario_input_proof(parse_options(&rest)?),
        "scenario-test-input-add42" => scenario_test_input_add42(parse_options(&rest)?),
        "scenario-confidential-token" => scenario_confidential_token(parse_options(&rest)?),
        "token-reset" => token_reset_cmd(parse_options(&rest)?),
        "token-mint-to" => token_mint_to_cmd(parse_options(&rest)?),
        "token-transfer" => token_transfer_cmd(parse_options(&rest)?),
        "token-approve-delegate" => token_approve_delegate_cmd(parse_options(&rest)?),
        "token-transfer-as-delegate" => token_transfer_as_delegate_cmd(parse_options(&rest)?),
        "token-balance" => token_balance_cmd(parse_options(&rest)?),
        "token-delegate-allowance" => token_delegate_allowance_cmd(parse_options(&rest)?),
        "token-supply" => token_supply_cmd(parse_options(&rest)?),
        "help" | "--help" | "-h" => {
            print_usage();
            Ok(())
        }
        _ => Err(format!("unknown command: {command}").into()),
    }
}

fn init_local(options: HashMap<String, String>) -> DynResult<()> {
    let env_values = env::vars().collect::<HashMap<_, _>>();
    init_local_with_env(&options, &env_values)
}

fn init_local_with_env(
    options: &HashMap<String, String>,
    env_values: &HashMap<String, String>,
) -> DynResult<()> {
    let rpc_url = required_option(&options, "rpc-url")?;
    let ws_url = required_option(&options, "ws-url")?;
    let output_rpc_url = options
        .get("output-rpc-url")
        .map(String::as_str)
        .unwrap_or(rpc_url);
    let output_ws_url = options
        .get("output-ws-url")
        .map(String::as_str)
        .unwrap_or(ws_url);
    let payer_keypair_path = required_option(&options, "payer-keypair")?;
    let program_id = parse_pubkey(required_option(&options, "program-id")?)?;
    let test_input_program_id = parse_pubkey(required_option(&options, "test-input-program-id")?)?;
    let confidential_token_program_id =
        parse_pubkey(required_option(&options, "confidential-token-program-id")?)?;
    let addresses_env = PathBuf::from(required_option(&options, "addresses-env")?);
    let addresses_json = PathBuf::from(required_option(&options, "addresses-json")?);

    let payer = read_keypair(&payer_keypair_path)?;
    let client = rpc_client(rpc_url);
    let config = local_host_config_from_values(program_id, payer.pubkey(), env_values)?;
    let state_pda = maybe_initialize_state(&client, &payer, program_id, config.clone())?;
    let test_input_state_pda =
        maybe_initialize_test_input_state(&client, &payer, test_input_program_id, program_id)?;
    let confidential_token_state_pda = maybe_initialize_confidential_token_state(
        &client,
        &payer,
        confidential_token_program_id,
        program_id,
    )?;

    write_local_addresses(
        &addresses_env,
        &addresses_json,
        &LocalAddresses {
            rpc_url: output_rpc_url.to_owned(),
            ws_url: output_ws_url.to_owned(),
            program_id,
            state_pda,
            session_pda: find_session_pda(&program_id, &payer.pubkey()).0,
            test_input_program_id,
            test_input_state_pda,
            confidential_token_program_id,
            confidential_token_state_pda,
            authority: payer.pubkey(),
            token_recipient: token_recipient_pubkey()?,
            host_chain_id: config.host_chain_id,
            gateway_chain_id: config.input_verifier.source_chain_id,
            input_verification_address: config.input_verifier.source_contract,
            decryption_address: config.kms_verifier.source_contract,
            coprocessor_signers: config.input_verifier.signers.clone(),
            kms_signers: config.kms_verifier.signers.clone(),
            coprocessor_threshold: config.input_verifier.threshold,
            public_decryption_threshold: config.kms_verifier.threshold,
        },
    )?;

    println!("Initialized Solana host local deployment");
    println!("host_program_id={program_id}");
    println!("host_state_pda={state_pda}");
    println!("test_input_program_id={test_input_program_id}");
    println!("test_input_state_pda={test_input_state_pda}");
    println!("confidential_token_program_id={confidential_token_program_id}");
    println!("confidential_token_state_pda={confidential_token_state_pda}");
    println!("addresses_env={}", addresses_env.display());
    println!("addresses_json={}", addresses_json.display());

    Ok(())
}

fn bootstrap_localnet(options: HashMap<String, String>) -> DynResult<()> {
    let env_values = load_local_bootstrap_env()?;
    let rpc_url = options
        .get("rpc-url")
        .map(String::as_str)
        .unwrap_or(DEFAULT_LOCAL_RPC_URL);
    let ws_url = options
        .get("ws-url")
        .map(String::as_str)
        .unwrap_or(DEFAULT_LOCAL_WS_URL);
    let output_rpc_url = options
        .get("output-rpc-url")
        .map(String::as_str)
        .or_else(|| env_values.get("SOLANA_HOST_OUTPUT_RPC_URL").map(String::as_str))
        .filter(|value| !value.is_empty())
        .unwrap_or(rpc_url);
    let output_ws_url = options
        .get("output-ws-url")
        .map(String::as_str)
        .or_else(|| env_values.get("SOLANA_HOST_OUTPUT_WS_URL").map(String::as_str))
        .filter(|value| !value.is_empty())
        .unwrap_or(ws_url);
    let payer_keypair = options
        .get("payer-keypair")
        .cloned()
        .unwrap_or(default_payer_keypair_path()?);
    let recipient_keypair = options
        .get("recipient-keypair")
        .cloned()
        .unwrap_or(default_token_recipient_keypair_path()?);
    let addresses_env = options
        .get("addresses-env")
        .cloned()
        .unwrap_or_else(default_addresses_env_path);
    let addresses_json = options
        .get("addresses-json")
        .cloned()
        .unwrap_or_else(default_addresses_json_path);
    let program_id = options
        .get("program-id")
        .map(String::as_str)
        .unwrap_or(DEFAULT_HOST_PROGRAM_ID);
    let test_input_program_id = options
        .get("test-input-program-id")
        .map(String::as_str)
        .unwrap_or(DEFAULT_TEST_INPUT_PROGRAM_ID);
    let confidential_token_program_id = options
        .get("confidential-token-program-id")
        .map(String::as_str)
        .unwrap_or(DEFAULT_CONFIDENTIAL_TOKEN_PROGRAM_ID);
    let wait_for_programs = optional_bool(&options, "wait-for-programs", false)?;
    let timeout_secs = optional_u64(
        &options,
        "timeout-secs",
        DEFAULT_LOCALNET_BOOTSTRAP_TIMEOUT_SECS,
    )?;
    let authority_airdrop_sol = optional_u64(
        &options,
        "authority-airdrop-sol",
        env_values
            .get("SOLANA_HOST_AIRDROP_SOL")
            .and_then(|value| parse_u64(value).ok())
            .unwrap_or(DEFAULT_LOCALNET_AIRDROP_SOL),
    )?;
    let recipient_airdrop_sol = optional_u64(
        &options,
        "recipient-airdrop-sol",
        DEFAULT_TOKEN_RECIPIENT_AIRDROP_SOL,
    )?;

    wait_for_rpc_health(rpc_url, timeout_secs)?;
    if wait_for_programs {
        wait_for_program_deployments(
            rpc_url,
            &[
                parse_pubkey(program_id)?,
                parse_pubkey(test_input_program_id)?,
                parse_pubkey(confidential_token_program_id)?,
            ],
            timeout_secs,
        )?;
    }

    let client = rpc_client(rpc_url);
    let authority = read_keypair(&payer_keypair)?;
    request_and_confirm_airdrop(
        &client,
        authority.pubkey(),
        authority_airdrop_sol * LAMPORTS_PER_SOL,
        timeout_secs,
    )?;
    let recipient = read_keypair(&recipient_keypair)?;
    request_and_confirm_airdrop(
        &client,
        recipient.pubkey(),
        recipient_airdrop_sol * LAMPORTS_PER_SOL,
        timeout_secs,
    )?;

    let mut init_options = HashMap::new();
    init_options.insert("rpc-url".to_owned(), rpc_url.to_owned());
    init_options.insert("ws-url".to_owned(), ws_url.to_owned());
    init_options.insert("output-rpc-url".to_owned(), output_rpc_url.to_owned());
    init_options.insert("output-ws-url".to_owned(), output_ws_url.to_owned());
    init_options.insert("payer-keypair".to_owned(), payer_keypair);
    init_options.insert("program-id".to_owned(), program_id.to_owned());
    init_options.insert(
        "test-input-program-id".to_owned(),
        test_input_program_id.to_owned(),
    );
    init_options.insert(
        "confidential-token-program-id".to_owned(),
        confidential_token_program_id.to_owned(),
    );
    init_options.insert("addresses-env".to_owned(), addresses_env);
    init_options.insert("addresses-json".to_owned(), addresses_json);
    init_local_with_env(&init_options, &env_values)
}

fn smoke_rand(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let client = rpc_client(&runtime.rpc_url);
    ensure_state_exists(&client, runtime.state_pda)?;
    let rand_type = parse_fhe_type(
        options
            .get("rand-type")
            .map(String::as_str)
            .unwrap_or("uint8"),
    )?;
    let session_nonce = optional_u64(&options, "session-nonce", 1)?;

    let receipt = send_host_instruction(
        &runtime,
        HostInstruction::FheRand {
            rand_type,
            charge_hcu: false,
        },
        session_nonce,
    )?;

    println!("Smoke transaction sent");
    println!("signature={}", receipt.signature);
    println!("program_id={}", runtime.program_id);
    println!("state_pda={}", runtime.state_pda);
    println!("session_pda={}", runtime.session_pda);
    Ok(())
}

fn show_pdas(options: HashMap<String, String>) -> DynResult<()> {
    let program_id = parse_pubkey(required_option(&options, "program-id")?)?;
    let authority = parse_pubkey(required_option(&options, "authority")?)?;

    println!("program_id={program_id}");
    println!("state_pda={}", find_state_pda(&program_id).0);
    println!(
        "session_pda={}",
        find_session_pda(&program_id, &authority).0
    );
    Ok(())
}

fn trivial_encrypt_cmd(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let session_nonce = optional_u64(&options, "session-nonce", 1)?;
    let value = required_option(&options, "value")?;
    let to_type = parse_fhe_type(required_option(&options, "to-type")?)?;
    let plaintext = scalar_word_from_literal(value)?;

    let receipt = send_host_instruction(
        &runtime,
        HostInstruction::TrivialEncrypt {
            plaintext,
            to_type,
            charge_hcu: false,
        },
        session_nonce,
    )?;

    print_json(&json!({
        "command": "trivial-encrypt",
        "signature": receipt.signature,
        "returned_handles": handles_to_hex(&receipt.returned_handles),
    }))
}

fn binary_op_cmd(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let session_nonce = optional_u64(&options, "session-nonce", 1)?;
    let op = parse_binary_operator(required_option(&options, "op")?)?;
    let lhs = parse_handle(required_option(&options, "lhs")?)?;
    let rhs = if let Some(rhs_handle) = options.get("rhs-handle") {
        BinaryOperand::Handle(parse_handle(rhs_handle)?)
    } else if let Some(rhs_scalar) = options.get("rhs-scalar") {
        BinaryOperand::Scalar(scalar_word_from_literal(rhs_scalar)?)
    } else {
        return Err("binary-op requires either --rhs-handle or --rhs-scalar".into());
    };
    let result_type = parse_fhe_type(required_option(&options, "result-type")?)?;

    let receipt = send_host_instruction(
        &runtime,
        HostInstruction::BinaryOp {
            op,
            lhs,
            rhs,
            result_type,
            charge_hcu: false,
        },
        session_nonce,
    )?;

    print_json(&json!({
        "command": "binary-op",
        "signature": receipt.signature,
        "returned_handles": handles_to_hex(&receipt.returned_handles),
    }))
}

fn cast_cmd(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let session_nonce = optional_u64(&options, "session-nonce", 1)?;
    let ct = parse_handle(required_option(&options, "handle")?)?;
    let to_type = parse_fhe_type(required_option(&options, "to-type")?)?;

    let receipt = send_host_instruction(
        &runtime,
        HostInstruction::Cast {
            ct,
            to_type,
            charge_hcu: false,
        },
        session_nonce,
    )?;

    print_json(&json!({
        "command": "cast",
        "signature": receipt.signature,
        "returned_handles": handles_to_hex(&receipt.returned_handles),
    }))
}

fn allow_cmd(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let session_nonce = optional_u64(&options, "session-nonce", 1)?;
    let handle = parse_handle(required_option(&options, "handle")?)?;
    let account = HostPubkey::from(parse_pubkey(required_option(&options, "account")?)?.to_bytes());

    let receipt = send_host_instruction(
        &runtime,
        HostInstruction::Allow { handle, account },
        session_nonce,
    )?;

    print_json(&json!({
        "command": "allow",
        "signature": receipt.signature,
    }))
}

fn allow_for_decryption_cmd(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let session_nonce = optional_u64(&options, "session-nonce", 1)?;
    let handles = parse_handles_csv(required_option(&options, "handles")?)?;

    let receipt = send_host_instruction(
        &runtime,
        HostInstruction::AllowForDecryption { handles },
        session_nonce,
    )?;

    print_json(&json!({
        "command": "allow-for-decryption",
        "signature": receipt.signature,
    }))
}

fn verify_input_cmd(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let session_nonce = optional_u64(&options, "session-nonce", 1)?;
    let fhe_type = parse_fhe_type(
        options
            .get("input-type")
            .map(String::as_str)
            .unwrap_or("uint64"),
    )?;
    let ciphertext = options
        .get("ciphertext")
        .cloned()
        .unwrap_or_else(|| "solana-input-proof".to_owned())
        .into_bytes();
    let user_id = options
        .get("user-id")
        .map(|value| parse_host_pubkey(value))
        .transpose()?
        .unwrap_or_else(|| HostPubkey::from([4; 32]));
    let contract_id = options
        .get("contract-id")
        .map(|value| parse_host_pubkey(value))
        .transpose()?
        .unwrap_or_else(|| HostPubkey::from([5; 32]));
    let extra_data = parse_optional_hex_bytes(options.get("extra-data").map(String::as_str))?
        .unwrap_or_else(|| vec![0xAA, 0xBB, 0xCC]);

    let bundle = build_input_verification_bundle(
        &runtime,
        &ciphertext,
        fhe_type,
        ContextUserInputs {
            user_id,
            contract_id,
        },
        extra_data,
    )?;

    let receipt = send_host_instruction(
        &runtime,
        HostInstruction::VerifyInput {
            context: bundle.context,
            input_handle: bundle.selected_handle,
            input_proof: bundle.input_proof,
        },
        session_nonce,
    )?;

    print_json(&json!({
        "command": "verify-input",
        "signature": receipt.signature,
        "input_handle": handle_to_hex(bundle.selected_handle),
        "all_handles": handles_to_hex(&bundle.handles),
        "input_type": fhe_type_label(fhe_type),
    }))
}

fn scenario_basic(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let mut next_session_nonce = optional_u64(&options, "session-nonce", 1)?;
    let mut signatures = Vec::new();

    let h1 = single_handle(send_host_instruction(
        &runtime,
        HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(2),
            to_type: FheType::Uint8,
            charge_hcu: false,
        },
        next_nonce(&mut next_session_nonce),
    )?)?;
    signatures.push(h1.1);
    allow_for_decryption(&runtime, &mut signatures, &[h1.0], &mut next_session_nonce)?;

    let h2 = single_handle(send_host_instruction(
        &runtime,
        HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(3),
            to_type: FheType::Uint8,
            charge_hcu: false,
        },
        next_nonce(&mut next_session_nonce),
    )?)?;
    signatures.push(h2.1);
    allow_for_decryption(&runtime, &mut signatures, &[h2.0], &mut next_session_nonce)?;

    let h3 = single_handle(send_host_batch(
        &runtime,
        vec![HostInstruction::BinaryOp {
            op: Operator::FheAdd,
            lhs: h1.0,
            rhs: BinaryOperand::Handle(h2.0),
            result_type: FheType::Uint8,
            charge_hcu: false,
        }],
        next_nonce(&mut next_session_nonce),
    )?)?;
    signatures.push(h3.1);
    allow_for_decryption(&runtime, &mut signatures, &[h3.0], &mut next_session_nonce)?;

    print_scenario_result(
        "basic",
        signatures,
        vec![h1.0, h2.0, h3.0],
        vec![h3.0],
        vec![("5", "uint8")],
        vec![],
    )
}

fn scenario_scalar(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let mut next_session_nonce = optional_u64(&options, "session-nonce", 1)?;
    let mut signatures = Vec::new();

    let base = single_handle(send_host_instruction(
        &runtime,
        HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(7),
            to_type: FheType::Uint8,
            charge_hcu: false,
        },
        next_nonce(&mut next_session_nonce),
    )?)?;
    signatures.push(base.1);
    allow_for_decryption(
        &runtime,
        &mut signatures,
        &[base.0],
        &mut next_session_nonce,
    )?;

    let result = single_handle(send_host_batch(
        &runtime,
        vec![HostInstruction::BinaryOp {
            op: Operator::FheAdd,
            lhs: base.0,
            rhs: BinaryOperand::Scalar(scalar_word_from_u64(42)),
            result_type: FheType::Uint8,
            charge_hcu: false,
        }],
        next_nonce(&mut next_session_nonce),
    )?)?;
    signatures.push(result.1);
    allow_for_decryption(
        &runtime,
        &mut signatures,
        &[result.0],
        &mut next_session_nonce,
    )?;

    print_scenario_result(
        "scalar",
        signatures,
        vec![base.0, result.0],
        vec![result.0],
        vec![("49", "uint8")],
        vec![],
    )
}

fn scenario_cast(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let mut next_session_nonce = optional_u64(&options, "session-nonce", 1)?;
    let mut signatures = Vec::new();

    let base = single_handle(send_host_instruction(
        &runtime,
        HostInstruction::TrivialEncrypt {
            plaintext: scalar_word_from_u64(7),
            to_type: FheType::Uint8,
            charge_hcu: false,
        },
        next_nonce(&mut next_session_nonce),
    )?)?;
    signatures.push(base.1);
    allow_for_decryption(
        &runtime,
        &mut signatures,
        &[base.0],
        &mut next_session_nonce,
    )?;

    let casted = single_handle(send_host_batch(
        &runtime,
        vec![HostInstruction::Cast {
            ct: base.0,
            to_type: FheType::Uint64,
            charge_hcu: false,
        }],
        next_nonce(&mut next_session_nonce),
    )?)?;
    signatures.push(casted.1);
    allow_for_decryption(
        &runtime,
        &mut signatures,
        &[casted.0],
        &mut next_session_nonce,
    )?;

    print_scenario_result(
        "cast",
        signatures,
        vec![base.0, casted.0],
        vec![casted.0],
        vec![("7", "uint64")],
        vec![],
    )
}

fn scenario_batch(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let mut next_session_nonce = optional_u64(&options, "session-nonce", 1)?;
    let mut signatures = Vec::new();

    let first_batch = send_host_batch(
        &runtime,
        vec![
            HostInstruction::TrivialEncrypt {
                plaintext: scalar_word_from_u64(2),
                to_type: FheType::Uint8,
                charge_hcu: false,
            },
            HostInstruction::TrivialEncrypt {
                plaintext: scalar_word_from_u64(3),
                to_type: FheType::Uint8,
                charge_hcu: false,
            },
        ],
        next_nonce(&mut next_session_nonce),
    )?;
    let (h1, h2) = first_two_handles(&first_batch)?;
    signatures.push(first_batch.signature);
    allow_for_decryption(
        &runtime,
        &mut signatures,
        &[h1, h2],
        &mut next_session_nonce,
    )?;

    let second_batch = send_host_batch(
        &runtime,
        vec![
            HostInstruction::BinaryOp {
                op: Operator::FheAdd,
                lhs: h1,
                rhs: BinaryOperand::Handle(h2),
                result_type: FheType::Uint8,
                charge_hcu: false,
            },
            HostInstruction::Cast {
                ct: h1,
                to_type: FheType::Uint64,
                charge_hcu: false,
            },
        ],
        next_nonce(&mut next_session_nonce),
    )?;
    let (h3, h4) = first_two_handles(&second_batch)?;
    signatures.push(second_batch.signature);
    allow_for_decryption(
        &runtime,
        &mut signatures,
        &[h3, h4],
        &mut next_session_nonce,
    )?;

    print_scenario_result(
        "batch",
        signatures,
        vec![h1, h2, h3, h4],
        vec![h3, h4],
        vec![("5", "uint8"), ("2", "uint64")],
        vec![String::from(
            "Dependent operations inside the same Solana batch are not covered yet because handles are derived from runtime slot/timestamp/blockhash and cannot be precomputed safely client-side.",
        )],
    )
}

fn scenario_public_ebool(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let receipt = send_test_input_instruction(&runtime, TestInputInstruction::CreatePublicEbool)?;
    let value = single_handle(receipt)?;

    print_scenario_result(
        "public-ebool",
        vec![value.1],
        vec![value.0],
        vec![value.0],
        vec![("true", "bool")],
        vec![],
    )
}

fn scenario_public_mixed(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let receipt = send_test_input_instruction(&runtime, TestInputInstruction::CreatePublicMixed)?;
    let bool_handle = receipt
        .returned_handles
        .first()
        .copied()
        .ok_or("missing bool handle")?;
    let uint32_handle = receipt
        .returned_handles
        .get(1)
        .copied()
        .ok_or("missing uint32 handle")?;
    let address_handle = receipt
        .returned_handles
        .get(2)
        .copied()
        .ok_or("missing address handle")?;

    print_scenario_result(
        "public-mixed",
        vec![receipt.signature],
        vec![bool_handle, uint32_handle, address_handle],
        vec![bool_handle, uint32_handle, address_handle],
        vec![
            ("true", "bool"),
            ("242", "uint32"),
            ("0xfC4382C084fCA3f4fB07c3BCDA906C01797595a8", "address"),
        ],
        vec![],
    )
}

fn scenario_user_decrypt(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let user_id = scenario_user_id(&options, &runtime.payer.pubkey())?;
    let contract_id = HostPubkey::from(runtime.test_input_state_pda.to_bytes());
    let mut signatures = Vec::new();
    let mut handles = Vec::new();
    for (start_fixture_index, fixture_count) in
        [(0_u8, 2_u8), (2, 2), (4, 1), (5, 1), (6, 1), (7, 1)]
    {
        let receipt = send_test_input_instruction(
            &runtime,
            TestInputInstruction::CreateUserDecryptFixturesChunk {
                start_fixture_index,
                fixture_count,
                user_id,
            },
        )?;
        signatures.push(receipt.signature);
        handles.extend(receipt.returned_handles);
    }
    if handles.len() != 8 {
        return Err(format!(
            "expected 8 returned handles for user decrypt fixtures, got {}",
            handles.len()
        )
        .into());
    }

    print_json(&json!({
        "scenario": "user-decrypt",
        "signatures": signatures,
        "produced_handles": handles_to_hex(&handles),
        "final_handles": handles_to_hex(&handles),
        "x_bool_handle": handle_to_hex(handles[0]),
        "x_uint8_handle": handle_to_hex(handles[1]),
        "x_uint16_handle": handle_to_hex(handles[2]),
        "x_uint32_handle": handle_to_hex(handles[3]),
        "x_uint64_handle": handle_to_hex(handles[4]),
        "x_uint128_handle": handle_to_hex(handles[5]),
        "x_address_handle": handle_to_hex(handles[6]),
        "x_uint256_handle": handle_to_hex(handles[7]),
        "expected": [
            { "name": "xBool", "value": "true", "output_type": "bool" },
            { "name": "xUint8", "value": "42", "output_type": "uint8" },
            { "name": "xUint16", "value": "16", "output_type": "uint16" },
            { "name": "xUint32", "value": "32", "output_type": "uint32" },
            { "name": "xUint64", "value": "18446744073709551600", "output_type": "uint64" },
            { "name": "xUint128", "value": "145275933516363203950142179850024740765", "output_type": "uint128" },
            { "name": "xAddress", "value": "0x8ba1f109551bd432803012645ac136ddd64dba72", "output_type": "address" },
            { "name": "xUint256", "value": "74285495974541385002137713624115238327312291047062397922780925695323480915729", "output_type": "uint256" }
        ],
        "incompatibilities": [],
        "user_id_hex": host_pubkey_hex_string(user_id),
        "contract_id_hex": host_pubkey_hex_string(contract_id),
    }))
}

fn runtime_identities(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let recipient = read_keypair(&default_token_recipient_keypair_path()?)?;

    print_json(&json!({
        "payer_pubkey": runtime.payer.pubkey().to_string(),
        "payer_pubkey_hex": pubkey_hex_string(runtime.payer.pubkey()),
        "host_program_id": runtime.program_id.to_string(),
        "host_program_id_hex": pubkey_hex_string(runtime.program_id),
        "host_state_pda": runtime.state_pda.to_string(),
        "host_state_pda_hex": pubkey_hex_string(runtime.state_pda),
        "host_session_pda": runtime.session_pda.to_string(),
        "host_session_pda_hex": pubkey_hex_string(runtime.session_pda),
        "test_input_program_id": runtime.test_input_program_id.to_string(),
        "test_input_program_id_hex": pubkey_hex_string(runtime.test_input_program_id),
        "test_input_state_pda": runtime.test_input_state_pda.to_string(),
        "test_input_state_pda_hex": pubkey_hex_string(runtime.test_input_state_pda),
        "test_input_contract_id_hex": host_pubkey_hex_string(HostPubkey::from(runtime.test_input_state_pda.to_bytes())),
        "confidential_token_program_id": runtime.confidential_token_program_id.to_string(),
        "confidential_token_program_id_hex": pubkey_hex_string(runtime.confidential_token_program_id),
        "confidential_token_state_pda": runtime.confidential_token_state_pda.to_string(),
        "confidential_token_state_pda_hex": pubkey_hex_string(runtime.confidential_token_state_pda),
        "payer_user_id_hex": host_pubkey_hex_string(HostPubkey::from(runtime.payer.pubkey().to_bytes())),
        "confidential_token_contract_id_hex": host_pubkey_hex_string(HostPubkey::from(runtime.confidential_token_state_pda.to_bytes())),
        "token_recipient_pubkey": recipient.pubkey().to_string(),
        "token_recipient_pubkey_hex": pubkey_hex_string(recipient.pubkey()),
        "token_recipient_user_id_hex": host_pubkey_hex_string(HostPubkey::from(recipient.pubkey().to_bytes())),
    }))
}

fn runtime_state(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let client = rpc_client(&runtime.rpc_url);
    let account = client.get_account(&runtime.state_pda)?;
    if account.data.len() < 12 {
        return Err("host state account is too small".into());
    }
    if &account.data[..8] != b"FHEHOST0" {
        return Err("host state discriminator mismatch".into());
    }
    let mut slice: &[u8] = &account.data[12..];
    let state = HostProgramState::deserialize(&mut slice)?;
    let input_verifier = state.input_verifier();
    let kms_verifier = state.kms_verifier();

    print_json(&json!({
        "host_chain_id": state.host_chain_id(),
        "owner": host_pubkey_hex_string(state.owner()),
        "upgrade_authority": host_pubkey_hex_string(state.upgrade_authority()),
        "host_state_pda": runtime.state_pda.to_string(),
        "input_verifier": {
            "source_chain_id": input_verifier.get_source_chain_id(),
            "source_contract": evm_address_string(input_verifier.get_source_contract()),
            "threshold": input_verifier.get_threshold(),
            "signers": input_verifier
                .get_coprocessor_signers()
                .iter()
                .copied()
                .map(evm_address_string)
                .collect::<Vec<_>>(),
        },
        "kms_verifier": {
            "source_chain_id": kms_verifier.get_source_chain_id(),
            "source_contract": evm_address_string(kms_verifier.get_source_contract()),
            "threshold": kms_verifier.get_threshold(),
            "signers": kms_verifier
                .get_signers_for_kms_context(solana_host_contracts_core::KmsContextId::base())
                .into_iter()
                .map(evm_address_string)
                .collect::<Vec<_>>(),
        },
    }))
}

fn scenario_input_proof(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let user_id = scenario_user_id(&options, &runtime.payer.pubkey())?;
    let contract_id = HostPubkey::from(runtime.test_input_state_pda.to_bytes());
    let bundle = input_verification_bundle_from_options_or_default(
        &options,
        &runtime,
        b"solana-input-proof",
        FheType::Uint64,
        ContextUserInputs {
            user_id,
            contract_id,
        },
        vec![0xAA, 0xBB, 0xCC],
    )?;

    let receipt = send_test_input_instruction(
        &runtime,
        TestInputInstruction::RequestUint64NonTrivial {
            input_handle: bundle.selected_handle,
            input_proof: bundle.input_proof,
            user_id,
        },
    )?;

    print_json(&json!({
        "scenario": "input-proof",
        "signatures": [receipt.signature],
        "produced_handles": handles_to_hex(&bundle.handles),
        "final_handles": [handle_to_hex(bundle.selected_handle)],
        "expected": [{
            "value": "verified",
            "output_type": fhe_type_label(FheType::Uint64),
        }],
        "notes": [
            "This mirrors the EVM requestUint64NonTrivial shape: VerifyInput plus a durable Allow event on the verified handle. The gateway InputVerification flow materializes the proof outputs, while the Solana host listener ingests the host-side ACL follow-up events."
        ],
        "user_id_hex": host_pubkey_hex_string(user_id),
        "contract_id_hex": host_pubkey_hex_string(contract_id),
    }))
}

fn scenario_test_input_add42(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let user_id = scenario_user_id(&options, &runtime.payer.pubkey())?;
    let contract_id = HostPubkey::from(runtime.test_input_state_pda.to_bytes());
    let bundle = input_verification_bundle_from_options_or_default(
        &options,
        &runtime,
        b"solana-test-input-add42",
        FheType::Uint64,
        ContextUserInputs {
            user_id,
            contract_id,
        },
        vec![0xAA, 0x42, 0xCC],
    )?;

    let receipt = send_test_input_instruction(
        &runtime,
        TestInputInstruction::Add42ToInput64 {
            input_handle: bundle.selected_handle,
            input_proof: bundle.input_proof,
            user_id,
        },
    )?;

    let result_handle = receipt
        .returned_handles
        .first()
        .copied()
        .ok_or("missing add42 result handle")?;

    print_json(&json!({
        "scenario": "test-input-add42",
        "signatures": [receipt.signature],
        "produced_handles": handles_to_hex(&[bundle.selected_handle, result_handle]),
        "final_handles": [handle_to_hex(result_handle)],
        "expected": [{
            "value": "49",
            "output_type": "uint64",
        }],
        "incompatibilities": [],
        "user_id_hex": host_pubkey_hex_string(user_id),
        "contract_id_hex": host_pubkey_hex_string(contract_id),
    }))
}

fn scenario_user_id(
    options: &HashMap<String, String>,
    default_user: &solana_sdk::pubkey::Pubkey,
) -> DynResult<HostPubkey> {
    options
        .get("user-id")
        .map(|value| parse_host_pubkey(value))
        .transpose()?
        .map(Ok)
        .unwrap_or_else(|| Ok(HostPubkey::from(default_user.to_bytes())))
}

fn scenario_confidential_token(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let recipient = read_keypair(&default_token_recipient_keypair_path()?)?;
    let recipient_pubkey = recipient.pubkey();
    let proof_user_id = scenario_user_id(&options, &runtime.payer.pubkey())?;

    let mint_receipt = send_confidential_token_instruction(
        &runtime,
        &runtime.payer,
        ConfidentialTokenInstruction::MintTo {
            recipient: HostPubkey::from(runtime.payer.pubkey().to_bytes()),
            amount: DEFAULT_TOKEN_MINT_AMOUNT,
        },
    )?;
    let mint_balance_query_receipt = send_confidential_token_instruction(
        &runtime,
        &runtime.payer,
        ConfidentialTokenInstruction::Balance {
            owner: HostPubkey::from(runtime.payer.pubkey().to_bytes()),
        },
    )?;
    let mint_balance_handle = mint_balance_query_receipt
        .returned_handles
        .first()
        .copied()
        .ok_or("missing mint balance handle")?;

    let transfer_bundle = input_verification_bundle_from_options_or_default(
        &options,
        &runtime,
        format!("solana-confidential-token-transfer:{DEFAULT_TOKEN_TRANSFER_AMOUNT}").as_bytes(),
        FheType::Uint64,
        ContextUserInputs {
            user_id: proof_user_id,
            contract_id: HostPubkey::from(runtime.confidential_token_state_pda.to_bytes()),
        },
        vec![0xE2, 0xC2, 0x00, 0x01],
    )?;

    let transfer_receipt = send_confidential_token_instruction(
        &runtime,
        &runtime.payer,
        ConfidentialTokenInstruction::Transfer {
            recipient: HostPubkey::from(recipient_pubkey.to_bytes()),
            input_handle: transfer_bundle.selected_handle,
            input_proof: transfer_bundle.input_proof.clone(),
        },
    )?;
    let alice_balance_query_receipt = send_confidential_token_instruction(
        &runtime,
        &runtime.payer,
        ConfidentialTokenInstruction::Balance {
            owner: HostPubkey::from(runtime.payer.pubkey().to_bytes()),
        },
    )?;
    let bob_balance_query_receipt = send_confidential_token_instruction(
        &runtime,
        &recipient,
        ConfidentialTokenInstruction::Balance {
            owner: HostPubkey::from(recipient_pubkey.to_bytes()),
        },
    )?;
    let alice_balance_after_transfer = alice_balance_query_receipt
        .returned_handles
        .first()
        .copied()
        .ok_or("missing alice transfer balance handle")?;
    let bob_balance_after_transfer = bob_balance_query_receipt
        .returned_handles
        .first()
        .copied()
        .ok_or("missing bob transfer balance handle")?;

    print_json(&json!({
        "scenario": "confidential-token",
        "signatures": [
            mint_receipt.signature,
            mint_balance_query_receipt.signature,
            transfer_receipt.signature,
            alice_balance_query_receipt.signature,
            bob_balance_query_receipt.signature,
        ],
        "mint_balance_handle": handle_to_hex(mint_balance_handle),
        "transfer_input_handle": handle_to_hex(transfer_bundle.selected_handle),
        "alice_balance_handle_after_transfer": handle_to_hex(alice_balance_after_transfer),
        "recipient_balance_handle_after_transfer": handle_to_hex(bob_balance_after_transfer),
        "confidential_token_state_pda": runtime.confidential_token_state_pda.to_string(),
        "confidential_token_contract_id_hex": host_pubkey_hex_string(HostPubkey::from(runtime.confidential_token_state_pda.to_bytes())),
        "alice_pubkey": runtime.payer.pubkey().to_string(),
        "recipient_pubkey": recipient_pubkey.to_string(),
        "total_supply": mint_receipt.total_supply.unwrap_or_default().to_string(),
        "expected": {
            "mint_balance": DEFAULT_TOKEN_MINT_AMOUNT.to_string(),
            "transfer_amount": DEFAULT_TOKEN_TRANSFER_AMOUNT.to_string(),
            "alice_after_transfer": (DEFAULT_TOKEN_MINT_AMOUNT - DEFAULT_TOKEN_TRANSFER_AMOUNT).to_string(),
            "recipient_after_transfer": DEFAULT_TOKEN_TRANSFER_AMOUNT.to_string(),
        },
        "incompatibilities": [],
    }))
}

fn token_reset_cmd(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let session_nonce = optional_u64(&options, "session-nonce", 1)?;
    send_host_instruction(&runtime, HostInstruction::ResetAclState, session_nonce)?;
    let receipt = send_confidential_token_instruction(
        &runtime,
        &runtime.payer,
        ConfidentialTokenInstruction::ResetState,
    )?;
    print_json(&json!({
        "command": "token-reset",
        "signature": receipt.signature,
        "returned_handles": handles_to_hex(&receipt.returned_handles),
        "total_supply": receipt.total_supply.unwrap_or_default().to_string(),
    }))
}

fn token_mint_to_cmd(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let amount = parse_u64(required_option(&options, "amount")?)?;
    let recipient = HostPubkey::from(
        options
            .get("recipient")
            .map(String::as_str)
            .map(parse_pubkey)
            .transpose()?
            .unwrap_or_else(|| runtime.payer.pubkey())
            .to_bytes(),
    );
    let receipt = send_confidential_token_instruction(
        &runtime,
        &runtime.payer,
        ConfidentialTokenInstruction::MintTo {
            recipient,
            amount,
        },
    )?;
    print_json(&json!({
        "command": "token-mint-to",
        "signature": receipt.signature,
        "returned_handles": handles_to_hex(&receipt.returned_handles),
        "total_supply": receipt.total_supply.unwrap_or_default().to_string(),
    }))
}

fn token_transfer_cmd(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let recipient = HostPubkey::from(parse_pubkey(required_option(&options, "recipient")?)?.to_bytes());
    let input_handle = parse_handle(required_option(&options, "input-handle")?)?;
    let input_proof = parse_hex_bytes(required_option(&options, "input-proof")?)?;
    let receipt = send_confidential_token_instruction(
        &runtime,
        &runtime.payer,
        ConfidentialTokenInstruction::Transfer {
            recipient,
            input_handle,
            input_proof,
        },
    )?;
    print_json(&json!({
        "command": "token-transfer",
        "signature": receipt.signature,
        "returned_handles": handles_to_hex(&receipt.returned_handles),
        "total_supply": receipt.total_supply.unwrap_or_default().to_string(),
    }))
}

fn token_approve_delegate_cmd(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let delegate = HostPubkey::from(parse_pubkey(required_option(&options, "delegate")?)?.to_bytes());
    let input_handle = parse_handle(required_option(&options, "input-handle")?)?;
    let input_proof = parse_hex_bytes(required_option(&options, "input-proof")?)?;
    let receipt = send_confidential_token_instruction(
        &runtime,
        &runtime.payer,
        ConfidentialTokenInstruction::ApproveDelegate {
            delegate,
            input_handle,
            input_proof,
        },
    )?;
    print_json(&json!({
        "command": "token-approve-delegate",
        "signature": receipt.signature,
        "returned_handles": handles_to_hex(&receipt.returned_handles),
        "total_supply": receipt.total_supply.unwrap_or_default().to_string(),
    }))
}

fn token_transfer_as_delegate_cmd(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let source = HostPubkey::from(parse_pubkey(required_option(&options, "source")?)?.to_bytes());
    let recipient = HostPubkey::from(parse_pubkey(required_option(&options, "recipient")?)?.to_bytes());
    let input_handle = parse_handle(required_option(&options, "input-handle")?)?;
    let input_proof = parse_hex_bytes(required_option(&options, "input-proof")?)?;
    let receipt = send_confidential_token_instruction(
        &runtime,
        &runtime.payer,
        ConfidentialTokenInstruction::TransferAsDelegate {
            source,
            recipient,
            input_handle,
            input_proof,
        },
    )?;
    print_json(&json!({
        "command": "token-transfer-as-delegate",
        "signature": receipt.signature,
        "returned_handles": handles_to_hex(&receipt.returned_handles),
        "total_supply": receipt.total_supply.unwrap_or_default().to_string(),
    }))
}

fn token_balance_cmd(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let owner = HostPubkey::from(parse_pubkey(required_option(&options, "owner")?)?.to_bytes());
    let receipt = send_confidential_token_instruction(
        &runtime,
        &runtime.payer,
        ConfidentialTokenInstruction::Balance {
            owner,
        },
    )?;
    print_json(&json!({
        "command": "token-balance",
        "signature": receipt.signature,
        "returned_handles": handles_to_hex(&receipt.returned_handles),
        "total_supply": receipt.total_supply.unwrap_or_default().to_string(),
    }))
}

fn token_delegate_allowance_cmd(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let owner = HostPubkey::from(parse_pubkey(required_option(&options, "owner")?)?.to_bytes());
    let delegate = HostPubkey::from(parse_pubkey(required_option(&options, "delegate")?)?.to_bytes());
    let receipt = send_confidential_token_instruction(
        &runtime,
        &runtime.payer,
        ConfidentialTokenInstruction::DelegateAllowance { owner, delegate },
    )?;
    print_json(&json!({
        "command": "token-delegate-allowance",
        "signature": receipt.signature,
        "returned_handles": handles_to_hex(&receipt.returned_handles),
        "total_supply": receipt.total_supply.unwrap_or_default().to_string(),
    }))
}

fn token_supply_cmd(options: HashMap<String, String>) -> DynResult<()> {
    let runtime = runtime_from_options(&options)?;
    let receipt = send_confidential_token_instruction(
        &runtime,
        &runtime.payer,
        ConfidentialTokenInstruction::Supply,
    )?;
    print_json(&json!({
        "command": "token-supply",
        "signature": receipt.signature,
        "returned_handles": handles_to_hex(&receipt.returned_handles),
        "total_supply": receipt.total_supply.unwrap_or_default().to_string(),
    }))
}

fn allow_for_decryption(
    runtime: &LocalRuntime,
    signatures: &mut Vec<String>,
    handles: &[solana_host_contracts_core::Handle],
    next_session_nonce: &mut u64,
) -> DynResult<()> {
    let receipt = send_host_instruction(
        runtime,
        HostInstruction::AllowForDecryption {
            handles: handles.to_vec(),
        },
        next_nonce(next_session_nonce),
    )?;
    signatures.push(receipt.signature);
    Ok(())
}

fn next_nonce(current: &mut u64) -> u64 {
    let out = *current;
    *current = current.saturating_add(1);
    out
}

fn single_handle(
    receipt: ExecutionReceipt,
) -> DynResult<(solana_host_contracts_core::Handle, String)> {
    let handle = receipt
        .returned_handles
        .first()
        .copied()
        .ok_or("expected one returned handle")?;
    Ok((handle, receipt.signature))
}

fn first_two_handles(
    receipt: &ExecutionReceipt,
) -> DynResult<(
    solana_host_contracts_core::Handle,
    solana_host_contracts_core::Handle,
)> {
    let first = receipt
        .returned_handles
        .first()
        .copied()
        .ok_or("expected first returned handle")?;
    let second = receipt
        .returned_handles
        .get(1)
        .copied()
        .ok_or("expected second returned handle")?;
    Ok((first, second))
}

fn build_input_verification_bundle(
    runtime: &LocalRuntime,
    ciphertext: &[u8],
    fhe_type: FheType,
    context: ContextUserInputs,
    extra_data: Vec<u8>,
) -> DynResult<InputVerificationBundle> {
    let bit_width = fhe_type
        .bit_width()
        .ok_or_else(|| format!("unsupported input-proof type: {}", fhe_type_label(fhe_type)))?
        as usize;
    let host_chain_id = runtime_host_chain_id(runtime)?;
    let handles = solana_host_contracts_core::ExecutorState::compute_input_handles(
        ciphertext,
        &[bit_width],
        HostPubkey::from(runtime.program_id.to_bytes()),
        host_chain_id,
    )?;
    let selected_handle = handles
        .first()
        .copied()
        .ok_or("expected one computed input handle")?;
    let message = solana_host_contracts_core::Secp256k1ProofVerifier::input_verification_message(
        &solana_host_contracts_core::CiphertextVerification {
            ct_handles: handles.clone(),
            user_id: context.user_id,
            contract_id: context.contract_id,
            contract_chain_id: host_chain_id,
            extra_data: extra_data.clone(),
        },
        runtime_gateway_chain_id(runtime)?,
        runtime_input_verifier_address(runtime)?,
    );

    let signing_keys = load_private_signing_keys(
        parse_env_usize("NUM_COPROCESSORS")?,
        "PRIVATE_KEY_COPROCESSOR_ACCOUNT_",
    )?;

    let mut input_proof = Vec::new();
    input_proof.push(handles.len() as u8);
    input_proof.push(signing_keys.len() as u8);
    for handle in &handles {
        input_proof.extend_from_slice(handle.as_bytes());
    }
    for signing_key in &signing_keys {
        input_proof.extend_from_slice(&sign_message(signing_key, &message));
    }
    input_proof.extend_from_slice(&extra_data);

    Ok(InputVerificationBundle {
        context,
        handles,
        selected_handle,
        input_proof,
    })
}

fn input_verification_bundle_from_options_or_default(
    options: &HashMap<String, String>,
    runtime: &LocalRuntime,
    ciphertext: &[u8],
    fhe_type: FheType,
    context: ContextUserInputs,
    extra_data: Vec<u8>,
) -> DynResult<InputVerificationBundle> {
    let maybe_input_proof =
        parse_optional_hex_bytes(options.get("input-proof").map(String::as_str))?;
    let Some(input_proof) = maybe_input_proof else {
        return build_input_verification_bundle(runtime, ciphertext, fhe_type, context, extra_data);
    };

    let handles = parse_input_proof_handles(&input_proof)?;
    let selected_handle = options
        .get("input-handle")
        .map(|value| parse_handle(value))
        .transpose()?
        .unwrap_or_else(|| handles[0]);

    if !handles.contains(&selected_handle) {
        return Err(format!(
            "selected input handle {} is not present in the provided input proof",
            handle_to_hex(selected_handle)
        )
        .into());
    }

    Ok(InputVerificationBundle {
        context,
        handles,
        selected_handle,
        input_proof,
    })
}

fn parse_input_proof_handles(
    input_proof: &[u8],
) -> DynResult<Vec<solana_host_contracts_core::Handle>> {
    if input_proof.len() < 2 {
        return Err("input proof is too short to contain a header".into());
    }

    let handle_count = input_proof[0] as usize;
    let signer_count = input_proof[1] as usize;
    let handles_bytes = handle_count
        .checked_mul(32)
        .ok_or("input proof handle count overflow")?;
    let signatures_bytes = signer_count
        .checked_mul(65)
        .ok_or("input proof signer count overflow")?;
    let minimum_len = 2usize
        .checked_add(handles_bytes)
        .and_then(|value| value.checked_add(signatures_bytes))
        .ok_or("input proof length overflow")?;

    if input_proof.len() < minimum_len {
        return Err(format!(
            "input proof is truncated: expected at least {minimum_len} bytes, got {}",
            input_proof.len()
        )
        .into());
    }

    let handles_slice = &input_proof[2..(2 + handles_bytes)];
    let mut handles = Vec::with_capacity(handle_count);
    for raw_handle in handles_slice.chunks(32) {
        let handle_bytes: [u8; 32] = raw_handle
            .try_into()
            .map_err(|_| "failed to decode 32-byte handle from input proof")?;
        handles.push(solana_host_contracts_core::Handle::from(handle_bytes));
    }

    if handles.is_empty() {
        return Err("input proof does not contain any handles".into());
    }

    Ok(handles)
}

fn runtime_from_options(options: &HashMap<String, String>) -> DynResult<LocalRuntime> {
    let addresses_env = PathBuf::from(
        options
            .get("addresses-env")
            .cloned()
            .unwrap_or_else(default_addresses_env_path),
    );
    let addresses = load_addresses_env(&addresses_env)?;
    let rpc_url = options
        .get("rpc-url")
        .cloned()
        .or_else(|| addresses.get("SOLANA_HOST_RPC_URL").cloned())
        .ok_or("missing rpc-url and SOLANA_HOST_RPC_URL")?;
    let program_id = options
        .get("program-id")
        .map(|value| parse_pubkey(value))
        .transpose()?
        .or_else(|| {
            addresses
                .get("SOLANA_HOST_PROGRAM_ID")
                .and_then(|value| Pubkey::from_str(value).ok())
        })
        .ok_or("missing program-id and SOLANA_HOST_PROGRAM_ID")?;
    let payer_keypair_path = options
        .get("payer-keypair")
        .cloned()
        .unwrap_or(default_payer_keypair_path()?);
    let payer = read_keypair(&payer_keypair_path)?;
    let state_pda = find_state_pda(&program_id).0;
    let session_pda = find_session_pda(&program_id, &payer.pubkey()).0;
    let test_input_program_id = options
        .get("test-input-program-id")
        .map(|value| parse_pubkey(value))
        .transpose()?
        .or_else(|| {
            addresses
                .get("SOLANA_TEST_INPUT_PROGRAM_ID")
                .and_then(|value| Pubkey::from_str(value).ok())
        })
        .ok_or("missing test-input-program-id and SOLANA_TEST_INPUT_PROGRAM_ID")?;
    let test_input_state_pda = find_test_input_state_pda(&test_input_program_id).0;
    let confidential_token_program_id = options
        .get("confidential-token-program-id")
        .map(|value| parse_pubkey(value))
        .transpose()?
        .or_else(|| {
            addresses
                .get("SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_ID")
                .and_then(|value| Pubkey::from_str(value).ok())
        })
        .ok_or("missing confidential-token-program-id and SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_ID")?;
    let confidential_token_state_pda = find_confidential_token_state_pda(&confidential_token_program_id).0;
    Ok(LocalRuntime {
        rpc_url,
        program_id,
        payer,
        state_pda,
        session_pda,
        test_input_program_id,
        test_input_state_pda,
        confidential_token_program_id,
        confidential_token_state_pda,
        addresses_env,
    })
}

fn maybe_initialize_state(
    client: &RpcClient,
    payer: &Keypair,
    program_id: Pubkey,
    config: HostProgramConfig,
) -> DynResult<Pubkey> {
    let state_pda = find_state_pda(&program_id).0;
    if client.get_account(&state_pda).is_ok() {
        return Ok(state_pda);
    }

    let instruction = OnchainInstruction::InitializePda { config };
    let ix = host_program_instruction(
        program_id,
        payer.pubkey(),
        state_pda,
        find_session_pda(&program_id, &payer.pubkey()).0,
        instruction,
    )?;
    match send_transaction(client, payer, &[ix]) {
        Ok(signature) => println!("State PDA initialized with signature={signature}"),
        Err(error) => {
            if client.get_account(&state_pda).is_ok() {
                println!("State PDA already initialized; reusing existing account");
                return Ok(state_pda);
            }
            return Err(error);
        }
    }
    Ok(state_pda)
}

fn maybe_initialize_test_input_state(
    client: &RpcClient,
    payer: &Keypair,
    test_input_program_id: Pubkey,
    host_program_id: Pubkey,
) -> DynResult<Pubkey> {
    let state_pda = find_test_input_state_pda(&test_input_program_id).0;
    if client.get_account(&state_pda).is_ok() {
        return Ok(state_pda);
    }

    let ix = test_input_program_instruction(
        test_input_program_id,
        payer.pubkey(),
        state_pda,
        host_program_id,
        TestInputInstruction::InitializePda {
            owner: HostPubkey::from(payer.pubkey().to_bytes()),
            host_program: HostPubkey::from(host_program_id.to_bytes()),
        },
    )?;
    match send_transaction(client, payer, &[ix]) {
        Ok(signature) => println!("TestInput state PDA initialized with signature={signature}"),
        Err(error) => {
            if client.get_account(&state_pda).is_ok() {
                println!("TestInput state PDA already initialized; reusing existing account");
                return Ok(state_pda);
            }
            return Err(error);
        }
    }
    Ok(state_pda)
}

fn maybe_initialize_confidential_token_state(
    client: &RpcClient,
    payer: &Keypair,
    confidential_token_program_id: Pubkey,
    host_program_id: Pubkey,
) -> DynResult<Pubkey> {
    let state_pda = find_confidential_token_state_pda(&confidential_token_program_id).0;
    if client.get_account(&state_pda).is_ok() {
        return Ok(state_pda);
    }

    let ix = confidential_token_program_instruction(
        confidential_token_program_id,
        payer.pubkey(),
        state_pda,
        host_program_id,
        ConfidentialTokenInstruction::InitializePda {
            owner: HostPubkey::from(payer.pubkey().to_bytes()),
            host_program: HostPubkey::from(host_program_id.to_bytes()),
            name: DEFAULT_TOKEN_NAME.to_owned(),
            symbol: DEFAULT_TOKEN_SYMBOL.to_owned(),
            max_balance_entries: 16,
            max_allowance_entries: 16,
        },
    )?;
    match send_transaction(client, payer, &[ix]) {
        Ok(signature) => {
            println!("ConfidentialToken state PDA initialized with signature={signature}")
        }
        Err(error) => {
            if client.get_account(&state_pda).is_ok() {
                println!("ConfidentialToken state PDA already initialized; reusing existing account");
                return Ok(state_pda);
            }
            return Err(error);
        }
    }
    Ok(state_pda)
}

fn send_host_instruction(
    runtime: &LocalRuntime,
    instruction: HostInstruction,
    session_nonce: u64,
) -> DynResult<ExecutionReceipt> {
    send_host_onchain_instruction(
        runtime,
        OnchainInstruction::Execute {
            instruction,
            session_nonce,
            recent_blockhash: [0; 32],
        },
    )
}

fn send_host_batch(
    runtime: &LocalRuntime,
    instructions: Vec<HostInstruction>,
    session_nonce: u64,
) -> DynResult<ExecutionReceipt> {
    send_host_onchain_instruction(
        runtime,
        OnchainInstruction::ExecuteBatch {
            instructions,
            session_nonce,
            recent_blockhash: [0; 32],
        },
    )
}

fn send_host_onchain_instruction(
    runtime: &LocalRuntime,
    instruction: OnchainInstruction,
) -> DynResult<ExecutionReceipt> {
    let client = rpc_client(&runtime.rpc_url);
    ensure_state_exists(&client, runtime.state_pda)?;
    let ix = host_program_instruction(
        runtime.program_id,
        runtime.payer.pubkey(),
        runtime.state_pda,
        runtime.session_pda,
        instruction,
    )?;
    let signature = send_transaction(&client, &runtime.payer, &[ix])?;
    let returned_handles = fetch_returned_handles(&runtime.rpc_url, &signature)?;
    Ok(ExecutionReceipt {
        signature,
        returned_handles,
    })
}

fn send_test_input_instruction(
    runtime: &LocalRuntime,
    instruction: TestInputInstruction,
) -> DynResult<ExecutionReceipt> {
    let client = rpc_client(&runtime.rpc_url);
    ensure_state_exists(&client, runtime.state_pda)?;
    ensure_state_exists(&client, runtime.test_input_state_pda)?;
    let ix = test_input_program_instruction(
        runtime.test_input_program_id,
        runtime.payer.pubkey(),
        runtime.test_input_state_pda,
        runtime.program_id,
        instruction,
    )?;
    let signature = send_transaction(&client, &runtime.payer, &[ix])?;
    let returned_handles = fetch_test_input_returned_handles(&runtime.rpc_url, &signature)?;
    Ok(ExecutionReceipt {
        signature,
        returned_handles,
    })
}

fn send_confidential_token_instruction(
    runtime: &LocalRuntime,
    payer: &Keypair,
    instruction: ConfidentialTokenInstruction,
) -> DynResult<TokenExecutionReceipt> {
    let client = rpc_client(&runtime.rpc_url);
    ensure_state_exists(&client, runtime.state_pda)?;
    ensure_state_exists(&client, runtime.confidential_token_state_pda)?;
    let ix = confidential_token_program_instruction(
        runtime.confidential_token_program_id,
        payer.pubkey(),
        runtime.confidential_token_state_pda,
        runtime.program_id,
        instruction,
    )?;
    let signature = send_transaction(&client, payer, &[ix])?;
    let result = fetch_confidential_token_result(&runtime.rpc_url, &signature)?;
    Ok(TokenExecutionReceipt {
        signature,
        returned_handles: result.returned_handles,
        total_supply: result.total_supply,
    })
}

fn host_program_instruction(
    program_id: Pubkey,
    authority: Pubkey,
    state_pda: Pubkey,
    session_pda: Pubkey,
    instruction: OnchainInstruction,
) -> DynResult<Instruction> {
    let accounts = match &instruction {
        OnchainInstruction::Initialize { .. } => vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(state_pda, false),
        ],
        OnchainInstruction::InitializePda { .. } => vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        OnchainInstruction::Execute { .. } | OnchainInstruction::ExecuteBatch { .. } => vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new(session_pda, false),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(sysvar::slot_hashes::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
    };

    Ok(Instruction {
        program_id,
        accounts,
        data: to_vec(&instruction)?,
    })
}

fn test_input_program_instruction(
    program_id: Pubkey,
    authority: Pubkey,
    state_pda: Pubkey,
    host_program_id: Pubkey,
    instruction: TestInputInstruction,
) -> DynResult<Instruction> {
    let accounts = match &instruction {
        TestInputInstruction::InitializePda { .. } => vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        TestInputInstruction::RequestUint64NonTrivial { .. }
        | TestInputInstruction::Add42ToInput64 { .. }
        | TestInputInstruction::CreateUserDecryptFixture { .. }
        | TestInputInstruction::CreateUserDecryptFixtures { .. }
        | TestInputInstruction::CreateUserDecryptFixturesChunk { .. }
        | TestInputInstruction::CreatePublicEbool
        | TestInputInstruction::CreatePublicMixed => vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new_readonly(host_program_id, false),
            AccountMeta::new(find_state_pda(&host_program_id).0, false),
            AccountMeta::new(find_session_pda(&host_program_id, &authority).0, false),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(sysvar::slot_hashes::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
    };

    Ok(Instruction {
        program_id,
        accounts,
        data: to_vec(&instruction)?,
    })
}

fn confidential_token_program_instruction(
    program_id: Pubkey,
    authority: Pubkey,
    state_pda: Pubkey,
    host_program_id: Pubkey,
    instruction: ConfidentialTokenInstruction,
) -> DynResult<Instruction> {
    let accounts = match &instruction {
        ConfidentialTokenInstruction::InitializePda { .. } => vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        ConfidentialTokenInstruction::ResetState
        | ConfidentialTokenInstruction::MintTo { .. }
        | ConfidentialTokenInstruction::Transfer { .. }
        | ConfidentialTokenInstruction::ApproveDelegate { .. }
        | ConfidentialTokenInstruction::TransferAsDelegate { .. }
        | ConfidentialTokenInstruction::Balance { .. }
        | ConfidentialTokenInstruction::DelegateAllowance { .. }
        | ConfidentialTokenInstruction::Supply => vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new_readonly(host_program_id, false),
            AccountMeta::new(find_state_pda(&host_program_id).0, false),
            AccountMeta::new(find_session_pda(&host_program_id, &state_pda).0, false),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(sysvar::slot_hashes::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
    };

    Ok(Instruction {
        program_id,
        accounts,
        data: to_vec(&instruction)?,
    })
}

fn fetch_returned_handles(
    rpc_url: &str,
    signature: &str,
) -> DynResult<Vec<solana_host_contracts_core::Handle>> {
    let payload = rpc_call(
        rpc_url,
        "getTransaction",
        json!([
            signature,
            {
                "encoding": "json",
                "commitment": "confirmed",
                "maxSupportedTransactionVersion": 0
            }
        ]),
    )?;
    let data_b64 = payload["meta"]["returnData"]["data"][0]
        .as_str()
        .ok_or("missing returnData in transaction response")?;
    let bytes = BASE64_STANDARD.decode(data_b64)?;
    let results = Vec::<InstructionResult>::try_from_slice(&bytes)?;
    Ok(results
        .into_iter()
        .filter_map(|result| result.returned_handle)
        .collect())
}

fn fetch_test_input_returned_handles(
    rpc_url: &str,
    signature: &str,
) -> DynResult<Vec<solana_host_contracts_core::Handle>> {
    let payload = rpc_call(
        rpc_url,
        "getTransaction",
        json!([
            signature,
            {
                "encoding": "json",
                "commitment": "confirmed",
                "maxSupportedTransactionVersion": 0
            }
        ]),
    )?;
    let data_b64 = payload["meta"]["returnData"]["data"][0]
        .as_str()
        .ok_or("missing returnData in transaction response")?;
    let bytes = BASE64_STANDARD.decode(data_b64)?;
    let result = TestInputExecutionResult::try_from_slice(&bytes)?;
    Ok(result.returned_handles)
}

fn fetch_confidential_token_result(
    rpc_url: &str,
    signature: &str,
) -> DynResult<ConfidentialTokenExecutionResult> {
    let payload = rpc_call(
        rpc_url,
        "getTransaction",
        json!([
            signature,
            {
                "encoding": "json",
                "commitment": "confirmed",
                "maxSupportedTransactionVersion": 0
            }
        ]),
    )?;
    let data_b64 = payload["meta"]["returnData"]["data"][0]
        .as_str()
        .ok_or("missing returnData in transaction response")?;
    let bytes = BASE64_STANDARD.decode(data_b64)?;
    Ok(ConfidentialTokenExecutionResult::try_from_slice(&bytes)?)
}

fn rpc_call(rpc_url: &str, method: &str, params: Value) -> DynResult<Value> {
    let response = reqwest::blocking::Client::new()
        .post(rpc_url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params,
        }))
        .send()?
        .error_for_status()?
        .json::<Value>()?;

    if let Some(error) = response.get("error") {
        return Err(format!("rpc {method} failed: {error}").into());
    }

    response
        .get("result")
        .cloned()
        .ok_or_else(|| format!("rpc {method} missing result").into())
}

fn ensure_state_exists(client: &RpcClient, state_pda: Pubkey) -> DynResult<()> {
    if client.get_account(&state_pda).is_ok() {
        Ok(())
    } else {
        Err("state PDA is not initialized; run init-local or make localnet first".into())
    }
}

fn send_transaction(
    client: &RpcClient,
    payer: &Keypair,
    instructions: &[Instruction],
) -> DynResult<String> {
    let recent_blockhash = client.get_latest_blockhash()?;
    let mut transaction_instructions = Vec::with_capacity(instructions.len() + 2);
    transaction_instructions.push(ComputeBudgetInstruction::request_heap_frame(
        LOCAL_TRANSACTION_HEAP_FRAME_BYTES,
    ));
    transaction_instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(
        LOCAL_TRANSACTION_COMPUTE_UNIT_LIMIT,
    ));
    transaction_instructions.extend_from_slice(instructions);
    let tx = Transaction::new_signed_with_payer(
        &transaction_instructions,
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    Ok(client.send_and_confirm_transaction(&tx)?.to_string())
}

fn local_host_config_from_values(
    program_id: Pubkey,
    authority: Pubkey,
    values: &HashMap<String, String>,
) -> DynResult<HostProgramConfig> {
    let gateway_chain_id = parse_value_u64(values, "CHAIN_ID_GATEWAY")?;
    let input_verifier_address = parse_value_evm_address(values, "INPUT_VERIFICATION_ADDRESS")?;
    let decryption_address = parse_value_evm_address(values, "DECRYPTION_ADDRESS")?;
    let coprocessor_threshold = parse_value_u32(values, "COPROCESSOR_THRESHOLD")?;
    let public_decryption_threshold = parse_value_u32(values, "PUBLIC_DECRYPTION_THRESHOLD")?;

    Ok(HostProgramConfig {
        owner: HostPubkey::from(authority.to_bytes()),
        upgrade_authority: HostPubkey::from(authority.to_bytes()),
        acl_program: HostPubkey::from(program_id.to_bytes()),
        host_chain_id: parse_value_u64(values, "SOLANA_HOST_CHAIN_ID")?,
        input_verifier: VerifierContextConfig {
            source_contract: input_verifier_address,
            source_chain_id: gateway_chain_id,
            signers: load_signers_from_values(
                values,
                parse_value_usize(values, "NUM_COPROCESSORS")?,
                "PRIVATE_KEY_COPROCESSOR_ACCOUNT_",
                "COPROCESSOR_SIGNER_ADDRESS_",
            )?,
            threshold: coprocessor_threshold,
        },
        kms_verifier: VerifierContextConfig {
            source_contract: decryption_address,
            source_chain_id: gateway_chain_id,
            signers: load_signers_from_values(
                values,
                parse_value_usize(values, "NUM_KMS_NODES")?,
                "PRIVATE_KEY_KMS_SIGNER_",
                "KMS_SIGNER_ADDRESS_",
            )?,
            threshold: public_decryption_threshold,
        },
        hcu: HcuConfig {
            hcu_cap_per_block: parse_value_u64(values, "HCU_CAP_PER_BLOCK")?,
            max_hcu_depth_per_tx: parse_value_u64(values, "MAX_HCU_DEPTH_PER_TX")?,
            max_hcu_per_tx: parse_value_u64(values, "MAX_HCU_PER_TX")?,
        },
    })
}

fn load_signers_from_values(
    values: &HashMap<String, String>,
    count: usize,
    private_key_prefix: &str,
    address_prefix: &str,
) -> DynResult<Vec<EvmAddress>> {
    let mut signers = Vec::with_capacity(count);
    for index in 0..count {
        let private_key_var = format!("{private_key_prefix}{index}");
        let address_var = format!("{address_prefix}{index}");
        if let Some(private_key) = values.get(&private_key_var) {
            signers.push(evm_address_from_private_key(private_key)?);
        } else {
            signers.push(parse_value_evm_address(values, &address_var)?);
        }
    }
    Ok(signers)
}

fn evm_address_from_private_key(private_key: &str) -> DynResult<EvmAddress> {
    let bytes = parse_hex_32(private_key)?;
    let signing_key = SigningKey::from_bytes((&bytes).into())?;
    let encoded = signing_key.verifying_key().to_encoded_point(false);
    let hash = Keccak256::digest(&encoded.as_bytes()[1..65]);
    let mut address = [0_u8; 20];
    address.copy_from_slice(&hash[12..]);
    Ok(EvmAddress::new(address))
}

fn rpc_client(rpc_url: &str) -> RpcClient {
    RpcClient::new_with_commitment(rpc_url.to_owned(), CommitmentConfig::confirmed())
}

fn wait_for_rpc_health(rpc_url: &str, timeout_secs: u64) -> DynResult<()> {
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    while Instant::now() < deadline {
        if rpc_call(rpc_url, "getHealth", json!([])).is_ok() {
            return Ok(());
        }
        thread::sleep(Duration::from_secs(1));
    }
    Err(format!("timed out waiting for Solana RPC health at {rpc_url}").into())
}

fn wait_for_program_deployments(
    rpc_url: &str,
    program_ids: &[Pubkey],
    timeout_secs: u64,
) -> DynResult<()> {
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    while Instant::now() < deadline {
        let mut deployed = true;
        for program_id in program_ids {
            let account_info = rpc_call(
                rpc_url,
                "getAccountInfo",
                json!([
                    program_id.to_string(),
                    {
                        "encoding": "base64"
                    }
                ]),
            )?;
            if account_info["value"]["executable"] != json!(true) {
                deployed = false;
                break;
            }
        }
        if deployed {
            return Ok(());
        }
        thread::sleep(Duration::from_secs(1));
    }
    Err("timed out waiting for Solana programs to deploy".into())
}

fn request_and_confirm_airdrop(
    client: &RpcClient,
    recipient: Pubkey,
    lamports: u64,
    timeout_secs: u64,
) -> DynResult<()> {
    let signature = client.request_airdrop(&recipient, lamports)?;
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    while Instant::now() < deadline {
        if client.confirm_transaction(&signature)? {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(500));
    }
    Err(format!(
        "timed out waiting for airdrop confirmation for {recipient}"
    )
    .into())
}

fn write_local_addresses(
    env_path: &Path,
    json_path: &Path,
    addresses: &LocalAddresses,
) -> DynResult<()> {
    ensure_parent_dir(env_path)?;
    ensure_parent_dir(json_path)?;

    fs::write(
        env_path,
        format!(
            "SOLANA_HOST_RPC_URL={rpc_url}\n\
SOLANA_HOST_WS_URL={ws_url}\n\
SOLANA_HOST_KIND=solana\n\
SOLANA_HOST_PROGRAM_ID={program_id}\n\
SOLANA_HOST_STATE_PDA={state_pda}\n\
SOLANA_HOST_SESSION_PDA={session_pda}\n\
SOLANA_HOST_ACL_PROGRAM_ID={program_id}\n\
SOLANA_TEST_INPUT_PROGRAM_ID={test_input_program_id}\n\
SOLANA_TEST_INPUT_STATE_PDA={test_input_state_pda}\n\
SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_ID={confidential_token_program_id}\n\
SOLANA_CONFIDENTIAL_TOKEN_STATE_PDA={confidential_token_state_pda}\n\
SOLANA_HOST_AUTHORITY={authority}\n\
SOLANA_TOKEN_RECIPIENT={token_recipient}\n\
SOLANA_HOST_CHAIN_ID={host_chain_id}\n\
CHAIN_ID_GATEWAY={gateway_chain_id}\n\
INPUT_VERIFICATION_ADDRESS={input_verification_address}\n\
DECRYPTION_ADDRESS={decryption_address}\n\
NUM_COPROCESSORS={num_coprocessors}\n\
COPROCESSOR_THRESHOLD={coprocessor_threshold}\n\
NUM_KMS_NODES={num_kms_nodes}\n\
PUBLIC_DECRYPTION_THRESHOLD={public_decryption_threshold}\n{coprocessor_signers}{kms_signers}",
            rpc_url = addresses.rpc_url,
            ws_url = addresses.ws_url,
            program_id = addresses.program_id,
            state_pda = addresses.state_pda,
            session_pda = addresses.session_pda,
            test_input_program_id = addresses.test_input_program_id,
            test_input_state_pda = addresses.test_input_state_pda,
            confidential_token_program_id = addresses.confidential_token_program_id,
            confidential_token_state_pda = addresses.confidential_token_state_pda,
            authority = addresses.authority,
            token_recipient = addresses.token_recipient,
            host_chain_id = addresses.host_chain_id,
            gateway_chain_id = addresses.gateway_chain_id,
            input_verification_address = evm_address_string(addresses.input_verification_address),
            decryption_address = evm_address_string(addresses.decryption_address),
            num_coprocessors = addresses.coprocessor_signers.len(),
            coprocessor_threshold = addresses.coprocessor_threshold,
            num_kms_nodes = addresses.kms_signers.len(),
            public_decryption_threshold = addresses.public_decryption_threshold,
            coprocessor_signers = env_signer_lines(
                "COPROCESSOR_SIGNER_ADDRESS_",
                &addresses.coprocessor_signers,
            ),
            kms_signers = env_signer_lines("KMS_SIGNER_ADDRESS_", &addresses.kms_signers),
        ),
    )?;

    fs::write(
        json_path,
        format!(
            "{{\n  \"rpc_url\": \"{rpc_url}\",\n  \"ws_url\": \"{ws_url}\",\n  \"host_kind\": \"solana\",\n  \"program_id\": \"{program_id}\",\n  \"state_pda\": \"{state_pda}\",\n  \"session_pda\": \"{session_pda}\",\n  \"acl_program_id\": \"{program_id}\",\n  \"test_input_program_id\": \"{test_input_program_id}\",\n  \"test_input_state_pda\": \"{test_input_state_pda}\",\n  \"confidential_token_program_id\": \"{confidential_token_program_id}\",\n  \"confidential_token_state_pda\": \"{confidential_token_state_pda}\",\n  \"authority\": \"{authority}\",\n  \"token_recipient\": \"{token_recipient}\",\n  \"host_chain_id\": {host_chain_id},\n  \"gateway_chain_id\": {gateway_chain_id},\n  \"input_verification_address\": \"{input_verification_address}\",\n  \"decryption_address\": \"{decryption_address}\",\n  \"coprocessor_threshold\": {coprocessor_threshold},\n  \"public_decryption_threshold\": {public_decryption_threshold},\n  \"coprocessor_signers\": [{coprocessor_signers}],\n  \"kms_signers\": [{kms_signers}]\n}}\n",
            rpc_url = addresses.rpc_url,
            ws_url = addresses.ws_url,
            program_id = addresses.program_id,
            state_pda = addresses.state_pda,
            session_pda = addresses.session_pda,
            test_input_program_id = addresses.test_input_program_id,
            test_input_state_pda = addresses.test_input_state_pda,
            confidential_token_program_id = addresses.confidential_token_program_id,
            confidential_token_state_pda = addresses.confidential_token_state_pda,
            authority = addresses.authority,
            token_recipient = addresses.token_recipient,
            host_chain_id = addresses.host_chain_id,
            gateway_chain_id = addresses.gateway_chain_id,
            input_verification_address = evm_address_string(addresses.input_verification_address),
            decryption_address = evm_address_string(addresses.decryption_address),
            coprocessor_threshold = addresses.coprocessor_threshold,
            public_decryption_threshold = addresses.public_decryption_threshold,
            coprocessor_signers = json_signer_list(&addresses.coprocessor_signers),
            kms_signers = json_signer_list(&addresses.kms_signers),
        ),
    )?;

    Ok(())
}

fn env_signer_lines(prefix: &str, signers: &[EvmAddress]) -> String {
    let mut output = String::new();
    for (index, signer) in signers.iter().enumerate() {
        let _ = writeln!(
            output,
            "{prefix}{index}={address}",
            address = evm_address_string(*signer)
        );
    }
    output
}

fn json_signer_list(signers: &[EvmAddress]) -> String {
    signers
        .iter()
        .map(|signer| format!("\"{}\"", evm_address_string(*signer)))
        .collect::<Vec<_>>()
        .join(", ")
}

fn evm_address_string(address: EvmAddress) -> String {
    let mut output = String::from("0x");
    for byte in address.as_bytes() {
        let _ = write!(output, "{byte:02x}");
    }
    output
}

fn pubkey_hex_string(pubkey: Pubkey) -> String {
    let mut output = String::from("0x");
    for byte in pubkey.to_bytes() {
        let _ = write!(output, "{byte:02x}");
    }
    output
}

fn host_pubkey_hex_string(pubkey: HostPubkey) -> String {
    let mut output = String::from("0x");
    for byte in pubkey.as_bytes() {
        let _ = write!(output, "{byte:02x}");
    }
    output
}

fn parse_fhe_type(value: &str) -> DynResult<FheType> {
    match value {
        "bool" => Ok(FheType::Bool),
        "uint8" => Ok(FheType::Uint8),
        "uint16" => Ok(FheType::Uint16),
        "uint32" => Ok(FheType::Uint32),
        "uint64" => Ok(FheType::Uint64),
        "uint128" => Ok(FheType::Uint128),
        "uint160" | "address" => Ok(FheType::Uint160),
        "uint256" => Ok(FheType::Uint256),
        _ => Err(format!("unsupported type: {value}").into()),
    }
}

fn parse_binary_operator(value: &str) -> DynResult<Operator> {
    match value {
        "add" => Ok(Operator::FheAdd),
        "sub" => Ok(Operator::FheSub),
        "mul" => Ok(Operator::FheMul),
        _ => Err(format!("unsupported binary operator: {value}").into()),
    }
}

fn parse_options(args: &[String]) -> DynResult<HashMap<String, String>> {
    let mut options = HashMap::new();
    let mut idx = 0;
    while idx < args.len() {
        let key = args
            .get(idx)
            .ok_or("missing option name")?
            .strip_prefix("--")
            .ok_or("expected --option-name")?
            .to_owned();
        let value = args
            .get(idx + 1)
            .ok_or_else(|| format!("missing value for --{key}"))?
            .to_owned();
        options.insert(key, value);
        idx += 2;
    }
    Ok(options)
}

fn required_option<'a>(options: &'a HashMap<String, String>, key: &str) -> DynResult<&'a str> {
    options
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("missing required option --{key}").into())
}

fn optional_u64(options: &HashMap<String, String>, key: &str, default: u64) -> DynResult<u64> {
    match options.get(key) {
        Some(value) => parse_u64(value),
        None => Ok(default),
    }
}

fn optional_bool(options: &HashMap<String, String>, key: &str, default: bool) -> DynResult<bool> {
    match options.get(key) {
        Some(value) => match value.as_str() {
            "1" | "true" | "TRUE" | "yes" | "on" => Ok(true),
            "0" | "false" | "FALSE" | "no" | "off" => Ok(false),
            _ => Err(format!("invalid boolean for --{key}: {value}").into()),
        },
        None => Ok(default),
    }
}

fn read_keypair(path: &str) -> DynResult<Keypair> {
    read_keypair_file(path)
        .map_err(|error| format!("failed to read keypair {path}: {error}").into())
}

fn required_value<'a>(values: &'a HashMap<String, String>, key: &str) -> DynResult<&'a str> {
    values
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("missing {key} in bootstrap configuration").into())
}

fn parse_value_u64(values: &HashMap<String, String>, key: &str) -> DynResult<u64> {
    parse_u64(required_value(values, key)?)
}

fn parse_value_u32(values: &HashMap<String, String>, key: &str) -> DynResult<u32> {
    Ok(required_value(values, key)?.parse()?)
}

fn parse_value_usize(values: &HashMap<String, String>, key: &str) -> DynResult<usize> {
    Ok(required_value(values, key)?.parse()?)
}

fn parse_value_evm_address(
    values: &HashMap<String, String>,
    key: &str,
) -> DynResult<EvmAddress> {
    parse_evm_address(required_value(values, key)?)
}

fn parse_pubkey(value: &str) -> DynResult<Pubkey> {
    Ok(Pubkey::from_str(value)?)
}

fn parse_host_pubkey(value: &str) -> DynResult<HostPubkey> {
    if value.starts_with("0x") {
        return Ok(HostPubkey::from(parse_hex_32(value)?));
    }
    Ok(HostPubkey::from(parse_pubkey(value)?.to_bytes()))
}

fn parse_env_usize(name: &str) -> DynResult<usize> {
    Ok(env::var(name)?.parse()?)
}

fn parse_u64(value: &str) -> DynResult<u64> {
    Ok(value.parse()?)
}

fn parse_evm_address(value: &str) -> DynResult<EvmAddress> {
    let clean = value.strip_prefix("0x").unwrap_or(value);
    if clean.len() != 40 {
        return Err(format!("invalid EVM address length: {value}").into());
    }
    let mut bytes = [0_u8; 20];
    for (idx, chunk) in clean.as_bytes().chunks(2).enumerate() {
        bytes[idx] = u8::from_str_radix(std::str::from_utf8(chunk)?, 16)?;
    }
    Ok(EvmAddress::new(bytes))
}

fn parse_hex_32(value: &str) -> DynResult<[u8; 32]> {
    let clean = value.strip_prefix("0x").unwrap_or(value);
    if clean.len() != 64 {
        return Err(format!("expected 32-byte hex value, got {value}").into());
    }
    let mut bytes = [0_u8; 32];
    for (idx, chunk) in clean.as_bytes().chunks(2).enumerate() {
        bytes[idx] = u8::from_str_radix(std::str::from_utf8(chunk)?, 16)?;
    }
    Ok(bytes)
}

fn parse_handle(value: &str) -> DynResult<solana_host_contracts_core::Handle> {
    Ok(solana_host_contracts_core::Handle::from(parse_hex_32(
        value,
    )?))
}

fn parse_handles_csv(value: &str) -> DynResult<Vec<solana_host_contracts_core::Handle>> {
    value
        .split(',')
        .map(|item| parse_handle(item.trim()))
        .collect()
}

fn scalar_word_from_u64(value: u64) -> [u8; 32] {
    let mut output = [0_u8; 32];
    output[24..].copy_from_slice(&value.to_be_bytes());
    output
}

fn scalar_word_from_literal(value: &str) -> DynResult<[u8; 32]> {
    if let Some(clean) = value.strip_prefix("0x") {
        if clean.len() > 64 || clean.len() % 2 != 0 {
            return Err(format!("invalid hex scalar literal: {value}").into());
        }
        let bytes = hex::decode(clean)?;
        let mut output = [0_u8; 32];
        output[(32 - bytes.len())..].copy_from_slice(&bytes);
        return Ok(output);
    }

    let parsed: u128 = value.parse()?;
    let mut output = [0_u8; 32];
    output[16..].copy_from_slice(&parsed.to_be_bytes());
    Ok(output)
}

fn parse_optional_hex_bytes(value: Option<&str>) -> DynResult<Option<Vec<u8>>> {
    let Some(value) = value else {
        return Ok(None);
    };

    let clean = value.strip_prefix("0x").unwrap_or(value);
    Ok(Some(hex::decode(clean)?))
}

fn parse_hex_bytes(value: &str) -> DynResult<Vec<u8>> {
    let clean = value.strip_prefix("0x").unwrap_or(value);
    Ok(hex::decode(clean)?)
}

fn load_private_signing_keys(count: usize, prefix: &str) -> DynResult<Vec<SigningKey>> {
    let mut signing_keys = Vec::with_capacity(count);
    for index in 0..count {
        let variable = format!("{prefix}{index}");
        let private_key = env::var(&variable)
            .map_err(|_| format!("missing {variable}; source solana-host-contracts/.env.example before running verify-input scenarios"))?;
        let bytes = parse_hex_32(&private_key)?;
        signing_keys.push(SigningKey::from_bytes((&bytes).into())?);
    }
    Ok(signing_keys)
}

fn sign_message(signing_key: &SigningKey, message: &[u8]) -> Vec<u8> {
    let (signature, recovery_id) = signing_key
        .sign_digest_recoverable(Keccak256::new_with_prefix(message))
        .expect("sign message");
    let mut bytes = signature.to_bytes().to_vec();
    bytes.push(recovery_id.to_byte());
    bytes
}

fn runtime_host_chain_id(runtime: &LocalRuntime) -> DynResult<u64> {
    let addresses = load_addresses_env(&runtime.addresses_env)?;
    addresses
        .get("SOLANA_HOST_CHAIN_ID")
        .ok_or("missing SOLANA_HOST_CHAIN_ID in addresses env".into())
        .and_then(|value| parse_u64(value))
}

fn runtime_gateway_chain_id(runtime: &LocalRuntime) -> DynResult<u64> {
    let addresses = load_addresses_env(&runtime.addresses_env)?;
    addresses
        .get("CHAIN_ID_GATEWAY")
        .ok_or("missing CHAIN_ID_GATEWAY in addresses env".into())
        .and_then(|value| parse_u64(value))
}

fn runtime_input_verifier_address(runtime: &LocalRuntime) -> DynResult<EvmAddress> {
    let addresses = load_addresses_env(&runtime.addresses_env)?;
    addresses
        .get("INPUT_VERIFICATION_ADDRESS")
        .ok_or("missing INPUT_VERIFICATION_ADDRESS in addresses env".into())
        .and_then(|value| parse_evm_address(value))
}

fn fhe_type_label(fhe_type: FheType) -> &'static str {
    match fhe_type {
        FheType::Bool => "bool",
        FheType::Uint8 => "uint8",
        FheType::Uint16 => "uint16",
        FheType::Uint32 => "uint32",
        FheType::Uint64 => "uint64",
        FheType::Uint128 => "uint128",
        FheType::Uint160 => "address",
        FheType::Uint256 => "uint256",
        _ => "unknown",
    }
}

fn ensure_parent_dir(path: &Path) -> DynResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn load_addresses_env(path: &Path) -> DynResult<HashMap<String, String>> {
    let contents = fs::read_to_string(path)?;
    Ok(parse_env_contents(&contents))
}

fn parse_env_contents(contents: &str) -> HashMap<String, String> {
    let mut values = HashMap::new();
    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        values.insert(
            key.trim().to_owned(),
            value.trim().trim_matches('"').to_owned(),
        );
    }
    values
}

fn load_local_bootstrap_env() -> DynResult<HashMap<String, String>> {
    let local_cli_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let solana_root = local_cli_root.join("..");
    let gateway_env = solana_root.join("../test-suite/fhevm/env/staging/.env.gateway-sc.local");
    let host_env = solana_root.join("../test-suite/fhevm/env/staging/.env.host-sc.local");

    let mut merged = parse_env_contents(&fs::read_to_string(solana_root.join(".env.example"))?);
    if gateway_env.exists() {
        merged.extend(parse_env_contents(&fs::read_to_string(gateway_env)?));
    }
    if host_env.exists() {
        merged.extend(parse_env_contents(&fs::read_to_string(host_env)?));
    }

    for key in ENV_OVERRIDE_KEYS {
        if let Ok(value) = env::var(key) {
            merged.insert((*key).to_owned(), value);
        }
    }

    for (key, value) in env::vars() {
        if key.starts_with("PRIVATE_KEY_KMS_SIGNER_")
            || key.starts_with("PRIVATE_KEY_COPROCESSOR_ACCOUNT_")
            || key.starts_with("KMS_SIGNER_ADDRESS_")
            || key.starts_with("COPROCESSOR_SIGNER_ADDRESS_")
        {
            merged.insert(key, value);
        }
    }

    prefer_configured_signer_addresses(
        &mut merged,
        "PRIVATE_KEY_COPROCESSOR_ACCOUNT_",
        "COPROCESSOR_SIGNER_ADDRESS_",
    );
    prefer_configured_signer_addresses(
        &mut merged,
        "PRIVATE_KEY_KMS_SIGNER_",
        "KMS_SIGNER_ADDRESS_",
    );

    Ok(merged)
}

fn prefer_configured_signer_addresses(
    values: &mut HashMap<String, String>,
    private_key_prefix: &str,
    address_prefix: &str,
) {
    let suffixes = values
        .keys()
        .filter_map(|key| key.strip_prefix(address_prefix).map(str::to_owned))
        .collect::<Vec<_>>();
    for suffix in suffixes {
        values.remove(&format!("{private_key_prefix}{suffix}"));
    }
}

fn default_payer_keypair_path() -> DynResult<String> {
    if let Ok(path) = env::var("ANCHOR_WALLET") {
        return Ok(path);
    }
    if let Ok(path) = env::var("SOLANA_KEYPAIR") {
        return Ok(path);
    }
    let anchor_fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../tests/fixtures/anchor-authority.json");
    if anchor_fixture.exists() {
        return Ok(anchor_fixture.display().to_string());
    }
    let home = env::var("HOME")?;
    Ok(format!("{home}/.config/solana/id.json"))
}

fn default_addresses_env_path() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../addresses/.env.host")
        .display()
        .to_string()
}

fn default_addresses_json_path() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../addresses/localnet.json")
        .display()
        .to_string()
}

fn default_token_recipient_keypair_path() -> DynResult<String> {
    if let Ok(path) = env::var("SOLANA_TOKEN_RECIPIENT_KEYPAIR") {
        return Ok(path);
    }
    let recipient_fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../tests/fixtures/confidential-token-recipient.json");
    if recipient_fixture.exists() {
        return Ok(recipient_fixture.display().to_string());
    }
    Err("missing confidential token recipient keypair fixture".into())
}

fn token_recipient_pubkey() -> DynResult<Pubkey> {
    Ok(read_keypair(&default_token_recipient_keypair_path()?)?.pubkey())
}

fn handles_to_hex(handles: &[solana_host_contracts_core::Handle]) -> Vec<String> {
    handles
        .iter()
        .map(|handle| handle_to_hex(*handle))
        .collect()
}

fn handle_to_hex(handle: solana_host_contracts_core::Handle) -> String {
    format!("0x{}", hex::encode(handle.as_bytes()))
}

fn print_scenario_result(
    name: &str,
    signatures: Vec<String>,
    produced_handles: Vec<solana_host_contracts_core::Handle>,
    final_handles: Vec<solana_host_contracts_core::Handle>,
    expected: Vec<(&str, &str)>,
    incompatibilities: Vec<String>,
) -> DynResult<()> {
    let expected_json = expected
        .into_iter()
        .map(|(value, output_type)| {
            json!({
                "value": value,
                "output_type": output_type,
            })
        })
        .collect::<Vec<_>>();

    print_json(&json!({
        "scenario": name,
        "signatures": signatures,
        "produced_handles": handles_to_hex(&produced_handles),
        "final_handles": handles_to_hex(&final_handles),
        "expected": expected_json,
        "incompatibilities": incompatibilities,
    }))
}

fn print_json(value: &Value) -> DynResult<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

fn print_usage() {
    eprintln!(
        "Usage:\n\
  local-cli init-local --rpc-url <url> --ws-url <url> [--output-rpc-url <url>] [--output-ws-url <url>] --payer-keypair <path> --program-id <pubkey> --test-input-program-id <pubkey> --confidential-token-program-id <pubkey> --addresses-env <path> --addresses-json <path>\n\
  local-cli bootstrap-localnet [--rpc-url <url>] [--ws-url <url>] [--output-rpc-url <url>] [--output-ws-url <url>] [--payer-keypair <path>] [--recipient-keypair <path>] [--program-id <pubkey>] [--test-input-program-id <pubkey>] [--confidential-token-program-id <pubkey>] [--addresses-env <path>] [--addresses-json <path>] [--wait-for-programs 1] [--timeout-secs 180]\n\
  local-cli smoke-rand [--addresses-env ../addresses/.env.host] [--payer-keypair <path>] [--rpc-url <url>] [--program-id <pubkey>] [--rand-type uint8] [--session-nonce 1]\n\
  local-cli trivial-encrypt --value <n> --to-type <type> [--addresses-env ../addresses/.env.host] [--session-nonce 1]\n\
  local-cli binary-op --op <add|sub|mul> --lhs <0xhandle> (--rhs-handle <0xhandle> | --rhs-scalar <n>) --result-type <type> [--addresses-env ../addresses/.env.host] [--session-nonce 1]\n\
  local-cli cast --handle <0xhandle> --to-type <type> [--addresses-env ../addresses/.env.host] [--session-nonce 1]\n\
  local-cli allow --handle <0xhandle> --account <pubkey> [--addresses-env ../addresses/.env.host] [--session-nonce 1]\n\
  local-cli allow-for-decryption --handles <0xhandle[,0xhandle...]> [--addresses-env ../addresses/.env.host] [--session-nonce 1]\n\
  local-cli verify-input [--input-type uint64] [--ciphertext <text>] [--user-address <0x...>] [--contract-address <0x...>] [--extra-data <0x...>] [--addresses-env ../addresses/.env.host] [--session-nonce 1]\n\
  local-cli scenario-basic [--addresses-env ../addresses/.env.host] [--session-nonce 1]\n\
  local-cli scenario-scalar [--addresses-env ../addresses/.env.host] [--session-nonce 1]\n\
  local-cli scenario-cast [--addresses-env ../addresses/.env.host] [--session-nonce 1]\n\
  local-cli scenario-batch [--addresses-env ../addresses/.env.host] [--session-nonce 1]\n\
  local-cli scenario-public-ebool [--addresses-env ../addresses/.env.host] [--session-nonce 1]\n\
  local-cli scenario-public-mixed [--addresses-env ../addresses/.env.host] [--session-nonce 1]\n\
  local-cli scenario-user-decrypt [--addresses-env ../addresses/.env.host] [--user-id <pubkey|0x...>]\n\
  local-cli runtime-identities [--addresses-env ../addresses/.env.host]\n\
  local-cli scenario-input-proof [--addresses-env ../addresses/.env.host] [--input-handle <0x...>] [--input-proof <0x...>] [--user-id <pubkey|0x...>] [--session-nonce 1]\n\
  local-cli scenario-test-input-add42 [--addresses-env ../addresses/.env.host] [--input-handle <0x...>] [--input-proof <0x...>] [--user-id <pubkey|0x...>] [--session-nonce 1]\n\
  local-cli scenario-confidential-token [--addresses-env ../addresses/.env.host] [--input-handle <0x...>] [--input-proof <0x...>] [--user-id <pubkey|0x...>]\n\
  local-cli token-reset [--addresses-env ../addresses/.env.host]\n\
  local-cli token-mint-to --amount <n> [--recipient <pubkey>] [--addresses-env ../addresses/.env.host]\n\
  local-cli token-transfer --recipient <pubkey> --input-handle <0x...> --input-proof <0x...> [--addresses-env ../addresses/.env.host]\n\
  local-cli token-approve-delegate --delegate <pubkey> --input-handle <0x...> --input-proof <0x...> [--addresses-env ../addresses/.env.host]\n\
  local-cli token-transfer-as-delegate --source <pubkey> --recipient <pubkey> --input-handle <0x...> --input-proof <0x...> [--addresses-env ../addresses/.env.host] [--payer-keypair <path>]\n\
  local-cli token-balance --owner <pubkey> [--addresses-env ../addresses/.env.host] [--payer-keypair <path>]\n\
  local-cli token-delegate-allowance --owner <pubkey> --delegate <pubkey> [--addresses-env ../addresses/.env.host] [--payer-keypair <path>]\n\
  local-cli token-supply [--addresses-env ../addresses/.env.host] [--payer-keypair <path>]\n\
  local-cli show-pdas --program-id <pubkey> --authority <pubkey>\n\
\n\
Environment variables for init-local are read from the current process environment.\n\
Required values include: SOLANA_HOST_CHAIN_ID, CHAIN_ID_GATEWAY, INPUT_VERIFICATION_ADDRESS,\n\
DECRYPTION_ADDRESS, NUM_KMS_NODES, PUBLIC_DECRYPTION_THRESHOLD, NUM_COPROCESSORS,\n\
COPROCESSOR_THRESHOLD, HCU_CAP_PER_BLOCK, MAX_HCU_DEPTH_PER_TX, MAX_HCU_PER_TX,\n\
and either signer private keys or signer addresses for the configured KMS/coprocessor nodes.\n\
\n\
Runtime commands default to the Anchor wallet at $ANCHOR_WALLET or ~/.config/solana/id.json.\n\
1 SOL is {LAMPORTS_PER_SOL} lamports on the local validator."
    );
}

struct LocalAddresses {
    rpc_url: String,
    ws_url: String,
    program_id: Pubkey,
    state_pda: Pubkey,
    session_pda: Pubkey,
    test_input_program_id: Pubkey,
    test_input_state_pda: Pubkey,
    confidential_token_program_id: Pubkey,
    confidential_token_state_pda: Pubkey,
    authority: Pubkey,
    token_recipient: Pubkey,
    host_chain_id: u64,
    gateway_chain_id: u64,
    input_verification_address: EvmAddress,
    decryption_address: EvmAddress,
    coprocessor_signers: Vec<EvmAddress>,
    kms_signers: Vec<EvmAddress>,
    coprocessor_threshold: u32,
    public_decryption_threshold: u32,
}

struct LocalRuntime {
    rpc_url: String,
    program_id: Pubkey,
    payer: Keypair,
    state_pda: Pubkey,
    session_pda: Pubkey,
    test_input_program_id: Pubkey,
    test_input_state_pda: Pubkey,
    confidential_token_program_id: Pubkey,
    confidential_token_state_pda: Pubkey,
    #[allow(dead_code)]
    addresses_env: PathBuf,
}

struct ExecutionReceipt {
    signature: String,
    returned_handles: Vec<solana_host_contracts_core::Handle>,
}

struct TokenExecutionReceipt {
    signature: String,
    returned_handles: Vec<solana_host_contracts_core::Handle>,
    total_supply: Option<u64>,
}

struct InputVerificationBundle {
    context: ContextUserInputs,
    handles: Vec<solana_host_contracts_core::Handle>,
    selected_handle: solana_host_contracts_core::Handle,
    input_proof: Vec<u8>,
}

#[derive(Clone, Copy)]
struct NoopVerifier;

impl solana_host_contracts_core::InputProofVerifier for NoopVerifier {
    fn verify(
        &self,
        _payload: &solana_host_contracts_core::CiphertextVerification,
        _signatures: &[Vec<u8>],
        _signers: &[EvmAddress],
        _threshold: u32,
        _source_chain_id: u64,
        _source_contract: EvmAddress,
    ) -> solana_host_contracts_core::Result<()> {
        Err(solana_host_contracts_core::HostContractError::InvalidSigner)
    }
}

impl solana_host_contracts_core::KmsProofVerifier for NoopVerifier {
    fn verify(
        &self,
        _payload: &solana_host_contracts_core::PublicDecryptVerification,
        _signatures: &[Vec<u8>],
        _signers: &[EvmAddress],
        _threshold: u32,
        _source_chain_id: u64,
        _source_contract: EvmAddress,
    ) -> solana_host_contracts_core::Result<()> {
        Err(solana_host_contracts_core::HostContractError::InvalidKmsSigner)
    }
}

#[allow(dead_code)]
fn simulate_instructions(
    config: HostProgramConfig,
    caller: Pubkey,
    slot: u64,
    timestamp: i64,
    recent_blockhash: [u8; 32],
    instructions: &[HostInstruction],
) -> DynResult<Vec<InstructionResult>> {
    let mut state = HostProgramState::new(config)?;
    let mut session = HostProgramSession::default();
    let context = ProgramContext {
        caller: HostPubkey::from(caller.to_bytes()),
        chain_id: state.host_chain_id(),
        slot,
        timestamp,
        recent_blockhash,
    };
    let proof_verifier = NoopVerifier;
    let mut results = Vec::with_capacity(instructions.len());
    for instruction in instructions {
        results.push(state.process_instruction(
            instruction,
            context,
            &mut session,
            &proof_verifier,
            &proof_verifier,
        )?);
    }
    Ok(results)
}
