//! Semantic compute backends: cleartext today, worker/async in tfhe-worker tests.

use litesvm::types::TransactionMetadata;
use solana_sdk::pubkey::Pubkey;

use crate::{cleartext::Handle, scenarios::TransferScenario};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransferExpect {
    pub alice: u64,
    pub bob: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("cleartext backend: {0}")]
    Cleartext(String),
    #[error("handle {handle:?} has no decrypted value")]
    MissingHandle { handle: Handle },
    #[error("unexpected cleartext type for handle {handle:?}")]
    UnexpectedType { handle: Handle },
}

/// Ingest `emit_cpi!` host events and answer semantic decrypt queries.
pub trait SemanticBackend {
    fn seed_u64(&mut self, handle: Handle, value: u64);
    fn ingest_host_transaction(
        &mut self,
        meta: &TransactionMetadata,
        account_keys: &[Pubkey],
        program_id: Pubkey,
    ) -> Result<(), BackendError>;
    fn decrypt_u64(&self, handle: Handle) -> Result<u64, BackendError>;
}

pub fn assert_transfer_semantics(alice: u64, bob: u64, expect: TransferExpect) {
    assert_eq!(alice, expect.alice, "alice confidential balance");
    assert_eq!(bob, expect.bob, "bob confidential balance");
}

pub fn decrypt_transfer_balances(
    backend: &impl SemanticBackend,
    scenario: &TransferScenario,
) -> Result<(u64, u64), BackendError> {
    Ok((
        backend.decrypt_u64(scenario.new_alice_handle)?,
        backend.decrypt_u64(scenario.new_bob_handle)?,
    ))
}

pub fn seed_transfer_inputs(
    backend: &mut impl SemanticBackend,
    scenario: &TransferScenario,
    alice_initial: u64,
    bob_initial: u64,
    amount: u64,
) {
    backend.seed_u64(scenario.alice_before, alice_initial);
    backend.seed_u64(scenario.bob_before, bob_initial);
    backend.seed_u64(scenario.amount_handle, amount);
}

/// Run a transfer on LiteSVM and assert balances via the cleartext semantic backend.
pub fn assert_transfer_cleartext(
    fixture: &mut crate::fixture::TokenFixture,
    setup: crate::scenarios::TransferSetup,
    alice_initial: u64,
    bob_initial: u64,
    expect: TransferExpect,
) -> TransferScenario {
    use crate::cleartext::CleartextBackend;
    use crate::scenarios::run_transfer_scenario;

    let scenario = run_transfer_scenario(fixture, setup);
    let mut backend = CleartextBackend::default();
    seed_transfer_inputs(
        &mut backend,
        &scenario,
        alice_initial,
        bob_initial,
        setup.amount,
    );
    backend
        .ingest_host_transaction(
            &scenario.meta,
            &scenario.account_keys,
            scenario.host_program_id,
        )
        .expect("cleartext ingest");
    let (alice, bob) = decrypt_transfer_balances(&backend, &scenario).expect("cleartext decrypt");
    assert_transfer_semantics(alice, bob, expect);
    scenario
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComputeBackendKind {
    Cleartext,
    Worker,
}

pub fn compute_backend_kind_from_env() -> ComputeBackendKind {
    match std::env::var("FHE_COMPUTE_BACKEND").as_deref() {
        Ok("worker") => ComputeBackendKind::Worker,
        _ => ComputeBackendKind::Cleartext,
    }
}
