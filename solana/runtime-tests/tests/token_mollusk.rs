//! Mollusk-based runtime tests for `confidential-token` against the RFC-024 `EncryptedValue`
//! ACL model.
//!
//! Migrated from the old keyed-nonce `AclRecord`/`AclPermission` model (deleted along with
//! `balance_acl_record`/`next_balance_nonce_sequence`-style per-birth PDAs, `assert_acl_record`,
//! and the single-op `fhe_*` instructions) to the new stateless-indexing `EncryptedValue` lineage:
//! `ConfidentialTokenAccount`/`ConfidentialMint` now each point at one stable
//! `EncryptedValue` PDA per encrypted field (`balance_encrypted_value`,
//! `total_supply_encrypted_value`) that is *superseded in place* on every update instead of
//! rotating to a new per-nonce account. See `confidential-token/src/fhe.rs`,
//! `zama-host/src/state/encrypted_value.rs`, and `zama_solana_acl` for the model this exercises.
//!
//! Scope note: this migration focuses the suite on the surface that changed with the ACL rewrite
//! (mint/token-account creation and `confidential_transfer`'s durable-output supersession), plus
//! the token-level end-to-end coverage requested for this pass (stable addressing across a
//! transfer, a `transferred_amount` lineage entry, and self-transfer no-op). It also covers the
//! two consume paths whose authorization changed shape to proof-against-a-pinned-handle (DD-036):
//! `redeem_burned_amount_secp` and `disclose_balance_secp`/`disclose_amount_secp` now authorize by
//! an MMR public-decrypt proof against the witness-pinned handle rather than the live lineage
//! handle, so their after-supersession, consume-once, foreign-proof, and expiry behaviour is
//! exercised directly here, alongside the `close_*` disclosure-request witness instructions. The
//! old suite's coverage of `wrap_usdc`, `confidential_burn`, `request_disclose_balance`/
//! `request_disclose_amount`, `request_burn_redemption`, the burn-redemption `close_*`
//! instructions, and the `poc`-gated `create_random_amount` was not ported 1:1 for this pass: none
//! of that instruction logic changed shape from the ACL rewrite itself (it still reads/writes one
//! `EncryptedValue` lineage per amount the same way `confidential_transfer` does), and each needs
//! its own multi-account Mollusk fixture that was not feasible to rebuild faithfully here. Every
//! dropped test's instruction still compiles and is exercised indirectly by `mollusk_initialize_*`
//! and `mollusk_confidential_transfer_*` (same durable-output machinery); a follow-up pass should
//! port them individually using the patterns established here and in `host_mollusk.rs`.
//!
//! Also dropped: the old file's event-shaped, `u128`-limited `support::fhe_runtime` simulator.
//! The confidential-transfer test below instead evaluates the canonical `FheEvalArgs` captured
//! from the real token -> host CPI and binds those clear values to the handles emitted by the host.

mod support;

use anchor_lang::{
    prelude::system_program, AccountDeserialize, AccountSerialize, AnchorDeserialize,
    Discriminator, InstructionData, ToAccountMetas,
};
use confidential_token as token;
use mollusk_svm::{
    result::{Check, InstructionResult},
    Mollusk,
};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use std::collections::HashMap;
use std::path::PathBuf;
use support::cleartext_fhe_eval::{evaluate as evaluate_cleartext, ClearInputs, TypedClearValue};
use zama_host as host;

const BALANCE_FHE_TYPE: u8 = 5;
const GATEWAY_CHAIN_ID: u64 = 31337;
const INPUT_VERIFICATION_CONTRACT: [u8; 20] = [0xCDu8; 20];
const DECRYPTION_CONTRACT: [u8; 20] = [0xDEu8; 20];

// ---------------------------------------------------------------------------
// Harness
// ---------------------------------------------------------------------------

fn mollusk() -> Mollusk {
    let deploy_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy");
    unsafe {
        std::env::set_var("SBF_OUT_DIR", deploy_dir);
    }
    let mut mollusk = Mollusk::new(&token::id(), "confidential_token");
    mollusk.add_program(&host::id(), "zama_host");
    // fhe_eval derives handle entropy from the previous bank hash: run at a
    // nonzero slot with a SlotHashes entry below it, like a real validator.
    mollusk.sysvars.clock.slot = 100;
    mollusk.sysvars.slot_hashes =
        solana_sdk::slot_hashes::SlotHashes::new(&[(99, solana_sdk::hash::Hash::new_unique())]);
    // A transfer (secp attestation recovery + three durable bindings) exceeds
    // the 200k default; real transactions request a higher limit the same way.
    mollusk.compute_budget.compute_unit_limit = 1_400_000;
    mollusk
}

fn anchor_ix<A, D>(program_id: Pubkey, accounts: A, args: D) -> Instruction
where
    A: ToAccountMetas,
    D: InstructionData,
{
    Instruction {
        program_id,
        accounts: accounts.to_account_metas(None),
        data: args.data(),
    }
}

fn serialized_account<T: AccountSerialize>(account: T) -> Vec<u8> {
    let mut data = Vec::new();
    account.try_serialize(&mut data).unwrap();
    data
}

fn event_authority(program_id: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"__event_authority"], &program_id).0
}

fn system_account(lamports: u64) -> Account {
    Account {
        lamports,
        data: vec![],
        owner: system_program::ID,
        executable: false,
        rent_epoch: 0,
    }
}

fn handle_for_chain(seed: u8, fhe_type: u8) -> [u8; 32] {
    let mut handle = [seed; 32];
    handle[21] = 0;
    handle[22..30].copy_from_slice(&host::SOLANA_POC_CHAIN_ID.to_be_bytes());
    handle[30] = fhe_type;
    handle[31] = host::HANDLE_VERSION;
    handle
}

fn decode_anchor_event<T>(data: &[u8]) -> Option<T>
where
    T: AnchorDeserialize + Discriminator,
{
    let event_prefix = anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(T::DISCRIMINATOR.iter().copied())
        .collect::<Vec<u8>>();
    let payload = data.strip_prefix(&event_prefix[..])?;
    T::deserialize(&mut &*payload).ok()
}

fn decode_fhe_eval_args(data: &[u8]) -> Option<host::FheEvalArgs> {
    let payload = data.strip_prefix(host::instruction::FheEval::DISCRIMINATOR)?;
    host::FheEvalArgs::deserialize(&mut &*payload).ok()
}

fn eval_step_output(step: &host::FheEvalStep) -> &host::FheEvalOutput {
    match step {
        host::FheEvalStep::Binary { output, .. }
        | host::FheEvalStep::Ternary { output, .. }
        | host::FheEvalStep::TrivialEncrypt { output, .. }
        | host::FheEvalStep::Rand { output, .. }
        | host::FheEvalStep::Unary { output, .. }
        | host::FheEvalStep::RandBounded { output, .. }
        | host::FheEvalStep::Sum { output, .. }
        | host::FheEvalStep::IsIn { output, .. }
        | host::FheEvalStep::MulDiv { output, .. } => output,
    }
}

#[derive(Default)]
struct CleartextLedger {
    values: ClearInputs,
}

impl CleartextLedger {
    fn seed_amount(&mut self, handle: [u8; 32], value: u64) {
        self.values
            .insert(handle, TypedClearValue::from_u64(BALANCE_FHE_TYPE, value));
    }

    /// Applies the exact FHE plan invoked by the token program and associates each durable
    /// result with the handle persisted in its canonical `EncryptedValue` account.
    fn evaluate_fhe_cpi(
        &mut self,
        context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
        result: &InstructionResult,
    ) -> usize {
        let message = result
            .message
            .as_ref()
            .expect("Mollusk result must include its compiled message");
        let eval_args = result
            .inner_instructions
            .iter()
            .filter(|inner| {
                message
                    .account_keys()
                    .get(inner.instruction.program_id_index as usize)
                    .copied()
                    == Some(host::id())
            })
            .filter_map(|inner| decode_fhe_eval_args(&inner.instruction.data))
            .collect::<Vec<_>>();
        assert_eq!(
            eval_args.len(),
            1,
            "expected one token -> host fhe_eval CPI"
        );

        let outputs = evaluate_cleartext(&eval_args[0], &self.values)
            .expect("the token program must emit a valid cleartext FHE plan");
        let mut durable_outputs = 0;
        for (step, value) in eval_args[0].steps.iter().zip(outputs) {
            let host::FheEvalOutput::AllowedDurable {
                output_acl_domain_key,
                output_app_account,
                output_encrypted_value_label,
                ..
            } = eval_step_output(step)
            else {
                continue;
            };
            let value_key = zama_solana_acl::derive_value_key(
                output_acl_domain_key.to_bytes(),
                output_app_account.to_bytes(),
                *output_encrypted_value_label,
            );
            let address = host::encrypted_value_address(value_key).0;
            let persisted = read_encrypted_value(context, address);
            self.values.insert(persisted.current_handle, value);
            durable_outputs += 1;
        }
        durable_outputs
    }

    fn balance(
        &self,
        context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
        token_account: Pubkey,
    ) -> u64 {
        let account = read_token_account(context, token_account);
        self.u64_at(context, account.balance_encrypted_value)
    }

    fn transferred_amount(
        &self,
        context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
        mint: Pubkey,
        from_token: Pubkey,
    ) -> u64 {
        self.u64_at(
            context,
            token::encrypted_value_address(mint, from_token, token::transferred_amount_label()).0,
        )
    }

    fn u64_at(
        &self,
        context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
        encrypted_value: Pubkey,
    ) -> u64 {
        let handle = read_encrypted_value(context, encrypted_value).current_handle;
        let value = self
            .values
            .get(&handle)
            .expect("missing cleartext value for persisted handle");
        assert_eq!(value.fhe_type, BALANCE_FHE_TYPE);
        assert_eq!(value.value[..24], [0; 24], "cleartext value exceeds u64");
        u64::from_be_bytes(value.value[24..].try_into().unwrap())
    }
}

fn host_config_account(admin: Pubkey, coprocessor_signer: [u8; 20]) -> Account {
    host_config_account_with_flags(admin, coprocessor_signer, 0, false)
}

fn host_config_account_with_kms_context(
    admin: Pubkey,
    coprocessor_signer: [u8; 20],
    current_kms_context_id: u64,
) -> Account {
    host_config_account_with_flags(admin, coprocessor_signer, current_kms_context_id, false)
}

