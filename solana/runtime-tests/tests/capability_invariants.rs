//! Capability invariants of `zama-host`, checked over random instruction sequences against the
//! shipped artifact.
//!
//! - **H1** (INVARIANTS #35, #36): a trust root changes only in a transaction the admin signed.
//!   The trust roots are `HostConfig` (admin, coprocessor signers, EIP-712 domain, pause flags,
//!   deny-list switch, HCU limits), every KMS context, the deny and HCU trust records, and the
//!   pauser records. The one exception is a pause: a key whose pauser record was enabled may set
//!   pause flags and nothing else. A `HostConfig` change also stamps the current slot and emits an
//!   event CPI.
//! - **H2** (INVARIANTS #11): a Store changes only in a transaction its authority signed.
//!
//! The oracles compare raw account bytes before and after each transaction against the
//! transaction's signer flags. They find the guarded accounts by discriminator among every
//! host-owned account, and fail on a host account type nobody classified. They read program
//! layouts only for a Store's authority, a pauser record's grant, and `HostConfig`'s pause flags
//! and `updated_slot`. The only state they carry
//! is who the admin is, which moves when a `set_admin` succeeds. Every instruction of the host
//! IDL has a generator, and every admin and Store-authority role is also filled by keys that lack
//! it, signing or not.
//! `scripts/check-planted-bugs.sh` rebuilds the host with each patch in `planted-bugs/` and
//! requires this suite to fail.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::OnceLock;

use anchor_lang::prelude::system_program;
use anchor_lang::{AccountDeserialize, AccountSerialize, Discriminator, InstructionData};
use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::{Config, TestRunner};
use solana_sdk::{account::Account, instruction::Instruction, pubkey::Pubkey};
use zama_host::encode::ExecutionDictionary;
use zama_host::{self as host, AppScope, FheExecuteArgs, FheExecuteStep};
use zama_solana_test_kit::{
    anchor_ix, canonical_test_context_id, empty_system_account, encrypted_store_account,
    event_authority, funded_system_account, host_config_account, host_svm, kms_context_account,
    label, new_encrypted_store, pauser_record_account, program_data_account, program_owned_account,
    readonly, readonly_signer, system_program_account, transaction::fhe_transaction, writable, Ctx,
    HostConfigParams,
};

mod host_fixtures;
use host_fixtures::{fhe_execute_ix, store_authority, StoreAuthority};

const WALLETS: usize = 4;
const AUTHORITIES: usize = 3;
const SCOPES: usize = 2;
const STORES: usize = AUTHORITIES * SCOPES;
const PROGRAMS: usize = 2;
const APPS: usize = PROGRAMS * SCOPES;
const KMS_CONTEXTS: usize = 3;
/// The wallet whose pauser record exists at genesis; `Key::Holder` for `pause`.
const GENESIS_PAUSER: usize = 1;

/// Which key fills a role account. `Holder` is the key that holds the role: the admin for admin
/// instructions, the Store's authority for Store instructions.
#[derive(Clone, Copy, Debug)]
enum Key {
    Holder,
    Wallet(usize),
    Authority(usize),
}

#[derive(Clone, Copy, Debug)]
struct Role {
    key: Key,
    signs: bool,
}

/// How an `fhe_execute` writing another authority's Store presents that authority.
#[derive(Clone, Copy, Debug)]
enum Witness {
    Absent,
    Unsigned,
    Signed,
}

/// One instruction of the host IDL with the choices its generator makes.
#[derive(Clone, Debug)]
enum Action {
    InitializeHostConfig {
        payer: usize,
        admin: Role,
    },
    SetAdmin {
        admin: Role,
        new_admin: Key,
        cosign: bool,
    },
    Pause {
        pauser: Role,
        /// The wallet whose pauser record is presented, when not the pauser's own.
        record_of: Option<usize>,
        areas: host::PauseFlags,
    },
    Unpause {
        admin: Role,
        areas: host::PauseFlags,
    },
    SetPauser {
        payer: usize,
        admin: Role,
        pauser: usize,
        enabled: bool,
    },
    SetGrantDenyListEnabled {
        admin: Role,
        enabled: bool,
    },
    SetEip712Domain {
        admin: Role,
        gateway_chain_id: u64,
    },
    SetCoprocessorSigners {
        admin: Role,
        signer: u8,
    },
    SetMaxHcuPerTx {
        admin: Role,
        value: u64,
    },
    SetMaxHcuDepthPerTx {
        admin: Role,
        value: u64,
    },
    SetHcuBlockCapPerApp {
        admin: Role,
        value: u64,
    },
    SetHcuAppTrusted {
        payer: usize,
        admin: Role,
        app: usize,
        trusted: bool,
    },
    SetDenyScope {
        payer: usize,
        admin: Role,
        app: usize,
        denied: bool,
    },
    DefineKmsContext {
        admin: Role,
        context: usize,
    },
    DestroyKmsContext {
        admin: Role,
        context: usize,
    },
    CreateEncryptedStore {
        payer: usize,
        store: usize,
        authority: Role,
    },
    FheExecute {
        payer: usize,
        producer: usize,
        authority: Role,
        target: usize,
        witness: Witness,
        make_public: bool,
        allow_viewer: bool,
    },
    MakeStoreHandlePublic {
        payer: usize,
        store: usize,
        authority: Role,
    },
    OpenTransientStore {
        payer: usize,
    },
    CloseTransientStore {
        payer: usize,
    },
    DelegateForUserDecryption {
        delegator: usize,
    },
    RevokeDelegationForUserDecryption {
        delegator: usize,
    },
    RevokePermits {
        user: usize,
    },
    VerifyPublicDecrypt {
        store: usize,
    },
}

