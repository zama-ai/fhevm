//! Drives zama-host / confidential-token instructions against the LOCAL validator,
//! building each from the program crates' own anchor-generated instruction/accounts
//! types + pub PDA derivations (authoritative, not hand-replicated). Uses anchor-client
//! so all solana types are version-coherent with anchor-lang 1.0.2.
//!
//! Live flow proven so far:
//!   1. initialize_host_config  (host program executes real on-chain logic)
//!   2. initialize_mint         (host<->token CPI: trivial-encrypt + ACL-record init)
//!
//! Underlying SPL mint is passed via $UNDERLYING_MINT (create it with `spl-token
//! create-token`). RPC is pinned to the local validator — never mainnet.

use std::rc::Rc;
use std::str::FromStr;

use anchor_client::{Client, Cluster, CommitmentConfig, Program, Signer};
use anchor_lang::solana_program::{pubkey::Pubkey, system_program};
use anchor_lang::{AccountDeserialize, AnchorDeserialize};
use serde::Deserialize;
use solana_keypair::{read_keypair_file, Keypair};

const EVENT_AUTHORITY_SEED: &[u8] = b"__event_authority";
const HISTORICAL_LABEL_MARKER: u8 = 3;
const MMR_MODE_HISTORICAL: u8 = 0x01;
const MMR_MODE_PUBLIC: u8 = 0x02;

type LineageState = ([u8; 32], Vec<Pubkey>);

struct DurableEvalTarget {
    value: u64,
    plaintext: [u8; 32],
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    value_key: [u8; 32],
    encrypted_value: Pubkey,
    context_id: [u8; 32],
}

struct DurableEvalResult {
    encrypted_value: Pubkey,
    value_key: [u8; 32],
    handle: [u8; 32],
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let payer =
        Rc::new(read_keypair_file(format!("{home}/.config/solana/id.json")).expect("wallet"));
    let cluster = Cluster::Custom("http://127.0.0.1:8899".into(), "ws://127.0.0.1:8900".into());
    let client = Client::new_with_options(cluster, payer.clone(), CommitmentConfig::confirmed());
    let host = client.program(zama_host::ID)?;
    let token = client.program(confidential_token::ID)?;

    let (host_config, _) =
        Pubkey::find_program_address(&[zama_host::HOST_CONFIG_SEED], &zama_host::ID);

    // BOOTSTRAP initializes the singleton HostConfig + active KMS context from the REAL live
    // gateway/ProtocolConfig values (passed via env). Typed anchor-client path that replaces
    // the former bootstrap.mjs (@solana/web3.js); mock-input + test-shims OFF.
    if std::env::var("BOOTSTRAP").is_ok() {
        bootstrap(&host, &payer, host_config)?;
        return Ok(());
    }

    // TRIVIAL_ENCRYPT drives a real zama-host FHE op (trivial encryption): the program
    // computes the result handle on-chain and emits a TrivialEncryptEvent the live
    // host-listener ingests into the coprocessor DB for the tfhe-worker to materialize.
    if std::env::var("TRIVIAL_ENCRYPT").is_ok() {
        trivial_encrypt_and_bind(&host, &payer, host_config)?;
        return Ok(());
    }

    // TRIVIAL_ENCRYPT_EVAL drives the SAME trivial-encryption through the eval-plan executor
    // (fhe_eval): a single TrivialEncrypt step with a durable output ACL record. The host computes
    // the result handle on-chain and emits the same TrivialEncryptEvent the host-listener ingests,
    // exercising the #2755 eval path instead of the standalone trivial_encrypt_and_bind.
    if std::env::var("TRIVIAL_ENCRYPT_EVAL").is_ok() {
        trivial_encrypt_eval(&host, &payer, host_config)?;
        return Ok(());
    }

    // HISTORICAL_STEP=compute creates the old handle, then the shell waits for SNS materialization.
    // HISTORICAL_STEP=supersede rotates the same lineage and prints the historical MMR proof.
    if let Ok(step) = std::env::var("HISTORICAL_STEP") {
        historical_supersede_step(&host, &payer, host_config, &step)?;
        return Ok(());
    }

    // PUBLIC_DECRYPT_PROOF emits a mode-0x02 PublicDecryptLeaf MMR proof for a handle already
    // released by make_handle_public. Inputs: PUB_HANDLE and either PUB_ACL or PUB_ACL_VALUE_KEY.
    if std::env::var("PUBLIC_DECRYPT_PROOF").is_ok() {
        public_decrypt_proof_step(&host)?;
        return Ok(());
    }

    // FHE_EVAL_VERIFIED_INPUT drives the #1539 input flow in one fhe_eval: a Binary Add of a
    // coprocessor-attested external input (FheEvalOperand::VerifiedInput, re-verified in-frame via
    // secp256k1 with no scratch PDA) and a public scalar, binding the result to a durable output ACL
    // record under the attested acl_domain_key. The attestation comes from the same relayer
    // input-proof the BIND_INPUT leg uses (BIND_* env); TE_ADD is the scalar addend (default 2);
    // TE_ALLOW makes the result publicly decryptable. Proves encrypt V -> +2 -> decrypt V+2.
    if std::env::var("FHE_EVAL_VERIFIED_INPUT").is_ok() {
        fhe_eval_verified_input_add(&host, &payer, host_config)?;
        return Ok(());
    }

    // CONSUME_WRAP: deposit public USDC into a confidential balance (wrap_usdc) — the balance
    // a subsequent confidential_burn draws from on the redeem path. MINT + WRAP_AMOUNT via env.
    if std::env::var("CONSUME_WRAP").is_ok() {
        consume_wrap(&token, &payer, host_config)?;
        return Ok(());
    }
    // CONSUME_AMOUNT: stand up a confidential token account and mint a token-scoped random
    // encrypted amount (a real transfer-amount handle), the operand the disclose path needs.
    if std::env::var("CONSUME_AMOUNT").is_ok() {
        consume_amount(&token, &payer, host_config)?;
        return Ok(());
    }
    // CONSUME_SEAL: seal a token amount's ciphertext material on-chain and request public
    // disclosure (releases the amount for public decrypt) — the precondition for disclose.
    if std::env::var("CONSUME_SEAL").is_ok() {
        consume_seal(&host, &token, &payer, host_config)?;
        return Ok(());
    }
    // CONSUME_DISCLOSE: verify the KMS PublicDecryptVerification cert on-chain via secp256k1
    // (disclose_amount_secp) — the Consume seam against the ProtocolConfig-mirrored KMS context.
    if std::env::var("CONSUME_DISCLOSE").is_ok() {
        consume_disclose(&token, &payer, host_config)?;
        return Ok(());
    }
    // CONSUME_BURN: burn an encrypted amount from the confidential balance (confidential_burn) —
    // the heaviest FHE instruction (5 host CPIs). Produces the burned-amount handle the redeem
    // path publicly decrypts, then releases against the vault. MINT via env.
    if std::env::var("CONSUME_BURN").is_ok() {
        consume_burn(&token, &payer, host_config)?;
        return Ok(());
    }
    // CONSUME_REQUEST_REDEEM: create the burn-redemption request witness (request_burn_redemption)
    // pinning the host's current KMS context id + expires_slot + request_hash before the secp
    // consume. Releases the burned handle for public decrypt via the zama-host allow CPI.
    if std::env::var("CONSUME_REQUEST_REDEEM").is_ok() {
        consume_request_redeem(&host, &token, &payer, host_config)?;
        return Ok(());
    }
    // CONSUME_REDEEM: redeem a KMS-certified burned amount from the SPL vault
    // (redeem_burned_amount_secp) — commits the burned handle's material, then verifies the KMS
    // PublicDecryptVerification cert on-chain via secp256k1 and releases the cleartext amount of
    // underlying USDC to the owner. The vault-releasing Consume seam.
    if std::env::var("CONSUME_REDEEM").is_ok() {
        consume_redeem(&host, &token, &payer, host_config)?;
        return Ok(());
    }

    // FHE_EVAL_BINARY drives a single binary fhe_eval step. BINARY_OP names the op (Sub/Mul/…);
    // BINARY_A and BINARY_B are the u64 operands (defaults 10/5); BINARY_FHE_TYPE is the input
    // FHE type byte (default 5 = euint64). BINARY_B_SCALAR=1 passes rhs as a plaintext scalar.
    // Output FHE type is auto-derived: 0 (ebool) for comparison ops, else BINARY_FHE_TYPE.
    // BINARY_ALLOW marks the result publicly decryptable.
    if std::env::var("FHE_EVAL_BINARY").is_ok() {
        fhe_eval_binary(&host, &payer, host_config)?;
        return Ok(());
    }

    // FHE_EVAL_UNARY drives a single unary fhe_eval step. UNARY_OP names the op (Neg/Not/Cast);
    // UNARY_A is the u64 operand (default 42); UNARY_IN_FHE_TYPE is the input type (default 2 =
    // euint8); UNARY_OUT_FHE_TYPE is the output type (defaults to UNARY_IN_FHE_TYPE). UNARY_ALLOW
    // marks the result publicly decryptable.
    if std::env::var("FHE_EVAL_UNARY").is_ok() {
        fhe_eval_unary(&host, &payer, host_config)?;
        return Ok(());
    }

    // FHE_EVAL_TERNARY drives a ternary IfThenElse fhe_eval step. TERNARY_CTRL (0 or 1, default 1)
    // selects the branch; TERNARY_TRUE/TERNARY_FALSE are the euint64 branch values (defaults 42/99);
    // TERNARY_FHE_TYPE selects the branch FHE type (default 5). TERNARY_ALLOW marks the result
    // publicly decryptable.
    if std::env::var("FHE_EVAL_TERNARY").is_ok() {
        fhe_eval_ternary(&host, &payer, host_config)?;
        return Ok(());
    }

    // FHE_EVAL_RAND_BOUNDED drives a bounded random fhe_eval step. RAND_UPPER is the exclusive
    // u64 upper bound (default 100); RAND_FHE_TYPE is the output FHE type (default 5 = euint64).
    // RAND_ALLOW marks the result publicly decryptable.
    if std::env::var("FHE_EVAL_RAND_BOUNDED").is_ok() {
        fhe_eval_rand_bounded(&host, &payer, host_config)?;
        return Ok(());
    }

    // FHE_EVAL_SUM drives a multi-step fhe_eval: two TrivialEncrypt steps (AllowedLocal) followed
    // by a Sum step (AllowedDurable). SUM_A/SUM_B select the euint64 addends (defaults 10/20);
    // SUM_ALLOW makes the result publicly decryptable. Expected cleartext: SUM_A + SUM_B.
    if std::env::var("FHE_EVAL_SUM").is_ok() {
        fhe_eval_sum(&host, &payer, host_config)?;
        return Ok(());
    }

    // FHE_EVAL_IS_IN drives a multi-step fhe_eval: TrivialEncrypt steps for value and set elements
    // (all AllowedLocal) followed by an IsIn step (AllowedDurable → ebool). ISIN_VALUE selects the
    // euint64 value (default 42); the set is hardcoded as [10, 42, 100]. ISIN_ALLOW makes the
    // result publicly decryptable. Expected cleartext: 1 (true) when ISIN_VALUE is in the set.
    if std::env::var("FHE_EVAL_IS_IN").is_ok() {
        fhe_eval_is_in(&host, &payer, host_config)?;
        return Ok(());
    }

    // FHE_EVAL_MUL_DIV drives a multi-step fhe_eval: TrivialEncrypt(factor1, AllowedLocal) then
    // MulDiv(factor1, scalar_factor2, divisor, AllowedDurable). MULDIV_A/MULDIV_B/MULDIV_D select
    // the euint64 operands (defaults 6/7/3); MULDIV_ALLOW makes the result publicly decryptable.
    // Expected cleartext: MULDIV_A * MULDIV_B / MULDIV_D (integer division).
    if std::env::var("FHE_EVAL_MUL_DIV").is_ok() {
        fhe_eval_mul_div(&host, &payer, host_config)?;
        return Ok(());
    }

    ensure_host_config(&host, &payer, host_config)?;
    initialize_mint(&token, &payer, host_config)?;
    Ok(())
}

/// Decodes a `0x`-prefixed or bare hex string into bytes.
fn hexdec(s: &str) -> Vec<u8> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
        .collect()
}

