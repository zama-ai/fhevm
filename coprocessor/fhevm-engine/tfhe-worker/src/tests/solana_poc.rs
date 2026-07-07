use std::path::PathBuf;

use anchor_lang::{
    prelude::{system_instruction, system_program},
    AccountDeserialize, AccountSerialize, InstructionData, ToAccountMetas,
};
use anchor_spl::token::spl_token;
use fhevm_engine_common::{
    tfhe_ops::current_ciphertext_version,
    types::{AllowEvents, SupportedFheCiphertexts},
};
use host_listener::{
    contracts::TfheContract::TfheContractEvents,
    database::tfhe_event_propagate::Handle,
    database::tfhe_event_propagate::{tfhe_inputs_handle, tfhe_result_handle},
    generated::{
        FheBinaryOpCode as SolanaFheBinaryOpCode, FheBinaryOpEvent, FheRandBoundedEvent,
        FheTernaryOpCode as SolanaFheTernaryOpCode, FheTernaryOpEvent, TrivialEncryptEvent,
        EVENT_VERSION,
    },
    solana_adapter::{
        decode_anchor_cpi_events, decode_anchor_log_events, decode_solana_transaction_events,
        insert_solana_events, normalize_solana_events_for_db, solana_transaction_id,
        SolanaAclAllowedEvent, SolanaBlockMeta, SolanaFinalizedAccountFetch,
        SolanaFinalizedAccountFetchKind, SolanaHostEvent,
    },
};
use litesvm::{types::TransactionMetadata, LiteSVM};
use serial_test::serial;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    message::{Message, VersionedMessage},
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    transaction::VersionedTransaction,
};
use tfhe::prelude::FheTryEncrypt;
use time::{Date, Month, PrimitiveDateTime, Time};
use zama_host::{
    AclRecord, AclSubjectEntry, FheEvalArgs, FheEvalOperand, FheEvalOutput, FheEvalStep, HostConfig,
};

use crate::tests::{
    event_helpers::{decrypt_handles, setup_event_harness, wait_until_computed},
    utils::latest_db_key,
};

use confidential_token as token;
use zama_host as host;

// Small "fast" euint8 type used by the standalone fhe-rand round-trip.
const FAST_REAL_FHE_TYPE: u8 = 2;
// Must equal the token program's BALANCE_FHE_TYPE: confidential balances and
// transfer amounts are euint64 (type 5); the program rejects other handle types.
const BALANCE_FHE_TYPE: u8 = 5;
const TOKEN_BALANCE_FHE_TYPE: u8 = 5;
type SeededCiphertext = ([u8; 32], i16, Vec<u8>);
const DEFAULT_SOLANA_LOG_BYTES_LIMIT: usize = 10_000;
// Coprocessor `CiphertextVerification` EIP-712 domain used by the `fromExternal` transfer amount
// attestations; must match the fixture's `host_config` verifier settings.
const SECP_GATEWAY_CHAIN_ID: u64 = 31337;
const INPUT_VERIFICATION_CONTRACT: [u8; 20] = [0xCDu8; 20];

#[tokio::test]
#[serial(db)]
#[ignore = "runs LiteSVM plus the real TFHE worker against a disposable Postgres DB"]
async fn solana_confidential_transfer_with_real_ciphertexts_computes_and_decrypts(
) -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let mut fixture = token_fixture();

    let amount_handle = balance_handle(0x09);

    seed_real_fast_ciphertexts(
        &harness.pool,
        &[
            (fixture.alice_initial, 125),
            (fixture.bob_initial, 20),
            (amount_handle, 100),
        ],
    )
    .await?;

    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, amount_handle);
    let (meta, account_keys, signature) =
        send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    let new_alice_handle = read_acl_record(&fixture.svm, output.alice)
        .expect("expected Alice output ACL")
        .handle;
    let new_bob_handle = read_acl_record(&fixture.svm, output.bob)
        .expect("expected Bob output ACL")
        .handle;
    let host_events = host_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(count_tfhe_events(&host_events), 5);
    assert_eq!(count_acl_events(&host_events), 7);

    let transaction_id = solana_transaction_id(signature.as_ref());
    let block = SolanaBlockMeta {
        block_number: 1,
        block_timestamp: PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 11)?,
            Time::MIDNIGHT,
        ),
    };
    let mut db_tx = harness.listener_db.new_transaction().await?;
    let stats = insert_solana_events(
        &harness.listener_db,
        &mut db_tx,
        host_events,
        transaction_id,
        block,
    )
    .await?;
    db_tx.commit().await?;

    assert_eq!(stats.tfhe_events, 5);
    assert_eq!(stats.acl_events, 3);

    wait_until_computed(&harness.app).await?;
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &signed_user_decrypt_request(
            &fixture.alice,
            vec![fixture.mint.pubkey()],
            vec![UserDecryptHandleEntry {
                handle: new_alice_handle,
                owner: fixture.alice.pubkey(),
                acl_record: output.alice,
            }],
        ),
    ));
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &signed_user_decrypt_request(
            &fixture.bob,
            vec![fixture.mint.pubkey()],
            vec![UserDecryptHandleEntry {
                handle: new_bob_handle,
                owner: fixture.bob.pubkey(),
                acl_record: output.bob,
            }],
        ),
    ));
    let decrypted = decrypt_handles(
        &harness.pool,
        &[Handle::from(new_alice_handle), Handle::from(new_bob_handle)],
    )
    .await?;

    assert_eq!(decrypted[0].output_type, BALANCE_FHE_TYPE as i16);
    assert_eq!(decrypted[0].value, "25");
    assert_eq!(decrypted[1].output_type, BALANCE_FHE_TYPE as i16);
    assert_eq!(decrypted[1].value, "120");

    Ok(())
}