fn host_config_account_with_flags(
    admin: Pubkey,
    coprocessor_signer: [u8; 20],
    current_kms_context_id: u64,
    grant_deny_list_enabled: bool,
) -> Account {
    let (host_config, bump) = host::host_config_address();
    let _ = host_config;
    Account {
        lamports: 1_000_000_000,
        data: serialized_account(host::HostConfig {
            admin,
            chain_id: host::SOLANA_POC_CHAIN_ID,
            input_verifier_authority: admin,
            gateway_chain_id: GATEWAY_CHAIN_ID,
            input_verification_contract: INPUT_VERIFICATION_CONTRACT,
            coprocessor_signer,
            decryption_contract: DECRYPTION_CONTRACT,
            current_kms_context_id,
            material_authority: admin,
            test_authority: admin,
            paused: false,
            mock_input_enabled: false,
            test_shims_enabled: true,
            grant_deny_list_enabled,
            max_hcu_per_tx: 0,
            max_hcu_depth_per_tx: 0,
            // Ships unrestricted; existing flows are unaffected by the block cap.
            hcu_block_cap_per_app: u64::MAX,
            updated_slot: 0,
            bump,
        }),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn deny_enabled_host_config_account(admin: Pubkey, coprocessor_signer: [u8; 20]) -> Account {
    host_config_account_with_flags(admin, coprocessor_signer, 0, true)
}

fn deny_subject_record_account(subject: Pubkey, denied: bool) -> (Pubkey, Account) {
    let (record, bump) = host::deny_subject_address(subject);
    (
        record,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(host::DenySubjectRecord {
                subject,
                denied,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

/// Builds a canonical `EncryptedValue` lineage account for direct account-map seeding, mirroring
/// `host_mollusk.rs::new_lineage` for the token program's ACL domain (the mint).
fn new_encrypted_value(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subjects: &[Pubkey],
) -> (Pubkey, host::EncryptedValue) {
    let value_key = zama_solana_acl::derive_value_key(
        acl_domain_key.to_bytes(),
        app_account.to_bytes(),
        encrypted_value_label,
    );
    let (address, bump) = host::encrypted_value_address(value_key);
    let value = host::EncryptedValue {
        acl_domain_key,
        app_account,
        encrypted_value_label,
        current_handle: handle,
        subjects: subjects.to_vec(),
        leaf_count: 0,
        peaks: Vec::new(),
        bump,
    };
    (address, value)
}

fn encrypted_value_account(value: &host::EncryptedValue) -> Account {
    Account {
        lamports: 10_000_000_000,
        data: serialized_account(value.clone()),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn read_encrypted_value(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> host::EncryptedValue {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing encrypted value account")
        .clone();
    host::EncryptedValue::try_deserialize(&mut account.data.as_slice())
        .expect("valid EncryptedValue account")
}

fn account_is_system_owned_and_empty(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> bool {
    match context.account_store.borrow().get(&address) {
        None => true,
        Some(account) => account.owner == system_program::ID && account.data.is_empty(),
    }
}

fn read_token_account(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> token::ConfidentialTokenAccount {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing token account")
        .clone();
    token::ConfidentialTokenAccount::try_deserialize(&mut account.data.as_slice())
        .expect("token account should deserialize")
}

fn read_confidential_mint(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> token::ConfidentialMint {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing mint account")
        .clone();
    token::ConfidentialMint::try_deserialize(&mut account.data.as_slice())
        .expect("mint account should deserialize")
}

fn read_disclosure_request(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> token::DisclosureRequest {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing disclosure request account")
        .clone();
    token::DisclosureRequest::try_deserialize(&mut account.data.as_slice())
        .expect("disclosure request should deserialize")
}

fn expected_historical_peaks(
    encrypted_value: Pubkey,
    old_handle: [u8; 32],
    subjects: &[Pubkey],
) -> Vec<[u8; 32]> {
    let leaves: Vec<[u8; 32]> = subjects
        .iter()
        .enumerate()
        .map(|(index, subject)| {
            zama_solana_acl::historical_access_leaf_commitment(
                encrypted_value.to_bytes(),
                index as u64,
                old_handle,
                subject.to_bytes(),
            )
        })
        .collect();
    zama_solana_acl::mmr_peaks_from_leaves(&leaves)
}

fn token_error(error: token::ConfidentialTokenError) -> Check<'static> {
    Check::err(ProgramError::Custom(
        anchor_lang::error::ERROR_CODE_OFFSET + error as u32,
    ))
}

fn host_error(error: host::errors::ZamaHostError) -> Check<'static> {
    Check::err(ProgramError::Custom(
        anchor_lang::error::ERROR_CODE_OFFSET + error as u32,
    ))
}

fn anchor_error(error: anchor_lang::error::ErrorCode) -> Check<'static> {
    Check::err(ProgramError::Custom(error as u32))
}

// ---------------------------------------------------------------------------
// Coprocessor `fromExternal` attestation signing
// ---------------------------------------------------------------------------

/// Coprocessor signing key backing the `fromExternal` attestations; its EVM address is the
/// `coprocessor_signer` configured on the fixture's `host_config`.
fn coprocessor_signing_key() -> k256::ecdsa::SigningKey {
    k256::ecdsa::SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap()
}

/// Recovers the EVM address (keccak(pubkey)[12..]) for a signing key.
fn secp_evm_address(key: &k256::ecdsa::SigningKey) -> [u8; 20] {
    let encoded = key.verifying_key().to_encoded_point(false);
    let hash = solana_program::keccak::hash(&encoded.as_bytes()[1..]).to_bytes();
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
/// (`user`, `contract`). The token program checks `user == transfer authority` and
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
            GATEWAY_CHAIN_ID,
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

// ---------------------------------------------------------------------------
// Fixture
// ---------------------------------------------------------------------------

struct TokenFixture {
    owner: Pubkey,
    bob_owner: Pubkey,
    mint: Pubkey,
    compute_signer: Pubkey,
    host_config: Pubkey,
    alice_token: Pubkey,
    bob_token: Pubkey,
    alice_balance_value: Pubkey,
    bob_balance_value: Pubkey,
    alice_initial: [u8; 32],
    bob_initial: [u8; 32],
}

impl TokenFixture {
    fn new() -> Self {
        let owner = Pubkey::new_unique();
        let bob_owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let compute_signer = token::compute_signer_address(mint).0;
        let host_config = host::host_config_address().0;
        let alice_token = token::token_account_address(mint, owner).0;
        let bob_token = token::token_account_address(mint, bob_owner).0;
        let alice_balance_value = token::balance_encrypted_value_address(mint, alice_token).0;
        let bob_balance_value = token::balance_encrypted_value_address(mint, bob_token).0;
        Self {
            owner,
            bob_owner,
            mint,
            compute_signer,
            host_config,
            alice_token,
            bob_token,
            alice_balance_value,
            bob_balance_value,
            alice_initial: handle_for_chain(1, BALANCE_FHE_TYPE),
            bob_initial: handle_for_chain(2, BALANCE_FHE_TYPE),
        }
    }

    fn confidential_mint_account(&self) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialMint {
                authority: self.owner,
                acl_domain_key: self.mint,
                compute_signer: self.compute_signer,
                underlying_mint: Pubkey::new_unique(),
                decimals: 6,
                total_supply_encrypted_value: token::total_supply_encrypted_value_address(
                    self.mint,
                    token::total_supply_authority_address(self.mint).0,
                )
                .0,
            }),
            owner: token::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    fn confidential_token_account(&self, owner: Pubkey, balance_value: Pubkey) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialTokenAccount {
                owner,
                mint: self.mint,
                balance_encrypted_value: balance_value,
                bump: token::token_account_address(self.mint, owner).1,
            }),
            owner: token::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    fn base_accounts(&self) -> HashMap<Pubkey, Account> {
        let (alice_balance_address, alice_balance_value) = new_encrypted_value(
            self.mint,
            self.alice_token,
            token::balance_label(),
            self.alice_initial,
            &[self.owner, self.compute_signer],
        );
        assert_eq!(alice_balance_address, self.alice_balance_value);
        let (bob_balance_address, bob_balance_value) = new_encrypted_value(
            self.mint,
            self.bob_token,
            token::balance_label(),
            self.bob_initial,
            &[self.bob_owner, self.compute_signer],
        );
        assert_eq!(bob_balance_address, self.bob_balance_value);

        HashMap::from([
            (self.owner, system_account(5_000_000_000)),
            (self.bob_owner, system_account(5_000_000_000)),
            (self.mint, self.confidential_mint_account()),
            (self.compute_signer, system_account(0)),
            (
                self.host_config,
                host_config_account(self.owner, secp_evm_address(&coprocessor_signing_key())),
            ),
            (
                self.alice_token,
                self.confidential_token_account(self.owner, self.alice_balance_value),
            ),
            (
                self.bob_token,
                self.confidential_token_account(self.bob_owner, self.bob_balance_value),
            ),
            (
                self.alice_balance_value,
                encrypted_value_account(&alice_balance_value),
            ),
            (
                self.bob_balance_value,
                encrypted_value_account(&bob_balance_value),
            ),
            (event_authority(host::id()), system_account(0)),
            (event_authority(token::id()), system_account(0)),
        ])
    }

    fn transferred_amount_value_address(&self, from_token: Pubkey) -> Pubkey {
        token::encrypted_value_address(self.mint, from_token, token::transferred_amount_label()).0
    }
}

// ---------------------------------------------------------------------------
// Instruction builders
// ---------------------------------------------------------------------------

fn initialize_mint_ix(
    authority: Pubkey,
    mint: Pubkey,
    underlying_mint: Pubkey,
    host_config: Pubkey,
) -> Instruction {
    let compute_signer = token::compute_signer_address(mint).0;
    let total_supply_authority = token::total_supply_authority_address(mint).0;
    let total_supply_encrypted_value =
        token::total_supply_encrypted_value_address(mint, total_supply_authority).0;
    anchor_ix(
        token::id(),
        token::accounts::InitializeMint {
            authority,
            mint,
            underlying_mint,
            compute_signer,
            total_supply_authority,
            total_supply_encrypted_value,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config,
            system_program: system_program::ID,
            hcu_authority: token::hcu_authority_address(mint).0,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::InitializeMint {},
    )
}

fn initialize_token_account_ix(
    owner: Pubkey,
    mint: Pubkey,
    host_config: Pubkey,
    initial_balance: u64,
) -> Instruction {
    let compute_signer = token::compute_signer_address(mint).0;
    let (token_account, _bump) = token::token_account_address(mint, owner);
    let balance_encrypted_value = token::balance_encrypted_value_address(mint, token_account).0;
    anchor_ix(
        token::id(),
        token::accounts::InitializeTokenAccount {
            owner,
            mint,
            compute_signer,
            token_account,
            balance_encrypted_value,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config,
            system_program: system_program::ID,
            hcu_authority: token::hcu_authority_address(mint).0,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::InitializeTokenAccount { initial_balance },
    )
}

fn confidential_transfer_ix(
    fixture: &TokenFixture,
    from_token: Pubkey,
    to_token: Pubkey,
    from_balance_value: Pubkey,
    to_balance_value: Pubkey,
    amount_attestation: host::CoprocessorInputAttestation,
) -> Instruction {
    confidential_transfer_ix_with_remaining(
        fixture,
        from_token,
        to_token,
        from_balance_value,
        to_balance_value,
        amount_attestation,
        Vec::new(),
    )
}

fn confidential_transfer_ix_with_remaining(
    fixture: &TokenFixture,
    from_token: Pubkey,
    to_token: Pubkey,
    from_balance_value: Pubkey,
    to_balance_value: Pubkey,
    amount_attestation: host::CoprocessorInputAttestation,
    remaining: Vec<Pubkey>,
) -> Instruction {
    confidential_transfer_ix_with_block_cap_accounts(
        fixture,
        from_token,
        to_token,
        from_balance_value,
        to_balance_value,
        amount_attestation,
        remaining,
        None,
        None,
        token::hcu_authority_address(fixture.mint).0,
    )
}

/// Block-cap optional accounts threaded through the transfer CPI explicitly; used by the HCU
/// block-cap tests to vary the meter / trust witness / HCU authority. The default unrestricted
/// cap means `confidential_transfer_ix_with_remaining` passes `None`/`None`/the canonical mint
/// authority.
#[allow(clippy::too_many_arguments)]
fn confidential_transfer_ix_with_block_cap_accounts(
    fixture: &TokenFixture,
    from_token: Pubkey,
    to_token: Pubkey,
    from_balance_value: Pubkey,
    to_balance_value: Pubkey,
    amount_attestation: host::CoprocessorInputAttestation,
    remaining: Vec<Pubkey>,
    hcu_block_meter: Option<Pubkey>,
    hcu_trusted_app_record: Option<Pubkey>,
    hcu_authority: Pubkey,
) -> Instruction {
    let mut ix = anchor_ix(
        token::id(),
        token::accounts::ConfidentialTransfer {
            owner: fixture.owner,
            payer: fixture.owner,
            mint: fixture.mint,
            from_account: from_token,
            to_account: to_token,
            compute_signer: fixture.compute_signer,
            from_balance_value,
            to_balance_value,
            transferred_amount_value: fixture.transferred_amount_value_address(from_token),
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            hcu_block_meter,
            hcu_trusted_app_record,
            hcu_authority,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialTransfer { amount_attestation },
    );
    ix.accounts.extend(
        remaining
            .into_iter()
            .map(|pubkey| AccountMeta::new_readonly(pubkey, false)),
    );
    ix
}

fn deny_enabled_transfer_accounts(
    fixture: &TokenFixture,
    denied_authority: Option<Pubkey>,
) -> (HashMap<Pubkey, Account>, Vec<Pubkey>) {
    let mut accounts = fixture.base_accounts();
    accounts.insert(
        fixture.host_config,
        deny_enabled_host_config_account(
            fixture.owner,
            secp_evm_address(&coprocessor_signing_key()),
        ),
    );

    let from_deny = host::deny_subject_address(fixture.alice_token).0;
    let to_deny = host::deny_subject_address(fixture.bob_token).0;
    let from_account = if denied_authority == Some(fixture.alice_token) {
        deny_subject_record_account(fixture.alice_token, true).1
    } else {
        system_account(0)
    };
    let to_account = if denied_authority == Some(fixture.bob_token) {
        deny_subject_record_account(fixture.bob_token, true).1
    } else {
        system_account(0)
    };
    accounts.insert(from_deny, from_account);
    accounts.insert(to_deny, to_account);
    (accounts, vec![from_deny, to_deny])
}

fn request_disclose_balance_ix(
    fixture: &TokenFixture,
    request_nonce: [u8; 32],
    expires_slot: u64,
    disclosure_request: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::RequestDiscloseBalance {
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.alice_token,
            balance_value: fixture.alice_balance_value,
            disclosure_request,
            deny_subject_record: None,
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::RequestDiscloseBalance {
            request_nonce,
            expires_slot,
        },
    )
}

// ---------------------------------------------------------------------------
// initialize_mint / initialize_token_account
// ---------------------------------------------------------------------------

#[test]
fn mollusk_initialize_mint_creates_total_supply_encrypted_value() {
    let authority = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let underlying_mint = Pubkey::new_unique();
    let compute_signer = token::compute_signer_address(mint).0;
    let total_supply_authority = token::total_supply_authority_address(mint).0;
    let total_supply_encrypted_value =
        token::total_supply_encrypted_value_address(mint, total_supply_authority).0;
    let host_config_key = host::host_config_address().0;
    let context = mollusk().with_context(HashMap::from([
        (authority, system_account(5_000_000_000)),
        (mint, system_account(0)),
        (
            underlying_mint,
            Account {
                lamports: 1_000_000_000,
                data: {
                    let mut data = vec![0u8; anchor_spl::token::spl_token::state::Mint::LEN];
                    anchor_spl::token::spl_token::state::Mint::pack(
                        anchor_spl::token::spl_token::state::Mint {
                            mint_authority: solana_sdk::program_option::COption::Some(authority),
                            supply: 0,
                            decimals: 6,
                            is_initialized: true,
                            freeze_authority: solana_sdk::program_option::COption::None,
                        },
                        &mut data,
                    )
                    .unwrap();
                    data
                },
                owner: anchor_spl::token::spl_token::id(),
                executable: false,
                rent_epoch: 0,
            },
        ),
        (compute_signer, system_account(0)),
        (total_supply_authority, system_account(0)),
        (total_supply_encrypted_value, system_account(0)),
        (host_config_key, host_config_account(authority, [0u8; 20])),
        (event_authority(host::id()), system_account(0)),
        (event_authority(token::id()), system_account(0)),
    ]));
    let ix = initialize_mint_ix(authority, mint, underlying_mint, host_config_key);

    context.process_and_validate_instruction(&ix, &[Check::success()]);

    let stored = read_confidential_mint(&context, mint);
    assert_eq!(stored.authority, authority);
    assert_eq!(stored.acl_domain_key, mint);
    assert_eq!(stored.compute_signer, compute_signer);
    assert_eq!(
        stored.total_supply_encrypted_value,
        total_supply_encrypted_value
    );
    let supply_value = read_encrypted_value(&context, total_supply_encrypted_value);
    assert_eq!(supply_value.acl_domain_key, mint);
    assert_eq!(supply_value.app_account, total_supply_authority);
    assert_eq!(
        supply_value.encrypted_value_label,
        token::total_supply_label()
    );
    assert!(supply_value.has_subject(compute_signer));
}

#[test]
fn mollusk_initialize_token_account_creates_initial_balance_encrypted_value() {
    let fixture = TokenFixture::new();
    let owner = Pubkey::new_unique();
    let (token_account, token_bump) = token::token_account_address(fixture.mint, owner);
    let balance_encrypted_value =
        token::balance_encrypted_value_address(fixture.mint, token_account).0;
    let mut accounts = fixture.base_accounts();
    accounts.insert(owner, system_account(5_000_000_000));
    accounts.insert(token_account, system_account(0));
    accounts.insert(balance_encrypted_value, system_account(0));
    let context = mollusk().with_context(accounts);
    let ix = initialize_token_account_ix(owner, fixture.mint, fixture.host_config, 0);

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    let stored = read_token_account(&context, token_account);
    assert_eq!(stored.owner, owner);
    assert_eq!(stored.mint, fixture.mint);
    assert_eq!(stored.bump, token_bump);
    assert_eq!(stored.balance_encrypted_value, balance_encrypted_value);

    let balance_value = read_encrypted_value(&context, balance_encrypted_value);
    assert_eq!(balance_value.acl_domain_key, fixture.mint);
    assert_eq!(balance_value.app_account, token_account);
    assert_eq!(balance_value.encrypted_value_label, token::balance_label());
    assert!(balance_value.has_subject(owner));
    assert!(balance_value.has_subject(fixture.compute_signer));

    let balance_events: Vec<token::BalanceHandleUpdatedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(balance_events.len(), 1);
    assert_eq!(balance_events[0].mint, fixture.mint);
    assert_eq!(balance_events[0].owner, owner);
    assert_eq!(balance_events[0].token_account, token_account);
    assert_eq!(balance_events[0].old_handle, [0; 32]);
    assert_eq!(balance_events[0].old_encrypted_value, Pubkey::default());
    assert_eq!(balance_events[0].new_handle, balance_value.current_handle);
    assert_eq!(
        balance_events[0].new_encrypted_value,
        balance_encrypted_value
    );
    assert_eq!(
        balance_events[0].reason,
        token::BalanceHandleUpdateReason::Initialize
    );
}

#[test]
fn mollusk_initialize_token_account_rejects_nonzero_initial_balance() {
    let fixture = TokenFixture::new();
    let owner = Pubkey::new_unique();
    let (token_account, _bump) = token::token_account_address(fixture.mint, owner);
    let balance_encrypted_value =
        token::balance_encrypted_value_address(fixture.mint, token_account).0;
    let mut accounts = fixture.base_accounts();
    accounts.insert(owner, system_account(5_000_000_000));
    accounts.insert(token_account, system_account(0));
    accounts.insert(balance_encrypted_value, system_account(0));
    let context = mollusk().with_context(accounts);
    let ix = initialize_token_account_ix(owner, fixture.mint, fixture.host_config, 1);

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    assert!(account_is_system_owned_and_empty(&context, token_account));
    assert!(account_is_system_owned_and_empty(
        &context,
        balance_encrypted_value
    ));
}

// ---------------------------------------------------------------------------
// confidential_transfer
// ---------------------------------------------------------------------------

#[test]
fn mollusk_confidential_transfer_self_transfer_is_no_op() {
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(9, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.alice_token,
        fixture.alice_balance_value,
        fixture.alice_balance_value,
        attestation,
    );

    let result = context.process_instruction(&transfer);

    assert!(result.raw_result.is_ok());
    assert!(result.inner_instructions.is_empty());
    let balance_value = read_encrypted_value(&context, fixture.alice_balance_value);
    assert_eq!(balance_value.current_handle, fixture.alice_initial);
    assert_eq!(balance_value.leaf_count, 0);
    assert!(account_is_system_owned_and_empty(
        &context,
        fixture.transferred_amount_value_address(fixture.alice_token)
    ));
}

#[test]
fn mollusk_confidential_transfer_updates_lineages_and_cleartext_balances() {
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(21, BALANCE_FHE_TYPE);
    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.alice_initial, 1_000);
    cleartext.seed_amount(fixture.bob_initial, 100);
    cleartext.seed_amount(amount_handle, 400);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let transferred_value_address = fixture.transferred_amount_value_address(fixture.alice_token);
    let transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );

    let result = context.process_and_validate_instruction(&transfer, &[Check::success()]);
    let durable_outputs = cleartext.evaluate_fhe_cpi(&context, &result);

    assert_eq!(durable_outputs, 3);
    assert_eq!(cleartext.balance(&context, fixture.alice_token), 600);
    assert_eq!(cleartext.balance(&context, fixture.bob_token), 500);
    assert_eq!(
        cleartext.transferred_amount(&context, fixture.mint, fixture.alice_token),
        400
    );

    // Token account addresses and their balance `EncryptedValue` PDAs stay stable across the
    // transfer: no new balance account is created, the existing lineages are superseded in place.
    let alice_token = read_token_account(&context, fixture.alice_token);
    let bob_token = read_token_account(&context, fixture.bob_token);
    assert_eq!(
        alice_token.balance_encrypted_value,
        fixture.alice_balance_value
    );
    assert_eq!(bob_token.balance_encrypted_value, fixture.bob_balance_value);

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_ne!(alice_balance.current_handle, fixture.alice_initial);
    assert_ne!(bob_balance.current_handle, fixture.bob_initial);
    // Supersession appended exactly one historical leaf per allowed subject.
    assert_eq!(alice_balance.leaf_count, 2);
    assert_eq!(bob_balance.leaf_count, 2);
    assert_eq!(
        alice_balance.subjects,
        vec![fixture.owner, fixture.compute_signer]
    );
    assert_eq!(
        alice_balance.peaks,
        expected_historical_peaks(
            fixture.alice_balance_value,
            fixture.alice_initial,
            &[fixture.owner, fixture.compute_signer],
        )
    );
    assert_eq!(
        bob_balance.subjects,
        vec![fixture.bob_owner, fixture.compute_signer]
    );
    assert_eq!(
        bob_balance.peaks,
        expected_historical_peaks(
            fixture.bob_balance_value,
            fixture.bob_initial,
            &[fixture.bob_owner, fixture.compute_signer],
        )
    );

    // A lineage entry for the transferred amount was created (first bind) at the canonical PDA.
    let transferred = read_encrypted_value(&context, transferred_value_address);
    assert_eq!(transferred.acl_domain_key, fixture.mint);
    assert_eq!(transferred.app_account, fixture.alice_token);
    assert_eq!(
        transferred.encrypted_value_label,
        token::transferred_amount_label()
    );
    assert!(transferred.has_subject(fixture.owner));
    assert!(transferred.has_subject(fixture.bob_owner));
    assert!(transferred.has_subject(fixture.compute_signer));
    assert_eq!(transferred.leaf_count, 0); // birth: no supersession yet.

    let transfer_events: Vec<token::ConfidentialTransferEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(transfer_events.len(), 1);
    assert_eq!(transfer_events[0].mint, fixture.mint);
    assert_eq!(transfer_events[0].from_owner, fixture.owner);
    assert_eq!(transfer_events[0].from_token_account, fixture.alice_token);
    assert_eq!(transfer_events[0].to_owner, fixture.bob_owner);
    assert_eq!(transfer_events[0].to_token_account, fixture.bob_token);
    assert_eq!(
        transfer_events[0].transferred_handle,
        transferred.current_handle
    );
    assert_eq!(
        transfer_events[0].transferred_encrypted_value,
        transferred_value_address
    );

    let balance_events: Vec<token::BalanceHandleUpdatedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(balance_events.len(), 2);
    assert_eq!(
        balance_events[0].reason,
        token::BalanceHandleUpdateReason::TransferDebit
    );
    assert_eq!(balance_events[0].old_handle, fixture.alice_initial);
    assert_eq!(balance_events[0].new_handle, alice_balance.current_handle);
    assert_eq!(
        balance_events[1].reason,
        token::BalanceHandleUpdateReason::TransferCredit
    );
    assert_eq!(balance_events[1].old_handle, fixture.bob_initial);
    assert_eq!(balance_events[1].new_handle, bob_balance.current_handle);
}

#[test]
fn mollusk_confidential_transfer_with_deny_list_succeeds_when_neither_authority_is_denied() {
    let fixture = TokenFixture::new();
    let (accounts, deny_records) = deny_enabled_transfer_accounts(&fixture, None);
    let context = mollusk().with_context(accounts);
    let amount_handle = handle_for_chain(23, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let ix = confidential_transfer_ix_with_remaining(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
        deny_records,
    );

    context.process_and_validate_instruction(&ix, &[Check::success()]);

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_ne!(alice_balance.current_handle, fixture.alice_initial);
    assert_ne!(bob_balance.current_handle, fixture.bob_initial);
    assert!(!account_is_system_owned_and_empty(
        &context,
        fixture.transferred_amount_value_address(fixture.alice_token)
    ));
}

#[test]
fn mollusk_confidential_transfer_with_deny_list_rejects_denied_from_authority() {
    let fixture = TokenFixture::new();
    let (accounts, deny_records) =
        deny_enabled_transfer_accounts(&fixture, Some(fixture.alice_token));
    let context = mollusk().with_context(accounts);
    let amount_handle = handle_for_chain(24, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let ix = confidential_transfer_ix_with_remaining(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
        deny_records,
    );

    context.process_and_validate_instruction(
        &ix,
        &[host_error(host::errors::ZamaHostError::AclSubjectDenied)],
    );

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
    assert_eq!(bob_balance.current_handle, fixture.bob_initial);
    assert!(account_is_system_owned_and_empty(
        &context,
        fixture.transferred_amount_value_address(fixture.alice_token)
    ));
}

#[test]
fn mollusk_confidential_transfer_with_deny_list_rejects_denied_to_authority() {
    let fixture = TokenFixture::new();
    let (accounts, deny_records) =
        deny_enabled_transfer_accounts(&fixture, Some(fixture.bob_token));
    let context = mollusk().with_context(accounts);
    let amount_handle = handle_for_chain(25, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let ix = confidential_transfer_ix_with_remaining(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
        deny_records,
    );

    context.process_and_validate_instruction(
        &ix,
        &[host_error(host::errors::ZamaHostError::AclSubjectDenied)],
    );

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
    assert_eq!(bob_balance.current_handle, fixture.bob_initial);
    assert!(account_is_system_owned_and_empty(
        &context,
        fixture.transferred_amount_value_address(fixture.alice_token)
    ));
}

#[test]
fn mollusk_request_disclose_balance_after_transfer_marks_current_handle_public() {
    let fixture = TokenFixture::new();
    let kms_context_id = 7;
    let mut accounts = fixture.base_accounts();
    accounts.insert(
        fixture.host_config,
        host_config_account_with_kms_context(
            fixture.owner,
            secp_evm_address(&coprocessor_signing_key()),
            kms_context_id,
        ),
    );
    let context = mollusk().with_context(accounts);

    let amount_handle = handle_for_chain(22, BALANCE_FHE_TYPE);
    let transfer_ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer),
    );
    context.process_and_validate_instruction(&transfer_ix, &[Check::success()]);

    let before = read_encrypted_value(&context, fixture.alice_balance_value);
    assert_ne!(before.current_handle, fixture.alice_initial);
    assert_eq!(before.leaf_count, 2);

    let request_nonce = [0x77; 32];
    let expires_slot = 200;
    let disclosure_request = token::disclosure_request_address(
        fixture.mint,
        fixture.owner,
        before.current_handle,
        request_nonce,
    )
    .0;
    context
        .account_store
        .borrow_mut()
        .insert(disclosure_request, system_account(0));

    let disclose_ix =
        request_disclose_balance_ix(&fixture, request_nonce, expires_slot, disclosure_request);
    let result = context.process_and_validate_instruction(&disclose_ix, &[Check::success()]);

    let after = read_encrypted_value(&context, fixture.alice_balance_value);
    assert_eq!(after.current_handle, before.current_handle);
    assert_eq!(after.leaf_count, 3);
    let mut expected_peaks = before.peaks.clone();
    let mut expected_count = before.leaf_count;
    let public_leaf = zama_solana_acl::public_decrypt_leaf_commitment(
        fixture.alice_balance_value.to_bytes(),
        before.leaf_count,
        before.current_handle,
    );
    zama_solana_acl::mmr_append(&mut expected_peaks, &mut expected_count, public_leaf).unwrap();
    assert_eq!(expected_count, after.leaf_count);
    assert_eq!(after.peaks, expected_peaks);

    let request = read_disclosure_request(&context, disclosure_request);
    assert_eq!(request.mint, fixture.mint);
    assert_eq!(request.requester, fixture.owner);
    assert_eq!(request.token_account, fixture.alice_token);
    assert_eq!(request.app_account, fixture.alice_token);
    assert_eq!(request.handle, before.current_handle);
    assert_eq!(request.encrypted_value, fixture.alice_balance_value);
    assert_eq!(request.host_config, fixture.host_config);
    assert_eq!(request.kms_context_id, kms_context_id);
    assert_eq!(request.request_nonce, request_nonce);
    assert_eq!(request.chain_id, host::SOLANA_POC_CHAIN_ID);
    assert_eq!(request.expires_slot, expires_slot);
    assert_eq!(request.mode, token::DISCLOSURE_REQUEST_MODE_BALANCE);
    assert_eq!(request.status, token::REQUEST_STATUS_PENDING);

    let events: Vec<token::BalanceDisclosureRequestedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].handle, before.current_handle);
    assert_eq!(events[0].encrypted_value, fixture.alice_balance_value);
    assert_eq!(events[0].request, disclosure_request);
    assert_eq!(events[0].request_hash, request.request_hash);
    assert_eq!(events[0].kms_context_id, kms_context_id);
    assert_eq!(events[0].expires_slot, expires_slot);
}

#[test]
fn mollusk_confidential_transfer_rejects_owner_mismatch() {
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(30, BALANCE_FHE_TYPE);
    // Attestation correctly authored by bob, but bob is not `from_account`'s owner: the
    // instruction's own owner-signer check must reject this before any ACL work happens.
    let attestation =
        amount_attestation_for(amount_handle, fixture.bob_owner, fixture.compute_signer);
    let mut ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );
    // Sign as bob (matches the attestation) so only the owner-mismatch check can fail.
    for meta in ix.accounts.iter_mut() {
        if meta.pubkey == fixture.owner {
            meta.pubkey = fixture.bob_owner;
        }
    }

    context.process_and_validate_instruction(
        &ix,
        &[token_error(token::ConfidentialTokenError::OwnerMismatch)],
    );

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
}

#[test]
fn mollusk_confidential_transfer_rejects_attestation_user_mismatch() {
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(31, BALANCE_FHE_TYPE);
    // fromExternal binding: an attestation authored by someone other than the transfer authority
    // (owner) must be rejected before any balance rotation.
    let attestation =
        amount_attestation_for(amount_handle, fixture.bob_owner, fixture.compute_signer);
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );

    context.process_and_validate_instruction(
        &ix,
        &[token_error(
            token::ConfidentialTokenError::AttestationUserMismatch,
        )],
    );

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
    assert_eq!(bob_balance.current_handle, fixture.bob_initial);
    assert!(account_is_system_owned_and_empty(
        &context,
        fixture.transferred_amount_value_address(fixture.alice_token)
    ));
}

#[test]
fn mollusk_confidential_transfer_rejects_attestation_contract_mismatch() {
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(32, BALANCE_FHE_TYPE);
    // fromExternal binding: an attestation bound to a contract other than the mint compute-signer
    // PDA must be rejected before any balance rotation.
    let attestation = amount_attestation_for(amount_handle, fixture.owner, Pubkey::new_unique());
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );

    context.process_and_validate_instruction(
        &ix,
        &[token_error(
            token::ConfidentialTokenError::AttestationContractMismatch,
        )],
    );

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
    assert_eq!(bob_balance.current_handle, fixture.bob_initial);
}

#[test]
fn mollusk_confidential_transfer_rejects_stale_balance_encrypted_value() {
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(33, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    // A balance `EncryptedValue` address that does not match `from_account.balance_encrypted_value`
    // must be rejected by the account's `address = from_account.balance_encrypted_value` constraint.
    let (stale_address, stale_value) = new_encrypted_value(
        fixture.mint,
        fixture.alice_token,
        token::wrap_amount_label(),
        fixture.alice_initial,
        &[fixture.owner],
    );
    context
        .account_store
        .borrow_mut()
        .insert(stale_address, encrypted_value_account(&stale_value));
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        stale_address,
        fixture.bob_balance_value,
        attestation,
    );

    context.process_and_validate_instruction(
        &ix,
        &[anchor_error(
            anchor_lang::error::ErrorCode::ConstraintAddress,
        )],
    );

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
}

#[test]
fn mollusk_confidential_transfer_rejects_balance_wrong_mint_acl_domain() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    let wrong_mint = Pubkey::new_unique();
    let (_, mut wrong_domain_value) = new_encrypted_value(
        fixture.mint,
        fixture.alice_token,
        token::balance_label(),
        fixture.alice_initial,
        &[fixture.owner, fixture.compute_signer],
    );
    wrong_domain_value.acl_domain_key = wrong_mint;
    accounts.insert(
        fixture.alice_balance_value,
        encrypted_value_account(&wrong_domain_value),
    );
    let context = mollusk().with_context(accounts);
    let amount_handle = handle_for_chain(34, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );

    context.process_and_validate_instruction(
        &ix,
        &[host_error(
            host::errors::ZamaHostError::EncryptedValuePdaMismatch,
        )],
    );

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_eq!(alice_balance.acl_domain_key, wrong_mint);
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
    assert_eq!(bob_balance.current_handle, fixture.bob_initial);
    assert!(account_is_system_owned_and_empty(
        &context,
        fixture.transferred_amount_value_address(fixture.alice_token)
    ));
}

#[test]
fn mollusk_confidential_transfer_rejects_balance_wrong_token_account_app_account() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    let wrong_token_account = Pubkey::new_unique();
    let (_, mut wrong_app_account_value) = new_encrypted_value(
        fixture.mint,
        fixture.alice_token,
        token::balance_label(),
        fixture.alice_initial,
        &[fixture.owner, fixture.compute_signer],
    );
    wrong_app_account_value.app_account = wrong_token_account;
    accounts.insert(
        fixture.alice_balance_value,
        encrypted_value_account(&wrong_app_account_value),
    );
    let context = mollusk().with_context(accounts);
    let amount_handle = handle_for_chain(35, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );

    context.process_and_validate_instruction(
        &ix,
        &[host_error(
            host::errors::ZamaHostError::EncryptedValuePdaMismatch,
        )],
    );

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_eq!(alice_balance.app_account, wrong_token_account);
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
    assert_eq!(bob_balance.current_handle, fixture.bob_initial);
    assert!(account_is_system_owned_and_empty(
        &context,
        fixture.transferred_amount_value_address(fixture.alice_token)
    ));
}

// ---------------------------------------------------------------------------
// confidential_burn -> request_burn_redemption -> redeem_burned_amount_secp
//
// Vector 2 (burn-stranding) fix: every burn is born publicly decryptable at the
// burn instant (ERC-7984 `unwrap` parity), so a historical burned handle stays
// redeemable even after a later burn supersedes the shared `burned_amount`
// lineage. Authorization at redeem is by MMR public-decrypt proof (theme-A).
// ---------------------------------------------------------------------------

use anchor_spl::token::spl_token;
use solana_sdk::program_option::COption;

/// KMS signing key backing `PublicDecryptVerification` certs; its EVM address is
/// the sole signer of the fixture's pinned KMS context.
fn kms_signing_key() -> k256::ecdsa::SigningKey {
    k256::ecdsa::SigningKey::from_bytes(&[0x55u8; 32].into()).unwrap()
}

/// Builds a KMS `PublicDecryptVerification` secp256k1 cert (`signatures`, `extra_data`)
/// over `handle`/`cleartext_amount`, matching `assert_kms_public_decrypt_cert_for_request`.
/// `extra_data == [0x00]` binds the cert to the request's current KMS context.
fn kms_public_decrypt_cert(handle: [u8; 32], cleartext_amount: u64) -> (Vec<[u8; 65]>, Vec<u8>) {
    let extra_data = vec![0x00u8];
    let mut decrypted = [0u8; 32];
    decrypted[24..].copy_from_slice(&cleartext_amount.to_be_bytes());
    let digest = host::eip712::typed_data_digest(
        &host::eip712::domain_separator(
            b"Decryption",
            b"1",
            GATEWAY_CHAIN_ID,
            &DECRYPTION_CONTRACT,
        ),
        &host::eip712::public_decrypt_struct_hash(&[handle], &decrypted, &extra_data),
    );
    (vec![secp_sign(&kms_signing_key(), &digest)], extra_data)
}

fn kms_context_account(context_id: u64) -> Account {
    let (_, bump) = host::kms_context_address(context_id);
    Account {
        lamports: 1_000_000_000,
        data: serialized_account(host::KmsContext {
            context_id,
            signers: vec![secp_evm_address(&kms_signing_key())],
            thresholds: host::KmsThresholds {
                public_decryption: 1,
                user_decryption: 1,
                kms_gen: 1,
                mpc: 1,
            },
            destroyed: false,
            bump,
        }),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn spl_mint_account(decimals: u8) -> Account {
    let mut data = vec![0u8; spl_token::state::Mint::LEN];
    spl_token::state::Mint::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 1_000_000,
            decimals,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        &mut data,
    )
    .unwrap();
    Account {
        lamports: 1_000_000_000,
        data,
        owner: spl_token::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn spl_token_account(mint: Pubkey, owner: Pubkey, amount: u64) -> Account {
    let mut data = vec![0u8; spl_token::state::Account::LEN];
    spl_token::state::Account::pack(
        spl_token::state::Account {
            mint,
            owner,
            amount,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        &mut data,
    )
    .unwrap();
    Account {
        lamports: 1_000_000_000,
        data,
        owner: spl_token::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn read_spl_amount(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> u64 {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing spl token account")
        .clone();
    spl_token::state::Account::unpack(&account.data)
        .expect("valid spl token account")
        .amount
}

/// Self-contained fixture for the burn/request/redeem vertical: one owner, one
/// confidential mint with an SPL-backed vault, and one funded token account.
struct BurnRedeemFixture {
    owner: Pubkey,
    mint: Pubkey,
    compute_signer: Pubkey,
    host_config: Pubkey,
    token_account: Pubkey,
    balance_value: Pubkey,
    total_supply_authority: Pubkey,
    total_supply_value: Pubkey,
    burned_amount_value: Pubkey,
    underlying_mint: Pubkey,
    vault_authority: Pubkey,
    vault_usdc: Pubkey,
    destination_usdc: Pubkey,
    kms_context_id: u64,
    kms_context: Pubkey,
    initial_balance: [u8; 32],
    initial_total_supply: [u8; 32],
}

impl BurnRedeemFixture {
    fn new() -> Self {
        let owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let compute_signer = token::compute_signer_address(mint).0;
        let host_config = host::host_config_address().0;
        let token_account = token::token_account_address(mint, owner).0;
        let balance_value = token::balance_encrypted_value_address(mint, token_account).0;
        let total_supply_authority = token::total_supply_authority_address(mint).0;
        let total_supply_value =
            token::total_supply_encrypted_value_address(mint, total_supply_authority).0;
        let burned_amount_value =
            token::encrypted_value_address(mint, token_account, token::burned_amount_label()).0;
        let underlying_mint = Pubkey::new_unique();
        let vault_authority = token::vault_authority_address(mint).0;
        let vault_usdc = token::vault_token_account_address(mint, underlying_mint);
        let destination_usdc = Pubkey::new_unique();
        let kms_context_id = 9;
        let kms_context = host::kms_context_address(kms_context_id).0;
        Self {
            owner,
            mint,
            compute_signer,
            host_config,
            token_account,
            balance_value,
            total_supply_authority,
            total_supply_value,
            burned_amount_value,
            underlying_mint,
            vault_authority,
            vault_usdc,
            destination_usdc,
            kms_context_id,
            kms_context,
            initial_balance: handle_for_chain(1, BALANCE_FHE_TYPE),
            initial_total_supply: handle_for_chain(2, BALANCE_FHE_TYPE),
        }
    }

    fn confidential_mint_account(&self) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialMint {
                authority: self.owner,
                acl_domain_key: self.mint,
                compute_signer: self.compute_signer,
                underlying_mint: self.underlying_mint,
                decimals: 6,
                total_supply_encrypted_value: self.total_supply_value,
            }),
            owner: token::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    fn accounts(&self, vault_balance: u64) -> HashMap<Pubkey, Account> {
        let (_, balance_value) = new_encrypted_value(
            self.mint,
            self.token_account,
            token::balance_label(),
            self.initial_balance,
            &[self.owner, self.compute_signer],
        );
        let (_, total_supply_value) = new_encrypted_value(
            self.mint,
            self.total_supply_authority,
            token::total_supply_label(),
            self.initial_total_supply,
            &[self.compute_signer],
        );
        HashMap::from([
            (self.owner, system_account(50_000_000_000)),
            (self.mint, self.confidential_mint_account()),
            (self.compute_signer, system_account(0)),
            (self.total_supply_authority, system_account(0)),
            (self.vault_authority, system_account(0)),
            (
                self.host_config,
                host_config_account_with_kms_context(
                    self.owner,
                    secp_evm_address(&coprocessor_signing_key()),
                    self.kms_context_id,
                ),
            ),
            (self.kms_context, kms_context_account(self.kms_context_id)),
            (
                self.token_account,
                token_account_account(self.mint, self.owner, self.balance_value),
            ),
            (self.balance_value, encrypted_value_account(&balance_value)),
            (
                self.total_supply_value,
                encrypted_value_account(&total_supply_value),
            ),
            (self.burned_amount_value, system_account(0)),
            (self.underlying_mint, spl_mint_account(6)),
            (
                self.vault_usdc,
                spl_token_account(self.underlying_mint, self.vault_authority, vault_balance),
            ),
            (
                self.destination_usdc,
                spl_token_account(self.underlying_mint, self.owner, 0),
            ),
            (event_authority(host::id()), system_account(0)),
            (event_authority(token::id()), system_account(0)),
            mollusk_svm_programs_token::token::keyed_account(),
        ])
    }
}

fn token_account_account(mint: Pubkey, owner: Pubkey, balance_value: Pubkey) -> Account {
    Account {
        lamports: 1_000_000_000,
        data: serialized_account(token::ConfidentialTokenAccount {
            owner,
            mint,
            balance_encrypted_value: balance_value,
            bump: token::token_account_address(mint, owner).1,
        }),
        owner: token::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn burn_redeem_mollusk() -> Mollusk {
    let mut mollusk = mollusk();
    mollusk_svm_programs_token::token::add_program(&mut mollusk);
    mollusk
}

fn confidential_burn_ix(
    fixture: &BurnRedeemFixture,
    amount_attestation: host::CoprocessorInputAttestation,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::ConfidentialBurn {
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.token_account,
            compute_signer: fixture.compute_signer,
            total_supply_authority: fixture.total_supply_authority,
            balance_value: fixture.balance_value,
            total_supply_value: fixture.total_supply_value,
            burned_amount_value: fixture.burned_amount_value,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            hcu_authority: token::hcu_authority_address(fixture.mint).0,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialBurn { amount_attestation },
    )
}

fn request_burn_redemption_ix(
    fixture: &BurnRedeemFixture,
    burned_handle: [u8; 32],
    request_nonce: [u8; 32],
    expires_slot: u64,
    redemption_request: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::RequestBurnRedemption {
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.token_account,
            underlying_mint: fixture.underlying_mint,
            destination_usdc: fixture.destination_usdc,
            burned_amount_value: fixture.burned_amount_value,
            redemption_request,
            deny_subject_record: None,
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::RequestBurnRedemption {
            burned_handle,
            request_nonce,
            expires_slot,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn redeem_burned_amount_secp_ix(
    fixture: &BurnRedeemFixture,
    burned_handle: [u8; 32],
    cleartext_amount: u64,
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
    proof: token::MmrInclusionProof,
    redemption_request: Pubkey,
    redemption_record: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::RedeemBurnedAmountSecp {
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.token_account,
            underlying_mint: fixture.underlying_mint,
            vault_usdc: fixture.vault_usdc,
            destination_usdc: fixture.destination_usdc,
            vault_authority: fixture.vault_authority,
            burned_amount_value: fixture.burned_amount_value,
            redemption_request,
            redemption_record,
            host_config: fixture.host_config,
            kms_context: fixture.kms_context,
            token_program: spl_token::id(),
            system_program: system_program::ID,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::RedeemBurnedAmountSecp {
            burned_handle,
            cleartext_amount,
            signatures,
            extra_data,
            proof,
        },
    )
}

/// Burns `amount_seed`'s attested amount and returns the resulting burned handle
/// (the lineage's new current handle) read back from the superseded lineage.
fn run_burn(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &BurnRedeemFixture,
    amount_seed: u8,
) -> [u8; 32] {
    let amount_handle = handle_for_chain(amount_seed, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let ix = confidential_burn_ix(fixture, attestation);
    context.process_and_validate_instruction(&ix, &[Check::success()]);
    read_encrypted_value(context, fixture.burned_amount_value).current_handle
}

#[test]
fn mollusk_confidential_burn_makes_burned_amount_publicly_decryptable() {
    let fixture = BurnRedeemFixture::new();
    let context = burn_redeem_mollusk().with_context(fixture.accounts(1_000));

    let burned_handle = run_burn(&context, &fixture, 41);

    // First burn creates the lineage (no create leaf) and then appends exactly one
    // public-decrypt leaf for the just-bound burned handle.
    let lineage = read_encrypted_value(&context, fixture.burned_amount_value);
    assert_eq!(lineage.current_handle, burned_handle);
    assert_eq!(
        lineage.subjects,
        vec![fixture.owner, fixture.compute_signer]
    );
    assert_eq!(lineage.leaf_count, 1);
    let public_leaf = zama_solana_acl::public_decrypt_leaf_commitment(
        fixture.burned_amount_value.to_bytes(),
        0,
        burned_handle,
    );
    assert_eq!(
        lineage.peaks,
        zama_solana_acl::mmr_peaks_from_leaves(&[public_leaf])
    );
}

/// Reconstructs the burned_amount lineage's four leaves after two burns and
/// builds a public-decrypt inclusion proof for the FIRST burn's handle (leaf 0).
fn public_decrypt_proof_for_first_burn(
    fixture: &BurnRedeemFixture,
    first_handle: [u8; 32],
    second_handle: [u8; 32],
) -> (token::MmrInclusionProof, Vec<[u8; 32]>) {
    let acct = fixture.burned_amount_value.to_bytes();
    let leaves = vec![
        zama_solana_acl::public_decrypt_leaf_commitment(acct, 0, first_handle),
        zama_solana_acl::historical_access_leaf_commitment(
            acct,
            1,
            first_handle,
            fixture.owner.to_bytes(),
        ),
        zama_solana_acl::historical_access_leaf_commitment(
            acct,
            2,
            first_handle,
            fixture.compute_signer.to_bytes(),
        ),
        zama_solana_acl::public_decrypt_leaf_commitment(acct, 3, second_handle),
    ];
    let proof = zama_solana_acl::mmr_build_proof(&leaves, 0).expect("proof for leaf 0");
    (
        token::MmrInclusionProof {
            leaf_index: proof.leaf_index,
            siblings: proof.siblings,
        },
        zama_solana_acl::mmr_peaks_from_leaves(&leaves),
    )
}

#[test]
fn mollusk_request_and_redeem_historical_burned_handle_after_supersession() {
    // Vector 2 regression, validated against the burned_amount lineage state that two
    // real burns produce (public(H1)@0, hist(H1,owner)@1, hist(H1,compute)@2, public(H2)@3,
    // current handle H2). The two-burn *execution* is exercised by the `#[ignore]`d E2E test
    // below; it currently overflows the 32 KiB per-transaction heap on the supersede burn
    // (see the migration report). This test drives the exact request + redeem code paths the
    // fix changed: request accepting a HISTORICAL handle, and redeem authorizing it via the
    // burn-appended public-decrypt MMR proof.
    let fixture = BurnRedeemFixture::new();
    let first_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let second_handle = handle_for_chain(42, BALANCE_FHE_TYPE);
    let (proof, expected_peaks) =
        public_decrypt_proof_for_first_burn(&fixture, first_handle, second_handle);

    let mut accounts = fixture.accounts(1_000);
    let (_, mut lineage) = new_encrypted_value(
        fixture.mint,
        fixture.token_account,
        token::burned_amount_label(),
        second_handle,
        &[fixture.owner, fixture.compute_signer],
    );
    lineage.leaf_count = 4;
    lineage.peaks = expected_peaks.clone();
    accounts.insert(
        fixture.burned_amount_value,
        encrypted_value_account(&lineage),
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    // request(H1) SUCCEEDS even though H1 is historical (lineage current handle is H2).
    let request_nonce = [0x99u8; 32];
    let expires_slot = 200;
    let redemption_request = token::burn_redemption_request_address(
        fixture.mint,
        fixture.owner,
        first_handle,
        request_nonce,
    )
    .0;
    context
        .account_store
        .borrow_mut()
        .insert(redemption_request, system_account(0));
    context.process_and_validate_instruction(
        &request_burn_redemption_ix(
            &fixture,
            first_handle,
            request_nonce,
            expires_slot,
            redemption_request,
        ),
        &[Check::success()],
    );

    // request appends NO leaf (the burn already made H1 public); proof position is stable.
    let after_request = read_encrypted_value(&context, fixture.burned_amount_value);
    assert_eq!(after_request.leaf_count, 4);
    assert_eq!(after_request.peaks, expected_peaks);

    // redeem(H1) with the real public-decrypt proof + KMS cert releases H1's amount.
    let cleartext_amount = 500;
    let (signatures, extra_data) = kms_public_decrypt_cert(first_handle, cleartext_amount);
    let redemption_record = token::burn_redemption_address(fixture.mint, first_handle).0;
    context
        .account_store
        .borrow_mut()
        .insert(redemption_record, system_account(0));
    context.process_and_validate_instruction(
        &redeem_burned_amount_secp_ix(
            &fixture,
            first_handle,
            cleartext_amount,
            signatures.clone(),
            extra_data.clone(),
            proof.clone(),
            redemption_request,
            redemption_record,
        ),
        &[Check::success()],
    );

    assert_eq!(
        read_spl_amount(&context, fixture.destination_usdc),
        cleartext_amount
    );
    assert_eq!(
        read_spl_amount(&context, fixture.vault_usdc),
        1_000 - cleartext_amount
    );
    let record = context
        .account_store
        .borrow()
        .get(&redemption_record)
        .expect("redemption record")
        .clone();
    let redemption = token::BurnRedemption::try_deserialize(&mut record.data.as_slice())
        .expect("burn redemption record");
    assert_eq!(redemption.burned_handle, first_handle);
    assert_eq!(redemption.cleartext_amount, cleartext_amount);

    // Double-redeem of the same handle is blocked (consumed request + per-handle marker PDA).
    let dup = redeem_burned_amount_secp_ix(
        &fixture,
        first_handle,
        cleartext_amount,
        signatures,
        extra_data,
        proof,
        redemption_request,
        redemption_record,
    );
    assert!(context.process_instruction(&dup).raw_result.is_err());
    assert_eq!(
        read_spl_amount(&context, fixture.destination_usdc),
        cleartext_amount
    );
}

#[test]
fn mollusk_redeem_rejects_foreign_public_decrypt_proof() {
    // Same seeded post-supersession lineage as the regression test above.
    let fixture = BurnRedeemFixture::new();
    let first_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let second_handle = handle_for_chain(42, BALANCE_FHE_TYPE);
    let (mut proof, expected_peaks) =
        public_decrypt_proof_for_first_burn(&fixture, first_handle, second_handle);

    let mut accounts = fixture.accounts(1_000);
    let (_, mut lineage) = new_encrypted_value(
        fixture.mint,
        fixture.token_account,
        token::burned_amount_label(),
        second_handle,
        &[fixture.owner, fixture.compute_signer],
    );
    lineage.leaf_count = 4;
    lineage.peaks = expected_peaks;
    accounts.insert(
        fixture.burned_amount_value,
        encrypted_value_account(&lineage),
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let request_nonce = [0x99u8; 32];
    let redemption_request = token::burn_redemption_request_address(
        fixture.mint,
        fixture.owner,
        first_handle,
        request_nonce,
    )
    .0;
    context
        .account_store
        .borrow_mut()
        .insert(redemption_request, system_account(0));
    context.process_and_validate_instruction(
        &request_burn_redemption_ix(
            &fixture,
            first_handle,
            request_nonce,
            200,
            redemption_request,
        ),
        &[Check::success()],
    );

    // A structurally valid proof aimed at the WRONG leaf position: authorize_public recomputes
    // public(H1)@leaf_index, which no longer matches the lineage peaks, so it is rejected.
    proof.leaf_index = 3; // H2's public-decrypt leaf, not H1's.

    let cleartext_amount = 500;
    let (signatures, extra_data) = kms_public_decrypt_cert(first_handle, cleartext_amount);
    let redemption_record = token::burn_redemption_address(fixture.mint, first_handle).0;
    context
        .account_store
        .borrow_mut()
        .insert(redemption_record, system_account(0));
    context.process_and_validate_instruction(
        &redeem_burned_amount_secp_ix(
            &fixture,
            first_handle,
            cleartext_amount,
            signatures,
            extra_data,
            proof,
            redemption_request,
            redemption_record,
        ),
        &[token_error(
            token::ConfidentialTokenError::PublicDecryptProofInvalid,
        )],
    );

    // Vault untouched on a rejected proof.
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 1_000);
    assert_eq!(read_spl_amount(&context, fixture.destination_usdc), 0);
}

// Full end-to-end Vector 2 regression: burn H1 -> burn H2 (supersede, no request between)
// -> request(H1) -> redeem(H1). The burned delta is now born publicly decryptable inside the
// single eval CPI (DD-036), so the supersede burn no longer runs a second `make_handle_public`
// CPI — the transaction stays within Solana's 32 KiB bump heap. This drives two real burns and
// then the historical request + proof-authorized redeem end to end.
#[test]
fn mollusk_e2e_vector2_burn_supersede_request_redeem() {
    let fixture = BurnRedeemFixture::new();
    let context = burn_redeem_mollusk().with_context(fixture.accounts(1_000));

    let first_handle = run_burn(&context, &fixture, 41);
    let second_handle = run_burn(&context, &fixture, 42);
    assert_ne!(first_handle, second_handle);

    let lineage = read_encrypted_value(&context, fixture.burned_amount_value);
    assert_eq!(lineage.current_handle, second_handle);
    assert_eq!(lineage.leaf_count, 4);
    let (proof, expected_peaks) =
        public_decrypt_proof_for_first_burn(&fixture, first_handle, second_handle);
    assert_eq!(lineage.peaks, expected_peaks);

    let request_nonce = [0x99u8; 32];
    let redemption_request = token::burn_redemption_request_address(
        fixture.mint,
        fixture.owner,
        first_handle,
        request_nonce,
    )
    .0;
    context
        .account_store
        .borrow_mut()
        .insert(redemption_request, system_account(0));
    context.process_and_validate_instruction(
        &request_burn_redemption_ix(
            &fixture,
            first_handle,
            request_nonce,
            200,
            redemption_request,
        ),
        &[Check::success()],
    );

    let cleartext_amount = 500;
    let (signatures, extra_data) = kms_public_decrypt_cert(first_handle, cleartext_amount);
    let redemption_record = token::burn_redemption_address(fixture.mint, first_handle).0;
    context
        .account_store
        .borrow_mut()
        .insert(redemption_record, system_account(0));
    context.process_and_validate_instruction(
        &redeem_burned_amount_secp_ix(
            &fixture,
            first_handle,
            cleartext_amount,
            signatures,
            extra_data,
            proof,
            redemption_request,
            redemption_record,
        ),
        &[Check::success()],
    );
    assert_eq!(
        read_spl_amount(&context, fixture.destination_usdc),
        cleartext_amount
    );
}

// ---------------------------------------------------------------------------
// disclose_*_secp consume: proof-authorized against the WITNESS-pinned handle
// (DD-036 disclose twin of the burn->redeem consume path). These drive the
// exact code path the fix changed: consume authorizes by an MMR public-decrypt
// proof against the request-pinned handle, not the live lineage handle, so a
// disclosure survives its lineage being superseded during the KMS round-trip.
// ---------------------------------------------------------------------------

/// Self-contained fixture for the disclose consume vertical: one owner, one
/// confidential mint, a balance lineage, and one token-scoped amount lineage.
struct DiscloseFixture {
    owner: Pubkey,
    mint: Pubkey,
    compute_signer: Pubkey,
    host_config: Pubkey,
    token_account: Pubkey,
    balance_value: Pubkey,
    amount_value: Pubkey,
    kms_context_id: u64,
    kms_context: Pubkey,
}

impl DiscloseFixture {
    fn new() -> Self {
        let owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let compute_signer = token::compute_signer_address(mint).0;
        let host_config = host::host_config_address().0;
        let token_account = token::token_account_address(mint, owner).0;
        let balance_value = token::balance_encrypted_value_address(mint, token_account).0;
        // Amount mode discloses any token-scoped amount lineage; use the burned_amount slot.
        let amount_value =
            token::encrypted_value_address(mint, token_account, token::burned_amount_label()).0;
        let kms_context_id = 9;
        let kms_context = host::kms_context_address(kms_context_id).0;
        Self {
            owner,
            mint,
            compute_signer,
            host_config,
            token_account,
            balance_value,
            amount_value,
            kms_context_id,
            kms_context,
        }
    }

    fn confidential_mint_account(&self) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialMint {
                authority: self.owner,
                acl_domain_key: self.mint,
                compute_signer: self.compute_signer,
                underlying_mint: Pubkey::new_unique(),
                decimals: 6,
                total_supply_encrypted_value: token::total_supply_encrypted_value_address(
                    self.mint,
                    token::total_supply_authority_address(self.mint).0,
                )
                .0,
            }),
            owner: token::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    fn base(&self) -> HashMap<Pubkey, Account> {
        HashMap::from([
            (self.owner, system_account(50_000_000_000)),
            (self.mint, self.confidential_mint_account()),
            (
                self.host_config,
                host_config_account_with_kms_context(
                    self.owner,
                    secp_evm_address(&coprocessor_signing_key()),
                    self.kms_context_id,
                ),
            ),
            (self.kms_context, kms_context_account(self.kms_context_id)),
            (event_authority(token::id()), system_account(0)),
        ])
    }
}

/// Builds a lineage carrying a public-decrypt leaf (leaf 0) for `pinned`, and the inclusion
/// proof for it. With `supersede_to = Some(h2)` the lineage is grown into a post-supersession
/// state (public(pinned)@0, hist(pinned,subj0)@1, hist(pinned,subj1)@2, public(h2)@3, current
/// handle h2), modeling the pinned handle becoming historical after the request. `subjects`
/// must hold at least two entries when superseding.
fn public_leaf_lineage(
    account: Pubkey,
    app_account: Pubkey,
    mint: Pubkey,
    label: [u8; 32],
    subjects: &[Pubkey],
    pinned: [u8; 32],
    supersede_to: Option<[u8; 32]>,
) -> (host::EncryptedValue, token::MmrInclusionProof) {
    let acct = account.to_bytes();
    let mut leaves = vec![zama_solana_acl::public_decrypt_leaf_commitment(
        acct, 0, pinned,
    )];
    let current = match supersede_to {
        Some(h2) => {
            leaves.push(zama_solana_acl::historical_access_leaf_commitment(
                acct,
                1,
                pinned,
                subjects[0].to_bytes(),
            ));
            leaves.push(zama_solana_acl::historical_access_leaf_commitment(
                acct,
                2,
                pinned,
                subjects[1].to_bytes(),
            ));
            leaves.push(zama_solana_acl::public_decrypt_leaf_commitment(acct, 3, h2));
            h2
        }
        None => pinned,
    };
    let (address, mut value) = new_encrypted_value(mint, app_account, label, current, subjects);
    assert_eq!(address, account, "lineage account address mismatch");
    value.leaf_count = leaves.len() as u64;
    value.peaks = zama_solana_acl::mmr_peaks_from_leaves(&leaves);
    let proof = zama_solana_acl::mmr_build_proof(&leaves, 0).expect("proof for leaf 0");
    (
        value,
        token::MmrInclusionProof {
            leaf_index: proof.leaf_index,
            siblings: proof.siblings,
        },
    )
}

/// Seeds a fully-formed disclosure request witness (canonical PDA + recomputed request_hash).
#[allow(clippy::too_many_arguments)]
fn disclosure_request_account(
    mint: Pubkey,
    requester: Pubkey,
    token_account: Pubkey,
    app_account: Pubkey,
    handle: [u8; 32],
    encrypted_value: Pubkey,
    host_config: Pubkey,
    kms_context_id: u64,
    request_nonce: [u8; 32],
    expires_slot: u64,
    mode: u8,
    status: u8,
) -> (Pubkey, Account) {
    let (address, bump) = token::disclosure_request_address(mint, requester, handle, request_nonce);
    let chain_id = host::SOLANA_POC_CHAIN_ID;
    let request_hash = token::disclosure_request_hash(
        token::id(),
        address,
        mint,
        requester,
        token_account,
        app_account,
        handle,
        encrypted_value,
        host_config,
        kms_context_id,
        request_nonce,
        chain_id,
        expires_slot,
        mode,
    );
    let request = token::DisclosureRequest {
        mint,
        requester,
        token_account,
        app_account,
        handle,
        encrypted_value,
        host_config,
        kms_context_id,
        request_nonce,
        request_hash,
        chain_id,
        expires_slot,
        mode,
        status,
        bump,
    };
    (
        address,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(request),
            owner: token::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn disclose_amount_secp_ix(
    fixture: &DiscloseFixture,
    disclosure_request: Pubkey,
    amount_handle: [u8; 32],
    cleartext_amount: u64,
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
    proof: token::MmrInclusionProof,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::DiscloseAmountSecp {
            mint: fixture.mint,
            amount_value: fixture.amount_value,
            disclosure_request,
            host_config: fixture.host_config,
            kms_context: fixture.kms_context,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::DiscloseAmountSecp {
            amount_handle,
            cleartext_amount,
            signatures,
            extra_data,
            proof,
        },
    )
}

fn disclose_balance_secp_ix(
    fixture: &DiscloseFixture,
    disclosure_request: Pubkey,
    cleartext_amount: u64,
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
    proof: token::MmrInclusionProof,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::DiscloseBalanceSecp {
            mint: fixture.mint,
            token_account: fixture.token_account,
            balance_value: fixture.balance_value,
            disclosure_request,
            host_config: fixture.host_config,
            kms_context: fixture.kms_context,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::DiscloseBalanceSecp {
            cleartext_amount,
            signatures,
            extra_data,
            proof,
        },
    )
}

#[test]
fn mollusk_disclose_amount_after_supersession_consumes_with_public_proof() {
    // The bug case: request pins H1 while it is current; the amount lineage is then superseded
    // to H2 during the KMS round-trip. Consume must still authorize H1 by its public-decrypt
    // proof (sealed at request time), not by the live handle.
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(41, BALANCE_FHE_TYPE);
    let superseded = handle_for_chain(42, BALANCE_FHE_TYPE);
    let (lineage, proof) = public_leaf_lineage(
        fixture.amount_value,
        fixture.token_account,
        fixture.mint,
        token::burned_amount_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        Some(superseded),
    );
    assert_ne!(lineage.current_handle, pinned);

    let request_nonce = [0x51u8; 32];
    let expires_slot = 200;
    let (request_addr, request_account) = disclosure_request_account(
        fixture.mint,
        fixture.owner,
        Pubkey::default(),
        lineage.app_account,
        pinned,
        fixture.amount_value,
        fixture.host_config,
        fixture.kms_context_id,
        request_nonce,
        expires_slot,
        token::DISCLOSURE_REQUEST_MODE_AMOUNT,
        token::REQUEST_STATUS_PENDING,
    );

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_value, encrypted_value_account(&lineage));
    accounts.insert(request_addr, request_account);
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, cleartext_amount);
    let result = context.process_and_validate_instruction(
        &disclose_amount_secp_ix(
            &fixture,
            request_addr,
            pinned,
            cleartext_amount,
            signatures,
            extra_data,
            proof,
        ),
        &[Check::success()],
    );

    assert_eq!(
        read_disclosure_request(&context, request_addr).status,
        token::REQUEST_STATUS_CONSUMED
    );
    let events: Vec<token::AmountDisclosedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].handle, pinned);
    assert_eq!(events[0].cleartext_amount, cleartext_amount);
}

#[test]
fn mollusk_disclose_balance_after_supersession_consumes_with_public_proof() {
    // Balance mode is third-party griefable: any inbound transfer rotates the balance lineage.
    // The pinned handle must still be disclosable by its public-decrypt proof after that.
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(31, BALANCE_FHE_TYPE);
    let superseded = handle_for_chain(32, BALANCE_FHE_TYPE);
    let (lineage, proof) = public_leaf_lineage(
        fixture.balance_value,
        fixture.token_account,
        fixture.mint,
        token::balance_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        Some(superseded),
    );
    assert_ne!(lineage.current_handle, pinned);

    let request_nonce = [0x31u8; 32];
    let expires_slot = 200;
    let (request_addr, request_account) = disclosure_request_account(
        fixture.mint,
        fixture.owner,
        fixture.token_account,
        fixture.token_account,
        pinned,
        fixture.balance_value,
        fixture.host_config,
        fixture.kms_context_id,
        request_nonce,
        expires_slot,
        token::DISCLOSURE_REQUEST_MODE_BALANCE,
        token::REQUEST_STATUS_PENDING,
    );

    let mut accounts = fixture.base();
    accounts.insert(
        fixture.token_account,
        token_account_account(fixture.mint, fixture.owner, fixture.balance_value),
    );
    accounts.insert(fixture.balance_value, encrypted_value_account(&lineage));
    accounts.insert(request_addr, request_account);
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 700;
    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, cleartext_amount);
    let result = context.process_and_validate_instruction(
        &disclose_balance_secp_ix(
            &fixture,
            request_addr,
            cleartext_amount,
            signatures,
            extra_data,
            proof,
        ),
        &[Check::success()],
    );

    assert_eq!(
        read_disclosure_request(&context, request_addr).status,
        token::REQUEST_STATUS_CONSUMED
    );
    let events: Vec<token::BalanceDisclosedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].handle, pinned);
    assert_eq!(events[0].owner, fixture.owner);
    assert_eq!(events[0].cleartext_amount, cleartext_amount);
}

#[test]
fn mollusk_disclose_amount_consume_happy_path() {
    // Non-superseded: pinned handle is still the live current handle.
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(43, BALANCE_FHE_TYPE);
    let (lineage, proof) = public_leaf_lineage(
        fixture.amount_value,
        fixture.token_account,
        fixture.mint,
        token::burned_amount_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        None,
    );
    assert_eq!(lineage.current_handle, pinned);

    let request_nonce = [0x43u8; 32];
    let (request_addr, request_account) = disclosure_request_account(
        fixture.mint,
        fixture.owner,
        Pubkey::default(),
        lineage.app_account,
        pinned,
        fixture.amount_value,
        fixture.host_config,
        fixture.kms_context_id,
        request_nonce,
        200,
        token::DISCLOSURE_REQUEST_MODE_AMOUNT,
        token::REQUEST_STATUS_PENDING,
    );

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_value, encrypted_value_account(&lineage));
    accounts.insert(request_addr, request_account);
    let context = mollusk().with_context(accounts);

    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, 500);
    context.process_and_validate_instruction(
        &disclose_amount_secp_ix(
            &fixture,
            request_addr,
            pinned,
            500,
            signatures,
            extra_data,
            proof,
        ),
        &[Check::success()],
    );
    assert_eq!(
        read_disclosure_request(&context, request_addr).status,
        token::REQUEST_STATUS_CONSUMED
    );
}

#[test]
fn mollusk_disclose_balance_consume_happy_path() {
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(33, BALANCE_FHE_TYPE);
    let (lineage, proof) = public_leaf_lineage(
        fixture.balance_value,
        fixture.token_account,
        fixture.mint,
        token::balance_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        None,
    );

    let request_nonce = [0x34u8; 32];
    let (request_addr, request_account) = disclosure_request_account(
        fixture.mint,
        fixture.owner,
        fixture.token_account,
        fixture.token_account,
        pinned,
        fixture.balance_value,
        fixture.host_config,
        fixture.kms_context_id,
        request_nonce,
        200,
        token::DISCLOSURE_REQUEST_MODE_BALANCE,
        token::REQUEST_STATUS_PENDING,
    );

    let mut accounts = fixture.base();
    accounts.insert(
        fixture.token_account,
        token_account_account(fixture.mint, fixture.owner, fixture.balance_value),
    );
    accounts.insert(fixture.balance_value, encrypted_value_account(&lineage));
    accounts.insert(request_addr, request_account);
    let context = mollusk().with_context(accounts);

    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, 700);
    context.process_and_validate_instruction(
        &disclose_balance_secp_ix(&fixture, request_addr, 700, signatures, extra_data, proof),
        &[Check::success()],
    );
    assert_eq!(
        read_disclosure_request(&context, request_addr).status,
        token::REQUEST_STATUS_CONSUMED
    );
}

#[test]
fn mollusk_disclose_amount_consume_once_rejects_second() {
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(44, BALANCE_FHE_TYPE);
    let (lineage, proof) = public_leaf_lineage(
        fixture.amount_value,
        fixture.token_account,
        fixture.mint,
        token::burned_amount_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        None,
    );

    let request_nonce = [0x44u8; 32];
    let (request_addr, request_account) = disclosure_request_account(
        fixture.mint,
        fixture.owner,
        Pubkey::default(),
        lineage.app_account,
        pinned,
        fixture.amount_value,
        fixture.host_config,
        fixture.kms_context_id,
        request_nonce,
        200,
        token::DISCLOSURE_REQUEST_MODE_AMOUNT,
        token::REQUEST_STATUS_PENDING,
    );

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_value, encrypted_value_account(&lineage));
    accounts.insert(request_addr, request_account);
    let context = mollusk().with_context(accounts);

    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, 500);
    context.process_and_validate_instruction(
        &disclose_amount_secp_ix(
            &fixture,
            request_addr,
            pinned,
            500,
            signatures.clone(),
            extra_data.clone(),
            proof.clone(),
        ),
        &[Check::success()],
    );
    // Second consume of the same (now CONSUMED) witness is rejected.
    context.process_and_validate_instruction(
        &disclose_amount_secp_ix(
            &fixture,
            request_addr,
            pinned,
            500,
            signatures,
            extra_data,
            proof,
        ),
        &[token_error(
            token::ConfidentialTokenError::RequestWitnessUnavailable,
        )],
    );
}

#[test]
fn mollusk_disclose_amount_rejects_expired_at_consume() {
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(45, BALANCE_FHE_TYPE);
    let (lineage, proof) = public_leaf_lineage(
        fixture.amount_value,
        fixture.token_account,
        fixture.mint,
        token::burned_amount_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        None,
    );

    let request_nonce = [0x45u8; 32];
    // Clock slot is 100; an expires_slot below it is past the consume window.
    let (request_addr, request_account) = disclosure_request_account(
        fixture.mint,
        fixture.owner,
        Pubkey::default(),
        lineage.app_account,
        pinned,
        fixture.amount_value,
        fixture.host_config,
        fixture.kms_context_id,
        request_nonce,
        50,
        token::DISCLOSURE_REQUEST_MODE_AMOUNT,
        token::REQUEST_STATUS_PENDING,
    );

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_value, encrypted_value_account(&lineage));
    accounts.insert(request_addr, request_account);
    let context = mollusk().with_context(accounts);

    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, 500);
    context.process_and_validate_instruction(
        &disclose_amount_secp_ix(
            &fixture,
            request_addr,
            pinned,
            500,
            signatures,
            extra_data,
            proof,
        ),
        &[token_error(
            token::ConfidentialTokenError::RequestWitnessUnavailable,
        )],
    );
}

#[test]
fn mollusk_disclose_amount_rejects_foreign_public_decrypt_proof() {
    // A structurally valid proof aimed at the WRONG leaf position (H2's public leaf, not H1's):
    // authorize_public recomputes public(H1)@leaf_index against the peaks and rejects it.
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(46, BALANCE_FHE_TYPE);
    let superseded = handle_for_chain(47, BALANCE_FHE_TYPE);
    let (lineage, mut proof) = public_leaf_lineage(
        fixture.amount_value,
        fixture.token_account,
        fixture.mint,
        token::burned_amount_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        Some(superseded),
    );
    proof.leaf_index = 3; // H2's public-decrypt leaf, not H1's.

    let request_nonce = [0x46u8; 32];
    let (request_addr, request_account) = disclosure_request_account(
        fixture.mint,
        fixture.owner,
        Pubkey::default(),
        lineage.app_account,
        pinned,
        fixture.amount_value,
        fixture.host_config,
        fixture.kms_context_id,
        request_nonce,
        200,
        token::DISCLOSURE_REQUEST_MODE_AMOUNT,
        token::REQUEST_STATUS_PENDING,
    );

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_value, encrypted_value_account(&lineage));
    accounts.insert(request_addr, request_account);
    let context = mollusk().with_context(accounts);

    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, 500);
    context.process_and_validate_instruction(
        &disclose_amount_secp_ix(
            &fixture,
            request_addr,
            pinned,
            500,
            signatures,
            extra_data,
            proof,
        ),
        &[token_error(
            token::ConfidentialTokenError::PublicDecryptProofInvalid,
        )],
    );
    assert_eq!(
        read_disclosure_request(&context, request_addr).status,
        token::REQUEST_STATUS_PENDING
    );
}

fn close_expired_disclosure_request_ix(
    requester: Pubkey,
    disclosure_request: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::CloseExpiredDisclosureRequest {
            requester,
            disclosure_request,
        },
        token::instruction::CloseExpiredDisclosureRequest {},
    )
}

fn close_consumed_disclosure_request_ix(
    requester: Pubkey,
    disclosure_request: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::CloseConsumedDisclosureRequest {
            requester,
            disclosure_request,
        },
        token::instruction::CloseConsumedDisclosureRequest {},
    )
}

