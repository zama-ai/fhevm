//! Mollusk-based runtime tests for the `confidential-batcher` — both the
//! deposit and the redeem direction.
//!
//! The batcher composes four programs — zama-host (FHE compute + ACL),
//! confidential-token (transfers, burn, redeem, wrap), demo-vault (public
//! share pricing), and SPL Token — so this harness registers all of them and
//! drives the REAL batcher instructions end to end. Every CPI the batcher
//! issues under its per-batch authority PDA is therefore exercised through a
//! genuine `invoke_signed` (init token account, attested transfer, transfer
//! from value, whole-balance burn, redeem, vault deposit/withdraw, wrap), not
//! through the marked-signer stand-in used by the token suite's PDA-owner
//! test.
//!
//! Encrypted state is checked with the cleartext ledger: each instruction's
//! `fhe_eval` CPIs (there can be several — a token CPI's eval plus the
//! batcher's own) are decoded from the inner instructions and replayed in
//! cleartext, binding results to the handles the host persisted.
//!
//! The fixture is direction-parametric: the same two confidential mints (one
//! wrapping the vault's underlying, one wrapping its share mint) serve a
//! deposit batcher (join = underlying, payout = shares) or a redeem batcher
//! (join = shares, payout = underlying).

mod support;

// Deliberate `#[path]` include (not `support::cost_snapshot`): each Mollusk
// binary compiles its own copy.
#[path = "support/cost_snapshot.rs"]
mod cost_snapshot;

use anchor_lang::{
    prelude::system_program, AccountDeserialize, AccountSerialize, AnchorDeserialize,
    Discriminator, InstructionData, ToAccountMetas,
};
use anchor_spl::token::spl_token;
use confidential_batcher as batcher;
use confidential_token as token;
use demo_vault as vault;
use mollusk_svm::{
    result::{Check, InstructionResult},
    Mollusk,
};
use solana_sdk::{
    account::Account, instruction::Instruction, program_error::ProgramError,
    program_option::COption, program_pack::Pack, pubkey::Pubkey,
};
use std::collections::HashMap;
use std::path::PathBuf;
use support::cleartext_fhe_eval::{evaluate as evaluate_cleartext, ClearInputs, TypedClearValue};
use support::kms_cert::{cleartext_u256, kms_signing_key, secp_evm_address, secp_sign};
use zama_host as host;

const BALANCE_FHE_TYPE: u8 = 5;
const GATEWAY_CHAIN_ID: u64 = 31337;
const INPUT_VERIFICATION_CONTRACT: [u8; 20] = [0xCDu8; 20];
const DECRYPTION_CONTRACT: [u8; 20] = [0xDEu8; 20];
const KMS_CONTEXT_ID: u64 = 9;
const DECIMALS: u8 = 6;
/// Generous batch-authority funding for owner-charged rent (token-account and
/// encrypted value account creation at open; the redeem marker and wrap growth at settle).
const AUTHORITY_FUNDING: u64 = 100_000_000;

type Ctx = mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>;

// ---------------------------------------------------------------------------
// Harness
// ---------------------------------------------------------------------------

