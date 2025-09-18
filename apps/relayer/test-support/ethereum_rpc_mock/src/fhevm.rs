use crate::{
    blockchain::ScheduledTransaction,
    mock_server::{rpc_types::Response, MockServer},
    pattern_matcher::UsageLimit,
};
use alloy::primitives::{Address, Bytes, B256, U256};
use alloy::primitives::{Log, LogData};
use alloy::sol_types::{SolCall, SolEvent};
use rand::{Rng, RngCore};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::time::Duration;
use tracing::{debug, info};

// Re-export FHEVM bindings for convenience
pub use fhevm_gateway_bindings::decryption::Decryption;
pub use fhevm_gateway_bindings::input_verification::InputVerification;

// Constants
const RESPONSE_DELAY_MS: u64 = 500;
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
fn matches_contract_and_selector(
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
    next_decryption_id: Arc<AtomicU64>,
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
            next_decryption_id: Arc::new(AtomicU64::new(1)),
            next_zk_proof_id: Arc::new(AtomicU64::new(1)),
            decryption_contract,
            input_proof_contract,
        }
    }

    pub fn next_decryption_id(&self) -> u64 {
        self.next_decryption_id.fetch_add(1, Ordering::SeqCst)
    }

    pub fn next_zk_proof_id(&self) -> u64 {
        self.next_zk_proof_id.fetch_add(1, Ordering::SeqCst)
    }

    // Generic setup methods to eliminate duplication

    /// Generic registration method for decryption patterns
    fn register_decrypt_pattern<F>(
        &self,
        operation_type: &str,
        contract: Address,
        selector: [u8; 4],
        log_creator: F,
    ) where
        F: FnOnce(u64, Address) -> (Log, Log),
    {
        let id = self.next_decryption_id();
        debug!(decryption_id = id, "Registering {} pattern", operation_type);
        let (request_log, response_log) = log_creator(id, contract);
        self.register_pattern(contract, selector, request_log, response_log);
    }

    /// Generic registration method for input proof patterns
    fn register_input_pattern<F>(&self, operation_type: &str, log_creator: F)
    where
        F: FnOnce(u64, Address) -> (Log, Log),
    {
        let id = self.next_zk_proof_id();
        debug!(zk_proof_id = id, "Registering {} pattern", operation_type);
        let (request_log, response_log) = log_creator(id, self.input_proof_contract);
        self.register_pattern(
            self.input_proof_contract,
            InputVerification::verifyProofRequestCall::SELECTOR,
            request_log,
            response_log,
        );
    }

    // Public API methods

    /// Register user decryption that succeeds with the provided result
    pub fn on_user_decrypt_success(&self, handles: Vec<B256>, user: Address, result: Bytes) {
        self.register_decrypt_pattern(
            "user decryption success",
            self.decryption_contract,
            Decryption::userDecryptionRequestCall::SELECTOR,
            |id, contract| {
                let request_log = build_user_decrypt_request(contract, id, user, handles);
                let response_log = build_user_decrypt_response(contract, id, vec![result]);
                (request_log, response_log)
            },
        );
    }

    /// Register user decryption that reverts with specified reason
    pub fn on_user_decrypt_revert(&self, reason: &str) {
        self.json_rpc_server.on_transaction(
            matches_contract_and_selector(
                self.decryption_contract,
                Decryption::userDecryptionRequestCall::SELECTOR,
            ),
            Response::Revert {
                hash: Some(random_hash()),
                reason: Some(reason.to_string()),
                scheduled_transaction: None,
            },
            UsageLimit::Unlimited,
        );
    }

    /// Register public decryption that succeeds with the provided values
    pub fn on_public_decrypt_success(&self, handles: Vec<B256>, values: Vec<u64>) {
        self.register_decrypt_pattern(
            "public decryption success",
            self.decryption_contract,
            Decryption::publicDecryptionRequestCall::SELECTOR,
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
            matches_contract_and_selector(
                self.decryption_contract,
                Decryption::publicDecryptionRequestCall::SELECTOR,
            ),
            Response::Revert {
                hash: Some(random_hash()),
                reason: Some(reason.to_string()),
                scheduled_transaction: None,
            },
            UsageLimit::Unlimited,
        );
    }

    /// Register successful input proof verification
    pub fn on_input_proof_success(&self, user: Address, data: Bytes) {
        self.register_input_pattern("input proof success", |id, contract| {
            let request_log = build_input_request(contract, id, user, data);
            let response_log = build_input_success_response(contract, id, vec![]);
            (request_log, response_log)
        });
    }

    /// Register input proof rejection
    pub fn on_input_proof_error(&self, user: Address, data: Bytes) {
        self.register_input_pattern("input proof rejection", |id, contract| {
            let request_log = build_input_request(contract, id, user, data);
            let response_log = build_input_reject_response(contract, id);
            (request_log, response_log)
        });
    }

    /// Register input proof that reverts
    pub fn on_input_proof_revert(&self, reason: &str) {
        self.json_rpc_server.on_transaction(
            matches_contract_and_selector(
                self.input_proof_contract,
                InputVerification::verifyProofRequestCall::SELECTOR,
            ),
            Response::Revert {
                hash: Some(random_hash()),
                reason: Some(reason.to_string()),
                scheduled_transaction: None,
            },
            UsageLimit::Unlimited,
        );
    }

    /// Register user decryption that fails with error response
    pub fn on_user_decrypt_error(&self, handles: Vec<B256>, user: Address) {
        self.register_decrypt_pattern(
            "user decryption error",
            self.decryption_contract,
            Decryption::userDecryptionRequestCall::SELECTOR,
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
            |id, contract| {
                let request_log = build_public_decrypt_request(contract, id, handles);
                let response_log = build_public_decrypt_response(contract, id, vec![], false); // Error response uses success=false
                (request_log, response_log)
            },
        );
    }

    // Internal helper methods

    /// Generic pattern registration for all FHEVM operations
    fn register_pattern(
        &self,
        contract: Address,
        selector: [u8; 4],
        request_log: Log,
        response_log: Log,
    ) {
        // Create scheduled transaction for delayed response using the inner response log
        let scheduled_tx =
            ScheduledTransaction::new_to(Duration::from_millis(RESPONSE_DELAY_MS), contract)
                .with_response_event(response_log);

        // Create immediate response with request log and scheduled transaction
        let immediate_response = Response::Success {
            hash: Some(random_hash()),
            data: crate::mock_server::ResponseData::Logs(vec![request_log]),
            scheduled_transaction: Some(scheduled_tx),
        };

        // Register pattern that returns immediate response with scheduled transaction
        self.json_rpc_server.on_transaction(
            matches_contract_and_selector(contract, selector),
            immediate_response,
            UsageLimit::Unlimited,
        );

        debug!("Registered FHEVM pattern with scheduled response");
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
    decryption_id: u64,
    user_address: Address,
    handles: Vec<B256>,
) -> Log {
    let request = Decryption::UserDecryptionRequest {
        decryptionId: U256::from(decryption_id),
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
            B256::from(U256::from(decryption_id)),
        ],
    )
}

