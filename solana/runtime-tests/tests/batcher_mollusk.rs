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
//! Encrypted values come from the cleartext host build, which records each
//! plaintext in its store. Each instruction's `fhe_execute` CPIs (there can be
//! several — a token CPI's execution plus the batcher's own) are decoded from
//! the inner instructions, counted and checked against the runtime sysvars.
//!
//! The fixture is direction-parametric: the same two confidential mints (one
//! wrapping the vault's underlying, one wrapping its share mint) serve a
//! deposit batcher (join = underlying, payout = shares) or a redeem batcher
//! (join = shares, payout = underlying).

use anchor_lang::{
    prelude::{system_program, Instructions},
    AccountDeserialize, Discriminator,
};
use anchor_spl::associated_token::get_associated_token_address_with_program_id;
use anchor_spl::token::spl_token;
use confidential_batcher as batcher;
use confidential_token as token;
use demo_vault as vault;
use mollusk_svm::{
    result::{Check, InstructionResult},
    Mollusk,
};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::SysvarId,
};
use std::collections::HashMap;
use zama_host as host;
use zama_solana_test_kit::cleartext::{fixture_context, seed_u64, store_u64};
use zama_solana_test_kit::executions;
use zama_solana_test_kit::signing::{
    amount_attestation_for, amount_public_decrypt_cert, amount_public_decrypt_cert_signed_by,
    kms_signing_key, kms_signing_key_n, production_amount_attestation_for, secp_evm_address,
};
use zama_solana_test_kit::{
    anchor_error_check, anchor_framework_error_check, anchor_ix, coprocessor_signer_address,
    cost_snapshot, deny_scope_record_account, encrypted_store_account, ensure_system_accounts,
    event_authority, handle_for_chain, hcu_trusted_app_record_account, host_config_account,
    kms_context_account, new_encrypted_store, paused_host_config, read_account, read_spl_amount,
    read_store_handle, readonly, serialized_account, spl_mint_account, spl_token_account,
    system_account, Ctx, HostConfigParams, BALANCE_FHE_TYPE, DECIMALS,
};

const KMS_CONTEXT_ID: [u8; 32] = {
    let mut id = [0u8; 32];
    id[31] = 9;
    id
};
/// Generous batch-authority funding for owner-charged rent (token-account and
/// encrypted store creation at open; the redeem marker and wrap growth at settle).
/// Sized for the cleartext host build, whose stores carry a plaintext section and cost more rent.
const AUTHORITY_FUNDING: u64 = 1_000_000_000;

// ---------------------------------------------------------------------------
// Harness
// ---------------------------------------------------------------------------

/// The batcher stack over the cleartext host build, so behavior tests read plaintexts from chain
/// state.
fn mollusk() -> Mollusk {
    mollusk_with_host(zama_solana_test_kit::cleartext::HOST_PROGRAM)
}

/// The batcher stack over the production host build.
fn production_mollusk() -> Mollusk {
    mollusk_with_host("zama_host")
}

fn mollusk_with_host(host_program: &str) -> Mollusk {
    let mut mollusk = zama_solana_test_kit::svm(&batcher::id(), "confidential_batcher");
    mollusk.add_program(&host::id(), host_program);
    mollusk.add_program(&token::id(), "confidential_token");
    mollusk.add_program(&vault::id(), "demo_vault");
    mollusk_svm_programs_token::token::add_program(&mut mollusk);
    zama_solana_test_kit::set_previous_bank_hash_sysvars(&mut mollusk);
    // Batcher instructions chain a token execution and the batcher's own execution;
    // real transactions request the same higher limit.
    mollusk.compute_budget.compute_unit_limit = 1_400_000;
    mollusk
}

fn batcher_error(error: batcher::BatcherError) -> Check<'static> {
    anchor_error_check(error as u32)
}

fn host_error(error: host::errors::ZamaHostError) -> Check<'static> {
    anchor_error_check(error as u32)
}

/// Wraps the batcher's FHE entry points in the same envelope clients submit.
fn check_batcher_instruction(
    context: &Ctx,
    ix: &Instruction,
    checks: &[Check],
) -> InstructionResult {
    if ix.program_id == batcher::ID {
        let payer_index = [
            (batcher::instruction::Join::DISCRIMINATOR, 1),
            (batcher::instruction::Quit::DISCRIMINATOR, 1),
            (batcher::instruction::OpenBatch::DISCRIMINATOR, 0),
            (batcher::instruction::Dispatch::DISCRIMINATOR, 0),
            (batcher::instruction::CancelDispatch::DISCRIMINATOR, 0),
            (batcher::instruction::Settle::DISCRIMINATOR, 0),
            (batcher::instruction::Claim::DISCRIMINATOR, 0),
        ]
        .into_iter()
        .find_map(|(tag, index)| ix.data.starts_with(tag).then_some(index));
        if let Some(index) = payer_index {
            return zama_solana_test_kit::transaction::process_fhe_instruction(
                context,
                ix.accounts[index].pubkey,
                ix,
                checks,
            );
        }
    }
    context.process_and_validate_instruction(ix, checks)
}

/// Checks a batcher instruction's `fhe_execute` CPIs (a token CPI's execution plus the
/// batcher's own) and returns how many there were, which the tests assert exactly.
fn check_fhe_cpis(context: &Ctx, result: &InstructionResult) -> usize {
    let checked = executions::check(context, result);
    assert!(
        checked.executions > 0,
        "expected at least one fhe_execute CPI in this instruction"
    );
    checked.executions
}

fn read_batch(context: &Ctx, address: Pubkey) -> batcher::Batch {
    read_account(context, address)
}

fn read_join_record(context: &Ctx, address: Pubkey) -> batcher::JoinRecord {
    read_account(context, address)
}

// ---------------------------------------------------------------------------
// Fixture
// ---------------------------------------------------------------------------

/// One confidential mint with its full PDA family.
struct ConfidentialMintKeys {
    mint: Pubkey,
    underlying_mint: Pubkey,
    total_supply_authority: Pubkey,
    total_supply_store: Pubkey,
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
            total_supply_authority,
            total_supply_store: token::encrypted_store_address(mint, total_supply_authority).0,
            vault_authority: token::vault_authority_address(mint).0,
            vault_underlying: token::vault_token_account_address(
                mint,
                underlying_mint,
                spl_token::id(),
            ),
            initial_total_supply: handle_for_chain(total_supply_seed, BALANCE_FHE_TYPE),
        }
    }

    fn mint_account(&self, authority: Pubkey) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialMint {
                authority,
                underlying_mint: self.underlying_mint,
                decimals: DECIMALS,
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
    balance_store: Pubkey,
    transferred_value: Pubkey,
    initial_balance: [u8; 32],
}

impl UserMintKeys {
    fn new(user: Pubkey, mint: &ConfidentialMintKeys, seed: u8) -> Self {
        let token_account = token::token_account_address(mint.mint, user).0;
        Self {
            token_account,
            balance_store: token::encrypted_store_address(mint.mint, token_account).0,
            transferred_value: token::encrypted_store_address(mint.mint, token_account).0,
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
    index: u64,
    batch: Pubkey,
    batch_authority: Pubkey,
    join_token_account: Pubkey,
    join_balance_store: Pubkey,
    burned_amount_store: Pubkey,
    payout_token_account: Pubkey,
    payout_balance_store: Pubkey,
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
            index,
            batch,
            batch_authority,
            join_token_account,
            join_balance_store: token::encrypted_store_address(join_mint.mint, join_token_account)
                .0,
            burned_amount_store: token::encrypted_store_address(join_mint.mint, join_token_account)
                .0,
            payout_token_account,
            payout_balance_store: token::encrypted_store_address(
                payout_mint.mint,
                payout_token_account,
            )
            .0,
            payout_transferred_value: token::encrypted_store_address(
                payout_mint.mint,
                payout_token_account,
            )
            .0,
            join_underlying: batcher::batch_join_underlying_address(batch).0,
            payout_underlying: batcher::batch_payout_underlying_address(batch).0,
        }
    }

    /// Canonical pending burn for this batch's confidential join token account.
    fn pending_burn(&self, join_mint: Pubkey) -> Pubkey {
        token::pending_burn_address(join_mint, self.join_token_account).0
    }

    fn pending_join_value(&self, user: Pubkey) -> Pubkey {
        batcher::join_store_id(self.batch, self.join_record(user)).address()
    }

    fn claim_amount_store(&self, user: Pubkey) -> Pubkey {
        self.pending_join_value(user)
    }

    fn join_record(&self, user: Pubkey) -> Pubkey {
        batcher::join_record_address(self.batch, user).0
    }

    /// The application the batcher's own executions for this batch run as.
    fn app(&self) -> host::AppScope {
        batch_app(self.batch)
    }
}

fn batch_app(batch: Pubkey) -> host::AppScope {
    host::AppScope {
        program: batcher::id(),
        scope: batch,
    }
}

/// The host levers a fixture turns on. Its instruction builders then carry what an honest client
/// supplies: the deny records of every execution while the deny list is on, and each
/// application's block meter and trust record while the block cap binds.
#[derive(Clone, Copy, Default)]
struct HostLevers {
    deny_list: bool,
    block_cap: BlockCap,
}

#[derive(Clone, Copy, Default, PartialEq)]
enum BlockCap {
    #[default]
    Unrestricted,
    /// [`BINDING_BLOCK_CAP`], with every application the batcher runs as trusted.
    Trusted,
    /// [`BINDING_BLOCK_CAP`], with every application metered.
    Metered,
}

/// A finite per-application block cap, above what one test spends in one slot.
const BINDING_BLOCK_CAP: u64 = 100_000_000;

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
    levers: HostLevers,
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
            levers: HostLevers::default(),
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
            levers: self.levers,
        }
    }

    /// The application the join mint's token executions run as.
    fn join_app(&self) -> host::AppScope {
        token::token_app(self.join_mint().mint)
    }

    /// The application the payout mint's token executions run as.
    fn payout_app(&self) -> host::AppScope {
        token::token_app(self.payout_mint().mint)
    }

    /// The applications this fixture's first two batches run as.
    fn apps(&self) -> [host::AppScope; 4] {
        [
            self.join_app(),
            self.payout_app(),
            batch_app(batcher::batch_address(self.batcher, 0).0),
            batch_app(batcher::batch_address(self.batcher, 1).0),
        ]
    }

    fn hcu_block_meter(&self, app: host::AppScope) -> Option<Pubkey> {
        (self.levers.block_cap != BlockCap::Unrestricted)
            .then(|| host::hcu_block_meter_address(app).0)
    }

    fn hcu_trusted_app_record(&self, app: host::AppScope) -> Option<Pubkey> {
        (self.levers.block_cap != BlockCap::Unrestricted)
            .then(|| host::hcu_trusted_app_address(app).0)
    }

    /// Appends, while the deny list is on, the deny records of each execution in order.
    fn with_deny_records(
        &self,
        mut ix: Instruction,
        executions: &[&[host::AppScope]],
    ) -> Instruction {
        if self.levers.deny_list {
            ix.accounts.extend(
                executions
                    .iter()
                    .flat_map(|apps| apps.iter())
                    .map(|app| readonly(host::deny_scope_address(*app).0)),
            );
        }
        ix
    }

    fn host_config_account(&self) -> Account {
        host_config_account(&HostConfigParams {
            current_kms_context_id: KMS_CONTEXT_ID,
            // This suite mints `fromExternal` attestations, so it registers the kit's signing key
            // explicitly; the default signer set trusts nobody.
            coprocessor_signers: vec![coprocessor_signer_address()],
            grant_deny_list_enabled: self.levers.deny_list,
            hcu_block_cap_per_app: match self.levers.block_cap {
                BlockCap::Unrestricted => u64::MAX,
                BlockCap::Trusted | BlockCap::Metered => BINDING_BLOCK_CAP,
            },
            ..HostConfigParams::new(self.payer)
        })
        .1
    }

    fn kms_context_account(&self) -> Account {
        kms_context_account(
            KMS_CONTEXT_ID,
            vec![secp_evm_address(&kms_signing_key())],
            1,
        )
        .1
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
        _balance_store: Pubkey,
    ) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialTokenAccount {
                owner,
                mint: mint.mint,
                bump: token::token_account_address(mint.mint, owner).1,
            }),
            owner: token::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    /// Full account set: host + KMS fixtures, both confidential mints, the
    /// demo vault at `(total_assets, total_shares)`, and both users with
    /// seeded balance encrypted stores on both mints. The confidential mints' plain
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
            accounts.insert(mint.total_supply_authority, system_account(0));
            accounts.insert(mint.vault_authority, system_account(0));
            accounts.insert(
                mint.vault_underlying,
                spl_token_account(mint.underlying_mint, mint.vault_authority, escrow_amount),
            );
            let (_, total_supply) = new_encrypted_store(
                token::token_app(mint.mint),
                mint.total_supply_authority,
                [(token::total_supply_key(), mint.initial_total_supply)],
            );
            accounts.insert(
                mint.total_supply_store,
                encrypted_store_account(&total_supply),
            );
        }
        for user in [&self.alice, &self.bob] {
            for (mint, keys) in [
                (&self.underlying_cmint, &user.underlying),
                (&self.shares_cmint, &user.shares),
            ] {
                accounts.insert(
                    keys.token_account,
                    self.confidential_token_account(mint, user.user, keys.balance_store),
                );
                let (_, balance) = new_encrypted_store(
                    token::token_app(mint.mint),
                    keys.token_account,
                    [(token::balance_key(), keys.initial_balance)],
                );
                accounts.insert(keys.balance_store, encrypted_store_account(&balance));
            }
        }
        if self.levers.deny_list {
            // Denying an unrelated application must not affect the batcher.
            let unrelated = host::AppScope {
                program: Pubkey::new_unique(),
                scope: Pubkey::new_unique(),
            };
            for app in self.apps() {
                let (record, account) = deny_scope_record_account(app, false);
                accounts.insert(record, account);
            }
            let (record, account) = deny_scope_record_account(unrelated, true);
            accounts.insert(record, account);
        }
        if self.levers.block_cap != BlockCap::Unrestricted {
            for app in self.apps() {
                accounts.insert(host::hcu_block_meter_address(app).0, system_account(0));
                let (record, account) = match self.levers.block_cap {
                    BlockCap::Trusted => hcu_trusted_app_record_account(app, true),
                    _ => (host::hcu_trusted_app_address(app).0, system_account(0)),
                };
                accounts.insert(record, account);
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

    /// Seeds the plaintexts of the fixture's initial handles:
    /// per-user `(underlying, shares)` balances and both encrypted supplies.
    fn seed_values(&self, context: &Ctx, alice: (u64, u64), bob: (u64, u64), supplies: (u64, u64)) {
        seed_u64(context, self.alice.underlying.initial_balance, alice.0);
        seed_u64(context, self.alice.shares.initial_balance, alice.1);
        seed_u64(context, self.bob.underlying.initial_balance, bob.0);
        seed_u64(context, self.bob.shares.initial_balance, bob.1);
        seed_u64(
            context,
            self.underlying_cmint.initial_total_supply,
            supplies.0,
        );
        seed_u64(context, self.shares_cmint.initial_total_supply, supplies.1);
    }
}

// ---------------------------------------------------------------------------
// Instruction builders
// ---------------------------------------------------------------------------

/// The settle deadline every test batcher uses.
const SETTLE_DEADLINE_SECS: u64 = 3_600;

/// Moves the clock to the settle deadline of a batch dispatched at the current time: settle is
/// refused from then on, and anyone may cancel the dispatch.
fn reach_settle_deadline(context: &mut Ctx) {
    context.mollusk.sysvars.clock.unix_timestamp += SETTLE_DEADLINE_SECS as i64;
}

/// Has `payer` pay for `ix` in place of the account at `payer_index`. The transaction's transient
/// store belongs to its payer, so it moves too.
fn with_payer(mut ix: Instruction, payer_index: usize, payer: Pubkey) -> Instruction {
    let replaced = std::mem::replace(&mut ix.accounts[payer_index].pubkey, payer);
    ix.accounts
        .iter_mut()
        .find(|meta| meta.pubkey == host::transient_store_address(replaced).0)
        .unwrap()
        .pubkey = host::transient_store_address(payer).0;
    ix
}

fn initialize_batcher_ix(fixture: &BatcherFixture, min_batch_age_secs: u64) -> Instruction {
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
            min_batch_age_secs,
            settle_deadline_secs: SETTLE_DEADLINE_SECS,
            direction: fixture.direction,
        },
    )
}