impl Action {
    /// The IDL name of the instruction this action sends.
    fn instruction_name(&self) -> &'static str {
        match self {
            Action::InitializeHostConfig { .. } => "initialize_host_config",
            Action::SetAdmin { .. } => "set_admin",
            Action::Pause { .. } => "pause",
            Action::Unpause { .. } => "unpause",
            Action::SetPauser { .. } => "set_pauser",
            Action::SetGrantDenyListEnabled { .. } => "set_grant_deny_list_enabled",
            Action::SetEip712Domain { .. } => "set_eip712_domain",
            Action::SetCoprocessorSigners { .. } => "set_coprocessor_signers",
            Action::SetMaxHcuPerTx { .. } => "set_max_hcu_per_tx",
            Action::SetMaxHcuDepthPerTx { .. } => "set_max_hcu_depth_per_tx",
            Action::SetHcuBlockCapPerApp { .. } => "set_hcu_block_cap_per_app",
            Action::SetHcuAppTrusted { .. } => "set_hcu_app_trusted",
            Action::SetDenyScope { .. } => "set_deny_scope",
            Action::DefineKmsContext { .. } => "define_kms_context",
            Action::DestroyKmsContext { .. } => "destroy_kms_context",
            Action::CreateEncryptedStore { .. } => "create_encrypted_store",
            Action::FheExecute { .. } => "fhe_execute",
            Action::MakeStoreHandlePublic { .. } => "make_store_handle_public",
            Action::OpenTransientStore { .. } => "open_transient_store",
            Action::CloseTransientStore { .. } => "close_transient_store",
            Action::DelegateForUserDecryption { .. } => "delegate_for_user_decryption",
            Action::RevokeDelegationForUserDecryption { .. } => {
                "revoke_delegation_for_user_decryption"
            }
            Action::RevokePermits { .. } => "revoke_permits",
            Action::VerifyPublicDecrypt { .. } => "verify_public_decrypt",
        }
    }
}

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        3 => Just(Key::Holder),
        1 => (0..WALLETS).prop_map(Key::Wallet),
        1 => (0..AUTHORITIES).prop_map(Key::Authority),
    ]
}

fn role() -> impl Strategy<Value = Role> {
    (key(), prop::bool::weighted(0.9)).prop_map(|(key, signs)| Role { key, signs })
}

fn hcu_limit() -> impl Strategy<Value = u64> {
    prop_oneof![3 => Just(u64::MAX), 1 => Just(5_000_000u64), 1 => Just(0u64)]
}

fn areas() -> impl Strategy<Value = host::PauseFlags> {
    any::<[bool; 4]>().prop_map(|[execution, verified_inputs, acl_writes, public_decrypt]| {
        host::PauseFlags {
            execution,
            verified_inputs,
            acl_writes,
            public_decrypt,
        }
    })
}

fn wallet() -> impl Strategy<Value = usize> {
    0..WALLETS
}

fn witness() -> impl Strategy<Value = Witness> {
    prop_oneof![
        Just(Witness::Absent),
        Just(Witness::Unsigned),
        Just(Witness::Signed)
    ]
}

fn action() -> impl Strategy<Value = Action> {
    prop_oneof![
        1 => (wallet(), role()).prop_map(|(payer, admin)| Action::InitializeHostConfig { payer, admin }),
        2 => (role(), key(), any::<bool>())
            .prop_map(|(admin, new_admin, cosign)| Action::SetAdmin { admin, new_admin, cosign }),
        2 => (role(), prop::option::weighted(0.25, wallet()), areas())
            .prop_map(|(pauser, record_of, areas)| Action::Pause { pauser, record_of, areas }),
        2 => (role(), areas()).prop_map(|(admin, areas)| Action::Unpause { admin, areas }),
        // Half the grants and withdrawals target the genesis pauser, the key `Pause` signs with
        // most, so a withdrawn pauser trying to pause is common rather than rare.
        3 => (wallet(), role(), prop_oneof![Just(GENESIS_PAUSER), wallet()], any::<bool>())
            .prop_map(|(payer, admin, pauser, enabled)| Action::SetPauser { payer, admin, pauser, enabled }),
        2 => (role(), any::<bool>())
            .prop_map(|(admin, enabled)| Action::SetGrantDenyListEnabled { admin, enabled }),
        1 => (role(), 1..4u64)
            .prop_map(|(admin, gateway_chain_id)| Action::SetEip712Domain { admin, gateway_chain_id }),
        1 => (role(), 1..4u8).prop_map(|(admin, signer)| Action::SetCoprocessorSigners { admin, signer }),
        1 => (role(), hcu_limit()).prop_map(|(admin, value)| Action::SetMaxHcuPerTx { admin, value }),
        1 => (role(), hcu_limit()).prop_map(|(admin, value)| Action::SetMaxHcuDepthPerTx { admin, value }),
        1 => (role(), hcu_limit()).prop_map(|(admin, value)| Action::SetHcuBlockCapPerApp { admin, value }),
        2 => (wallet(), role(), 0..APPS, any::<bool>())
            .prop_map(|(payer, admin, app, trusted)| Action::SetHcuAppTrusted { payer, admin, app, trusted }),
        2 => (wallet(), role(), 0..APPS, any::<bool>())
            .prop_map(|(payer, admin, app, denied)| Action::SetDenyScope { payer, admin, app, denied }),
        1 => (role(), 0..KMS_CONTEXTS).prop_map(|(admin, context)| Action::DefineKmsContext { admin, context }),
        1 => (role(), 0..KMS_CONTEXTS).prop_map(|(admin, context)| Action::DestroyKmsContext { admin, context }),
        2 => (wallet(), 0..STORES, role())
            .prop_map(|(payer, store, authority)| Action::CreateEncryptedStore { payer, store, authority }),
        6 => (wallet(), 0..AUTHORITIES, role(), 0..STORES, witness(), any::<bool>(), any::<bool>()).prop_map(
            |(payer, producer, authority, target, witness, make_public, allow_viewer)| Action::FheExecute {
                payer,
                producer,
                authority,
                target,
                witness,
                make_public,
                allow_viewer,
            }
        ),
        3 => (wallet(), 0..STORES, role())
            .prop_map(|(payer, store, authority)| Action::MakeStoreHandlePublic { payer, store, authority }),
        1 => wallet().prop_map(|payer| Action::OpenTransientStore { payer }),
        1 => wallet().prop_map(|payer| Action::CloseTransientStore { payer }),
        1 => wallet().prop_map(|delegator| Action::DelegateForUserDecryption { delegator }),
        1 => wallet().prop_map(|delegator| Action::RevokeDelegationForUserDecryption { delegator }),
        1 => wallet().prop_map(|user| Action::RevokePermits { user }),
        1 => (0..STORES).prop_map(|store| Action::VerifyPublicDecrypt { store }),
    ]
}