/// Parses a 0x-prefixed 20-byte EVM address.
fn addr20(s: &str) -> [u8; 20] {
    hexdec(s)
        .try_into()
        .expect("expected a 20-byte EVM address")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn hex_csv(values: &[[u8; 32]]) -> String {
    values
        .iter()
        .map(|value| format!("0x{}", hex(value)))
        .collect::<Vec<_>>()
        .join(",")
}

fn te_value() -> u64 {
    std::env::var("TE_VALUE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(42)
}

fn encrypted_value_address(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
) -> Pubkey {
    let value_key = zama_solana_acl::derive_value_key(
        acl_domain_key.to_bytes(),
        app_account.to_bytes(),
        encrypted_value_label,
    );
    zama_host::encrypted_value_address(value_key).0
}

fn durable_eval_target(payer: &Rc<Keypair>, label_marker: u8) -> DurableEvalTarget {
    let value = te_value();
    let mut plaintext = [0u8; 32];
    plaintext[24..32].copy_from_slice(&value.to_be_bytes());

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let mut encrypted_value_label = [label_marker; 32];
    encrypted_value_label[24..32].copy_from_slice(&value.to_be_bytes());
    let value_key = zama_solana_acl::derive_value_key(
        acl_domain_key.to_bytes(),
        app_account.to_bytes(),
        encrypted_value_label,
    );
    let encrypted_value =
        encrypted_value_address(acl_domain_key, app_account, encrypted_value_label);

    let mut context_id = [0u8; 32];
    context_id[0] = 0xc0;
    context_id[1] = label_marker;
    context_id[24..32].copy_from_slice(&value.to_be_bytes());

    DurableEvalTarget {
        value,
        plaintext,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        value_key,
        encrypted_value,
        context_id,
    }
}

fn fetch_encrypted_value(
    program: &Program<Rc<Keypair>>,
    encrypted_value: Pubkey,
) -> Result<zama_host::EncryptedValue, Box<dyn std::error::Error>> {
    let account = program.rpc().get_account(&encrypted_value)?;
    let mut data: &[u8] = &account.data;
    Ok(zama_host::EncryptedValue::try_deserialize(&mut data)?)
}

fn bytes32_env(name: &str) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    Ok(hexdec(&std::env::var(name)?).try_into().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("{name} must be 32 bytes"),
        )
    })?)
}

fn public_proof_encrypted_value_from_env() -> Result<Pubkey, Box<dyn std::error::Error>> {
    if let Ok(account) = std::env::var("PUB_ACL") {
        return Ok(Pubkey::from_str(&account)?);
    }
    if std::env::var("PUB_ACL_VALUE_KEY").is_ok() {
        return Ok(zama_host::encrypted_value_address(bytes32_env("PUB_ACL_VALUE_KEY")?).0);
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "PUBLIC_DECRYPT_PROOF requires PUB_ACL or PUB_ACL_VALUE_KEY",
    )
    .into())
}

fn existing_lineage_state(
    program: &Program<Rc<Keypair>>,
    encrypted_value: Pubkey,
) -> Result<Option<LineageState>, Box<dyn std::error::Error>> {
    match program.rpc().get_account(&encrypted_value) {
        Ok(account) if account.owner == zama_host::ID => {
            let mut data: &[u8] = &account.data;
            let value = zama_host::EncryptedValue::try_deserialize(&mut data)?;
            Ok(Some((value.current_handle, value.subjects)))
        }
        Ok(_) => Ok(None),
        Err(_) => Ok(None),
    }
}

fn owner_subject(pubkey: Pubkey) -> zama_host::AclSubjectEntry {
    zama_host::AclSubjectEntry { pubkey }
}

#[allow(clippy::too_many_arguments)]
fn durable_output(
    program: &Program<Rc<Keypair>>,
    encrypted_value: Pubkey,
    index: u16,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    subjects: Vec<zama_host::AclSubjectEntry>,
) -> Result<zama_host::FheEvalOutput, Box<dyn std::error::Error>> {
    let (previous_handle, previous_subjects) =
        match existing_lineage_state(program, encrypted_value)? {
            Some((handle, subjects)) => (Some(handle), Some(subjects)),
            None => (None, None),
        };
    Ok(zama_host::FheEvalOutput::AllowedDurable {
        output_encrypted_value_index: index,
        output_app_account_authority_index: None,
        output_acl_domain_key: acl_domain_key,
        output_app_account: app_account,
        output_encrypted_value_label: encrypted_value_label,
        output_subjects: subjects,
        previous_handle,
        previous_subjects,
        make_public: false,
    })
}

fn allow_for_decryption(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
    encrypted_value: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let grant_sig = host
        .request()
        .accounts(zama_host::accounts::AllowEncryptedValueSubjects {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            encrypted_value,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
        })
        .args(zama_host::instruction::AllowSubjects {
            subjects: vec![zama_host::instructions::EncryptedValueSubjectGrant {
                subject: payer.pubkey(),
            }],
        })
        .send()?;
    println!("OK allow_subjects (idempotent membership): {grant_sig}");

    let (handle, _) = existing_lineage_state(host, encrypted_value)?.ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "encrypted value lineage missing before make_handle_public",
        )
    })?;
    let public_sig = host
        .request()
        .accounts(zama_host::accounts::MakeEncryptedValueHandlePublic {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            encrypted_value,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
        })
        .args(zama_host::instruction::MakeHandlePublic { handle })
        .send()?;
    println!("OK allow_for_decryption (public): {public_sig}");
    Ok(())
}

/// Bootstraps the singleton HostConfig + the active KMS context from the REAL live
/// gateway/ProtocolConfig values (env-supplied), via the typed anchor-client path. Replaces
/// the former bootstrap.mjs (which used the deprecated @solana/web3.js and hand-rolled Anchor
/// discriminators). Idempotent: skips initialize_host_config if the singleton already exists.
/// mock-input + test-shims OFF — the live secp256k1 paths are authoritative.
fn bootstrap(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let authority = payer.pubkey();
    let chain_id: u64 = std::env::var("SOLANA_HOST_CHAIN_ID")?.parse()?;
    let gateway_chain_id: u64 = std::env::var("GATEWAY_CHAIN_ID")?.parse()?;
    let input_verification_contract = addr20(&std::env::var("INPUT_VERIFICATION_ADDRESS")?);
    let coprocessor_signer = addr20(&std::env::var("COPROCESSOR_SIGNER")?);
    let decryption_contract = addr20(&std::env::var("DECRYPTION_ADDRESS")?);
    let kms_signers: Vec<[u8; 20]> = std::env::var("KMS_SIGNERS")?
        .split(',')
        .map(|s| addr20(s.trim()))
        .collect();

    if host.rpc().get_account(&host_config).is_err() {
        let sig = host
            .request()
            .accounts(zama_host::accounts::InitializeHostConfig {
                payer: authority,
                admin: authority,
                host_config,
                system_program: system_program::ID,
            })
            .args(zama_host::instruction::InitializeHostConfig {
                args: zama_host::InitializeHostConfigArgs {
                    chain_id,
                    input_verifier_authority: authority, // inert: mock/signed input paths OFF
                    gateway_chain_id,
                    input_verification_contract,
                    coprocessor_signer,
                    decryption_contract,
                    material_authority: authority,
                    test_authority: authority, // inert: test shims OFF
                    mock_input_enabled: false,
                    test_shims_enabled: false,
                    grant_deny_list_enabled: false,
                },
            })
            .send()?;
        println!("OK initialize_host_config: {sig}");
    } else {
        println!("host_config already initialized — skipping initialize_host_config");
    }

    let context_id: u64 = 1;
    let (kms_context, _) = Pubkey::find_program_address(
        &[zama_host::KMS_CONTEXT_SEED, &context_id.to_le_bytes()],
        &zama_host::ID,
    );
    let signer_count = kms_signers.len();
    let sig = host
        .request()
        .accounts(zama_host::accounts::DefineKmsContext {
            admin: authority,
            host_config,
            kms_context,
            system_program: system_program::ID,
        })
        .args(zama_host::instruction::DefineKmsContext {
            context_id,
            signers: kms_signers,
            thresholds: zama_host::KmsThresholds {
                public_decryption: 1,
                user_decryption: 1,
                kms_gen: 1,
                mpc: 1,
            },
        })
        .send()?;
    println!("OK define_kms_context: {sig} (signers: {signer_count})");
    Ok(())
}

/// Drives a real zama-host trivial-encrypt FHE op: the program computes the result handle
/// on-chain (entropy-bound, no client pre-computation) and emits a TrivialEncryptEvent over
/// emit_cpi. The live host-listener ingests it into the coprocessor DB, where the tfhe-worker
/// materializes the trivial ciphertext. TE_VALUE selects the euint64 plaintext (default 42).
fn trivial_encrypt_and_bind(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    trivial_encrypt_eval_with_label(host, payer, host_config, 1, "trivial_encrypt_and_bind")?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn trivial_encrypt_eval_with_label(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
    label_marker: u8,
    ok_marker: &str,
) -> Result<DurableEvalResult, Box<dyn std::error::Error>> {
    use anchor_lang::solana_program::instruction::AccountMeta;

    let target = durable_eval_target(payer, label_marker);
    let fhe_type: u8 = 5; // euint64

    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![owner_subject(payer.pubkey())];

    let output = durable_output(
        host,
        target.encrypted_value,
        0,
        target.acl_domain_key,
        target.app_account,
        target.encrypted_value_label,
        subjects,
    )?;
    let args = zama_host::FheEvalArgs {
        context_id: target.context_id,
        steps: vec![zama_host::FheEvalStep::TrivialEncrypt {
            plaintext: target.plaintext,
            fhe_type,
            output,
        }],
    };

    // SlotHashes (read via sol_get_sysvar for the result-handle entropy) is populated in
    // real execution but not in RPC preflight simulation, so skip preflight for this op.
    let sig = host
        .request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            // Block-cap optional accounts: default cap is unrestricted, so existing flows
            // pass None/None and behave exactly as before the feature. The mandatory HCU
            // authority is the payer itself for this wallet-driven PoC leg.
            hcu_authority: payer.pubkey(),
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(vec![AccountMeta::new(target.encrypted_value, false)])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK {ok_marker} ({} as euint64): {sig}", target.value);

    let value_account = fetch_encrypted_value(host, target.encrypted_value)?;
    let handle = value_account.current_handle;
    let handle_hex = hex(&handle);
    println!("  output ACL record {}", target.encrypted_value);
    println!("  acl value key 0x{}", hex(&target.value_key));
    println!("  result handle 0x{handle_hex}  (tfhe-worker materializes this ciphertext)");

    if std::env::var("TE_ALLOW").is_ok() {
        allow_for_decryption(host, payer, host_config, target.encrypted_value)?;
    }
    Ok(DurableEvalResult {
        encrypted_value: target.encrypted_value,
        value_key: target.value_key,
        handle,
    })
}

/// Eval-based compute leg: drives a single-step fhe_eval plan (one TrivialEncrypt step with a
/// durable output ACL record) instead of the standalone trivial_encrypt_and_bind. The host runs
/// the eval executor, computes the result handle on-chain, creates the durable output ACL record
/// (passed as the sole remaining_account), and emits the same TrivialEncryptEvent the live
/// host-listener ingests for the tfhe-worker to materialize. TE_VALUE selects the euint64
/// plaintext; TE_ALLOW marks it publicly decryptable afterward.
fn trivial_encrypt_eval(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    trivial_encrypt_eval_with_label(host, payer, host_config, 2, "fhe_eval trivial_encrypt")?;
    Ok(())
}

fn build_historical_access_proof(
    encrypted_value_account: [u8; 32],
    peaks: &[[u8; 32]],
    leaf_count: u64,
    old_handle: [u8; 32],
    subject: [u8; 32],
) -> Result<(zama_solana_acl::MmrProof, Vec<u8>), Box<dyn std::error::Error>> {
    let leaf = zama_solana_acl::historical_access_leaf_commitment(
        encrypted_value_account,
        0,
        old_handle,
        subject,
    );
    let leaves = [leaf];
    let proof = zama_solana_acl::mmr_build_proof(&leaves, 0).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "failed to build historical MMR proof for leaf 0",
        )
    })?;
    if !zama_solana_acl::mmr_verify(peaks, leaf_count, leaf, &proof) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("historical MMR proof did not verify against live leaf_count={leaf_count}"),
        )
        .into());
    }

    let proof_bytes = proof_blob(MMR_MODE_HISTORICAL, &proof)?;
    Ok((proof, proof_bytes))
}

fn proof_blob(
    mode: u8,
    proof: &zama_solana_acl::MmrProof,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut proof_bytes = vec![mode];
    proof_bytes.extend_from_slice(&borsh::to_vec(proof)?);
    Ok(proof_bytes)
}

fn locate_public_decrypt_leaf_index(
    encrypted_value_account: [u8; 32],
    leaves: &[[u8; 32]],
    handle: [u8; 32],
) -> Option<u64> {
    leaves.iter().enumerate().find_map(|(index, leaf)| {
        let index = index as u64;
        let public_leaf =
            zama_solana_acl::public_decrypt_leaf_commitment(encrypted_value_account, index, handle);
        (*leaf == public_leaf).then_some(index)
    })
}