#[test]
#[ignore = "requires built Solana PoC programs; validates FHE eval replay from LiteSVM metadata"]
fn solana_fhe_eval_replays_threshold_logs_from_litesvm_metadata() {
    let mut fixture = host_fixture();
    let lhs_handle = typed_balance_handle(0x51);
    let rhs_handle = typed_balance_handle(0x52);
    let acl_domain_key = Pubkey::new_unique();
    let app_account = fixture.payer.pubkey();
    let lhs_label = label_bytes(b"lhs");
    let rhs_label = label_bytes(b"rhs");
    let output_label = label_bytes(b"out");
    let lhs_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, lhs_label);
    let rhs_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, rhs_label);
    let output_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, output_label);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut fixture.svm,
        lhs_nonce_key,
        0,
        acl_domain_key,
        app_account,
        lhs_label,
        lhs_handle,
        fixture.payer.pubkey(),
    );
    let rhs_acl_record = seed_authorizing_acl_record(
        &mut fixture.svm,
        rhs_nonce_key,
        0,
        acl_domain_key,
        app_account,
        rhs_label,
        rhs_handle,
        fixture.payer.pubkey(),
    );
    let output_acl_record = host::acl_record_address(output_nonce_key, 1).0;
    let mut ix = Instruction {
        program_id: fixture.host_program_id,
        accounts: host::accounts::FheEval {
            payer: fixture.payer.pubkey(),
            compute_subject: fixture.payer.pubkey(),
            app_account_authority: fixture.payer.pubkey(),
            host_config: Pubkey::find_program_address(
                &[host::HOST_CONFIG_SEED],
                &fixture.host_program_id,
            )
            .0,
            system_program: system_program::ID,
            // Block-cap optional accounts: default cap is unrestricted, so existing flows
            // pass None/None and behave exactly as before the feature. The mandatory HCU
            // authority is the payer for this wallet-driven test frame.
            hcu_authority: fixture.payer.pubkey(),
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(fixture.host_program_id),
            program: fixture.host_program_id,
        }
        .to_account_metas(None),
        data: host::instruction::FheEval {
            args: FheEvalArgs {
                context_id: [7; 32],
                steps: vec![FheEvalStep::Binary {
                    op: host::FheBinaryOpCode::Add,
                    lhs: FheEvalOperand::AllowedDurable {
                        handle: lhs_handle,
                        acl_record_index: 0,
                        permission_index: None,
                    },
                    rhs: FheEvalOperand::AllowedDurable {
                        handle: rhs_handle,
                        acl_record_index: 1,
                        permission_index: None,
                    },
                    output_fhe_type: TOKEN_BALANCE_FHE_TYPE,
                    output: FheEvalOutput::AllowedDurable {
                        output_acl_record_index: 2,
                        output_app_account_authority_index: None,
                        output_nonce_key,
                        output_nonce_sequence: 1,
                        output_acl_domain_key: acl_domain_key,
                        output_app_account: app_account,
                        output_encrypted_value_label: output_label,
                        output_subjects: vec![
                            AclSubjectEntry::use_only(fixture.payer.pubkey()),
                            AclSubjectEntry::use_only(Pubkey::new_unique()),
                            AclSubjectEntry::use_only(Pubkey::new_unique()),
                            AclSubjectEntry::use_only(Pubkey::new_unique()),
                        ],
                        output_public_decrypt: false,
                    },
                }],
            },
        }
        .data(),
    };
    ix.accounts
        .push(AccountMeta::new_readonly(lhs_acl_record, false));
    ix.accounts
        .push(AccountMeta::new_readonly(rhs_acl_record, false));
    ix.accounts.push(AccountMeta::new(output_acl_record, false));

    let (meta, account_keys, signature) = send_with_meta(&mut fixture.svm, &fixture.payer, ix);

    assert!(
        !meta.logs.iter().any(|log| log == "Log truncated"),
        "thresholded transfer logs exceeded LiteSVM's default Solana log byte limit"
    );
    let recorded_log_bytes: usize = meta.logs.iter().map(String::len).sum();
    assert!(
        recorded_log_bytes < DEFAULT_SOLANA_LOG_BYTES_LIMIT,
        "{recorded_log_bytes}"
    );

    let cpi_events = host_cpi_events(&meta, &account_keys, fixture.host_program_id);
    assert!(
        cpi_events.is_empty(),
        "large fhe_eval frames must not fall back to self-CPI transport"
    );

    let log_events = host_log_events(&meta, fixture.host_program_id);
    assert_eq!(count_tfhe_events(&log_events), 1);
    assert_eq!(count_acl_events(&log_events), 4);

    let host_events = host_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(count_tfhe_events(&host_events), 1);
    assert_eq!(count_acl_events(&host_events), 4);

    let transaction_id = solana_transaction_id(signature.as_ref());
    let block = SolanaBlockMeta {
        block_number: 1,
        block_timestamp: PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 11).unwrap(),
            Time::MIDNIGHT,
        ),
    };
    let (tfhe_logs, account_fetches) =
        normalize_solana_events_for_db(host_events, transaction_id, block);

    assert_eq!(tfhe_logs.len(), 1);
    assert_eq!(account_fetches.len(), 1);
    assert_eq!(account_fetches[0].account_key, output_acl_record.to_bytes());
    assert_eq!(
        account_fetches[0].kind,
        SolanaFinalizedAccountFetchKind::AclRecord
    );
    assert_eq!(account_fetches[0].reason, "acl_record_bound");
    assert_eq!(
        account_fetches[0].handle,
        tfhe_result_handle(&tfhe_logs[0].event.data)
    );
    assert!(tfhe_logs[0].is_allowed);
}

