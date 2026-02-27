use crate::{
    blockchain::ScheduledTransaction,
    mock_server::{self, rpc_types::Response, MockServer},
    pattern_matcher::UsageLimit,
};
use alloy::primitives::{Address, Bytes, B256, U256};
use alloy::primitives::{Log, LogData};
use alloy::sol_types::{SolCall, SolEvent};
use rand::{Rng, RngExt};
use std::{
    str::FromStr,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};
use tracing::{debug, info};

// Re-export FHEVM bindings for convenience
pub use fhevm_gateway_bindings::decryption::Decryption;
pub use fhevm_gateway_bindings::input_verification::InputVerification;

/// Selects between direct and delegated user decryption mock patterns.
#[derive(Debug, Clone, Copy)]
pub enum UserDecryptKind {
    Direct,
    Delegated,
}

impl UserDecryptKind {
    fn selector(&self) -> [u8; 4] {
        match self {
            Self::Direct => Decryption::userDecryptionRequestCall::SELECTOR,
            Self::Delegated => Decryption::delegatedUserDecryptionRequestCall::SELECTOR,
        }
    }
}

// Use actual gateway binding events instead of custom ones
// Individual responses use: Decryption::UserDecryptionResponse
// Consensus events use the topic hash from get_consensus_event_topic()

// Constants
const RESPONSE_DELAY_MS: u64 = 500;
const BLOCK_DELAY_1_MS: u64 = 500; // Block N+1: 3 events
const BLOCK_DELAY_2_MS: u64 = 1000; // Block N+2: 3 events
const BLOCK_DELAY_3_MS: u64 = 1500; // Block N+3: 3 events
const BLOCK_DELAY_4_MS: u64 = 2000; // Block N+4: 1 consensus event
const MOCK_CHAIN_ID: u64 = 1337;
const MOCK_PUBLIC_KEY_SIZE: usize = 32;
const MOCK_SIGNATURE_SIZE: usize = 65;

// Mock data generation helpers

/// Generate a random hash for transaction IDs
fn random_hash() -> B256 {
    B256::from(rand::rng().random::<[u8; 32]>())
}

// Predicate helpers for pattern matching
/// Helper function to create predicates matching contract address and function selector
fn matches_contract_and_selector_for_call(
    contract: Address,
    selector: [u8; 4],
) -> impl Fn(&crate::mock_server::CallParams) -> bool + Send + Sync + 'static {
    move |params: &crate::mock_server::CallParams| -> bool {
        params.to == contract && params.input.len() >= 4 && params.input[0..4] == selector
    }
}

/// Helper function to create predicates matching contract address and function selector
fn matches_contract_and_selector_for_txn(
    contract: Address,
    selector: [u8; 4],
) -> impl Fn(&crate::mock_server::TxParams) -> bool + Send + Sync + 'static {
    move |params: &crate::mock_server::TxParams| -> bool {
        params.to == Some(contract) && params.data.len() >= 4 && params.data[0..4] == selector
    }
}

/// FHEVM mock wrapper with simplified direct API
#[derive(Clone)]
pub struct FhevmMockWrapper {
    json_rpc_server: MockServer,
    next_zk_proof_id: Arc<AtomicU64>,
    pub decryption_contract: Address,
    pub input_proof_contract: Address,
}

impl FhevmMockWrapper {
    /// Create new FHEVM mock wrapper with contract addresses
    pub fn new(
        json_rpc_server: MockServer,
        decryption_contract: Address,
        input_proof_contract: Address,
    ) -> Self {
        info!(
            decryption_contract = %decryption_contract,
            input_proof_contract = %input_proof_contract,
            "Initializing FHEVM mock wrapper"
        );

        // Set up contract bytecode in the blockchain state
        info!(
            decryption_contract = %decryption_contract,
            bytecode_len = Decryption::DEPLOYED_BYTECODE.len(),
            "Setting decryption contract bytecode"
        );
        json_rpc_server.set_code(decryption_contract, Decryption::DEPLOYED_BYTECODE.clone());

        info!(
            input_proof_contract = %input_proof_contract,
            bytecode_len = InputVerification::DEPLOYED_BYTECODE.len(),
            "Setting input proof contract bytecode"
        );
        json_rpc_server.set_code(
            input_proof_contract,
            InputVerification::DEPLOYED_BYTECODE.clone(),
        );

        Self {
            json_rpc_server,
            next_zk_proof_id: Arc::new(
                AtomicU64::new(rand::rng().random_range(0..u16::MAX) as u64), // Use u16 max, so that incrementing by 1 will not hit u32 max anytime.
            ),
            decryption_contract,
            input_proof_contract,
        }
    }

