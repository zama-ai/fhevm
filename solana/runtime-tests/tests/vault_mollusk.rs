//! Mollusk-based runtime tests for the `demo-vault` public share-mint vault.
//!
//! The vault is standalone plain-SPL code (no `zama-host` / `confidential-token`
//! involvement), so this harness only registers the SPL Token program alongside
//! `demo_vault` — no host program, no FHE fixtures.
//!
//! Coverage: initialization (state + PDA-owned share mint and token account),
//! the first-deposit 1:1 price, a full deposit/withdraw round trip, that
//! `harvest` raises the share price so a later depositor gets fewer shares, the
//! ERC-4626 first-depositor inflation-attack regression (victim keeps shares and
//! the attacker cannot profit), and the input-validation rejects (zero amount,
//! withdraw more than owned, empty-vault withdraw). Rounding direction is
//! asserted in the deposit/withdraw math both here and in the program's own unit
//! tests.

// Deliberate `#[path]` include (not `mod support;`): pull in only the shared
// cost-snapshot helper, not the FHE support modules that `support/mod.rs`
// declares (this vault has no host/FHE dependency).
#[path = "support/cost_snapshot.rs"]
mod cost_snapshot;

use anchor_lang::{
    prelude::system_program, AccountDeserialize, AccountSerialize, InstructionData, ToAccountMetas,
};
use anchor_spl::token::spl_token;
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

const DECIMALS: u8 = 6;

// ---------------------------------------------------------------------------
// Harness
// ---------------------------------------------------------------------------

fn mollusk() -> Mollusk {
    let deploy_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy");
    unsafe {
        std::env::set_var("SBF_OUT_DIR", deploy_dir);
    }
    let mut mollusk = Mollusk::new(&vault::id(), "demo_vault");
    mollusk_svm_programs_token::token::add_program(&mut mollusk);
    // A deposit/withdraw issues two token CPIs; give it headroom over the 200k default.
    mollusk.compute_budget.compute_unit_limit = 400_000;
    mollusk
}

fn anchor_ix<A, D>(accounts: A, args: D) -> Instruction
where
    A: ToAccountMetas,
    D: InstructionData,
{
    Instruction {
        program_id: vault::id(),
        accounts: accounts.to_account_metas(None),
        data: args.data(),
    }
}

fn vault_error(error: vault::DemoVaultError) -> Check<'static> {
    Check::err(ProgramError::Custom(
        anchor_lang::error::ERROR_CODE_OFFSET + error as u32,
    ))
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