#[test]
fn solana_worker_replay_shape_preserves_eval_dependencies_and_ignores_same_tx_acl_allowance() {
    let comparison = typed_handle(0x60, 0);
    let alice_balance = typed_balance_handle(0x61);
    let transfer_amount = typed_balance_handle(0x62);
    let trivial_amount = typed_balance_handle(0x63);
    let selected_balance = typed_balance_handle(0x64);
    let random_amount = typed_balance_handle(0x65);
    let tx_id = solana_transaction_id(&[9_u8; 64]);
    let block = SolanaBlockMeta {
        block_number: 7,
        block_timestamp: PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 11).unwrap(),
            Time::MIDNIGHT,
        ),
    };

    let events = vec![
        SolanaHostEvent::FheBinaryOp(FheBinaryOpEvent {
            version: EVENT_VERSION,
            op: SolanaFheBinaryOpCode::Ge,
            subject: [0; 32],
            lhs: alice_balance,
            rhs: transfer_amount,
            scalar: false,
            result: comparison,
        }),
        SolanaHostEvent::TrivialEncrypt(TrivialEncryptEvent {
            version: EVENT_VERSION,
            subject: [0; 32],
            plaintext: amount_to_plaintext(25),
            fhe_type: TOKEN_BALANCE_FHE_TYPE,
            result: trivial_amount,
        }),
        SolanaHostEvent::FheTernaryOp(FheTernaryOpEvent {
            version: EVENT_VERSION,
            op: SolanaFheTernaryOpCode::IfThenElse,
            subject: [0; 32],
            control: comparison,
            if_true: trivial_amount,
            if_false: alice_balance,
            result: selected_balance,
        }),
        SolanaHostEvent::AclAllowed(SolanaAclAllowedEvent {
            handle: Handle::from(selected_balance),
            subject: format!("0x{}", "07".repeat(32)),
            event_type: AllowEvents::AllowedAccount,
        }),
        SolanaHostEvent::FheRandBounded(FheRandBoundedEvent {
            version: EVENT_VERSION,
            subject: [0; 32],
            upper_bound: amount_to_plaintext(16),
            seed: [8; 16],
            fhe_type: TOKEN_BALANCE_FHE_TYPE,
            result: random_amount,
        }),
        SolanaHostEvent::AclAllowed(SolanaAclAllowedEvent {
            handle: Handle::from(random_amount),
            subject: format!("0x{}", "08".repeat(32)),
            event_type: AllowEvents::AllowedAccount,
        }),
    ];
    assert_eq!(count_tfhe_events(&events), 4);
    assert_eq!(count_acl_events(&events), 2);

    let (tfhe_logs, account_fetches) = normalize_solana_events_for_db(events, tx_id, block);

    assert!(account_fetches.is_empty());
    assert_eq!(tfhe_logs.len(), 4);
    assert_eq!(
        tfhe_logs
            .iter()
            .map(|log| log.log_index)
            .collect::<Vec<_>>(),
        vec![Some(0), Some(1), Some(2), Some(4)]
    );
    assert_eq!(
        tfhe_logs
            .iter()
            .map(|log| log.is_allowed)
            .collect::<Vec<_>>(),
        vec![false, false, false, false]
    );
    assert_eq!(
        tfhe_logs
            .iter()
            .map(|log| log.transaction_hash)
            .collect::<Vec<_>>(),
        vec![Some(tx_id); 4]
    );
    assert_eq!(
        tfhe_logs
            .iter()
            .map(|log| log.dependence_chain)
            .collect::<Vec<_>>(),
        vec![tx_id; 4]
    );

    assert!(matches!(
        tfhe_logs[0].event.data,
        TfheContractEvents::FheGe(_)
    ));
    assert!(matches!(
        tfhe_logs[1].event.data,
        TfheContractEvents::TrivialEncrypt(_)
    ));
    assert!(matches!(
        tfhe_logs[2].event.data,
        TfheContractEvents::FheIfThenElse(_)
    ));
    assert!(matches!(
        tfhe_logs[3].event.data,
        TfheContractEvents::FheRandBounded(_)
    ));

    assert_eq!(
        tfhe_inputs_handle(&tfhe_logs[0].event.data),
        vec![Handle::from(alice_balance), Handle::from(transfer_amount)]
    );
    assert_eq!(
        tfhe_inputs_handle(&tfhe_logs[1].event.data),
        Vec::<Handle>::new()
    );
    assert_eq!(
        tfhe_inputs_handle(&tfhe_logs[2].event.data),
        vec![
            Handle::from(comparison),
            Handle::from(trivial_amount),
            Handle::from(alice_balance)
        ]
    );
    assert_eq!(
        tfhe_inputs_handle(&tfhe_logs[3].event.data),
        Vec::<Handle>::new()
    );
    assert_eq!(
        tfhe_logs
            .iter()
            .map(|log| tfhe_result_handle(&log.event.data))
            .collect::<Vec<_>>(),
        vec![
            Some(Handle::from(comparison)),
            Some(Handle::from(trivial_amount)),
            Some(Handle::from(selected_balance)),
            Some(Handle::from(random_amount)),
        ]
    );
}

#[tokio::test]
#[serial(db)]
#[ignore = "runs LiteSVM plus the real TFHE worker against a disposable Postgres DB"]
async fn solana_trivial_encrypt_then_confidential_transfer_computes_and_decrypts(
) -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let mut fixture = token_fixture();

    let amount_handle = balance_handle(0x19);

    let initial_ixs = vec![
        test_emit_trivial_encrypt_ix(
            fixture.host_program_id,
            fixture.alice.pubkey(),
            fixture.compute_signer,
            125,
            fixture.alice_initial,
        ),
        test_emit_acl_allowed_ix(
            fixture.host_program_id,
            fixture.alice.pubkey(),
            fixture.alice_initial,
            fixture.compute_signer,
        ),
        test_emit_trivial_encrypt_ix(
            fixture.host_program_id,
            fixture.alice.pubkey(),
            fixture.compute_signer,
            20,
            fixture.bob_initial,
        ),
        test_emit_acl_allowed_ix(
            fixture.host_program_id,
            fixture.alice.pubkey(),
            fixture.bob_initial,
            fixture.compute_signer,
        ),
        test_emit_trivial_encrypt_ix(
            fixture.host_program_id,
            fixture.alice.pubkey(),
            fixture.compute_signer,
            100,
            amount_handle,
        ),
        test_emit_acl_allowed_ix(
            fixture.host_program_id,
            fixture.alice.pubkey(),
            amount_handle,
            fixture.compute_signer,
        ),
    ];
    let (meta, account_keys, signature) =
        send_many_with_meta(&mut fixture.svm, &fixture.alice, initial_ixs);
    let mut initial_events = host_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(count_tfhe_events(&initial_events), 3);
    assert_eq!(count_acl_events(&initial_events), 3);
    add_allow_fetches(
        &mut initial_events,
        &[fixture.alice_initial, fixture.bob_initial, amount_handle],
    );

    insert_host_events(&harness.listener_db, initial_events, signature, 1).await?;
    wait_until_computed(&harness.app).await?;

    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, amount_handle);
    let (meta, account_keys, signature) =
        send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    let new_alice_handle = read_acl_record(&fixture.svm, output.alice)
        .expect("expected Alice output ACL")
        .handle;
    let new_bob_handle = read_acl_record(&fixture.svm, output.bob)
        .expect("expected Bob output ACL")
        .handle;
    let transfer_events = host_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(count_tfhe_events(&transfer_events), 5);
    assert_eq!(count_acl_events(&transfer_events), 7);

    insert_host_events(&harness.listener_db, transfer_events, signature, 2).await?;
    wait_until_computed(&harness.app).await?;
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &signed_user_decrypt_request(
            &fixture.alice,
            vec![fixture.mint.pubkey()],
            vec![UserDecryptHandleEntry {
                handle: new_alice_handle,
                owner: fixture.alice.pubkey(),
                acl_record: output.alice,
            }],
        ),
    ));
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &signed_user_decrypt_request(
            &fixture.bob,
            vec![fixture.mint.pubkey()],
            vec![UserDecryptHandleEntry {
                handle: new_bob_handle,
                owner: fixture.bob.pubkey(),
                acl_record: output.bob,
            }],
        ),
    ));
    let decrypted = decrypt_handles(
        &harness.pool,
        &[Handle::from(new_alice_handle), Handle::from(new_bob_handle)],
    )
    .await?;

    assert_eq!(decrypted[0].output_type, BALANCE_FHE_TYPE as i16);
    assert_eq!(decrypted[0].value, "25");
    assert_eq!(decrypted[1].output_type, BALANCE_FHE_TYPE as i16);
    assert_eq!(decrypted[1].value, "120");

    Ok(())
}