fn seeded_disclosure_witness(
    fixture: &DiscloseFixture,
    request_nonce: [u8; 32],
    expires_slot: u64,
    status: u8,
) -> (Pubkey, Account) {
    let pinned = handle_for_chain(48, BALANCE_FHE_TYPE);
    disclosure_request_account(
        fixture.mint,
        fixture.owner,
        Pubkey::default(),
        fixture.token_account,
        pinned,
        fixture.amount_value,
        fixture.host_config,
        fixture.kms_context_id,
        request_nonce,
        expires_slot,
        token::DISCLOSURE_REQUEST_MODE_AMOUNT,
        status,
    )
}

#[test]
fn mollusk_close_expired_disclosure_request_succeeds_after_expiry() {
    let fixture = DiscloseFixture::new();
    // Clock slot is 100; expires_slot below it means expired-and-unconsumed.
    let (request_addr, request_account) =
        seeded_disclosure_witness(&fixture, [0x61u8; 32], 50, token::REQUEST_STATUS_PENDING);
    let mut accounts = fixture.base();
    accounts.insert(request_addr, request_account);
    let context = mollusk().with_context(accounts);

    context.process_and_validate_instruction(
        &close_expired_disclosure_request_ix(fixture.owner, request_addr),
        &[Check::success()],
    );
    assert!(account_is_system_owned_and_empty(&context, request_addr));
}

