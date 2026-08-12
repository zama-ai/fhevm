//! The Solana vertical at the worker boundary: a LiteSVM confidential transfer is reconstructed
//! off-chain, propagated into a disposable migrated Postgres, computed by the REAL TFHE worker,
//! and decrypted — the one test that crosses from Solana transaction metadata all the way to
//! cleartexts without a deployed stack. CI runs it in solana-e2e's `worker-vertical` job; the
//! scenario suite covers the same arc live, but only this test pins the reconstruct-to-worker
//! seam against real ciphertexts.
//!
//! What it deliberately does not cross: the geyser/gRPC transport. Records are decoded straight
//! out of LiteSVM's transaction metadata instead of arriving over Yellowstone, and the block
//! metadata is supplied by the fixture because LiteSVM computes no bank hash. Everything from
//! `reconstruct_fhe_execute_records` inward is the production path; the wire above it is not.

use std::path::PathBuf;

use anchor_lang::{
    prelude::{system_instruction, system_program},
    AccountDeserialize, AccountSerialize, InstructionData, ToAccountMetas,
};
use anchor_spl::token::spl_token;
use fhevm_engine_common::{tfhe_ops::current_ciphertext_version, types::SupportedFheCiphertexts};
use host_listener::{
    database::tfhe_event_propagate::Handle,
    solana_adapter::{
        insert_solana_records, solana_transaction_id, SolanaBlockMeta, SolanaHostRecord,
    },
    solana_reconstruct::{
        decode_fhe_execute_args, reconstruct_fhe_execute_records, ReconstructContext,
    },
};
use litesvm::{types::TransactionMetadata, LiteSVM};
use serial_test::serial;
use solana_sdk::{
    account::Account,
    clock::Clock,
    hash::Hash,
    instruction::Instruction,
    message::{Message, VersionedMessage},
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    slot_hashes::SlotHashes,
    transaction::VersionedTransaction,
};
use tfhe::prelude::FheTryEncrypt;
use time::{OffsetDateTime, PrimitiveDateTime};
use zama_host::{EncryptedValue, HostConfig};

use crate::tests::{
    event_helpers::{decrypt_handles, setup_event_harness, wait_until_computed, EventHarness},
    utils::latest_db_key,
};

use confidential_token as token;
use zama_host as host;

const BALANCE_FHE_TYPE: u8 = 5;
const SECP_GATEWAY_CHAIN_ID: u64 = 31337;
const INPUT_VERIFICATION_CONTRACT: [u8; 20] = [0xCD; 20];
/// The parent slot's bank hash: seeded into `SlotHashes`, consumed by the reconstructor as the
/// previous bank hash, and carried as the parent hash of the block the transfers land in.
const PREVIOUS_BANK_HASH: [u8; 32] = [0x42; 32];
/// LiteSVM computes no bank hash, so the fixture supplies the current slot's. Deliberately
/// distinct from `PREVIOUS_BANK_HASH` so a parent/child mix-up cannot pass unnoticed.
const CURRENT_BANK_HASH: [u8; 32] = [0x43; 32];
/// Slot and wall-clock the fixture pins its `Clock` to. The block metadata handed to the listener
/// is read back off that sysvar rather than invented next to it.
const FIXTURE_SLOT: u64 = 100;
/// 2026-05-11T00:00:00Z.
const FIXTURE_UNIX_TIMESTAMP: i64 = 1_778_457_600;
/// Balance both token accounts share before the diverging transfer.
const OPENING_BALANCE: u8 = 125;
/// First transfer. It exists only to leave the two accounts on different handles holding
/// different values, so the asserted transfer can detect a swapped balance operand.
const DIVERGE_AMOUNT: u8 = 20;
/// Second transfer — the one this test asserts on.
const TRANSFER_AMOUNT: u8 = 40;
type SeededCiphertext = ([u8; 32], i16, Vec<u8>);