#[tokio::test]
#[serial(db)]
#[ignore = "runs LiteSVM plus the real TFHE worker against a disposable Postgres DB"]
async fn solana_fhe_rand_creates_ciphertext_and_decrypts() -> Result<(), Box<dyn std::error::Error>>
{
    let harness = setup_event_harness().await?;
    let mut fixture = host_fixture();
    let rand_handle = typed_fast_handle(0x29);

    let ixs = vec![
        test_emit_fhe_rand_ix(
            fixture.host_program_id,
            fixture.payer.pubkey(),
            fixture.payer.pubkey(),
            [7_u8; 16],
            rand_handle,
        ),
        test_emit_acl_allowed_ix(
            fixture.host_program_id,
            fixture.payer.pubkey(),
            rand_handle,
            fixture.payer.pubkey(),
        ),
    ];
    let (meta, account_keys, signature) =
        send_many_with_meta(&mut fixture.svm, &fixture.payer, ixs);
    let mut events = host_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(count_tfhe_events(&events), 1);
    assert_eq!(count_acl_events(&events), 1);
    add_allow_fetches(&mut events, &[rand_handle]);

    insert_host_events(&harness.listener_db, events, signature, 1).await?;
    wait_until_computed(&harness.app).await?;

    let decrypted = decrypt_handles(&harness.pool, &[Handle::from(rand_handle)]).await?;
    assert_eq!(decrypted[0].output_type, FAST_REAL_FHE_TYPE as i16);
    let value = decrypted[0].value.parse::<u16>()?;
    assert!(value <= u8::MAX as u16);

    Ok(())
}

#[test]
#[ignore = "requires built Solana PoC programs; validates user-decrypt ACL semantics without running the worker"]
fn solana_user_decrypt_acl_invariants_match_evm_semantics() {
    let mut fixture = token_fixture();
    let amount_handle = balance_handle(0x39);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, amount_handle);
    send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    let new_alice_handle = read_acl_record(&fixture.svm, output.alice)
        .expect("expected Alice output ACL")
        .handle;

    let valid = signed_user_decrypt_request(
        &fixture.alice,
        vec![fixture.mint.pubkey()],
        vec![UserDecryptHandleEntry {
            handle: new_alice_handle,
            owner: fixture.alice.pubkey(),
            acl_record: output.alice,
        }],
    );
    assert!(kms_like_user_decrypt_check(&fixture.svm, &valid));

    let missing_user_acl = signed_user_decrypt_request(
        &fixture.alice,
        vec![fixture.mint.pubkey()],
        vec![UserDecryptHandleEntry {
            handle: new_alice_handle,
            owner: fixture.alice.pubkey(),
            acl_record: output.bob,
        }],
    );
    assert!(!kms_like_user_decrypt_check(
        &fixture.svm,
        &missing_user_acl
    ));

    let missing_domain = signed_user_decrypt_request(
        &fixture.alice,
        vec![Pubkey::new_unique()],
        valid.handles.clone(),
    );
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &missing_domain));

    let wrong_owner = signed_user_decrypt_request(
        &fixture.bob,
        vec![fixture.mint.pubkey()],
        valid.handles.clone(),
    );
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &wrong_owner));

    let wrong_handle = signed_user_decrypt_request(
        &fixture.alice,
        vec![fixture.mint.pubkey()],
        vec![UserDecryptHandleEntry {
            handle: balance_handle(0x7f),
            ..valid.handles[0]
        }],
    );
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &wrong_handle));
}

struct TokenFixture {
    svm: LiteSVM,
    host_program_id: Pubkey,
    host_config: Pubkey,
    token_program_id: Pubkey,
    alice: Keypair,
    bob: Keypair,
    mint: Keypair,
    compute_signer: Pubkey,
    alice_token: Pubkey,
    bob_token: Pubkey,
    alice_initial: [u8; 32],
    bob_initial: [u8; 32],
    alice_current_compute_acl: Pubkey,
    bob_current_compute_acl: Pubkey,
}

struct HostFixture {
    svm: LiteSVM,
    host_program_id: Pubkey,
    payer: Keypair,
}

#[derive(Clone, Copy)]
struct TransferOutputAccounts {
    alice: Pubkey,
    bob: Pubkey,
    transferred: Pubkey,
}

#[derive(Clone)]
struct UserDecryptAuthorizationPayload {
    user: Pubkey,
    reencryption_public_key: [u8; 32],
    allowed_acl_domain_keys: Vec<Pubkey>,
    start_timestamp: i64,
    duration_seconds: u64,
    extra_data: Vec<u8>,
}

#[derive(Clone)]
struct UserDecryptRequest {
    authorization: UserDecryptAuthorizationPayload,
    signature: Signature,
    handles: Vec<UserDecryptHandleEntry>,
}

#[derive(Clone, Copy)]
struct UserDecryptHandleEntry {
    handle: [u8; 32],
    owner: Pubkey,
    acl_record: Pubkey,
}