/// The vendored host IDL, which CI keeps byte-identical to a fresh build of the default artifact.
fn idl_discriminators() -> &'static HashMap<String, Vec<u8>> {
    static IDL: OnceLock<HashMap<String, Vec<u8>>> = OnceLock::new();
    IDL.get_or_init(|| {
        let idl: serde_json::Value = serde_json::from_str(include_str!(
            "../../../coprocessor/fhevm-engine/host-listener/idl/zama_host.json"
        ))
        .expect("host IDL parses");
        idl["instructions"]
            .as_array()
            .expect("IDL instructions")
            .iter()
            .map(|instruction| {
                let name = instruction["name"].as_str().expect("instruction name");
                let discriminator = instruction["discriminator"]
                    .as_array()
                    .expect("instruction discriminator")
                    .iter()
                    .map(|byte| byte.as_u64().expect("discriminator byte") as u8)
                    .collect();
                (name.to_owned(), discriminator)
            })
            .collect()
    })
}

/// Host account types only the admin may change (H1), except that a pauser sets pause flags.
const TRUST_ROOTS: [&[u8]; 5] = [
    host::HostConfig::DISCRIMINATOR,
    host::KmsContext::DISCRIMINATOR,
    host::DenyScopeRecord::DISCRIMINATOR,
    host::HcuTrustedAppRecord::DISCRIMINATOR,
    host::PauserRecord::DISCRIMINATOR,
];

/// Host account types outside H1 and H2: transaction scratch, the global rand nonce and the
/// per-app HCU meters that executions advance, and records owned by the user who signs for them.
const UNGUARDED: [&[u8]; 5] = [
    host::TransientStore::DISCRIMINATOR,
    host::RandNonce::DISCRIMINATOR,
    host::HcuBlockMeter::DISCRIMINATOR,
    host::PermitInvalidation::DISCRIMINATOR,
    host::UserDecryptionDelegation::DISCRIMINATOR,
];

/// Clears `key`'s signer flag in `ix` unless the role signs.
fn unsign(mut ix: Instruction, key: Pubkey, signs: bool) -> Instruction {
    if !signs {
        for meta in ix.accounts.iter_mut().filter(|meta| meta.pubkey == key) {
            meta.is_signer = false;
        }
    }
    ix
}

type Snapshot = BTreeMap<Pubkey, Vec<u8>>;

/// The data of host-owned accounts by address, split by the invariant that guards them.
#[derive(Default)]
struct HostAccounts {
    trust_roots: Snapshot,
    stores: Snapshot,
}

/// Every address whose data differs between two snapshots, with the data it had before and
/// after; a created or closed account has only one.
fn changed<'a>(before: &'a Snapshot, after: &'a Snapshot) -> Vec<(Pubkey, Vec<&'a [u8]>)> {
    let addresses: BTreeSet<&Pubkey> = before.keys().chain(after.keys()).collect();
    addresses
        .into_iter()
        .filter(|address| before.get(address) != after.get(address))
        .map(|address| {
            let versions = [before.get(address), after.get(address)];
            (
                *address,
                versions.into_iter().flatten().map(Vec::as_slice).collect(),
            )
        })
        .collect()
}

/// The deployment the sequences run against: one host config, a current and a rotated-out KMS
/// context, Stores of three authorities across two programs, and four wallets. Wallet 0 is the
/// first admin and the upgrade authority; wallet 1 is the first pauser.
struct World {
    context: Ctx,
    admin: Pubkey,
    wallets: [Pubkey; WALLETS],
    authorities: [StoreAuthority; AUTHORITIES],
}

