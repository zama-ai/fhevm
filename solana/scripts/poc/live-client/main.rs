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
use solana_keypair::{read_keypair_file, Keypair};

const EVENT_AUTHORITY_SEED: &[u8] = b"__event_authority";

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

    // BIND_INPUT drives the coprocessor-attested input bind against the live host_config
    // (created by BOOTSTRAP with the real gateway verifier config); it does not touch
    // host_config or the mint, so it runs standalone with the relayer-returned attestation.
    if std::env::var("BIND_INPUT").is_ok() {
        bind_coprocessor_input(&host, host_config)?;
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

/// Verifies a coprocessor-attested input on the live zama-host: feeds the handle + EIP-712
/// `CiphertextVerification` attestation (handles + signature) the relayer returned, which the
/// host verifies on-chain via secp256k1_recover against the configured coprocessor signer
/// before emitting the verified-input receipt. No persistent ACL is created (EVM parity).
/// Inputs come from the relayer response via env
/// (BIND_HANDLE, BIND_COPRO_SIG, BIND_USER, BIND_CONTRACT, BIND_CHAIN_ID, optional BIND_EXTRA).
fn bind_coprocessor_input(
    host: &Program<Rc<Keypair>>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let handle: [u8; 32] = hexdec(&std::env::var("BIND_HANDLE")?)
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

    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);

    let handle_hex: String = handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("binding input_handle 0x{handle_hex}");
    let sig = host
        .request()
        .accounts(zama_host::accounts::VerifyCoprocessorInput {
            host_config,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .args(zama_host::instruction::VerifyCoprocessorInput {
            input_handle: handle,
            ct_handles: vec![handle],
            handle_index: 0,
            user_address,
            contract_address,
            contract_chain_id,
            extra_data,
            signatures: vec![signature],
        })
        .send()?;
    println!("OK verify_coprocessor_input: {sig}");
    println!("  (secp256k1 coprocessor attestation verified on-chain; no persistent input ACL)");
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
    let value: u64 = std::env::var("TE_VALUE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(42);
    // ClearConst::from_be_slice reads the plaintext big-endian, so place the u64 in the low bytes.
    let mut plaintext = [0u8; 32];
    plaintext[24..32].copy_from_slice(&value.to_be_bytes());
    let fhe_type: u8 = 5; // euint64

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    // Distinct per value (and from the input-bind record) so repeated runs derive distinct
    // output ACL record PDAs rather than colliding on an already-initialized account.
    let mut encrypted_value_label = [1u8; 32];
    encrypted_value_label[24..32].copy_from_slice(&value.to_be_bytes());
    let output_nonce_key =
        zama_host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_nonce_sequence: u64 = 0;
    let (output_acl_record, _) =
        zama_host::acl_record_address(output_nonce_key, output_nonce_sequence);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    // ACL_ROLE_ALL (user) grants USE | GRANT | PUBLIC_DECRYPT, so the subject can later mark
    // this compute output publicly decryptable via allow_for_decryption.
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];

    // SlotHashes (read via sol_get_sysvar for the result-handle entropy) is populated in
    // real execution but not in RPC preflight simulation, so skip preflight for this op.
    let sig = host
        .request()
        .accounts(zama_host::accounts::TrivialEncryptAndBind {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .args(zama_host::instruction::TrivialEncryptAndBind {
            plaintext,
            fhe_type,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: subjects,
            output_public_decrypt: false,
        })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK trivial_encrypt_and_bind ({value} as euint64): {sig}");

    // The result handle is computed on-chain; read it back from the ACL record so the
    // decrypt step can target it.
    let record: zama_host::AclRecord = host.account(output_acl_record)?;
    let handle_hex: String = record.handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("  output ACL record {output_acl_record}");
    println!("  result handle 0x{handle_hex}  (tfhe-worker materializes this ciphertext)");

    // TE_ALLOW marks the freshly-computed handle publicly decryptable on the zama-host ACL
    // record (the subject holds ACL_ROLE_PUBLIC_DECRYPT via the user role above), emitting a
    // PublicDecryptAllowedEvent — the precondition for a public decrypt of this handle.
    if std::env::var("TE_ALLOW").is_ok() {
        let allow_sig = host
            .request()
            .accounts(zama_host::accounts::AllowForDecryption {
                authority: payer.pubkey(),
                authority_permission_record: None,
                acl_record: output_acl_record,
                host_config,
                deny_subject_record: None,
                event_authority: zama_event_authority,
                program: zama_host::ID,
            })
            .args(zama_host::instruction::AllowForDecryption {
                handle: record.handle,
            })
            .send()?;
        println!("OK allow_for_decryption (public): {allow_sig}");
    }
    Ok(())
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
    use anchor_lang::solana_program::instruction::AccountMeta;
    let value: u64 = std::env::var("TE_VALUE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(42);
    let mut plaintext = [0u8; 32];
    plaintext[24..32].copy_from_slice(&value.to_be_bytes());
    let fhe_type: u8 = 5; // euint64

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    // Distinct label per value so repeated runs derive distinct output ACL record PDAs. Use a
    // marker byte distinct from trivial_encrypt_and_bind's [1u8;32] label so the two compute paths
    // never collide on an already-initialized account.
    let mut encrypted_value_label = [2u8; 32];
    encrypted_value_label[24..32].copy_from_slice(&value.to_be_bytes());
    let output_nonce_key =
        zama_host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_nonce_sequence: u64 = 0;
    let (output_acl_record, _) =
        zama_host::acl_record_address(output_nonce_key, output_nonce_sequence);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    // ACL_ROLE_ALL (user) grants USE | GRANT | PUBLIC_DECRYPT so the subject can later mark this
    // compute output publicly decryptable. public_decrypt MUST be false at birth (the eval birth
    // policy rejects setting it at creation); allow_for_decryption flips it after.
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];

    // Context id: any non-zero 32-byte domain separator for the transient handles in this eval.
    let mut context_id = [0u8; 32];
    context_id[0] = 0xc0;
    context_id[24..32].copy_from_slice(&value.to_be_bytes());

    let args = zama_host::FheEvalArgs {
        context_id,
        steps: vec![zama_host::FheEvalStep::TrivialEncrypt {
            plaintext,
            fhe_type,
            output: zama_host::FheEvalOutput::AllowedDurable {
                output_acl_record_index: 0,
                output_app_account_authority_index: None,
                output_nonce_key,
                output_nonce_sequence,
                output_acl_domain_key: acl_domain_key,
                output_app_account: app_account,
                output_encrypted_value_label: encrypted_value_label,
                output_subjects: subjects,
                output_public_decrypt: false,
            },
        }],
    };

    // SlotHashes (read for the result-handle entropy) is populated only in real execution, not in
    // RPC preflight simulation, so skip preflight for this op.
    let sig = host
        .request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        // The single durable output ACL record is remaining_accounts[0] (writable, created by the
        // eval executor); it is referenced by output_acl_record_index: 0 in the plan.
        .accounts(vec![AccountMeta::new(output_acl_record, false)])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval trivial_encrypt ({value} as euint64): {sig}");

    let record: zama_host::AclRecord = host.account(output_acl_record)?;
    let handle_hex: String = record.handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("  output ACL record {output_acl_record}");
    println!("  result handle 0x{handle_hex}  (tfhe-worker materializes this ciphertext)");

    if std::env::var("TE_ALLOW").is_ok() {
        let allow_sig = host
            .request()
            .accounts(zama_host::accounts::AllowForDecryption {
                authority: payer.pubkey(),
                authority_permission_record: None,
                acl_record: output_acl_record,
                host_config,
                deny_subject_record: None,
                event_authority: zama_event_authority,
                program: zama_host::ID,
            })
            .args(zama_host::instruction::AllowForDecryption {
                handle: record.handle,
            })
            .send()?;
        println!("OK allow_for_decryption (public): {allow_sig}");
    }
    Ok(())
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
    let output_nonce_key =
        zama_host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_nonce_sequence: u64 = 0;
    let (output_acl_record, _) =
        zama_host::acl_record_address(output_nonce_key, output_nonce_sequence);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    // ACL_ROLE_ALL (user) grants USE | GRANT | PUBLIC_DECRYPT so the subject can later mark the
    // result publicly decryptable. public_decrypt MUST be false at birth; allow_for_decryption flips
    // it after.
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];

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
            lhs: zama_host::FheEvalOperand::VerifiedInput { attestation },
            rhs: zama_host::FheEvalOperand::Scalar(scalar),
            output_fhe_type: fhe_type,
            output: zama_host::FheEvalOutput::AllowedDurable {
                output_acl_record_index: 0,
                output_app_account_authority_index: None,
                output_nonce_key,
                output_nonce_sequence,
                output_acl_domain_key: acl_domain_key,
                output_app_account: app_account,
                output_encrypted_value_label: encrypted_value_label,
                output_subjects: subjects,
                output_public_decrypt: false,
            },
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
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        // The single durable output ACL record is remaining_accounts[0] (writable, created by the
        // eval executor); referenced by output_acl_record_index: 0 in the plan.
        .accounts(vec![AccountMeta::new(output_acl_record, false)])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval verified-input add: {sig}");

    let record: zama_host::AclRecord = host.account(output_acl_record)?;
    let handle_hex: String = record.handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("  output ACL record {output_acl_record}");
    println!("  result handle 0x{handle_hex}  (tfhe-worker materializes input + {addend})");

    if std::env::var("TE_ALLOW").is_ok() {
        let allow_sig = host
            .request()
            .accounts(zama_host::accounts::AllowForDecryption {
                authority: payer.pubkey(),
                authority_permission_record: None,
                acl_record: output_acl_record,
                host_config,
                deny_subject_record: None,
                event_authority: zama_event_authority,
                program: zama_host::ID,
            })
            .args(zama_host::instruction::AllowForDecryption {
                handle: record.handle,
            })
            .send()?;
        println!("OK allow_for_decryption (public): {allow_sig}");
    }
    Ok(())
}

/// Consume step 1: seal the token amount's ciphertext material on-chain (commit_handle_material,
/// signed by the configured material_authority, with the real ct64/ct128 digests) and create the
/// disclosure request witness (request_disclose_amount → CPIs zama-host allow_public_decrypt, pins
/// the host's current KMS context id + expires_slot + request_hash into a DisclosureRequest PDA
/// the disclose_amount_secp consume step then binds to). The material commitment MUST exist before
/// the request (the request validates it), so commit always precedes the request here. Inputs via
/// env: MINT, TS_ACL, TS_HANDLE, KEY_ID, CT64_DIGEST, CT128_DIGEST, COPROC_SET_DIGEST; optional
/// SEAL_SKIP_COMMIT (material already committed), REQUEST_NONCE (32-byte hex), REQUEST_TTL_SLOTS.
fn consume_seal(
    host: &Program<Rc<Keypair>>,
    token: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let mint = Pubkey::from_str(&std::env::var("MINT")?)?;
    let ts_acl = Pubkey::from_str(&std::env::var("TS_ACL")?)?;
    let handle: [u8; 32] = hexdec(&std::env::var("TS_HANDLE")?)
        .try_into()
        .expect("TS_HANDLE");
    let (zama_evt, _) = Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let (material_commitment, _) = zama_host::handle_material_address(ts_acl);

    // The disclosure request validates the material commitment, so it must already exist: commit
    // it now unless SEAL_SKIP_COMMIT says a prior step already did.
    if std::env::var("SEAL_SKIP_COMMIT").is_err() {
        let key_id: [u8; 32] = hexdec(&std::env::var("KEY_ID")?)
            .try_into()
            .expect("KEY_ID");
        let ct64: [u8; 32] = hexdec(&std::env::var("CT64_DIGEST")?)
            .try_into()
            .expect("CT64_DIGEST");
        let ct128: [u8; 32] = hexdec(&std::env::var("CT128_DIGEST")?)
            .try_into()
            .expect("CT128_DIGEST");
        let coproc: [u8; 32] = hexdec(&std::env::var("COPROC_SET_DIGEST")?)
            .try_into()
            .expect("COPROC_SET_DIGEST");

        let sig = host
            .request()
            .accounts(zama_host::accounts::CommitHandleMaterial {
                payer: payer.pubkey(),
                material_authority: payer.pubkey(),
                host_config,
                acl_record: ts_acl,
                material_commitment,
                system_program: system_program::ID,
                event_authority: zama_evt,
                program: zama_host::ID,
            })
            .args(zama_host::instruction::CommitHandleMaterial {
                key_id,
                ciphertext_digest: ct64,
                sns_ciphertext_digest: ct128,
                coprocessor_set_digest: coproc,
            })
            .send()?;
        println!("OK commit_handle_material: {sig}");
        println!("  material commitment {material_commitment}");
    }

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
            amount_acl_record: ts_acl,
            amount_material_commitment: material_commitment,
            disclosure_request,
            // The disclosable amount (e.g. a confidential_burn burned amount) grants the owner
            // ACL_ROLE_ALL inline, so the requester is found inline — assert_record_subject_role
            // requires the overflow permission witness to be absent in that case.
            authority_permission_record: None,
            deny_subject_record: None,
            zama_event_authority: zama_evt,
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
    let ctx_id: u64 = std::env::var("KMS_CTX_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let nonce = request_nonce_from_env();
    let (material_commitment, _) = zama_host::handle_material_address(ts_acl);
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
            amount_acl_record: ts_acl,
            amount_material_commitment: material_commitment,
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

    // 1. initialize_token_account (initial balance 0) — creates the account + balance ACL via CPI.
    let (balance_acl, _) = zama_host::acl_record_address(
        confidential_token::balance_nonce_key(mint, token_account),
        0,
    );
    if token.rpc().get_account(&token_account).is_err() {
        let sig = token
            .request()
            .accounts(confidential_token::accounts::InitializeTokenAccount {
                owner,
                mint,
                compute_signer,
                token_account,
                acl_record: balance_acl,
                zama_event_authority: zama_evt,
                zama_program: zama_host::ID,
                host_config,
                system_program: system_program::ID,
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
    let (amount_acl, _) =
        zama_host::acl_record_address(confidential_token::nonce_key(mint, owner, label), 0);
    let sig2 = token
        .request()
        .accounts(confidential_token::accounts::CreateRandomAmount {
            owner,
            mint,
            token_account,
            compute_signer,
            amount_acl_record: amount_acl,
            zama_event_authority: zama_evt,
            zama_program: zama_host::ID,
            host_config,
            system_program: system_program::ID,
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

    let acct = token.rpc().get_account(&amount_acl)?;
    let mut handle = [0u8; 32];
    handle.copy_from_slice(&acct.data[8..40]);
    let hh: String = handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("  amount ACL    {amount_acl}");
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

    // Current balance/total-supply ACLs (sequence 0 from init) and their rotated outputs
    // (sequence 1). The wrapped public amount is trivial-encrypted into its own amount ACL.
    let bal_nk = confidential_token::balance_nonce_key(mint, token_account);
    let (current_balance_acl, _) = zama_host::acl_record_address(bal_nk, 0);
    let (output_balance_acl, _) = zama_host::acl_record_address(bal_nk, 1);
    let ts_nk = confidential_token::total_supply_nonce_key(mint, ts_authority);
    let (current_ts_acl, _) = zama_host::acl_record_address(ts_nk, 0);
    let (output_ts_acl, _) = zama_host::acl_record_address(ts_nk, 1);
    // The wrap amount is trivial-encrypted as a transient inside wrap_usdc's eval frame, so no
    // separate amount ACL account is supplied.
    let (zama_evt, _) = Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let (token_evt, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &confidential_token::ID);

    // The confidential token account (balance ACL at sequence 0) must exist before wrap draws
    // from it; create it (idempotent) if missing.
    if token.rpc().get_account(&token_account).is_err() {
        let init_sig = token
            .request()
            .accounts(confidential_token::accounts::InitializeTokenAccount {
                owner,
                mint,
                compute_signer,
                token_account,
                acl_record: current_balance_acl,
                zama_event_authority: zama_evt,
                zama_program: zama_host::ID,
                host_config,
                system_program: system_program::ID,
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
            current_compute_acl: current_balance_acl,
            current_total_supply_acl: current_ts_acl,
            output_acl: output_balance_acl,
            total_supply_output_acl: output_ts_acl,
            zama_event_authority: zama_evt,
            zama_program: zama_host::ID,
            host_config,
            token_program: spl_token_id,
            system_program: system_program::ID,
            event_authority: token_evt,
            program: confidential_token::ID,
        })
        .args(confidential_token::instruction::WrapUsdc { amount })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: std::env::var("WRAP_NO_PREFLIGHT").is_err(),
            ..Default::default()
        })?;
    println!("OK wrap_usdc({amount}): {sig}");
    println!("  new balance ACL {output_balance_acl}");
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

    // Read live token-account state: current balance ACL + next balance/amount nonce sequences.
    // Layout after the 8-byte discriminator: owner(32) mint(32) balance_handle(32)
    // balance_acl_record(32)[104..136] next_balance_nonce_sequence(8)[136..144]
    // next_amount_nonce_sequence(8)[144..152].
    let ta = token.rpc().get_account(&token_account)?;
    let current_balance_acl = Pubkey::new_from_array(ta.data[104..136].try_into().unwrap());
    let bal_seq = u64::from_le_bytes(ta.data[136..144].try_into().unwrap());
    let amt_seq = u64::from_le_bytes(ta.data[144..152].try_into().unwrap());

    // Read live mint state: current total-supply ACL + next total-supply nonce sequence.
    // ConfidentialMint layout incl. 8-byte disc: authority(32)[8..40] acl_domain_key(32)[40..72]
    // compute_signer(32)[72..104] underlying_mint(32)[104..136] decimals(1)[136..137]
    // total_supply_handle(32)[137..169] total_supply_acl_record(32)[169..201]
    // next_total_supply_nonce_sequence(8)[201..209]. (Account body is 201 bytes + 8 disc = 209.)
    let mi = token.rpc().get_account(&mint)?;
    let current_ts_acl = Pubkey::new_from_array(mi.data[169..201].try_into().unwrap());
    let ts_seq = u64::from_le_bytes(mi.data[201..209].try_into().unwrap());

    // 1. create_random_amount(Burn): owner-scoped random burn amount at the amount sequence.
    // BURN_BOUND (a power of two) uses the bounded variant so the random amount stays below the
    // balance and the burn yields a positive burned amount (unbounded randoms exceed the balance,
    // so burn_success is false and the burned amount is 0).
    let burn_label = confidential_token::burn_amount_label();
    let (burn_amount_acl, _) = zama_host::acl_record_address(
        confidential_token::nonce_key(mint, owner, burn_label),
        amt_seq,
    );
    let accounts = confidential_token::accounts::CreateRandomAmount {
        owner,
        mint,
        token_account,
        compute_signer,
        amount_acl_record: burn_amount_acl,
        zama_event_authority: zama_evt,
        zama_program: zama_host::ID,
        host_config,
        system_program: system_program::ID,
        event_authority: token_evt,
        program: confidential_token::ID,
    };
    let send_cfg = anchor_client::RpcSendTransactionConfig {
        skip_preflight: true,
        ..Default::default()
    };
    let sig0 = if let Ok(bound) = std::env::var("BURN_BOUND") {
        let bound: u64 = bound.parse()?;
        let mut upper_bound = [0u8; 32];
        upper_bound[24..].copy_from_slice(&bound.to_be_bytes());
        token
            .request()
            .accounts(accounts)
            .args(confidential_token::instruction::CreateRandomBoundedAmount {
                amount_kind: confidential_token::ConfidentialAmountKind::Burn,
                upper_bound,
            })
            .send_with_spinner_and_config(send_cfg)?
    } else {
        token
            .request()
            .accounts(accounts)
            .args(confidential_token::instruction::CreateRandomAmount {
                amount_kind: confidential_token::ConfidentialAmountKind::Burn,
            })
            .send_with_spinner_and_config(send_cfg)?
    };
    println!("OK create_random_amount(Burn): {sig0}");
    let amt_acct = token.rpc().get_account(&burn_amount_acl)?;
    let amount_handle: [u8; 32] = amt_acct.data[8..40].try_into().unwrap();
    let hh: String = amount_handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("  burn amount ACL {burn_amount_acl}  handle 0x{hh}");

    // 2. Derive the three durable burn output ACLs: the rotated balance + burned amount at
    // bal_seq plus the total-supply output at ts_seq. burn_success and debit_candidate are
    // transient inside confidential_burn's eval frame, so no durable accounts are supplied.
    let bal_nk = confidential_token::balance_nonce_key(mint, token_account);
    let (output_balance_acl, _) = zama_host::acl_record_address(bal_nk, bal_seq);
    let (burned_acl, _) = zama_host::acl_record_address(
        confidential_token::nonce_key(
            mint,
            token_account,
            confidential_token::burned_amount_label(),
        ),
        bal_seq,
    );
    let (ts_output_acl, _) = zama_host::acl_record_address(
        confidential_token::total_supply_nonce_key(mint, ts_authority),
        ts_seq,
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
            current_compute_acl: current_balance_acl,
            current_total_supply_acl: current_ts_acl,
            amount_compute_acl: burn_amount_acl,
            output_acl: output_balance_acl,
            burned_amount_acl: burned_acl,
            total_supply_output_acl: ts_output_acl,
            zama_event_authority: zama_evt,
            zama_program: zama_host::ID,
            host_config,
            system_program: system_program::ID,
            event_authority: token_evt,
            program: confidential_token::ID,
        })
        .args(confidential_token::instruction::ConfidentialBurn { amount_handle })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: std::env::var("BURN_NO_PREFLIGHT").is_err(),
            ..Default::default()
        })?;
    println!("OK confidential_burn: {sig}");
    println!("  burned amount ACL {burned_acl}");
    let burned_acct = token.rpc().get_account(&burned_acl)?;
    let burned_handle: [u8; 32] = burned_acct.data[8..40].try_into().unwrap();
    let bh: String = burned_handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("  burned handle 0x{bh}  (redeem path publicly decrypts this)");
    Ok(())
}

/// Redeem step 1: commit the burned handle's ciphertext material (commit_handle_material, signed
/// by material_authority) then create the burn-redemption request witness
/// (request_burn_redemption), which pins the host's current KMS context id + expires_slot +
/// request_hash into a BurnRedemptionRequest PDA the redeem_burned_amount_secp consume step binds
/// to, and releases the burned handle for public decrypt via the zama-host allow CPI. The material
/// commitment MUST exist before the request, so commit precedes it here. Inputs via env: MINT,
/// UNDERLYING_MINT, BURNED_ACL, BURNED_HANDLE, KEY_ID, CT64_DIGEST, CT128_DIGEST, COPROC_SET_DIGEST;
/// optional REDEEM_SKIP_COMMIT, REQUEST_NONCE, REQUEST_TTL_SLOTS.
fn consume_request_redeem(
    host: &Program<Rc<Keypair>>,
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
    let (material_commitment, _) = zama_host::handle_material_address(burned_acl);
    let nonce = request_nonce_from_env();
    let expires_slot = request_expires_slot(token)?;
    let (redemption_request, _) =
        confidential_token::burn_redemption_request_address(mint, owner, burned_handle, nonce);
    let (zama_evt, _) = Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let (token_evt, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &confidential_token::ID);

    // commit_handle_material for the burned handle (real key_id + ct/sns digests), unless already
    // committed (REDEEM_SKIP_COMMIT). The request validates this commitment, so it must precede it.
    if std::env::var("REDEEM_SKIP_COMMIT").is_err() {
        let key_id: [u8; 32] = hexdec(&std::env::var("KEY_ID")?)
            .try_into()
            .expect("KEY_ID");
        let ct64: [u8; 32] = hexdec(&std::env::var("CT64_DIGEST")?)
            .try_into()
            .expect("CT64_DIGEST");
        let ct128: [u8; 32] = hexdec(&std::env::var("CT128_DIGEST")?)
            .try_into()
            .expect("CT128_DIGEST");
        let coproc: [u8; 32] = hexdec(&std::env::var("COPROC_SET_DIGEST")?)
            .try_into()
            .expect("COPROC_SET_DIGEST");
        let sig = host
            .request()
            .accounts(zama_host::accounts::CommitHandleMaterial {
                payer: owner,
                material_authority: owner,
                host_config,
                acl_record: burned_acl,
                material_commitment,
                system_program: system_program::ID,
                event_authority: zama_evt,
                program: zama_host::ID,
            })
            .args(zama_host::instruction::CommitHandleMaterial {
                key_id,
                ciphertext_digest: ct64,
                sns_ciphertext_digest: ct128,
                coprocessor_set_digest: coproc,
            })
            .send()?;
        println!("OK commit_handle_material (burned): {sig}");
    }

    let sig = token
        .request()
        .accounts(confidential_token::accounts::RequestBurnRedemption {
            owner,
            mint,
            token_account,
            underlying_mint: underlying,
            destination_usdc,
            burned_amount_acl: burned_acl,
            burned_material_commitment: material_commitment,
            redemption_request,
            authority_permission_record: None,
            deny_subject_record: None,
            zama_event_authority: zama_evt,
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
/// MINT, UNDERLYING_MINT, BURNED_ACL, BURNED_HANDLE, CLEARTEXT, KMS_SIG, EXTRA, optional
/// KMS_CTX_ID (default 1), REQUEST_NONCE.
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
    let (material_commitment, _) = zama_host::handle_material_address(burned_acl);
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
            burned_amount_acl: burned_acl,
            burned_material_commitment: material_commitment,
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
        "Add"  => Ok(zama_host::FheBinaryOpCode::Add),
        "Sub"  => Ok(zama_host::FheBinaryOpCode::Sub),
        "Mul"  => Ok(zama_host::FheBinaryOpCode::Mul),
        "Div"  => Ok(zama_host::FheBinaryOpCode::Div),
        "Rem"  => Ok(zama_host::FheBinaryOpCode::Rem),
        "And"  => Ok(zama_host::FheBinaryOpCode::And),
        "Or"   => Ok(zama_host::FheBinaryOpCode::Or),
        "Xor"  => Ok(zama_host::FheBinaryOpCode::Xor),
        "Shl"  => Ok(zama_host::FheBinaryOpCode::Shl),
        "Shr"  => Ok(zama_host::FheBinaryOpCode::Shr),
        "Rotl" => Ok(zama_host::FheBinaryOpCode::Rotl),
        "Rotr" => Ok(zama_host::FheBinaryOpCode::Rotr),
        "Eq"   => Ok(zama_host::FheBinaryOpCode::Eq),
        "Ne"   => Ok(zama_host::FheBinaryOpCode::Ne),
        "Ge"   => Ok(zama_host::FheBinaryOpCode::Ge),
        "Gt"   => Ok(zama_host::FheBinaryOpCode::Gt),
        "Le"   => Ok(zama_host::FheBinaryOpCode::Le),
        "Lt"   => Ok(zama_host::FheBinaryOpCode::Lt),
        "Min"  => Ok(zama_host::FheBinaryOpCode::Min),
        "Max"  => Ok(zama_host::FheBinaryOpCode::Max),
        _ => Err(format!("unknown binary op: {s}")),
    }
}

fn parse_unary_op(s: &str) -> Result<zama_host::FheUnaryOpCode, String> {
    match s {
        "Neg"  => Ok(zama_host::FheUnaryOpCode::Neg),
        "Not"  => Ok(zama_host::FheUnaryOpCode::Not),
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

/// Generic binary fhe_eval: TrivialEncrypt(lhs) [+ TrivialEncrypt(rhs)] → Binary(op, AllowedDurable).
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
    let a: u64 = std::env::var("BINARY_A").ok().and_then(|s| s.parse().ok()).unwrap_or(10);
    let b: u64 = std::env::var("BINARY_B").ok().and_then(|s| s.parse().ok()).unwrap_or(5);
    let b_scalar = std::env::var("BINARY_B_SCALAR").is_ok();
    let in_fhe_type: u8 = std::env::var("BINARY_FHE_TYPE").ok().and_then(|s| s.parse().ok()).unwrap_or(5);
    let output_fhe_type: u8 = if is_comparison_op(op) { 0 } else { in_fhe_type };

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let mut encrypted_value_label = [7u8; 32];
    encrypted_value_label[1] = op.as_u8();
    encrypted_value_label[2] = in_fhe_type;
    encrypted_value_label[3] = if b_scalar { 1 } else { 0 };
    encrypted_value_label[8..16].copy_from_slice(&a.to_be_bytes());
    encrypted_value_label[16..24].copy_from_slice(&b.to_be_bytes());
    let output_nonce_key =
        zama_host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_nonce_sequence: u64 = 0;
    let (output_acl_record, _) =
        zama_host::acl_record_address(output_nonce_key, output_nonce_sequence);
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

    let mut plaintext_a = [0u8; 32];
    plaintext_a[24..32].copy_from_slice(&a.to_be_bytes());

    let (rhs_operand, b_step) = if b_scalar {
        let mut scalar_b = [0u8; 32];
        scalar_b[24..32].copy_from_slice(&b.to_be_bytes());
        (zama_host::FheEvalOperand::Scalar(scalar_b), None)
    } else {
        let mut plaintext_b = [0u8; 32];
        plaintext_b[24..32].copy_from_slice(&b.to_be_bytes());
        (
            zama_host::FheEvalOperand::AllowedLocal { producer_index: 1 },
            Some(zama_host::FheEvalStep::TrivialEncrypt {
                plaintext: plaintext_b,
                fhe_type: in_fhe_type,
                output: zama_host::FheEvalOutput::AllowedLocal,
            }),
        )
    };

    let mut steps = vec![zama_host::FheEvalStep::TrivialEncrypt {
        plaintext: plaintext_a,
        fhe_type: in_fhe_type,
        output: zama_host::FheEvalOutput::AllowedLocal,
    }];
    if let Some(step) = b_step {
        steps.push(step);
    }
    steps.push(zama_host::FheEvalStep::Binary {
        op,
        lhs: zama_host::FheEvalOperand::AllowedLocal { producer_index: 0 },
        rhs: rhs_operand,
        output_fhe_type,
        output: zama_host::FheEvalOutput::AllowedDurable {
            output_acl_record_index: 0,
            output_app_account_authority_index: None,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: subjects,
            output_public_decrypt: false,
        },
    });

    let b_desc = if b_scalar { format!("scalar({b})") } else { format!("enc({b})") };
    let sig = host
        .request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(vec![AccountMeta::new(output_acl_record, false)])
        .args(zama_host::instruction::FheEval { args: zama_host::FheEvalArgs { context_id, steps } })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval binary {op_name}(enc({a}), {b_desc}) fhe_type={in_fhe_type} out_type={output_fhe_type}: {sig}");

    let record: zama_host::AclRecord = host.account(output_acl_record)?;
    let handle_hex: String = record.handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("  output ACL record {output_acl_record}");
    println!("  result handle 0x{handle_hex}");

    if std::env::var("BINARY_ALLOW").is_ok() {
        let allow_sig = host
            .request()
            .accounts(zama_host::accounts::AllowForDecryption {
                authority: payer.pubkey(),
                authority_permission_record: None,
                acl_record: output_acl_record,
                host_config,
                deny_subject_record: None,
                event_authority: zama_event_authority,
                program: zama_host::ID,
            })
            .args(zama_host::instruction::AllowForDecryption {
                handle: record.handle,
            })
            .send()?;
        println!("OK allow_for_decryption (public): {allow_sig}");
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
    let a: u64 = std::env::var("UNARY_A").ok().and_then(|s| s.parse().ok()).unwrap_or(42);
    let in_fhe_type: u8 = std::env::var("UNARY_IN_FHE_TYPE").ok().and_then(|s| s.parse().ok()).unwrap_or(2);
    let out_fhe_type: u8 = std::env::var("UNARY_OUT_FHE_TYPE").ok().and_then(|s| s.parse().ok()).unwrap_or(in_fhe_type);

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let mut encrypted_value_label = [8u8; 32];
    encrypted_value_label[1] = op.as_u8();
    encrypted_value_label[2] = in_fhe_type;
    encrypted_value_label[3] = out_fhe_type;
    encrypted_value_label[24..32].copy_from_slice(&a.to_be_bytes());
    let output_nonce_key =
        zama_host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_nonce_sequence: u64 = 0;
    let (output_acl_record, _) =
        zama_host::acl_record_address(output_nonce_key, output_nonce_sequence);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];

    let mut context_id = [0u8; 32];
    context_id[0] = 0xb1;
    context_id[1] = op.as_u8();
    context_id[2] = in_fhe_type;
    context_id[3] = out_fhe_type;
    context_id[24..32].copy_from_slice(&a.to_be_bytes());

    let mut plaintext_a = [0u8; 32];
    plaintext_a[24..32].copy_from_slice(&a.to_be_bytes());

    let args = zama_host::FheEvalArgs {
        context_id,
        steps: vec![
            zama_host::FheEvalStep::TrivialEncrypt {
                plaintext: plaintext_a,
                fhe_type: in_fhe_type,
                output: zama_host::FheEvalOutput::AllowedLocal,
            },
            zama_host::FheEvalStep::Unary {
                op,
                operand: zama_host::FheEvalOperand::AllowedLocal { producer_index: 0 },
                output_fhe_type: out_fhe_type,
                output: zama_host::FheEvalOutput::AllowedDurable {
                    output_acl_record_index: 0,
                    output_app_account_authority_index: None,
                    output_nonce_key,
                    output_nonce_sequence,
                    output_acl_domain_key: acl_domain_key,
                    output_app_account: app_account,
                    output_encrypted_value_label: encrypted_value_label,
                    output_subjects: subjects,
                    output_public_decrypt: false,
                },
            },
        ],
    };

    let sig = host
        .request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(vec![AccountMeta::new(output_acl_record, false)])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval unary {op_name}(enc({a})) in_type={in_fhe_type} out_type={out_fhe_type}: {sig}");

    let record: zama_host::AclRecord = host.account(output_acl_record)?;
    let handle_hex: String = record.handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("  output ACL record {output_acl_record}");
    println!("  result handle 0x{handle_hex}");

    if std::env::var("UNARY_ALLOW").is_ok() {
        let allow_sig = host
            .request()
            .accounts(zama_host::accounts::AllowForDecryption {
                authority: payer.pubkey(),
                authority_permission_record: None,
                acl_record: output_acl_record,
                host_config,
                deny_subject_record: None,
                event_authority: zama_event_authority,
                program: zama_host::ID,
            })
            .args(zama_host::instruction::AllowForDecryption {
                handle: record.handle,
            })
            .send()?;
        println!("OK allow_for_decryption (public): {allow_sig}");
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

    let ctrl: u64 = std::env::var("TERNARY_CTRL").ok().and_then(|s| s.parse().ok()).unwrap_or(1);
    let if_true: u64 = std::env::var("TERNARY_TRUE").ok().and_then(|s| s.parse().ok()).unwrap_or(42);
    let if_false: u64 = std::env::var("TERNARY_FALSE").ok().and_then(|s| s.parse().ok()).unwrap_or(99);
    let fhe_type: u8 = std::env::var("TERNARY_FHE_TYPE").ok().and_then(|s| s.parse().ok()).unwrap_or(5);

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let mut encrypted_value_label = [9u8; 32];
    encrypted_value_label[1] = ctrl as u8;
    encrypted_value_label[2] = fhe_type;
    encrypted_value_label[8..16].copy_from_slice(&if_true.to_be_bytes());
    encrypted_value_label[16..24].copy_from_slice(&if_false.to_be_bytes());
    let output_nonce_key =
        zama_host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_nonce_sequence: u64 = 0;
    let (output_acl_record, _) =
        zama_host::acl_record_address(output_nonce_key, output_nonce_sequence);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];

    let mut context_id = [0u8; 32];
    context_id[0] = 0xb2;
    context_id[1] = ctrl as u8;
    context_id[2] = fhe_type;
    context_id[8..16].copy_from_slice(&if_true.to_be_bytes());
    context_id[16..24].copy_from_slice(&if_false.to_be_bytes());

    let mut pt_ctrl = [0u8; 32];
    pt_ctrl[31] = ctrl as u8;
    let mut pt_true = [0u8; 32];
    pt_true[24..32].copy_from_slice(&if_true.to_be_bytes());
    let mut pt_false = [0u8; 32];
    pt_false[24..32].copy_from_slice(&if_false.to_be_bytes());

    let expected = if ctrl != 0 { if_true } else { if_false };
    let args = zama_host::FheEvalArgs {
        context_id,
        steps: vec![
            zama_host::FheEvalStep::TrivialEncrypt {
                plaintext: pt_ctrl,
                fhe_type: 0, // ebool
                output: zama_host::FheEvalOutput::AllowedLocal,
            },
            zama_host::FheEvalStep::TrivialEncrypt {
                plaintext: pt_true,
                fhe_type,
                output: zama_host::FheEvalOutput::AllowedLocal,
            },
            zama_host::FheEvalStep::TrivialEncrypt {
                plaintext: pt_false,
                fhe_type,
                output: zama_host::FheEvalOutput::AllowedLocal,
            },
            zama_host::FheEvalStep::Ternary {
                op: zama_host::FheTernaryOpCode::IfThenElse,
                control: zama_host::FheEvalOperand::AllowedLocal { producer_index: 0 },
                if_true: zama_host::FheEvalOperand::AllowedLocal { producer_index: 1 },
                if_false: zama_host::FheEvalOperand::AllowedLocal { producer_index: 2 },
                output_fhe_type: fhe_type,
                output: zama_host::FheEvalOutput::AllowedDurable {
                    output_acl_record_index: 0,
                    output_app_account_authority_index: None,
                    output_nonce_key,
                    output_nonce_sequence,
                    output_acl_domain_key: acl_domain_key,
                    output_app_account: app_account,
                    output_encrypted_value_label: encrypted_value_label,
                    output_subjects: subjects,
                    output_public_decrypt: false,
                },
            },
        ],
    };

    let sig = host
        .request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(vec![AccountMeta::new(output_acl_record, false)])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval ternary select(ctrl={ctrl}, true={if_true}, false={if_false}) -> {expected}: {sig}");

    let record: zama_host::AclRecord = host.account(output_acl_record)?;
    let handle_hex: String = record.handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("  output ACL record {output_acl_record}");
    println!("  result handle 0x{handle_hex}  (expected cleartext={expected})");

    if std::env::var("TERNARY_ALLOW").is_ok() {
        let allow_sig = host
            .request()
            .accounts(zama_host::accounts::AllowForDecryption {
                authority: payer.pubkey(),
                authority_permission_record: None,
                acl_record: output_acl_record,
                host_config,
                deny_subject_record: None,
                event_authority: zama_event_authority,
                program: zama_host::ID,
            })
            .args(zama_host::instruction::AllowForDecryption {
                handle: record.handle,
            })
            .send()?;
        println!("OK allow_for_decryption (public): {allow_sig}");
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

    let upper: u64 = std::env::var("RAND_UPPER").ok().and_then(|s| s.parse().ok()).unwrap_or(100);
    let fhe_type: u8 = std::env::var("RAND_FHE_TYPE").ok().and_then(|s| s.parse().ok()).unwrap_or(5);

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let mut encrypted_value_label = [0x0au8; 32];
    encrypted_value_label[1] = fhe_type;
    encrypted_value_label[24..32].copy_from_slice(&upper.to_be_bytes());
    let output_nonce_key =
        zama_host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_nonce_sequence: u64 = 0;
    let (output_acl_record, _) =
        zama_host::acl_record_address(output_nonce_key, output_nonce_sequence);
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
            output: zama_host::FheEvalOutput::AllowedDurable {
                output_acl_record_index: 0,
                output_app_account_authority_index: None,
                output_nonce_key,
                output_nonce_sequence,
                output_acl_domain_key: acl_domain_key,
                output_app_account: app_account,
                output_encrypted_value_label: encrypted_value_label,
                output_subjects: subjects,
                output_public_decrypt: false,
            },
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
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(vec![AccountMeta::new(output_acl_record, false)])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval rand_bounded(upper={upper}) fhe_type={fhe_type}: {sig}");

    let record: zama_host::AclRecord = host.account(output_acl_record)?;
    let handle_hex: String = record.handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("  output ACL record {output_acl_record}");
    println!("  result handle 0x{handle_hex}  (expected cleartext in [0, {upper}))");

    if std::env::var("RAND_ALLOW").is_ok() {
        let allow_sig = host
            .request()
            .accounts(zama_host::accounts::AllowForDecryption {
                authority: payer.pubkey(),
                authority_permission_record: None,
                acl_record: output_acl_record,
                host_config,
                deny_subject_record: None,
                event_authority: zama_event_authority,
                program: zama_host::ID,
            })
            .args(zama_host::instruction::AllowForDecryption {
                handle: record.handle,
            })
            .send()?;
        println!("OK allow_for_decryption (public): {allow_sig}");
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
    let a: u64 = std::env::var("SUM_A").ok().and_then(|s| s.parse().ok()).unwrap_or(10);
    let b: u64 = std::env::var("SUM_B").ok().and_then(|s| s.parse().ok()).unwrap_or(20);
    let fhe_type: u8 = 5; // euint64

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let mut encrypted_value_label = [4u8; 32];
    encrypted_value_label[16..24].copy_from_slice(&a.to_be_bytes());
    encrypted_value_label[24..32].copy_from_slice(&b.to_be_bytes());
    let output_nonce_key =
        zama_host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_nonce_sequence: u64 = 0;
    let (output_acl_record, _) =
        zama_host::acl_record_address(output_nonce_key, output_nonce_sequence);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];

    let mut context_id = [0u8; 32];
    context_id[0] = 0xa0;
    context_id[16..24].copy_from_slice(&a.to_be_bytes());
    context_id[24..32].copy_from_slice(&b.to_be_bytes());

    let mut plaintext_a = [0u8; 32];
    plaintext_a[24..32].copy_from_slice(&a.to_be_bytes());
    let mut plaintext_b = [0u8; 32];
    plaintext_b[24..32].copy_from_slice(&b.to_be_bytes());

    let args = zama_host::FheEvalArgs {
        context_id,
        steps: vec![
            zama_host::FheEvalStep::TrivialEncrypt {
                plaintext: plaintext_a,
                fhe_type,
                output: zama_host::FheEvalOutput::AllowedLocal,
            },
            zama_host::FheEvalStep::TrivialEncrypt {
                plaintext: plaintext_b,
                fhe_type,
                output: zama_host::FheEvalOutput::AllowedLocal,
            },
            zama_host::FheEvalStep::Sum {
                operands: vec![
                    zama_host::FheEvalOperand::AllowedLocal { producer_index: 0 },
                    zama_host::FheEvalOperand::AllowedLocal { producer_index: 1 },
                ],
                fhe_type,
                output: zama_host::FheEvalOutput::AllowedDurable {
                    output_acl_record_index: 0,
                    output_app_account_authority_index: None,
                    output_nonce_key,
                    output_nonce_sequence,
                    output_acl_domain_key: acl_domain_key,
                    output_app_account: app_account,
                    output_encrypted_value_label: encrypted_value_label,
                    output_subjects: subjects,
                    output_public_decrypt: false,
                },
            },
        ],
    };

    let sig = host
        .request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(vec![AccountMeta::new(output_acl_record, false)])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval sum({a} + {b} as euint64): {sig}");

    let record: zama_host::AclRecord = host.account(output_acl_record)?;
    let handle_hex: String = record.handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("  output ACL record {output_acl_record}");
    println!("  result handle 0x{handle_hex}  (tfhe-worker materializes sum({a} + {b}))");

    if std::env::var("SUM_ALLOW").is_ok() {
        let allow_sig = host
            .request()
            .accounts(zama_host::accounts::AllowForDecryption {
                authority: payer.pubkey(),
                authority_permission_record: None,
                acl_record: output_acl_record,
                host_config,
                deny_subject_record: None,
                event_authority: zama_event_authority,
                program: zama_host::ID,
            })
            .args(zama_host::instruction::AllowForDecryption {
                handle: record.handle,
            })
            .send()?;
        println!("OK allow_for_decryption (public): {allow_sig}");
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
    let output_nonce_key =
        zama_host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_nonce_sequence: u64 = 0;
    let (output_acl_record, _) =
        zama_host::acl_record_address(output_nonce_key, output_nonce_sequence);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];

    let mut context_id = [0u8; 32];
    context_id[0] = 0xa1;
    context_id[24..32].copy_from_slice(&value.to_be_bytes());

    let mut plaintext_value = [0u8; 32];
    plaintext_value[24..32].copy_from_slice(&value.to_be_bytes());
    let set_values: [u64; 3] = [10, 42, 100];
    let plaintext_set: Vec<[u8; 32]> = set_values
        .iter()
        .map(|&v| {
            let mut p = [0u8; 32];
            p[24..32].copy_from_slice(&v.to_be_bytes());
            p
        })
        .collect();

    // Steps: TE(value) at producer 0, TE(set[0..2]) at producers 1..3, IsIn at producer 4.
    let mut steps = vec![zama_host::FheEvalStep::TrivialEncrypt {
        plaintext: plaintext_value,
        fhe_type: elem_fhe_type,
        output: zama_host::FheEvalOutput::AllowedLocal,
    }];
    for pt in &plaintext_set {
        steps.push(zama_host::FheEvalStep::TrivialEncrypt {
            plaintext: *pt,
            fhe_type: elem_fhe_type,
            output: zama_host::FheEvalOutput::AllowedLocal,
        });
    }
    let set_operands: Vec<zama_host::FheEvalOperand> = (1u16..=3)
        .map(|i| zama_host::FheEvalOperand::AllowedLocal { producer_index: i })
        .collect();
    steps.push(zama_host::FheEvalStep::IsIn {
        value: zama_host::FheEvalOperand::AllowedLocal { producer_index: 0 },
        set: set_operands,
        fhe_type: elem_fhe_type,
        output: zama_host::FheEvalOutput::AllowedDurable {
            output_acl_record_index: 0,
            output_app_account_authority_index: None,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: subjects,
            output_public_decrypt: false,
        },
    });

    let args = zama_host::FheEvalArgs { context_id, steps };
    let in_set = set_values.contains(&value);

    let sig = host
        .request()
        .accounts(zama_host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(vec![AccountMeta::new(output_acl_record, false)])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval isIn({value} in {set_values:?} as euint64): {sig}");

    let record: zama_host::AclRecord = host.account(output_acl_record)?;
    let handle_hex: String = record.handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("  output ACL record {output_acl_record}");
    println!("  result handle 0x{handle_hex}  (ebool; tfhe-worker materializes isIn -> {in_set})");

    if std::env::var("ISIN_ALLOW").is_ok() {
        let allow_sig = host
            .request()
            .accounts(zama_host::accounts::AllowForDecryption {
                authority: payer.pubkey(),
                authority_permission_record: None,
                acl_record: output_acl_record,
                host_config,
                deny_subject_record: None,
                event_authority: zama_event_authority,
                program: zama_host::ID,
            })
            .args(zama_host::instruction::AllowForDecryption {
                handle: record.handle,
            })
            .send()?;
        println!("OK allow_for_decryption (public): {allow_sig}");
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
    let a: u64 = std::env::var("MULDIV_A").ok().and_then(|s| s.parse().ok()).unwrap_or(6);
    let b: u64 = std::env::var("MULDIV_B").ok().and_then(|s| s.parse().ok()).unwrap_or(7);
    let d: u64 = std::env::var("MULDIV_D").ok().and_then(|s| s.parse().ok()).unwrap_or(3);
    let fhe_type: u8 = 5; // euint64

    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let mut encrypted_value_label = [6u8; 32];
    encrypted_value_label[8..16].copy_from_slice(&a.to_be_bytes());
    encrypted_value_label[16..24].copy_from_slice(&b.to_be_bytes());
    encrypted_value_label[24..32].copy_from_slice(&d.to_be_bytes());
    let output_nonce_key =
        zama_host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_nonce_sequence: u64 = 0;
    let (output_acl_record, _) =
        zama_host::acl_record_address(output_nonce_key, output_nonce_sequence);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![zama_host::AclSubjectEntry::user(payer.pubkey())];

    let mut context_id = [0u8; 32];
    context_id[0] = 0xa2;
    context_id[8..16].copy_from_slice(&a.to_be_bytes());
    context_id[16..24].copy_from_slice(&b.to_be_bytes());
    context_id[24..32].copy_from_slice(&d.to_be_bytes());

    let mut plaintext_a = [0u8; 32];
    plaintext_a[24..32].copy_from_slice(&a.to_be_bytes());
    let mut scalar_b = [0u8; 32];
    scalar_b[24..32].copy_from_slice(&b.to_be_bytes());
    let mut divisor = [0u8; 32];
    divisor[24..32].copy_from_slice(&d.to_be_bytes());

    let args = zama_host::FheEvalArgs {
        context_id,
        steps: vec![
            zama_host::FheEvalStep::TrivialEncrypt {
                plaintext: plaintext_a,
                fhe_type,
                output: zama_host::FheEvalOutput::AllowedLocal,
            },
            zama_host::FheEvalStep::MulDiv {
                factor1: zama_host::FheEvalOperand::AllowedLocal { producer_index: 0 },
                factor2: zama_host::FheEvalOperand::Scalar(scalar_b),
                divisor,
                output_fhe_type: fhe_type,
                output: zama_host::FheEvalOutput::AllowedDurable {
                    output_acl_record_index: 0,
                    output_app_account_authority_index: None,
                    output_nonce_key,
                    output_nonce_sequence,
                    output_acl_domain_key: acl_domain_key,
                    output_app_account: app_account,
                    output_encrypted_value_label: encrypted_value_label,
                    output_subjects: subjects,
                    output_public_decrypt: false,
                },
            },
        ],
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
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .accounts(vec![AccountMeta::new(output_acl_record, false)])
        .args(zama_host::instruction::FheEval { args })
        .send_with_spinner_and_config(anchor_client::RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;
    println!("OK fhe_eval mulDiv({a} * {b} / {d} = {expected} as euint64): {sig}");

    let record: zama_host::AclRecord = host.account(output_acl_record)?;
    let handle_hex: String = record.handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("  output ACL record {output_acl_record}");
    println!("  result handle 0x{handle_hex}  (tfhe-worker materializes mulDiv -> {expected})");

    if std::env::var("MULDIV_ALLOW").is_ok() {
        let allow_sig = host
            .request()
            .accounts(zama_host::accounts::AllowForDecryption {
                authority: payer.pubkey(),
                authority_permission_record: None,
                acl_record: output_acl_record,
                host_config,
                deny_subject_record: None,
                event_authority: zama_event_authority,
                program: zama_host::ID,
            })
            .args(zama_host::instruction::AllowForDecryption {
                handle: record.handle,
            })
            .send()?;
        println!("OK allow_for_decryption (public): {allow_sig}");
    }
    Ok(())
}

/// Creates a confidential mint wrapping $UNDERLYING_MINT. Exercises the host<->token
/// CPI that trivial-encrypts the initial total-supply handle and creates its ACL record.
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
    let (total_supply_acl_record, _) = zama_host::acl_record_address(
        confidential_token::total_supply_nonce_key(mint_pk, total_supply_authority),
        0,
    );
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
            total_supply_acl_record,
            zama_event_authority,
            zama_program: zama_host::ID,
            host_config,
            system_program: system_program::ID,
            event_authority: token_event_authority,
            program: confidential_token::ID,
        })
        .args(confidential_token::instruction::InitializeMint {})
        .signer(&mint)
        .send()?;
    println!("OK initialize_mint: {sig}");
    println!("  confidential mint  {mint_pk}");
    println!("  underlying SPL     {underlying_mint}");
    println!("  total_supply ACL   {total_supply_acl_record}");

    let acl = token.rpc().get_account(&total_supply_acl_record)?;
    println!(
        "  ACL record owner={} bytes={}  (created via host CPI)",
        acl.owner,
        acl.data.len()
    );
    Ok(())
}