fn host_fixture() -> HostFixture {
    let host_program_id = host::id();
    let host_program_path = host_program_so_path();
    assert!(
        host_program_path.exists(),
        "missing {}; run `cd solana && anchor build` before this test",
        host_program_path.display()
    );

    let mut svm = LiteSVM::new();
    svm.add_program_from_file(host_program_id, &host_program_path)
        .unwrap();
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    let _host_config = seed_host_config(
        &mut svm,
        host_program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    HostFixture {
        svm,
        host_program_id,
        payer,
    }
}

fn seed_host_config(
    svm: &mut LiteSVM,
    program_id: Pubkey,
    admin: Pubkey,
    input_verifier_authority: Pubkey,
    test_authority: Pubkey,
) -> Pubkey {
    let (host_config, bump) = Pubkey::find_program_address(&[host::HOST_CONFIG_SEED], &program_id);
    svm.set_account(
        host_config,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(HostConfig {
                admin,
                chain_id: host::SOLANA_POC_CHAIN_ID,
                input_verifier_authority,
                // Coprocessor `fromExternal` verifier: transfers bind the amount via a
                // secp256k1 EIP-712 attestation that fhe_eval re-verifies in-frame.
                gateway_chain_id: SECP_GATEWAY_CHAIN_ID,
                input_verification_contract: INPUT_VERIFICATION_CONTRACT,
                coprocessor_signer: secp_evm_address(&coprocessor_signing_key()),
                decryption_contract: [0u8; 20],
                current_kms_context_id: 0,
                material_authority: input_verifier_authority,
                test_authority,
                paused: false,
                mock_input_enabled: true,
                test_shims_enabled: true,
                grant_deny_list_enabled: false,
                max_hcu_per_tx: 0,
                max_hcu_depth_per_tx: 0,
                // Unrestricted (the ship default): the block cap short-circuits without
                // requiring the optional meter/trust accounts.
                hcu_block_cap_per_app: u64::MAX,
                updated_slot: 0,
                bump,
            }),
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
    host_config
}

fn token_fixture() -> TokenFixture {
    token_fixture_with_initial_balances(125, 20)
}

fn token_fixture_with_initial_balances(
    // Balances are injected as input ciphertexts keyed by handle (the program forbids
    // nonzero init balances and funds via wrap), so these are documentation only.
    _alice_initial_balance: u64,
    _bob_initial_balance: u64,
) -> TokenFixture {
    let host_program_id = host::id();
    let token_program_id = token::id();
    let host_program_path = host_program_so_path();
    let token_program_path = token_program_so_path();
    assert!(
        host_program_path.exists(),
        "missing {}; run `cd solana && anchor build` before this test",
        host_program_path.display()
    );
    assert!(
        token_program_path.exists(),
        "missing {}; run `cd solana && anchor build` before this test",
        token_program_path.display()
    );

    let mut svm = LiteSVM::new();
    svm.add_program_from_file(host_program_id, &host_program_path)
        .unwrap();
    svm.add_program_from_file(token_program_id, &token_program_path)
        .unwrap();

    let alice = Keypair::new();
    let bob = Keypair::new();
    let mint = Keypair::new();
    let underlying_mint = Keypair::new();
    svm.airdrop(&alice.pubkey(), 2_000_000_000).unwrap();
    svm.airdrop(&bob.pubkey(), 1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        host_program_id,
        alice.pubkey(),
        alice.pubkey(),
        alice.pubkey(),
    );
    create_spl_mint(&mut svm, &alice, &underlying_mint, 6);
    let compute_signer = token::compute_signer_address(mint.pubkey()).0;
    let total_supply_authority = token::total_supply_authority_address(mint.pubkey()).0;
    let total_supply_acl_record = acl_record_address(
        host_program_id,
        token::total_supply_nonce_key(mint.pubkey(), total_supply_authority),
        0,
    );

    send_with_signers(
        &mut svm,
        &alice.pubkey(),
        Instruction {
            program_id: token_program_id,
            accounts: token::accounts::InitializeMint {
                authority: alice.pubkey(),
                mint: mint.pubkey(),
                underlying_mint: underlying_mint.pubkey(),
                compute_signer,
                total_supply_authority,
                total_supply_acl_record,
                zama_event_authority: event_authority(host_program_id),
                zama_program: host_program_id,
                host_config,
                system_program: system_program::ID,
                hcu_authority: hcu_authority_address(token_program_id, mint.pubkey()),
                hcu_block_meter: None,
                hcu_trusted_app_record: None,
                event_authority: event_authority(token_program_id),
                program: token_program_id,
            }
            .to_account_metas(None),
            data: token::instruction::InitializeMint {}.data(),
        },
        &[&alice, &mint],
    );

    let alice_token = token_account_address(token_program_id, mint.pubkey(), alice.pubkey());
    let bob_token = token_account_address(token_program_id, mint.pubkey(), bob.pubkey());
    let alice_current_compute_acl =
        balance_acl_record_address(host_program_id, mint.pubkey(), alice_token, 0);
    let bob_current_compute_acl =
        balance_acl_record_address(host_program_id, mint.pubkey(), bob_token, 0);

    initialize_token_account(
        &mut svm,
        &alice,
        TokenAccountInit {
            token_program_id,
            host_program_id,
            host_config,
            mint: mint.pubkey(),
            token_account: alice_token,
            compute_signer,
            acl_record: alice_current_compute_acl,
            // Program forbids nonzero init balances (funded via wrap); the test injects
            // the real input ciphertext value (125) into the DB keyed by this handle.
            initial_balance: 0,
        },
    );
    initialize_token_account(
        &mut svm,
        &bob,
        TokenAccountInit {
            token_program_id,
            host_program_id,
            host_config,
            mint: mint.pubkey(),
            token_account: bob_token,
            compute_signer,
            acl_record: bob_current_compute_acl,
            initial_balance: 0,
        },
    );
    let alice_initial = read_acl_record(&svm, alice_current_compute_acl)
        .expect("expected Alice initial ACL")
        .handle;
    let bob_initial = read_acl_record(&svm, bob_current_compute_acl)
        .expect("expected Bob initial ACL")
        .handle;

    TokenFixture {
        svm,
        host_program_id,
        host_config,
        token_program_id,
        alice,
        bob,
        mint,
        compute_signer,
        alice_token,
        bob_token,
        alice_initial,
        bob_initial,
        alice_current_compute_acl,
        bob_current_compute_acl,
    }
}

struct TokenAccountInit {
    token_program_id: Pubkey,
    host_program_id: Pubkey,
    host_config: Pubkey,
    mint: Pubkey,
    token_account: Pubkey,
    compute_signer: Pubkey,
    acl_record: Pubkey,
    initial_balance: u64,
}

fn initialize_token_account(svm: &mut LiteSVM, owner: &Keypair, init: TokenAccountInit) {
    send(
        svm,
        owner,
        Instruction {
            program_id: init.token_program_id,
            accounts: token::accounts::InitializeTokenAccount {
                owner: owner.pubkey(),
                mint: init.mint,
                compute_signer: init.compute_signer,
                token_account: init.token_account,
                acl_record: init.acl_record,
                zama_event_authority: event_authority(init.host_program_id),
                zama_program: init.host_program_id,
                host_config: init.host_config,
                system_program: system_program::ID,
                hcu_authority: hcu_authority_address(init.token_program_id, init.mint),
                hcu_block_meter: None,
                hcu_trusted_app_record: None,
                event_authority: event_authority(init.token_program_id),
                program: init.token_program_id,
            }
            .to_account_metas(None),
            data: token::instruction::InitializeTokenAccount {
                initial_balance: init.initial_balance,
            }
            .data(),
        },
    );
}

fn transfer_output_accounts(fixture: &TokenFixture, nonce_sequence: u64) -> TransferOutputAccounts {
    TransferOutputAccounts {
        alice: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.alice_token,
            nonce_sequence,
        ),
        bob: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.bob_token,
            nonce_sequence,
        ),
        transferred: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.alice_token,
                token::transferred_amount_label(),
            ),
            nonce_sequence,
        ),
    }
}