fn program(index: usize) -> Pubkey {
    Pubkey::new_from_array([0xA0 + index as u8; 32])
}

/// A scope is an account of its program, so each program has its own.
fn scope(program: Pubkey, index: usize) -> Pubkey {
    let mut scope = label(&format!("scope-{index}"));
    scope[31] = program.to_bytes()[0];
    Pubkey::new_from_array(scope)
}

fn app(index: usize) -> AppScope {
    let program = program(index / SCOPES);
    AppScope {
        program,
        scope: scope(program, index % SCOPES),
    }
}

fn kms_context_id(index: usize) -> [u8; 32] {
    canonical_test_context_id(index as u8 + 1)
}

fn slot_key() -> [u8; 32] {
    label("slot")
}

impl World {
    fn new() -> Self {
        let wallets = std::array::from_fn(|index| Pubkey::new_from_array([0x10 + index as u8; 32]));
        // Two authorities of the first program and one of the second.
        let authorities = std::array::from_fn(|index| {
            store_authority(
                program(index / 2),
                Pubkey::new_from_array([0x20 + index as u8; 32]),
            )
        });
        let admin = wallets[0];
        let (host_config, host_config_account) = host_config_account(&HostConfigParams {
            current_kms_context_id: kms_context_id(0),
            ..HostConfigParams::new(admin)
        });
        let mut accounts = HashMap::from([
            (host_config, host_config_account),
            kms_context_account(kms_context_id(0), vec![[0x33; 20]], 1),
            kms_context_account(kms_context_id(1), vec![[0x34; 20]], 1),
            program_data_account(Some(admin)),
            pauser_record_account(wallets[GENESIS_PAUSER], true),
            (system_program::ID, system_program_account()),
            (event_authority(host::id()), Account::default()),
        ]);
        for wallet in wallets {
            accounts.insert(wallet, funded_system_account());
        }
        for index in 0..APPS {
            let app = app(index);
            accounts.insert(app.scope, program_owned_account(app.program));
        }
        let world = Self {
            context: host_svm().with_context(HashMap::new()),
            admin,
            wallets,
            authorities,
        };
        for authority in authorities {
            accounts.insert(authority.key, empty_system_account());
        }
        // Every authority's first-scope Store exists; the others are left for
        // `create_encrypted_store`.
        for store in 0..STORES {
            if store % SCOPES == 0 {
                let authority = world.store_authority(store);
                let (address, state) =
                    new_encrypted_store(authority.app(world.store_scope(store)), authority.key, []);
                accounts.insert(address, encrypted_store_account(&state));
            }
        }
        *world.context.account_store.borrow_mut() = accounts;
        world
    }

    fn store_authority(&self, store: usize) -> StoreAuthority {
        self.authorities[store / SCOPES]
    }

    fn store_scope(&self, store: usize) -> Pubkey {
        scope(self.store_authority(store).program, store % SCOPES)
    }

    fn store_address(&self, store: usize) -> Pubkey {
        self.store_authority(store)
            .state_address(self.store_scope(store))
    }

    fn store_app(&self, store: usize) -> AppScope {
        self.store_authority(store).app(self.store_scope(store))
    }

    fn key(&self, key: Key, holder: Pubkey) -> Pubkey {
        match key {
            Key::Holder => holder,
            Key::Wallet(index) => self.wallets[index],
            Key::Authority(index) => self.authorities[index].key,
        }
    }

    /// The data of every host-owned account. Lamports are left out because anyone may fund any
    /// account. An account that leaves the host, by closing or reassignment, drops out.
    fn host_accounts(&self) -> Result<HostAccounts, TestCaseError> {
        let mut accounts = HostAccounts::default();
        for (address, account) in self.context.account_store.borrow().iter() {
            if account.owner != host::id() {
                continue;
            }
            let has_type =
                |types: &[&[u8]]| types.iter().any(|kind| account.data.starts_with(kind));
            if has_type(&TRUST_ROOTS) {
                accounts.trust_roots.insert(*address, account.data.clone());
            } else if has_type(&[host::EncryptedStore::DISCRIMINATOR]) {
                accounts.stores.insert(*address, account.data.clone());
            } else {
                prop_assert!(
                    has_type(&UNGUARDED),
                    "host account {address} has a type that neither H1 nor H2 classifies"
                );
            }
        }
        Ok(accounts)
    }

    /// The live config, read only to build instructions the host can accept.
    fn config(&self) -> host::HostConfig {
        let store = self.context.account_store.borrow();
        let account = store
            .get(&host::host_config_address().0)
            .expect("host config");
        host::HostConfig::try_deserialize(&mut account.data.as_slice())
            .expect("host config decodes")
    }

    /// The live Store, read only to build instructions the host can accept.
    fn store_state(&self, store: usize) -> Option<host::EncryptedStore> {
        let accounts = self.context.account_store.borrow();
        let account = accounts.get(&self.store_address(store))?;
        host::EncryptedStore::try_deserialize(&mut account.data.as_slice()).ok()
    }

    /// A delegation from `delegator` to the last wallet in the first Store's application: one
    /// record per delegator, so a revoke can find what a delegate created.
    fn delegation(&self, delegator: usize) -> (Pubkey, Pubkey) {
        (self.wallets[delegator], self.wallets[WALLETS - 1])
    }