fn open_batch_ix(
    fixture: &BatcherFixture,
    keys: &BatchKeys,
    previous_batch: Option<Pubkey>,
) -> Instruction {
    let ix = anchor_ix(
        batcher::id(),
        batcher::accounts::OpenBatch {
            transient_store: host::transient_store_address(fixture.payer).0,
            instructions: Instructions::id(),
            payer: fixture.payer,
            batcher: fixture.batcher,
            previous_batch,
            batch: keys.batch,
            batch_authority: keys.batch_authority,
            join_confidential_mint: fixture.join_mint().mint,
            batch_join_token_account: keys.join_token_account,
            batch_join_balance_store: keys.join_balance_store,
            payout_confidential_mint: fixture.payout_mint().mint,
            batch_payout_token_account: keys.payout_token_account,
            batch_payout_balance_store: keys.payout_balance_store,
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
            join_mint_hcu_block_meter: fixture.hcu_block_meter(fixture.join_app()),
            join_mint_hcu_trusted_app_record: fixture.hcu_trusted_app_record(fixture.join_app()),
            payout_mint_hcu_block_meter: fixture.hcu_block_meter(fixture.payout_app()),
            payout_mint_hcu_trusted_app_record: fixture
                .hcu_trusted_app_record(fixture.payout_app()),
        },
        batcher::instruction::OpenBatch {
            index: keys.index,
            authority_funding_lamports: AUTHORITY_FUNDING,
        },
    );
    fixture.with_deny_records(ix, &[&[fixture.join_app()], &[fixture.payout_app()]])
}

fn owner_ata(owner: Pubkey, underlying_mint: Pubkey) -> Pubkey {
    get_associated_token_address_with_program_id(&owner, &underlying_mint, &spl_token::id())
}

fn join_ix(
    fixture: &BatcherFixture,
    keys: &BatchKeys,
    user: &UserKeys,
    amount_attestation: host::CoprocessorInputAttestation,
) -> Instruction {
    let user_join = fixture.user_join(user);
    let ix = anchor_ix(
        batcher::id(),
        batcher::accounts::Join {
            user: user.user,
            payer: user.user,
            batcher: fixture.batcher,
            batch: keys.batch,
            batch_authority: keys.batch_authority,
            join_record: keys.join_record(user.user),
            join_confidential_mint: fixture.join_mint().mint,
            join_underlying_mint: fixture.join_mint().underlying_mint,
            user_ata: owner_ata(user.user, fixture.join_mint().underlying_mint),
            batch_authority_ata: owner_ata(
                keys.batch_authority,
                fixture.join_mint().underlying_mint,
            ),
            user_token_account: user_join.token_account,
            batch_join_token_account: keys.join_token_account,
            user_balance_store: user_join.balance_store,
            batch_balance_store: keys.join_balance_store,
            join_store: keys.pending_join_value(user.user),
            transient_store: host::transient_store_address(user.user).0,
            instructions: Instructions::id(),
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            confidential_token_event_authority: event_authority(token::id()),
            confidential_token_program: token::id(),
            system_program: system_program::ID,
            join_mint_hcu_block_meter: fixture.hcu_block_meter(fixture.join_app()),
            join_mint_hcu_trusted_app_record: fixture.hcu_trusted_app_record(fixture.join_app()),
            batch_hcu_block_meter: fixture.hcu_block_meter(keys.app()),
            batch_hcu_trusted_app_record: fixture.hcu_trusted_app_record(keys.app()),
        },
        batcher::instruction::Join { amount_attestation },
    );
    fixture.with_deny_records(ix, &[&[fixture.join_app()], &[keys.app()]])
}

fn quit_ix(fixture: &BatcherFixture, keys: &BatchKeys, user: &UserKeys) -> Instruction {
    let user_join = fixture.user_join(user);
    let mut ix = anchor_ix(
        batcher::id(),
        batcher::accounts::Quit {
            transient_store: host::transient_store_address(user.user).0,
            instructions: Instructions::id(),
            user: user.user,
            payer: user.user,
            batcher: fixture.batcher,
            batch: keys.batch,
            batch_authority: keys.batch_authority,
            join_record: keys.join_record(user.user),
            join_confidential_mint: fixture.join_mint().mint,
            join_underlying_mint: fixture.join_mint().underlying_mint,
            batch_authority_ata: owner_ata(
                keys.batch_authority,
                fixture.join_mint().underlying_mint,
            ),
            user_ata: owner_ata(user.user, fixture.join_mint().underlying_mint),
            batch_join_token_account: keys.join_token_account,
            user_token_account: user_join.token_account,
            batch_balance_store: keys.join_balance_store,
            user_balance_store: user_join.balance_store,
            join_store: keys.pending_join_value(user.user),
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            confidential_token_event_authority: event_authority(token::id()),
            confidential_token_program: token::id(),
            system_program: system_program::ID,
            join_mint_hcu_block_meter: fixture.hcu_block_meter(fixture.join_app()),
            join_mint_hcu_trusted_app_record: fixture.hcu_trusted_app_record(fixture.join_app()),
            batch_hcu_block_meter: fixture.hcu_block_meter(keys.app()),
            batch_hcu_trusted_app_record: fixture.hcu_trusted_app_record(keys.app()),
        },
        batcher::instruction::Quit {},
    );
    // The user may sign; a pending batch's quit requires it.
    ix.accounts[0].is_signer = true;
    fixture.with_deny_records(ix, &[&[fixture.join_app(), keys.app()], &[keys.app()]])
}