fn build_user_decrypt_response(
    contract: Address,
    decryption_id: u64,
    decrypted_shares: Vec<Bytes>,
) -> Log {
    let response = Decryption::UserDecryptionResponse {
        decryptionId: U256::from(decryption_id),
        userDecryptedShares: decrypted_shares,
        signatures: vec![Bytes::from(vec![0u8; MOCK_SIGNATURE_SIZE])],
        extraData: Bytes::default(),
    };

    build_event_log(
        contract,
        &response,
        vec![
            Decryption::UserDecryptionResponse::SIGNATURE_HASH,
            B256::from(U256::from(decryption_id)),
        ],
    )
}

fn build_public_decrypt_request(contract: Address, decryption_id: u64, handles: Vec<B256>) -> Log {
    let request = Decryption::PublicDecryptionRequest {
        decryptionId: U256::from(decryption_id),
        snsCtMaterials: create_sns_materials(handles),
        extraData: Bytes::from(vec![0x00]),
    };

    build_event_log(
        contract,
        &request,
        vec![
            Decryption::PublicDecryptionRequest::SIGNATURE_HASH,
            B256::from(U256::from(decryption_id)),
        ],
    )
}

fn build_public_decrypt_response(
    contract: Address,
    decryption_id: u64,
    decrypted_values: Vec<u64>,
    success: bool,
) -> Log {
    let response = if success && !decrypted_values.is_empty() {
        Decryption::PublicDecryptionResponse {
            decryptionId: U256::from(decryption_id),
            decryptedResult: Bytes::from(decrypted_values[0].to_be_bytes().to_vec()),
            signatures: vec![Bytes::from(vec![0u8; MOCK_SIGNATURE_SIZE])],
            extraData: Bytes::default(),
        }
    } else {
        Decryption::PublicDecryptionResponse {
            decryptionId: U256::from(decryption_id),
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
            B256::from(U256::from(decryption_id)),
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
        assert!(id2 > id1, "Decryption IDs should increment");

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
        wrapper.on_user_decrypt_success(handles.clone(), user, Bytes::from("result"));
        wrapper.on_user_decrypt_error(handles.clone(), user);
        wrapper.on_user_decrypt_revert("test reason");

        wrapper.on_public_decrypt_success(handles.clone(), vec![42]);
        wrapper.on_public_decrypt_error(handles.clone());
        wrapper.on_public_decrypt_revert("test reason");

        wrapper.on_input_proof_success(user, test_data.clone());
        wrapper.on_input_proof_error(user, test_data);
        wrapper.on_input_proof_revert("test reason");
    }
}