fn read_spl_amount(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> u64 {
    let store = context.account_store.borrow();
    let account = store.get(&address).expect("missing spl token account");
    spl_token::state::Account::unpack(&account.data)
        .expect("valid spl token account")
        .amount
}

fn read_mint_supply(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> u64 {
    let store = context.account_store.borrow();
    let account = store.get(&address).expect("missing spl mint");
    spl_token::state::Mint::unpack(&account.data)
        .expect("valid spl mint")
        .supply
}

fn read_vault(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> vault::Vault {
    let store = context.account_store.borrow();
    let account = store.get(&address).expect("missing vault");
    vault::Vault::try_deserialize(&mut account.data.as_slice()).expect("valid vault")
}

// ---------------------------------------------------------------------------
// Fixture for an already-initialized vault with a chosen assets/shares state
// ---------------------------------------------------------------------------

struct VaultFixture {
    vault: Pubkey,
    vault_authority: Pubkey,
    authority_bump: u8,
    underlying_mint: Pubkey,
    share_mint: Pubkey,
    vault_token_account: Pubkey,
}

impl VaultFixture {
    fn new() -> Self {
        Self::from_keys(Pubkey::new_unique(), Pubkey::new_unique())
    }

    /// Fixture with fully fixed keys. Used by the cost-snapshot tests: the
    /// on-chain PDA bump search is part of the measured compute, so the vault
    /// key (and therefore the bumps) must be stable across runs — `new_unique`
    /// is a global counter whose value depends on test ordering.
    fn fixed(seed: u8) -> Self {
        Self::from_keys(
            Pubkey::new_from_array([seed; 32]),
            Pubkey::new_from_array([seed.wrapping_add(1); 32]),
        )
    }

    fn from_keys(vault: Pubkey, underlying_mint: Pubkey) -> Self {
        let (vault_authority, authority_bump) =
            Pubkey::find_program_address(&[b"authority", vault.as_ref()], &vault::id());
        let share_mint = Pubkey::find_program_address(&[b"shares", vault.as_ref()], &vault::id()).0;
        let vault_token_account =
            Pubkey::find_program_address(&[b"underlying", vault.as_ref()], &vault::id()).0;
        Self {
            vault,
            vault_authority,
            authority_bump,
            underlying_mint,
            share_mint,
            vault_token_account,
        }
    }

    fn vault_account(&self) -> Account {
        let mut data = Vec::new();
        vault::Vault {
            underlying_mint: self.underlying_mint,
            share_mint: self.share_mint,
            vault_token_account: self.vault_token_account,
            authority_bump: self.authority_bump,
        }
        .try_serialize(&mut data)
        .unwrap();
        Account {
            lamports: 1_000_000_000,
            data,
            owner: vault::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    /// Base account set for a vault holding `total_assets` underlying and having
    /// `total_shares` shares outstanding. Callers add the user token accounts.
    fn accounts(&self, total_assets: u64, total_shares: u64) -> HashMap<Pubkey, Account> {
        HashMap::from([
            (self.vault, self.vault_account()),
            (self.vault_authority, system_account(0)),
            (self.underlying_mint, spl_mint_account(None, 0)),
            (
                self.share_mint,
                spl_mint_account(Some(self.vault_authority), total_shares),
            ),
            (
                self.vault_token_account,
                spl_token_account(self.underlying_mint, self.vault_authority, total_assets),
            ),
        ])
    }
}

fn deposit_ix(
    fixture: &VaultFixture,
    depositor: Pubkey,
    depositor_underlying: Pubkey,
    depositor_shares: Pubkey,
    amount: u64,
) -> Instruction {
    anchor_ix(
        vault::accounts::Deposit {
            depositor,
            vault: fixture.vault,
            vault_authority: fixture.vault_authority,
            underlying_mint: fixture.underlying_mint,
            share_mint: fixture.share_mint,
            depositor_underlying,
            vault_token_account: fixture.vault_token_account,
            depositor_shares,
            token_program: spl_token::id(),
        },
        vault::instruction::Deposit { amount },
    )
}

fn withdraw_ix(
    fixture: &VaultFixture,
    owner: Pubkey,
    owner_shares: Pubkey,
    owner_underlying: Pubkey,
    shares: u64,
) -> Instruction {
    anchor_ix(
        vault::accounts::Withdraw {
            owner,
            vault: fixture.vault,
            vault_authority: fixture.vault_authority,
            underlying_mint: fixture.underlying_mint,
            share_mint: fixture.share_mint,
            owner_shares,
            vault_token_account: fixture.vault_token_account,
            owner_underlying,
            token_program: spl_token::id(),
        },
        vault::instruction::Withdraw { shares },
    )
}

fn harvest_ix(
    fixture: &VaultFixture,
    donor: Pubkey,
    donor_underlying: Pubkey,
    amount: u64,
) -> Instruction {
    anchor_ix(
        vault::accounts::Harvest {
            donor,
            vault: fixture.vault,
            underlying_mint: fixture.underlying_mint,
            donor_underlying,
            vault_token_account: fixture.vault_token_account,
            token_program: spl_token::id(),
        },
        vault::instruction::Harvest { amount },
    )
}

// ---------------------------------------------------------------------------
// initialize_vault
// ---------------------------------------------------------------------------

#[test]
fn mollusk_initialize_vault_creates_state_share_mint_and_token_account() {
    let payer = Pubkey::new_unique();
    let vault_key = Pubkey::new_unique();
    let underlying_mint = Pubkey::new_unique();
    let (vault_authority, _) =
        Pubkey::find_program_address(&[b"authority", vault_key.as_ref()], &vault::id());
    let share_mint = Pubkey::find_program_address(&[b"shares", vault_key.as_ref()], &vault::id()).0;
    let vault_token_account =
        Pubkey::find_program_address(&[b"underlying", vault_key.as_ref()], &vault::id()).0;

    let context = mollusk().with_context(HashMap::from([
        (payer, system_account(5_000_000_000)),
        (vault_key, system_account(0)),
        (underlying_mint, spl_mint_account(Some(payer), 0)),
        (vault_authority, system_account(0)),
        (share_mint, system_account(0)),
        (vault_token_account, system_account(0)),
    ]));

    let ix = anchor_ix(
        vault::accounts::InitializeVault {
            payer,
            vault: vault_key,
            underlying_mint,
            vault_authority,
            share_mint,
            vault_token_account,
            token_program: spl_token::id(),
            system_program: system_program::ID,
        },
        vault::instruction::InitializeVault {},
    );

    context.process_and_validate_instruction(&ix, &[Check::success()]);

    let stored = read_vault(&context, vault_key);
    assert_eq!(stored.underlying_mint, underlying_mint);
    assert_eq!(stored.share_mint, share_mint);
    assert_eq!(stored.vault_token_account, vault_token_account);

    // Share mint created with the vault authority as mint authority, matching decimals, zero supply.
    let store = context.account_store.borrow();
    let share = spl_token::state::Mint::unpack(&store.get(&share_mint).unwrap().data).unwrap();
    assert_eq!(share.mint_authority, COption::Some(vault_authority));
    assert_eq!(share.decimals, DECIMALS);
    assert_eq!(share.supply, 0);
    // Vault token account owned by the authority PDA, for the underlying mint, empty.
    let token =
        spl_token::state::Account::unpack(&store.get(&vault_token_account).unwrap().data).unwrap();
    assert_eq!(token.owner, vault_authority);
    assert_eq!(token.mint, underlying_mint);
    assert_eq!(token.amount, 0);
}

// ---------------------------------------------------------------------------
// deposit / withdraw math
// ---------------------------------------------------------------------------

#[test]
fn mollusk_first_deposit_is_one_to_one() {
    let fixture = VaultFixture::new();
    let depositor = Pubkey::new_unique();
    let depositor_underlying = Pubkey::new_unique();
    let depositor_shares = Pubkey::new_unique();

    let mut accounts = fixture.accounts(0, 0);
    accounts.insert(depositor, system_account(1_000_000_000));
    accounts.insert(
        depositor_underlying,
        spl_token_account(fixture.underlying_mint, depositor, 1_000),
    );
    accounts.insert(
        depositor_shares,
        spl_token_account(fixture.share_mint, depositor, 0),
    );
    let context = mollusk().with_context(accounts);

    let ix = deposit_ix(
        &fixture,
        depositor,
        depositor_underlying,
        depositor_shares,
        1_000,
    );
    context.process_and_validate_instruction(&ix, &[Check::success()]);

    // Empty vault prices 1:1: 1_000 assets -> 1_000 shares.
    assert_eq!(read_spl_amount(&context, depositor_shares), 1_000);
    assert_eq!(read_spl_amount(&context, depositor_underlying), 0);
    assert_eq!(
        read_spl_amount(&context, fixture.vault_token_account),
        1_000
    );
    assert_eq!(read_mint_supply(&context, fixture.share_mint), 1_000);
}

#[test]
fn mollusk_deposit_withdraw_round_trip_returns_principal() {
    let fixture = VaultFixture::new();
    let owner = Pubkey::new_unique();
    let owner_underlying = Pubkey::new_unique();
    let owner_shares = Pubkey::new_unique();

    let mut accounts = fixture.accounts(0, 0);
    accounts.insert(owner, system_account(1_000_000_000));
    accounts.insert(
        owner_underlying,
        spl_token_account(fixture.underlying_mint, owner, 1_000),
    );
    accounts.insert(
        owner_shares,
        spl_token_account(fixture.share_mint, owner, 0),
    );
    let context = mollusk().with_context(accounts);

    let deposit = deposit_ix(&fixture, owner, owner_underlying, owner_shares, 1_000);
    context.process_and_validate_instruction(&deposit, &[Check::success()]);
    assert_eq!(read_spl_amount(&context, owner_shares), 1_000);

    // Redeem every share; with no yield the round trip returns the full principal.
    let withdraw = withdraw_ix(&fixture, owner, owner_shares, owner_underlying, 1_000);
    context.process_and_validate_instruction(&withdraw, &[Check::success()]);

    assert_eq!(read_spl_amount(&context, owner_shares), 0);
    assert_eq!(read_spl_amount(&context, owner_underlying), 1_000);
    assert_eq!(read_spl_amount(&context, fixture.vault_token_account), 0);
    assert_eq!(read_mint_supply(&context, fixture.share_mint), 0);
}

#[test]
fn mollusk_harvest_raises_price_so_later_depositor_gets_fewer_shares() {
    // Vault already at 1:1 with 1_000 assets and 1_000 shares outstanding.
    let fixture = VaultFixture::new();
    let donor = Pubkey::new_unique();
    let donor_underlying = Pubkey::new_unique();
    let depositor = Pubkey::new_unique();
    let depositor_underlying = Pubkey::new_unique();
    let depositor_shares = Pubkey::new_unique();

    let mut accounts = fixture.accounts(1_000, 1_000);
    accounts.insert(donor, system_account(1_000_000_000));
    accounts.insert(
        donor_underlying,
        spl_token_account(fixture.underlying_mint, donor, 1_000),
    );
    accounts.insert(depositor, system_account(1_000_000_000));
    accounts.insert(
        depositor_underlying,
        spl_token_account(fixture.underlying_mint, depositor, 1_000),
    );
    accounts.insert(
        depositor_shares,
        spl_token_account(fixture.share_mint, depositor, 0),
    );
    let context = mollusk().with_context(accounts);

    // Harvest doubles the assets without minting shares: price becomes ~2 assets/share.
    let harvest = harvest_ix(&fixture, donor, donor_underlying, 1_000);
    context.process_and_validate_instruction(&harvest, &[Check::success()]);
    assert_eq!(
        read_spl_amount(&context, fixture.vault_token_account),
        2_000
    );
    assert_eq!(read_mint_supply(&context, fixture.share_mint), 1_000);

    // A 1_000 deposit now buys shares = 1_000 * (1_000 + 1) / (2_000 + 1) = 500 (floor).
    let deposit = deposit_ix(
        &fixture,
        depositor,
        depositor_underlying,
        depositor_shares,
        1_000,
    );
    context.process_and_validate_instruction(&deposit, &[Check::success()]);
    assert_eq!(read_spl_amount(&context, depositor_shares), 500);
}

#[test]
fn mollusk_inflation_attack_leaves_victim_with_shares_and_no_attacker_profit() {
    let fixture = VaultFixture::new();
    let attacker = Pubkey::new_unique();
    let attacker_underlying = Pubkey::new_unique();
    let attacker_shares = Pubkey::new_unique();
    let victim = Pubkey::new_unique();
    let victim_underlying = Pubkey::new_unique();
    let victim_shares = Pubkey::new_unique();

    const DONATION: u64 = 1_000_000;
    const VICTIM_DEPOSIT: u64 = 1_000_000;

    let mut accounts = fixture.accounts(0, 0);
    accounts.insert(attacker, system_account(1_000_000_000));
    // 1 to seed the vault + DONATION to inflate it.
    accounts.insert(
        attacker_underlying,
        spl_token_account(fixture.underlying_mint, attacker, 1 + DONATION),
    );
    accounts.insert(
        attacker_shares,
        spl_token_account(fixture.share_mint, attacker, 0),
    );
    accounts.insert(victim, system_account(1_000_000_000));
    accounts.insert(
        victim_underlying,
        spl_token_account(fixture.underlying_mint, victim, VICTIM_DEPOSIT),
    );
    accounts.insert(
        victim_shares,
        spl_token_account(fixture.share_mint, victim, 0),
    );
    let context = mollusk().with_context(accounts);

    // Attacker seeds 1 asset -> 1 share (empty vault, 1:1).
    let seed = deposit_ix(&fixture, attacker, attacker_underlying, attacker_shares, 1);
    context.process_and_validate_instruction(&seed, &[Check::success()]);
    assert_eq!(read_spl_amount(&context, attacker_shares), 1);

    // Attacker donates directly via harvest to inflate the price.
    let donate = harvest_ix(&fixture, attacker, attacker_underlying, DONATION);
    context.process_and_validate_instruction(&donate, &[Check::success()]);

    // Victim deposits; the virtual offset keeps them from being rounded to zero shares.
    let victim_deposit = deposit_ix(
        &fixture,
        victim,
        victim_underlying,
        victim_shares,
        VICTIM_DEPOSIT,
    );
    context.process_and_validate_instruction(&victim_deposit, &[Check::success()]);
    let victim_share_balance = read_spl_amount(&context, victim_shares);
    assert!(
        victim_share_balance > 0,
        "virtual offset must leave the victim with shares, got {victim_share_balance}"
    );

    // Attacker redeems their single share; the donation is shared with the victim, so the
    // attacker cannot recover what they spent (1 seed + DONATION) — the attack is uneconomical.
    let attacker_exit = withdraw_ix(&fixture, attacker, attacker_shares, attacker_underlying, 1);
    context.process_and_validate_instruction(&attacker_exit, &[Check::success()]);
    let attacker_final = read_spl_amount(&context, attacker_underlying);
    assert!(
        attacker_final < 1 + DONATION,
        "attacker must not profit; started with {} and ended with {attacker_final}",
        1 + DONATION
    );
}

// ---------------------------------------------------------------------------
// rejects
// ---------------------------------------------------------------------------

#[test]
fn mollusk_deposit_zero_amount_rejects() {
    let fixture = VaultFixture::new();
    let depositor = Pubkey::new_unique();
    let depositor_underlying = Pubkey::new_unique();
    let depositor_shares = Pubkey::new_unique();

    let mut accounts = fixture.accounts(0, 0);
    accounts.insert(depositor, system_account(1_000_000_000));
    accounts.insert(
        depositor_underlying,
        spl_token_account(fixture.underlying_mint, depositor, 1_000),
    );
    accounts.insert(
        depositor_shares,
        spl_token_account(fixture.share_mint, depositor, 0),
    );
    let context = mollusk().with_context(accounts);

    let ix = deposit_ix(
        &fixture,
        depositor,
        depositor_underlying,
        depositor_shares,
        0,
    );
    context
        .process_and_validate_instruction(&ix, &[vault_error(vault::DemoVaultError::ZeroAmount)]);
}

#[test]
fn mollusk_withdraw_more_than_owned_rejects() {
    let fixture = VaultFixture::new();
    let owner = Pubkey::new_unique();
    let owner_underlying = Pubkey::new_unique();
    let owner_shares = Pubkey::new_unique();

    // Vault holds 1_000 assets / 1_000 shares; owner holds only 10 shares.
    let mut accounts = fixture.accounts(1_000, 1_000);
    accounts.insert(owner, system_account(1_000_000_000));
    accounts.insert(
        owner_underlying,
        spl_token_account(fixture.underlying_mint, owner, 0),
    );
    accounts.insert(
        owner_shares,
        spl_token_account(fixture.share_mint, owner, 10),
    );
    let context = mollusk().with_context(accounts);

    let ix = withdraw_ix(&fixture, owner, owner_shares, owner_underlying, 11);
    context.process_and_validate_instruction(
        &ix,
        &[vault_error(vault::DemoVaultError::InsufficientShares)],
    );
}

#[test]
fn mollusk_withdraw_from_empty_vault_rejects() {
    // Empty vault: supply is 0, so nobody can own shares; a withdraw of 1 fails
    // the ownership check before any burn/transfer.
    let fixture = VaultFixture::new();
    let owner = Pubkey::new_unique();
    let owner_underlying = Pubkey::new_unique();
    let owner_shares = Pubkey::new_unique();

    let mut accounts = fixture.accounts(0, 0);
    accounts.insert(owner, system_account(1_000_000_000));
    accounts.insert(
        owner_underlying,
        spl_token_account(fixture.underlying_mint, owner, 0),
    );
    accounts.insert(
        owner_shares,
        spl_token_account(fixture.share_mint, owner, 0),
    );
    let context = mollusk().with_context(accounts);

    let ix = withdraw_ix(&fixture, owner, owner_shares, owner_underlying, 1);
    context.process_and_validate_instruction(
        &ix,
        &[vault_error(vault::DemoVaultError::InsufficientShares)],
    );
}

// ---------------------------------------------------------------------------
// account-constraint rejects (has_one / token-account constraints)
// ---------------------------------------------------------------------------

#[test]
fn mollusk_deposit_wrong_mint_token_account_rejects() {
    let fixture = VaultFixture::new();
    let other_mint = Pubkey::new_unique();
    let depositor = Pubkey::new_unique();
    let depositor_underlying = Pubkey::new_unique();
    let depositor_shares = Pubkey::new_unique();

    let mut accounts = fixture.accounts(0, 0);
    accounts.insert(depositor, system_account(1_000_000_000));
    accounts.insert(other_mint, spl_mint_account(None, 0));
    // Source token account is for a different mint than the vault's underlying.
    accounts.insert(
        depositor_underlying,
        spl_token_account(other_mint, depositor, 1_000),
    );
    accounts.insert(
        depositor_shares,
        spl_token_account(fixture.share_mint, depositor, 0),
    );
    let context = mollusk().with_context(accounts);

    let ix = deposit_ix(
        &fixture,
        depositor,
        depositor_underlying,
        depositor_shares,
        1_000,
    );
    context
        .process_and_validate_instruction(&ix, &[vault_error(vault::DemoVaultError::MintMismatch)]);
}

#[test]
fn mollusk_withdraw_foreign_share_mint_rejects() {
    let fixture = VaultFixture::new();
    let foreign = VaultFixture::new();
    let owner = Pubkey::new_unique();
    let owner_underlying = Pubkey::new_unique();
    let owner_shares = Pubkey::new_unique();

    let mut accounts = fixture.accounts(1_000, 1_000);
    // Bring in a foreign vault's share mint and a matching owner share account.
    accounts.insert(
        foreign.share_mint,
        spl_mint_account(Some(foreign.vault_authority), 1_000),
    );
    accounts.insert(owner, system_account(1_000_000_000));
    accounts.insert(
        owner_underlying,
        spl_token_account(fixture.underlying_mint, owner, 0),
    );
    accounts.insert(
        owner_shares,
        spl_token_account(foreign.share_mint, owner, 1_000),
    );
    let context = mollusk().with_context(accounts);

    // Pass the foreign share mint where the vault expects its own -> has_one fails.
    let ix = anchor_ix(
        vault::accounts::Withdraw {
            owner,
            vault: fixture.vault,
            vault_authority: fixture.vault_authority,
            underlying_mint: fixture.underlying_mint,
            share_mint: foreign.share_mint,
            owner_shares,
            vault_token_account: fixture.vault_token_account,
            owner_underlying,
            token_program: spl_token::id(),
        },
        vault::instruction::Withdraw { shares: 100 },
    );
    context.process_and_validate_instruction(
        &ix,
        &[vault_error(vault::DemoVaultError::ShareMintMismatch)],
    );
}

#[test]
fn mollusk_harvest_wrong_vault_token_account_rejects() {
    let fixture = VaultFixture::new();
    let foreign = VaultFixture::new();
    let donor = Pubkey::new_unique();
    let donor_underlying = Pubkey::new_unique();

    let mut accounts = fixture.accounts(1_000, 1_000);
    // A valid token account for the right mint/authority but NOT the vault's recorded one.
    accounts.insert(
        foreign.vault_token_account,
        spl_token_account(fixture.underlying_mint, fixture.vault_authority, 0),
    );
    accounts.insert(donor, system_account(1_000_000_000));
    accounts.insert(
        donor_underlying,
        spl_token_account(fixture.underlying_mint, donor, 1_000),
    );
    let context = mollusk().with_context(accounts);

    let ix = anchor_ix(
        vault::accounts::Harvest {
            donor,
            vault: fixture.vault,
            underlying_mint: fixture.underlying_mint,
            donor_underlying,
            vault_token_account: foreign.vault_token_account,
            token_program: spl_token::id(),
        },
        vault::instruction::Harvest { amount: 1_000 },
    );
    context.process_and_validate_instruction(
        &ix,
        &[vault_error(
            vault::DemoVaultError::VaultTokenAccountMismatch,
        )],
    );
}

// ---------------------------------------------------------------------------
// cost snapshots
// ---------------------------------------------------------------------------

fn assert_deposit_cost(profile: &str, result: &InstructionResult, ix: &Instruction) {
    cost_snapshot::assert_cost_snapshot("vault_mollusk", profile, ix, result);
}

#[test]
fn cost_snapshot_deposit() {
    let fixture = VaultFixture::fixed(0x11);
    let depositor = Pubkey::new_from_array([0x21; 32]);
    let depositor_underlying = Pubkey::new_from_array([0x22; 32]);
    let depositor_shares = Pubkey::new_from_array([0x23; 32]);
    let mut accounts = fixture.accounts(0, 0);
    accounts.insert(depositor, system_account(1_000_000_000));
    accounts.insert(
        depositor_underlying,
        spl_token_account(fixture.underlying_mint, depositor, 1_000),
    );
    accounts.insert(
        depositor_shares,
        spl_token_account(fixture.share_mint, depositor, 0),
    );
    let context = mollusk().with_context(accounts);
    let ix = deposit_ix(
        &fixture,
        depositor,
        depositor_underlying,
        depositor_shares,
        1_000,
    );
    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);
    assert_deposit_cost("deposit", &result, &ix);
}

#[test]
fn cost_snapshot_withdraw() {
    let fixture = VaultFixture::fixed(0x31);
    let owner = Pubkey::new_from_array([0x41; 32]);
    let owner_underlying = Pubkey::new_from_array([0x42; 32]);
    let owner_shares = Pubkey::new_from_array([0x43; 32]);
    let mut accounts = fixture.accounts(1_000, 1_000);
    accounts.insert(owner, system_account(1_000_000_000));
    accounts.insert(
        owner_underlying,
        spl_token_account(fixture.underlying_mint, owner, 0),
    );
    accounts.insert(
        owner_shares,
        spl_token_account(fixture.share_mint, owner, 1_000),
    );
    let context = mollusk().with_context(accounts);
    let ix = withdraw_ix(&fixture, owner, owner_shares, owner_underlying, 500);
    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);
    assert_deposit_cost("withdraw", &result, &ix);
}

#[test]
fn cost_snapshot_harvest() {
    let fixture = VaultFixture::fixed(0x51);
    let donor = Pubkey::new_from_array([0x61; 32]);
    let donor_underlying = Pubkey::new_from_array([0x62; 32]);
    let mut accounts = fixture.accounts(1_000, 1_000);
    accounts.insert(donor, system_account(1_000_000_000));
    accounts.insert(
        donor_underlying,
        spl_token_account(fixture.underlying_mint, donor, 1_000),
    );
    let context = mollusk().with_context(accounts);
    let ix = harvest_ix(&fixture, donor, donor_underlying, 1_000);
    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);
    assert_deposit_cost("harvest", &result, &ix);
}