/// A confidential transfer of `amount` from `donor`'s account on `mint` to `recipient`'s account: a
/// gift the batcher keeps no record of.
fn donate_ix(
    fixture: &BatcherFixture,
    donor: &UserKeys,
    mint: &ConfidentialMintKeys,
    recipient: Pubkey,
    amount_handle: [u8; 32],
    amount: u64,
) -> Instruction {
    let from_account = token::token_account_address(mint.mint, donor.user).0;
    let to_account = token::token_account_address(mint.mint, recipient).0;
    let app = token::token_app(mint.mint);
    let ix = anchor_ix(
        token::id(),
        token::accounts::ConfidentialTransfer {
            transient_store: host::transient_store_address(donor.user).0,
            instructions: Instructions::id(),
            owner: donor.user,
            payer: donor.user,
            mint: mint.mint,
            underlying_mint: mint.underlying_mint,
            from_ata: owner_ata(donor.user, mint.underlying_mint),
            to_ata: owner_ata(recipient, mint.underlying_mint),
            from_account,
            to_account,
            from_store: token::encrypted_store_address(mint.mint, from_account).0,
            to_store: token::encrypted_store_address(mint.mint, to_account).0,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            hcu_block_meter: fixture.hcu_block_meter(app),
            hcu_trusted_app_record: fixture.hcu_trusted_app_record(app),
            result_store: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialTransfer {
            amount_attestation: amount_attestation_for(
                amount_handle,
                amount,
                donor.user,
                token::id(),
            ),
        },
    );
    fixture.with_deny_records(ix, &[&[app]])
}

fn dispatch_ix(fixture: &BatcherFixture, keys: &BatchKeys) -> Instruction {
    let ix = anchor_ix(
        batcher::id(),
        batcher::accounts::Dispatch {
            transient_store: host::transient_store_address(fixture.payer).0,
            instructions: Instructions::id(),
            payer: fixture.payer,
            batcher: fixture.batcher,
            batch: keys.batch,
            batch_authority: keys.batch_authority,
            join_confidential_mint: fixture.join_mint().mint,
            join_underlying_mint: fixture.join_mint().underlying_mint,
            batch_authority_ata: owner_ata(
                keys.batch_authority,
                fixture.join_mint().underlying_mint,
            ),
            total_supply_authority: fixture.join_mint().total_supply_authority,
            batch_join_token_account: keys.join_token_account,
            batch_balance_store: keys.join_balance_store,
            total_supply_store: fixture.join_mint().total_supply_store,
            pending_burn: keys.pending_burn(fixture.join_mint().mint),
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            confidential_token_event_authority: event_authority(token::id()),
            confidential_token_program: token::id(),
            system_program: system_program::ID,
            join_mint_hcu_block_meter: fixture.hcu_block_meter(fixture.join_app()),
            join_mint_hcu_trusted_app_record: fixture.hcu_trusted_app_record(fixture.join_app()),
        },
        batcher::instruction::Dispatch {},
    );
    fixture.with_deny_records(ix, &[&[fixture.join_app()]])
}

fn cancel_dispatch_ix(fixture: &BatcherFixture, keys: &BatchKeys) -> Instruction {
    let ix = anchor_ix(
        batcher::id(),
        batcher::accounts::CancelDispatch {
            transient_store: host::transient_store_address(fixture.payer).0,
            instructions: Instructions::id(),
            payer: fixture.payer,
            batcher: fixture.batcher,
            batch: keys.batch,
            batch_authority: keys.batch_authority,
            join_confidential_mint: fixture.join_mint().mint,
            total_supply_authority: fixture.join_mint().total_supply_authority,
            batch_join_token_account: keys.join_token_account,
            batch_balance_store: keys.join_balance_store,
            total_supply_store: fixture.join_mint().total_supply_store,
            pending_burn: keys.pending_burn(fixture.join_mint().mint),
            host_config: fixture.host_config,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            confidential_token_event_authority: event_authority(token::id()),
            confidential_token_program: token::id(),
            system_program: system_program::ID,
            join_mint_hcu_block_meter: fixture.hcu_block_meter(fixture.join_app()),
            join_mint_hcu_trusted_app_record: fixture.hcu_trusted_app_record(fixture.join_app()),
        },
        batcher::instruction::CancelDispatch {
            authority_funding_lamports: AUTHORITY_FUNDING,
        },
    );
    fixture.with_deny_records(ix, &[&[fixture.join_app()]])
}

fn settle_ix(
    fixture: &BatcherFixture,
    keys: &BatchKeys,
    cleartext_total: u64,
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
    pending_burn: Pubkey,
) -> Instruction {
    let ix = anchor_ix(
        batcher::id(),
        batcher::accounts::Settle {
            transient_store: host::transient_store_address(fixture.payer).0,
            instructions: Instructions::id(),
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
            batch_burned_amount_store: keys.burned_amount_store,
            pending_burn,
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
            payout_total_supply_authority: fixture.payout_mint().total_supply_authority,
            batch_payout_balance_store: keys.payout_balance_store,
            payout_total_supply_store: fixture.payout_mint().total_supply_store,
            join_total_supply_authority: fixture.join_mint().total_supply_authority,
            join_total_supply_store: fixture.join_mint().total_supply_store,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            confidential_token_event_authority: event_authority(token::id()),
            confidential_token_program: token::id(),
            demo_vault_program: vault::id(),
            token_program: spl_token::id(),
            system_program: system_program::ID,
            payout_mint_hcu_block_meter: fixture.hcu_block_meter(fixture.payout_app()),
            payout_mint_hcu_trusted_app_record: fixture
                .hcu_trusted_app_record(fixture.payout_app()),
            join_mint_hcu_block_meter: fixture.hcu_block_meter(fixture.join_app()),
            join_mint_hcu_trusted_app_record: fixture.hcu_trusted_app_record(fixture.join_app()),
        },
        batcher::instruction::Settle {
            cleartext_total,
            signatures,
            extra_data,
            authority_funding_lamports: AUTHORITY_FUNDING,
        },
    );
    let wraps: &[host::AppScope] = if cleartext_total == 0 {
        &[]
    } else {
        &[fixture.payout_app(), fixture.join_app()]
    };
    fixture.with_deny_records(ix, &[wraps])
}

fn claim_ix(fixture: &BatcherFixture, keys: &BatchKeys, user: &UserKeys) -> Instruction {
    let user_payout = fixture.user_payout(user);
    let ix = anchor_ix(
        batcher::id(),
        batcher::accounts::Claim {
            payer: fixture.payer,
            user: user.user,
            batcher: fixture.batcher,
            batch: keys.batch,
            batch_authority: keys.batch_authority,
            join_record: keys.join_record(user.user),
            join_store: keys.pending_join_value(user.user),
            transient_store: host::transient_store_address(fixture.payer).0,
            instructions: Instructions::id(),
            payout_confidential_mint: fixture.payout_mint().mint,
            payout_underlying_mint: fixture.payout_mint().underlying_mint,
            batch_authority_payout_ata: owner_ata(
                keys.batch_authority,
                fixture.payout_mint().underlying_mint,
            ),
            user_payout_ata: owner_ata(user.user, fixture.payout_mint().underlying_mint),
            batch_payout_token_account: keys.payout_token_account,
            user_payout_token_account: user_payout.token_account,
            batch_payout_balance_store: keys.payout_balance_store,
            user_payout_balance_store: user_payout.balance_store,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            confidential_token_event_authority: event_authority(token::id()),
            confidential_token_program: token::id(),
            system_program: system_program::ID,
            batch_hcu_block_meter: fixture.hcu_block_meter(keys.app()),
            batch_hcu_trusted_app_record: fixture.hcu_trusted_app_record(keys.app()),
            payout_mint_hcu_block_meter: fixture.hcu_block_meter(fixture.payout_app()),
            payout_mint_hcu_trusted_app_record: fixture
                .hcu_trusted_app_record(fixture.payout_app()),
        },
        batcher::instruction::Claim {},
    );
    fixture.with_deny_records(ix, &[&[keys.app()], &[fixture.payout_app()]])
}

fn reclaim_batch_authority_ix(
    fixture: &BatcherFixture,
    keys: &BatchKeys,
    authority: Pubkey,
) -> Instruction {
    anchor_ix(
        batcher::id(),
        batcher::accounts::ReclaimBatchAuthority {
            authority,
            batcher: fixture.batcher,
            batch: keys.batch,
            batch_authority: keys.batch_authority,
            join_confidential_mint: fixture.join_mint().mint,
            system_program: system_program::ID,
        },
        batcher::instruction::ReclaimBatchAuthority {},
    )
}

fn close_join_record_ix(keys: &BatchKeys, user: Pubkey) -> Instruction {
    anchor_ix(
        batcher::id(),
        batcher::accounts::CloseJoinRecord {
            user,
            batch: keys.batch,
            join_record: keys.join_record(user),
        },
        batcher::instruction::CloseJoinRecord {},
    )
}

fn lamports_of(context: &Ctx, address: Pubkey) -> u64 {
    context
        .account_store
        .borrow()
        .get(&address)
        .map_or(0, |account| account.lamports)
}

/// Reclaims the batch authority funding as the join mint's wrapper authority (the fixture payer)
/// and asserts every lamport moved.
fn run_reclaim_batch_authority(context: &Ctx, fixture: &BatcherFixture, keys: &BatchKeys) {
    let funding = lamports_of(context, keys.batch_authority);
    assert!(
        funding > 0,
        "the batch authority holds its funding until reclaimed"
    );
    let before = lamports_of(context, fixture.payer);
    check_batcher_instruction(
        context,
        &reclaim_batch_authority_ix(fixture, keys, fixture.payer),
        &[Check::success()],
    );
    assert_eq!(lamports_of(context, keys.batch_authority), 0);
    assert_eq!(lamports_of(context, fixture.payer), before + funding);
}

// ---------------------------------------------------------------------------
// Lifecycle drivers
// ---------------------------------------------------------------------------

/// Initializes the batcher and opens batch 0, returning its keys.
fn initialize_and_open_first_batch(
    context: &Ctx,
    fixture: &BatcherFixture,
    min_batch_age_secs: u64,
) -> BatchKeys {
    check_batcher_instruction(
        context,
        &initialize_batcher_ix(fixture, min_batch_age_secs),
        &[Check::success()],
    );
    let keys = BatchKeys::new(fixture, 0);
    ensure_open_batch_accounts(context, fixture, &keys);
    check_batcher_instruction(
        context,
        &open_batch_ix(fixture, &keys, None),
        &[Check::success()],
    );
    keys
}

fn ensure_open_batch_accounts(context: &Ctx, fixture: &BatcherFixture, keys: &BatchKeys) {
    ensure_system_accounts(
        context,
        &[
            keys.batch,
            keys.batch_authority,
            keys.join_token_account,
            keys.join_balance_store,
            keys.payout_token_account,
            keys.payout_balance_store,
            keys.join_underlying,
            keys.payout_underlying,
            owner_ata(keys.batch_authority, fixture.join_mint().underlying_mint),
            owner_ata(keys.batch_authority, fixture.payout_mint().underlying_mint),
        ],
    );
}

/// Runs one join and checks its FHE CPIs.
fn run_join(
    context: &Ctx,
    fixture: &BatcherFixture,
    keys: &BatchKeys,
    user: &UserKeys,
    amount_handle: [u8; 32],
    amount: u64,
) {
    ensure_system_accounts(
        context,
        &[
            keys.join_record(user.user),
            keys.pending_join_value(user.user),
            owner_ata(user.user, fixture.join_mint().underlying_mint),
            owner_ata(keys.batch_authority, fixture.join_mint().underlying_mint),
        ],
    );
    let attestation = amount_attestation_for(amount_handle, amount, user.user, token::id());
    let ix = join_ix(fixture, keys, user, attestation);
    let result = check_batcher_instruction(context, &ix, &[Check::success()]);
    assert_eq!(check_fhe_cpis(context, &result), 2);
}

/// Quits one user's join, checking the refund's and the reset's executions.
fn run_quit(context: &Ctx, fixture: &BatcherFixture, keys: &BatchKeys, user: &UserKeys) {
    let result =
        check_batcher_instruction(context, &quit_ix(fixture, keys, user), &[Check::success()]);
    assert_eq!(check_fhe_cpis(context, &result), 2);
}

/// Dispatches the batch and returns the created-public burned handle.
fn run_dispatch(context: &Ctx, fixture: &BatcherFixture, keys: &BatchKeys) -> [u8; 32] {
    ensure_system_accounts(
        context,
        &[
            keys.burned_amount_store,
            keys.pending_burn(fixture.join_mint().mint),
            owner_ata(keys.batch_authority, fixture.join_mint().underlying_mint),
        ],
    );
    let ix = dispatch_ix(fixture, keys);
    let result = check_batcher_instruction(context, &ix, &[Check::success()]);
    assert_eq!(check_fhe_cpis(context, &result), 1);
    read_batch(context, keys.batch).burned_total_handle
}

/// Settles the batch with a real KMS cert over `total`, replaying the wrap's
/// execution when the batch is non-zero. Returns the instruction and result so the
/// fixed-key lifecycle can snapshot cost; behavior-test callers ignore them.
fn run_settle(
    context: &Ctx,
    fixture: &BatcherFixture,
    keys: &BatchKeys,
    burned_handle: [u8; 32],
    total: u64,
) -> (Instruction, InstructionResult) {
    let (signatures, extra_data) = amount_public_decrypt_cert(burned_handle, total);
    let pending_burn = keys.pending_burn(fixture.join_mint().mint);
    let ix = settle_ix(fixture, keys, total, signatures, extra_data, pending_burn);
    let result = check_batcher_instruction(context, &ix, &[Check::success()]);
    if total > 0 {
        // Only the wrap phase drives an execution at settle.
        assert_eq!(check_fhe_cpis(context, &result), 1);
    }
    (ix, result)
}

/// Claims for one user, replaying the MulDiv and transfer executions.
fn run_claim(context: &Ctx, fixture: &BatcherFixture, keys: &BatchKeys, user: &UserKeys) {
    ensure_system_accounts(
        context,
        &[
            owner_ata(keys.batch_authority, fixture.payout_mint().underlying_mint),
            owner_ata(user.user, fixture.payout_mint().underlying_mint),
        ],
    );
    let ix = claim_ix(fixture, keys, user);
    let result = check_batcher_instruction(context, &ix, &[Check::success()]);
    // The claim issues the batcher's MulDiv execution plus the transfer's execution.
    assert_eq!(check_fhe_cpis(context, &result), 2);
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
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);

    // Batch opens with an encrypted zero balance on both sides.
    let open_result = read_batch(&context, keys.batch);
    assert_eq!(open_result.status, batcher::BatchStatus::Pending);

    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.bob,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        500,
    );

    // Encrypted accounting after the joins: user balances debited, the batch
    // account holds the (still encrypted) sum, each joined encrypted store carries
    // that user's amount and only that user's amount.
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.underlying.balance_store,
            token::balance_key()
        ),
        700
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.bob.underlying.balance_store,
            token::balance_key()
        ),
        1_500
    );
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        800
    );
    assert_eq!(
        store_u64(
            &context,
            keys.pending_join_value(fixture.alice.user),
            batcher::JOINED_AMOUNT_KEY
        ),
        300
    );
    assert_eq!(
        store_u64(
            &context,
            keys.pending_join_value(fixture.bob.user),
            batcher::JOINED_AMOUNT_KEY
        ),
        500
    );
    assert_eq!(read_batch(&context, keys.batch).join_count, 2);

    // Dispatch burns the batch's whole balance; the burned encrypted store carries the
    // batch total, created publicly decryptable.
    let burned_handle = run_dispatch(&context, &fixture, &keys);
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        0
    );
    assert_eq!(
        store_u64(
            &context,
            keys.burned_amount_store,
            token::burned_amount_key()
        ),
        800
    );
    assert_eq!(
        read_batch(&context, keys.batch).status,
        batcher::BatchStatus::Dispatched
    );
    // The authority funding stays with a live batch: settle still charges the authority.
    check_batcher_instruction(
        &context,
        &reclaim_batch_authority_ix(&fixture, &keys, fixture.payer),
        &[batcher_error(batcher::BatcherError::BatchStillLive)],
    );

    // Settle: the KMS certifies 800; the vault (empty, 1:1) mints 800 shares;
    // the informational rate lands at exactly RATE_SCALE.
    run_settle(&context, &fixture, &keys, burned_handle, 800);
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
    assert_eq!(
        store_u64(&context, keys.payout_balance_store, token::balance_key()),
        800
    );

    // A record is spent only once its payout is claimed.
    check_batcher_instruction(
        &context,
        &close_join_record_ix(&keys, fixture.alice.user),
        &[batcher_error(batcher::BatcherError::JoinRecordStillLive)],
    );

    // Claims: each user receives their exact proportional encrypted shares.
    run_claim(&context, &fixture, &keys, &fixture.alice);

    // Once settled, the operator takes the unspent authority funding back. A stranger cannot, and
    // Bob's claim afterwards shows claims pay their own rent: the drained authority only signs.
    let stranger = Pubkey::new_unique();
    context
        .account_store
        .borrow_mut()
        .insert(stranger, system_account(1_000_000_000));
    check_batcher_instruction(
        &context,
        &reclaim_batch_authority_ix(&fixture, &keys, stranger),
        &[batcher_error(
            batcher::BatcherError::ReclaimAuthorityMismatch,
        )],
    );
    run_reclaim_batch_authority(&context, &fixture, &keys);
    run_claim(&context, &fixture, &keys, &fixture.bob);
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.shares.balance_store,
            token::balance_key()
        ),
        300
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.bob.shares.balance_store,
            token::balance_key()
        ),
        500
    );
    assert_eq!(
        store_u64(&context, keys.payout_balance_store, token::balance_key()),
        0
    );
    assert!(read_join_record(&context, keys.join_record(fixture.alice.user)).claimed);
    assert!(read_join_record(&context, keys.join_record(fixture.bob.user)).claimed);

    // Alice closes her spent record and gets its rent back; Bob's stays until he does the same.
    let record = keys.join_record(fixture.alice.user);
    let rent = lamports_of(&context, record);
    let before = lamports_of(&context, fixture.alice.user);
    check_batcher_instruction(
        &context,
        &close_join_record_ix(&keys, fixture.alice.user),
        &[Check::success()],
    );
    assert_eq!(lamports_of(&context, record), 0);
    assert_eq!(lamports_of(&context, fixture.alice.user), before + rent);
    assert!(read_join_record(&context, keys.join_record(fixture.bob.user)).claimed);
}

