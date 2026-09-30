//! Plaintext views of the cleartext host build, `zama_host_cleartext.so` (zama-host's `cleartext`
//! feature). That build records the plaintext of every handle it produces in the accounts it
//! writes, so a test reads values straight from chain state: no replay, nothing to keep in sync.
//!
//! Give a fixture store its plaintexts with [`store_account`] or, once the context holds it, with
//! [`seed`] (forge-fhevm-std's `seedCleartext`); give an input its plaintext with
//! [`input_extra_data`] in its attestation. Read results with [`store_value`] or [`store_u64`].
//! A slot nobody gave a value to fails the read, as it fails the host, except that
//! [`fixture_context`] records 0 behind every fixture store slot, a fixture balance's starting
//! value: a test seeds every other value it relies on.

use std::collections::HashMap;

use anchor_lang::AccountDeserialize;
use mollusk_svm::Mollusk;
use solana_sdk::{account::Account, pubkey::Pubkey};
use zama_host::{self as host, cleartext::layout};

pub use zama_host::cleartext::Value;

use crate::{Ctx, BALANCE_FHE_TYPE};

/// The artifact name to pass to `Mollusk::add_program` for the cleartext host.
pub const HOST_PROGRAM: &str = "zama_host_cleartext";

/// [`crate::host_svm`] on the cleartext host build.
pub fn host_svm() -> Mollusk {
    let mut mollusk = crate::svm(&host::id(), HOST_PROGRAM);
    crate::set_previous_bank_hash_sysvars(&mut mollusk);
    mollusk
}

/// A `euint64` plaintext.
pub fn u64_value(value: u64) -> Value {
    Value::new(BALANCE_FHE_TYPE, value.into()).expect("euint64 is a shipped type")
}

/// An `EncryptedStore` account as the cleartext build creates it, holding `values` for its slots
/// in slot order. Slots past `values` hold no value.
pub fn store_account(state: &host::EncryptedStore, values: &[Value]) -> Account {
    assert!(
        values.len() <= state.slots.len(),
        "one value per slot at most"
    );
    let mut account = crate::encrypted_store_account(state);
    add_store_section(&mut account.data);
    for (index, value) in values.iter().enumerate() {
        layout::set_store_value(&mut account.data, index, *value).unwrap();
    }
    account
}

/// A context over `accounts` for a Mollusk running the cleartext host, where every fixture
/// `EncryptedStore` records 0 for each of its slots ([`zero_fixture_store`]). Tests [`seed`] the
/// values they assert on top.
pub fn fixture_context(mollusk: Mollusk, accounts: HashMap<Pubkey, Account>) -> Ctx {
    let context = mollusk.with_context(accounts);
    for account in context.account_store.borrow_mut().values_mut() {
        zero_fixture_store(account);
    }
    context
}

/// Records 0 behind each slot of a production-layout fixture `EncryptedStore`, the value a fixture
/// balance starts at, giving it a plaintext section first. Leaves any other account as it is.
pub fn zero_fixture_store(account: &mut Account) {
    let Some(state) = fixture_store(account) else {
        return;
    };
    for (index, slot) in state.slots.iter().enumerate() {
        if let Ok(zero) = Value::new(host::handle_fhe_type(slot.handle), 0) {
            layout::set_store_value(&mut account.data, index, zero).unwrap();
        }
    }
}

/// Records `value` behind every store slot in the context that holds `handle`, giving a
/// production-layout fixture store its plaintext section first.
pub fn seed(context: &Ctx, handle: [u8; 32], value: Value) {
    assert_eq!(
        value.fhe_type,
        host::handle_fhe_type(handle),
        "value type matches the handle"
    );
    let mut seeded = 0;
    for account in context.account_store.borrow_mut().values_mut() {
        let Some(state) = fixture_store(account) else {
            continue;
        };
        for (index, slot) in state.slots.iter().enumerate() {
            if slot.handle == handle {
                layout::set_store_value(&mut account.data, index, value).unwrap();
                seeded += 1;
            }
        }
    }
    assert!(
        seeded > 0,
        "no store in the context holds the seeded handle"
    );
}