#[test]
fn mollusk_close_consumed_disclosure_request_succeeds_after_expiry() {
    let fixture = DiscloseFixture::new();
    let (request_addr, request_account) =
        seeded_disclosure_witness(&fixture, [0x62u8; 32], 50, token::REQUEST_STATUS_CONSUMED);
    let mut accounts = fixture.base();
    accounts.insert(request_addr, request_account);
    let context = mollusk().with_context(accounts);

    context.process_and_validate_instruction(
        &close_consumed_disclosure_request_ix(fixture.owner, request_addr),
        &[Check::success()],
    );
    assert!(account_is_system_owned_and_empty(&context, request_addr));
}

#[test]
fn mollusk_close_disclosure_request_before_expiry_rejected() {
    let fixture = DiscloseFixture::new();
    // expires_slot at/after the clock slot (100) is still within the live window.
    let (request_addr, request_account) =
        seeded_disclosure_witness(&fixture, [0x63u8; 32], 200, token::REQUEST_STATUS_PENDING);
    let mut accounts = fixture.base();
    accounts.insert(request_addr, request_account);
    let context = mollusk().with_context(accounts);

    context.process_and_validate_instruction(
        &close_expired_disclosure_request_ix(fixture.owner, request_addr),
        &[token_error(
            token::ConfidentialTokenError::RequestWitnessUnavailable,
        )],
    );
    assert!(!account_is_system_owned_and_empty(&context, request_addr));
}