fn build_public_decrypt_proof(
    encrypted_value_account: [u8; 32],
    value_account: &zama_host::EncryptedValue,
    handle: [u8; 32],
) -> Result<(zama_solana_acl::MmrProof, Vec<u8>), Box<dyn std::error::Error>> {
    // The burned/disclosed lineage can hold MORE than one public-decrypt leaf for the
    // SAME handle: a born-public burn (DD-036) seals one, and a later explicit
    // `make_handle_public` seal appends another — the host has no live "already public"
    // flag by design (lib.rs authorize_public docs), so it never dedups. Reconstruct one
    // `MarkedPublic` per on-chain leaf so the reconstructed peaks/leaf_count match the
    // live account. NOTE (PoC shortcut): this assumes the lineage is entirely
    // same-handle public leaves, which holds for full-vertical's single-burn flow; a
    // lineage that also carries supersede (historical) leaves cannot be rebuilt from the
    // handle alone — route through the relayer proof service (`solana_proof::build_proof`)
    // for that (tracked follow-up).
    let events: Vec<zama_solana_acl::LineageEvent> =
        std::iter::repeat(zama_solana_acl::LineageEvent::MarkedPublic { handle })
            .take(value_account.leaf_count as usize)
            .collect();
    let reconstructed =
        zama_solana_acl::reconstruct(encrypted_value_account, &events).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("failed to reconstruct public-decrypt lineage: {e:?}"),
            )
        })?;
    let leaf_index =
        locate_public_decrypt_leaf_index(encrypted_value_account, &reconstructed.leaves, handle)
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "failed to locate PublicDecryptLeaf in reconstructed lineage",
                )
            })?;
    let proof = reconstructed
        .build_verified_proof(&value_account.peaks, value_account.leaf_count, leaf_index)
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("public-decrypt lineage reconstruction diverged from live account: {e:?}"),
            )
        })?;
    let shared = value_account.to_shared();
    zama_solana_acl::authorize_public(encrypted_value_account, &shared, handle, &proof).map_err(
        |e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("public-decrypt MMR proof did not verify against live lineage: {e:?}"),
            )
        },
    )?;
    let proof_bytes = proof_blob(MMR_MODE_PUBLIC, &proof)?;
    Ok((proof, proof_bytes))
}

/// Proof source for the public + historical MMR proofs.
/// - `Relayer` (default): fetch the inclusion proof from the running relayer proof service
///   (`GET /internal/solana/mmr-proof`). This is what the e2e uses — the relayer, not the PoC,
///   produces the accepted proof.
/// - `Local`: synthesize the proof in-process by reconstructing the lineage from thin chain facts.
///   Reserved for the born-public burned leg (unresolvable over RPC in the emitless arm — see the
///   follow-up issue) and the unit tests. Selected explicitly via `PROOF_SOURCE=local`.
enum ProofSource {
    Relayer,
    Local,
}

fn proof_source() -> ProofSource {
    match std::env::var("PROOF_SOURCE").ok().as_deref() {
        Some("local") => ProofSource::Local,
        // Anything else (unset or "relayer") is the relayer path: the e2e sources proofs from the
        // relayer by construction, so the synthetic path is never reached without opting in.
        _ => ProofSource::Relayer,
    }
}

fn relayer_url() -> String {
    std::env::var("RELAYER_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".to_string())
}

/// JSON mirror of `relayer::solana_proof::http::MmrProofResponse`.
#[derive(Deserialize)]
struct RelayerMmrProofResponse {
    mmr_proof: Option<RelayerMmrProofDto>,
    leaf_count: u64,
    #[allow(dead_code)]
    proof_slot: u64,
    verified: bool,
    status: String,
}

#[derive(Deserialize)]
struct RelayerMmrProofDto {
    leaf_index: u64,
    siblings: Vec<String>,
}

/// Fetches a verified MMR inclusion proof from the relayer proof service, retrying a bounded number
/// of times while the service's ingestion is still catching up to chain (`503 lagging`). REQUIRES
/// `verified == true`; any other non-verified response fails loudly at once. Retrying HERE (not by
/// re-invoking the client) keeps the caller idempotent — the historical `supersede` step appends
/// on-chain leaves, so re-running it would corrupt the lineage.
fn fetch_relayer_mmr_proof(
    encrypted_value: &Pubkey,
    leaf_index: u64,
) -> Result<zama_solana_acl::MmrProof, Box<dyn std::error::Error>> {
    const MAX_ATTEMPTS: u32 = 15;
    for attempt in 1..=MAX_ATTEMPTS {
        match fetch_relayer_mmr_proof_once(encrypted_value, leaf_index) {
            Ok(proof) => return Ok(proof),
            Err(e) if e.to_string().contains("lagging") && attempt < MAX_ATTEMPTS => {
                eprintln!(
                    "relayer proof lagging (attempt {attempt}/{MAX_ATTEMPTS}); retrying in 2s"
                );
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            Err(e) => return Err(e),
        }
    }
    Err("relayer proof still lagging after retries".into())
}

fn fetch_relayer_mmr_proof_once(
    encrypted_value: &Pubkey,
    leaf_index: u64,
) -> Result<zama_solana_acl::MmrProof, Box<dyn std::error::Error>> {
    let base = relayer_url();
    let url = format!(
        "{}/internal/solana/mmr-proof?encrypted_value={}&leaf_index={}",
        base.trim_end_matches('/'),
        encrypted_value,
        leaf_index
    );
    let body: RelayerMmrProofResponse = match ureq::get(&url).call() {
        Ok(resp) => resp.into_json()?,
        Err(ureq::Error::Status(code, resp)) => {
            let status = resp.into_string().unwrap_or_default();
            return Err(format!("relayer proof HTTP {code}: {status}").into());
        }
        Err(e) => return Err(format!("relayer proof request to {url} failed: {e}").into()),
    };
    if !body.verified || body.mmr_proof.is_none() {
        return Err(format!(
            "relayer proof not verified (status={}, leaf_count={})",
            body.status, body.leaf_count
        )
        .into());
    }
    let dto = body.mmr_proof.expect("checked is_some above");
    let siblings = dto
        .siblings
        .iter()
        .map(|h| {
            hexdec(h).try_into().map_err(|_| {
                Box::<dyn std::error::Error>::from(format!("relayer sibling {h} is not 32 bytes"))
            })
        })
        .collect::<Result<Vec<[u8; 32]>, _>>()?;
    Ok(zama_solana_acl::MmrProof {
        leaf_index: dto.leaf_index,
        siblings,
    })
}

fn public_decrypt_proof_step(
    host: &Program<Rc<Keypair>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let handle = bytes32_env("PUB_HANDLE")?;
    let encrypted_value = public_proof_encrypted_value_from_env()?;
    let value_account = fetch_encrypted_value(host, encrypted_value)?;
    let value_key = value_account.value_key();
    let (proof, proof_bytes) = match proof_source() {
        ProofSource::Local => {
            build_public_decrypt_proof(encrypted_value.to_bytes(), &value_account, handle)?
        }
        ProofSource::Relayer => {
            // Chain facts (peaks/leaf_count/current handle) come from the live account; the proof
            // itself comes from the relayer. The public-decrypt leaf is the lineage's last leaf.
            let leaf_index = value_account
                .leaf_count
                .checked_sub(1)
                .ok_or("public-decrypt lineage has no leaves")?;
            let proof = fetch_relayer_mmr_proof(&encrypted_value, leaf_index)?;
            // Fail-fast: the relayer proof must authorize this handle against the live lineage
            // before it is handed to the KMS / on-chain consume steps.
            let shared = value_account.to_shared();
            zama_solana_acl::authorize_public(encrypted_value.to_bytes(), &shared, handle, &proof)
                .map_err(|e| {
                    Box::<dyn std::error::Error>::from(format!(
                        "relayer public-decrypt proof did not verify against live lineage: {e:?}"
                    ))
                })?;
            let proof_bytes = proof_blob(MMR_MODE_PUBLIC, &proof)?;
            (proof, proof_bytes)
        }
    };

    println!("PUB H 0x{}", hex(&handle));
    println!("PUB encryptedValueAccount {encrypted_value}");
    println!(
        "PUB encryptedValueAccountHex 0x{}",
        hex(&encrypted_value.to_bytes())
    );
    println!("PUB aclValueKey 0x{}", hex(&value_key));
    println!("PUB peaks {}", hex_csv(&value_account.peaks));
    println!("PUB leafCount {}", value_account.leaf_count);
    println!("PUB proofSlot {}", value_account.leaf_count);
    println!("PUB leafIndex {}", proof.leaf_index);
    println!("PUB siblings {}", hex_csv(&proof.siblings));
    println!("PUB mmrProofBytes 0x{}", hex(&proof_bytes));
    // Same proof, mode-byte stripped: proof_blob prepends a 1-byte MMR_MODE tag, but the
    // on-chain consume steps (redeem_burned_amount_secp / disclose_*_secp, PROOF env) borsh-decode
    // a bare MmrInclusionProof, whose wire shape == borsh(MmrProof). So this is proof_bytes[1..].
    println!("PUB mmrInclusionProofBytes 0x{}", hex(&proof_bytes[1..]));
    Ok(())
}

fn historical_supersede_step(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
    step: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match step {
        "compute" => {
            let target = durable_eval_target(payer, HISTORICAL_LABEL_MARKER);
            if existing_lineage_state(host, target.encrypted_value)?.is_some() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "HISTORICAL_STEP=compute requires a fresh historical lineage",
                )
                .into());
            }
            let result = trivial_encrypt_eval_with_label(
                host,
                payer,
                host_config,
                HISTORICAL_LABEL_MARKER,
                "historical compute",
            )?;
            println!("HIST H_old 0x{}", hex(&result.handle));
            println!("HIST encryptedValueAccount {}", result.encrypted_value);
            println!(
                "HIST encryptedValueAccountHex 0x{}",
                hex(&result.encrypted_value.to_bytes())
            );
            println!("HIST aclValueKey 0x{}", hex(&result.value_key));
            Ok(())
        }
        "supersede" => {
            let target = durable_eval_target(payer, HISTORICAL_LABEL_MARKER);
            let Some((old_handle, _)) = existing_lineage_state(host, target.encrypted_value)?
            else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "HISTORICAL_STEP=supersede requires HISTORICAL_STEP=compute first",
                )
                .into());
            };

            let result = trivial_encrypt_eval_with_label(
                host,
                payer,
                host_config,
                HISTORICAL_LABEL_MARKER,
                "historical supersede",
            )?;
            let value_account = fetch_encrypted_value(host, target.encrypted_value)?;
            let subject = payer.pubkey().to_bytes();
            let (proof, proof_bytes) = match proof_source() {
                ProofSource::Local => build_historical_access_proof(
                    target.encrypted_value.to_bytes(),
                    &value_account.peaks,
                    value_account.leaf_count,
                    old_handle,
                    subject,
                )?,
                ProofSource::Relayer => {
                    // The e2e historical lineage has exactly one historical leaf, at index 0.
                    let proof = fetch_relayer_mmr_proof(&target.encrypted_value, 0)?;
                    // Fail-fast: the relayer proof must verify against the live peaks for the
                    // historical-access leaf (old handle bound to this subject).
                    let leaf = zama_solana_acl::historical_access_leaf_commitment(
                        target.encrypted_value.to_bytes(),
                        0,
                        old_handle,
                        subject,
                    );
                    if !zama_solana_acl::mmr_verify(
                        &value_account.peaks,
                        value_account.leaf_count,
                        leaf,
                        &proof,
                    ) {
                        return Err(format!(
                            "relayer historical proof did not verify against live leaf_count={}",
                            value_account.leaf_count
                        )
                        .into());
                    }
                    let proof_bytes = proof_blob(MMR_MODE_HISTORICAL, &proof)?;
                    (proof, proof_bytes)
                }
            };

            println!("HIST H_old 0x{}", hex(&old_handle));
            println!("HIST H_new 0x{}", hex(&result.handle));
            println!("HIST encryptedValueAccount {}", target.encrypted_value);
            println!(
                "HIST encryptedValueAccountHex 0x{}",
                hex(&target.encrypted_value.to_bytes())
            );
            println!("HIST aclValueKey 0x{}", hex(&target.value_key));
            println!("HIST peaks {}", hex_csv(&value_account.peaks));
            println!("HIST leafCount {}", value_account.leaf_count);
            println!("HIST proofSlot {}", value_account.leaf_count);
            println!("HIST leafIndex {}", proof.leaf_index);
            println!("HIST siblings {}", hex_csv(&proof.siblings));
            println!("HIST subject 0x{}", hex(&payer.pubkey().to_bytes()));
            println!("HIST mmrProofBytes 0x{}", hex(&proof_bytes));
            Ok(())
        }
        other => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("unknown HISTORICAL_STEP={other}; expected compute or supersede"),
        )
        .into()),
    }
}