    pub fn next_decryption_id(&self) -> U256 {
        U256::from(rand::random::<u64>())
    }

    pub fn next_zk_proof_id(&self) -> u64 {
        self.next_zk_proof_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Configure readiness checks to return false (simulating not ready state)
    pub fn set_readiness_failure(&self) {
        debug!("Configuring readiness checks to return false");
        self.register_readiness_patterns(false);
    }

    /// Configure readiness checks to return true (simulating ready state)
    pub fn set_readiness_success(&self) {
        debug!("Configuring readiness checks to return true");
        self.register_readiness_patterns(true);
    }

    /// Configure readiness checks to return a JSON-RPC error (simulating node unavailable / contract error).
    /// This causes `ReadinessCheckError::ContractError` after max retries, dispatching `ReadinessCheckFailed`.
    pub fn set_readiness_contract_error(&self) {
        debug!("Configuring readiness checks to return RPC error");

        let error_response = Response::Error("RPC error: node unavailable".to_string());

        // Register public decryption readiness check error
        self.json_rpc_server.on_call(
            matches_contract_and_selector_for_call(
                self.decryption_contract,
                Decryption::isPublicDecryptionReadyCall::SELECTOR,
            ),
            error_response.clone(),
            UsageLimit::Unlimited,
        );

        // Register user decryption readiness check error
        self.json_rpc_server.on_call(
            matches_contract_and_selector_for_call(
                self.decryption_contract,
                Decryption::isUserDecryptionReadyCall::SELECTOR,
            ),
            error_response,
            UsageLimit::Unlimited,
        );
    }

    /// Queue a sequence of transaction responses for a contract + selector (first-match-wins, Once per response).
    pub fn queue_tx_responses_for_selector(
        &self,
        contract: Address,
        selector: [u8; 4],
        responses: Vec<Response>,
    ) {
        for response in responses {
            self.json_rpc_server.on_transaction(
                matches_contract_and_selector_for_txn(contract, selector),
                response,
                UsageLimit::Once,
            );
        }
    }

    /// Queue call responses (e.g., estimateGas) for a contract + selector.
    pub fn queue_call_responses_for_selector(
        &self,
        contract: Address,
        selector: [u8; 4],
        responses: Vec<Response>,
    ) {
        for response in responses {
            self.json_rpc_server.on_call(
                matches_contract_and_selector_for_call(contract, selector),
                response,
                UsageLimit::Once,
            );
        }
    }

    /// Configure readiness checks to fail `n` times (Once each), then succeed (Unlimited).
    /// Useful for holding multiple requests in the readiness retry loop so they all
    /// pass readiness at roughly the same time on the next retry cycle.
    pub fn set_readiness_success_after_n_failures(&self, n: usize) {
        debug!(
            n,
            "Configuring readiness checks: {} failures then success", n
        );

        // Register n Once(false) patterns — consumed one per eth_call
        for _ in 0..n {
            self.register_readiness_patterns_with_limit(false, UsageLimit::Once);
        }
        // Then Unlimited(true) — all subsequent calls succeed
        self.register_readiness_patterns_with_limit(true, UsageLimit::Unlimited);
    }

    /// Register readiness check patterns directly with the mock server
    fn register_readiness_patterns(&self, ready: bool) {
        self.register_readiness_patterns_with_limit(ready, UsageLimit::Unlimited);
    }

    /// Register readiness check patterns with a specific usage limit
    fn register_readiness_patterns_with_limit(&self, ready: bool, usage_limit: UsageLimit) {
        let response_value = if ready {
            "0x0000000000000000000000000000000000000000000000000000000000000001"
        } else {
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        };

        let readiness_response = Response::Success {
            hash: None,
            data: crate::mock_server::ResponseData::Bytes(Bytes::from_str(response_value).unwrap()),
            scheduled_transactions: Vec::new(),
        };

        // Register public decryption readiness check
        self.json_rpc_server.on_call(
            matches_contract_and_selector_for_call(
                self.decryption_contract,
                Decryption::isPublicDecryptionReadyCall::SELECTOR,
            ),
            readiness_response.clone(),
            usage_limit,
        );

        // Register user decryption readiness check
        self.json_rpc_server.on_call(
            matches_contract_and_selector_for_call(
                self.decryption_contract,
                Decryption::isUserDecryptionReadyCall::SELECTOR,
            ),
            readiness_response,
            usage_limit,
        );
    }

    // Generic setup methods to eliminate duplication

    /// Generic registration method for decryption patterns with subscription targeting
    fn register_decrypt_pattern<F>(
        &self,
        operation_type: &str,
        contract: Address,
        selector: [u8; 4],
        target: mock_server::SubscriptionTarget,
        usage_limit: UsageLimit,
        log_creator: F,
    ) where
        F: FnOnce(U256, Address) -> (Log, Log),
    {
        let id = self.next_decryption_id();
        debug!(decryption_id = %id, "Registering {} pattern", operation_type);
        let (request_log, response_log) = log_creator(id, contract);
        self.register_pattern(
            contract,
            selector,
            request_log,
            response_log,
            target,
            usage_limit,
        );
    }

    // Public API methods — User Decryption (direct + delegated)

    /// Register user decryption that succeeds with the multi-response pattern.
    /// Emits events across multiple blocks using 3-3-3-1 pattern + consensus.
    pub fn on_user_decrypt_success(
        &self,
        kind: UserDecryptKind,
        handles: Vec<B256>,
        user: Address,
        target: mock_server::SubscriptionTarget,
    ) {
        self.register_user_decrypt_success(
            kind.selector(),
            handles,
            user,
            vec![target],
            UsageLimit::Unlimited,
        );
    }

    /// Same as `on_user_decrypt_success` but allows custom per-event targets (cycled if shorter).
    ///
    /// User decryption emits **10 events** (3+3+3+1 block pattern). If fewer targets are provided,
    /// they cycle. Example: `[Only([0]), Only([1])]` becomes `[0,1,0,1,0,1,0,1,0,1]` across the 10 events.
    /// Uses Once usage limit for redundancy tests that register multiple patterns.
    pub fn on_user_decrypt_success_with_targets(
        &self,
        kind: UserDecryptKind,
        handles: Vec<B256>,
        user: Address,
        targets: Vec<mock_server::SubscriptionTarget>,
    ) {
        self.register_user_decrypt_success(
            kind.selector(),
            handles,
            user,
            targets,
            UsageLimit::Once,
        );
    }

    /// Register user decryption that reverts with specified reason.
    pub fn on_user_decrypt_revert(&self, kind: UserDecryptKind, reason: &str) {
        self.register_user_decrypt_revert(kind.selector(), reason);
    }

    // Shared internals for user / delegated-user decryption

    /// Internal: register a user-decrypt success pattern for the given TX selector.
    fn register_user_decrypt_success(
        &self,
        selector: [u8; 4],
        handles: Vec<B256>,
        user: Address,
        targets: Vec<mock_server::SubscriptionTarget>,
        usage_limit: UsageLimit,
    ) {
        // Set up readiness check patterns to return true (ready)
        self.set_readiness_success();

        let id = self.next_decryption_id();
        debug!(
            decryption_id = %id,
            "Registering user decryption success with multi-block pattern"
        );

        // Generate mock data for 9 shares and signatures
        let user_shares = generate_mock_user_shares(9);
        let signatures = generate_mock_signatures(9);
        let extra_data = Bytes::from(vec![0x00]); // Same extraData for all events in a decryption

        // Build the request log (immediate response)
        let request_log = build_user_decrypt_request(self.decryption_contract, id, user, handles);

        // Build events using hard-coded 3-3-3-1 block pattern (targets resolved later)
        let events: Vec<(Duration, Log)> = vec![
            // Block N+1: 3 individual events (indexShare 0, 1, 2)
            (
                Duration::from_millis(BLOCK_DELAY_1_MS),
                build_individual_user_decrypt_response(
                    self.decryption_contract,
                    id,
                    0,
                    user_shares[0].clone(),
                    signatures[0].clone(),
                    extra_data.clone(),
                ),
            ),
            (
                Duration::from_millis(BLOCK_DELAY_1_MS),
                build_individual_user_decrypt_response(
                    self.decryption_contract,
                    id,
                    1,
                    user_shares[1].clone(),
                    signatures[1].clone(),
                    extra_data.clone(),
                ),
            ),
            (
                Duration::from_millis(BLOCK_DELAY_1_MS),
                build_individual_user_decrypt_response(
                    self.decryption_contract,
                    id,
                    2,
                    user_shares[2].clone(),
                    signatures[2].clone(),
                    extra_data.clone(),
                ),
            ),
            // Block N+2: 3 individual events (indexShare 3, 4, 5)
            (
                Duration::from_millis(BLOCK_DELAY_2_MS),
                build_individual_user_decrypt_response(
                    self.decryption_contract,
                    id,
                    3,
                    user_shares[3].clone(),
                    signatures[3].clone(),
                    extra_data.clone(),
                ),
            ),
            (
                Duration::from_millis(BLOCK_DELAY_2_MS),
                build_individual_user_decrypt_response(
                    self.decryption_contract,
                    id,
                    4,
                    user_shares[4].clone(),
                    signatures[4].clone(),
                    extra_data.clone(),
                ),
            ),
            (
                Duration::from_millis(BLOCK_DELAY_2_MS),
                build_individual_user_decrypt_response(
                    self.decryption_contract,
                    id,
                    5,
                    user_shares[5].clone(),
                    signatures[5].clone(),
                    extra_data.clone(),
                ),
            ),
            // Block N+3: 3 individual events (indexShare 6, 7, 8)
            (
                Duration::from_millis(BLOCK_DELAY_3_MS),
                build_individual_user_decrypt_response(
                    self.decryption_contract,
                    id,
                    6,
                    user_shares[6].clone(),
                    signatures[6].clone(),
                    extra_data.clone(),
                ),
            ),
            (
                Duration::from_millis(BLOCK_DELAY_3_MS),
                build_individual_user_decrypt_response(
                    self.decryption_contract,
                    id,
                    7,
                    user_shares[7].clone(),
                    signatures[7].clone(),
                    extra_data.clone(),
                ),
            ),
            (
                Duration::from_millis(BLOCK_DELAY_3_MS),
                build_individual_user_decrypt_response(
                    self.decryption_contract,
                    id,
                    8,
                    user_shares[8].clone(),
                    signatures[8].clone(),
                    extra_data.clone(),
                ),
            ),
            // Block N+4: 1 consensus event
            (
                Duration::from_millis(BLOCK_DELAY_4_MS),
                build_user_decrypt_threshold_reached(self.decryption_contract, id),
            ),
        ];

        // Resolve per-event targets (cycle supplied list, default to All)
        let event_targets = self.resolve_targets(events.len(), targets);

        // Create scheduled transaction with multiple events
        let scheduled_tx = ScheduledTransaction {
            target_address: Some(self.decryption_contract),
            response_events: events
                .into_iter()
                .zip(event_targets)
                .map(|((delay, log), target)| (delay, log, target))
                .collect(),
        };

        // Create immediate response with request log and scheduled transaction
        let immediate_response = Response::Success {
            hash: Some(random_hash()),
            data: crate::mock_server::ResponseData::Logs(vec![request_log]),
            scheduled_transactions: vec![scheduled_tx],
        };

        // Set up default readiness patterns (ready state)
        self.register_readiness_patterns(true);

        // Register pattern that returns immediate response with scheduled transaction
        self.json_rpc_server.on_transaction(
            matches_contract_and_selector_for_txn(self.decryption_contract, selector),
            immediate_response,
            usage_limit,
        );

        debug!("Registered FHEVM user decryption pattern with multi-block scheduled responses");
    }

    /// Internal: register a user-decrypt revert pattern for the given TX selector.
    fn register_user_decrypt_revert(&self, selector: [u8; 4], reason: &str) {
        self.json_rpc_server.on_transaction(
            matches_contract_and_selector_for_txn(self.decryption_contract, selector),
            Response::Revert {
                hash: Some(random_hash()),
                reason: Some(reason.to_string()),
            },
            UsageLimit::Unlimited,
        );
    }

    /// Register public decryption that succeeds with the provided values
    pub fn on_public_decrypt_success(
        &self,
        handles: Vec<B256>,
        values: Vec<u64>,
        target: mock_server::SubscriptionTarget,
    ) {
        // Set up readiness check patterns to return true (ready)
        self.set_readiness_success();

        // Register the transaction pattern with Once limit for redundancy tests
        // (each pattern registration in a loop should be consumed once)
        self.register_decrypt_pattern(
            "public decryption success",
            self.decryption_contract,
            Decryption::publicDecryptionRequestCall::SELECTOR,
            target,
            UsageLimit::Once,
            |id, contract| {
                let request_log = build_public_decrypt_request(contract, id, handles);
                let response_log = build_public_decrypt_response(contract, id, values, true);
                (request_log, response_log)
            },
        );
    }

    /// Register public decryption that reverts
    pub fn on_public_decrypt_revert(&self, reason: &str) {
        self.json_rpc_server.on_transaction(
            matches_contract_and_selector_for_txn(
                self.decryption_contract,
                Decryption::publicDecryptionRequestCall::SELECTOR,
            ),
            Response::Revert {
                hash: Some(random_hash()),
                reason: Some(reason.to_string()),
            },
            UsageLimit::Unlimited,
        );
    }

    /// Register successful input proof verification with dynamic ID generation
    pub fn on_input_proof_success(
        &self,
        user: Address,
        data: Bytes,
        count: usize,
        target: mock_server::SubscriptionTarget,
    ) {
        self.register_dynamic_input_proof_pattern(user, data, true, count, target);
    }

    /// Register input proof rejection with dynamic ID generation
    pub fn on_input_proof_error(&self, user: Address, data: Bytes, count: usize) {
        self.register_dynamic_input_proof_pattern(
            user,
            data,
            false,
            count,
            mock_server::SubscriptionTarget::All,
        );
    }

    /// Register unique input proof patterns, each generating a different zkProofId
    fn register_dynamic_input_proof_pattern(
        &self,
        user: Address,
        data: Bytes,
        success: bool,
        count: usize,
        target: mock_server::SubscriptionTarget,
    ) {
        for _i in 0..count {
            let id = self.next_zk_proof_id();
            let request_log =
                build_input_request(self.input_proof_contract, id, user, data.clone());
            let response_log = if success {
                build_input_success_response(self.input_proof_contract, id, vec![])
            } else {
                build_input_reject_response(self.input_proof_contract, id)
            };

            let scheduled_tx = ScheduledTransaction {
                target_address: Some(self.input_proof_contract),
                response_events: vec![(
                    Duration::from_millis(RESPONSE_DELAY_MS),
                    response_log,
                    target.clone(),
                )],
            };

            let response = Response::Success {
                hash: Some(random_hash()),
                data: crate::mock_server::ResponseData::Logs(vec![request_log]),
                scheduled_transactions: vec![scheduled_tx],
            };

            self.json_rpc_server.on_transaction(
                matches_contract_and_selector_for_txn(
                    self.input_proof_contract,
                    InputVerification::verifyProofRequestCall::SELECTOR,
                ),
                response,
                UsageLimit::Once,
            );
        }
    }

    /// Register input proof request event only (for timeout testing)
    /// Emits the request event but NO response event, causing the relayer to timeout
    pub fn on_input_proof_request_only(&self, user: Address, data: Bytes) {
        let id = self.next_zk_proof_id();
        let request_log = build_input_request(self.input_proof_contract, id, user, data);

        let response = Response::Success {
            hash: Some(random_hash()),
            data: crate::mock_server::ResponseData::Logs(vec![request_log]),
            scheduled_transactions: vec![], // NO response event - will timeout
        };

        self.json_rpc_server.on_transaction(
            matches_contract_and_selector_for_txn(
                self.input_proof_contract,
                InputVerification::verifyProofRequestCall::SELECTOR,
            ),
            response,
            UsageLimit::Once,
        );
    }

    /// Register input proof that reverts
    pub fn on_input_proof_revert(&self, reason: &str) {
        self.json_rpc_server.on_transaction(
            matches_contract_and_selector_for_txn(
                self.input_proof_contract,
                InputVerification::verifyProofRequestCall::SELECTOR,
            ),
            Response::Revert {
                hash: Some(random_hash()),
                reason: Some(reason.to_string()),
            },
            UsageLimit::Unlimited,
        );
    }

    /// Register public decryption request event only (for timeout testing)
    /// Emits the request event but NO response event, causing the relayer to timeout
    pub fn on_public_decrypt_request_only(&self, handles: Vec<B256>) {
        // Set up readiness check to return true (ready)
        self.set_readiness_success();

        let id = self.next_decryption_id();
        let request_log = build_public_decrypt_request(self.decryption_contract, id, handles);
        self.register_request_only(
            self.decryption_contract,
            Decryption::publicDecryptionRequestCall::SELECTOR,
            request_log,
        );
    }

    /// Register user decryption request event only (for timeout testing).
    /// Emits the request event but NO response event, causing the relayer to timeout.
    pub fn on_user_decrypt_request_only(
        &self,
        kind: UserDecryptKind,
        handles: Vec<B256>,
        user: Address,
    ) {
        self.register_user_decrypt_request_only(kind.selector(), handles, user);
    }

    /// Internal: register a request-only pattern (no response) for the given TX selector.
    fn register_user_decrypt_request_only(
        &self,
        selector: [u8; 4],
        handles: Vec<B256>,
        user: Address,
    ) {
        self.set_readiness_success();
        let id = self.next_decryption_id();
        let request_log = build_user_decrypt_request(self.decryption_contract, id, user, handles);
        self.register_request_only(self.decryption_contract, selector, request_log);
    }

    /// Register user decryption that fails with error response
    pub fn on_user_decrypt_error(&self, kind: UserDecryptKind, handles: Vec<B256>, user: Address) {
        self.register_decrypt_pattern(
            "user decryption error",
            self.decryption_contract,
            kind.selector(),
            mock_server::SubscriptionTarget::All,
            UsageLimit::Once,
            |id, contract| {
                let request_log = build_user_decrypt_request(contract, id, user, handles);
                let response_log = build_user_decrypt_response(contract, id, vec![]); // Error response has empty decrypted shares
                (request_log, response_log)
            },
        );
    }

    /// Register public decryption that fails with error response
    pub fn on_public_decrypt_error(&self, handles: Vec<B256>) {
        self.register_decrypt_pattern(
            "public decryption error",
            self.decryption_contract,
            Decryption::publicDecryptionRequestCall::SELECTOR,
            mock_server::SubscriptionTarget::All,
            UsageLimit::Once,
            |id, contract| {
                let request_log = build_public_decrypt_request(contract, id, handles);
                let response_log = build_public_decrypt_response(contract, id, vec![], false); // Error response uses success=false
                (request_log, response_log)
            },
        );
    }

    /// Expand a target list to match the number of events by cycling.
    ///
    /// # Examples
    ///
    /// - Empty list → all events use `SubscriptionTarget::All` (default)
    /// - One target → all events use that same target
    /// - Multiple targets → cycle through them repeatedly
    ///
    /// ```text
    /// resolve_targets(10, [Only([0]), Only([1])])
    ///   → [Only([0]), Only([1]), Only([0]), Only([1]), Only([0]), ...]  (10 total)
    /// ```
    ///
    /// This enables tests to simulate patterns like "events alternate between listener 0 and 1".
    fn resolve_targets(
        &self,
        event_count: usize,
        targets: Vec<mock_server::SubscriptionTarget>,
    ) -> Vec<mock_server::SubscriptionTarget> {
        if targets.is_empty() {
            return vec![mock_server::SubscriptionTarget::All; event_count];
        }
        if targets.len() == 1 {
            return vec![targets[0].clone(); event_count];
        }
        targets.into_iter().cycle().take(event_count).collect()
    }

    // Internal helper methods

    /// Generic pattern registration for all FHEVM operations
    fn register_pattern(
        &self,
        contract: Address,
        selector: [u8; 4],
        request_log: Log,
        response_log: Log,
        target: mock_server::SubscriptionTarget,
        usage_limit: UsageLimit,
    ) {
        // Create scheduled transaction for delayed response with target
        let scheduled_tx = ScheduledTransaction {
            target_address: Some(contract),
            response_events: vec![(
                Duration::from_millis(RESPONSE_DELAY_MS),
                response_log,
                target,
            )],
        };

        // Create immediate response with request log and scheduled transaction
        let immediate_response = Response::Success {
            hash: Some(random_hash()),
            data: crate::mock_server::ResponseData::Logs(vec![request_log]),
            scheduled_transactions: vec![scheduled_tx],
        };

        // Register pattern that returns immediate response with scheduled transaction
        self.json_rpc_server.on_transaction(
            matches_contract_and_selector_for_txn(contract, selector),
            immediate_response,
            usage_limit,
        );

        debug!("Registered FHEVM pattern with scheduled response and subscription targeting");
    }

    /// Register request event only (for timeout testing)
    /// Emits just the request event in transaction receipt, NO response event
    fn register_request_only(&self, contract: Address, selector: [u8; 4], request_log: Log) {
        let immediate_response = Response::Success {
            hash: Some(random_hash()),
            data: crate::mock_server::ResponseData::Logs(vec![request_log]),
            scheduled_transactions: vec![], // NO response - will timeout
        };

        self.json_rpc_server.on_transaction(
            matches_contract_and_selector_for_txn(contract, selector),
            immediate_response,
            UsageLimit::Once,
        );
        debug!("Registered FHEVM request-only pattern (no response, for timeout testing)");
    }
}

// Test utility functions

/// Generate a random ciphertext handle for testing
pub fn generate_ciphertext_handle() -> B256 {
    let mut rng = rand::rng();
    let mut handle = [0u8; 32];
    rng.fill_bytes(&mut handle);
    B256::from(handle)
}

/// Generate a mock plaintext value for testing
pub fn generate_mock_plaintext() -> u64 {
    rand::rng().random_range(1..=5000000)
}

// Helper functions for event creation

// Direct utility functions replacing LogBuilder abstraction

fn build_event_log<T: SolEvent>(contract: Address, event: &T, topics: Vec<B256>) -> Log {
    let event_data = event.encode_data();
    Log {
        address: contract,
        data: LogData::new_unchecked(topics, event_data.into()),
    }
}

fn build_user_decrypt_request(
    contract: Address,
    decryption_id: U256,
    user_address: Address,
    handles: Vec<B256>,
) -> Log {
    let request = Decryption::UserDecryptionRequest {
        decryptionId: decryption_id,
        snsCtMaterials: create_sns_materials(handles),
        userAddress: user_address,
        publicKey: Bytes::from(vec![0x00; MOCK_PUBLIC_KEY_SIZE]),
        extraData: Bytes::from(vec![0x00]),
    };

    build_event_log(
        contract,
        &request,
        vec![
            Decryption::UserDecryptionRequest::SIGNATURE_HASH,
            B256::from(decryption_id),
        ],
    )
}

fn build_user_decrypt_response(
    contract: Address,
    decryption_id: U256,
    decrypted_shares: Vec<Bytes>,
) -> Log {
    // For old-style responses, use the first share (or empty if none)
    let first_share = decrypted_shares.first().cloned().unwrap_or_default();

    let response = Decryption::UserDecryptionResponse {
        decryptionId: decryption_id,
        indexShare: U256::from(0), // Default to index 0 for old-style responses
        userDecryptedShare: first_share,
        signature: Bytes::from(vec![0u8; MOCK_SIGNATURE_SIZE]),
        extraData: Bytes::default(),
    };

    build_event_log(
        contract,
        &response,
        vec![
            Decryption::UserDecryptionResponse::SIGNATURE_HASH,
            B256::from(decryption_id),
            B256::from(U256::from(0)), // indexShare topic
        ],
    )
}

fn build_public_decrypt_request(contract: Address, decryption_id: U256, handles: Vec<B256>) -> Log {
    let request = Decryption::PublicDecryptionRequest {
        decryptionId: decryption_id,
        snsCtMaterials: create_sns_materials(handles),
        extraData: Bytes::from(vec![0x00]),
    };

    build_event_log(
        contract,
        &request,
        vec![
            Decryption::PublicDecryptionRequest::SIGNATURE_HASH,
            B256::from(decryption_id),
        ],
    )
}

fn build_public_decrypt_response(
    contract: Address,
    decryption_id: U256,
    decrypted_values: Vec<u64>,
    success: bool,
) -> Log {
    let response = if success && !decrypted_values.is_empty() {
        Decryption::PublicDecryptionResponse {
            decryptionId: decryption_id,
            decryptedResult: Bytes::from(decrypted_values[0].to_be_bytes().to_vec()),
            signatures: vec![Bytes::from(vec![0u8; MOCK_SIGNATURE_SIZE])],
            extraData: Bytes::default(),
        }
    } else {
        Decryption::PublicDecryptionResponse {
            decryptionId: decryption_id,
            decryptedResult: Bytes::default(),
            signatures: vec![],
            extraData: Bytes::default(),
        }
    };

    build_event_log(
        contract,
        &response,
        vec![
            Decryption::PublicDecryptionResponse::SIGNATURE_HASH,
            B256::from(decryption_id),
            if success {
                B256::from([1u8; 32])
            } else {
                B256::ZERO
            },
        ],
    )
}

fn build_input_request(
    contract: Address,
    request_id: u64,
    user_address: Address,
    data: Bytes,
) -> Log {
    let request = InputVerification::VerifyProofRequest {
        zkProofId: U256::from(request_id),
        contractChainId: U256::from(MOCK_CHAIN_ID),
        contractAddress: Address::ZERO,
        userAddress: user_address,
        ciphertextWithZKProof: data,
        extraData: Bytes::from(vec![0x00]),
    };

    build_event_log(
        contract,
        &request,
        vec![
            InputVerification::VerifyProofRequest::SIGNATURE_HASH,
            B256::from(U256::from(request_id)),
            B256::from(U256::from(MOCK_CHAIN_ID)),
        ],
    )
}

fn build_input_success_response(contract: Address, request_id: u64, handles: Vec<B256>) -> Log {
    let response = InputVerification::VerifyProofResponse {
        zkProofId: U256::from(request_id),
        ctHandles: if handles.is_empty() {
            vec![B256::from([5u8; 32]), B256::from([2u8; 32])] // Mock handles
        } else {
            handles
        },
        signatures: vec![Bytes::from(vec![0x00; MOCK_SIGNATURE_SIZE])],
    };

    build_event_log(
        contract,
        &response,
        vec![
            InputVerification::VerifyProofResponse::SIGNATURE_HASH,
            B256::from(U256::from(request_id)),
        ],
    )
}

fn build_input_reject_response(contract: Address, request_id: u64) -> Log {
    let response = InputVerification::RejectProofResponse {
        zkProofId: U256::from(request_id),
    };

    build_event_log(
        contract,
        &response,
        vec![
            InputVerification::RejectProofResponse::SIGNATURE_HASH,
            B256::from(U256::from(request_id)),
        ],
    )
}

// New event builder functions for multi-response user decryption pattern

/// Build individual user decryption response event using actual gateway bindings
fn build_individual_user_decrypt_response(
    contract: Address,
    decryption_id: U256,
    index_share: u64,
    user_decrypted_share: Bytes,
    signature: Bytes,
    extra_data: Bytes,
) -> Log {
    let response = Decryption::UserDecryptionResponse {
        decryptionId: decryption_id,
        indexShare: U256::from(index_share),
        userDecryptedShare: user_decrypted_share,
        signature,
        extraData: extra_data,
    };

    build_event_log(
        contract,
        &response,
        vec![
            Decryption::UserDecryptionResponse::SIGNATURE_HASH,
            B256::from(decryption_id),
            B256::from(U256::from(index_share)),
        ],
    )
}

/// Build consensus threshold reached event using the expected topic hash
fn build_user_decrypt_threshold_reached(contract: Address, decryption_id: U256) -> Log {
    let response = Decryption::UserDecryptionResponseThresholdReached {
        decryptionId: decryption_id,
    };

    build_event_log(
        contract,
        &response,
        vec![
            Decryption::UserDecryptionResponseThresholdReached::SIGNATURE_HASH,
            B256::from(decryption_id),
        ],
    )
}

/// Generate realistic mock data for user decryption shares
fn generate_mock_user_shares(count: usize) -> Vec<Bytes> {
    (0..count)
        .map(|i| {
            let mut rng = rand::rng();
            let mut share_data = vec![0u8; 32]; // Mock 32-byte shares
            rng.fill_bytes(&mut share_data);
            // Make each share slightly different by setting the first byte to the index
            share_data[0] = i as u8;
            Bytes::from(share_data)
        })
        .collect()
}

/// Generate realistic mock signatures for user decryption
fn generate_mock_signatures(count: usize) -> Vec<Bytes> {
    (0..count)
        .map(|i| {
            let mut rng = rand::rng();
            let mut sig_data = vec![0u8; MOCK_SIGNATURE_SIZE];
            rng.fill_bytes(&mut sig_data);
            // Make each signature slightly different by setting the first byte to the index
            sig_data[0] = i as u8;
            Bytes::from(sig_data)
        })
        .collect()
}

/// Helper to create SnsCiphertextMaterial structs from handles
fn create_sns_materials(handles: Vec<B256>) -> Vec<Decryption::SnsCiphertextMaterial> {
    handles
        .iter()
        .map(|h| Decryption::SnsCiphertextMaterial {
            ctHandle: *h,
            keyId: U256::from(1),
            snsCiphertextDigest: *h,
            coprocessorTxSenderAddresses: vec![],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mock_server::MockConfig, mock_server::MockServer};

    #[test]
    fn test_fhevm_wrapper_creation_and_id_generation() {
        let server = MockServer::new(MockConfig::new());
        let decryption_addr = Address::repeat_byte(1);
        let input_addr = Address::repeat_byte(2);

        let wrapper = FhevmMockWrapper::new(server, decryption_addr, input_addr);

        assert_eq!(
            wrapper.decryption_contract, decryption_addr,
            "Decryption contract address should match"
        );
        assert_eq!(
            wrapper.input_proof_contract, input_addr,
            "Input proof contract address should match"
        );

        // Test ID generation
        let id1 = wrapper.next_decryption_id();
        let id2 = wrapper.next_decryption_id();
        assert_ne!(id1, id2, "Decryption IDs should be unique");

        let proof_id1 = wrapper.next_zk_proof_id();
        let proof_id2 = wrapper.next_zk_proof_id();
        assert!(proof_id2 > proof_id1, "ZK proof IDs should increment");
    }

    #[test]
    fn test_pattern_setup_methods() {
        // Tests if all pattern setup methods complete without panicking

        let server = MockServer::new(MockConfig::new());
        let wrapper =
            FhevmMockWrapper::new(server, Address::repeat_byte(1), Address::repeat_byte(2));

        let handles = vec![generate_ciphertext_handle()];
        let user = Address::repeat_byte(3);
        let test_data = Bytes::from("test proof data");

        // Test pattern setup methods don't panic
        wrapper.on_user_decrypt_success(
            UserDecryptKind::Direct,
            handles.clone(),
            user,
            mock_server::SubscriptionTarget::All,
        );
        wrapper.on_user_decrypt_error(UserDecryptKind::Direct, handles.clone(), user);
        wrapper.on_user_decrypt_revert(UserDecryptKind::Direct, "test reason");

        wrapper.on_public_decrypt_success(
            handles.clone(),
            vec![42],
            mock_server::SubscriptionTarget::All,
        );
        wrapper.on_public_decrypt_error(handles.clone());
        wrapper.on_public_decrypt_revert("test reason");

        wrapper.on_input_proof_success(
            user,
            test_data.clone(),
            10,
            mock_server::SubscriptionTarget::All,
        );
        wrapper.on_input_proof_error(user, test_data, 10);
        wrapper.on_input_proof_revert("test reason");
    }
}