/// [`seed`] for a `euint64` handle.
pub fn seed_u64(context: &Ctx, handle: [u8; 32], value: u64) {
    seed(context, handle, u64_value(value));
}

/// Decodes `account` as an `EncryptedStore`, giving it a plaintext section if it has none.
fn fixture_store(account: &mut Account) -> Option<host::EncryptedStore> {
    if account.owner != host::id() {
        return None;
    }
    let state = host::EncryptedStore::try_deserialize(&mut &account.data[..]).ok()?;
    if !layout::has_store_section(&account.data) {
        add_store_section(&mut account.data);
    }
    Some(state)
}

fn add_store_section(data: &mut Vec<u8>) {
    assert!(
        data.len() <= layout::STORE_SECTION_OFFSET,
        "store data reaches the section"
    );
    data.resize(layout::STORE_ACCOUNT_SIZE, 0);
    layout::init_store_section(data).unwrap();
}

/// The plaintext behind slot `key` of the store at `address`.
pub fn store_value(context: &Ctx, address: Pubkey, key: [u8; 32]) -> Value {
    let state = crate::read_encrypted_store(context, address);
    let index = state
        .slots
        .iter()
        .position(|slot| slot.key == key)
        .expect("encrypted store slot should exist");
    let accounts = context.account_store.borrow();
    let data = &accounts.get(&address).expect("store account").data;
    layout::store_value(data, index, state.slots[index].handle)
        .expect("store written by the cleartext host build")
}

/// The `euint64` plaintext behind slot `key` of the store at `address`.
pub fn store_u64(context: &Ctx, address: Pubkey, key: [u8; 32]) -> u64 {
    let value = store_value(context, address, key);
    assert_eq!(value.fhe_type, BALANCE_FHE_TYPE, "slot holds a euint64");
    value.bits as u64
}

/// The plaintext of `handle` as recorded in the store at `address`: its slot if a slot holds it,
/// else the store's history of recent results (a transfer amount, a replaced balance).
pub fn handle_value(context: &Ctx, address: Pubkey, handle: [u8; 32]) -> Value {
    let state = crate::read_encrypted_store(context, address);
    let accounts = context.account_store.borrow();
    let data = &accounts.get(&address).expect("store account").data;
    match state.slots.iter().position(|slot| slot.handle == handle) {
        Some(index) => layout::store_value(data, index, handle),
        None => layout::store_history_value(data, handle),
    }
    .expect("the cleartext host build recorded this handle in the store")
}

/// Input attestation `extra_data` carrying `values`, one per attested handle in `ct_handles` order.
pub fn input_extra_data(values: &[Value]) -> Vec<u8> {
    zama_host::cleartext::encode_input_values(values).expect("shipped input types")
}

/// Evaluates `args` natively with the host's own cleartext evaluator, the one the cleartext build
/// runs on chain. Store-slot and transient operands take their bits from `values` by handle and
/// their type from the handle, as on chain; verified inputs decode their attestation's
/// `extra_data`; rand steps use a fixed seed.
pub fn evaluate(
    args: &host::FheExecuteArgs,
    values: &std::collections::HashMap<[u8; 32], u128>,
) -> anchor_lang::Result<Vec<Value>> {
    host::cleartext::evaluate_steps(
        args,
        |operand, _| {
            let handle_index = match operand {
                host::FheExecuteOperand::StoreSlot { handle_index, .. }
                | host::FheExecuteOperand::TransientResult { handle_index, .. } => *handle_index,
                _ => unreachable!("the evaluator resolves steps, scalars and inputs itself"),
            };
            let handle = args.dictionary_bytes(handle_index)?;
            let bits = values.get(&handle).ok_or_else(|| {
                anchor_lang::error!(host::cleartext::CleartextError::ValueUnknown)
            })?;
            Value::new(host::handle_fhe_type(handle), *bits)
        },
        |step_index| Ok([step_index as u8; 16]),
    )
}