/// Lifecycle against a vault with existing yield (2_000 assets / 1_000
/// shares): the floor-rounded exact-proportional claims never exceed the
/// wrapped shares.
#[test]
fn mollusk_lifecycle_with_yield_rate_rounds_down() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    // Share price ~2: 2_000 assets backing 1_000 shares.
    let context = fixture_context(mollusk(), fixture.accounts(2_000, 1_000));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);

    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.bob,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        500,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys);
    run_settle(&context, &fixture, &keys, burned_handle, 800);

    // shares = 800 * (1_000 + 1) / (2_000 + 1) = 400 (floor);
    // informational rate = 400 * RATE_SCALE / 800 = RATE_SCALE / 2.
    let settled = read_batch(&context, keys.batch);
    assert_eq!(settled.payout_received, 400);
    assert_eq!(settled.payout_rate, batcher::RATE_SCALE / 2);

    run_claim(&context, &fixture, &keys, &fixture.alice);
    run_claim(&context, &fixture, &keys, &fixture.bob);
    // 300 * 400 / 800 = 150 and 500 -> 250; the claims sum exactly to the
    // wrapped 400 here, and can never exceed it by the floor rounding.
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.shares.balance_store,
            token::balance_key()
        ),
        150
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.bob.shares.balance_store,
            token::balance_key()
        ),
        250
    );
    assert_eq!(
        store_u64(&context, keys.payout_balance_store, token::balance_key()),
        0
    );
}

/// A single-participant batch settles correctly but reveals that participant's
/// amount: the certified public total IS their deposit. This documents the
/// known privacy caveat (CONFIDENTIAL_VAULTS.md) — privacy grows with genuine
/// participants per batch, and the design deliberately does not gate on
/// participant count.
#[test]
fn mollusk_single_user_batch_reveals_that_users_amount() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);

    let alice_amount = 777;
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        alice_amount,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys);
    run_settle(&context, &fixture, &keys, burned_handle, alice_amount);

    // The amount-reveal caveat: with one participant, the public batch total
    // equals their private deposit exactly.
    let settled = read_batch(&context, keys.batch);
    assert_eq!(settled.total_joined, alice_amount);
    assert_eq!(
        settled.total_joined,
        store_u64(
            &context,
            keys.pending_join_value(fixture.alice.user),
            batcher::JOINED_AMOUNT_KEY
        )
    );

    run_claim(&context, &fixture, &keys, &fixture.alice);
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.shares.balance_store,
            token::balance_key()
        ),
        alice_amount
    );
}

/// Repeated joins accumulate in the joined encrypted store (the operand-aliases-output
/// update), quit refunds the exact accumulated amount all-or-nothing and
/// resets the encrypted store to zero, and a re-join after quit accumulates from zero.
#[test]
fn mollusk_repeat_join_accumulates_and_quit_refunds_exactly() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);

    // Two joins accumulate: the second join's execution reads the joined encrypted store
    // as an operand AND updates it as the output (the #3238 aliasing class
    // for the batcher's own execution — the standard same-slot update).
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        100,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        250,
    );
    let pending = keys.pending_join_value(fixture.alice.user);
    assert_eq!(
        store_u64(&context, pending, batcher::JOINED_AMOUNT_KEY),
        350
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.underlying.balance_store,
            token::balance_key()
        ),
        650
    );

    // Quit refunds exactly 350 (all-or-nothing) and resets the encrypted store to zero.
    let quit = quit_ix(&fixture, &keys, &fixture.alice);
    let result = check_batcher_instruction(&context, &quit, &[Check::success()]);
    assert_eq!(check_fhe_cpis(&context, &result), 2);
    assert_eq!(store_u64(&context, pending, batcher::JOINED_AMOUNT_KEY), 0);
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.underlying.balance_store,
            token::balance_key()
        ),
        1_000
    );
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        0
    );

    // Re-join after quit accumulates from zero, not from the stale amount.
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(43, BALANCE_FHE_TYPE),
        40,
    );
    assert_eq!(store_u64(&context, pending, batcher::JOINED_AMOUNT_KEY), 40);
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        40
    );
}

/// A dispatched batch is not dependent on eventual KMS availability: cancellation restores the
/// confidential burn, closes the token account's pending burn, and moves the batch into a
/// refund-only state in which each participant can retrieve their recorded amount.
#[test]
fn mollusk_cancel_dispatch_restores_burn_and_allows_refunds() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let mut context = fixture_context(mollusk(), fixture.accounts(0, 0));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );

    // Dispatch a full deadline after opening, so a deadline counted from anything but the dispatch
    // would already have passed.
    context.mollusk.sysvars.clock.unix_timestamp += SETTLE_DEADLINE_SECS as i64;
    let pending_burn = keys.pending_burn(fixture.join_mint().mint);
    let burned_handle = run_dispatch(&context, &fixture, &keys);
    let (signatures, extra_data) = amount_public_decrypt_cert(burned_handle, 300);
    let settle = settle_ix(&fixture, &keys, 300, signatures, extra_data, pending_burn);
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        0
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.join_mint().total_supply_store,
            token::total_supply_key()
        ),
        999_700
    );

    // Before the settle deadline nobody can force the terminal refund path, so a watcher cannot
    // front-run settlement. From the deadline on, settle is refused and anyone may cancel.
    let stranger = Pubkey::new_unique();
    context
        .account_store
        .borrow_mut()
        .insert(stranger, system_account(5_000_000_000));
    let cancel = with_payer(cancel_dispatch_ix(&fixture, &keys), 0, stranger);
    context.mollusk.sysvars.clock.unix_timestamp += SETTLE_DEADLINE_SECS as i64 - 1;
    check_batcher_instruction(
        &context,
        &cancel,
        &[batcher_error(
            batcher::BatcherError::SettleDeadlineNotReached,
        )],
    );
    context.mollusk.sysvars.clock.unix_timestamp += 1;
    check_batcher_instruction(
        &context,
        &settle,
        &[batcher_error(batcher::BatcherError::SettleDeadlinePassed)],
    );
    assert_eq!(
        read_batch(&context, keys.batch).status,
        batcher::BatchStatus::Dispatched
    );
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        0
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.join_mint().total_supply_store,
            token::total_supply_key()
        ),
        999_700
    );

    let result = check_batcher_instruction(&context, &cancel, &[Check::success()]);
    assert_eq!(check_fhe_cpis(&context, &result), 1);
    let batch = read_batch(&context, keys.batch);
    assert_eq!(batch.status, batcher::BatchStatus::Refunding);
    assert_eq!(batch.burned_total_handle, [0; 32]);
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        300
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.join_mint().total_supply_store,
            token::total_supply_key()
        ),
        1_000_000
    );
    if let Some(account) = context.account_store.borrow().get(&pending_burn) {
        assert_eq!(account.owner, system_program::ID);
        assert!(account.data.is_empty());
    }

    // Refunding is terminal for aggregation and settlement, but remains live for withdrawals.
    check_batcher_instruction(
        &context,
        &dispatch_ix(&fixture, &keys),
        &[batcher_error(batcher::BatcherError::BatchNotPending)],
    );
    check_batcher_instruction(
        &context,
        &cancel,
        &[batcher_error(batcher::BatcherError::BatchNotDispatched)],
    );
    check_batcher_instruction(
        &context,
        &settle,
        &[batcher_error(batcher::BatcherError::BatchNotDispatched)],
    );
    check_batcher_instruction(
        &context,
        &join_ix(
            &fixture,
            &keys,
            &fixture.bob,
            amount_attestation_for(
                handle_for_chain(42, BALANCE_FHE_TYPE),
                0,
                fixture.bob.user,
                token::id(),
            ),
        ),
        &[batcher_error(batcher::BatcherError::BatchNotPending)],
    );

    // Refunding is final for the authority's spending too: its funding goes back to the operator
    // before the refunds, which pay their own rent. The record still authorizes the quit, so it
    // cannot be closed.
    run_reclaim_batch_authority(&context, &fixture, &keys);
    check_batcher_instruction(
        &context,
        &close_join_record_ix(&keys, fixture.alice.user),
        &[batcher_error(batcher::BatcherError::JoinRecordStillLive)],
    );

    // Anyone may run a refunding batch's quit for the user; the refund and the record's rent still
    // go to the user, whose balance moves only by that rent.
    let mut quit = with_payer(quit_ix(&fixture, &keys, &fixture.alice), 1, fixture.payer);
    quit.accounts[0].is_signer = false;
    // Whoever runs it cannot redirect the refund to another account.
    let alice_join = fixture.user_join(&fixture.alice);
    let bob_join = fixture.user_join(&fixture.bob);
    let mut redirected = quit.clone();
    for meta in redirected.accounts.iter_mut() {
        if meta.pubkey == alice_join.token_account {
            meta.pubkey = bob_join.token_account;
        } else if meta.pubkey == alice_join.balance_store {
            meta.pubkey = bob_join.balance_store;
        }
    }
    check_batcher_instruction(
        &context,
        &redirected,
        &[batcher_error(batcher::BatcherError::DerivedAccountMismatch)],
    );
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        300
    );
    let join_record = keys.join_record(fixture.alice.user);
    let record_rent = lamports_of(&context, join_record);
    let user_lamports = lamports_of(&context, fixture.alice.user);
    let result = check_batcher_instruction(&context, &quit, &[Check::success()]);
    assert_eq!(check_fhe_cpis(&context, &result), 2);
    assert_eq!(lamports_of(&context, join_record), 0);
    assert_eq!(
        lamports_of(&context, fixture.alice.user),
        user_lamports + record_rent
    );
    // The refund is single-use: without its record, a second quit cannot run.
    check_batcher_instruction(
        &context,
        &quit,
        &[anchor_framework_error_check(
            anchor_lang::error::ErrorCode::AccountNotInitialized,
        )],
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.underlying.balance_store,
            token::balance_key()
        ),
        1_000
    );
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        0
    );
    assert_eq!(
        store_u64(
            &context,
            keys.pending_join_value(fixture.alice.user),
            batcher::JOINED_AMOUNT_KEY
        ),
        0
    );
}

/// A batch with no joins burns zero, and settle with the KMS-certified zero
/// cancels the batch: no vault deposit, no wrap, no rate, and the next batch
/// can open. The zero-total division-by-zero path is unreachable by
/// construction.
#[test]
fn mollusk_zero_total_batch_cancels_at_settle() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);

    let burned_handle = run_dispatch(&context, &fixture, &keys);
    assert_eq!(
        store_u64(
            &context,
            keys.burned_amount_store,
            token::burned_amount_key()
        ),
        0
    );
    run_settle(&context, &fixture, &keys, burned_handle, 0);

    let batch = read_batch(&context, keys.batch);
    assert_eq!(batch.status, batcher::BatchStatus::Canceled);
    assert_eq!(batch.payout_rate, 0);
    assert_eq!(read_spl_amount(&context, fixture.vault_token_account), 0);
    run_reclaim_batch_authority(&context, &fixture, &keys);

    // The next batch opens against the canceled one.
    let next = BatchKeys::new(&fixture, 1);
    ensure_open_batch_accounts(&context, &fixture, &next);
    check_batcher_instruction(
        &context,
        &open_batch_ix(&fixture, &next, Some(keys.batch)),
        &[Check::success()],
    );
    assert_eq!(read_batch(&context, next.batch).index, 1);
}