/// Input flow leg (#1539): one fhe_eval that adds a public scalar to a coprocessor-attested external
/// input and binds the result to a durable output ACL record. The verified input is resolved in-frame
/// (FheEvalOperand::VerifiedInput) — its attestation is re-verified on-chain via secp256k1 with no
/// scratch PDA — and the durable output is pinned to the attested acl_domain_key (the input's domain;
/// the on-chain binding rejects any other). The attestation is supplied via the same BIND_* env vars
/// the standalone input-verify leg uses; TE_ADD is the scalar addend (default 2). The host-listener
/// ingests the emitted FheBinaryOpEvent so the tfhe-worker materializes (input + TE_ADD); TE_ALLOW
/// then makes the result publicly decryptable.
fn fhe_eval_verified_input_add(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    use anchor_lang::solana_program::instruction::AccountMeta;

    // Attestation from the relayer input-proof (same inputs as BIND_INPUT).
    let input_handle: [u8; 32] = hexdec(&std::env::var("BIND_HANDLE")?)
        .try_into()
        .expect("BIND_HANDLE must be 32 bytes");
    let signature: [u8; 65] = hexdec(&std::env::var("BIND_COPRO_SIG")?)
        .try_into()
        .expect("BIND_COPRO_SIG must be 65 bytes");
    let user_address: [u8; 32] = hexdec(&std::env::var("BIND_USER")?)
        .try_into()
        .expect("BIND_USER must be 32 bytes");
    let contract_address: [u8; 32] = hexdec(&std::env::var("BIND_CONTRACT")?)
        .try_into()
        .expect("BIND_CONTRACT must be 32 bytes");
    let contract_chain_id: u64 = std::env::var("BIND_CHAIN_ID")?.parse()?;
    let extra_data = std::env::var("BIND_EXTRA")
        .map(|s| hexdec(&s))
        .unwrap_or_else(|_| vec![0u8]);

    // The attested contract IS the input's acl_domain_key; the durable output must bind that exact
    // domain (the on-chain VerifiedInput -> durable binding enforces it). app_account is the signer.
    let acl_domain_key = Pubkey::new_from_array(contract_address);
    let app_account = payer.pubkey();

    // Scalar addend (default 2). ClearConst reads big-endian, so place the u64 in the low bytes.
    let addend: u64 = std::env::var("TE_ADD")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2);
    let mut scalar = [0u8; 32];
    scalar[24..32].copy_from_slice(&addend.to_be_bytes());
    let fhe_type: u8 = 5; // euint64

    // Label distinct from the trivial-encrypt legs ([1]/[2] markers) so output PDAs never collide;
    // keyed on the input handle tail so distinct inputs derive distinct output records.
    let mut encrypted_value_label = [3u8; 32];
    encrypted_value_label[24..32].copy_from_slice(&input_handle[24..32]);
    let output_encrypted_value =
        encrypted_value_address(acl_domain_key, app_account, encrypted_value_label);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![owner_subject(payer.pubkey())];

    // Non-zero context-id domain separator for this eval's transient handles.
    let mut context_id = [0u8; 32];
    context_id[0] = 0xe0;
    context_id[24..32].copy_from_slice(&input_handle[24..32]);

    let attestation = zama_host::CoprocessorInputAttestation {
        input_handle,
        ct_handles: vec![input_handle],
        handle_index: 0,
        user_address,
        contract_address,
        contract_chain_id,
        extra_data,
        signatures: vec![signature],
    };

    let args = zama_host::FheEvalArgs {
        context_id,
        steps: vec![zama_host::FheEvalStep::Binary {
            op: zama_host::FheBinaryOpCode::Add,
            lhs: zama_host::FheEvalOperand::VerifiedInput {
                attestation: Box::new(attestation),
            },
            rhs: zama_host::FheEvalOperand::Scalar(scalar),
            output_fhe_type: fhe_type,
            output: durable_output(
                host,
                output_encrypted_value,
                0,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                subjects,
            )?,
        }],
    };

    let input_hex: String = input_handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("fhe_eval verified input 0x{input_hex} + {addend} (euint64)");
    let sig = host
        .request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            // Block-cap optional accounts: default cap is unrestricted, so existing flows
            // pass None/None and behave exactly as before the feature. The mandatory HCU
            // authority is the payer itself for this wallet-driven PoC leg.
            hcu_authority: payer.pubkey(),
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(vec![AccountMeta::new(output_encrypted_value, false)])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval verified-input add: {sig}");

    let value_account = fetch_encrypted_value(host, output_encrypted_value)?;
    let handle_hex: String = value_account
        .current_handle
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    println!("  output ACL record {output_encrypted_value}");
    println!("  acl value key 0x{}", hex(&value_account.value_key()));
    println!("  result handle 0x{handle_hex}  (tfhe-worker materializes input + {addend})");

    if std::env::var("TE_ALLOW").is_ok() {
        allow_for_decryption(host, payer, host_config, output_encrypted_value)?;
    }
    Ok(())
}

/// Consume step 1: create the disclosure request witness for a token amount lineage
/// (request_disclose_amount → CPIs zama-host allow_subjects + make_handle_public, pins the host's
/// current KMS context id + expires_slot + request_hash into a DisclosureRequest PDA the
/// disclose_amount_secp consume step then binds to). Inputs via env: MINT, TS_ACL, TS_HANDLE;
/// optional REQUEST_NONCE (32-byte hex), REQUEST_TTL_SLOTS.
fn consume_seal(
    _host: &Program<Rc<Keypair>>,
    token: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let mint = Pubkey::from_str(&std::env::var("MINT")?)?;
    let ts_acl = Pubkey::from_str(&std::env::var("TS_ACL")?)?;
    let handle: [u8; 32] = hexdec(&std::env::var("TS_HANDLE")?)
        .try_into()
        .expect("TS_HANDLE");

    let nonce = request_nonce_from_env();
    let expires_slot = request_expires_slot(token)?;
    let (disclosure_request, _) =
        confidential_token::disclosure_request_address(mint, payer.pubkey(), handle, nonce);

    let (token_evt, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &confidential_token::ID);
    let sig2 = token
        .request()
        .accounts(confidential_token::accounts::RequestDiscloseAmount {
            requester: payer.pubkey(),
            mint,
            amount_value: ts_acl,
            disclosure_request,
            deny_subject_record: None,
            zama_program: zama_host::ID,
            host_config,
            system_program: system_program::ID,
            event_authority: token_evt,
            program: confidential_token::ID,
        })
        .args(confidential_token::instruction::RequestDiscloseAmount {
            amount_handle: handle,
            request_nonce: nonce,
            expires_slot,
        })
        .send()?;
    println!("OK request_disclose_amount: {sig2}  (handle released for public decrypt)");
    println!("  disclosure request witness {disclosure_request}  (kms_context pinned, expires_slot {expires_slot})");
    Ok(())
}

/// Returns the 32-byte request nonce from REQUEST_NONCE (hex) or a default fixed nonce so the
/// witness PDA is deterministic within one e2e run.
fn request_nonce_from_env() -> [u8; 32] {
    match std::env::var("REQUEST_NONCE") {
        Ok(s) => hexdec(&s)
            .try_into()
            .expect("REQUEST_NONCE must be 32 bytes"),
        Err(_) => [7u8; 32],
    }
}