// ===========================================================================
// HCU per-app block cap enforced through the confidential-token -> fhe_eval CPI.
//
// Ported from PR #2991 ("per-app HCU limit per block"), rewritten against the merged
// EncryptedValue durable-output model: `confidential_transfer` reaches `fhe_eval` only by CPI,
// so these tests prove the block cap (ban / metering-band charge / canonical-authority pinning)
// survives that CPI boundary — not just direct `fhe_eval` calls (see `host_mollusk.rs`).
//
// `create_random_amount`'s HCU tests from the same PR are NOT ported here: this file's
// migration intentionally dropped `create_random_amount` coverage entirely (see the module doc
// comment above — no instruction builder or fixture for it exists in this file), and building
// one from scratch is a materially larger lift than threading the HCU accounts already present
// on the fixtures ported here.
// ===========================================================================

/// Exact HCU cost of the combined transfer eval frame (`execute_transfer_eval`): `Ge` at ebool
/// (21_000) + debit `Sub` at euint64 (38_000) + `IfThenElse` at euint64 (45_000) + transferred
/// `Sub` at euint64 (38_000) + credit `Add` at euint64 (38_000). The `VerifiedInput` amount is an
/// operand, not a step, so it adds no HCU.
const TRANSFER_FRAME_HCU: u64 = 21_000 + 38_000 + 45_000 + 38_000 + 38_000; // 180_000