fn mollusk() -> Mollusk {
    let deploy_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy");
    unsafe {
        std::env::set_var("SBF_OUT_DIR", deploy_dir);
    }
    let mut mollusk = Mollusk::new(&batcher::id(), "confidential_batcher");
    mollusk.add_program(&host::id(), "zama_host");
    mollusk.add_program(&token::id(), "confidential_token");
    mollusk.add_program(&vault::id(), "demo_vault");
    mollusk_svm_programs_token::token::add_program(&mut mollusk);
    // fhe_eval derives handle entropy from the previous bank hash: run at a
    // nonzero slot with a SlotHashes entry below it, like a real validator.
    mollusk.sysvars.clock.slot = 100;
    mollusk.sysvars.slot_hashes =
        solana_sdk::slot_hashes::SlotHashes::new(&[(99, solana_sdk::hash::Hash::new_unique())]);
    // Batcher instructions chain a token eval and the batcher's own eval;
    // real transactions request the same higher limit.
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

fn batcher_error(error: batcher::BatcherError) -> Check<'static> {
    Check::err(ProgramError::Custom(
        anchor_lang::error::ERROR_CODE_OFFSET + error as u32,
    ))
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

/// Cleartext mirror of every encrypted handle the tests touch.
#[derive(Default)]
struct CleartextLedger {
    values: ClearInputs,
}

impl CleartextLedger {
    fn seed_amount(&mut self, handle: [u8; 32], value: u64) {
        self.values
            .insert(handle, TypedClearValue::from_u64(BALANCE_FHE_TYPE, value));
    }

    /// Replays every `fhe_eval` CPI a batcher instruction issued — in order,
    /// so a later eval can consume an earlier eval's persisted outputs (the
    /// join re-materialization reads the transfer's `transferred_amount`).
    /// Each instruction writes any encrypted value account at most once, so binding results to
    /// the end-of-instruction persisted handles is exact.
    fn evaluate_fhe_cpis(&mut self, context: &Ctx, result: &InstructionResult) -> usize {
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
        assert!(
            !eval_args.is_empty(),
            "expected at least one fhe_eval CPI in this instruction"
        );

        let mut evals = 0;
        for args in &eval_args {
            let outputs = evaluate_cleartext(args, &self.values)
                .expect("every emitted FHE plan must be valid in cleartext");
            for (step, value) in args.steps.iter().zip(outputs) {
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
            }
            evals += 1;
        }
        evals
    }

    fn u64_at(&self, context: &Ctx, encrypted_value: Pubkey) -> u64 {
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

fn read_encrypted_value(context: &Ctx, address: Pubkey) -> host::EncryptedValue {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing encrypted value account")
        .clone();
    host::EncryptedValue::try_deserialize(&mut account.data.as_slice())
        .expect("valid EncryptedValue account")
}

fn read_batch(context: &Ctx, address: Pubkey) -> batcher::Batch {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing batch account")
        .clone();
    batcher::Batch::try_deserialize(&mut account.data.as_slice()).expect("valid batch account")
}

fn read_join_record(context: &Ctx, address: Pubkey) -> batcher::JoinRecord {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing join record")
        .clone();
    batcher::JoinRecord::try_deserialize(&mut account.data.as_slice()).expect("valid join record")
}

fn read_spl_amount(context: &Ctx, address: Pubkey) -> u64 {
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

/// Inserts fresh, empty system accounts for addresses an instruction will
/// create (Mollusk requires every referenced account to exist in the store).
fn ensure_system_accounts(context: &Ctx, addresses: &[Pubkey]) {
    let mut store = context.account_store.borrow_mut();
    for address in addresses {
        store.entry(*address).or_insert_with(|| system_account(0));
    }
}

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

fn spl_mint_account(mint_authority: Option<Pubkey>, supply: u64) -> Account {
    let mut data = vec![0u8; spl_token::state::Mint::LEN];
    spl_token::state::Mint::pack(
        spl_token::state::Mint {
            mint_authority: mint_authority.map(COption::Some).unwrap_or(COption::None),
            supply,
            decimals: DECIMALS,
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

// ---------------------------------------------------------------------------
// Attestations and KMS certs
// ---------------------------------------------------------------------------

fn coprocessor_signing_key() -> k256::ecdsa::SigningKey {
    k256::ecdsa::SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap()
}

/// Coprocessor-signed `fromExternal` attestation over `amount_handle`, bound
/// to (`user`, the join mint's compute-signer PDA).
fn amount_attestation_for(
    amount_handle: [u8; 32],
    user: Pubkey,
    contract: Pubkey,
) -> host::CoprocessorInputAttestation {
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
        signatures: vec![secp_sign(&coprocessor_signing_key(), &digest)],
    }
}

/// KMS `PublicDecryptVerification` cert over the burned batch total.
fn kms_public_decrypt_cert(handle: [u8; 32], cleartext_amount: u64) -> (Vec<[u8; 65]>, Vec<u8>) {
    let extra_data = vec![0x00u8];
    let signatures = support::kms_cert::kms_public_decrypt_cert_signed_by(
        handle,
        cleartext_u256(cleartext_amount),
        GATEWAY_CHAIN_ID,
        &DECRYPTION_CONTRACT,
        &extra_data,
        &[kms_signing_key()],
    );
    (signatures, extra_data)
}

/// Public-decrypt inclusion proof for the batch's single-burn encrypted value account (the
/// sole leaf, at index 0, is the burned handle's public-decrypt commitment).
fn single_burn_public_decrypt_proof(
    burned_amount_value: Pubkey,
    burned_handle: [u8; 32],
) -> host::instructions::MmrInclusionProof {
    let leaves = vec![zama_solana_acl::public_decrypt_leaf_commitment(
        burned_amount_value.to_bytes(),
        0,
        burned_handle,
    )];
    let proof = zama_solana_acl::mmr_build_proof(&leaves, 0).expect("proof for the sole burn leaf");
    host::instructions::MmrInclusionProof {
        leaf_index: proof.leaf_index,
        siblings: proof.siblings,
    }
}

// ---------------------------------------------------------------------------
// Fixture
// ---------------------------------------------------------------------------

/// One confidential mint with its full PDA family.
struct ConfidentialMintKeys {
    mint: Pubkey,
    underlying_mint: Pubkey,
    compute_signer: Pubkey,
    total_supply_authority: Pubkey,
    total_supply_value: Pubkey,
    vault_authority: Pubkey,
    vault_underlying: Pubkey,
    initial_total_supply: [u8; 32],
}

impl ConfidentialMintKeys {
    fn new(mint: Pubkey, underlying_mint: Pubkey, total_supply_seed: u8) -> Self {
        let total_supply_authority = token::total_supply_authority_address(mint).0;
        Self {
            mint,
            underlying_mint,
            compute_signer: token::compute_signer_address(mint).0,
            total_supply_authority,
            total_supply_value: token::total_supply_encrypted_value_address(
                mint,
                total_supply_authority,
            )
            .0,
            vault_authority: token::vault_authority_address(mint).0,
            vault_underlying: token::vault_token_account_address(mint, underlying_mint),
            initial_total_supply: handle_for_chain(total_supply_seed, BALANCE_FHE_TYPE),
        }
    }

    fn mint_account(&self, authority: Pubkey) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialMint {
                authority,
                acl_domain_key: self.mint,
                compute_signer: self.compute_signer,
                underlying_mint: self.underlying_mint,
                decimals: DECIMALS,
                total_supply_encrypted_value: self.total_supply_value,
            }),
            owner: token::id(),
            executable: false,
            rent_epoch: 0,
        }
    }
}

/// One user's accounts on one confidential mint.
struct UserMintKeys {
    token_account: Pubkey,
    balance_value: Pubkey,
    transferred_value: Pubkey,
    initial_balance: [u8; 32],
}

impl UserMintKeys {
    fn new(user: Pubkey, mint: &ConfidentialMintKeys, seed: u8) -> Self {
        let token_account = token::token_account_address(mint.mint, user).0;
        Self {
            token_account,
            balance_value: token::balance_encrypted_value_address(mint.mint, token_account).0,
            transferred_value: token::encrypted_value_address(
                mint.mint,
                token_account,
                token::transferred_amount_label(),
            )
            .0,
            initial_balance: handle_for_chain(seed, BALANCE_FHE_TYPE),
        }
    }
}

/// One user with token accounts on both confidential mints.
struct UserKeys {
    user: Pubkey,
    /// The user's accounts on the confidential mint wrapping the vault's
    /// underlying (the join side of deposit batchers, the payout side of
    /// redeem batchers).
    underlying: UserMintKeys,
    /// The user's accounts on the confidential mint wrapping the vault's
    /// share mint (the payout side of deposit batchers, the join side of
    /// redeem batchers).
    shares: UserMintKeys,
}

impl UserKeys {
    fn new(
        user: Pubkey,
        fixture_mints: (&ConfidentialMintKeys, &ConfidentialMintKeys),
        seed: u8,
    ) -> Self {
        let (underlying_cmint, shares_cmint) = fixture_mints;
        Self {
            user,
            underlying: UserMintKeys::new(user, underlying_cmint, seed),
            shares: UserMintKeys::new(user, shares_cmint, seed + 1),
        }
    }
}

/// Per-batch derived addresses, resolved for the fixture's direction.
struct BatchKeys {
    batch: Pubkey,
    batch_authority: Pubkey,
    join_token_account: Pubkey,
    join_balance_value: Pubkey,
    join_transferred_value: Pubkey,
    burned_amount_value: Pubkey,
    payout_token_account: Pubkey,
    payout_balance_value: Pubkey,
    payout_transferred_value: Pubkey,
    join_underlying: Pubkey,
    payout_underlying: Pubkey,
}

impl BatchKeys {
    fn new(fixture: &BatcherFixture, index: u64) -> Self {
        let batch = batcher::batch_address(fixture.batcher, index).0;
        let batch_authority = batcher::batch_authority_address(batch).0;
        let join_mint = fixture.join_mint();
        let payout_mint = fixture.payout_mint();
        let join_token_account = token::token_account_address(join_mint.mint, batch_authority).0;
        let payout_token_account =
            token::token_account_address(payout_mint.mint, batch_authority).0;
        Self {
            batch,
            batch_authority,
            join_token_account,
            join_balance_value: token::balance_encrypted_value_address(
                join_mint.mint,
                join_token_account,
            )
            .0,
            join_transferred_value: token::encrypted_value_address(
                join_mint.mint,
                join_token_account,
                token::transferred_amount_label(),
            )
            .0,
            burned_amount_value: token::encrypted_value_address(
                join_mint.mint,
                join_token_account,
                token::burned_amount_label(),
            )
            .0,
            payout_token_account,
            payout_balance_value: token::balance_encrypted_value_address(
                payout_mint.mint,
                payout_token_account,
            )
            .0,
            payout_transferred_value: token::encrypted_value_address(
                payout_mint.mint,
                payout_token_account,
                token::transferred_amount_label(),
            )
            .0,
            join_underlying: batcher::batch_join_underlying_address(batch).0,
            payout_underlying: batcher::batch_payout_underlying_address(batch).0,
        }
    }

    fn pending_join_value(&self, user: Pubkey) -> Pubkey {
        batcher::batcher_encrypted_value_address(
            self.batch,
            self.batch_authority,
            batcher::pending_join_label(user),
        )
        .0
    }

    fn claim_amount_value(&self, user: Pubkey) -> Pubkey {
        batcher::batcher_encrypted_value_address(
            self.batch,
            self.batch_authority,
            batcher::claim_amount_label(user),
        )
        .0
    }

    fn join_record(&self, user: Pubkey) -> Pubkey {
        batcher::join_record_address(self.batch, user).0
    }
}

struct BatcherFixture {
    direction: batcher::BatchDirection,
    payer: Pubkey,
    batcher: Pubkey,
    /// Confidential mint wrapping the vault's underlying mint.
    underlying_cmint: ConfidentialMintKeys,
    /// Confidential mint wrapping the vault's share mint.
    shares_cmint: ConfidentialMintKeys,
    vault: Pubkey,
    vault_authority: Pubkey,
    share_mint: Pubkey,
    vault_token_account: Pubkey,
    underlying_mint: Pubkey,
    host_config: Pubkey,
    kms_context: Pubkey,
    alice: UserKeys,
    bob: UserKeys,
}

impl BatcherFixture {
    fn new(direction: batcher::BatchDirection) -> Self {
        Self::with_keys(
            direction,
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
        )
    }

    /// Fixed-key variant for cost snapshots: PDA bump searches are part of the
    /// measured compute, so profile addresses must not change between runs.
    fn fixed(direction: batcher::BatchDirection, seed: u8) -> Self {
        Self::with_keys(
            direction,
            Pubkey::new_from_array([seed; 32]),
            Pubkey::new_from_array([seed.wrapping_add(1); 32]),
            Pubkey::new_from_array([seed.wrapping_add(2); 32]),
            Pubkey::new_from_array([seed.wrapping_add(3); 32]),
            Pubkey::new_from_array([seed.wrapping_add(4); 32]),
            Pubkey::new_from_array([seed.wrapping_add(5); 32]),
            Pubkey::new_from_array([seed.wrapping_add(6); 32]),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn with_keys(
        direction: batcher::BatchDirection,
        payer: Pubkey,
        batcher_key: Pubkey,
        underlying_cmint: Pubkey,
        shares_cmint: Pubkey,
        underlying_mint: Pubkey,
        vault_key: Pubkey,
        users_seed: Pubkey,
    ) -> Self {
        let vault_authority =
            Pubkey::find_program_address(&[b"authority", vault_key.as_ref()], &vault::id()).0;
        let share_mint =
            Pubkey::find_program_address(&[b"shares", vault_key.as_ref()], &vault::id()).0;
        let vault_token_account =
            Pubkey::find_program_address(&[b"underlying", vault_key.as_ref()], &vault::id()).0;
        let underlying_cmint = ConfidentialMintKeys::new(underlying_cmint, underlying_mint, 3);
        let shares_cmint = ConfidentialMintKeys::new(shares_cmint, share_mint, 4);
        // Derive two deterministic user keys from the seed key so the fixed
        // fixture stays stable across runs.
        let mut alice_bytes = users_seed.to_bytes();
        alice_bytes[31] = alice_bytes[31].wrapping_add(1);
        let mut bob_bytes = users_seed.to_bytes();
        bob_bytes[31] = bob_bytes[31].wrapping_add(2);
        let alice = UserKeys::new(
            Pubkey::new_from_array(alice_bytes),
            (&underlying_cmint, &shares_cmint),
            10,
        );
        let bob = UserKeys::new(
            Pubkey::new_from_array(bob_bytes),
            (&underlying_cmint, &shares_cmint),
            20,
        );
        Self {
            direction,
            payer,
            batcher: batcher_key,
            underlying_cmint,
            shares_cmint,
            vault: vault_key,
            vault_authority,
            share_mint,
            vault_token_account,
            underlying_mint,
            host_config: host::host_config_address().0,
            kms_context: host::kms_context_address(KMS_CONTEXT_ID).0,
            alice,
            bob,
        }
    }

    /// The confidential mint users join batches with.
    fn join_mint(&self) -> &ConfidentialMintKeys {
        match self.direction {
            batcher::BatchDirection::Deposit => &self.underlying_cmint,
            batcher::BatchDirection::Redeem => &self.shares_cmint,
        }
    }

    /// The confidential mint claims pay out in.
    fn payout_mint(&self) -> &ConfidentialMintKeys {
        match self.direction {
            batcher::BatchDirection::Deposit => &self.shares_cmint,
            batcher::BatchDirection::Redeem => &self.underlying_cmint,
        }
    }

    /// The given user's accounts on the join mint.
    fn user_join<'a>(&self, user: &'a UserKeys) -> &'a UserMintKeys {
        match self.direction {
            batcher::BatchDirection::Deposit => &user.underlying,
            batcher::BatchDirection::Redeem => &user.shares,
        }
    }

    /// The given user's accounts on the payout mint.
    fn user_payout<'a>(&self, user: &'a UserKeys) -> &'a UserMintKeys {
        match self.direction {
            batcher::BatchDirection::Deposit => &user.shares,
            batcher::BatchDirection::Redeem => &user.underlying,
        }
    }

    /// A redeem-direction batcher instance over this fixture's exact world:
    /// same mints, vault, users, and payer — only the batcher config account
    /// differs. The two-instance pattern for the concurrency test.
    fn redeem_twin(&self, batcher_key: Pubkey) -> Self {
        Self {
            direction: batcher::BatchDirection::Redeem,
            payer: self.payer,
            batcher: batcher_key,
            underlying_cmint: ConfidentialMintKeys::new(
                self.underlying_cmint.mint,
                self.underlying_mint,
                3,
            ),
            shares_cmint: ConfidentialMintKeys::new(self.shares_cmint.mint, self.share_mint, 4),
            vault: self.vault,
            vault_authority: self.vault_authority,
            share_mint: self.share_mint,
            vault_token_account: self.vault_token_account,
            underlying_mint: self.underlying_mint,
            host_config: self.host_config,
            kms_context: self.kms_context,
            alice: UserKeys::new(
                self.alice.user,
                (&self.underlying_cmint, &self.shares_cmint),
                10,
            ),
            bob: UserKeys::new(
                self.bob.user,
                (&self.underlying_cmint, &self.shares_cmint),
                20,
            ),
        }
    }

    fn host_config_account(&self) -> Account {
        let (_, bump) = host::host_config_address();
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(host::HostConfig {
                admin: self.payer,
                chain_id: host::SOLANA_POC_CHAIN_ID,
                gateway_chain_id: GATEWAY_CHAIN_ID,
                input_verification_contract: INPUT_VERIFICATION_CONTRACT,
                coprocessor_signers: host::pack_coprocessor_signers(&[secp_evm_address(
                    &coprocessor_signing_key(),
                )]),
                coprocessor_signer_count: 1,
                coprocessor_threshold: 1,
                decryption_contract: DECRYPTION_CONTRACT,
                current_kms_context_id: KMS_CONTEXT_ID,
                paused: false,
                grant_deny_list_enabled: false,
                max_hcu_per_tx: 0,
                max_hcu_depth_per_tx: 0,
                hcu_block_cap_per_app: u64::MAX,
                updated_slot: 0,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    fn kms_context_account(&self) -> Account {
        let (_, bump) = host::kms_context_address(KMS_CONTEXT_ID);
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(host::KmsContext {
                context_id: KMS_CONTEXT_ID,
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

    fn vault_account(&self) -> Account {
        let authority_bump =
            Pubkey::find_program_address(&[b"authority", self.vault.as_ref()], &vault::id()).1;
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(vault::Vault {
                underlying_mint: self.underlying_mint,
                share_mint: self.share_mint,
                vault_token_account: self.vault_token_account,
                authority_bump,
            }),
            owner: vault::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    fn confidential_token_account(
        &self,
        mint: &ConfidentialMintKeys,
        owner: Pubkey,
        balance_value: Pubkey,
    ) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialTokenAccount {
                owner,
                mint: mint.mint,
                balance_encrypted_value: balance_value,
                bump: token::token_account_address(mint.mint, owner).1,
            }),
            owner: token::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    /// Full account set: host + KMS fixtures, both confidential mints, the
    /// demo vault at `(total_assets, total_shares)`, and both users with
    /// seeded balance encrypted value accounts on both mints. The confidential mints' plain
    /// escrows hold `underlying_escrow` / `shares_escrow` — deposit tests
    /// escrow underlying (the users' shielded deposits), redeem tests escrow
    /// vault shares (the users' shielded share positions).
    fn accounts_with_escrows(
        &self,
        vault_total_assets: u64,
        vault_total_shares: u64,
        underlying_escrow: u64,
        shares_escrow: u64,
    ) -> HashMap<Pubkey, Account> {
        let mut accounts = HashMap::from([
            (self.payer, system_account(50_000_000_000)),
            (self.alice.user, system_account(50_000_000_000)),
            (self.bob.user, system_account(50_000_000_000)),
            (self.batcher, system_account(0)),
            (self.host_config, self.host_config_account()),
            (self.kms_context, self.kms_context_account()),
            (self.underlying_mint, spl_mint_account(None, 1_000_000_000)),
            (
                self.share_mint,
                spl_mint_account(Some(self.vault_authority), vault_total_shares),
            ),
            (self.vault, self.vault_account()),
            (self.vault_authority, system_account(0)),
            (
                self.vault_token_account,
                spl_token_account(
                    self.underlying_mint,
                    self.vault_authority,
                    vault_total_assets,
                ),
            ),
            (event_authority(host::id()), system_account(0)),
            (event_authority(token::id()), system_account(0)),
            mollusk_svm_programs_token::token::keyed_account(),
        ]);
        for (mint, escrow_amount) in [
            (&self.underlying_cmint, underlying_escrow),
            (&self.shares_cmint, shares_escrow),
        ] {
            accounts.insert(mint.mint, mint.mint_account(self.payer));
            accounts.insert(mint.compute_signer, system_account(0));
            accounts.insert(mint.total_supply_authority, system_account(0));
            accounts.insert(mint.vault_authority, system_account(0));
            accounts.insert(
                mint.vault_underlying,
                spl_token_account(mint.underlying_mint, mint.vault_authority, escrow_amount),
            );
            let (_, total_supply) = new_encrypted_value(
                mint.mint,
                mint.total_supply_authority,
                token::total_supply_label(),
                mint.initial_total_supply,
                &[mint.compute_signer],
            );
            accounts.insert(
                mint.total_supply_value,
                encrypted_value_account(&total_supply),
            );
        }
        for user in [&self.alice, &self.bob] {
            for (mint, keys) in [
                (&self.underlying_cmint, &user.underlying),
                (&self.shares_cmint, &user.shares),
            ] {
                accounts.insert(
                    keys.token_account,
                    self.confidential_token_account(mint, user.user, keys.balance_value),
                );
                let (_, balance) = new_encrypted_value(
                    mint.mint,
                    keys.token_account,
                    token::balance_label(),
                    keys.initial_balance,
                    &[user.user, mint.compute_signer],
                );
                accounts.insert(keys.balance_value, encrypted_value_account(&balance));
            }
        }
        accounts
    }

    /// Deposit-shaped account set: the underlying escrow holds the users'
    /// shielded deposits, the shares escrow starts empty.
    fn accounts(
        &self,
        vault_total_assets: u64,
        vault_total_shares: u64,
    ) -> HashMap<Pubkey, Account> {
        self.accounts_with_escrows(vault_total_assets, vault_total_shares, 1_000_000, 0)
    }

    /// Seeds ledger values for the fixture's initial handles:
    /// per-user `(underlying, shares)` balances and both encrypted supplies.
    fn seed_ledger(
        &self,
        ledger: &mut CleartextLedger,
        alice: (u64, u64),
        bob: (u64, u64),
        supplies: (u64, u64),
    ) {
        ledger.seed_amount(self.alice.underlying.initial_balance, alice.0);
        ledger.seed_amount(self.alice.shares.initial_balance, alice.1);
        ledger.seed_amount(self.bob.underlying.initial_balance, bob.0);
        ledger.seed_amount(self.bob.shares.initial_balance, bob.1);
        ledger.seed_amount(self.underlying_cmint.initial_total_supply, supplies.0);
        ledger.seed_amount(self.shares_cmint.initial_total_supply, supplies.1);
    }
}

// ---------------------------------------------------------------------------
// Instruction builders
// ---------------------------------------------------------------------------

fn initialize_batcher_ix(fixture: &BatcherFixture, min_batch_age_slots: u64) -> Instruction {
    anchor_ix(
        batcher::id(),
        batcher::accounts::InitializeBatcher {
            payer: fixture.payer,
            batcher: fixture.batcher,
            join_confidential_mint: fixture.join_mint().mint,
            payout_confidential_mint: fixture.payout_mint().mint,
            vault: fixture.vault,
            system_program: system_program::ID,
        },
        batcher::instruction::InitializeBatcher {
            min_batch_age_slots,
            direction: fixture.direction,
        },
    )
}

fn open_batch_ix(
    fixture: &BatcherFixture,
    keys: &BatchKeys,
    previous_batch: Option<Pubkey>,
) -> Instruction {
    anchor_ix(
        batcher::id(),
        batcher::accounts::OpenBatch {
            payer: fixture.payer,
            batcher: fixture.batcher,
            previous_batch,
            batch: keys.batch,
            batch_authority: keys.batch_authority,
            join_confidential_mint: fixture.join_mint().mint,
            join_compute_signer: fixture.join_mint().compute_signer,
            batch_join_token_account: keys.join_token_account,
            batch_join_balance_value: keys.join_balance_value,
            payout_confidential_mint: fixture.payout_mint().mint,
            payout_compute_signer: fixture.payout_mint().compute_signer,
            batch_payout_token_account: keys.payout_token_account,
            batch_payout_balance_value: keys.payout_balance_value,
            join_underlying_mint: fixture.join_mint().underlying_mint,
            payout_underlying_mint: fixture.payout_mint().underlying_mint,
            batch_join_underlying: keys.join_underlying,
            batch_payout_underlying: keys.payout_underlying,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            confidential_token_event_authority: event_authority(token::id()),
            confidential_token_program: token::id(),
            token_program: spl_token::id(),
            system_program: system_program::ID,
        },
        batcher::instruction::OpenBatch {
            authority_funding_lamports: AUTHORITY_FUNDING,
        },
    )
}

fn join_ix(
    fixture: &BatcherFixture,
    keys: &BatchKeys,
    user: &UserKeys,
    amount_attestation: host::CoprocessorInputAttestation,
) -> Instruction {
    let user_join = fixture.user_join(user);
    anchor_ix(
        batcher::id(),
        batcher::accounts::Join {
            user: user.user,
            payer: user.user,
            batcher: fixture.batcher,
            batch: keys.batch,
            batch_authority: keys.batch_authority,
            join_record: keys.join_record(user.user),
            join_confidential_mint: fixture.join_mint().mint,
            join_compute_signer: fixture.join_mint().compute_signer,
            user_token_account: user_join.token_account,
            batch_join_token_account: keys.join_token_account,
            user_balance_value: user_join.balance_value,
            batch_balance_value: keys.join_balance_value,
            user_transferred_value: user_join.transferred_value,
            pending_join_value: keys.pending_join_value(user.user),
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            confidential_token_event_authority: event_authority(token::id()),
            confidential_token_program: token::id(),
            system_program: system_program::ID,
        },
        batcher::instruction::Join { amount_attestation },
    )
}

fn quit_ix(fixture: &BatcherFixture, keys: &BatchKeys, user: &UserKeys) -> Instruction {
    let user_join = fixture.user_join(user);
    anchor_ix(
        batcher::id(),
        batcher::accounts::Quit {
            user: user.user,
            payer: user.user,
            batcher: fixture.batcher,
            batch: keys.batch,
            batch_authority: keys.batch_authority,
            join_record: keys.join_record(user.user),
            join_confidential_mint: fixture.join_mint().mint,
            join_compute_signer: fixture.join_mint().compute_signer,
            batch_join_token_account: keys.join_token_account,
            user_token_account: user_join.token_account,
            batch_balance_value: keys.join_balance_value,
            user_balance_value: user_join.balance_value,
            batch_transferred_value: keys.join_transferred_value,
            pending_join_value: keys.pending_join_value(user.user),
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            confidential_token_event_authority: event_authority(token::id()),
            confidential_token_program: token::id(),
            system_program: system_program::ID,
        },
        batcher::instruction::Quit {},
    )
}

fn dispatch_ix(fixture: &BatcherFixture, keys: &BatchKeys) -> Instruction {
    anchor_ix(
        batcher::id(),
        batcher::accounts::Dispatch {
            payer: fixture.payer,
            batcher: fixture.batcher,
            batch: keys.batch,
            batch_authority: keys.batch_authority,
            join_confidential_mint: fixture.join_mint().mint,
            join_compute_signer: fixture.join_mint().compute_signer,
            total_supply_authority: fixture.join_mint().total_supply_authority,
            batch_join_token_account: keys.join_token_account,
            batch_balance_value: keys.join_balance_value,
            total_supply_value: fixture.join_mint().total_supply_value,
            batch_burned_amount_value: keys.burned_amount_value,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            confidential_token_event_authority: event_authority(token::id()),
            confidential_token_program: token::id(),
            system_program: system_program::ID,
        },
        batcher::instruction::Dispatch {},
    )
}

fn settle_ix(
    fixture: &BatcherFixture,
    keys: &BatchKeys,
    cleartext_total: u64,
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
    proof: host::instructions::MmrInclusionProof,
    redemption_record: Pubkey,
) -> Instruction {
    anchor_ix(
        batcher::id(),
        batcher::accounts::Settle {
            payer: fixture.payer,
            batcher: fixture.batcher,
            batch: keys.batch,
            batch_authority: keys.batch_authority,
            join_confidential_mint: fixture.join_mint().mint,
            batch_join_token_account: keys.join_token_account,
            join_underlying_mint: fixture.join_mint().underlying_mint,
            join_mint_vault_underlying: fixture.join_mint().vault_underlying,
            join_mint_vault_authority: fixture.join_mint().vault_authority,
            batch_join_underlying: keys.join_underlying,
            batch_burned_amount_value: keys.burned_amount_value,
            redemption_record,
            host_config: fixture.host_config,
            kms_context: fixture.kms_context,
            vault: fixture.vault,
            vault_authority: fixture.vault_authority,
            vault_token_account: fixture.vault_token_account,
            batch_payout_underlying: keys.payout_underlying,
            payout_confidential_mint: fixture.payout_mint().mint,
            payout_underlying_mint: fixture.payout_mint().underlying_mint,
            batch_payout_token_account: keys.payout_token_account,
            payout_mint_vault_underlying: fixture.payout_mint().vault_underlying,
            payout_mint_vault_authority: fixture.payout_mint().vault_authority,
            payout_compute_signer: fixture.payout_mint().compute_signer,
            payout_total_supply_authority: fixture.payout_mint().total_supply_authority,
            batch_payout_balance_value: keys.payout_balance_value,
            payout_total_supply_value: fixture.payout_mint().total_supply_value,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            confidential_token_event_authority: event_authority(token::id()),
            confidential_token_program: token::id(),
            demo_vault_program: vault::id(),
            token_program: spl_token::id(),
            system_program: system_program::ID,
        },
        batcher::instruction::Settle {
            cleartext_total,
            signatures,
            extra_data,
            proof,
            authority_funding_lamports: AUTHORITY_FUNDING,
        },
    )
}

fn claim_ix(fixture: &BatcherFixture, keys: &BatchKeys, user: &UserKeys) -> Instruction {
    let user_payout = fixture.user_payout(user);
    anchor_ix(
        batcher::id(),
        batcher::accounts::Claim {
            payer: fixture.payer,
            user: user.user,
            batcher: fixture.batcher,
            batch: keys.batch,
            batch_authority: keys.batch_authority,
            join_record: keys.join_record(user.user),
            pending_join_value: keys.pending_join_value(user.user),
            claim_amount_value: keys.claim_amount_value(user.user),
            payout_confidential_mint: fixture.payout_mint().mint,
            payout_compute_signer: fixture.payout_mint().compute_signer,
            batch_payout_token_account: keys.payout_token_account,
            user_payout_token_account: user_payout.token_account,
            batch_payout_balance_value: keys.payout_balance_value,
            user_payout_balance_value: user_payout.balance_value,
            batch_payout_transferred_value: keys.payout_transferred_value,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            confidential_token_event_authority: event_authority(token::id()),
            confidential_token_program: token::id(),
            system_program: system_program::ID,
        },
        batcher::instruction::Claim {},
    )
}

// ---------------------------------------------------------------------------
// Lifecycle drivers
// ---------------------------------------------------------------------------

/// Initializes the batcher and opens batch 0, returning its keys.
fn initialize_and_open_first_batch(
    context: &Ctx,
    fixture: &BatcherFixture,
    min_batch_age_slots: u64,
) -> BatchKeys {
    context.process_and_validate_instruction(
        &initialize_batcher_ix(fixture, min_batch_age_slots),
        &[Check::success()],
    );
    let keys = BatchKeys::new(fixture, 0);
    ensure_open_batch_accounts(context, &keys);
    context.process_and_validate_instruction(
        &open_batch_ix(fixture, &keys, None),
        &[Check::success()],
    );
    keys
}

fn ensure_open_batch_accounts(context: &Ctx, keys: &BatchKeys) {
    ensure_system_accounts(
        context,
        &[
            keys.batch,
            keys.batch_authority,
            keys.join_token_account,
            keys.join_balance_value,
            keys.payout_token_account,
            keys.payout_balance_value,
            keys.join_underlying,
            keys.payout_underlying,
        ],
    );
}

/// Seeds the batch's freshly created (encrypted zero) balance encrypted value accounts.
fn seed_open_batch_balances(context: &Ctx, keys: &BatchKeys, ledger: &mut CleartextLedger) {
    ledger.seed_amount(
        read_encrypted_value(context, keys.join_balance_value).current_handle,
        0,
    );
    ledger.seed_amount(
        read_encrypted_value(context, keys.payout_balance_value).current_handle,
        0,
    );
}

/// Runs one join and replays its FHE CPIs into the ledger.
fn run_join(
    context: &Ctx,
    fixture: &BatcherFixture,
    keys: &BatchKeys,
    user: &UserKeys,
    ledger: &mut CleartextLedger,
    amount_handle: [u8; 32],
    amount: u64,
) {
    ledger.seed_amount(amount_handle, amount);
    ensure_system_accounts(
        context,
        &[
            keys.join_record(user.user),
            fixture.user_join(user).transferred_value,
            keys.pending_join_value(user.user),
        ],
    );
    let attestation =
        amount_attestation_for(amount_handle, user.user, fixture.join_mint().compute_signer);
    let ix = join_ix(fixture, keys, user, attestation);
    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);
    // The join issues the transfer's eval plus the batcher's re-materialization.
    assert_eq!(ledger.evaluate_fhe_cpis(context, &result), 2);
}

/// Dispatches the batch and returns the born-public burned handle.
fn run_dispatch(
    context: &Ctx,
    fixture: &BatcherFixture,
    keys: &BatchKeys,
    ledger: &mut CleartextLedger,
) -> [u8; 32] {
    ensure_system_accounts(context, &[keys.burned_amount_value]);
    let ix = dispatch_ix(fixture, keys);
    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);
    assert_eq!(ledger.evaluate_fhe_cpis(context, &result), 1);
    read_batch(context, keys.batch).burned_total_handle
}

/// Settles the batch with a real KMS cert over `total`, replaying the wrap's
/// eval when the batch is non-zero. Returns settle's consumed compute units so
/// the fixed-key cost lifecycle can bound them; behavior-test callers ignore
/// the value (the module policy is that behavior tests never depend on cost).
fn run_settle(
    context: &Ctx,
    fixture: &BatcherFixture,
    keys: &BatchKeys,
    ledger: &mut CleartextLedger,
    burned_handle: [u8; 32],
    total: u64,
) -> u64 {
    let (signatures, extra_data) = kms_public_decrypt_cert(burned_handle, total);
    let proof = single_burn_public_decrypt_proof(keys.burned_amount_value, burned_handle);
    let redemption_record =
        token::burn_redemption_address(fixture.join_mint().mint, burned_handle).0;
    ensure_system_accounts(context, &[redemption_record]);
    let ix = settle_ix(
        fixture,
        keys,
        total,
        signatures,
        extra_data,
        proof,
        redemption_record,
    );
    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);
    if total > 0 {
        // Only the wrap leg drives an eval at settle.
        assert_eq!(ledger.evaluate_fhe_cpis(context, &result), 1);
    }
    result.compute_units_consumed
}

/// Claims for one user, replaying the MulDiv and transfer evals.
fn run_claim(
    context: &Ctx,
    fixture: &BatcherFixture,
    keys: &BatchKeys,
    user: &UserKeys,
    ledger: &mut CleartextLedger,
) {
    ensure_system_accounts(
        context,
        &[
            keys.claim_amount_value(user.user),
            keys.payout_transferred_value,
        ],
    );
    let ix = claim_ix(fixture, keys, user);
    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);
    // The claim issues the batcher's MulDiv eval plus the transfer's eval.
    assert_eq!(ledger.evaluate_fhe_cpis(context, &result), 2);
}

// ---------------------------------------------------------------------------
// Deposit lifecycle tests
// ---------------------------------------------------------------------------

/// Full multi-user lifecycle against a fresh (1:1) vault: two users join with
/// encrypted amounts, only the total (800) is revealed by the burn+redeem, the
/// vault mints 800 shares, and each user claims encrypted shares equal to
/// their exact proportional part. Every batcher CPI is a real `invoke_signed`
/// by the per-batch authority PDA.
#[test]
fn mollusk_lifecycle_two_users_deposit_dispatch_settle_claim() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = mollusk().with_context(fixture.accounts(0, 0));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);

    // Batch opens with an encrypted zero balance on both sides.
    let open_result = read_batch(&context, keys.batch);
    assert_eq!(open_result.status, batcher::BatchStatus::Pending);
    seed_open_batch_balances(&context, &keys, &mut ledger);

    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.bob,
        &mut ledger,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        500,
    );

    // Encrypted accounting after the joins: user balances debited, the batch
    // account holds the (still encrypted) sum, each joined encrypted value account carries
    // that user's amount and only that user's amount.
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.underlying.balance_value),
        700
    );
    assert_eq!(
        ledger.u64_at(&context, fixture.bob.underlying.balance_value),
        1_500
    );
    assert_eq!(ledger.u64_at(&context, keys.join_balance_value), 800);
    assert_eq!(
        ledger.u64_at(&context, keys.pending_join_value(fixture.alice.user)),
        300
    );
    assert_eq!(
        ledger.u64_at(&context, keys.pending_join_value(fixture.bob.user)),
        500
    );
    assert_eq!(read_batch(&context, keys.batch).join_count, 2);

    // Dispatch burns the batch's whole balance; the burned encrypted value account carries the
    // batch total, born publicly decryptable.
    let burned_handle = run_dispatch(&context, &fixture, &keys, &mut ledger);
    assert_eq!(ledger.u64_at(&context, keys.join_balance_value), 0);
    assert_eq!(ledger.u64_at(&context, keys.burned_amount_value), 800);
    assert_eq!(
        read_batch(&context, keys.batch).status,
        batcher::BatchStatus::Dispatched
    );

    // Settle: the KMS certifies 800; the vault (empty, 1:1) mints 800 shares;
    // the informational rate lands at exactly RATE_SCALE.
    run_settle(&context, &fixture, &keys, &mut ledger, burned_handle, 800);
    let settled = read_batch(&context, keys.batch);
    assert_eq!(settled.status, batcher::BatchStatus::Settled);
    assert_eq!(settled.total_joined, 800);
    assert_eq!(settled.payout_received, 800);
    assert_eq!(settled.payout_rate, batcher::RATE_SCALE);
    assert_eq!(read_spl_amount(&context, fixture.vault_token_account), 800);
    // The received shares were wrapped: the plain payout account drained into
    // the shares mint's escrow, and the batch's confidential payout balance is
    // the aggregate.
    assert_eq!(read_spl_amount(&context, keys.payout_underlying), 0);
    assert_eq!(
        read_spl_amount(&context, fixture.shares_cmint.vault_underlying),
        800
    );
    assert_eq!(ledger.u64_at(&context, keys.payout_balance_value), 800);

    // Claims: each user receives their exact proportional encrypted shares.
    run_claim(&context, &fixture, &keys, &fixture.alice, &mut ledger);
    run_claim(&context, &fixture, &keys, &fixture.bob, &mut ledger);
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.shares.balance_value),
        300
    );
    assert_eq!(
        ledger.u64_at(&context, fixture.bob.shares.balance_value),
        500
    );
    assert_eq!(ledger.u64_at(&context, keys.payout_balance_value), 0);
    assert!(read_join_record(&context, keys.join_record(fixture.alice.user)).claimed);
    assert!(read_join_record(&context, keys.join_record(fixture.bob.user)).claimed);
}