/// Computes the witness expiry slot: current slot + REQUEST_TTL_SLOTS (default 5000), giving the
/// public-decrypt + consume steps ample time before the witness expires.
fn request_expires_slot(program: &Program<Rc<Keypair>>) -> Result<u64, Box<dyn std::error::Error>> {
    let ttl: u64 = std::env::var("REQUEST_TTL_SLOTS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5000);
    let slot = program.rpc().get_slot()?;
    Ok(slot + ttl)
}

/// Consume step 2: publish a KMS-certified cleartext by verifying the KMS PublicDecryptVerification
/// EIP-712 cert on-chain via secp256k1 (disclose_amount_secp) against the active KMS context's
/// ProtocolConfig-mirrored signer set. Inputs via env: MINT, TS_ACL, TS_HANDLE, CLEARTEXT, KMS_SIG,
/// EXTRA, KMS_CTX_ID.
fn consume_disclose(
    token: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let mint = Pubkey::from_str(&std::env::var("MINT")?)?;
    let ts_acl = Pubkey::from_str(&std::env::var("TS_ACL")?)?;
    let handle: [u8; 32] = hexdec(&std::env::var("TS_HANDLE")?)
        .try_into()
        .expect("TS_HANDLE");
    let cleartext: u64 = std::env::var("CLEARTEXT")?.parse()?;
    let kms_sig: [u8; 65] = hexdec(&std::env::var("KMS_SIG")?)
        .try_into()
        .expect("KMS_SIG 65 bytes");
    let extra = hexdec(&std::env::var("EXTRA")?);
    // Borsh-serialized MmrInclusionProof for the witness-pinned handle's
    // public-decrypt leaf, produced by the relayer proof service (same shape as
    // the redeem path). Empty env yields an empty proof, which fails on-chain
    // authorization by design.
    let proof = confidential_token::MmrInclusionProof::try_from_slice(&hexdec(
        &std::env::var("PROOF").unwrap_or_default(),
    ))
    .unwrap_or_default();
    let ctx_id: u64 = std::env::var("KMS_CTX_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let nonce = request_nonce_from_env();
    let (disclosure_request, _) =
        confidential_token::disclosure_request_address(mint, payer.pubkey(), handle, nonce);
    let (kms_context, _) = Pubkey::find_program_address(
        &[zama_host::KMS_CONTEXT_SEED, &ctx_id.to_le_bytes()],
        &zama_host::ID,
    );
    let (token_evt, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &confidential_token::ID);

    let sig = token
        .request()
        .accounts(confidential_token::accounts::DiscloseAmountSecp {
            mint,
            amount_value: ts_acl,
            disclosure_request,
            host_config,
            kms_context,
            event_authority: token_evt,
            program: confidential_token::ID,
        })
        .args(confidential_token::instruction::DiscloseAmountSecp {
            amount_handle: handle,
            cleartext_amount: cleartext,
            signatures: vec![kms_sig],
            extra_data: extra,
            proof,
        })
        .send()?;
    println!("OK disclose_amount_secp: {sig}");
    println!(
        "  KMS PublicDecryptVerification cert verified on-chain (secp256k1); cleartext {cleartext}"
    );
    Ok(())
}

/// Stands up a confidential token account and mints a token-scoped random transfer amount — a
/// real amount-ACL handle (BALANCE_FHE_TYPE, acl_domain_key = mint, token-amount label), which
/// is what the disclose path requires (the total-supply handle uses a different ACL domain).
/// MINT via env; owner is the payer.
fn consume_amount(
    token: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let mint = Pubkey::from_str(&std::env::var("MINT")?)?;
    let owner = payer.pubkey();
    let (compute_signer, _) = confidential_token::compute_signer_address(mint);
    let (token_account, _) = confidential_token::token_account_address(mint, owner);
    let (zama_evt, _) = Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let (token_evt, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &confidential_token::ID);

    // 1. initialize_token_account (initial balance 0) creates the account + stable balance lineage.
    let (balance_value, _) =
        confidential_token::balance_encrypted_value_address(mint, token_account);
    if token.rpc().get_account(&token_account).is_err() {
        let sig = token
            .request()
            .accounts(confidential_token::accounts::InitializeTokenAccount {
                owner,
                mint,
                compute_signer,
                token_account,
                balance_encrypted_value: balance_value,
                zama_event_authority: zama_evt,
                zama_program: zama_host::ID,
                host_config,
                system_program: system_program::ID,
                hcu_authority: confidential_token::hcu_authority_address(mint).0,
                hcu_block_meter: None,
                hcu_trusted_app_record: None,
                event_authority: token_evt,
                program: confidential_token::ID,
            })
            .args(confidential_token::instruction::InitializeTokenAccount { initial_balance: 0 })
            .send()?;
        println!("OK initialize_token_account: {sig}");
    } else {
        println!("token account {token_account} already initialized — skipping");
    }

    // 2. create_random_amount (Transfer kind) — a RandU64 token amount; uses SlotHashes entropy
    // for the result handle, so skip preflight (populated only in real execution).
    let label = confidential_token::transfer_amount_label();
    let (amount_value, _) = confidential_token::encrypted_value_address(mint, owner, label);
    let sig2 = token
        .request()
        .accounts(confidential_token::accounts::CreateRandomAmount {
            owner,
            mint,
            token_account,
            compute_signer,
            amount_value,
            zama_event_authority: zama_evt,
            zama_program: zama_host::ID,
            host_config,
            system_program: system_program::ID,
            hcu_authority: confidential_token::hcu_authority_address(mint).0,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: token_evt,
            program: confidential_token::ID,
        })
        .args(confidential_token::instruction::CreateRandomAmount {
            amount_kind: confidential_token::ConfidentialAmountKind::Transfer,
        })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK create_random_amount: {sig2}");

    let value = fetch_encrypted_value(token, amount_value)?;
    let hh: String = value
        .current_handle
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    println!("  amount ACL    {amount_value}");
    println!("  amount handle 0x{hh}  (token-scoped transfer amount; tfhe-worker materializes it)");
    Ok(())
}

/// Deposits public USDC into a confidential balance via wrap_usdc: escrows USDC into the mint's
/// vault and FHE-adds the wrapped amount to the balance + total supply. Produces the balance a
/// confidential_burn (redeem path) draws from. MINT, UNDERLYING_MINT, WRAP_AMOUNT via env.
fn consume_wrap(
    token: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
    let mint = Pubkey::from_str(&std::env::var("MINT")?)?;
    let underlying = Pubkey::from_str(&std::env::var("UNDERLYING_MINT")?)?;
    let amount: u64 = std::env::var("WRAP_AMOUNT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);
    let owner = payer.pubkey();
    let spl_token_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?;
    let ata_prog = Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")?;

    let (vault_authority, _) = confidential_token::vault_authority_address(mint);
    let vault_usdc = confidential_token::vault_token_account_address(mint, underlying);
    let (compute_signer, _) = confidential_token::compute_signer_address(mint);
    let (token_account, _) = confidential_token::token_account_address(mint, owner);
    let (ts_authority, _) = confidential_token::total_supply_authority_address(mint);
    let (user_usdc, _) = Pubkey::find_program_address(
        &[owner.as_ref(), spl_token_id.as_ref(), underlying.as_ref()],
        &ata_prog,
    );

    let (balance_value, _) =
        confidential_token::balance_encrypted_value_address(mint, token_account);
    let mint_state: confidential_token::ConfidentialMint = token.account(mint)?;
    let total_supply_value = mint_state.total_supply_encrypted_value;
    // The wrap amount is trivial-encrypted as a transient inside wrap_usdc's eval frame.
    let (zama_evt, _) = Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let (token_evt, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &confidential_token::ID);

    // The confidential token account must exist before wrap draws from it; create it if missing.
    if token.rpc().get_account(&token_account).is_err() {
        let init_sig = token
            .request()
            .accounts(confidential_token::accounts::InitializeTokenAccount {
                owner,
                mint,
                compute_signer,
                token_account,
                balance_encrypted_value: balance_value,
                zama_event_authority: zama_evt,
                zama_program: zama_host::ID,
                host_config,
                system_program: system_program::ID,
                hcu_authority: confidential_token::hcu_authority_address(mint).0,
                hcu_block_meter: None,
                hcu_trusted_app_record: None,
                event_authority: token_evt,
                program: confidential_token::ID,
            })
            .args(confidential_token::instruction::InitializeTokenAccount { initial_balance: 0 })
            .send()?;
        println!("OK initialize_token_account: {init_sig}");
    }

    // The vault's USDC ATA must exist before wrap; create it (idempotent) if missing.
    if token.rpc().get_account(&vault_usdc).is_err() {
        let ata_ix = Instruction {
            program_id: ata_prog,
            accounts: vec![
                AccountMeta::new(owner, true),
                AccountMeta::new(vault_usdc, false),
                AccountMeta::new_readonly(vault_authority, false),
                AccountMeta::new_readonly(underlying, false),
                AccountMeta::new_readonly(system_program::ID, false),
                AccountMeta::new_readonly(spl_token_id, false),
            ],
            data: vec![1], // CreateIdempotent
        };
        token.request().instruction(ata_ix).send()?;
        println!("created vault USDC ATA {vault_usdc}");
    }

    // wrap_usdc runs several FHE ops in one instruction and exhausts the default 32KB SBF
    // heap; request the max heap frame (256KB) and raise the compute-unit limit.
    let cb_prog = Pubkey::from_str("ComputeBudget111111111111111111111111111111")?;
    let mut heap_data = vec![1u8];
    heap_data.extend_from_slice(&(256u32 * 1024).to_le_bytes());
    let heap_ix = Instruction {
        program_id: cb_prog,
        accounts: vec![],
        data: heap_data,
    };
    let mut cu_data = vec![2u8];
    cu_data.extend_from_slice(&1_400_000u32.to_le_bytes());
    let cu_ix = Instruction {
        program_id: cb_prog,
        accounts: vec![],
        data: cu_data,
    };

    let sig = token
        .request()
        .instruction(heap_ix)
        .instruction(cu_ix)
        .accounts(confidential_token::accounts::WrapUsdc {
            owner,
            mint,
            token_account,
            underlying_mint: underlying,
            user_usdc,
            vault_usdc,
            vault_authority,
            compute_signer,
            total_supply_authority: ts_authority,
            balance_value,
            total_supply_value,
            zama_event_authority: zama_evt,
            zama_program: zama_host::ID,
            host_config,
            token_program: spl_token_id,
            system_program: system_program::ID,
            hcu_authority: confidential_token::hcu_authority_address(mint).0,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: token_evt,
            program: confidential_token::ID,
        })
        .args(confidential_token::instruction::WrapUsdc { amount })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: std::env::var("WRAP_NO_PREFLIGHT").is_err(),
            ..Default::default()
        })?;
    println!("OK wrap_usdc({amount}): {sig}");
    println!("  new balance ACL {balance_value}");
    Ok(())
}

/// Burns an encrypted amount from the confidential balance (confidential_burn): the heaviest
/// FHE instruction — ge + sub + select + sub + sub across five zama-host CPIs. Reads the current
/// balance/total-supply ACLs and next nonce sequences live from chain, mints an owner-scoped
/// random burn amount (create_random_amount Burn kind), then burns it. Produces the burned-amount
/// handle (and ACL) the redeem path publicly decrypts and releases against the vault. MINT via env.
fn consume_burn(
    token: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    use anchor_lang::solana_program::instruction::Instruction;
    let mint = Pubkey::from_str(&std::env::var("MINT")?)?;
    let owner = payer.pubkey();
    let (compute_signer, _) = confidential_token::compute_signer_address(mint);
    let (token_account, _) = confidential_token::token_account_address(mint, owner);
    let (ts_authority, _) = confidential_token::total_supply_authority_address(mint);
    let (zama_evt, _) = Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let (token_evt, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &confidential_token::ID);

    let token_state: confidential_token::ConfidentialTokenAccount = token.account(token_account)?;
    let mint_state: confidential_token::ConfidentialMint = token.account(mint)?;
    let balance_value = token_state.balance_encrypted_value;
    let total_supply_value = mint_state.total_supply_encrypted_value;

    // 1. fromExternal burn amount: a coprocessor-attested external input (BIND_* env from the
    // relayer input-proof), bound to (user = owner, contract = compute_signer PDA). confidential_burn
    // re-verifies the EIP-712 attestation in-frame and transient-allows it for the burn eval — no
    // durable amount ACL, no create_random_amount. The attested handle must be a euint64.
    let input_handle: [u8; 32] = hexdec(&std::env::var("BIND_HANDLE")?)
        .try_into()
        .expect("BIND_HANDLE must be 32 bytes");
    let signature: [u8; 65] = hexdec(&std::env::var("BIND_COPRO_SIG")?)
        .try_into()
        .expect("BIND_COPRO_SIG must be 65 bytes");
    let user_address: [u8; 32] = hexdec(&std::env::var("BIND_USER")?)
        .try_into()
        .expect("BIND_USER must be 32 bytes");
    let contract_address: [u8; 32] = hexdec(&std::env::var("BIND_CONTRACT")?)
        .try_into()
        .expect("BIND_CONTRACT must be 32 bytes");
    let contract_chain_id: u64 = std::env::var("BIND_CHAIN_ID")?.parse()?;
    let extra_data = std::env::var("BIND_EXTRA")
        .map(|s| hexdec(&s))
        .unwrap_or_else(|_| vec![0u8]);
    let amount_attestation = zama_host::CoprocessorInputAttestation {
        input_handle,
        ct_handles: vec![input_handle],
        handle_index: 0,
        user_address,
        contract_address,
        contract_chain_id,
        extra_data,
        signatures: vec![signature],
    };
    let hh: String = input_handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("  burn amount (attested external input) handle 0x{hh}");

    // 2. Durable burn outputs supersede the stable balance/total-supply lineages in place and
    // create or supersede the stable burned-amount lineage for this token account.
    let (burned_acl, _) = confidential_token::encrypted_value_address(
        mint,
        token_account,
        confidential_token::burned_amount_label(),
    );

    // 3. confidential_burn — five FHE ops in one instruction; request the max heap frame + a
    // raised CU limit like wrap. SlotHashes entropy is only populated in real execution, so skip
    // preflight unless BURN_NO_PREFLIGHT forces an on-chain log capture.
    let cb_prog = Pubkey::from_str("ComputeBudget111111111111111111111111111111")?;
    let mut heap_data = vec![1u8];
    heap_data.extend_from_slice(&(256u32 * 1024).to_le_bytes());
    let heap_ix = Instruction {
        program_id: cb_prog,
        accounts: vec![],
        data: heap_data,
    };
    let mut cu_data = vec![2u8];
    cu_data.extend_from_slice(&1_400_000u32.to_le_bytes());
    let cu_ix = Instruction {
        program_id: cb_prog,
        accounts: vec![],
        data: cu_data,
    };

    let sig = token
        .request()
        .instruction(heap_ix)
        .instruction(cu_ix)
        .accounts(confidential_token::accounts::ConfidentialBurn {
            owner,
            mint,
            token_account,
            compute_signer,
            total_supply_authority: ts_authority,
            balance_value,
            total_supply_value,
            burned_amount_value: burned_acl,
            zama_event_authority: zama_evt,
            zama_program: zama_host::ID,
            host_config,
            system_program: system_program::ID,
            hcu_authority: confidential_token::hcu_authority_address(mint).0,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: token_evt,
            program: confidential_token::ID,
        })
        .args(confidential_token::instruction::ConfidentialBurn { amount_attestation })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: std::env::var("BURN_NO_PREFLIGHT").is_err(),
            ..Default::default()
        })?;
    println!("OK confidential_burn: {sig}");
    println!("  burned amount ACL {burned_acl}");
    let burned_value = fetch_encrypted_value(token, burned_acl)?;
    let bh: String = burned_value
        .current_handle
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    println!("  burned handle 0x{bh}  (redeem path publicly decrypts this)");
    Ok(())
}

/// Redeem step 1: create the burn-redemption request witness, which pins the host's current KMS
/// context id + expires_slot + request_hash into a BurnRedemptionRequest PDA the
/// redeem_burned_amount_secp consume step binds to, and releases the burned handle for public
/// decrypt through the token program's host CPIs. Inputs via env: MINT, UNDERLYING_MINT,
/// BURNED_ACL, BURNED_HANDLE; optional REQUEST_NONCE, REQUEST_TTL_SLOTS.
fn consume_request_redeem(
    _host: &Program<Rc<Keypair>>,
    token: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let mint = Pubkey::from_str(&std::env::var("MINT")?)?;
    let underlying = Pubkey::from_str(&std::env::var("UNDERLYING_MINT")?)?;
    let burned_acl = Pubkey::from_str(&std::env::var("BURNED_ACL")?)?;
    let burned_handle: [u8; 32] = hexdec(&std::env::var("BURNED_HANDLE")?)
        .try_into()
        .expect("BURNED_HANDLE");
    let owner = payer.pubkey();
    let spl_token_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?;
    let ata_prog = Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")?;

    let (token_account, _) = confidential_token::token_account_address(mint, owner);
    let (destination_usdc, _) = Pubkey::find_program_address(
        &[owner.as_ref(), spl_token_id.as_ref(), underlying.as_ref()],
        &ata_prog,
    );
    let nonce = request_nonce_from_env();
    let expires_slot = request_expires_slot(token)?;
    let (redemption_request, _) =
        confidential_token::burn_redemption_request_address(mint, owner, burned_handle, nonce);
    let (token_evt, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &confidential_token::ID);

    let sig = token
        .request()
        .accounts(confidential_token::accounts::RequestBurnRedemption {
            owner,
            mint,
            token_account,
            underlying_mint: underlying,
            destination_usdc,
            burned_amount_value: burned_acl,
            redemption_request,
            deny_subject_record: None,
            zama_program: zama_host::ID,
            host_config,
            system_program: system_program::ID,
            event_authority: token_evt,
            program: confidential_token::ID,
        })
        .args(confidential_token::instruction::RequestBurnRedemption {
            burned_handle,
            request_nonce: nonce,
            expires_slot,
        })
        .send()?;
    println!("OK request_burn_redemption: {sig}");
    println!("  burn-redemption request witness {redemption_request}  (kms_context pinned, expires_slot {expires_slot})");
    Ok(())
}