/// Settle redeems through `verify_public_decrypt`, which never pauses, and wraps a nonzero payout
/// through `fhe_execute`, so no other pause stops it. The revert is atomic, so the batch stays
/// dispatched and settles once execution resumes.
#[test]
fn mollusk_settle_stops_only_under_the_execution_pause() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));
    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        100,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys);

    let live = context.account_store.borrow()[&fixture.host_config].clone();
    let pause = |areas| {
        context
            .account_store
            .borrow_mut()
            .insert(fixture.host_config, paused_host_config(&live, areas));
    };
    pause(host::PauseFlags {
        execution: true,
        verified_inputs: false,
        acl_writes: false,
    });
    let (signatures, extra_data) = amount_public_decrypt_cert(burned_handle, 100);
    let pending_burn = keys.pending_burn(fixture.join_mint().mint);
    check_batcher_instruction(
        &context,
        &settle_ix(&fixture, &keys, 100, signatures, extra_data, pending_burn),
        &[anchor_error_check(
            host::errors::ZamaHostError::ExecutionPaused as u32,
        )],
    );
    assert_eq!(
        read_batch(&context, keys.batch).status,
        batcher::BatchStatus::Dispatched
    );

    pause(host::PauseFlags {
        execution: false,
        verified_inputs: true,
        acl_writes: true,
    });
    run_settle(&context, &fixture, &keys, burned_handle, 100);
    assert_eq!(
        read_batch(&context, keys.batch).status,
        batcher::BatchStatus::Settled
    );
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
    let context = fixture_context(mollusk(), redeem_accounts(&fixture, 1_000, 1_000, 1_000));
    fixture.seed_values(&context, (0, 600), (0, 400), (0, 1_000));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    assert_eq!(
        read_batch(&context, keys.batch).status,
        batcher::BatchStatus::Pending
    );

    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.bob,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        400,
    );

    // Encrypted accounting after the joins: confidential SHARE balances
    // debited, the batch account holds the encrypted share sum.
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.shares.balance_store,
            token::balance_key()
        ),
        300
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.bob.shares.balance_store,
            token::balance_key()
        ),
        0
    );
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        700
    );
    assert_eq!(read_batch(&context, keys.batch).join_count, 2);

    // Dispatch burns the batch's whole confidential-share balance.
    let burned_handle = run_dispatch(&context, &fixture, &keys);
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        0
    );
    assert_eq!(
        store_u64(
            &context,
            keys.burned_amount_store,
            token::burned_amount_key()
        ),
        700
    );

    // Settle: the KMS certifies 700 shares; the vault (1:1) pays 700
    // underlying; the underlying is wrapped into the batch's confidential
    // payout account.
    run_settle(&context, &fixture, &keys, burned_handle, 700);
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
    assert_eq!(
        store_u64(&context, keys.payout_balance_store, token::balance_key()),
        700
    );

    // Claims: each user receives their exact proportional encrypted underlying.
    run_claim(&context, &fixture, &keys, &fixture.alice);
    run_claim(&context, &fixture, &keys, &fixture.bob);
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.underlying.balance_store,
            token::balance_key()
        ),
        300
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.bob.underlying.balance_store,
            token::balance_key()
        ),
        400
    );
    assert_eq!(
        store_u64(&context, keys.payout_balance_store, token::balance_key()),
        0
    );
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
    let context = fixture_context(mollusk(), redeem_accounts(&fixture, 2_000, 1_000, 1_000));
    fixture.seed_values(&context, (0, 600), (0, 400), (0, 1_000));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);

    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.bob,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        400,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys);
    run_settle(&context, &fixture, &keys, burned_handle, 700);

    // assets = 700 * (2_000 + 1) / (1_000 + 1) = 1_399 (floor).
    let settled = read_batch(&context, keys.batch);
    assert_eq!(settled.payout_received, 1_399);

    run_claim(&context, &fixture, &keys, &fixture.alice);
    run_claim(&context, &fixture, &keys, &fixture.bob);
    // Exact proportional floors: 300 * 1_399 / 700 = 599, 400 * 1_399 / 700
    // = 799. Sum 1_398 <= 1_399; the one dust unit stays with the batch.
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.underlying.balance_store,
            token::balance_key()
        ),
        599
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.bob.underlying.balance_store,
            token::balance_key()
        ),
        799
    );
    assert_eq!(
        store_u64(&context, keys.payout_balance_store, token::balance_key()),
        1
    );
}

/// The redeem twin of the deposit repeat-join/quit/re-join test: repeated
/// SHARE joins accumulate in the joined encrypted store (the operand-aliases-output
/// same-slot update — the aliasing class this test exists to pin), quit
/// refunds the exact accumulated shares all-or-nothing and resets the encrypted store
/// to zero, and a re-join after quit accumulates from zero.
#[test]
fn mollusk_redeem_repeat_join_accumulates_and_quit_refunds_exactly() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Redeem);
    let context = fixture_context(mollusk(), redeem_accounts(&fixture, 1_000, 1_000, 1_000));
    fixture.seed_values(&context, (0, 600), (0, 400), (0, 1_000));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);

    // Two joins accumulate: the second join's execution reads the joined encrypted store
    // as an operand AND updates it as the output.
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        100,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        250,
    );
    let pending = keys.pending_join_value(fixture.alice.user);
    assert_eq!(
        store_u64(&context, pending, batcher::JOINED_AMOUNT_KEY),
        350
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.shares.balance_store,
            token::balance_key()
        ),
        250
    );

    // Quit refunds exactly 350 shares (all-or-nothing) and resets the encrypted store.
    let quit = quit_ix(&fixture, &keys, &fixture.alice);
    let result = check_batcher_instruction(&context, &quit, &[Check::success()]);
    assert_eq!(check_fhe_cpis(&context, &result), 2);
    assert_eq!(store_u64(&context, pending, batcher::JOINED_AMOUNT_KEY), 0);
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.shares.balance_store,
            token::balance_key()
        ),
        600
    );
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        0
    );

    // Re-join after quit accumulates from zero, not from the stale amount.
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(43, BALANCE_FHE_TYPE),
        40,
    );
    assert_eq!(store_u64(&context, pending, batcher::JOINED_AMOUNT_KEY), 40);
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        40
    );
}

/// A quit whose refund destination is not the user's own token account is refused before
/// anything moves; the join and the batch balance stay as they were.
#[test]
fn mollusk_quit_rejects_refund_destination_that_is_not_the_users_account() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        100,
    );
    let pending = keys.pending_join_value(fixture.alice.user);

    // Destination = source: the token transfer would move nothing and the reset would erase
    // the join, handing Alice's 100 to the other participants at settle.
    let mut quit = quit_ix(&fixture, &keys, &fixture.alice);
    let user_join = fixture.user_join(&fixture.alice);
    for meta in quit.accounts.iter_mut() {
        if meta.pubkey == user_join.token_account {
            meta.pubkey = keys.join_token_account;
        }
    }
    check_batcher_instruction(
        &context,
        &quit,
        &[batcher_error(batcher::BatcherError::DerivedAccountMismatch)],
    );
    assert_eq!(
        store_u64(&context, pending, batcher::JOINED_AMOUNT_KEY),
        100
    );
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        100
    );
}

/// A user who quits before dispatch has no join record left, so a claim for them after the batch
/// settles on the other participants is refused, and the others' claims pay the full batch.
/// Deposit direction only: quit and claim are direction-free shared code (settle's vault CPI is
/// the sole direction branch), so one direction pins the class.
#[test]
fn mollusk_claim_after_quit_is_refused() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);

    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.bob,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        500,
    );

    // Alice quits; the batch dispatches and settles on bob's 500 alone.
    let quit = quit_ix(&fixture, &keys, &fixture.alice);
    let result = check_batcher_instruction(&context, &quit, &[Check::success()]);
    assert_eq!(check_fhe_cpis(&context, &result), 2);
    let burned_handle = run_dispatch(&context, &fixture, &keys);
    assert_eq!(
        store_u64(
            &context,
            keys.burned_amount_store,
            token::burned_amount_key()
        ),
        500
    );
    run_settle(&context, &fixture, &keys, burned_handle, 500);

    // The quit closed Alice's record, so her claim has nothing to claim with.
    assert_eq!(
        lamports_of(&context, keys.join_record(fixture.alice.user)),
        0
    );
    ensure_system_accounts(
        &context,
        &[
            owner_ata(keys.batch_authority, fixture.payout_mint().underlying_mint),
            owner_ata(fixture.alice.user, fixture.payout_mint().underlying_mint),
        ],
    );
    check_batcher_instruction(
        &context,
        &claim_ix(&fixture, &keys, &fixture.alice),
        &[anchor_framework_error_check(
            anchor_lang::error::ErrorCode::AccountNotInitialized,
        )],
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.shares.balance_store,
            token::balance_key()
        ),
        0
    );

    // Bob's claim still pays the full settled batch.
    run_claim(&context, &fixture, &keys, &fixture.bob);
    assert_eq!(
        store_u64(
            &context,
            fixture.bob.shares.balance_store,
            token::balance_key()
        ),
        500
    );
    assert_eq!(
        store_u64(&context, keys.payout_balance_store, token::balance_key()),
        0
    );
}

/// A redeem batch with no joins cancels at settle exactly like a deposit
/// batch: the certified zero cancels trustlessly and the vault is untouched.
#[test]
fn mollusk_redeem_zero_total_batch_cancels_at_settle() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Redeem);
    let context = fixture_context(mollusk(), redeem_accounts(&fixture, 1_000, 1_000, 1_000));
    fixture.seed_values(&context, (0, 600), (0, 400), (0, 1_000));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);

    let burned_handle = run_dispatch(&context, &fixture, &keys);
    assert_eq!(
        store_u64(
            &context,
            keys.burned_amount_store,
            token::burned_amount_key()
        ),
        0
    );
    run_settle(&context, &fixture, &keys, burned_handle, 0);

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
    let context = fixture_context(mollusk(), redeem_accounts(&fixture, 2_000_000, 100, 100));
    fixture.seed_values(&context, (0, 60), (0, 40), (0, 100));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);

    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        1,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys);
    run_settle(&context, &fixture, &keys, burned_handle, 1);

    // assets = 1 * (2_000_000 + 1) / (100 + 1) = 19_801 (floor) — always
    // >= 1 per share at any reachable price, so no ZeroAssets, no stuck batch.
    let settled = read_batch(&context, keys.batch);
    assert_eq!(settled.status, batcher::BatchStatus::Settled);
    assert_eq!(settled.total_joined, 1);
    assert_eq!(settled.payout_received, 19_801);

    run_claim(&context, &fixture, &keys, &fixture.alice);
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.underlying.balance_store,
            token::balance_key()
        ),
        19_801
    );
}

/// SPL destinations cannot refuse incoming transfers, so an attacker can push
/// plain underlying into the PDA-owned `batch_payout_underlying` account of a
/// redeem batch before settlement. Settle must price and wrap only the
/// vault-paid delta across its withdraw phase — the redeem mirror of the
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
    let context = fixture_context(mollusk(), accounts);
    fixture.seed_values(&context, (0, 600), (0, 400), (0, 1_000));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.bob,
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
    check_batcher_instruction(&context, &preload_transfer, &[Check::success()]);
    assert_eq!(read_spl_amount(&context, keys.payout_underlying), PRELOAD);

    let burned_handle = run_dispatch(&context, &fixture, &keys);
    run_settle(&context, &fixture, &keys, burned_handle, 700);

    // Settle succeeded and the batch accounting reflects only the vault-paid
    // delta: 700 shares in, 700 underlying out at the 1:1 price. The preload
    // sits in the account, unwrapped, inert.
    let settled = read_batch(&context, keys.batch);
    assert_eq!(settled.status, batcher::BatchStatus::Settled);
    assert_eq!(settled.total_joined, 700);
    assert_eq!(settled.payout_received, 700);
    assert_eq!(read_spl_amount(&context, keys.payout_underlying), PRELOAD);
    assert_eq!(
        store_u64(&context, keys.payout_balance_store, token::balance_key()),
        700
    );

    run_claim(&context, &fixture, &keys, &fixture.alice);
    run_claim(&context, &fixture, &keys, &fixture.bob);
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.underlying.balance_store,
            token::balance_key()
        ),
        300
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.bob.underlying.balance_store,
            token::balance_key()
        ),
        400
    );
}

/// One deposit batcher and one redeem batcher run a FULL interleaved
/// lifecycle concurrently over the same vault, mints, and users — the
/// two-instance pattern. Cross-direction state confusion (a redeem batch
/// reading deposit-batch encrypted stores, the shared escrows or the vault mixing
/// phases) would surface here, not at open: both directions join, dispatch,
/// settle, and claim against the shared world, and every balance is checked.
#[test]
fn mollusk_deposit_and_redeem_batchers_run_concurrently() {
    let deposit = BatcherFixture::new(batcher::BatchDirection::Deposit);
    // Same physical world (mints, vault, users, payer); only the batcher
    // config account differs.
    let redeem = deposit.redeem_twin(Pubkey::new_unique());

    let mut accounts = deposit.accounts_with_escrows(1_000, 1_000, 1_000_000, 1_000);
    accounts.insert(redeem.batcher, system_account(0));
    let context = fixture_context(mollusk(), accounts);
    // Users hold both cUnderlying (to deposit) and cShares (to redeem).
    deposit.seed_values(&context, (1_000, 600), (2_000, 400), (1_000_000, 1_000));

    let deposit_keys = initialize_and_open_first_batch(&context, &deposit, 0);
    let redeem_keys = initialize_and_open_first_batch(&context, &redeem, 0);
    assert_ne!(deposit_keys.batch, redeem_keys.batch);

    // Interleaved joins: both batches are pending at once, and each user
    // participates in both directions.
    run_join(
        &context,
        &deposit,
        &deposit_keys,
        &deposit.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &redeem,
        &redeem_keys,
        &redeem.alice,
        handle_for_chain(43, BALANCE_FHE_TYPE),
        200,
    );
    run_join(
        &context,
        &deposit,
        &deposit_keys,
        &deposit.bob,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        500,
    );
    run_join(
        &context,
        &redeem,
        &redeem_keys,
        &redeem.bob,
        handle_for_chain(44, BALANCE_FHE_TYPE),
        300,
    );

    // Each batch account holds exactly its own direction's encrypted sum.
    assert_eq!(
        store_u64(
            &context,
            deposit_keys.join_balance_store,
            token::balance_key()
        ),
        800
    );
    assert_eq!(
        store_u64(
            &context,
            redeem_keys.join_balance_store,
            token::balance_key()
        ),
        500
    );

    // Interleaved dispatches: two independent burned handles on two mints.
    let deposit_burned = run_dispatch(&context, &deposit, &deposit_keys);
    let redeem_burned = run_dispatch(&context, &redeem, &redeem_keys);
    assert_ne!(deposit_burned, redeem_burned);

    // Settle the redeem batch first: 500 shares withdraw 500 underlying at
    // the 1:1 price, leaving the vault at (500, 500).
    run_settle(&context, &redeem, &redeem_keys, redeem_burned, 500);
    let redeem_settled = read_batch(&context, redeem_keys.batch);
    assert_eq!(redeem_settled.status, batcher::BatchStatus::Settled);
    assert_eq!(redeem_settled.total_joined, 500);
    assert_eq!(redeem_settled.payout_received, 500);
    assert_eq!(read_spl_amount(&context, deposit.vault_token_account), 500);

    // Then the deposit batch: 800 underlying at the (still 1:1) price mints
    // 800 shares; the vault ends at (1_300, 1_300).
    run_settle(&context, &deposit, &deposit_keys, deposit_burned, 800);
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
    run_claim(&context, &deposit, &deposit_keys, &deposit.alice);
    run_claim(&context, &redeem, &redeem_keys, &redeem.alice);
    run_claim(&context, &redeem, &redeem_keys, &redeem.bob);
    run_claim(&context, &deposit, &deposit_keys, &deposit.bob);

    // Final per-user balances: cShares = start - redeem join + deposit claim;
    // cUnderlying = start - deposit join + redeem claim.
    assert_eq!(
        store_u64(
            &context,
            deposit.alice.shares.balance_store,
            token::balance_key()
        ),
        600 - 200 + 300
    );
    assert_eq!(
        store_u64(
            &context,
            deposit.alice.underlying.balance_store,
            token::balance_key()
        ),
        1_000 - 300 + 200
    );
    assert_eq!(
        store_u64(
            &context,
            deposit.bob.shares.balance_store,
            token::balance_key()
        ),
        400 - 300 + 500
    );
    assert_eq!(
        store_u64(
            &context,
            deposit.bob.underlying.balance_store,
            token::balance_key()
        ),
        2_000 - 500 + 300
    );
    // Both batch payout accounts fully drained.
    assert_eq!(
        store_u64(
            &context,
            deposit_keys.payout_balance_store,
            token::balance_key()
        ),
        0
    );
    assert_eq!(
        store_u64(
            &context,
            redeem_keys.payout_balance_store,
            token::balance_key()
        ),
        0
    );
}