/// Lifecycle against a vault with existing yield (2_000 assets / 1_000
/// shares): the floor-rounded exact-proportional claims never exceed the
/// wrapped shares.
#[test]
fn mollusk_lifecycle_with_yield_rate_rounds_down() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    // Share price ~2: 2_000 assets backing 1_000 shares.
    let context = mollusk().with_context(fixture.accounts(2_000, 1_000));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    seed_open_batch_balances(&context, &keys, &mut ledger);

    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.bob,
        &mut ledger,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        500,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys, &mut ledger);
    run_settle(&context, &fixture, &keys, &mut ledger, burned_handle, 800);

    // shares = 800 * (1_000 + 1) / (2_000 + 1) = 400 (floor);
    // informational rate = 400 * RATE_SCALE / 800 = RATE_SCALE / 2.
    let settled = read_batch(&context, keys.batch);
    assert_eq!(settled.payout_received, 400);
    assert_eq!(settled.payout_rate, batcher::RATE_SCALE / 2);

    run_claim(&context, &fixture, &keys, &fixture.alice, &mut ledger);
    run_claim(&context, &fixture, &keys, &fixture.bob, &mut ledger);
    // 300 * 400 / 800 = 150 and 500 -> 250; the claims sum exactly to the
    // wrapped 400 here, and can never exceed it by the floor rounding.
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.shares.balance_value),
        150
    );
    assert_eq!(
        ledger.u64_at(&context, fixture.bob.shares.balance_value),
        250
    );
    assert_eq!(ledger.u64_at(&context, keys.payout_balance_value), 0);
}