/// The fixture's host config with the per-app block cap overridden to `cap`.
fn host_config_account_with_block_cap(
    admin: Pubkey,
    coprocessor_signer: [u8; 20],
    cap: u64,
) -> Account {
    let mut account = host_config_account(admin, coprocessor_signer);
    let mut config = {
        let mut data = account.data.as_slice();
        host::HostConfig::try_deserialize(&mut data).expect("valid host config")
    };
    config.hcu_block_cap_per_app = cap;
    account.data = serialized_account(config);
    account
}

fn read_hcu_block_meter(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<host::HcuBlockMeter> {
    let store = context.account_store.borrow();
    let account = store.get(&address)?;
    if account.owner != host::id() {
        return None;
    }
    let mut data = account.data.as_slice();
    host::HcuBlockMeter::try_deserialize(&mut data).ok()
}

#[test]
fn mollusk_confidential_transfer_block_cap_ban_is_enforced_through_cpi() {
    // A confidential transfer reaches fhe_eval only by CPI. With the cap at the ban sentinel
    // (0) and no trust witness threaded, the block-cap breach must surface through the CPI and
    // roll the whole transfer back atomically — exactly as a direct fhe_eval call is rejected.
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    accounts.insert(
        fixture.host_config,
        host_config_account_with_block_cap(
            fixture.owner,
            secp_evm_address(&coprocessor_signing_key()),
            0,
        ),
    );
    let context = mollusk().with_context(accounts);
    let amount_handle = handle_for_chain(200, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );

    context.process_and_validate_instruction(
        &ix,
        &[host_error(
            host::errors::ZamaHostError::HcuBlockLimitExceeded,
        )],
    );

    // Atomic revert: both balances are unchanged.
    assert_eq!(
        read_encrypted_value(&context, fixture.alice_balance_value).current_handle,
        fixture.alice_initial
    );
    assert_eq!(
        read_encrypted_value(&context, fixture.bob_balance_value).current_handle,
        fixture.bob_initial
    );
}

#[test]
fn mollusk_confidential_transfer_rejects_non_canonical_hcu_authority() {
    // The token program pins the mandatory HCU authority to the canonical
    // ["hcu-authority", mint] PDA before signing it into the CPI — an arbitrary account in
    // that slot (e.g. another mint's authority, to spend its budget) is rejected up front,
    // even while the host cap is unrestricted.
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(201, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let ix = confidential_transfer_ix_with_block_cap_accounts(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
        Vec::new(),
        None,
        None,
        token::hcu_authority_address(Pubkey::new_unique()).0,
    );

    context.process_and_validate_instruction(
        &ix,
        &[token_error(
            token::ConfidentialTokenError::HcuAuthorityMismatch,
        )],
    );
    assert_eq!(
        read_encrypted_value(&context, fixture.alice_balance_value).current_handle,
        fixture.alice_initial
    );
}

#[test]
fn mollusk_confidential_transfer_metering_band_charges_meter_through_cpi() {
    // The Some(meter) CPI shape — the production account set once the cap drops below
    // u64::MAX. With a metering-band cap, the mint's HCU authority signed in, and the meter
    // threaded through ConfidentialTransfer, the transfer must succeed and the meter must be
    // lazy-created and charged with exactly the frame's HCU, proving the three optional
    // accounts survive the token -> zama-fhe -> fhe_eval CPI encoding end to end. The metering
    // identity is the mint's ["hcu-authority", mint] PDA — one budget per mint, NOT per sender
    // token account.
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    accounts.insert(
        fixture.host_config,
        host_config_account_with_block_cap(
            fixture.owner,
            secp_evm_address(&coprocessor_signing_key()),
            500_000,
        ),
    );
    let context = mollusk().with_context(accounts);
    let amount_handle = handle_for_chain(21, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let hcu_authority = token::hcu_authority_address(fixture.mint).0;
    let meter_pda = host::hcu_block_meter_address(hcu_authority).0;
    let ix = confidential_transfer_ix_with_block_cap_accounts(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
        Vec::new(),
        Some(meter_pda),
        None,
        hcu_authority,
    );

    context.process_and_validate_instruction(&ix, &[Check::success()]);

    // The transfer completed: the sender's balance lineage moved off its initial handle.
    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    assert_ne!(alice_balance.current_handle, fixture.alice_initial);
    // The meter was lazy-created through the CPI, keyed on the mint's HCU authority, and
    // charged exactly the transfer frame's HCU at the current slot.
    let meter = read_hcu_block_meter(&context, meter_pda).expect("meter created through CPI");
    assert_eq!(meter.app, hcu_authority);
    assert_eq!(meter.used_hcu, TRANSFER_FRAME_HCU);
    assert_eq!(meter.last_seen_slot, context.mollusk.sysvars.clock.slot);
    // Regression guard on the metering granularity: nothing accrues to the sender token
    // account's key — a sybil minting fresh token accounts gets no fresh budget.
    assert!(read_hcu_block_meter(
        &context,
        host::hcu_block_meter_address(fixture.alice_token).0
    )
    .is_none());
}