// ---------------------------------------------------------------------------
// Lifecycle-gate rejects
// ---------------------------------------------------------------------------

/// Dispatch before `min_batch_age_secs` of wall-clock time is rejected, and succeeds from then on.
/// Slots alone do not age a batch.
#[test]
fn mollusk_dispatch_waits_for_min_batch_age() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let mut context = fixture_context(mollusk(), fixture.accounts(0, 0));
    // Open well past zero, so an age counted from anything but the opening would already suffice.
    context.mollusk.sysvars.clock.unix_timestamp += 1_000_000;
    let keys = initialize_and_open_first_batch(&context, &fixture, 1_000);
    ensure_system_accounts(
        &context,
        &[
            keys.burned_amount_store,
            keys.pending_burn(fixture.join_mint().mint),
        ],
    );
    check_batcher_instruction(
        &context,
        &dispatch_ix(&fixture, &keys),
        &[batcher_error(batcher::BatcherError::BatchTooYoung)],
    );
    context.mollusk.sysvars.clock.slot += 10_000;
    context.mollusk.sysvars.clock.unix_timestamp += 999;
    check_batcher_instruction(
        &context,
        &dispatch_ix(&fixture, &keys),
        &[batcher_error(batcher::BatcherError::BatchTooYoung)],
    );
    context.mollusk.sysvars.clock.unix_timestamp += 1;
    check_batcher_instruction(&context, &dispatch_ix(&fixture, &keys), &[Check::success()]);
}

/// Leaving a pending batch is the user's choice: a quit the user does not sign rejects, even when
/// someone else pays for it.
#[test]
fn mollusk_pending_quit_requires_the_user_signature() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));
    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );

    let mut quit = with_payer(quit_ix(&fixture, &keys, &fixture.alice), 1, fixture.payer);
    quit.accounts[0].is_signer = false;
    check_batcher_instruction(
        &context,
        &quit,
        &[Check::err(ProgramError::Custom(
            anchor_lang::error::ErrorCode::AccountNotSigner as u32,
        ))],
    );
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        300
    );
}

/// The batch ages are bounded: at most seven days to dispatch, and a settle deadline of more than
/// zero and at most thirty days.
#[test]
fn mollusk_initialize_batcher_bounds_the_batch_ages() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    let with_ages = |min_batch_age_secs, settle_deadline_secs| {
        let mut ix = initialize_batcher_ix(&fixture, 0);
        ix.data = anchor_lang::InstructionData::data(&batcher::instruction::InitializeBatcher {
            min_batch_age_secs,
            settle_deadline_secs,
            direction: fixture.direction,
        });
        ix
    };
    for (ages, error) in [
        (
            (batcher::MAX_MIN_BATCH_AGE_SECS + 1, SETTLE_DEADLINE_SECS),
            batcher::BatcherError::InvalidMinBatchAge,
        ),
        ((0, 0), batcher::BatcherError::InvalidSettleDeadline),
        (
            (0, batcher::MAX_SETTLE_DEADLINE_SECS + 1),
            batcher::BatcherError::InvalidSettleDeadline,
        ),
    ] {
        check_batcher_instruction(
            &context,
            &with_ages(ages.0, ages.1),
            &[batcher_error(error)],
        );
    }
    check_batcher_instruction(
        &context,
        &with_ages(
            batcher::MAX_MIN_BATCH_AGE_SECS,
            batcher::MAX_SETTLE_DEADLINE_SECS,
        ),
        &[Check::success()],
    );
}

/// After dispatch, the batch is frozen for users: join and quit both reject,
/// and a second dispatch rejects. Claims reject until settle.
#[test]
fn mollusk_join_quit_and_claim_respect_batch_status() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );

    // Claim before settle rejects.
    ensure_system_accounts(
        &context,
        &[
            keys.claim_amount_store(fixture.alice.user),
            keys.payout_transferred_value,
            owner_ata(keys.batch_authority, fixture.payout_mint().underlying_mint),
            owner_ata(fixture.alice.user, fixture.payout_mint().underlying_mint),
        ],
    );
    check_batcher_instruction(
        &context,
        &claim_ix(&fixture, &keys, &fixture.alice),
        &[batcher_error(batcher::BatcherError::BatchNotSettled)],
    );

    let _burned = run_dispatch(&context, &fixture, &keys);

    // Join after dispatch rejects.
    let attestation = amount_attestation_for(
        handle_for_chain(42, BALANCE_FHE_TYPE),
        0,
        fixture.alice.user,
        token::id(),
    );
    check_batcher_instruction(
        &context,
        &join_ix(&fixture, &keys, &fixture.alice, attestation),
        &[batcher_error(batcher::BatcherError::BatchNotPending)],
    );
    // Quit after dispatch rejects — the exit is the claim, pro rata. Before settle, the only exit is
    // the settle-deadline cancellation and its refunds (DD-045).
    check_batcher_instruction(
        &context,
        &quit_ix(&fixture, &keys, &fixture.alice),
        &[batcher_error(batcher::BatcherError::BatchNotRefundable)],
    );
    // Second dispatch rejects.
    check_batcher_instruction(
        &context,
        &dispatch_ix(&fixture, &keys),
        &[batcher_error(batcher::BatcherError::BatchNotPending)],
    );
}

/// A settled batch pays each record once: the second claim rejects on the
/// record's claimed flag.
#[test]
fn mollusk_double_claim_rejects() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys);
    run_settle(&context, &fixture, &keys, burned_handle, 300);
    run_claim(&context, &fixture, &keys, &fixture.alice);

    check_batcher_instruction(
        &context,
        &claim_ix(&fixture, &keys, &fixture.alice),
        &[batcher_error(batcher::BatcherError::AlreadyClaimed)],
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.shares.balance_store,
            token::balance_key()
        ),
        300
    );
}

/// Batches never overlap while pending: a second open against a pending
/// previous batch rejects, and opening without the previous batch account
/// rejects.
#[test]
fn mollusk_open_batch_requires_previous_batch_not_pending() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    let keys = initialize_and_open_first_batch(&context, &fixture, 0);

    let next = BatchKeys::new(&fixture, 1);
    ensure_open_batch_accounts(&context, &fixture, &next);
    check_batcher_instruction(
        &context,
        &open_batch_ix(&fixture, &next, Some(keys.batch)),
        &[batcher_error(
            batcher::BatcherError::PreviousBatchStillPending,
        )],
    );
    check_batcher_instruction(
        &context,
        &open_batch_ix(&fixture, &next, None),
        &[batcher_error(batcher::BatcherError::PreviousBatchMismatch)],
    );
}

/// The next batch opens once the previous one is dispatched, as on EVM, where dispatching opens it: a
/// join during the KMS round lands in the next batch, and the dispatched batch still settles and
/// pays its claim.
#[test]
fn mollusk_open_batch_accepts_dispatched_previous_batch() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));
    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys);

    let next = BatchKeys::new(&fixture, 1);
    ensure_open_batch_accounts(&context, &fixture, &next);
    check_batcher_instruction(
        &context,
        &open_batch_ix(&fixture, &next, Some(keys.batch)),
        &[Check::success()],
    );
    assert_eq!(
        read_batch(&context, keys.batch).status,
        batcher::BatchStatus::Dispatched
    );
    run_join(
        &context,
        &fixture,
        &next,
        &fixture.bob,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        500,
    );

    run_settle(&context, &fixture, &keys, burned_handle, 300);
    assert_eq!(
        read_batch(&context, keys.batch).status,
        batcher::BatchStatus::Settled
    );
    run_claim(&context, &fixture, &keys, &fixture.alice);
    let next_batch = read_batch(&context, next.batch);
    assert_eq!(next_batch.status, batcher::BatchStatus::Pending);
    assert_eq!(next_batch.join_count, 1);
}

/// A token account's owner does not sign its creation, and the next batch authority derives from the
/// public `next_batch_index`, so anyone can create the next batch's token accounts first. The open
/// still succeeds and keeps their zero balances.
#[test]
fn mollusk_open_batch_accepts_precreated_batch_token_accounts() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    check_batcher_instruction(
        &context,
        &initialize_batcher_ix(&fixture, 0),
        &[Check::success()],
    );
    let keys = BatchKeys::new(&fixture, 0);
    ensure_open_batch_accounts(&context, &fixture, &keys);
    let stranger = Pubkey::new_unique();
    context
        .account_store
        .borrow_mut()
        .insert(stranger, system_account(5_000_000_000));
    let precreated = [
        (
            fixture.join_app(),
            keys.join_token_account,
            keys.join_balance_store,
        ),
        (
            fixture.payout_app(),
            keys.payout_token_account,
            keys.payout_balance_store,
        ),
    ];
    for (app, token_account, balance_store) in precreated {
        let ix = anchor_ix(
            token::id(),
            token::accounts::InitializeTokenAccount {
                payer: stranger,
                owner: keys.batch_authority,
                mint: app.scope,
                token_account,
                balance_encrypted_store: balance_store,
                zama_event_authority: event_authority(host::id()),
                transient_store: host::transient_store_address(stranger).0,
                instructions: Instructions::id(),
                zama_program: host::id(),
                host_config: fixture.host_config,
                system_program: system_program::ID,
                hcu_block_meter: fixture.hcu_block_meter(app),
                hcu_trusted_app_record: fixture.hcu_trusted_app_record(app),
                event_authority: event_authority(token::id()),
                program: token::id(),
            },
            token::instruction::InitializeTokenAccount {},
        );
        zama_solana_test_kit::transaction::process_fhe_instruction(
            &context,
            stranger,
            &fixture.with_deny_records(ix, &[&[app]]),
            &[Check::success()],
        );
    }
    let handles =
        precreated.map(|(_, _, store)| read_store_handle(&context, store, token::balance_key()));

    check_batcher_instruction(
        &context,
        &open_batch_ix(&fixture, &keys, None),
        &[Check::success()],
    );

    assert_eq!(
        read_account::<batcher::Batcher>(&context, fixture.batcher).next_batch_index,
        1
    );
    for ((_, _, store), handle) in precreated.into_iter().zip(handles) {
        assert_eq!(
            read_store_handle(&context, store, token::balance_key()),
            handle
        );
        assert_eq!(store_u64(&context, store, token::balance_key()), 0);
    }
}

#[test]
fn mollusk_open_batch_rejects_wrong_next_index() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    let wrong = BatchKeys::new(&fixture, 2);
    ensure_open_batch_accounts(&context, &fixture, &wrong);
    check_batcher_instruction(
        &context,
        &open_batch_ix(&fixture, &wrong, Some(keys.batch)),
        &[batcher_error(batcher::BatcherError::BatchIndexMismatch)],
    );
    assert_eq!(
        read_account::<batcher::Batcher>(&context, fixture.batcher).next_batch_index,
        1
    );
}