/// A single-participant batch settles correctly but reveals that participant's
/// amount: the certified public total IS their deposit. This documents the
/// known privacy caveat (CONFIDENTIAL_VAULTS.md) — privacy grows with genuine
/// participants per batch, and the design deliberately does not gate on
/// participant count.
#[test]
fn mollusk_single_user_batch_reveals_that_users_amount() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = mollusk().with_context(fixture.accounts(0, 0));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    seed_open_batch_balances(&context, &keys, &mut ledger);

    let alice_amount = 777;
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        alice_amount,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys, &mut ledger);
    run_settle(
        &context,
        &fixture,
        &keys,
        &mut ledger,
        burned_handle,
        alice_amount,
    );

    // The amount-reveal caveat: with one participant, the public batch total
    // equals their private deposit exactly.
    let settled = read_batch(&context, keys.batch);
    assert_eq!(settled.total_joined, alice_amount);
    assert_eq!(
        settled.total_joined,
        ledger.u64_at(&context, keys.pending_join_value(fixture.alice.user))
    );

    run_claim(&context, &fixture, &keys, &fixture.alice, &mut ledger);
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.shares.balance_value),
        alice_amount
    );
}

/// Repeated joins accumulate in the joined encrypted value account (the operand-aliases-output
/// supersede), quit refunds the exact accumulated amount all-or-nothing and
/// resets the encrypted value account to zero, and a re-join after quit accumulates from zero.
#[test]
fn mollusk_repeat_join_accumulates_and_quit_refunds_exactly() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = mollusk().with_context(fixture.accounts(0, 0));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    seed_open_batch_balances(&context, &keys, &mut ledger);

    // Two joins accumulate: the second join's eval reads the joined encrypted value account
    // as an operand AND supersedes it as the output (the #3238 aliasing class
    // for the batcher's own eval — the standard same-slot supersede).
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        100,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        250,
    );
    let pending = keys.pending_join_value(fixture.alice.user);
    assert_eq!(ledger.u64_at(&context, pending), 350);
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.underlying.balance_value),
        650
    );

    // Quit refunds exactly 350 (all-or-nothing) and resets the encrypted value account to zero.
    let quit = quit_ix(&fixture, &keys, &fixture.alice);
    let result = context.process_and_validate_instruction(&quit, &[Check::success()]);
    assert_eq!(ledger.evaluate_fhe_cpis(&context, &result), 2);
    assert_eq!(ledger.u64_at(&context, pending), 0);
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.underlying.balance_value),
        1_000
    );
    assert_eq!(ledger.u64_at(&context, keys.join_balance_value), 0);

    // Re-join after quit accumulates from zero, not from the stale amount.
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(43, BALANCE_FHE_TYPE),
        40,
    );
    assert_eq!(ledger.u64_at(&context, pending), 40);
    assert_eq!(ledger.u64_at(&context, keys.join_balance_value), 40);
}