/// Redeem step 2: redeem the KMS-certified burned amount from the SPL vault
/// (redeem_burned_amount_secp): binds the BurnRedemptionRequest witness, verifies the KMS
/// PublicDecryptVerification EIP-712 cert on-chain via secp256k1 against the witness-pinned KMS
/// context, and releases the cleartext amount of underlying USDC to the owner. Inputs via env:
/// MINT, UNDERLYING_MINT, BURNED_ACL, BURNED_HANDLE, CLEARTEXT, KMS_SIG, EXTRA, PROOF (borsh
/// MmrInclusionProof for the burned handle's public-decrypt leaf), optional KMS_CTX_ID
/// (default 1), REQUEST_NONCE.
fn consume_redeem(
    _host: &Program<Rc<Keypair>>,
    token: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let mint = Pubkey::from_str(&std::env::var("MINT")?)?;
    let underlying = Pubkey::from_str(&std::env::var("UNDERLYING_MINT")?)?;
    let burned_acl = Pubkey::from_str(&std::env::var("BURNED_ACL")?)?;
    let burned_handle: [u8; 32] = hexdec(&std::env::var("BURNED_HANDLE")?)
        .try_into()
        .expect("BURNED_HANDLE");
    let cleartext: u64 = std::env::var("CLEARTEXT")?.parse()?;
    let kms_sig: [u8; 65] = hexdec(&std::env::var("KMS_SIG")?)
        .try_into()
        .expect("KMS_SIG 65 bytes");
    let extra = hexdec(&std::env::var("EXTRA")?);
    // Borsh-serialized MmrInclusionProof (leaf_index + siblings) for the burned
    // handle's public-decrypt leaf, produced by the relayer proof service. Empty
    // env yields an empty proof, which fails on-chain authorization by design.
    let proof = confidential_token::MmrInclusionProof::try_from_slice(&hexdec(
        &std::env::var("PROOF").unwrap_or_default(),
    ))
    .unwrap_or_default();
    let ctx_id: u64 = std::env::var("KMS_CTX_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let nonce = request_nonce_from_env();
    let owner = payer.pubkey();
    let spl_token_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?;
    let ata_prog = Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")?;

    let (token_account, _) = confidential_token::token_account_address(mint, owner);
    let (vault_authority, _) = confidential_token::vault_authority_address(mint);
    let vault_usdc = confidential_token::vault_token_account_address(mint, underlying);
    let (destination_usdc, _) = Pubkey::find_program_address(
        &[owner.as_ref(), spl_token_id.as_ref(), underlying.as_ref()],
        &ata_prog,
    );
    let (redemption_request, _) =
        confidential_token::burn_redemption_request_address(mint, owner, burned_handle, nonce);
    let (redemption_record, _) = Pubkey::find_program_address(
        &[b"burn-redemption", mint.as_ref(), burned_handle.as_ref()],
        &confidential_token::ID,
    );
    let (kms_context, _) = Pubkey::find_program_address(
        &[zama_host::KMS_CONTEXT_SEED, &ctx_id.to_le_bytes()],
        &zama_host::ID,
    );
    let (token_evt, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &confidential_token::ID);

    let sig = token
        .request()
        .accounts(confidential_token::accounts::RedeemBurnedAmountSecp {
            owner,
            mint,
            token_account,
            underlying_mint: underlying,
            vault_usdc,
            destination_usdc,
            vault_authority,
            burned_amount_value: burned_acl,
            redemption_request,
            redemption_record,
            host_config,
            kms_context,
            token_program: spl_token_id,
            system_program: system_program::ID,
            event_authority: token_evt,
            program: confidential_token::ID,
        })
        .args(confidential_token::instruction::RedeemBurnedAmountSecp {
            burned_handle,
            cleartext_amount: cleartext,
            signatures: vec![kms_sig],
            extra_data: extra,
            proof,
        })
        .send()?;
    println!("OK redeem_burned_amount_secp: {sig}");
    println!("  KMS PublicDecryptVerification cert verified on-chain (secp256k1); released {cleartext} USDC base units to {destination_usdc}");
    Ok(())
}

/// Idempotent: initialize_host_config only if the PDA doesn't already exist.
fn ensure_host_config(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    if host.rpc().get_account(&host_config).is_ok() {
        println!("host_config {host_config} already initialized — skipping");
        return Ok(());
    }
    let sig = host
        .request()
        .accounts(zama_host::accounts::InitializeHostConfig {
            payer: payer.pubkey(),
            admin: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
        })
        .args(zama_host::instruction::InitializeHostConfig {
            args: zama_host::InitializeHostConfigArgs {
                chain_id: zama_host::SOLANA_POC_CHAIN_ID,
                input_verifier_authority: payer.pubkey(),
                gateway_chain_id: 0,
                input_verification_contract: [0u8; 20],
                coprocessor_signer: [0u8; 20],
                decryption_contract: [0u8; 20],
                material_authority: payer.pubkey(),
                test_authority: payer.pubkey(),
                mock_input_enabled: true,
                test_shims_enabled: true,
                grant_deny_list_enabled: false,
            },
        })
        .send()?;
    println!("OK initialize_host_config: {sig}");
    Ok(())
}

fn parse_binary_op(s: &str) -> Result<zama_host::FheBinaryOpCode, String> {
    match s {
        "Add" => Ok(zama_host::FheBinaryOpCode::Add),
        "Sub" => Ok(zama_host::FheBinaryOpCode::Sub),
        "Mul" => Ok(zama_host::FheBinaryOpCode::Mul),
        "Div" => Ok(zama_host::FheBinaryOpCode::Div),
        "Rem" => Ok(zama_host::FheBinaryOpCode::Rem),
        "And" => Ok(zama_host::FheBinaryOpCode::And),
        "Or" => Ok(zama_host::FheBinaryOpCode::Or),
        "Xor" => Ok(zama_host::FheBinaryOpCode::Xor),
        "Shl" => Ok(zama_host::FheBinaryOpCode::Shl),
        "Shr" => Ok(zama_host::FheBinaryOpCode::Shr),
        "Rotl" => Ok(zama_host::FheBinaryOpCode::Rotl),
        "Rotr" => Ok(zama_host::FheBinaryOpCode::Rotr),
        "Eq" => Ok(zama_host::FheBinaryOpCode::Eq),
        "Ne" => Ok(zama_host::FheBinaryOpCode::Ne),
        "Ge" => Ok(zama_host::FheBinaryOpCode::Ge),
        "Gt" => Ok(zama_host::FheBinaryOpCode::Gt),
        "Le" => Ok(zama_host::FheBinaryOpCode::Le),
        "Lt" => Ok(zama_host::FheBinaryOpCode::Lt),
        "Min" => Ok(zama_host::FheBinaryOpCode::Min),
        "Max" => Ok(zama_host::FheBinaryOpCode::Max),
        _ => Err(format!("unknown binary op: {s}")),
    }
}

fn parse_unary_op(s: &str) -> Result<zama_host::FheUnaryOpCode, String> {
    match s {
        "Neg" => Ok(zama_host::FheUnaryOpCode::Neg),
        "Not" => Ok(zama_host::FheUnaryOpCode::Not),
        "Cast" => Ok(zama_host::FheUnaryOpCode::Cast),
        _ => Err(format!("unknown unary op: {s}")),
    }
}

fn is_comparison_op(op: zama_host::FheBinaryOpCode) -> bool {
    matches!(
        op,
        zama_host::FheBinaryOpCode::Eq
            | zama_host::FheBinaryOpCode::Ne
            | zama_host::FheBinaryOpCode::Ge
            | zama_host::FheBinaryOpCode::Gt
            | zama_host::FheBinaryOpCode::Le
            | zama_host::FheBinaryOpCode::Lt
    )
}

/// Creates a durable operand (TrivialEncrypt → AllowedDurable, `user` subject).
/// Returns (encrypted_value, current_handle).
fn create_durable_public_decrypt_operand(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
    value: u64,
    fhe_type: u8,
    label: [u8; 32],
) -> Result<(Pubkey, [u8; 32]), Box<dyn std::error::Error>> {
    use anchor_lang::solana_program::instruction::AccountMeta;
    let mut plaintext = [0u8; 32];
    plaintext[24..32].copy_from_slice(&value.to_be_bytes());
    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let encrypted_value = encrypted_value_address(acl_domain_key, app_account, label);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];
    let mut context_id = [0u8; 32];
    context_id[0] = 0x0a;
    context_id[1..9].copy_from_slice(&label[0..8]);
    let args = zama_host::FheEvalArgs {
        context_id,
        steps: vec![zama_host::FheEvalStep::TrivialEncrypt {
            plaintext,
            fhe_type,
            output: durable_output(
                host,
                encrypted_value,
                0,
                acl_domain_key,
                app_account,
                label,
                subjects,
            )?,
        }],
    };
    host.request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            hcu_authority: payer.pubkey(),
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(vec![AccountMeta::new(encrypted_value, false)])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    let value = fetch_encrypted_value(host, encrypted_value)?;
    Ok((encrypted_value, value.current_handle))
}

/// Generic binary fhe_eval: encrypted operands become durable public-decrypt handles (so
/// public-decrypt propagates to the output); scalar-RHS ops create only the LHS.
/// Env: BINARY_OP, BINARY_A, BINARY_B, BINARY_B_SCALAR (flag), BINARY_FHE_TYPE, BINARY_ALLOW.
/// Comparison ops (Eq/Ne/Ge/Gt/Le/Lt) automatically set output_fhe_type=0 (ebool).
fn fhe_eval_binary(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    use anchor_lang::solana_program::instruction::AccountMeta;

    let op_name = std::env::var("BINARY_OP").unwrap_or_else(|_| "Add".into());
    let op = parse_binary_op(&op_name).map_err(|e| e)?;
    let a: u64 = std::env::var("BINARY_A")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    let b: u64 = std::env::var("BINARY_B")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);
    let b_scalar = std::env::var("BINARY_B_SCALAR").is_ok();
    let in_fhe_type: u8 = std::env::var("BINARY_FHE_TYPE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);
    let output_fhe_type: u8 = if is_comparison_op(op) { 0 } else { in_fhe_type };

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let mut encrypted_value_label = [7u8; 32];
    encrypted_value_label[1] = op.as_u8();
    encrypted_value_label[2] = in_fhe_type;
    encrypted_value_label[3] = if b_scalar { 1 } else { 0 };
    encrypted_value_label[8..16].copy_from_slice(&a.to_be_bytes());
    encrypted_value_label[16..24].copy_from_slice(&b.to_be_bytes());
    let output_encrypted_value =
        encrypted_value_address(acl_domain_key, app_account, encrypted_value_label);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];

    let mut context_id = [0u8; 32];
    context_id[0] = 0xb0;
    context_id[1] = op.as_u8();
    context_id[2] = in_fhe_type;
    context_id[3] = if b_scalar { 1 } else { 0 };
    context_id[8..16].copy_from_slice(&a.to_be_bytes());
    context_id[16..24].copy_from_slice(&b.to_be_bytes());

    // Operand A. Label = marker 0x0a|op|type|pos|value → distinct PDA per operand/op (no collisions).
    let mut operand_label_a = [0x0au8; 32];
    operand_label_a[1] = op.as_u8();
    operand_label_a[2] = in_fhe_type;
    operand_label_a[3] = 0;
    operand_label_a[8..16].copy_from_slice(&a.to_be_bytes());
    let (encrypted_value_a, handle_a) = create_durable_public_decrypt_operand(
        host,
        payer,
        host_config,
        a,
        in_fhe_type,
        operand_label_a,
    )?;

    // remaining_accounts: [operand A, (operand B), output] — indices below map to this order.
    let mut operand_values = vec![encrypted_value_a];
    let rhs_operand = if b_scalar {
        let mut scalar_b = [0u8; 32];
        scalar_b[24..32].copy_from_slice(&b.to_be_bytes());
        zama_host::FheEvalOperand::Scalar(scalar_b)
    } else {
        let mut operand_label_b = [0x0au8; 32];
        operand_label_b[1] = op.as_u8();
        operand_label_b[2] = in_fhe_type;
        operand_label_b[3] = 1;
        operand_label_b[8..16].copy_from_slice(&b.to_be_bytes());
        let (encrypted_value_b, handle_b) = create_durable_public_decrypt_operand(
            host,
            payer,
            host_config,
            b,
            in_fhe_type,
            operand_label_b,
        )?;
        operand_values.push(encrypted_value_b);
        zama_host::FheEvalOperand::AllowedDurable {
            handle: handle_b,
            encrypted_value_index: 1,
        }
    };

    let output_index = operand_values.len() as u16;
    let steps = vec![zama_host::FheEvalStep::Binary {
        op,
        lhs: zama_host::FheEvalOperand::AllowedDurable {
            handle: handle_a,
            encrypted_value_index: 0,
        },
        rhs: rhs_operand,
        output_fhe_type,
        output: durable_output(
            host,
            output_encrypted_value,
            output_index,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            subjects,
        )?,
    }];

    // Operand lineages are read-only; the output lineage is writable.
    let mut remaining: Vec<AccountMeta> = operand_values
        .iter()
        .map(|encrypted_value| AccountMeta::new_readonly(*encrypted_value, false))
        .collect();
    remaining.push(AccountMeta::new(output_encrypted_value, false));

    let b_desc = if b_scalar {
        format!("scalar({b})")
    } else {
        format!("enc({b})")
    };
    let sig = host
        .request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            hcu_authority: payer.pubkey(),
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(remaining)
        .args(zama_host::instruction::FheEval {
            args: zama_host::FheEvalArgs { context_id, steps },
        })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval binary {op_name}(enc({a}), {b_desc}) fhe_type={in_fhe_type} out_type={output_fhe_type}: {sig}");

    let value = fetch_encrypted_value(host, output_encrypted_value)?;
    let handle_hex: String = value
        .current_handle
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    println!("  output encrypted value {output_encrypted_value}");
    println!("  result handle 0x{handle_hex}");

    if std::env::var("BINARY_ALLOW").is_ok() {
        allow_for_decryption(host, payer, host_config, output_encrypted_value)?;
    }
    Ok(())
}