/// Initializing a batcher with the direction's mint wiring swapped rejects:
/// a redeem batcher whose join mint wraps the vault underlying (instead of
/// its shares) is refused at setup.
#[test]
fn mollusk_initialize_batcher_rejects_swapped_direction_wiring() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Redeem);
    let context = fixture_context(mollusk(), redeem_accounts(&fixture, 0, 0, 0));
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
            min_batch_age_secs: 0,
            settle_deadline_secs: SETTLE_DEADLINE_SECS,
            direction: batcher::BatchDirection::Redeem,
        },
    );
    check_batcher_instruction(
        &context,
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
// fixed-key cost lifecycle (see `snapshot_lifecycle`). Exact snapshots detect
// ordinary drift; these bounds also keep unusually expensive runs below a
// documented ceiling. The sequential pending-burn PDA is keyed only by mint
// and token account, so its bump search is stable for the fixed fixture keys.
const SETTLE_DEPOSIT_MAX_COMPUTE_UNITS: u64 = 360_000;
const SETTLE_REDEEM_MAX_COMPUTE_UNITS: u64 = 430_000;

/// One fixed-key run through open/join/dispatch/settle/claim and the exits (quit, cancel_dispatch),
/// snapshotting each instruction's cost profile under `prefix`. Fixed fixture keys keep the PDA
/// bump searches — part of the measured compute — stable across runs.
fn snapshot_lifecycle(fixture: &BatcherFixture, context: &mut Ctx, prefix: &str) {
    check_batcher_instruction(
        context,
        &initialize_batcher_ix(fixture, 0),
        &[Check::success()],
    );
    let keys = BatchKeys::new(fixture, 0);
    ensure_open_batch_accounts(context, fixture, &keys);
    let open = open_batch_ix(fixture, &keys, None);
    let open_result = check_batcher_instruction(context, &open, &[Check::success()]);
    assert_batcher_cost(&format!("{prefix}open_batch"), &open, &open_result);

    let amount_handle = handle_for_chain(0x71, BALANCE_FHE_TYPE);
    ensure_system_accounts(
        context,
        &[
            keys.join_record(fixture.alice.user),
            fixture.user_join(&fixture.alice).transferred_value,
            keys.pending_join_value(fixture.alice.user),
            owner_ata(fixture.alice.user, fixture.join_mint().underlying_mint),
            owner_ata(keys.batch_authority, fixture.join_mint().underlying_mint),
        ],
    );
    let join = join_ix(
        fixture,
        &keys,
        &fixture.alice,
        production_amount_attestation_for(amount_handle, fixture.alice.user, token::id()),
    );
    let join_result = check_batcher_instruction(context, &join.clone(), &[Check::success()]);
    check_fhe_cpis(context, &join_result);
    assert_batcher_cost(&format!("{prefix}join"), &join, &join_result);

    ensure_system_accounts(
        context,
        &[
            keys.burned_amount_store,
            keys.pending_burn(fixture.join_mint().mint),
            owner_ata(keys.batch_authority, fixture.join_mint().underlying_mint),
        ],
    );
    let dispatch = dispatch_ix(fixture, &keys);
    let dispatch_result = check_batcher_instruction(context, &dispatch, &[Check::success()]);
    check_fhe_cpis(context, &dispatch_result);
    assert_batcher_cost(&format!("{prefix}dispatch"), &dispatch, &dispatch_result);

    let burned_handle = read_batch(context, keys.batch).burned_total_handle;
    let (settle, settle_result) = run_settle(context, fixture, &keys, burned_handle, 300);
    assert_batcher_cost(&format!("{prefix}settle"), &settle, &settle_result);
    let settle_compute_units = settle_result.compute_units_consumed;
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
            keys.claim_amount_store(fixture.alice.user),
            keys.payout_transferred_value,
            owner_ata(keys.batch_authority, fixture.payout_mint().underlying_mint),
            owner_ata(fixture.alice.user, fixture.payout_mint().underlying_mint),
        ],
    );
    let claim = claim_ix(fixture, &keys, &fixture.alice);
    let claim_result = check_batcher_instruction(context, &claim.clone(), &[Check::success()]);
    check_fhe_cpis(context, &claim_result);
    assert_batcher_cost(&format!("{prefix}claim"), &claim, &claim_result);

    let reclaim = reclaim_batch_authority_ix(fixture, &keys, fixture.payer);
    let reclaim_result = check_batcher_instruction(context, &reclaim, &[Check::success()]);
    assert_batcher_cost(
        &format!("{prefix}reclaim_batch_authority"),
        &reclaim,
        &reclaim_result,
    );

    let close = close_join_record_ix(&keys, fixture.alice.user);
    let close_result = check_batcher_instruction(context, &close, &[Check::success()]);
    assert_batcher_cost(&format!("{prefix}close_join_record"), &close, &close_result);

    // The exits run on the next batch, so the profiles above keep their measurements: bob quits it
    // while pending, a keeper cancels its dispatch at the settle deadline, and alice quits it while
    // refunding.
    let next = BatchKeys::new(fixture, 1);
    ensure_open_batch_accounts(context, fixture, &next);
    check_batcher_instruction(
        context,
        &open_batch_ix(fixture, &next, Some(keys.batch)),
        &[Check::success()],
    );
    run_join(
        context,
        fixture,
        &next,
        &fixture.alice,
        handle_for_chain(0x72, BALANCE_FHE_TYPE),
        100,
    );
    run_join(
        context,
        fixture,
        &next,
        &fixture.bob,
        handle_for_chain(0x73, BALANCE_FHE_TYPE),
        200,
    );

    let quit = quit_ix(fixture, &next, &fixture.bob);
    let quit_result = check_batcher_instruction(context, &quit, &[Check::success()]);
    check_fhe_cpis(context, &quit_result);
    assert_batcher_cost(&format!("{prefix}quit"), &quit, &quit_result);

    run_dispatch(context, fixture, &next);
    reach_settle_deadline(context);
    let cancel = cancel_dispatch_ix(fixture, &next);
    let cancel_result = check_batcher_instruction(context, &cancel, &[Check::success()]);
    check_fhe_cpis(context, &cancel_result);
    assert_batcher_cost(&format!("{prefix}cancel_dispatch"), &cancel, &cancel_result);

    let refund = quit_ix(fixture, &next, &fixture.alice);
    let refund_result = check_batcher_instruction(context, &refund, &[Check::success()]);
    check_fhe_cpis(context, &refund_result);
    assert_batcher_cost(&format!("{prefix}quit_refunding"), &refund, &refund_result);
}

#[test]
fn cost_snapshot_batcher_lifecycle() {
    let fixture = BatcherFixture::fixed(batcher::BatchDirection::Deposit, 0x61);
    let mut context = production_mollusk().with_context(fixture.accounts(0, 0));
    snapshot_lifecycle(&fixture, &mut context, "");
}

#[test]
fn cost_snapshot_batcher_redeem_lifecycle() {
    let fixture = BatcherFixture::fixed(batcher::BatchDirection::Redeem, 0x51);
    let mut context =
        production_mollusk().with_context(redeem_accounts(&fixture, 1_000, 1_000, 1_000));
    snapshot_lifecycle(&fixture, &mut context, "redeem_");
}

// ---------------------------------------------------------------------------
// Host levers: the deny list and the per-application block cap
// ---------------------------------------------------------------------------

/// Runs every batcher flow on the production host under `levers`, each instruction carrying the
/// witnesses an honest client supplies. Batch 0 takes two joins and a quit, then dispatches,
/// settles and pays a claim. Batch 1 takes a join, dispatches, is canceled and refunds it through
/// quit.
fn run_every_flow(levers: HostLevers) -> (BatcherFixture, Ctx) {
    let fixture = BatcherFixture {
        levers,
        ..BatcherFixture::new(batcher::BatchDirection::Deposit)
    };
    let mut context = production_mollusk().with_context(fixture.accounts(0, 0));
    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.bob,
        handle_for_chain(42, BALANCE_FHE_TYPE),
        500,
    );
    run_quit(&context, &fixture, &keys, &fixture.bob);
    let burned_handle = run_dispatch(&context, &fixture, &keys);
    run_settle(&context, &fixture, &keys, burned_handle, 300);
    run_claim(&context, &fixture, &keys, &fixture.alice);
    assert!(read_join_record(&context, keys.join_record(fixture.alice.user)).claimed);

    let next = BatchKeys::new(&fixture, 1);
    ensure_open_batch_accounts(&context, &fixture, &next);
    check_batcher_instruction(
        &context,
        &open_batch_ix(&fixture, &next, Some(keys.batch)),
        &[Check::success()],
    );
    run_join(
        &context,
        &fixture,
        &next,
        &fixture.alice,
        handle_for_chain(43, BALANCE_FHE_TYPE),
        200,
    );
    run_dispatch(&context, &fixture, &next);
    reach_settle_deadline(&mut context);
    let result = check_batcher_instruction(
        &context,
        &cancel_dispatch_ix(&fixture, &next),
        &[Check::success()],
    );
    assert_eq!(check_fhe_cpis(&context, &result), 1);
    run_quit(&context, &fixture, &next, &fixture.alice);
    (fixture, context)
}

fn read_hcu_block_meter(context: &Ctx, app: host::AppScope) -> Option<host::HcuBlockMeter> {
    let store = context.account_store.borrow();
    let account = store.get(&host::hcu_block_meter_address(app).0)?;
    (account.owner == host::id()).then(|| {
        host::HcuBlockMeter::try_deserialize(&mut account.data.as_slice())
            .expect("block meter deserializes")
    })
}

/// Denying one application never stops another: with the deny list on and an unrelated
/// application denied, every batcher flow succeeds, quit and claim included.
#[test]
fn mollusk_every_flow_runs_with_the_deny_list_on() {
    run_every_flow(HostLevers {
        deny_list: true,
        ..HostLevers::default()
    });
}

/// Under a binding block cap, trusted applications bypass their meters.
#[test]
fn mollusk_every_flow_runs_under_a_binding_block_cap_when_trusted() {
    let (fixture, context) = run_every_flow(HostLevers {
        block_cap: BlockCap::Trusted,
        ..HostLevers::default()
    });
    for app in fixture.apps() {
        assert!(read_hcu_block_meter(&context, app).is_none());
    }
}

/// Under a binding block cap, each metered application pays into its own meter.
#[test]
fn mollusk_every_flow_runs_under_a_binding_block_cap_when_metered() {
    let (fixture, context) = run_every_flow(HostLevers {
        block_cap: BlockCap::Metered,
        ..HostLevers::default()
    });
    for app in fixture.apps() {
        let meter = read_hcu_block_meter(&context, app).expect("the execution created the meter");
        assert_eq!((meter.program, meter.scope), (app.program, app.scope));
        assert!(meter.used_hcu > 0);
    }
}

/// Checks that `exit`, an exit whose own execution runs as the batch, fails with the error of the
/// layer that checks a missing or wrong witness, and that denying the batch itself still stops it.
/// It leaves the batch allowed again.
fn assert_exit_rejects_missing_or_wrong_witnesses(
    context: &Ctx,
    fixture: &BatcherFixture,
    keys: &BatchKeys,
    exit: &Instruction,
) {
    let replace = |from: Pubkey, to: AccountMeta| {
        let mut ix = exit.clone();
        let meta = ix
            .accounts
            .iter_mut()
            .rev()
            .find(|meta| meta.pubkey == from)
            .expect("the exit carries the witness");
        *meta = to;
        ix
    };

    let mut missing_deny_record = exit.clone();
    missing_deny_record.accounts.pop();
    check_batcher_instruction(
        context,
        &missing_deny_record,
        &[batcher_error(batcher::BatcherError::DenyRecordsMismatch)],
    );
    let wrong_deny_record = replace(
        host::deny_scope_address(keys.app()).0,
        readonly(host::deny_scope_address(fixture.join_app()).0),
    );
    check_batcher_instruction(
        context,
        &wrong_deny_record,
        &[host_error(host::errors::ZamaHostError::DenyRecordMissing)],
    );
    // Anchor encodes an absent optional account as the program id.
    let batch_meter = host::hcu_block_meter_address(keys.app()).0;
    let missing_meter = replace(batch_meter, readonly(batcher::id()));
    check_batcher_instruction(
        context,
        &missing_meter,
        &[host_error(
            host::errors::ZamaHostError::HcuBlockMeterMissing,
        )],
    );
    let wrong_meter = replace(
        batch_meter,
        AccountMeta::new(host::hcu_block_meter_address(fixture.join_app()).0, false),
    );
    check_batcher_instruction(
        context,
        &wrong_meter,
        &[host_error(
            host::errors::ZamaHostError::HcuBlockMeterMismatch,
        )],
    );

    let (record, denied) = deny_scope_record_account(keys.app(), true);
    context.account_store.borrow_mut().insert(record, denied);
    check_batcher_instruction(
        context,
        exit,
        &[host_error(host::errors::ZamaHostError::ScopeDenied)],
    );
    let (record, allowed) = deny_scope_record_account(keys.app(), false);
    context.account_store.borrow_mut().insert(record, allowed);
}

/// A batch with Alice joined, under the deny list and a metered binding cap.
fn witnessed_batch_with_alice() -> (BatcherFixture, Ctx, BatchKeys) {
    let fixture = BatcherFixture {
        levers: HostLevers {
            deny_list: true,
            block_cap: BlockCap::Metered,
        },
        ..BatcherFixture::new(batcher::BatchDirection::Deposit)
    };
    let context = production_mollusk().with_context(fixture.accounts(0, 0));
    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    (fixture, context, keys)
}