/// A batch with no joins burns zero, and settle with the KMS-certified zero
/// cancels the batch: no vault deposit, no wrap, no rate, and the next batch
/// can open. The zero-total division-by-zero path is unreachable by
/// construction.
#[test]
fn mollusk_zero_total_batch_cancels_at_settle() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = mollusk().with_context(fixture.accounts(0, 0));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    ledger.seed_amount(
        read_encrypted_value(&context, keys.join_balance_value).current_handle,
        0,
    );

    let burned_handle = run_dispatch(&context, &fixture, &keys, &mut ledger);
    assert_eq!(ledger.u64_at(&context, keys.burned_amount_value), 0);
    run_settle(&context, &fixture, &keys, &mut ledger, burned_handle, 0);

    let batch = read_batch(&context, keys.batch);
    assert_eq!(batch.status, batcher::BatchStatus::Canceled);
    assert_eq!(batch.payout_rate, 0);
    assert_eq!(read_spl_amount(&context, fixture.vault_token_account), 0);

    // The next batch opens against the canceled one.
    let next = BatchKeys::new(&fixture, 1);
    ensure_open_batch_accounts(&context, &next);
    context.process_and_validate_instruction(
        &open_batch_ix(&fixture, &next, Some(keys.batch)),
        &[Check::success()],
    );
    assert_eq!(read_batch(&context, next.batch).index, 1);
}

// ---------------------------------------------------------------------------
// Redeem lifecycle tests
// ---------------------------------------------------------------------------

/// Redeem-shaped account set: users hold confidential shares (backed by plain
/// vault shares in the shares mint's escrow) and claim underlying back.
fn redeem_accounts(
    fixture: &BatcherFixture,
    vault_total_assets: u64,
    vault_total_shares: u64,
    shares_escrow: u64,
) -> HashMap<Pubkey, Account> {
    fixture.accounts_with_escrows(vault_total_assets, vault_total_shares, 0, shares_escrow)
}

/// Full multi-user redeem lifecycle against a 1:1 vault: two users join with
/// encrypted SHARE amounts, only the share total (700) is revealed by the
/// burn+redeem, the vault pays 700 underlying for it, and each user claims
/// encrypted underlying equal to their exact proportional part. The mirror of
/// the deposit lifecycle with join and payout mints swapped.
#[test]
fn mollusk_redeem_lifecycle_two_users_join_dispatch_settle_claim() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Redeem);
    // 1_000 assets backing 1_000 outstanding shares, all of them wrapped as
    // the users' confidential share positions.
    let context = mollusk().with_context(redeem_accounts(&fixture, 1_000, 1_000, 1_000));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (0, 600), (0, 400), (0, 1_000));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    assert_eq!(
        read_batch(&context, keys.batch).status,
        batcher::BatchStatus::Pending
    );
    seed_open_batch_balances(&context, &keys, &mut ledger);

    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.bob,
        &mut ledger,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        400,
    );

    // Encrypted accounting after the joins: confidential SHARE balances
    // debited, the batch account holds the encrypted share sum.
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.shares.balance_value),
        300
    );
    assert_eq!(ledger.u64_at(&context, fixture.bob.shares.balance_value), 0);
    assert_eq!(ledger.u64_at(&context, keys.join_balance_value), 700);
    assert_eq!(read_batch(&context, keys.batch).join_count, 2);

    // Dispatch burns the batch's whole confidential-share balance.
    let burned_handle = run_dispatch(&context, &fixture, &keys, &mut ledger);
    assert_eq!(ledger.u64_at(&context, keys.join_balance_value), 0);
    assert_eq!(ledger.u64_at(&context, keys.burned_amount_value), 700);

    // Settle: the KMS certifies 700 shares; the vault (1:1) pays 700
    // underlying; the underlying is wrapped into the batch's confidential
    // payout account.
    run_settle(&context, &fixture, &keys, &mut ledger, burned_handle, 700);
    let settled = read_batch(&context, keys.batch);
    assert_eq!(settled.status, batcher::BatchStatus::Settled);
    assert_eq!(settled.total_joined, 700);
    assert_eq!(settled.payout_received, 700);
    assert_eq!(settled.payout_rate, batcher::RATE_SCALE);
    // 700 shares burned from the escrowed plain shares; 700 underlying left
    // the vault and got wrapped into the underlying mint's escrow.
    assert_eq!(read_spl_amount(&context, fixture.vault_token_account), 300);
    assert_eq!(read_spl_amount(&context, keys.join_underlying), 0);
    assert_eq!(read_spl_amount(&context, keys.payout_underlying), 0);
    assert_eq!(
        read_spl_amount(&context, fixture.underlying_cmint.vault_underlying),
        700
    );
    assert_eq!(ledger.u64_at(&context, keys.payout_balance_value), 700);

    // Claims: each user receives their exact proportional encrypted underlying.
    run_claim(&context, &fixture, &keys, &fixture.alice, &mut ledger);
    run_claim(&context, &fixture, &keys, &fixture.bob, &mut ledger);
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.underlying.balance_value),
        300
    );
    assert_eq!(
        ledger.u64_at(&context, fixture.bob.underlying.balance_value),
        400
    );
    assert_eq!(ledger.u64_at(&context, keys.payout_balance_value), 0);
    assert!(read_join_record(&context, keys.join_record(fixture.alice.user)).claimed);
    assert!(read_join_record(&context, keys.join_record(fixture.bob.user)).claimed);
}

/// Redeem lifecycle with yield (2_000 assets / 1_000 shares): 700 shares pay
/// 700 * 2_001 / 1_001 = 1_399 underlying (floor), and the exact-proportional
/// floor claims (599 + 799) never exceed the wrapped 1_399 — one dust unit
/// stays in the batch account instead of over-distributing.
#[test]
fn mollusk_redeem_lifecycle_with_yield_rounds_down() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Redeem);
    let context = mollusk().with_context(redeem_accounts(&fixture, 2_000, 1_000, 1_000));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (0, 600), (0, 400), (0, 1_000));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    seed_open_batch_balances(&context, &keys, &mut ledger);

    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.bob,
        &mut ledger,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        400,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys, &mut ledger);
    run_settle(&context, &fixture, &keys, &mut ledger, burned_handle, 700);

    // assets = 700 * (2_000 + 1) / (1_000 + 1) = 1_399 (floor).
    let settled = read_batch(&context, keys.batch);
    assert_eq!(settled.payout_received, 1_399);

    run_claim(&context, &fixture, &keys, &fixture.alice, &mut ledger);
    run_claim(&context, &fixture, &keys, &fixture.bob, &mut ledger);
    // Exact proportional floors: 300 * 1_399 / 700 = 599, 400 * 1_399 / 700
    // = 799. Sum 1_398 <= 1_399; the one dust unit stays with the batch.
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.underlying.balance_value),
        599
    );
    assert_eq!(
        ledger.u64_at(&context, fixture.bob.underlying.balance_value),
        799
    );
    assert_eq!(ledger.u64_at(&context, keys.payout_balance_value), 1);
}

/// The redeem twin of the deposit repeat-join/quit/re-join test: repeated
/// SHARE joins accumulate in the joined encrypted value account (the operand-aliases-output
/// same-slot supersede — the aliasing class this test exists to pin), quit
/// refunds the exact accumulated shares all-or-nothing and resets the encrypted value account
/// to zero, and a re-join after quit accumulates from zero.
#[test]
fn mollusk_redeem_repeat_join_accumulates_and_quit_refunds_exactly() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Redeem);
    let context = mollusk().with_context(redeem_accounts(&fixture, 1_000, 1_000, 1_000));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (0, 600), (0, 400), (0, 1_000));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    seed_open_batch_balances(&context, &keys, &mut ledger);

    // Two joins accumulate: the second join's eval reads the joined encrypted value account
    // as an operand AND supersedes it as the output.
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        100,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        250,
    );
    let pending = keys.pending_join_value(fixture.alice.user);
    assert_eq!(ledger.u64_at(&context, pending), 350);
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.shares.balance_value),
        250
    );

    // Quit refunds exactly 350 shares (all-or-nothing) and resets the encrypted value account.
    let quit = quit_ix(&fixture, &keys, &fixture.alice);
    let result = context.process_and_validate_instruction(&quit, &[Check::success()]);
    assert_eq!(ledger.evaluate_fhe_cpis(&context, &result), 2);
    assert_eq!(ledger.u64_at(&context, pending), 0);
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.shares.balance_value),
        600
    );
    assert_eq!(ledger.u64_at(&context, keys.join_balance_value), 0);

    // Re-join after quit accumulates from zero, not from the stale amount.
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(43, BALANCE_FHE_TYPE),
        40,
    );
    assert_eq!(ledger.u64_at(&context, pending), 40);
    assert_eq!(ledger.u64_at(&context, keys.join_balance_value), 40);
}

/// A user who quits before dispatch can still run the (permissionless) claim
/// after the batch settles on the other participants: their reset encrypted value account
/// makes the MulDiv produce an encrypted zero, the all-or-zero transfer moves
/// nothing, and the record is marked claimed. Deposit direction only: quit,
/// claim, and the encrypted value account reset are direction-free shared code (settle's
/// vault CPI is the sole direction branch), so one direction pins the class.
#[test]
fn mollusk_claim_after_quit_pays_zero() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = mollusk().with_context(fixture.accounts(0, 0));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    seed_open_batch_balances(&context, &keys, &mut ledger);

    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.bob,
        &mut ledger,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        500,
    );

    // Alice quits; the batch dispatches and settles on bob's 500 alone.
    let quit = quit_ix(&fixture, &keys, &fixture.alice);
    let result = context.process_and_validate_instruction(&quit, &[Check::success()]);
    assert_eq!(ledger.evaluate_fhe_cpis(&context, &result), 2);
    let burned_handle = run_dispatch(&context, &fixture, &keys, &mut ledger);
    assert_eq!(ledger.u64_at(&context, keys.burned_amount_value), 500);
    run_settle(&context, &fixture, &keys, &mut ledger, burned_handle, 500);

    // Alice's claim computes 0 * 500 / 500 = encrypted zero and transfers
    // nothing; her record still flips to claimed (the record survives quit).
    run_claim(&context, &fixture, &keys, &fixture.alice, &mut ledger);
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.shares.balance_value),
        0
    );
    assert_eq!(
        ledger.u64_at(&context, keys.claim_amount_value(fixture.alice.user)),
        0
    );
    assert!(read_join_record(&context, keys.join_record(fixture.alice.user)).claimed);

    // Bob's claim still pays the full settled batch.
    run_claim(&context, &fixture, &keys, &fixture.bob, &mut ledger);
    assert_eq!(
        ledger.u64_at(&context, fixture.bob.shares.balance_value),
        500
    );
    assert_eq!(ledger.u64_at(&context, keys.payout_balance_value), 0);
}