/// Generic unary fhe_eval: TrivialEncrypt(operand) → Unary(op, AllowedDurable).
/// Env: UNARY_OP (Neg/Not/Cast), UNARY_A, UNARY_IN_FHE_TYPE (default 2=euint8),
/// UNARY_OUT_FHE_TYPE (default = UNARY_IN_FHE_TYPE), UNARY_ALLOW.
fn fhe_eval_unary(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    use anchor_lang::solana_program::instruction::AccountMeta;

    let op_name = std::env::var("UNARY_OP").unwrap_or_else(|_| "Neg".into());
    let op = parse_unary_op(&op_name).map_err(|e| e)?;
    let a: u64 = std::env::var("UNARY_A")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(42);
    let in_fhe_type: u8 = std::env::var("UNARY_IN_FHE_TYPE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2);
    let out_fhe_type: u8 = std::env::var("UNARY_OUT_FHE_TYPE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(in_fhe_type);

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let mut encrypted_value_label = [8u8; 32];
    encrypted_value_label[1] = op.as_u8();
    encrypted_value_label[2] = in_fhe_type;
    encrypted_value_label[3] = out_fhe_type;
    encrypted_value_label[24..32].copy_from_slice(&a.to_be_bytes());
    let output_encrypted_value =
        encrypted_value_address(acl_domain_key, app_account, encrypted_value_label);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];

    let mut context_id = [0u8; 32];
    context_id[0] = 0xb1;
    context_id[1] = op.as_u8();
    context_id[2] = in_fhe_type;
    context_id[3] = out_fhe_type;
    context_id[24..32].copy_from_slice(&a.to_be_bytes());

    // Encrypted operand as a durable public-decrypt handle (propagates to output; AllowedLocal → 6063).
    let mut operand_label_a = [0x0bu8; 32];
    operand_label_a[1] = op.as_u8();
    operand_label_a[2] = in_fhe_type;
    operand_label_a[3] = out_fhe_type;
    operand_label_a[8..16].copy_from_slice(&a.to_be_bytes());
    let (encrypted_value_a, handle_a) = create_durable_public_decrypt_operand(
        host,
        payer,
        host_config,
        a,
        in_fhe_type,
        operand_label_a,
    )?;

    let args = zama_host::FheEvalArgs {
        context_id,
        steps: vec![zama_host::FheEvalStep::Unary {
            op,
            operand: zama_host::FheEvalOperand::AllowedDurable {
                handle: handle_a,
                encrypted_value_index: 0,
            },
            output_fhe_type: out_fhe_type,
            output: durable_output(
                host,
                output_encrypted_value,
                1,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                subjects,
            )?,
        }],
    };

    let sig = host
        .request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            hcu_authority: payer.pubkey(),
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(vec![
            AccountMeta::new_readonly(encrypted_value_a, false),
            AccountMeta::new(output_encrypted_value, false),
        ])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval unary {op_name}(enc({a})) in_type={in_fhe_type} out_type={out_fhe_type}: {sig}");

    let value = fetch_encrypted_value(host, output_encrypted_value)?;
    let handle_hex: String = value
        .current_handle
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    println!("  output encrypted value {output_encrypted_value}");
    println!("  result handle 0x{handle_hex}");

    if std::env::var("UNARY_ALLOW").is_ok() {
        allow_for_decryption(host, payer, host_config, output_encrypted_value)?;
    }
    Ok(())
}

/// Ternary fhe_eval: TrivialEncrypt(ctrl as ebool) + TrivialEncrypt(if_true) +
/// TrivialEncrypt(if_false) → Ternary(IfThenElse, AllowedDurable).
/// Env: TERNARY_CTRL (0|1, default 1), TERNARY_TRUE/TERNARY_FALSE (u64 defaults 42/99),
/// TERNARY_FHE_TYPE (default 5=euint64), TERNARY_ALLOW.
fn fhe_eval_ternary(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    use anchor_lang::solana_program::instruction::AccountMeta;

    let ctrl: u64 = std::env::var("TERNARY_CTRL")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let if_true: u64 = std::env::var("TERNARY_TRUE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(42);
    let if_false: u64 = std::env::var("TERNARY_FALSE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(99);
    let fhe_type: u8 = std::env::var("TERNARY_FHE_TYPE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let mut encrypted_value_label = [9u8; 32];
    encrypted_value_label[1] = ctrl as u8;
    encrypted_value_label[2] = fhe_type;
    encrypted_value_label[8..16].copy_from_slice(&if_true.to_be_bytes());
    encrypted_value_label[16..24].copy_from_slice(&if_false.to_be_bytes());
    let output_encrypted_value =
        encrypted_value_address(acl_domain_key, app_account, encrypted_value_label);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];

    let mut context_id = [0u8; 32];
    context_id[0] = 0xb2;
    context_id[1] = ctrl as u8;
    context_id[2] = fhe_type;
    context_id[8..16].copy_from_slice(&if_true.to_be_bytes());
    context_id[16..24].copy_from_slice(&if_false.to_be_bytes());

    let expected = if ctrl != 0 { if_true } else { if_false };

    // Each operand a durable public-decrypt handle. remaining_accounts: [control, if_true, if_false, output].
    let mut label_ctrl = [0x0cu8; 32];
    label_ctrl[1] = ctrl as u8;
    label_ctrl[3] = 0; // control is ebool (type 0)
    label_ctrl[8..16].copy_from_slice(&ctrl.to_be_bytes());
    let (value_ctrl, h_ctrl) =
        create_durable_public_decrypt_operand(host, payer, host_config, ctrl, 0, label_ctrl)?;

    let mut label_true = [0x0cu8; 32];
    label_true[2] = fhe_type;
    label_true[3] = 1;
    label_true[8..16].copy_from_slice(&if_true.to_be_bytes());
    let (value_true, h_true) = create_durable_public_decrypt_operand(
        host,
        payer,
        host_config,
        if_true,
        fhe_type,
        label_true,
    )?;

    let mut label_false = [0x0cu8; 32];
    label_false[2] = fhe_type;
    label_false[3] = 2;
    label_false[8..16].copy_from_slice(&if_false.to_be_bytes());
    let (value_false, h_false) = create_durable_public_decrypt_operand(
        host,
        payer,
        host_config,
        if_false,
        fhe_type,
        label_false,
    )?;

    let args = zama_host::FheEvalArgs {
        context_id,
        steps: vec![zama_host::FheEvalStep::Ternary {
            op: zama_host::FheTernaryOpCode::IfThenElse,
            control: zama_host::FheEvalOperand::AllowedDurable {
                handle: h_ctrl,
                encrypted_value_index: 0,
            },
            if_true: zama_host::FheEvalOperand::AllowedDurable {
                handle: h_true,
                encrypted_value_index: 1,
            },
            if_false: zama_host::FheEvalOperand::AllowedDurable {
                handle: h_false,
                encrypted_value_index: 2,
            },
            output_fhe_type: fhe_type,
            output: durable_output(
                host,
                output_encrypted_value,
                3,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                subjects,
            )?,
        }],
    };

    let sig = host
        .request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            hcu_authority: payer.pubkey(),
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(vec![
            AccountMeta::new_readonly(value_ctrl, false),
            AccountMeta::new_readonly(value_true, false),
            AccountMeta::new_readonly(value_false, false),
            AccountMeta::new(output_encrypted_value, false),
        ])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval ternary select(ctrl={ctrl}, true={if_true}, false={if_false}) -> {expected}: {sig}");

    let value = fetch_encrypted_value(host, output_encrypted_value)?;
    let handle_hex: String = value
        .current_handle
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    println!("  output encrypted value {output_encrypted_value}");
    println!("  result handle 0x{handle_hex}  (expected cleartext={expected})");

    if std::env::var("TERNARY_ALLOW").is_ok() {
        allow_for_decryption(host, payer, host_config, output_encrypted_value)?;
    }
    Ok(())
}

/// Bounded random fhe_eval: RandBounded(upper_bound, AllowedDurable).
/// Env: RAND_UPPER (u64 exclusive upper bound, default 100), RAND_FHE_TYPE (default 5=euint64),
/// RAND_ALLOW. Expected: cleartext in [0, RAND_UPPER).
fn fhe_eval_rand_bounded(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    use anchor_lang::solana_program::instruction::AccountMeta;

    let upper: u64 = std::env::var("RAND_UPPER")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);
    let fhe_type: u8 = std::env::var("RAND_FHE_TYPE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let mut encrypted_value_label = [0x0au8; 32];
    encrypted_value_label[1] = fhe_type;
    encrypted_value_label[24..32].copy_from_slice(&upper.to_be_bytes());
    let output_encrypted_value =
        encrypted_value_address(acl_domain_key, app_account, encrypted_value_label);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];

    let mut context_id = [0u8; 32];
    context_id[0] = 0xb3;
    context_id[1] = fhe_type;
    context_id[24..32].copy_from_slice(&upper.to_be_bytes());

    let mut upper_bound = [0u8; 32];
    upper_bound[24..32].copy_from_slice(&upper.to_be_bytes());

    let args = zama_host::FheEvalArgs {
        context_id,
        steps: vec![zama_host::FheEvalStep::RandBounded {
            upper_bound,
            fhe_type,
            output: durable_output(
                host,
                output_encrypted_value,
                0,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                subjects,
            )?,
        }],
    };

    let sig = host
        .request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            hcu_authority: payer.pubkey(),
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(vec![AccountMeta::new(output_encrypted_value, false)])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval rand_bounded(upper={upper}) fhe_type={fhe_type}: {sig}");

    let value = fetch_encrypted_value(host, output_encrypted_value)?;
    let handle_hex: String = value
        .current_handle
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    println!("  output encrypted value {output_encrypted_value}");
    println!("  result handle 0x{handle_hex}  (expected cleartext in [0, {upper}))");

    if std::env::var("RAND_ALLOW").is_ok() {
        allow_for_decryption(host, payer, host_config, output_encrypted_value)?;
    }
    Ok(())
}

/// Multi-step fhe_eval Sum: two TrivialEncrypt(AllowedLocal) → Sum(AllowedDurable).
/// SUM_A/SUM_B select the euint64 addends (defaults 10/20). Expected result: SUM_A + SUM_B.
/// SUM_ALLOW marks the output publicly decryptable after the eval.
fn fhe_eval_sum(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    use anchor_lang::solana_program::instruction::AccountMeta;
    let a: u64 = std::env::var("SUM_A")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    let b: u64 = std::env::var("SUM_B")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(20);
    let fhe_type: u8 = 5; // euint64

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let mut encrypted_value_label = [4u8; 32];
    encrypted_value_label[16..24].copy_from_slice(&a.to_be_bytes());
    encrypted_value_label[24..32].copy_from_slice(&b.to_be_bytes());
    let output_encrypted_value =
        encrypted_value_address(acl_domain_key, app_account, encrypted_value_label);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];

    let mut context_id = [0u8; 32];
    context_id[0] = 0xa0;
    context_id[16..24].copy_from_slice(&a.to_be_bytes());
    context_id[24..32].copy_from_slice(&b.to_be_bytes());

    // Each addend a durable public-decrypt handle. remaining_accounts: [a, b, output].
    let mut label_a = [0x0du8; 32];
    label_a[2] = fhe_type;
    label_a[3] = 0;
    label_a[8..16].copy_from_slice(&a.to_be_bytes());
    let (value_a, h_a) =
        create_durable_public_decrypt_operand(host, payer, host_config, a, fhe_type, label_a)?;

    let mut label_b = [0x0du8; 32];
    label_b[2] = fhe_type;
    label_b[3] = 1;
    label_b[8..16].copy_from_slice(&b.to_be_bytes());
    let (value_b, h_b) =
        create_durable_public_decrypt_operand(host, payer, host_config, b, fhe_type, label_b)?;

    let args = zama_host::FheEvalArgs {
        context_id,
        steps: vec![zama_host::FheEvalStep::Sum {
            operands: vec![
                zama_host::FheEvalOperand::AllowedDurable {
                    handle: h_a,
                    encrypted_value_index: 0,
                },
                zama_host::FheEvalOperand::AllowedDurable {
                    handle: h_b,
                    encrypted_value_index: 1,
                },
            ],
            fhe_type,
            output: durable_output(
                host,
                output_encrypted_value,
                2,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                subjects,
            )?,
        }],
    };

    let sig = host
        .request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            hcu_authority: payer.pubkey(),
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(vec![
            AccountMeta::new_readonly(value_a, false),
            AccountMeta::new_readonly(value_b, false),
            AccountMeta::new(output_encrypted_value, false),
        ])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval sum({a} + {b} as euint64): {sig}");

    let value = fetch_encrypted_value(host, output_encrypted_value)?;
    let handle_hex: String = value
        .current_handle
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    println!("  output encrypted value {output_encrypted_value}");
    println!("  result handle 0x{handle_hex}  (tfhe-worker materializes sum({a} + {b}))");

    if std::env::var("SUM_ALLOW").is_ok() {
        allow_for_decryption(host, payer, host_config, output_encrypted_value)?;
    }
    Ok(())
}

/// Multi-step fhe_eval IsIn: TrivialEncrypt(value) + TrivialEncrypt(set elements) (all AllowedLocal)
/// → IsIn(AllowedDurable → ebool). ISIN_VALUE selects the euint64 value (default 42); the set is
/// hardcoded as [10, 42, 100]. ISIN_ALLOW marks the output publicly decryptable. Expected result:
/// 1 (true) when ISIN_VALUE is in the set, 0 (false) otherwise.
fn fhe_eval_is_in(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    use anchor_lang::solana_program::instruction::AccountMeta;
    let value: u64 = std::env::var("ISIN_VALUE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(42);
    let elem_fhe_type: u8 = 5; // euint64

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let mut encrypted_value_label = [5u8; 32];
    encrypted_value_label[24..32].copy_from_slice(&value.to_be_bytes());
    let output_encrypted_value =
        encrypted_value_address(acl_domain_key, app_account, encrypted_value_label);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];

    let mut context_id = [0u8; 32];
    context_id[0] = 0xa1;
    context_id[24..32].copy_from_slice(&value.to_be_bytes());

    // value + set as durable public-decrypt handles. remaining_accounts: [value, set.., output].
    let set_values: [u64; 3] = [10, 42, 100];

    let mut label_value = [0x0eu8; 32];
    label_value[2] = elem_fhe_type;
    label_value[3] = 0;
    label_value[8..16].copy_from_slice(&value.to_be_bytes());
    let (encrypted_value_input, h_value) = create_durable_public_decrypt_operand(
        host,
        payer,
        host_config,
        value,
        elem_fhe_type,
        label_value,
    )?;

    let mut operand_values = vec![encrypted_value_input];
    let mut set_operands: Vec<zama_host::FheEvalOperand> = Vec::with_capacity(set_values.len());
    for (i, &v) in set_values.iter().enumerate() {
        let mut label = [0x0eu8; 32];
        label[2] = elem_fhe_type;
        label[3] = (i as u8) + 1;
        label[8..16].copy_from_slice(&v.to_be_bytes());
        label[16..24].copy_from_slice(&value.to_be_bytes());
        let (encrypted_value, handle) = create_durable_public_decrypt_operand(
            host,
            payer,
            host_config,
            v,
            elem_fhe_type,
            label,
        )?;
        set_operands.push(zama_host::FheEvalOperand::AllowedDurable {
            handle,
            encrypted_value_index: operand_values.len() as u16,
        });
        operand_values.push(encrypted_value);
    }
    let output_index = operand_values.len() as u16;

    let steps = vec![zama_host::FheEvalStep::IsIn {
        value: zama_host::FheEvalOperand::AllowedDurable {
            handle: h_value,
            encrypted_value_index: 0,
        },
        set: set_operands,
        fhe_type: elem_fhe_type,
        output: durable_output(
            host,
            output_encrypted_value,
            output_index,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            subjects,
        )?,
    }];

    let args = zama_host::FheEvalArgs { context_id, steps };
    let in_set = set_values.contains(&value);

    let mut remaining: Vec<AccountMeta> = operand_values
        .iter()
        .map(|encrypted_value| AccountMeta::new_readonly(*encrypted_value, false))
        .collect();
    remaining.push(AccountMeta::new(output_encrypted_value, false));

    let sig = host
        .request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            hcu_authority: payer.pubkey(),
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(remaining)
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval isIn({value} in {set_values:?} as euint64): {sig}");

    let value_account = fetch_encrypted_value(host, output_encrypted_value)?;
    let handle_hex: String = value_account
        .current_handle
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    println!("  output encrypted value {output_encrypted_value}");
    println!("  result handle 0x{handle_hex}  (ebool; tfhe-worker materializes isIn -> {in_set})");

    if std::env::var("ISIN_ALLOW").is_ok() {
        allow_for_decryption(host, payer, host_config, output_encrypted_value)?;
    }
    Ok(())
}

/// Multi-step fhe_eval MulDiv: TrivialEncrypt(factor1, AllowedLocal) → MulDiv(factor1, scalar_b,
/// divisor, AllowedDurable). MULDIV_A/MULDIV_B/MULDIV_D select the euint64 operands (defaults
/// 6/7/3). MULDIV_ALLOW marks the output publicly decryptable. Expected result: A * B / D.
fn fhe_eval_mul_div(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    use anchor_lang::solana_program::instruction::AccountMeta;
    let a: u64 = std::env::var("MULDIV_A")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(6);
    let b: u64 = std::env::var("MULDIV_B")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(7);
    let d: u64 = std::env::var("MULDIV_D")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3);
    let fhe_type: u8 = 5; // euint64

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let mut encrypted_value_label = [6u8; 32];
    encrypted_value_label[8..16].copy_from_slice(&a.to_be_bytes());
    encrypted_value_label[16..24].copy_from_slice(&b.to_be_bytes());
    encrypted_value_label[24..32].copy_from_slice(&d.to_be_bytes());
    let output_encrypted_value =
        encrypted_value_address(acl_domain_key, app_account, encrypted_value_label);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];

    let mut context_id = [0u8; 32];
    context_id[0] = 0xa2;
    context_id[8..16].copy_from_slice(&a.to_be_bytes());
    context_id[16..24].copy_from_slice(&b.to_be_bytes());
    context_id[24..32].copy_from_slice(&d.to_be_bytes());

    let mut scalar_b = [0u8; 32];
    scalar_b[24..32].copy_from_slice(&b.to_be_bytes());
    let mut divisor = [0u8; 32];
    divisor[24..32].copy_from_slice(&d.to_be_bytes());

    // factor1 as a durable public-decrypt handle (factor2/divisor are public scalars).
    let mut label_a = [0x0fu8; 32];
    label_a[2] = fhe_type;
    label_a[8..16].copy_from_slice(&a.to_be_bytes());
    let (encrypted_value_a, h_a) =
        create_durable_public_decrypt_operand(host, payer, host_config, a, fhe_type, label_a)?;

    let args = zama_host::FheEvalArgs {
        context_id,
        steps: vec![zama_host::FheEvalStep::MulDiv {
            factor1: zama_host::FheEvalOperand::AllowedDurable {
                handle: h_a,
                encrypted_value_index: 0,
            },
            factor2: zama_host::FheEvalOperand::Scalar(scalar_b),
            divisor,
            output_fhe_type: fhe_type,
            output: durable_output(
                host,
                output_encrypted_value,
                1,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                subjects,
            )?,
        }],
    };

    let expected = a * b / d;
    let sig = host
        .request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            hcu_authority: payer.pubkey(),
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(vec![
            AccountMeta::new_readonly(encrypted_value_a, false),
            AccountMeta::new(output_encrypted_value, false),
        ])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval mulDiv({a} * {b} / {d} = {expected} as euint64): {sig}");

    let value = fetch_encrypted_value(host, output_encrypted_value)?;
    let handle_hex: String = value
        .current_handle
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    println!("  output encrypted value {output_encrypted_value}");
    println!("  result handle 0x{handle_hex}  (tfhe-worker materializes mulDiv -> {expected})");

    if std::env::var("MULDIV_ALLOW").is_ok() {
        allow_for_decryption(host, payer, host_config, output_encrypted_value)?;
    }
    Ok(())
}