/// A zero-total settle cancels before the wrap, so under the deny list it takes no deny records.
#[test]
fn mollusk_zero_total_settle_takes_no_deny_records() {
    let fixture = BatcherFixture {
        levers: HostLevers {
            deny_list: true,
            ..HostLevers::default()
        },
        ..BatcherFixture::new(batcher::BatchDirection::Deposit)
    };
    let context = production_mollusk().with_context(fixture.accounts(0, 0));
    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    let burned_handle = run_dispatch(&context, &fixture, &keys);

    let (signatures, extra_data) = amount_public_decrypt_cert(burned_handle, 0);
    let pending_burn = keys.pending_burn(fixture.join_mint().mint);
    let mut extra_record = settle_ix(&fixture, &keys, 0, signatures, extra_data, pending_burn);
    extra_record
        .accounts
        .push(readonly(host::deny_scope_address(fixture.payout_app()).0));
    check_batcher_instruction(
        &context,
        &extra_record,
        &[batcher_error(batcher::BatcherError::DenyRecordsMismatch)],
    );

    run_settle(&context, &fixture, &keys, burned_handle, 0);
    assert_eq!(
        read_batch(&context, keys.batch).status,
        batcher::BatchStatus::Canceled
    );
}

#[test]
fn mollusk_quit_rejects_missing_or_wrong_witnesses() {
    let (fixture, context, keys) = witnessed_batch_with_alice();
    let quit = quit_ix(&fixture, &keys, &fixture.alice);
    assert_exit_rejects_missing_or_wrong_witnesses(&context, &fixture, &keys, &quit);
    run_quit(&context, &fixture, &keys, &fixture.alice);
}

#[test]
fn mollusk_claim_rejects_missing_or_wrong_witnesses() {
    let (fixture, context, keys) = witnessed_batch_with_alice();
    let burned_handle = run_dispatch(&context, &fixture, &keys);
    run_settle(&context, &fixture, &keys, burned_handle, 300);
    ensure_system_accounts(
        &context,
        &[
            owner_ata(keys.batch_authority, fixture.payout_mint().underlying_mint),
            owner_ata(fixture.alice.user, fixture.payout_mint().underlying_mint),
        ],
    );
    let claim = claim_ix(&fixture, &keys, &fixture.alice);
    assert_exit_rejects_missing_or_wrong_witnesses(&context, &fixture, &keys, &claim);
    run_claim(&context, &fixture, &keys, &fixture.alice);
}

// ---------------------------------------------------------------------------
// Dust settlement refunds.
// ---------------------------------------------------------------------------

/// A deposit below one share's worth would mint zero shares, so settle wraps the redeemed total
/// back and opens refunds instead of reverting; the participant gets their exact contribution back.
#[test]
fn mollusk_dust_total_settle_opens_exact_refunds() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    // Share price ~20_000 underlying per share (e.g. after an adversarial
    // harvest donation): 2_000_000 assets backing 100 shares.
    let context = fixture_context(mollusk(), fixture.accounts(2_000_000, 100));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);

    // Alice's 100 is dust at this price: 100 * (100 + 1) / (2_000_000 + 1)
    // floors to zero shares.
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        100,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys);
    let join_vault_before = read_spl_amount(&context, fixture.join_mint().vault_underlying);

    let (signatures, extra_data) = amount_public_decrypt_cert(burned_handle, 100);
    let pending_burn = keys.pending_burn(fixture.join_mint().mint);
    let settle = settle_ix(&fixture, &keys, 100, signatures, extra_data, pending_burn);
    let result = check_batcher_instruction(&context, &settle, &[Check::success()]);
    check_fhe_cpis(&context, &result);

    // The redeemed total went straight back into the join mint's vault, the batch's confidential
    // join balance holds it again, and the demo vault was never touched.
    let batch = read_batch(&context, keys.batch);
    assert_eq!(batch.status, batcher::BatchStatus::Refunding);
    assert_eq!(batch.burned_total_handle, [0; 32]);
    assert_eq!(read_spl_amount(&context, keys.join_underlying), 0);
    assert_eq!(
        read_spl_amount(&context, fixture.join_mint().vault_underlying),
        join_vault_before
    );
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        100
    );
    assert_eq!(
        read_spl_amount(&context, fixture.vault_token_account),
        2_000_000
    );
    assert_eq!(read_spl_amount(&context, keys.payout_underlying), 0);

    // A refunding batch can be neither settled nor cancelled again.
    check_batcher_instruction(
        &context,
        &settle,
        &[batcher_error(batcher::BatcherError::BatchNotDispatched)],
    );

    let result = check_batcher_instruction(
        &context,
        &quit_ix(&fixture, &keys, &fixture.alice),
        &[Check::success()],
    );
    check_fhe_cpis(&context, &result);
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.underlying.balance_store,
            token::balance_key(),
        ),
        1_000
    );
}

/// Settle predicts only `ZeroShares`: any other vault failure still reverts it, and the batch is
/// recovered by the deadline cancel and its refunds.
#[test]
fn mollusk_settle_overflow_waits_for_the_deadline_cancel() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    // An empty vault with the largest share supply: any deposit's share count overflows u64.
    let mut context = fixture_context(mollusk(), fixture.accounts(0, u64::MAX));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));
    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        100,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys);

    let (signatures, extra_data) = amount_public_decrypt_cert(burned_handle, 100);
    let pending_burn = keys.pending_burn(fixture.join_mint().mint);
    let settle = settle_ix(&fixture, &keys, 100, signatures, extra_data, pending_burn);
    check_batcher_instruction(
        &context,
        &settle,
        &[Check::err(ProgramError::Custom(
            anchor_lang::error::ERROR_CODE_OFFSET + vault::DemoVaultError::MathOverflow as u32,
        ))],
    );
    assert_eq!(
        read_batch(&context, keys.batch).status,
        batcher::BatchStatus::Dispatched
    );

    reach_settle_deadline(&mut context);
    let result = check_batcher_instruction(
        &context,
        &cancel_dispatch_ix(&fixture, &keys),
        &[Check::success()],
    );
    check_fhe_cpis(&context, &result);
    run_quit(&context, &fixture, &keys, &fixture.alice);
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.underlying.balance_store,
            token::balance_key(),
        ),
        1_000
    );
}

/// Settle predicts zero shares from the vault token account's balance, so it accepts only the
/// vault's own account: a planted account holding a huge balance cannot force a batch into refunds.
#[test]
fn mollusk_settle_rejects_a_planted_vault_token_account() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));
    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        100,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys);
    let planted = Pubkey::new_unique();
    context.account_store.borrow_mut().insert(
        planted,
        spl_token_account(fixture.underlying_mint, fixture.vault_authority, u64::MAX),
    );

    let (signatures, extra_data) = amount_public_decrypt_cert(burned_handle, 100);
    let pending_burn = keys.pending_burn(fixture.join_mint().mint);
    let settle = settle_ix(&fixture, &keys, 100, signatures, extra_data, pending_burn);
    let mut planted_settle = settle.clone();
    for meta in &mut planted_settle.accounts {
        if meta.pubkey == fixture.vault_token_account {
            meta.pubkey = planted;
        }
    }
    check_batcher_instruction(
        &context,
        &planted_settle,
        &[Check::err(ProgramError::Custom(
            anchor_lang::error::ERROR_CODE_OFFSET
                + vault::DemoVaultError::VaultTokenAccountMismatch as u32,
        ))],
    );

    check_batcher_instruction(&context, &settle, &[Check::success()]);
    assert_eq!(
        read_batch(&context, keys.batch).status,
        batcher::BatchStatus::Settled
    );
}

// ---------------------------------------------------------------------------
// Settle at the largest KMS certificate
// ---------------------------------------------------------------------------

/// Settle with a certificate at the host's largest KMS threshold, every witness present, stays
/// within the compute a transaction may request. The client sets each transaction's compute limit
/// from a simulation of it, so this cost is what that limit must cover. The transaction size at this
/// threshold is checked with Kit's version 1 encoder in `solana/demo-dapp/src/vault/settleBatch.test.ts`.
#[test]
fn mollusk_settle_at_the_largest_kms_certificate_fits_the_compute_budget() {
    let fixture = BatcherFixture {
        levers: HostLevers {
            deny_list: true,
            block_cap: BlockCap::Metered,
        },
        ..BatcherFixture::new(batcher::BatchDirection::Deposit)
    };
    let kms_keys: Vec<_> = (0..host::constants::MAX_KMS_SIGNERS)
        .map(|i| kms_signing_key_n(0x60 + i))
        .collect();
    let mut accounts = fixture.accounts(0, 0);
    accounts.insert(
        fixture.kms_context,
        kms_context_account(
            KMS_CONTEXT_ID,
            kms_keys.iter().map(secp_evm_address).collect(),
            host::constants::MAX_KMS_SIGNERS,
        )
        .1,
    );
    let context = production_mollusk().with_context(accounts);
    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    let burned_handle = run_dispatch(&context, &fixture, &keys);

    let (signatures, extra_data) =
        amount_public_decrypt_cert_signed_by(burned_handle, 300, &kms_keys);
    let ix = settle_ix(
        &fixture,
        &keys,
        300,
        signatures,
        extra_data,
        keys.pending_burn(fixture.join_mint().mint),
    );
    let result = check_batcher_instruction(&context, &ix, &[Check::success()]);
    println!(
        "settle at {} KMS signatures: {} CU",
        host::constants::MAX_KMS_SIGNERS,
        result.compute_units_consumed
    );
    // The most compute a transaction may request (solana-compute-budget MAX_COMPUTE_UNIT_LIMIT).
    assert!(result.compute_units_consumed <= 1_400_000);
}

// ---------------------------------------------------------------------------
// Preloaded shares must not poison the batch (settle prices the delta)
// ---------------------------------------------------------------------------

/// SPL destinations cannot refuse incoming transfers, so an attacker can push
/// vault shares into the PDA-owned `batch_payout_underlying` account of a
/// deposit batch before settlement. Settle must price and wrap only the
/// vault-minted delta across its deposit phase: with balance-based accounting,
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
    let context = fixture_context(mollusk(), accounts);
    fixture.seed_values(&context, (1_000, 0), (2_000, 0), (1_000_000, 0));

    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.bob,
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
    check_batcher_instruction(&context, &attacker_deposit, &[Check::success()]);
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
    check_batcher_instruction(&context, &preload_transfer, &[Check::success()]);
    assert_eq!(read_spl_amount(&context, keys.payout_underlying), PRELOAD);

    let burned_handle = run_dispatch(&context, &fixture, &keys);
    run_settle(&context, &fixture, &keys, burned_handle, 800);

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
    assert_eq!(
        store_u64(&context, keys.payout_balance_store, token::balance_key()),
        800
    );

    // Claims pay out exactly as in the clean lifecycle.
    run_claim(&context, &fixture, &keys, &fixture.alice);
    run_claim(&context, &fixture, &keys, &fixture.bob);
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.shares.balance_store,
            token::balance_key()
        ),
        300
    );
    assert_eq!(
        store_u64(
            &context,
            fixture.bob.shares.balance_store,
            token::balance_key()
        ),
        500
    );
    assert_eq!(
        store_u64(&context, keys.payout_balance_store, token::balance_key()),
        0
    );
}

/// Gifts to a batch's token accounts have no join record. A gift to the join account is burned with
/// the joins and raises the settled total; one to the payout account arrives after settle. Either
/// way each joiner claims exactly its share, and the gifts' part stays in the batch payout account.
#[test]
fn mollusk_donations_to_batch_accounts_leave_claims_exact() {
    let fixture = BatcherFixture::new(batcher::BatchDirection::Deposit);
    let context = fixture_context(mollusk(), fixture.accounts(0, 0));
    fixture.seed_values(&context, (1_000, 0), (2_000, 50), (1_000_000, 50));
    let keys = initialize_and_open_first_batch(&context, &fixture, 0);
    run_join(
        &context,
        &fixture,
        &keys,
        &fixture.alice,
        handle_for_chain(41, BALANCE_FHE_TYPE),
        300,
    );

    let donate = |mint: &ConfidentialMintKeys, handle: u8, amount: u64| {
        let ix = donate_ix(
            &fixture,
            &fixture.bob,
            mint,
            keys.batch_authority,
            handle_for_chain(handle, BALANCE_FHE_TYPE),
            amount,
        );
        zama_solana_test_kit::transaction::process_fhe_instruction(
            &context,
            fixture.bob.user,
            &ix,
            &[Check::success()],
        );
    };
    donate(fixture.join_mint(), 42, 100);
    assert_eq!(
        store_u64(&context, keys.join_balance_store, token::balance_key()),
        400
    );

    let burned_handle = run_dispatch(&context, &fixture, &keys);
    run_settle(&context, &fixture, &keys, burned_handle, 400);
    assert_eq!(read_batch(&context, keys.batch).total_joined, 400);
    donate(fixture.payout_mint(), 43, 50);
    assert_eq!(
        store_u64(&context, keys.payout_balance_store, token::balance_key()),
        450
    );

    run_claim(&context, &fixture, &keys, &fixture.alice);
    assert_eq!(
        store_u64(
            &context,
            fixture.alice.shares.balance_store,
            token::balance_key()
        ),
        300
    );
    assert_eq!(
        store_u64(&context, keys.payout_balance_store, token::balance_key()),
        150
    );
}