/// A redeem batch with no joins cancels at settle exactly like a deposit
/// batch: the certified zero cancels trustlessly and the vault is untouched.
#[test]
fn mollusk_redeem_zero_total_batch_cancels_at_settle() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Redeem);
    let context = mollusk().with_context(redeem_accounts(&fixture, 1_000, 1_000, 1_000));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (0, 600), (0, 400), (0, 1_000));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    ledger.seed_amount(
        read_encrypted_value(&context, keys.join_balance_value).current_handle,
        0,
    );

    let burned_handle = run_dispatch(&context, &fixture, &keys, &mut ledger);
    assert_eq!(ledger.u64_at(&context, keys.burned_amount_value), 0);
    run_settle(&context, &fixture, &keys, &mut ledger, burned_handle, 0);

    let batch = read_batch(&context, keys.batch);
    assert_eq!(batch.status, batcher::BatchStatus::Canceled);
    assert_eq!(batch.payout_rate, 0);
    assert_eq!(
        read_spl_amount(&context, fixture.vault_token_account),
        1_000
    );
}

/// A dust redeem batch has NO analog of the deposit direction's stuck state:
/// the vault's share price never drops below 1:1 (floor rounding favors the
/// vault, harvest only raises the price), so withdrawing any non-zero share
/// total always returns at least that many underlying units —
/// `demo_vault::withdraw`'s `ZeroAssets` is unreachable from a batch. One
/// share redeemed at an extreme (donation-pumped) price settles fine.
#[test]
fn mollusk_redeem_one_share_dust_settles_at_extreme_price() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Redeem);
    // Share price ~20_000 underlying per share: 2_000_000 assets backing 100
    // shares (the price shape that bricks sub-price deposit batches).
    let context = mollusk().with_context(redeem_accounts(&fixture, 2_000_000, 100, 100));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (0, 60), (0, 40), (0, 100));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    seed_open_batch_balances(&context, &keys, &mut ledger);

    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        1,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys, &mut ledger);
    run_settle(&context, &fixture, &keys, &mut ledger, burned_handle, 1);

    // assets = 1 * (2_000_000 + 1) / (100 + 1) = 19_801 (floor) — always
    // >= 1 per share at any reachable price, so no ZeroAssets, no stuck batch.
    let settled = read_batch(&context, keys.batch);
    assert_eq!(settled.status, batcher::BatchStatus::Settled);
    assert_eq!(settled.total_joined, 1);
    assert_eq!(settled.payout_received, 19_801);

    run_claim(&context, &fixture, &keys, &fixture.alice, &mut ledger);
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.underlying.balance_value),
        19_801
    );
}

/// SPL destinations cannot refuse incoming transfers, so an attacker can push
/// plain underlying into the PDA-owned `batch_payout_underlying` account of a
/// redeem batch before settlement. Settle must price and wrap only the
/// vault-paid delta across its withdraw leg — the redeem mirror of the
/// deposit direction's preload invariant. Preloaded tokens stay in the
/// account, unwrapped and unpriced (inert).
#[test]
fn mollusk_redeem_preloaded_underlying_stays_inert() {
    const PRELOAD: u64 = 15_000_000_000_000;
    let fixture = BatcherFixture::new(batcher::BatchDirection::Redeem);
    let attacker = Pubkey::new_unique();
    let attacker_underlying = Pubkey::new_unique();

    let mut accounts = redeem_accounts(&fixture, 1_000, 1_000, 1_000);
    accounts.insert(attacker, system_account(1_000_000_000));
    accounts.insert(
        attacker_underlying,
        spl_token_account(fixture.underlying_mint, attacker, PRELOAD),
    );
    let context = mollusk().with_context(accounts);
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (0, 600), (0, 400), (0, 1_000));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    seed_open_batch_balances(&context, &keys, &mut ledger);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.bob,
        &mut ledger,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        400,
    );

    // The attacker pushes plain underlying into the batch's payout account.
    let preload_transfer = spl_token::instruction::transfer(
        &spl_token::id(),
        &attacker_underlying,
        &keys.payout_underlying,
        &attacker,
        &[],
        PRELOAD,
    )
    .unwrap();
    context.process_and_validate_instruction(&preload_transfer, &[Check::success()]);
    assert_eq!(read_spl_amount(&context, keys.payout_underlying), PRELOAD);

    let burned_handle = run_dispatch(&context, &fixture, &keys, &mut ledger);
    run_settle(&context, &fixture, &keys, &mut ledger, burned_handle, 700);

    // Settle succeeded and the batch accounting reflects only the vault-paid
    // delta: 700 shares in, 700 underlying out at the 1:1 price. The preload
    // sits in the account, unwrapped, inert.
    let settled = read_batch(&context, keys.batch);
    assert_eq!(settled.status, batcher::BatchStatus::Settled);
    assert_eq!(settled.total_joined, 700);
    assert_eq!(settled.payout_received, 700);
    assert_eq!(read_spl_amount(&context, keys.payout_underlying), PRELOAD);
    assert_eq!(ledger.u64_at(&context, keys.payout_balance_value), 700);

    run_claim(&context, &fixture, &keys, &fixture.alice, &mut ledger);
    run_claim(&context, &fixture, &keys, &fixture.bob, &mut ledger);
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.underlying.balance_value),
        300
    );
    assert_eq!(
        ledger.u64_at(&context, fixture.bob.underlying.balance_value),
        400
    );
}

/// One deposit batcher and one redeem batcher run a FULL interleaved
/// lifecycle concurrently over the same vault, mints, and users — the
/// two-instance pattern. Cross-direction state confusion (a redeem batch
/// reading deposit-batch encrypted value accounts, the shared escrows or the vault mixing
/// legs) would surface here, not at open: both directions join, dispatch,
/// settle, and claim against the shared world, and every balance is checked.
#[test]
fn mollusk_deposit_and_redeem_batchers_run_concurrently() {
    let deposit = BatcherFixture::new(batcher::BatchDirection::Deposit);
    // Same physical world (mints, vault, users, payer); only the batcher
    // config account differs.
    let redeem = deposit.redeem_twin(Pubkey::new_unique());

    let mut accounts = deposit.accounts_with_escrows(1_000, 1_000, 1_000_000, 1_000);
    accounts.insert(redeem.batcher, system_account(0));
    let context = mollusk().with_context(accounts);
    let mut ledger = CleartextLedger::default();
    // Users hold both cUnderlying (to deposit) and cShares (to redeem).
    deposit.seed_ledger(&mut ledger, (1_000, 600), (2_000, 400), (1_000_000, 1_000));

    let deposit_keys = initialize_and_open_first_batch(&context, &deposit, 0);
    let redeem_keys = initialize_and_open_first_batch(&context, &redeem, 0);
    assert_ne!(deposit_keys.batch, redeem_keys.batch);
    seed_open_batch_balances(&context, &deposit_keys, &mut ledger);
    seed_open_batch_balances(&context, &redeem_keys, &mut ledger);

    // Interleaved joins: both batches are pending at once, and each user
    // participates in both directions.
    run_join(
        &context,
        &deposit,
        &deposit_keys,
        &deposit.alice,
        &mut ledger,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &redeem,
        &redeem_keys,
        &redeem.alice,
        &mut ledger,
        handle_for_chain(43, BALANCE_FHE_TYPE),
        200,
    );
    run_join(
        &context,
        &deposit,
        &deposit_keys,
        &deposit.bob,
        &mut ledger,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        500,
    );
    run_join(
        &context,
        &redeem,
        &redeem_keys,
        &redeem.bob,
        &mut ledger,
        handle_for_chain(44, BALANCE_FHE_TYPE),
        300,
    );

    // Each batch account holds exactly its own direction's encrypted sum.
    assert_eq!(
        ledger.u64_at(&context, deposit_keys.join_balance_value),
        800
    );
    assert_eq!(ledger.u64_at(&context, redeem_keys.join_balance_value), 500);

    // Interleaved dispatches: two independent burned handles on two mints.
    let deposit_burned = run_dispatch(&context, &deposit, &deposit_keys, &mut ledger);
    let redeem_burned = run_dispatch(&context, &redeem, &redeem_keys, &mut ledger);
    assert_ne!(deposit_burned, redeem_burned);

    // Settle the redeem batch first: 500 shares withdraw 500 underlying at
    // the 1:1 price, leaving the vault at (500, 500).
    run_settle(
        &context,
        &redeem,
        &redeem_keys,
        &mut ledger,
        redeem_burned,
        500,
    );
    let redeem_settled = read_batch(&context, redeem_keys.batch);
    assert_eq!(redeem_settled.status, batcher::BatchStatus::Settled);
    assert_eq!(redeem_settled.total_joined, 500);
    assert_eq!(redeem_settled.payout_received, 500);
    assert_eq!(read_spl_amount(&context, deposit.vault_token_account), 500);

    // Then the deposit batch: 800 underlying at the (still 1:1) price mints
    // 800 shares; the vault ends at (1_300, 1_300).
    run_settle(
        &context,
        &deposit,
        &deposit_keys,
        &mut ledger,
        deposit_burned,
        800,
    );
    let deposit_settled = read_batch(&context, deposit_keys.batch);
    assert_eq!(deposit_settled.status, batcher::BatchStatus::Settled);
    assert_eq!(deposit_settled.total_joined, 800);
    assert_eq!(deposit_settled.payout_received, 800);
    assert_eq!(
        read_spl_amount(&context, deposit.vault_token_account),
        1_300
    );
    // Shared escrows carry both directions without mixing: the shares escrow
    // lost the 500 redeemed and gained the 800 wrapped; the underlying escrow
    // lost the 800 redeemed and gained the 500 wrapped.
    assert_eq!(
        read_spl_amount(&context, deposit.shares_cmint.vault_underlying),
        1_000 - 500 + 800
    );
    assert_eq!(
        read_spl_amount(&context, deposit.underlying_cmint.vault_underlying),
        1_000_000 - 800 + 500
    );

    // Interleaved claims across both directions.
    run_claim(
        &context,
        &deposit,
        &deposit_keys,
        &deposit.alice,
        &mut ledger,
    );
    run_claim(&context, &redeem, &redeem_keys, &redeem.alice, &mut ledger);
    run_claim(&context, &redeem, &redeem_keys, &redeem.bob, &mut ledger);
    run_claim(&context, &deposit, &deposit_keys, &deposit.bob, &mut ledger);

    // Final per-user balances: cShares = start - redeem join + deposit claim;
    // cUnderlying = start - deposit join + redeem claim.
    assert_eq!(
        ledger.u64_at(&context, deposit.alice.shares.balance_value),
        600 - 200 + 300
    );
    assert_eq!(
        ledger.u64_at(&context, deposit.alice.underlying.balance_value),
        1_000 - 300 + 200
    );
    assert_eq!(
        ledger.u64_at(&context, deposit.bob.shares.balance_value),
        400 - 300 + 500
    );
    assert_eq!(
        ledger.u64_at(&context, deposit.bob.underlying.balance_value),
        2_000 - 500 + 300
    );
    // Both batch payout accounts fully drained.
    assert_eq!(
        ledger.u64_at(&context, deposit_keys.payout_balance_value),
        0
    );
    assert_eq!(ledger.u64_at(&context, redeem_keys.payout_balance_value), 0);
}