#[tokio::test]
#[serial(db)]
#[ignore = "needs the anchor-built programs, Docker, and the LFS test keys; solana-e2e's worker-vertical job runs it"]
async fn confidential_transfer_reconstructs_computes_and_decrypts(
) -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let mut fixture = token_fixture();
    let diverge_handle = balance_handle(0x09);
    let amount_handle = balance_handle(0x0A);

    // Account initialization trivially encrypts 0, and trivial-encrypt handles are derived from
    // the computation alone, so both accounts start on ONE shared zero-balance handle — seeding it
    // twice would only overwrite the first value. That sharing is also why this test transfers
    // twice: while both balances resolve to the same handle over the same ciphertext, swapping the
    // debit's operand from the sender to the recipient yields identical results and nothing
    // downstream can see it. The first transfer moves `DIVERGE_AMOUNT` so the two accounts hold
    // different handles over different values before the transfer that is actually asserted on.
    assert_eq!(
        fixture.alice_initial, fixture.bob_initial,
        "token accounts must start on the shared trivial-encrypt-zero handle"
    );
    seed_real_ciphertexts(
        &harness.pool,
        &[
            (fixture.alice_initial, OPENING_BALANCE),
            (diverge_handle, DIVERGE_AMOUNT),
            (amount_handle, TRANSFER_AMOUNT),
        ],
    )
    .await?;

    let outputs = transfer_output_accounts(&fixture);
    let diverged = run_transfer(&harness, &mut fixture, outputs, diverge_handle).await?;
    assert_ne!(
        diverged.alice_handle, diverged.bob_handle,
        "the diverging transfer must leave the balances on different handles"
    );

    let transfer = run_transfer(&harness, &mut fixture, outputs, amount_handle).await?;
    // The transfer batch: ge -> debit sub -> select -> transferred sub -> sender re-encrypt
    // (add 0, so the sender's handle rotates whether or not the debit succeeded) -> recipient
    // add. The re-encrypt step landed in the fhe_eval cleanup (fhevm-internal#1853); the select
    // result became transient then, so the persistent results are the last three.
    let events = &transfer.events;
    assert_eq!(
        events.len(),
        6,
        "token transfer must remain a six-step batch"
    );
    // The debit reads the SENDER's balance. Both operands are real, distinct handles by this
    // point, so this is the assertion the shared-handle opening state made impossible.
    assert!(matches!(
        &events[1],
        SolanaHostRecord::FheBinaryOp(event)
            if event.lhs == diverged.alice_handle && event.rhs == amount_handle
    ));
    // The select result is transient (no on-chain account carries it), so pin it by dataflow:
    // both the transferred-amount sub and the sender re-encrypt must consume it.
    let select_result = match &events[2] {
        SolanaHostRecord::FheTernaryOp(event) => event.result,
        other => panic!("step 2 must be the transfer's select, got {other:?}"),
    };
    assert!(matches!(
        &events[3],
        SolanaHostRecord::FheBinaryOp(event)
            if event.result == transfer.transferred_handle && event.rhs == select_result
    ));
    assert!(matches!(
        &events[4],
        SolanaHostRecord::FheBinaryOp(event)
            if event.result == transfer.alice_handle && event.lhs == select_result
    ));
    assert!(matches!(
        &events[5],
        SolanaHostRecord::FheBinaryOp(event)
            if event.result == transfer.bob_handle && event.lhs == diverged.bob_handle
    ));

    let decrypted = decrypt_handles(
        &harness.pool,
        &[
            Handle::from(diverged.alice_handle),
            Handle::from(diverged.bob_handle),
            Handle::from(transfer.alice_handle),
            Handle::from(transfer.bob_handle),
        ],
    )
    .await?;
    // Opening balance 125, shared. The first transfer moves 20 (sender 105, recipient 145); the
    // second moves 40 (sender 65, recipient 185). The second transfer reads two different handles
    // over two different values, so swapping its balance operands changes these numbers — the
    // property a single transfer out of the shared opening state could not give us. What value
    // alone still cannot pin: a successful transfer's `transferred` output always equals its
    // amount input, so that step is pinned by dataflow above rather than by arithmetic here.
    assert_eq!(decrypted[0].value, "105");
    assert_eq!(decrypted[1].value, "145");
    assert_eq!(decrypted[2].value, "65");
    assert_eq!(decrypted[3].value, "185");
    Ok(())
}

