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
    let client =
        Client::new_with_options(cluster, payer.clone(), CommitmentConfig::confirmed());
    let host = client.program(zama_host::ID)?;
    let token = client.program(confidential_token::ID)?;

    let (host_config, _) =
        Pubkey::find_program_address(&[zama_host::HOST_CONFIG_SEED], &zama_host::ID);

    // BIND_INPUT drives the coprocessor-attested input bind against the live host_config
    // (created by bootstrap.mjs with the real gateway verifier config); it does not touch
    // host_config or the mint, so it runs standalone with the relayer-returned attestation.
    if std::env::var("BIND_INPUT").is_ok() {
        bind_coprocessor_input(&host, &payer, host_config)?;
        return Ok(());
    }

    // TRIVIAL_ENCRYPT drives a real zama-host FHE op (trivial encryption): the program
    // computes the result handle on-chain and emits a TrivialEncryptEvent the live
    // host-listener ingests into the coprocessor DB for the tfhe-worker to materialize.
    if std::env::var("TRIVIAL_ENCRYPT").is_ok() {
        trivial_encrypt_and_bind(&host, &payer, host_config)?;
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

/// Binds a coprocessor-verified input on the live zama-host: feeds the handle + EIP-712
/// `CiphertextVerification` attestation (handles + signature) the relayer returned, which the
/// host verifies on-chain via secp256k1_recover against the configured coprocessor signer
/// before creating the output ACL record. Inputs come from the relayer response via env
/// (BIND_HANDLE, BIND_COPRO_SIG, BIND_USER, BIND_CONTRACT, BIND_CHAIN_ID, optional BIND_EXTRA).
fn bind_coprocessor_input(
    host: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
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

    // Output ACL record: app_account must equal the authorizing signer, and nonce_key must
    // equal acl_nonce_key(domain, app_account, label). The bound input is granted USE to the
    // user subject.
    let app_account = payer.pubkey();
    let acl_domain_key = payer.pubkey();
    let encrypted_value_label = [0u8; 32];
    let output_nonce_key =
        zama_host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_nonce_sequence: u64 = 0;
    let (output_acl_record, _) =
        zama_host::acl_record_address(output_nonce_key, output_nonce_sequence);
    let (zama_event_authority, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let subjects = vec![zama_host::AclSubjectEntry::use_only(payer.pubkey())];

    let handle_hex: String = handle.iter().map(|b| format!("{b:02x}")).collect();
    println!("binding input_handle 0x{handle_hex}");
    let sig = host
        .request()
        .accounts(zama_host::accounts::VerifyCoprocessorInputAndBind {
            payer: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: zama_event_authority,
            program: zama_host::ID,
        })
        .args(zama_host::instruction::VerifyCoprocessorInputAndBind {
            input_handle: handle,
            ct_handles: vec![handle],
            handle_index: 0,
            user_address,
            contract_address,
            contract_chain_id,
            extra_data,
            signatures: vec![signature],
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: subjects,
            output_public_decrypt: false,
        })
        .send()?;
    println!("OK verify_coprocessor_input_and_bind: {sig}");
    println!("  output ACL record {output_acl_record}  (secp256k1 attestation verified on-chain)");
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

/// Consume step 1: seal the token amount's ciphertext material on-chain (commit_handle_material,
/// signed by the configured material_authority, with the real ct64/ct128 digests) and request
/// public disclosure (request_disclose_amount → CPIs zama-host allow_public_decrypt). Inputs via
/// env: MINT, TS_ACL, TS_HANDLE, KEY_ID, CT64_DIGEST, CT128_DIGEST, COPROC_SET_DIGEST.
fn consume_seal(
    host: &Program<Rc<Keypair>>,
    token: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let mint = Pubkey::from_str(&std::env::var("MINT")?)?;
    let ts_acl = Pubkey::from_str(&std::env::var("TS_ACL")?)?;
    let handle: [u8; 32] = hexdec(&std::env::var("TS_HANDLE")?).try_into().expect("TS_HANDLE");
    let key_id: [u8; 32] = hexdec(&std::env::var("KEY_ID")?).try_into().expect("KEY_ID");
    let ct64: [u8; 32] = hexdec(&std::env::var("CT64_DIGEST")?).try_into().expect("CT64_DIGEST");
    let ct128: [u8; 32] = hexdec(&std::env::var("CT128_DIGEST")?).try_into().expect("CT128_DIGEST");
    let coproc: [u8; 32] =
        hexdec(&std::env::var("COPROC_SET_DIGEST")?).try_into().expect("COPROC_SET_DIGEST");
    let (material_commitment, _) = zama_host::handle_material_address(ts_acl);
    let (zama_evt, _) = Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);

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

    let (token_evt, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &confidential_token::ID);
    let sig2 = token
        .request()
        .accounts(confidential_token::accounts::RequestDiscloseAmount {
            requester: payer.pubkey(),
            mint,
            amount_acl_record: ts_acl,
            authority_permission_record: None,
            deny_subject_record: None,
            zama_event_authority: zama_evt,
            zama_program: zama_host::ID,
            host_config,
            event_authority: token_evt,
            program: confidential_token::ID,
        })
        .args(confidential_token::instruction::RequestDiscloseAmount { amount_handle: handle })
        .send()?;
    println!("OK request_disclose_amount: {sig2}  (handle released for public decrypt)");
    Ok(())
}

/// Consume step 2: publish a KMS-certified cleartext by verifying the KMS PublicDecryptVerification
/// EIP-712 cert on-chain via secp256k1 (disclose_amount_secp) against the active KMS context's
/// ProtocolConfig-mirrored signer set. Inputs via env: MINT, TS_ACL, TS_HANDLE, CLEARTEXT, KMS_SIG,
/// EXTRA, KMS_CTX_ID.
fn consume_disclose(
    token: &Program<Rc<Keypair>>,
    _payer: &Rc<Keypair>,
    host_config: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let mint = Pubkey::from_str(&std::env::var("MINT")?)?;
    let ts_acl = Pubkey::from_str(&std::env::var("TS_ACL")?)?;
    let handle: [u8; 32] = hexdec(&std::env::var("TS_HANDLE")?).try_into().expect("TS_HANDLE");
    let cleartext: u64 = std::env::var("CLEARTEXT")?.parse()?;
    let kms_sig: [u8; 65] = hexdec(&std::env::var("KMS_SIG")?).try_into().expect("KMS_SIG 65 bytes");
    let extra = hexdec(&std::env::var("EXTRA")?);
    let ctx_id: u64 = std::env::var("KMS_CTX_ID").ok().and_then(|s| s.parse().ok()).unwrap_or(1);
    let (material_commitment, _) = zama_host::handle_material_address(ts_acl);
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
    println!("  KMS PublicDecryptVerification cert verified on-chain (secp256k1); cleartext {cleartext}");
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
    let amount: u64 = std::env::var("WRAP_AMOUNT").ok().and_then(|s| s.parse().ok()).unwrap_or(100);
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
    // The wrap amount is trivial-encrypted with app_account = token_account at the balance
    // sequence (1), per wrap_usdc's fhe::trivial_encrypt_u64 output_nonce_key/sequence.
    let amt_seq: u64 = std::env::var("WRAP_AMOUNT_SEQ").ok().and_then(|s| s.parse().ok()).unwrap_or(1);
    let (wrap_amount_acl, _) = zama_host::acl_record_address(
        confidential_token::nonce_key(mint, token_account, confidential_token::wrap_amount_label()),
        amt_seq,
    );
    let (zama_evt, _) = Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &zama_host::ID);
    let (token_evt, _) =
        Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &confidential_token::ID);

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
    let heap_ix = Instruction { program_id: cb_prog, accounts: vec![], data: heap_data };
    let mut cu_data = vec![2u8];
    cu_data.extend_from_slice(&1_400_000u32.to_le_bytes());
    let cu_ix = Instruction { program_id: cb_prog, accounts: vec![], data: cu_data };

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
            amount_compute_acl: wrap_amount_acl,
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
            kms_verifier_authority: payer.pubkey(),
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