    fn deny_record(&self, app: AppScope) -> Option<Pubkey> {
        self.config()
            .grant_deny_list_enabled
            .then(|| host::deny_scope_address(app).0)
    }

    /// The transaction an action sends. Only the body instruction comes from the action; an
    /// `fhe_execute` travels in the transient-store envelope every FHE call needs.
    fn transaction(&self, action: &Action) -> Vec<Instruction> {
        let host_config = host::host_config_address().0;
        let admin_role = |role: &Role| {
            let key = self.key(role.key, self.admin);
            (key, role.signs)
        };
        let host_admin = |role: &Role, data: Vec<u8>| {
            let (admin, signs) = admin_role(role);
            let accounts = host::accounts::HostAdmin {
                admin,
                host_config,
                event_authority: event_authority(host::id()),
                program: host::id(),
            };
            let ix = Instruction {
                program_id: host::id(),
                accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
                data,
            };
            unsign(ix, admin, signs)
        };
        let body = match action {
            Action::InitializeHostConfig { payer, admin } => {
                let (admin, signs) = admin_role(admin);
                let ix = anchor_ix(
                    host::id(),
                    host::accounts::InitializeHostConfig {
                        payer: self.wallets[*payer],
                        admin,
                        program_data: program_data_account(None).0,
                        host_config,
                        rand_nonce: host::rand_nonce_address().0,
                        system_program: system_program::ID,
                        event_authority: event_authority(host::id()),
                        program: host::id(),
                    },
                    host::instruction::InitializeHostConfig {
                        args: host::InitializeHostConfigArgs {
                            chain_id: host::SOLANA_POC_CHAIN_ID,
                            gateway_chain_id: 7,
                            input_verification_contract: [0x44; 20],
                            coprocessor_signers: vec![[0x55; 20]],
                            coprocessor_threshold: 1,
                            decryption_contract: [0x66; 20],
                            grant_deny_list_enabled: false,
                        },
                    },
                );
                unsign(ix, admin, signs)
            }
            Action::SetAdmin {
                admin,
                new_admin,
                cosign,
            } => {
                let (admin, signs) = admin_role(admin);
                let new_admin = self.key(*new_admin, self.admin);
                let mut ix = anchor_ix(
                    host::id(),
                    host::accounts::SetAdmin {
                        admin,
                        host_config,
                        new_admin,
                        event_authority: event_authority(host::id()),
                        program: host::id(),
                    },
                    host::instruction::SetAdmin { new_admin },
                );
                if *cosign {
                    for meta in ix
                        .accounts
                        .iter_mut()
                        .filter(|meta| meta.pubkey == new_admin)
                    {
                        meta.is_signer = true;
                    }
                }
                unsign(ix, admin, signs)
            }
            Action::Pause {
                pauser: role,
                record_of,
                areas,
            } => {
                let pauser = self.key(role.key, self.wallets[GENESIS_PAUSER]);
                let record_owner = record_of.map_or(pauser, |wallet| self.wallets[wallet]);
                let ix = anchor_ix(
                    host::id(),
                    host::accounts::Pause {
                        pauser,
                        pauser_record: host::pauser_address(record_owner).0,
                        host_config,
                        event_authority: event_authority(host::id()),
                        program: host::id(),
                    },
                    host::instruction::Pause { areas: *areas },
                );
                unsign(ix, pauser, role.signs)
            }
            Action::Unpause { admin, areas } => {
                host_admin(admin, host::instruction::Unpause { areas: *areas }.data())
            }
            Action::SetPauser {
                payer,
                admin,
                pauser,
                enabled,
            } => {
                let (admin, signs) = admin_role(admin);
                let pauser = self.wallets[*pauser];
                let ix = anchor_ix(
                    host::id(),
                    host::accounts::SetPauser {
                        payer: self.wallets[*payer],
                        admin,
                        host_config,
                        pauser_record: host::pauser_address(pauser).0,
                        system_program: system_program::ID,
                        event_authority: event_authority(host::id()),
                        program: host::id(),
                    },
                    host::instruction::SetPauser {
                        pauser,
                        enabled: *enabled,
                    },
                );
                unsign(ix, admin, signs)
            }
            Action::SetGrantDenyListEnabled { admin, enabled } => host_admin(
                admin,
                host::instruction::SetGrantDenyListEnabled { enabled: *enabled }.data(),
            ),
            Action::SetEip712Domain {
                admin,
                gateway_chain_id,
            } => host_admin(
                admin,
                host::instruction::SetEip712Domain {
                    gateway_chain_id: *gateway_chain_id,
                    input_verification_contract: [0x77; 20],
                    decryption_contract: [0x88; 20],
                }
                .data(),
            ),
            Action::SetCoprocessorSigners { admin, signer } => host_admin(
                admin,
                host::instruction::SetCoprocessorSigners {
                    signers: vec![[*signer; 20]],
                    threshold: 1,
                }
                .data(),
            ),
            Action::SetMaxHcuPerTx { admin, value } => host_admin(
                admin,
                host::instruction::SetMaxHcuPerTx { value: *value }.data(),
            ),
            Action::SetMaxHcuDepthPerTx { admin, value } => host_admin(
                admin,
                host::instruction::SetMaxHcuDepthPerTx { value: *value }.data(),
            ),
            Action::SetHcuBlockCapPerApp { admin, value } => host_admin(
                admin,
                host::instruction::SetHcuBlockCapPerApp { value: *value }.data(),
            ),
            Action::SetHcuAppTrusted {
                payer,
                admin,
                app: index,
                trusted,
            } => {
                let (admin, signs) = admin_role(admin);
                let app = app(*index);
                let ix = anchor_ix(
                    host::id(),
                    host::accounts::SetHcuAppTrusted {
                        payer: self.wallets[*payer],
                        admin,
                        host_config,
                        hcu_trusted_app_record: host::hcu_trusted_app_address(app).0,
                        system_program: system_program::ID,
                        event_authority: event_authority(host::id()),
                        program: host::id(),
                    },
                    host::instruction::SetHcuAppTrusted {
                        program: app.program,
                        scope: app.scope,
                        trusted: *trusted,
                    },
                );
                unsign(ix, admin, signs)
            }
            Action::SetDenyScope {
                payer,
                admin,
                app: index,
                denied,
            } => {
                let (admin, signs) = admin_role(admin);
                let app = app(*index);
                let ix = anchor_ix(
                    host::id(),
                    host::accounts::SetDenyScope {
                        payer: self.wallets[*payer],
                        admin,
                        host_config,
                        deny_scope_record: host::deny_scope_address(app).0,
                        system_program: system_program::ID,
                        event_authority: event_authority(host::id()),
                        program: host::id(),
                    },
                    host::instruction::SetDenyScope {
                        program: app.program,
                        scope: app.scope,
                        denied: *denied,
                    },
                );
                unsign(ix, admin, signs)
            }
            Action::DefineKmsContext { admin, context } => {
                let (admin, signs) = admin_role(admin);
                let context_id = kms_context_id(*context);
                let ix = anchor_ix(
                    host::id(),
                    host::accounts::DefineKmsContext {
                        admin,
                        host_config,
                        kms_context: host::kms_context_address(context_id).0,
                        system_program: system_program::ID,
                        event_authority: event_authority(host::id()),
                        program: host::id(),
                    },
                    host::instruction::DefineKmsContext {
                        context_id,
                        signers: vec![[0x99; 20]],
                        thresholds: host::KmsThresholds {
                            public_decryption: 1,
                            user_decryption: 1,
                            kms_gen: 1,
                            mpc: 1,
                        },
                    },
                );
                unsign(ix, admin, signs)
            }
            Action::DestroyKmsContext { admin, context } => {
                let (admin, signs) = admin_role(admin);
                let context_id = kms_context_id(*context);
                let ix = anchor_ix(
                    host::id(),
                    host::accounts::DestroyKmsContext {
                        admin,
                        host_config,
                        kms_context: host::kms_context_address(context_id).0,
                        event_authority: event_authority(host::id()),
                        program: host::id(),
                    },
                    host::instruction::DestroyKmsContext { context_id },
                );
                unsign(ix, admin, signs)
            }
            Action::CreateEncryptedStore {
                payer,
                store,
                authority: role,
            } => {
                let owner = self.store_authority(*store);
                let authority = self.key(role.key, owner.key);
                let ix = anchor_ix(
                    host::id(),
                    host::accounts::CreateEncryptedStore {
                        payer: self.wallets[*payer],
                        authority,
                        scope: self.store_scope(*store),
                        encrypted_store: self.store_address(*store),
                        host_config,
                        system_program: system_program::ID,
                    },
                    host::instruction::CreateEncryptedStore {
                        args: host::instructions::CreateEncryptedStoreArgs {
                            program: owner.program,
                            authority_seeds: vec![
                                host_fixtures::VALUE_AUTHORITY_SEED.to_vec(),
                                owner.seed_key.to_bytes().to_vec(),
                                vec![owner.bump],
                            ],
                        },
                    },
                );
                unsign(ix, authority, role.signs)
            }
            Action::FheExecute {
                payer,
                producer,
                authority,
                target,
                witness,
                make_public,
                allow_viewer,
            } => {
                return self.fhe_execute(
                    *payer,
                    *producer,
                    *authority,
                    *target,
                    *witness,
                    *make_public,
                    *allow_viewer,
                );
            }
            Action::MakeStoreHandlePublic {
                payer,
                store,
                authority: role,
            } => {
                let owner = self.store_authority(*store);
                let authority = self.key(role.key, owner.key);
                let state = self.store_state(*store);
                let handle = state
                    .as_ref()
                    .and_then(|state| state.slots.iter().find(|slot| slot.key == slot_key()))
                    .map_or([0; 32], |slot| slot.handle);
                let ix = anchor_ix(
                    host::id(),
                    host::accounts::MakeStoreHandlePublic {
                        payer: self.wallets[*payer],
                        authority,
                        encrypted_store: self.store_address(*store),
                        host_config,
                        deny_scope_record: self.deny_record(self.store_app(*store)),
                        system_program: system_program::ID,
                    },
                    host::instruction::MakeStoreHandlePublic {
                        key: slot_key(),
                        handle,
                        previous_leaf_count: state.map_or(0, |state| state.leaf_count),
                    },
                );
                unsign(ix, authority, role.signs)
            }
            Action::OpenTransientStore { payer } => {
                fhe_transaction(self.wallets[*payer], []).swap_remove(0)
            }
            Action::CloseTransientStore { payer } => fhe_transaction(self.wallets[*payer], [])
                .pop()
                .expect("close"),
            Action::DelegateForUserDecryption { delegator } => {
                let (delegator, delegate) = self.delegation(*delegator);
                let app = self.store_app(0);
                anchor_ix(
                    host::id(),
                    host::accounts::DelegateForUserDecryption {
                        payer: delegator,
                        delegator,
                        host_config,
                        scope: app.scope,
                        delegation_record: host::user_decryption_delegation_address(
                            delegator, delegate, app,
                        )
                        .0,
                        system_program: system_program::ID,
                    },
                    host::instruction::DelegateForUserDecryption {
                        delegate,
                        program: app.program,
                        expires_at: u64::MAX,
                    },
                )
            }
            Action::RevokeDelegationForUserDecryption { delegator } => {
                let (delegator, delegate) = self.delegation(*delegator);
                anchor_ix(
                    host::id(),
                    host::accounts::RevokeDelegationForUserDecryption {
                        delegator,
                        host_config,
                        delegation_record: host::user_decryption_delegation_address(
                            delegator,
                            delegate,
                            self.store_app(0),
                        )
                        .0,
                    },
                    host::instruction::RevokeDelegationForUserDecryption {},
                )
            }
            Action::RevokePermits { user } => {
                let user = self.wallets[*user];
                anchor_ix(
                    host::id(),
                    host::accounts::RevokePermits {
                        user,
                        invalidation: host::permit_invalidation_address(user).0,
                        system_program: system_program::ID,
                    },
                    host::instruction::RevokePermits {},
                )
            }
            Action::VerifyPublicDecrypt { store } => {
                let handle = self
                    .store_state(*store)
                    .and_then(|state| state.slots.first().map(|slot| slot.handle))
                    .unwrap_or([0; 32]);
                anchor_ix(
                    host::id(),
                    host::accounts::VerifyPublicDecrypt {
                        host_config,
                        kms_context: host::kms_context_address(kms_context_id(0)).0,
                        encrypted_store: self.store_address(*store),
                    },
                    host::instruction::VerifyPublicDecrypt {
                        handle,
                        cleartext: [0; 32],
                        signatures: vec![],
                        extra_data: vec![0],
                        proof: host::instructions::MmrInclusionProof {
                            leaf_index: 0,
                            siblings: vec![],
                        },
                    },
                )
            }
        };
        vec![body]
    }

