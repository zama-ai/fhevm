//! Slow manual boundary coverage for Solana token plans and the real TFHE worker.

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
        insert_solana_events, solana_transaction_id, SolanaBlockMeta, SolanaHostEvent,
    },
    solana_reconstruct::{decode_fhe_eval_args, reconstruct_fhe_eval_events, ReconstructContext},
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
use time::{Date, Month, PrimitiveDateTime, Time};
use zama_host::{EncryptedValue, HostConfig};

use crate::tests::{
    event_helpers::{decrypt_handles, setup_event_harness, wait_until_computed},
    utils::latest_db_key,
};

use confidential_token as token;
use zama_host as host;

const BALANCE_FHE_TYPE: u8 = 5;
const SECP_GATEWAY_CHAIN_ID: u64 = 31337;
const INPUT_VERIFICATION_CONTRACT: [u8; 20] = [0xCD; 20];
const PREVIOUS_BANK_HASH: [u8; 32] = [0x42; 32];
type SeededCiphertext = ([u8; 32], i16, Vec<u8>);

#[tokio::test]
#[serial(db)]
#[ignore = "runs LiteSVM plus the real TFHE worker against a disposable Postgres DB"]
async fn confidential_transfer_reconstructs_computes_and_decrypts(
) -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let mut fixture = token_fixture();
    let amount_handle = balance_handle(0x09);

    seed_real_ciphertexts(
        &harness.pool,
        &[
            (fixture.alice_initial, 125),
            (fixture.bob_initial, 20),
            (amount_handle, 100),
        ],
    )
    .await?;

    let outputs = transfer_output_accounts(&fixture);
    let transfer = transfer_ix(&fixture, outputs, amount_handle);
    let (meta, account_keys, signature) =
        send_with_meta(&mut fixture.svm, &fixture.alice, transfer);
    let alice_handle = current_handle(&fixture.svm, outputs.alice);
    let bob_handle = current_handle(&fixture.svm, outputs.bob);
    let transferred_handle = current_handle(&fixture.svm, outputs.transferred);

    let events = reconstruct_transfer_events(&fixture, &meta, &account_keys);
    assert_eq!(
        events.len(),
        5,
        "token transfer must remain a five-step plan"
    );
    assert!(matches!(
        &events[2],
        SolanaHostEvent::FheTernaryOp(event) if event.result == alice_handle
    ));
    assert!(matches!(
        &events[3],
        SolanaHostEvent::FheBinaryOp(event) if event.result == transferred_handle
    ));
    assert!(matches!(
        &events[4],
        SolanaHostEvent::FheBinaryOp(event) if event.result == bob_handle
    ));

    let block = SolanaBlockMeta {
        block_number: 1,
        block_hash: [1; 32],
        parent_hash: [0; 32],
        block_timestamp: PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 11)?,
            Time::MIDNIGHT,
        ),
    };
    let mut db_tx = harness.listener_db.new_transaction().await?;
    let stats = insert_solana_events(
        &harness.listener_db,
        &mut db_tx,
        events,
        solana_transaction_id(signature.as_ref()),
        block,
    )
    .await?;
    db_tx.commit().await?;
    assert_eq!(stats.tfhe_events, 5);

    wait_until_computed(&harness.app).await?;
    let decrypted = decrypt_handles(
        &harness.pool,
        &[Handle::from(alice_handle), Handle::from(bob_handle)],
    )
    .await?;
    assert_eq!(decrypted[0].value, "25");
    assert_eq!(decrypted[1].value, "120");
    Ok(())
}

fn reconstruct_transfer_events(
    fixture: &TokenFixture,
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
) -> Vec<SolanaHostEvent> {
    let plan = meta
        .inner_instructions
        .iter()
        .flatten()
        .filter(|inner| *inner.instruction.program_id(account_keys) == fixture.host_program_id)
        .find_map(|inner| decode_fhe_eval_args(&inner.instruction.data))
        .expect("confidential transfer must CPI into zama-host fhe_eval");
    let clock = fixture.svm.get_sysvar::<Clock>();
    reconstruct_fhe_eval_events(
        &plan,
        fixture.compute_signer.to_bytes(),
        &ReconstructContext {
            chain_id: host::SOLANA_POC_CHAIN_ID,
            previous_bank_hash: PREVIOUS_BANK_HASH,
            unix_timestamp: clock.unix_timestamp,
        },
    )
    .expect("the token's accepted transfer plan must reconstruct")
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
                // secp256k1 EIP-712 attestation that fhe_eval re-verifies in-frame.
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
    clock.slot = 100;
    svm.set_sysvar(&clock);
    svm.set_sysvar(&SlotHashes::new(&[(
        99,
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
        TokenAccountInit {
            token_program_id,
            host_program_id,
            host_config,
            mint: mint.pubkey(),
            token_account: alice_token,
            compute_signer,
            balance_encrypted_value: alice_current_compute_acl,
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
            balance_encrypted_value: bob_current_compute_acl,
            initial_balance: 0,
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
            data: token::instruction::InitializeTokenAccount {
                initial_balance: init.initial_balance,
            }
            .data(),
        },
    );
}

/// Confidential-token `EncryptedValue` lineages are addressed by stable app-level
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
            token::transferred_amount_label(),
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
/// `contract == mint compute-signer PDA`; the host re-verifies this signature in-frame.
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