fn transfer_ix(
    fixture: &TokenFixture,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    Instruction {
        program_id: fixture.token_program_id,
        accounts: token::accounts::ConfidentialTransfer {
            // Block-cap optional accounts threaded through the transfer CPI; the default
            // unrestricted cap means None/None here. The mint's HCU authority is mandatory.
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            hcu_authority: hcu_authority_address(fixture.token_program_id, fixture.mint.pubkey()),
            owner: fixture.alice.pubkey(),
            payer: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            compute_signer: fixture.compute_signer,
            from_current_compute_acl: fixture.alice_current_compute_acl,
            to_current_compute_acl: fixture.bob_current_compute_acl,
            from_output_acl: output.alice,
            transferred_amount_acl: output.transferred,
            to_output_acl: output.bob,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        }
        .to_account_metas(None),
        data: token::instruction::ConfidentialTransfer {
            // fromExternal: the amount is a coprocessor-signed attestation bound to
            // (user = owner, contract = mint compute-signer PDA), re-verified in fhe_eval.
            amount_attestation: amount_attestation_for(
                amount_handle,
                fixture.alice.pubkey(),
                fixture.compute_signer,
            ),
        }
        .data(),
    }
}

/// Coprocessor signing key backing the `fromExternal` amount attestations; its EVM address is the
/// `coprocessor_signer` configured on the fixture's `host_config`.
fn coprocessor_signing_key() -> k256::ecdsa::SigningKey {
    k256::ecdsa::SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap()
}

/// Recovers the EVM address (keccak(pubkey)[12..]) for a coprocessor signing key, matching the
/// on-chain `secp256k1_recover` derivation.
fn secp_evm_address(key: &k256::ecdsa::SigningKey) -> [u8; 20] {
    let encoded = key.verifying_key().to_encoded_point(false); // 0x04 || X || Y
    let hash = solana_sdk::keccak::hash(&encoded.as_bytes()[1..]).to_bytes();
    let mut address = [0u8; 20];
    address.copy_from_slice(&hash[12..]);
    address
}

/// 65-byte `[r || s || v]` recoverable signature over an EIP-712 digest.
fn secp_sign(key: &k256::ecdsa::SigningKey, digest: &[u8; 32]) -> [u8; 65] {
    let (signature, recovery_id) = key.sign_prehash_recoverable(digest).unwrap();
    let mut out = [0u8; 65];
    out[..64].copy_from_slice(&signature.to_bytes());
    out[64] = 27 + recovery_id.to_byte();
    out
}

/// Builds a coprocessor-signed `fromExternal` attestation over `amount_handle`, binding it to
/// (`user`, `contract`). The token program checks `user == transfer owner` and
/// `contract == mint compute-signer PDA`; the host re-verifies this signature in-frame.
fn amount_attestation_for(
    amount_handle: [u8; 32],
    user: Pubkey,
    contract: Pubkey,
) -> host::CoprocessorInputAttestation {
    let key = coprocessor_signing_key();
    let ct_handles = vec![amount_handle];
    let contract_chain_id = 12345u64;
    let extra_data = vec![0x00u8];
    let digest = host::eip712::typed_data_digest(
        &host::eip712::domain_separator(
            b"InputVerification",
            b"1",
            SECP_GATEWAY_CHAIN_ID,
            &INPUT_VERIFICATION_CONTRACT,
        ),
        &host::eip712::ciphertext_verification_struct_hash(
            &ct_handles,
            &user.to_bytes(),
            &contract.to_bytes(),
            contract_chain_id,
            &extra_data,
        ),
    );
    host::CoprocessorInputAttestation {
        input_handle: amount_handle,
        ct_handles,
        handle_index: 0,
        user_address: user.to_bytes(),
        contract_address: contract.to_bytes(),
        contract_chain_id,
        extra_data,
        signatures: vec![secp_sign(&key, &digest)],
    }
}

fn test_emit_trivial_encrypt_ix(
    program_id: Pubkey,
    test_authority: Pubkey,
    subject: Pubkey,
    value: u64,
    result: [u8; 32],
) -> Instruction {
    Instruction {
        program_id,
        accounts: host::accounts::TestEmitProtocolEvent {
            test_authority,
            host_config: Pubkey::find_program_address(&[host::HOST_CONFIG_SEED], &program_id).0,
            event_authority: event_authority(program_id),
            program: program_id,
        }
        .to_account_metas(None),
        data: host::instruction::TestEmitTrivialEncrypt {
            subject,
            plaintext: amount_to_plaintext(value),
            fhe_type: BALANCE_FHE_TYPE,
            result,
        }
        .data(),
    }
}

fn test_emit_acl_allowed_ix(
    program_id: Pubkey,
    test_authority: Pubkey,
    handle: [u8; 32],
    subject: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: host::accounts::TestEmitProtocolEvent {
            test_authority,
            host_config: Pubkey::find_program_address(&[host::HOST_CONFIG_SEED], &program_id).0,
            event_authority: event_authority(program_id),
            program: program_id,
        }
        .to_account_metas(None),
        data: host::instruction::TestEmitAclAllowed { handle, subject }.data(),
    }
}

fn test_emit_fhe_rand_ix(
    program_id: Pubkey,
    test_authority: Pubkey,
    subject: Pubkey,
    seed: [u8; 16],
    result: [u8; 32],
) -> Instruction {
    Instruction {
        program_id,
        accounts: host::accounts::TestEmitProtocolEvent {
            test_authority,
            host_config: Pubkey::find_program_address(&[host::HOST_CONFIG_SEED], &program_id).0,
            event_authority: event_authority(program_id),
            program: program_id,
        }
        .to_account_metas(None),
        data: host::instruction::TestEmitFheRand {
            subject,
            seed,
            fhe_type: FAST_REAL_FHE_TYPE,
            result,
        }
        .data(),
    }
}

fn create_spl_mint(svm: &mut LiteSVM, payer: &Keypair, mint: &Keypair, decimals: u8) {
    let rent = svm.minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN);
    send_many_with_signers(
        svm,
        &payer.pubkey(),
        vec![
            system_instruction::create_account(
                &payer.pubkey(),
                &mint.pubkey(),
                rent,
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint2(
                &spl_token::id(),
                &mint.pubkey(),
                &payer.pubkey(),
                None,
                decimals,
            )
            .unwrap(),
        ],
        &[payer, mint],
    );
}