    /// An execution over `producer`'s first-scope Store that writes the target Store's slot. The
    /// role fills the execution's authority, the producer when it holds; a target under another
    /// authority carries that authority as the witness says.
    #[allow(clippy::too_many_arguments)]
    fn fhe_execute(
        &self,
        payer: usize,
        producer: usize,
        role: Role,
        target: usize,
        witness: Witness,
        make_public: bool,
        allow_viewer: bool,
    ) -> Vec<Instruction> {
        let producer_store = producer * SCOPES;
        let authority = self.key(role.key, self.authorities[producer].key);
        let target_authority = self.store_authority(target);
        let mut remaining = vec![writable(self.store_address(producer_store))];
        let target_index = if target == producer_store {
            0
        } else {
            remaining.push(writable(self.store_address(target)));
            1
        };
        if target_authority.key != authority {
            match witness {
                Witness::Absent => {}
                Witness::Unsigned => remaining.push(readonly(target_authority.key)),
                Witness::Signed => remaining.push(readonly_signer(target_authority.key)),
            }
        }
        let mut apps = vec![self.store_app(producer_store), self.store_app(target)];
        apps.dedup();
        remaining.extend(
            apps.into_iter()
                .filter_map(|app| self.deny_record(app))
                .map(readonly),
        );

        let state = self.store_state(target);
        let previous_handle = state
            .as_ref()
            .and_then(|state| state.slots.iter().find(|slot| slot.key == slot_key()))
            .map(|slot| slot.handle);
        let allows = if allow_viewer {
            vec![self.wallets[1]]
        } else {
            vec![]
        };
        let mut dictionary = ExecutionDictionary::default();
        let effect = target_authority.store_output(
            &mut dictionary,
            target_index,
            slot_key(),
            &allows,
            previous_handle,
            state.map_or(0, |state| state.leaf_count),
            make_public,
        );
        let payer = self.wallets[payer];
        let ix = fhe_execute_ix(
            payer,
            authority,
            host::host_config_address().0,
            FheExecuteArgs {
                execution_store_index: 0,
                effects: vec![effect],
                returned_results: vec![],
                account_count: 0,
                dictionary: dictionary.into_entries(),
                steps: vec![FheExecuteStep::TrivialEncrypt {
                    plaintext: [7; 32],
                    fhe_type: 5,
                }],
            },
            remaining,
        );
        fhe_transaction(payer, [unsign(ix, authority, role.signs)])
    }