/// One confidential transfer end to end: sent through LiteSVM, reconstructed off-chain, landed in
/// the listener database, and computed by the worker before returning.
async fn run_transfer(
    harness: &EventHarness,
    fixture: &mut TokenFixture,
    outputs: TransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Result<TransferOutcome, Box<dyn std::error::Error>> {
    let transfer = transfer_ix(fixture, outputs, amount_handle);
    let (meta, account_keys, signature) =
        send_with_meta(&mut fixture.svm, &fixture.alice, transfer);
    let outcome = TransferOutcome {
        alice_handle: current_handle(&fixture.svm, outputs.alice),
        bob_handle: current_handle(&fixture.svm, outputs.bob),
        transferred_handle: current_handle(&fixture.svm, outputs.transferred),
        events: reconstruct_transfer_events(fixture, &meta, &account_keys),
    };

    let mut db_tx = harness
        .listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    let stats = insert_solana_records(
        &harness.listener_db,
        &mut db_tx,
        outcome.events.clone(),
        solana_transaction_id(signature.as_ref()),
        block_meta(&fixture.svm)?,
    )
    .await?;
    db_tx.commit().await?;
    assert_eq!(stats.tfhe_events, 6);

    wait_until_computed(&harness.app).await?;
    Ok(outcome)
}

/// Block metadata for the fixture's transfers, read back off the SVM clock so it cannot contradict
/// the state the programs themselves observed. The listener schedules the dependence chain from
/// `block_timestamp` and establishes reorg identity from the hash pair, so incoherent values here
/// would exercise that machinery on a block no validator could produce.
fn block_meta(svm: &LiteSVM) -> Result<SolanaBlockMeta, Box<dyn std::error::Error>> {
    let clock = svm.get_sysvar::<Clock>();
    let timestamp = OffsetDateTime::from_unix_timestamp(clock.unix_timestamp)?;
    Ok(SolanaBlockMeta {
        block_number: clock.slot,
        block_hash: CURRENT_BANK_HASH,
        parent_hash: PREVIOUS_BANK_HASH,
        block_timestamp: PrimitiveDateTime::new(timestamp.date(), timestamp.time()),
    })
}

struct TransferOutcome {
    events: Vec<SolanaHostRecord>,
    alice_handle: [u8; 32],
    bob_handle: [u8; 32],
    transferred_handle: [u8; 32],
}

fn reconstruct_transfer_events(
    fixture: &TokenFixture,
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
) -> Vec<SolanaHostRecord> {
    // fhe_execute has 9 named accounts (incl. event-CPI authority + program); the rest are the
    // batch's remaining accounts, which the dictionary wire format references by index.
    const FHE_EXECUTE_REMAINING_BASE: usize = 9;
    let (batch, remaining_accounts) = meta
        .inner_instructions
        .iter()
        .flatten()
        .filter(|inner| *inner.instruction.program_id(account_keys) == fixture.host_program_id)
        .find_map(|inner| {
            let batch = decode_fhe_execute_args(&inner.instruction.data)?;
            let remaining = inner.instruction.accounts[FHE_EXECUTE_REMAINING_BASE..]
                .iter()
                .map(|index| account_keys[usize::from(*index)].to_bytes())
                .collect::<Vec<_>>();
            Some((batch, remaining))
        })
        .expect("confidential transfer must CPI into zama-host fhe_execute");
    let clock = fixture.svm.get_sysvar::<Clock>();
    reconstruct_fhe_execute_records(
        &batch,
        fixture.compute_signer.to_bytes(),
        &remaining_accounts,
        &[],
        &ReconstructContext {
            chain_id: host::SOLANA_POC_CHAIN_ID,
            previous_bank_hash: PREVIOUS_BANK_HASH,
            unix_timestamp: clock.unix_timestamp,
        },
    )
    .expect("the token's accepted transfer batch must reconstruct")
}

struct TokenFixture {
    svm: LiteSVM,
    host_program_id: Pubkey,
    host_config: Pubkey,
    token_program_id: Pubkey,
    alice: Keypair,
    mint: Keypair,
    compute_signer: Pubkey,
    alice_token: Pubkey,
    bob_token: Pubkey,
    alice_initial: [u8; 32],
    bob_initial: [u8; 32],
    alice_current_compute_acl: Pubkey,
    bob_current_compute_acl: Pubkey,
}

#[derive(Clone, Copy)]
struct TransferOutputAccounts {
    alice: Pubkey,
    bob: Pubkey,
    transferred: Pubkey,
}

fn seed_host_config(svm: &mut LiteSVM, program_id: Pubkey, admin: Pubkey) -> Pubkey {
    let (host_config, bump) = Pubkey::find_program_address(&[host::HOST_CONFIG_SEED], &program_id);
    svm.set_account(
        host_config,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(HostConfig {
                admin,
                chain_id: host::SOLANA_POC_CHAIN_ID,
                // Coprocessor `fromExternal` verifier: transfers bind the amount via a
                // secp256k1 EIP-712 attestation that fhe_execute re-verifies in-execution.
                gateway_chain_id: SECP_GATEWAY_CHAIN_ID,
                input_verification_contract: INPUT_VERIFICATION_CONTRACT,
                coprocessor_signers: host::pack_coprocessor_signers(&[secp_evm_address(
                    &coprocessor_signing_key(),
                )]),
                coprocessor_signer_count: 1,
                coprocessor_threshold: 1,
                decryption_contract: [0u8; 20],
                current_kms_context_id: 0,
                paused: false,
                grant_deny_list_enabled: false,
                max_hcu_per_tx: u64::MAX,
                max_hcu_depth_per_tx: u64::MAX,
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
    let mut clock = svm.get_sysvar::<Clock>();
    clock.slot = FIXTURE_SLOT;
    clock.unix_timestamp = FIXTURE_UNIX_TIMESTAMP;
    svm.set_sysvar(&clock);
    svm.set_sysvar(&SlotHashes::new(&[(
        FIXTURE_SLOT - 1,
        Hash::new_from_array(PREVIOUS_BANK_HASH),
    )]));
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
    let host_config = seed_host_config(&mut svm, host_program_id, alice.pubkey());
    create_spl_mint(&mut svm, &alice, &underlying_mint, 6);
    let compute_signer = token::compute_signer_address(mint.pubkey()).0;
    let total_supply_authority = token::total_supply_authority_address(mint.pubkey()).0;
    let total_supply_encrypted_value =
        token::total_supply_encrypted_value_address(mint.pubkey(), total_supply_authority).0;

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
                total_supply_encrypted_value,
                zama_event_authority: event_authority(host_program_id),
                zama_program: host_program_id,
                host_config,
                token_program: spl_token::id(),
                system_program: system_program::ID,
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
        token::balance_encrypted_value_address(mint.pubkey(), alice_token).0;
    let bob_current_compute_acl =
        token::balance_encrypted_value_address(mint.pubkey(), bob_token).0;

    initialize_token_account(
        &mut svm,
        &alice,
        alice.pubkey(),
        TokenAccountInit {
            token_program_id,
            host_program_id,
            host_config,
            mint: mint.pubkey(),
            token_account: alice_token,
            compute_signer,
            balance_encrypted_value: alice_current_compute_acl,
        },
    );
    initialize_token_account(
        &mut svm,
        &bob,
        bob.pubkey(),
        TokenAccountInit {
            token_program_id,
            host_program_id,
            host_config,
            mint: mint.pubkey(),
            token_account: bob_token,
            compute_signer,
            balance_encrypted_value: bob_current_compute_acl,
        },
    );
    let alice_initial = read_encrypted_value(&svm, alice_current_compute_acl)
        .expect("expected Alice initial ACL")
        .current_handle;
    let bob_initial = read_encrypted_value(&svm, bob_current_compute_acl)
        .expect("expected Bob initial ACL")
        .current_handle;

    TokenFixture {
        svm,
        host_program_id,
        host_config,
        token_program_id,
        alice,
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
    balance_encrypted_value: Pubkey,
}

fn initialize_token_account(
    svm: &mut LiteSVM,
    payer: &Keypair,
    owner: Pubkey,
    init: TokenAccountInit,
) {
    send(
        svm,
        payer,
        Instruction {
            program_id: init.token_program_id,
            accounts: token::accounts::InitializeTokenAccount {
                payer: payer.pubkey(),
                owner,
                mint: init.mint,
                compute_signer: init.compute_signer,
                token_account: init.token_account,
                balance_encrypted_value: init.balance_encrypted_value,
                zama_event_authority: event_authority(init.host_program_id),
                zama_program: init.host_program_id,
                host_config: init.host_config,
                system_program: system_program::ID,
                hcu_block_meter: None,
                hcu_trusted_app_record: None,
                event_authority: event_authority(init.token_program_id),
                program: init.token_program_id,
            }
            .to_account_metas(None),
            data: token::instruction::InitializeTokenAccount {}.data(),
        },
    );
}

/// Confidential-token `EncryptedValue` encrypted value accounts are addressed by stable app-level
/// keys (mint, token account, label) rather than a per-transfer nonce sequence
/// under RFC-024, so the same balance/transferred-amount accounts are reused
/// across every transfer.
fn transfer_output_accounts(fixture: &TokenFixture) -> TransferOutputAccounts {
    TransferOutputAccounts {
        alice: fixture.alice_current_compute_acl,
        bob: fixture.bob_current_compute_acl,
        transferred: token::encrypted_value_address(
            fixture.mint.pubkey(),
            fixture.alice_token,
            token::encrypted_transferred_amount_label(),
        )
        .0,
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
            // unrestricted cap means None/None here.
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            owner: fixture.alice.pubkey(),
            payer: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            compute_signer: fixture.compute_signer,
            from_balance_value: output.alice,
            to_balance_value: output.bob,
            transferred_amount_value: output.transferred,
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
            // (user = owner, contract = mint compute-signer PDA), re-verified in fhe_execute.
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
/// registered coprocessor signer set configured on the fixture's `host_config`.
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
/// `contract == mint compute-signer PDA`; the host re-verifies this signature in-execution.
fn amount_attestation_for(
    amount_handle: [u8; 32],
    user: Pubkey,
    contract: Pubkey,
) -> host::CoprocessorInputAttestation {
    let key = coprocessor_signing_key();
    let ct_handles = vec![amount_handle];
    let contract_chain_id = host::SOLANA_POC_CHAIN_ID;
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

async fn seed_real_ciphertexts(
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
fn read_encrypted_value(svm: &LiteSVM, address: Pubkey) -> Option<EncryptedValue> {
    let raw_account = svm.get_account(&address)?;
    let mut data = raw_account.data.as_slice();
    EncryptedValue::try_deserialize(&mut data).ok()
}

fn current_handle(svm: &LiteSVM, address: Pubkey) -> [u8; 32] {
    read_encrypted_value(svm, address)
        .expect("expected EncryptedValue account")
        .current_handle
}

fn serialized_account<T: AccountSerialize>(account: T) -> Vec<u8> {
    let mut data = Vec::new();
    account.try_serialize(&mut data).unwrap();
    data
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

fn balance_handle(seed: u8) -> [u8; 32] {
    typed_handle(seed, BALANCE_FHE_TYPE)
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
    // Confidential transfer's real euint64 FHE ops exceed the default 200k CU limit
    // (mollusk measures ~258k); raise it like a real client would.
    let ixs = [set_compute_unit_limit_ix(400_000), ix];
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