async fn seed_real_fast_ciphertexts(
    pool: &sqlx::PgPool,
    values: &[([u8; 32], u8)],
) -> Result<(), Box<dyn std::error::Error>> {
    let (key, _) = latest_db_key(pool).await;
    let values = values.to_vec();
    let ciphertexts =
        tokio::task::spawn_blocking(move || -> Result<Vec<SeededCiphertext>, String> {
            let client_key = key.cks.expect("test key must include a client key");
            tfhe::set_server_key(key.sks);

            values
                .into_iter()
                .map(|(handle, value)| {
                    let ciphertext = tfhe::FheUint64::try_encrypt(value as u64, &client_key)
                        .map_err(|err| err.to_string())?;
                    let supported = SupportedFheCiphertexts::FheUint64(ciphertext);
                    let ty = supported.type_num();
                    let compressed = supported.compress().map_err(|err| err.to_string())?;
                    Ok((handle, ty, compressed))
                })
                .collect()
        })
        .await?
        .map_err(std::io::Error::other)?;

    for (handle, ty, ciphertext) in ciphertexts {
        sqlx::query(
            r#"
                INSERT INTO ciphertexts(handle, ciphertext, ciphertext_version, ciphertext_type)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (handle, ciphertext_version) DO UPDATE
                SET ciphertext = EXCLUDED.ciphertext,
                    ciphertext_type = EXCLUDED.ciphertext_type
            "#,
        )
        .bind(handle.to_vec())
        .bind(ciphertext)
        .bind(current_ciphertext_version())
        .bind(ty)
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn insert_host_events(
    listener_db: &host_listener::database::tfhe_event_propagate::Database,
    host_events: Vec<SolanaHostEvent>,
    signature: Signature,
    block_number: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let transaction_id = solana_transaction_id(signature.as_ref());
    let block = SolanaBlockMeta {
        block_number,
        block_timestamp: PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 11)?,
            Time::MIDNIGHT,
        ),
    };
    let mut db_tx = listener_db.new_transaction().await?;
    insert_solana_events(listener_db, &mut db_tx, host_events, transaction_id, block).await?;
    db_tx.commit().await?;
    Ok(())
}

fn host_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<SolanaHostEvent> {
    let program_id = program_id.to_string();
    let inner_instructions = inner_instruction_refs(meta, account_keys);
    decode_solana_transaction_events(
        &meta.logs,
        inner_instructions
            .iter()
            .map(|(program_id, data)| (program_id.as_str(), *data)),
        &program_id,
    )
    .expect("host event transport must be ordered")
}

fn host_cpi_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<SolanaHostEvent> {
    let program_id = program_id.to_string();
    let inner_instructions = inner_instruction_refs(meta, account_keys);
    decode_anchor_cpi_events(
        inner_instructions
            .iter()
            .map(|(program_id, data)| (program_id.as_str(), *data)),
        &program_id,
    )
}

fn inner_instruction_refs<'a>(
    meta: &'a TransactionMetadata,
    account_keys: &[Pubkey],
) -> Vec<(String, &'a [u8])> {
    meta.inner_instructions
        .iter()
        .flatten()
        .map(|ix| {
            (
                ix.instruction.program_id(account_keys).to_string(),
                ix.instruction.data.as_slice(),
            )
        })
        .collect()
}

fn host_log_events(meta: &TransactionMetadata, program_id: Pubkey) -> Vec<SolanaHostEvent> {
    let program_id = program_id.to_string();
    decode_anchor_log_events(&meta.logs, &program_id)
}

fn count_tfhe_events(events: &[SolanaHostEvent]) -> usize {
    events
        .iter()
        .filter(|event| {
            matches!(
                event,
                SolanaHostEvent::FheBinaryOp(_)
                    | SolanaHostEvent::FheTernaryOp(_)
                    | SolanaHostEvent::TrivialEncrypt(_)
                    | SolanaHostEvent::FheRand(_)
                    | SolanaHostEvent::FheRandBounded(_)
            )
        })
        .count()
}

fn count_acl_events(events: &[SolanaHostEvent]) -> usize {
    events
        .iter()
        .filter(|event| matches!(event, SolanaHostEvent::AclAllowed(_)))
        .count()
}

fn add_allow_fetches(events: &mut Vec<SolanaHostEvent>, handles: &[[u8; 32]]) {
    events.extend(handles.iter().copied().map(|handle| {
        SolanaHostEvent::FinalizedAccountFetch(SolanaFinalizedAccountFetch {
            account_key: handle,
            kind: SolanaFinalizedAccountFetchKind::AclRecord,
            reason: "acl_record_bound",
            handle: Some(Handle::from(handle)),
            related_account: None,
            subject: None,
        })
    }));
}

fn signed_user_decrypt_request(
    signer: &Keypair,
    allowed_acl_domain_keys: Vec<Pubkey>,
    handles: Vec<UserDecryptHandleEntry>,
) -> UserDecryptRequest {
    let authorization = UserDecryptAuthorizationPayload {
        user: signer.pubkey(),
        reencryption_public_key: [7; 32],
        allowed_acl_domain_keys,
        start_timestamp: 1,
        duration_seconds: 300,
        extra_data: b"zama-solana-poc".to_vec(),
    };
    let signature = signer.sign_message(&authorization_payload_bytes(&authorization));

    UserDecryptRequest {
        authorization,
        signature,
        handles,
    }
}