    /// Sends one action, checks H1 and H2 on the accounts it changed, and reports whether the
    /// transaction succeeded.
    fn step(&mut self, action: &Action) -> Result<bool, TestCaseError> {
        let slot = self.context.mollusk.sysvars.clock.slot + 1;
        self.context.mollusk.warp_to_slot(slot);

        let transaction = self.transaction(action);
        let body = if transaction.len() == 1 {
            &transaction[0]
        } else {
            &transaction[1]
        };
        prop_assert!(
            body.data
                .starts_with(&idl_discriminators()[action.instruction_name()]),
            "{action:?} does not send {}",
            action.instruction_name()
        );
        let signers: BTreeSet<Pubkey> = transaction
            .iter()
            .flat_map(|ix| ix.accounts.iter())
            .filter(|meta| meta.is_signer)
            .map(|meta| meta.pubkey)
            .collect();

        let before = self.host_accounts()?;
        let result = self.context.process_transaction_instructions(&transaction);
        let after = self.host_accounts()?;

        let mut config_changed = false;
        for (root, versions) in changed(&before.trust_roots, &after.trust_roots) {
            prop_assert!(
                signers.contains(&self.admin)
                    || pauser_added_flags(&before, &after, &root, &signers),
                "H1: {action:?} changed trust root {root} without the admin {} signing",
                self.admin
            );
            config_changed |= versions
                .iter()
                .any(|data| data.starts_with(host::HostConfig::DISCRIMINATOR));
        }
        if config_changed {
            // #35: a config change stamps its slot and is announced through the event CPI.
            prop_assert_eq!(
                self.config().updated_slot,
                slot,
                "{:?} left updated_slot stale",
                action
            );
            let message = result.message.as_ref().expect("compiled message");
            let keys = message.account_keys();
            prop_assert!(
                result.inner_instructions.iter().flatten().any(|inner| {
                    keys.get(inner.instruction.program_id_index as usize) == Some(&host::id())
                        && inner
                            .instruction
                            .data
                            .starts_with(anchor_lang::event::EVENT_IX_TAG_LE)
                }),
                "{:?} changed the config without an event CPI",
                action
            );
        }
        for (store, versions) in changed(&before.stores, &after.stores) {
            // The authority recorded before and after, so rewriting it needs both.
            for mut data in versions {
                let authority = host::EncryptedStore::try_deserialize(&mut data)
                    .expect("Store decodes")
                    .authority;
                prop_assert!(
                    signers.contains(&authority),
                    "H2: {action:?} changed Store {store} without its authority {authority} signing"
                );
            }
        }

        if let (Action::SetAdmin { new_admin, .. }, Ok(())) = (action, &result.raw_result) {
            self.admin = self.key(*new_admin, self.admin);
        }
        Ok(result.raw_result.is_ok())
    }
}