/// Creates a confidential mint wrapping $UNDERLYING_MINT. Exercises the host<->token
/// CPI that trivial-encrypts the initial total-supply handle and creates its lineage.
fn initialize_mint(
    token: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let underlying_mint = Pubkey::from_str(&std::env::var("UNDERLYING_MINT")?)?;
    let mint = Keypair::new();
    let mint_pk = mint.pubkey();

    let (compute_signer, _) =
        Pubkey::find_program_address(&[b"fhe-compute", mint_pk.as_ref()], &confidential_token::ID);
    let (total_supply_authority, _) = Pubkey::find_program_address(
        &[b"total-supply", mint_pk.as_ref()],
        &confidential_token::ID,
    );
    let (total_supply_encrypted_value, _) =
        confidential_token::total_supply_encrypted_value_address(mint_pk, total_supply_authority);
    let (token_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &confidential_token::ID);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);

    let sig = token
        .request()
        .accounts(confidential_token::accounts::InitializeMint {
            authority: payer.pubkey(),
            mint: mint_pk,
            underlying_mint,
            compute_signer,
            total_supply_authority,
            total_supply_encrypted_value,
            zama_event_authority,
            zama_program: zama_host::ID,
            host_config,
            system_program: system_program::ID,
            hcu_authority: confidential_token::hcu_authority_address(mint_pk).0,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: token_event_authority,
            program: confidential_token::ID,
        })
        .args(confidential_token::instruction::InitializeMint {})
        .signer(&mint)
        .send()?;
    println!("OK initialize_mint: {sig}");
    println!("  confidential mint  {mint_pk}");
    // The mint compute-signer PDA is the fromExternal `contract` an attested transfer/burn amount
    // must bind to; print base58 + 0x-hex so the e2e can fetch a compute-signer-bound input-proof.
    println!(
        "  compute_signer     {compute_signer} 0x{}",
        compute_signer
            .to_bytes()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>()
    );
    println!("  underlying SPL     {underlying_mint}");
    println!("  total_supply ACL   {total_supply_encrypted_value}");

    let acl = token.rpc().get_account(&total_supply_encrypted_value)?;
    println!(
        "  ACL record owner={} bytes={}  (created via host CPI)",
        acl.owner,
        acl.data.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn historical_proof_for_first_supersession_verifies() {
        let encrypted_value_account = [0xAC; 32];
        let old_handle = [0x10; 32];
        let subject = [0x01; 32];
        let leaf = zama_solana_acl::historical_access_leaf_commitment(
            encrypted_value_account,
            0,
            old_handle,
            subject,
        );
        let mut peaks = Vec::new();
        let mut leaf_count = 0;
        zama_solana_acl::mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();

        let (proof, proof_bytes) = build_historical_access_proof(
            encrypted_value_account,
            &peaks,
            leaf_count,
            old_handle,
            subject,
        )
        .unwrap();

        assert_eq!(proof.leaf_index, 0);
        assert!(proof.siblings.is_empty());
        assert_eq!(proof_bytes[0], MMR_MODE_HISTORICAL);
        assert!(zama_solana_acl::mmr_verify(
            &peaks, leaf_count, leaf, &proof
        ));
    }

    #[test]
    fn public_decrypt_proof_for_made_public_handle_verifies() {
        let encrypted_value_account = [0xAC; 32];
        let handle = [0x20; 32];
        let events = [zama_solana_acl::LineageEvent::MarkedPublic { handle }];
        let reconstructed = zama_solana_acl::reconstruct(encrypted_value_account, &events).unwrap();
        let proof = reconstructed.build_proof(0).unwrap();
        let mut value = zama_solana_acl::EncryptedValue {
            current_handle: handle,
            leaf_count: reconstructed.leaf_count,
            peaks: reconstructed.peaks.clone(),
            ..Default::default()
        };
        value.subjects = vec![[0x01; 32]];
        let host_value = zama_host::EncryptedValue {
            acl_domain_key: Pubkey::new_from_array(value.acl_domain_key),
            app_account: Pubkey::new_from_array(value.app_account),
            encrypted_value_label: value.encrypted_value_label,
            current_handle: value.current_handle,
            subjects: value
                .subjects
                .iter()
                .copied()
                .map(Pubkey::new_from_array)
                .collect(),
            leaf_count: value.leaf_count,
            peaks: value.peaks.clone(),
            bump: value.bump,
        };

        let (built, proof_bytes) =
            build_public_decrypt_proof(encrypted_value_account, &host_value, handle).unwrap();
        assert_eq!(built, proof);
        assert_eq!(proof_bytes[0], MMR_MODE_PUBLIC);
        assert!(
            zama_solana_acl::authorize_public(encrypted_value_account, &value, handle, &built)
                .is_ok()
        );
    }

    #[test]
    fn public_decrypt_proof_for_twice_public_handle_verifies() {
        // Regression for the full-vertical born-public burned handle: it is made public
        // TWICE (born-public burn per DD-036, then an explicit make_handle_public seal),
        // so the on-chain lineage has leaf_count == 2 for the SAME handle. A single-event
        // reconstruction yields leaf_count == 1 and diverges (PeaksDiverged); the proof
        // builder must reconstruct one MarkedPublic per on-chain leaf.
        let encrypted_value_account = [0xAC; 32];
        let handle = [0x21; 32];
        let events = [
            zama_solana_acl::LineageEvent::MarkedPublic { handle },
            zama_solana_acl::LineageEvent::MarkedPublic { handle },
        ];
        let reconstructed = zama_solana_acl::reconstruct(encrypted_value_account, &events).unwrap();
        assert_eq!(reconstructed.leaf_count, 2);
        let mut value = zama_solana_acl::EncryptedValue {
            current_handle: handle,
            leaf_count: reconstructed.leaf_count,
            peaks: reconstructed.peaks.clone(),
            ..Default::default()
        };
        value.subjects = vec![[0x01; 32]];
        let host_value = zama_host::EncryptedValue {
            acl_domain_key: Pubkey::new_from_array(value.acl_domain_key),
            app_account: Pubkey::new_from_array(value.app_account),
            encrypted_value_label: value.encrypted_value_label,
            current_handle: value.current_handle,
            subjects: value
                .subjects
                .iter()
                .copied()
                .map(Pubkey::new_from_array)
                .collect(),
            leaf_count: value.leaf_count,
            peaks: value.peaks.clone(),
            bump: value.bump,
        };

        // Regressed as PeaksDiverged before the leaf_count-aware reconstruction fix.
        let (built, proof_bytes) =
            build_public_decrypt_proof(encrypted_value_account, &host_value, handle).unwrap();
        assert_eq!(proof_bytes[0], MMR_MODE_PUBLIC);
        assert!(
            zama_solana_acl::authorize_public(encrypted_value_account, &value, handle, &built)
                .is_ok()
        );
    }

    #[test]
    fn historical_leaf_is_rejected_by_public_verify() {
        let encrypted_value_account = [0xAC; 32];
        let old_handle = [0x10; 32];
        let subject = [0x01; 32];
        let historical_leaf = zama_solana_acl::historical_access_leaf_commitment(
            encrypted_value_account,
            0,
            old_handle,
            subject,
        );
        let mut peaks = Vec::new();
        let mut leaf_count = 0;
        zama_solana_acl::mmr_append(&mut peaks, &mut leaf_count, historical_leaf).unwrap();
        let proof = zama_solana_acl::mmr_build_proof(&[historical_leaf], 0).unwrap();
        let proof_bytes = proof_blob(MMR_MODE_HISTORICAL, &proof).unwrap();
        let value = zama_solana_acl::EncryptedValue {
            current_handle: [0x11; 32],
            leaf_count,
            peaks,
            ..Default::default()
        };

        assert_eq!(proof_bytes[0], MMR_MODE_HISTORICAL);
        assert!(zama_solana_acl::authorize_public(
            encrypted_value_account,
            &value,
            old_handle,
            &proof
        )
        .is_err());
    }
}