// ---------------------------------------------------------------------------
// Lifecycle-gate rejects
// ---------------------------------------------------------------------------

/// Dispatch before `min_batch_age_slots` is rejected — the aggregation window
/// is the only time gate in the flow.
#[test]
fn mollusk_dispatch_before_min_batch_age_rejects() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = mollusk().with_context(fixture.accounts(0, 0));
    // Batch opens at slot 100 and must age 1_000 slots.
    let keys = initialize_and_open_first_batch(&context, &fixture, 1_000);
    ensure_system_accounts(&context, &[keys.burned_amount_value]);
    context.process_and_validate_instruction(
        &dispatch_ix(&fixture, &keys),
        &[batcher_error(batcher::BatcherError::BatchTooYoung)],
    );
}

/// After dispatch, the batch is frozen for users: join and quit both reject,
/// and a second dispatch rejects. Claims reject until settle.
#[test]
fn mollusk_join_quit_and_claim_respect_batch_status() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = mollusk().with_context(fixture.accounts(0, 0));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    ledger.seed_amount(
        read_encrypted_value(&context, keys.join_balance_value).current_handle,
        0,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );

    // Claim before settle rejects.
    ensure_system_accounts(
        &context,
        &[
            keys.claim_amount_value(fixture.alice.user),
            keys.payout_transferred_value,
        ],
    );
    context.process_and_validate_instruction(
        &claim_ix(&fixture, &keys, &fixture.alice),
        &[batcher_error(batcher::BatcherError::BatchNotSettled)],
    );

    let _burned = run_dispatch(&context, &fixture, &keys, &mut ledger);

    // Join after dispatch rejects.
    let attestation = amount_attestation_for(
        handle_for_chain(42, BALANCE_FHE_TYPE),
        fixture.alice.user,
        fixture.join_mint().compute_signer,
    );
    context.process_and_validate_instruction(
        &join_ix(&fixture, &keys, &fixture.alice, attestation),
        &[batcher_error(batcher::BatcherError::BatchNotPending)],
    );
    // Quit after dispatch rejects — the exit is the claim, pro rata. There is
    // no exit between dispatch and settle (fhevm-internal#1773).
    context.process_and_validate_instruction(
        &quit_ix(&fixture, &keys, &fixture.alice),
        &[batcher_error(batcher::BatcherError::BatchNotPending)],
    );
    // Second dispatch rejects.
    context.process_and_validate_instruction(
        &dispatch_ix(&fixture, &keys),
        &[batcher_error(batcher::BatcherError::BatchNotPending)],
    );
}

/// A settled batch pays each record once: the second claim rejects on the
/// record's claimed flag.
#[test]
fn mollusk_double_claim_rejects() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = mollusk().with_context(fixture.accounts(0, 0));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    seed_open_batch_balances(&context, &keys, &mut ledger);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys, &mut ledger);
    run_settle(&context, &fixture, &keys, &mut ledger, burned_handle, 300);
    run_claim(&context, &fixture, &keys, &fixture.alice, &mut ledger);

    context.process_and_validate_instruction(
        &claim_ix(&fixture, &keys, &fixture.alice),
        &[batcher_error(batcher::BatcherError::AlreadyClaimed)],
    );
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.shares.balance_value),
        300
    );
}

/// Batches never overlap while pending: a second open against a pending
/// previous batch rejects, and opening without the previous batch account
/// rejects.
#[test]
fn mollusk_open_batch_requires_previous_batch_not_pending() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = mollusk().with_context(fixture.accounts(0, 0));
    let keys = initialize_and_open_first_batch(&context, &fixture, 0);

    let next = BatchKeys::new(&fixture, 1);
    ensure_open_batch_accounts(&context, &next);
    context.process_and_validate_instruction(
        &open_batch_ix(&fixture, &next, Some(keys.batch)),
        &[batcher_error(
            batcher::BatcherError::PreviousBatchStillPending,
        )],
    );
    context.process_and_validate_instruction(
        &open_batch_ix(&fixture, &next, None),
        &[batcher_error(batcher::BatcherError::PreviousBatchMismatch)],
    );
}

/// Initializing a batcher with the direction's mint wiring swapped rejects:
/// a redeem batcher whose join mint wraps the vault underlying (instead of
/// its shares) is refused at setup.
#[test]
fn mollusk_initialize_batcher_rejects_swapped_direction_wiring() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Redeem);
    let context = mollusk().with_context(redeem_accounts(&fixture, 0, 0, 0));
    let swapped = anchor_ix(
        batcher::id(),
        batcher::accounts::InitializeBatcher {
            payer: fixture.payer,
            batcher: fixture.batcher,
            // Deposit-shaped wiring under a Redeem direction.
            join_confidential_mint: fixture.underlying_cmint.mint,
            payout_confidential_mint: fixture.shares_cmint.mint,
            vault: fixture.vault,
            system_program: system_program::ID,
        },
        batcher::instruction::InitializeBatcher {
            min_batch_age_slots: 0,
            direction: batcher::BatchDirection::Redeem,
        },
    );
    context.process_and_validate_instruction(
        &swapped,
        &[batcher_error(batcher::BatcherError::JoinMintVaultMismatch)],
    );
}

// ---------------------------------------------------------------------------
// Cost snapshots
// ---------------------------------------------------------------------------

fn assert_batcher_cost(profile: &str, ix: &Instruction, result: &InstructionResult) {
    cost_snapshot::assert_cost_snapshot("batcher_mollusk", profile, ix, result);
}

// Deterministic upper bounds on settle's compute cost, asserted only in the
// fixed-key cost lifecycle (see `snapshot_lifecycle`). Settle gets no exact
// snapshot because its redemption-marker PDA is seeded by the runtime-derived
// burned handle, so its bump-search cost is not key-stable; a bound still
// catches a real regression. Baselines measured on the CI-pinned toolchain —
// deposit settle 302_224..=306_724 CU (vault `deposit` CPI + wrap growth; the
// handle-derived bump search makes this vary run to run — the same reason no
// exact snapshot is pinned), redeem settle 369_715 CU (vault `withdraw` CPI)
// — bounds take the highest observed value plus ~15% headroom rounded up to a
// clean number, which absorbs the ±10% bump-search variance plus minor
// legitimate drift while still catching real regressions.
const SETTLE_DEPOSIT_MAX_COMPUTE_UNITS: u64 = 360_000;
const SETTLE_REDEEM_MAX_COMPUTE_UNITS: u64 = 430_000;

/// One fixed-key run through open/join/dispatch/claim, snapshotting each
/// instruction's cost profile under `prefix`. Fixed fixture keys keep the PDA
/// bump searches — part of the measured compute — stable across runs. Settle
/// gets no exact snapshot: its redemption-marker PDA seed is runtime-derived
/// (the burned handle), so its bump search is not key-stable; it is instead
/// covered here by a deterministic upper-bound compute assertion (see
/// `SETTLE_DEPOSIT_MAX_COMPUTE_UNITS` / `SETTLE_REDEEM_MAX_COMPUTE_UNITS`).
fn snapshot_lifecycle(
    fixture: &BatcherFixture,
    context: &Ctx,
    ledger: &mut CleartextLedger,
    prefix: &str,
) {
    context
        .process_and_validate_instruction(&initialize_batcher_ix(fixture, 0), &[Check::success()]);
    let keys = BatchKeys::new(fixture, 0);
    ensure_open_batch_accounts(context, &keys);
    let open = open_batch_ix(fixture, &keys, None);
    let open_result = context.process_and_validate_instruction(&open, &[Check::success()]);
    assert_batcher_cost(&format!("{prefix}open_batch"), &open, &open_result);
    seed_open_batch_balances(context, &keys, ledger);

    let amount_handle = handle_for_chain(0x71, BALANCE_FHE_TYPE);
    ledger.seed_amount(amount_handle, 300);
    ensure_system_accounts(
        context,
        &[
            keys.join_record(fixture.alice.user),
            fixture.user_join(&fixture.alice).transferred_value,
            keys.pending_join_value(fixture.alice.user),
        ],
    );
    let join = join_ix(
        fixture,
        &keys,
        &fixture.alice,
        amount_attestation_for(
            amount_handle,
            fixture.alice.user,
            fixture.join_mint().compute_signer,
        ),
    );
    let join_result = context.process_and_validate_instruction(&join, &[Check::success()]);
    ledger.evaluate_fhe_cpis(context, &join_result);
    assert_batcher_cost(&format!("{prefix}join"), &join, &join_result);

    ensure_system_accounts(context, &[keys.burned_amount_value]);
    let dispatch = dispatch_ix(fixture, &keys);
    let dispatch_result = context.process_and_validate_instruction(&dispatch, &[Check::success()]);
    ledger.evaluate_fhe_cpis(context, &dispatch_result);
    assert_batcher_cost(&format!("{prefix}dispatch"), &dispatch, &dispatch_result);

    let burned_handle = read_batch(context, keys.batch).burned_total_handle;
    let settle_compute_units = run_settle(context, fixture, &keys, ledger, burned_handle, 300);
    let settle_bound = match fixture.direction {
        batcher::BatchDirection::Deposit => SETTLE_DEPOSIT_MAX_COMPUTE_UNITS,
        batcher::BatchDirection::Redeem => SETTLE_REDEEM_MAX_COMPUTE_UNITS,
    };
    assert!(
        settle_compute_units < settle_bound,
        "settle ({:?}) consumed {settle_compute_units} CU, over the {settle_bound} CU upper \
         bound. If this cost increase is intentional, set the matching \
         SETTLE_{{DEPOSIT,REDEEM}}_MAX_COMPUTE_UNITS to the new measured value plus ~15% headroom \
         rounded up to a clean number.",
        fixture.direction,
    );

    ensure_system_accounts(
        context,
        &[
            keys.claim_amount_value(fixture.alice.user),
            keys.payout_transferred_value,
        ],
    );
    let claim = claim_ix(fixture, &keys, &fixture.alice);
    let claim_result = context.process_and_validate_instruction(&claim, &[Check::success()]);
    ledger.evaluate_fhe_cpis(context, &claim_result);
    assert_batcher_cost(&format!("{prefix}claim"), &claim, &claim_result);
}

#[test]
fn cost_snapshot_batcher_lifecycle() {
    let fixture = BatcherFixture::fixed(batcher::BatchDirection::Deposit, 0x61);
    let context = mollusk().with_context(fixture.accounts(0, 0));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (1_000, 0), (2_000, 0), (1_000_000, 0));
    snapshot_lifecycle(&fixture, &context, &mut ledger, "");
}

#[test]
fn cost_snapshot_batcher_redeem_lifecycle() {
    let fixture = BatcherFixture::fixed(batcher::BatchDirection::Redeem, 0x51);
    let context = mollusk().with_context(redeem_accounts(&fixture, 1_000, 1_000, 1_000));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (0, 600), (0, 400), (0, 1_000));
    snapshot_lifecycle(&fixture, &context, &mut ledger, "redeem_");
}

// ---------------------------------------------------------------------------
// Known limitation: dust-total DEPOSIT batches cannot settle (pinned behavior)
// ---------------------------------------------------------------------------