/// Whether the only change to `root` is a signer with an enabled pauser record adding pause
/// flags to the config, the one trust-root change the admin need not sign.
fn pauser_added_flags(
    before: &HostAccounts,
    after: &HostAccounts,
    root: &Pubkey,
    signers: &BTreeSet<Pubkey>,
) -> bool {
    let decode = |accounts: &HostAccounts| {
        let data = accounts.trust_roots.get(root)?;
        host::HostConfig::try_deserialize(&mut data.as_slice()).ok()
    };
    let (Some(old), Some(mut new)) = (decode(before), decode(after)) else {
        return false;
    };
    let only_adds = new.paused == old.paused.with(new.paused);
    new.paused = old.paused;
    new.updated_slot = old.updated_slot;
    let serialize = |config: &host::HostConfig| {
        let mut data = Vec::new();
        config
            .try_serialize(&mut data)
            .expect("HostConfig serializes");
        data
    };
    let enabled_pauser = signers.iter().any(|signer| {
        before
            .trust_roots
            .get(&host::pauser_address(*signer).0)
            .and_then(|data| host::PauserRecord::try_deserialize(&mut data.as_slice()).ok())
            .is_some_and(|record| record.enabled)
    });
    only_adds && serialize(&new) == serialize(&old) && enabled_pauser
}

fn config() -> Config {
    Config {
        cases: 128,
        // A failure prints its shrunk sequence; no regression file is written into the tree.
        failure_persistence: None,
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(config())]

    #[test]
    fn only_the_admin_changes_trust_roots_and_only_an_authority_changes_its_store(
        actions in prop::collection::vec(action(), 1..48)
    ) {
        let mut world = World::new();
        for action in &actions {
            world.step(action)?;
        }
    }
}

#[test]
fn every_host_instruction_has_a_generator() {
    let mut runner = TestRunner::deterministic();
    let strategy = action();
    let generated: BTreeSet<&str> = (0..4_096)
        .map(|_| {
            strategy
                .new_tree(&mut runner)
                .expect("action")
                .current()
                .instruction_name()
        })
        .collect();
    let idl: BTreeSet<&str> = idl_discriminators().keys().map(String::as_str).collect();
    assert_eq!(generated, idl);
}

/// The sequences must reach every write the oracles guard, or the properties would hold only
/// because every write fails. The four instructions left out cannot succeed alone: the config
/// already exists, the transient store opens and closes only around an FHE call, and no
/// generated certificate is valid.
#[test]
fn generated_sequences_reach_every_state_change() {
    let mut runner = TestRunner::deterministic();
    let sequences = prop::collection::vec(action(), 1..48);
    let mut succeeded = BTreeSet::new();
    for _ in 0..64 {
        let mut world = World::new();
        for action in sequences.new_tree(&mut runner).expect("sequence").current() {
            if world.step(&action).expect("the invariants hold") {
                succeeded.insert(action.instruction_name());
            }
        }
    }
    let never = [
        "initialize_host_config",
        "open_transient_store",
        "close_transient_store",
        "verify_public_decrypt",
    ];
    let expected: BTreeSet<&str> = idl_discriminators()
        .keys()
        .map(String::as_str)
        .filter(|name| !never.contains(name))
        .collect();
    assert_eq!(succeeded, expected);
}