fn authorization_payload_bytes(authorization: &UserDecryptAuthorizationPayload) -> Vec<u8> {
    let mut bytes = b"Zama Solana UserDecrypt v0".to_vec();
    bytes.extend_from_slice(authorization.user.as_ref());
    bytes.extend_from_slice(&authorization.reencryption_public_key);
    bytes.extend_from_slice(&(authorization.allowed_acl_domain_keys.len() as u32).to_le_bytes());
    for account in &authorization.allowed_acl_domain_keys {
        bytes.extend_from_slice(account.as_ref());
    }
    bytes.extend_from_slice(&authorization.start_timestamp.to_le_bytes());
    bytes.extend_from_slice(&authorization.duration_seconds.to_le_bytes());
    bytes.extend_from_slice(&(authorization.extra_data.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&authorization.extra_data);
    bytes
}

fn kms_like_user_decrypt_check(svm: &LiteSVM, request: &UserDecryptRequest) -> bool {
    let authorization = &request.authorization;
    let signed_payload = authorization_payload_bytes(authorization);
    if !request
        .signature
        .verify(authorization.user.as_ref(), &signed_payload)
        || authorization.reencryption_public_key == [0; 32]
        || authorization.duration_seconds == 0
        || authorization.extra_data.is_empty()
        || authorization.start_timestamp < 0
        || request.handles.is_empty()
    {
        return false;
    }

    request.handles.iter().all(|entry| {
        if authorization.user != entry.owner {
            return false;
        }

        let Some(raw_account) = svm.get_account(&entry.acl_record) else {
            return false;
        };
        if raw_account.owner != host::id() {
            return false;
        }
        let mut data = raw_account.data.as_slice();
        let Ok(record) = AclRecord::try_deserialize(&mut data) else {
            return false;
        };
        let expected_nonce_key = token::nonce_key(
            record.acl_domain_key,
            record.app_account,
            record.encrypted_value_label,
        );
        let expected_acl_record =
            acl_record_address(host::id(), expected_nonce_key, record.nonce_sequence);

        authorization
            .allowed_acl_domain_keys
            .contains(&record.acl_domain_key)
            && record.handle == entry.handle
            && record.nonce_key == expected_nonce_key
            && entry.acl_record == expected_acl_record
            && record_subjects(&record).contains(&authorization.user)
    })
}

fn record_subjects(record: &AclRecord) -> Vec<Pubkey> {
    record.subjects[..record.subject_count as usize].to_vec()
}

fn read_acl_record(svm: &LiteSVM, address: Pubkey) -> Option<AclRecord> {
    let raw_account = svm.get_account(&address)?;
    let mut data = raw_account.data.as_slice();
    AclRecord::try_deserialize(&mut data).ok()
}

#[allow(clippy::too_many_arguments)]
fn seed_authorizing_acl_record(
    svm: &mut LiteSVM,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subject: Pubkey,
) -> Pubkey {
    let (address, bump) = host::acl_record_address(nonce_key, nonce_sequence);
    let mut subjects = [Pubkey::default(); host::MAX_ACL_SUBJECTS];
    let mut subject_roles = [0; host::MAX_ACL_SUBJECTS];
    subjects[0] = subject;
    subject_roles[0] = host::ACL_ROLE_USE;
    svm.set_account(
        address,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(AclRecord {
                handle,
                nonce_key,
                nonce_sequence,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                subjects,
                subject_roles,
                subject_count: 1,
                overflow_subject_count: 0,
                public_decrypt: false,
                material_commitment: Pubkey::default(),
                material_commitment_hash: [0; 32],
                material_key_id: [0; 32],
                created_slot: 0,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
    address
}

fn serialized_account<T: AccountSerialize>(account: T) -> Vec<u8> {
    let mut data = Vec::new();
    account.try_serialize(&mut data).unwrap();
    data
}

fn label_bytes(label: &[u8]) -> [u8; 32] {
    let mut bytes = [b'_'; 32];
    bytes[..label.len()].copy_from_slice(label);
    bytes
}

fn typed_balance_handle(seed: u8) -> [u8; 32] {
    typed_handle(seed, TOKEN_BALANCE_FHE_TYPE)
}

fn typed_handle(seed: u8, fhe_type: u8) -> [u8; 32] {
    // Canonical handle metadata the host validates on input bind: embedded chain
    // id (bytes 22..30), fhe type (byte 30), and handle version (byte 31).
    let mut handle = [seed; 32];
    handle[21] = 0;
    handle[22..30].copy_from_slice(&host::SOLANA_POC_CHAIN_ID.to_be_bytes());
    handle[30] = fhe_type;
    handle[31] = host::HANDLE_VERSION;
    handle
}

fn typed_fast_handle(seed: u8) -> [u8; 32] {
    typed_handle(seed, FAST_REAL_FHE_TYPE)
}

fn balance_handle(seed: u8) -> [u8; 32] {
    typed_handle(seed, BALANCE_FHE_TYPE)
}

fn amount_to_plaintext(amount: u64) -> [u8; 32] {
    let mut plaintext = [0_u8; 32];
    plaintext[24..].copy_from_slice(&amount.to_be_bytes());
    plaintext
}

fn host_program_so_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../solana/target/deploy/zama_host.so")
}

fn token_program_so_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../solana/target/deploy/confidential_token.so")
}

fn event_authority(program_id: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"__event_authority"], &program_id).0
}

fn token_account_address(program_id: Pubkey, mint: Pubkey, owner: Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"token-account", mint.as_ref(), owner.as_ref()],
        &program_id,
    )
    .0
}

fn hcu_authority_address(program_id: Pubkey, mint: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"hcu-authority", mint.as_ref()], &program_id).0
}

fn acl_record_address(program_id: Pubkey, nonce_key: [u8; 32], nonce_sequence: u64) -> Pubkey {
    Pubkey::find_program_address(
        &[
            b"acl-record",
            nonce_key.as_ref(),
            &nonce_sequence.to_le_bytes(),
        ],
        &program_id,
    )
    .0
}

fn balance_acl_record_address(
    program_id: Pubkey,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    nonce_sequence: u64,
) -> Pubkey {
    acl_record_address(
        program_id,
        token::balance_nonce_key(acl_domain_key, app_account),
        nonce_sequence,
    )
}

fn send(svm: &mut LiteSVM, payer: &Keypair, ix: Instruction) {
    send_with_signers(svm, &payer.pubkey(), ix, &[payer]);
}

/// ComputeBudget `SetComputeUnitLimit` instruction (consensus-stable wire format:
/// variant tag 2 + u32 LE), hand-built to avoid a version-skewed solana dep.
fn set_compute_unit_limit_ix(units: u32) -> Instruction {
    let program_id: Pubkey = "ComputeBudget111111111111111111111111111111"
        .parse()
        .unwrap();
    let mut data = vec![2u8];
    data.extend_from_slice(&units.to_le_bytes());
    Instruction {
        program_id,
        accounts: vec![],
        data,
    }
}

fn send_with_meta(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ix: Instruction,
) -> (TransactionMetadata, Vec<Pubkey>, Signature) {
    // Confidential transfer's real euint64 FHE ops exceed the default 200k CU limit;
    // use the same client-side budget as the PoC live client.
    let ixs = [set_compute_unit_limit_ix(1_400_000), ix];
    let message = Message::new_with_blockhash(&ixs, Some(&payer.pubkey()), &svm.latest_blockhash());
    let account_keys = message.account_keys.clone();
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[payer]).unwrap();
    let signature = tx.signatures[0];
    (svm.send_transaction(tx).unwrap(), account_keys, signature)
}

fn send_many_with_meta(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ixs: Vec<Instruction>,
) -> (TransactionMetadata, Vec<Pubkey>, Signature) {
    let mut ixs = ixs;
    ixs.insert(0, set_compute_unit_limit_ix(1_400_000));
    let message = Message::new_with_blockhash(&ixs, Some(&payer.pubkey()), &svm.latest_blockhash());
    let account_keys = message.account_keys.clone();
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[payer]).unwrap();
    let signature = tx.signatures[0];
    (svm.send_transaction(tx).unwrap(), account_keys, signature)
}

fn send_with_signers(
    svm: &mut LiteSVM,
    payer: &Pubkey,
    ix: Instruction,
    signers: &[&Keypair],
) -> TransactionMetadata {
    let message = Message::new_with_blockhash(&[ix], Some(payer), &svm.latest_blockhash());
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), signers).unwrap();
    svm.send_transaction(tx).unwrap()
}

fn send_many_with_signers(
    svm: &mut LiteSVM,
    payer: &Pubkey,
    ixs: Vec<Instruction>,
    signers: &[&Keypair],
) -> TransactionMetadata {
    let message = Message::new_with_blockhash(&ixs, Some(payer), &svm.latest_blockhash());
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), signers).unwrap();
    svm.send_transaction(tx).unwrap()
}