/// A deposit batch whose certified total floors to zero shares at the vault's
/// price cannot settle: `demo_vault::deposit` rejects `ZeroShares`, the settle
/// reverts atomically (nothing paid out, no marker written), and the batch
/// stays Dispatched forever — the demo vault's price only rises, so no retry
/// can ever succeed. An attacker holding ~all vault shares can manufacture
/// this near-free by `harvest`-donating to pump the price (the donation
/// accrues to their own shares). This test pins today's behavior so the
/// future cancel-and-refund settle branch (fhevm-internal#1773) has a failing
/// test to flip. The redeem direction has no analog — see
/// `mollusk_redeem_one_share_dust_settles_at_extreme_price`.
#[test]
fn mollusk_dust_total_settle_reverts_and_batch_stays_dispatched() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    // Share price ~20_000 underlying per share (e.g. after an adversarial
    // harvest donation): 2_000_000 assets backing 100 shares.
    let context = mollusk().with_context(fixture.accounts(2_000_000, 100));
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    ledger.seed_amount(
        read_encrypted_value(&context, keys.join_balance_value).current_handle,
        0,
    );

    // Alice's 100 is dust at this price: 100 * (100 + 1) / (2_000_000 + 1)
    // floors to zero shares.
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        100,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys, &mut ledger);

    let (signatures, extra_data) = kms_public_decrypt_cert(burned_handle, 100);
    let proof = single_burn_public_decrypt_proof(keys.burned_amount_value, burned_handle);
    let redemption_record =
        token::burn_redemption_address(fixture.join_mint().mint, burned_handle).0;
    ensure_system_accounts(&context, &[redemption_record]);
    let ix = settle_ix(
        &fixture,
        &keys,
        100,
        signatures,
        extra_data,
        proof,
        redemption_record,
    );
    context.process_and_validate_instruction(
        &ix,
        &[Check::err(ProgramError::Custom(
            anchor_lang::error::ERROR_CODE_OFFSET + vault::DemoVaultError::ZeroShares as u32,
        ))],
    );

    // Atomic revert: still Dispatched, no redemption marker paid out, no
    // underlying released, vault untouched.
    let batch = read_batch(&context, keys.batch);
    assert_eq!(batch.status, batcher::BatchStatus::Dispatched);
    assert_eq!(batch.total_joined, 0);
    assert_eq!(read_spl_amount(&context, keys.join_underlying), 0);
    assert_eq!(
        read_spl_amount(&context, fixture.vault_token_account),
        2_000_000
    );
}

// ---------------------------------------------------------------------------
// Settle transaction wire size: legacy vs v0 + address lookup table
// ---------------------------------------------------------------------------

/// Serializes the REAL settle instruction (the full account list and
/// realistically shaped cert/proof data) as (a) a legacy `Transaction` and
/// (b) a v0 `VersionedTransaction` whose non-payer accounts load through one
/// address lookup table, across cert thresholds and proof depths. Legacy
/// settle never fits one packet (the ~35-account meta list alone approaches
/// the limit); v0+ALT fits comfortably at every reachable batcher shape —
/// the batch's burned encrypted value account always holds exactly one leaf, so its proof
/// depth is 0 regardless of the KMS threshold. The deep-proof row is the
/// out-of-domain bound where even v0+ALT would need a split settle.
fn assert_settle_wire_sizes(fixture: &BatcherFixture) {
    let keys = BatchKeys::new(fixture, 0);
    let burned_handle = handle_for_chain(0x92, BALANCE_FHE_TYPE);
    let redemption_record =
        token::burn_redemption_address(fixture.join_mint().mint, burned_handle).0;

    let settle_with = |threshold: usize, proof_depth: usize| -> Instruction {
        settle_ix(
            fixture,
            &keys,
            800,
            vec![[0u8; 65]; threshold],
            vec![0x00],
            host::instructions::MmrInclusionProof {
                leaf_index: 0,
                siblings: vec![[0u8; 32]; proof_depth],
            },
            redemption_record,
        )
    };

    let legacy_size = |ix: &Instruction| -> usize {
        let message =
            solana_sdk::message::Message::new(std::slice::from_ref(ix), Some(&fixture.payer));
        bincode::serialize(&solana_sdk::transaction::Transaction::new_unsigned(message))
            .unwrap()
            .len()
    };
    let v0_with_lookup_table_size = |ix: &Instruction| -> usize {
        // One ALT carrying every instruction account except the fee payer.
        // `try_compile` keeps the payer and the invoked program id static and
        // loads the rest (CPI target programs included) through the table.
        let mut addresses: Vec<Pubkey> = Vec::new();
        for meta in &ix.accounts {
            if meta.pubkey != fixture.payer && !addresses.contains(&meta.pubkey) {
                addresses.push(meta.pubkey);
            }
        }
        let table = solana_sdk::message::AddressLookupTableAccount {
            key: Pubkey::new_from_array([0xAA; 32]),
            addresses,
        };
        let message = solana_sdk::message::v0::Message::try_compile(
            &fixture.payer,
            std::slice::from_ref(ix),
            &[table],
            solana_sdk::hash::Hash::default(),
        )
        .expect("settle compiles to a v0 message");
        let transaction = solana_sdk::transaction::VersionedTransaction {
            signatures: vec![solana_sdk::signature::Signature::default()],
            message: solana_sdk::message::VersionedMessage::V0(message),
        };
        bincode::serialize(&transaction).unwrap().len()
    };

    // (threshold, proof depth): the Mollusk fixture shape, the realistic
    // production cert (7-of-13 majority) at the batcher's real proof depth
    // (always 0 — one leaf per batch encrypted value account), and the out-of-domain deep
    // proof bound.
    let mut sizes = Vec::new();
    for (threshold, depth) in [(1usize, 0usize), (7, 0), (7, 20)] {
        let ix = settle_with(threshold, depth);
        let legacy = legacy_size(&ix);
        let v0 = v0_with_lookup_table_size(&ix);
        println!(
            "settle wire size ({:?}) t={threshold} depth={depth}: legacy={legacy}B v0+ALT={v0}B \
             (packet limit {})",
            fixture.direction,
            solana_packet::PACKET_DATA_SIZE
        );
        sizes.push((threshold, depth, legacy, v0));
    }

    for (threshold, depth, legacy, v0) in sizes {
        // Legacy settle never fits: a legacy transaction is impossible.
        assert!(
            legacy > solana_packet::PACKET_DATA_SIZE,
            "legacy settle t={threshold} depth={depth} unexpectedly fits: {legacy}B"
        );
        if depth == 0 {
            // Every reachable batcher settle (proof depth is always 0) fits
            // in one packet as v0 + one lookup table, up to the production
            // KMS threshold.
            assert!(
                v0 <= solana_packet::PACKET_DATA_SIZE,
                "v0+ALT settle t={threshold} depth={depth} overflows: {v0}B"
            );
        }
    }
}

#[test]
fn settle_transaction_size_needs_v0_lookup_table_and_fits() {
    assert_settle_wire_sizes(&BatcherFixture::fixed(
        batcher::BatchDirection::Deposit,
        0x91,
    ));
}

#[test]
fn redeem_settle_transaction_size_needs_v0_lookup_table_and_fits() {
    assert_settle_wire_sizes(&BatcherFixture::fixed(
        batcher::BatchDirection::Redeem,
        0x81,
    ));
}

// ---------------------------------------------------------------------------
// Preloaded shares must not poison the batch (settle prices the delta)
// ---------------------------------------------------------------------------

/// SPL destinations cannot refuse incoming transfers, so an attacker can push
/// vault shares into the PDA-owned `batch_payout_underlying` account of a
/// deposit batch before settlement. Settle must price and wrap only the
/// vault-minted delta across its deposit leg: with balance-based accounting,
/// this preload would have inflated the batch's payout accounting (and, under
/// the old rate math, overflowed the u64 rate and bricked the batch). The
/// attacker acquires the shares through a genuine demo-vault deposit and a
/// genuine SPL transfer — no seeded shortcuts.
#[test]
fn mollusk_preloaded_shares_do_not_poison_the_rate() {
    // Large enough that (PRELOAD + 800) * RATE_SCALE / 800 > u64::MAX, so a
    // balance-based rate would have left the u64 domain entirely.
    const PRELOAD: u64 = 15_000_000_000_000;
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let attacker = Pubkey::new_unique();
    let attacker_underlying = Pubkey::new_unique();
    let attacker_shares = Pubkey::new_unique();

    let mut accounts = fixture.accounts(0, 0);
    accounts.insert(attacker, system_account(1_000_000_000));
    accounts.insert(
        attacker_underlying,
        spl_token_account(fixture.underlying_mint, attacker, PRELOAD),
    );
    accounts.insert(
        attacker_shares,
        spl_token_account(fixture.share_mint, attacker, 0),
    );
    let context = mollusk().with_context(accounts);
    let mut ledger = CleartextLedger::default();
    fixture.seed_ledger(&mut ledger, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    seed_open_batch_balances(&context, &keys, &mut ledger);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        &mut ledger,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.bob,
        &mut ledger,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        500,
    );

    // The attacker deposits into the vault directly (empty vault, 1:1) ...
    let attacker_deposit = anchor_ix(
        vault::id(),
        vault::accounts::Deposit {
            depositor: attacker,
            vault: fixture.vault,
            vault_authority: fixture.vault_authority,
            underlying_mint: fixture.underlying_mint,
            share_mint: fixture.share_mint,
            depositor_underlying: attacker_underlying,
            vault_token_account: fixture.vault_token_account,
            depositor_shares: attacker_shares,
            token_program: spl_token::id(),
        },
        vault::instruction::Deposit { amount: PRELOAD },
    );
    context.process_and_validate_instruction(&attacker_deposit, &[Check::success()]);
    assert_eq!(read_spl_amount(&context, attacker_shares), PRELOAD);

    // ... and pushes the whole share balance into the batch's payout account.
    let preload_transfer = spl_token::instruction::transfer(
        &spl_token::id(),
        &attacker_shares,
        &keys.payout_underlying,
        &attacker,
        &[],
        PRELOAD,
    )
    .unwrap();
    context.process_and_validate_instruction(&preload_transfer, &[Check::success()]);
    assert_eq!(read_spl_amount(&context, keys.payout_underlying), PRELOAD);

    let burned_handle = run_dispatch(&context, &fixture, &keys, &mut ledger);
    run_settle(&context, &fixture, &keys, &mut ledger, burned_handle, 800);

    // Settle succeeded and the recorded payout reflects only the vault-minted
    // delta: 800 in, 800 shares out at the (still ~1:1) price, informational
    // rate exactly RATE_SCALE. The preloaded shares sit in the account,
    // unwrapped, inert.
    let settled = read_batch(&context, keys.batch);
    assert_eq!(settled.status, batcher::BatchStatus::Settled);
    assert_eq!(settled.total_joined, 800);
    assert_eq!(settled.payout_received, 800);
    assert_eq!(settled.payout_rate, batcher::RATE_SCALE);
    assert_eq!(read_spl_amount(&context, keys.payout_underlying), PRELOAD);
    assert_eq!(ledger.u64_at(&context, keys.payout_balance_value), 800);

    // Claims pay out exactly as in the clean lifecycle.
    run_claim(&context, &fixture, &keys, &fixture.alice, &mut ledger);
    run_claim(&context, &fixture, &keys, &fixture.bob, &mut ledger);
    assert_eq!(
        ledger.u64_at(&context, fixture.alice.shares.balance_value),
        300
    );
    assert_eq!(
        ledger.u64_at(&context, fixture.bob.shares.balance_value),
        500
    );
    assert_eq!(ledger.u64_at(&context, keys.payout_balance_value), 0);
}
