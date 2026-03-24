# ethereum_rpc_mock

Generic Ethereum JSON-RPC mock server for testing blockchain applications. Returns configured responses to standard RPC calls without requiring a live network. Includes FHEVM helper layer for encrypted computation testing.

```
                    ┌─────────────────────────┐
                    │      Client RPC         │
                    │  (alloy, web3, etc)     │
                    └─────────────┬───────────┘
                                  │
                    ┌─────────────▼───────────┐
                    │      MockServer         │
                    │   (JsonRPSee HTTP/WS)   │
                    └─────────────┬───────────┘
                                  │ coordinates
        ┌─────────────────────────┼─────────────────────────┐
        │                         │                         │
        ▼                         ▼                         ▼
┌─────────────┐         ┌─────────────┐         ┌─────────────┐
│Pattern      │         │Blockchain   │         │ Scheduled   │
│Matcher      │         │State        │         │Transactions │
│             │         │             │         │             │
│predicate→   │◄────────┤• accounts   │────────►│delay→emit   │
│response     │         │• receipts   │         │follow-up tx │
│UsageLimit   │         │• logs       │         │             │
│first wins   │         │             │         │             │
└─────────────┘         └─────┬───────┘         └─────┬───────┘
                               │                       │
                               └───────┬───────────────┘
                                       │
                         ┌─────────────▼───────────────┐
                         │     WebSocket Emission      │
                         │   • logs to subscribers     │
                         │   • newHeads notifications  │
                         │   • all events broadcast    │
                         └─────────────────────────────┘

        ┌─────────────┐
        │ FHEVM Layer │
        │• patterns   │
        │• auto IDs   │
        │• test_utils │
        └─────────────┘
```

## Usage for Testing

**Mock Standard RPC Calls**

- Configure responses for transactions and calls using predicate functions
- Set blockchain state (balances, nonces, contract code)
- Handle WebSocket subscriptions for events

**FHEVM Testing Patterns**

- Built-in helpers for input proof verification, decryption workflows
- Automatic request ID generation and response matching

## Basic Usage

```rust
let server = MockServer::new(MockConfig::new());
let handle = server.start().await?;

// Configure responses
server.on_transaction(|tx| tx.to == Some(addr), Response::transaction_success(), UsageLimit::Once);
server.set_balance(addr, U256::from(1000));

// FHEVM helpers
server.fhevm().on_input_proof_success();
server.fhevm().on_public_decrypt_success(plaintext);

handle.shutdown().await?;
```

**See tests for complete examples:**

- `tests/test_core_rpc.rs` - Standard RPC testing patterns
- `tests/test_fhevm.rs` - FHEVM encrypted computation workflows

## Main Functions

**Server Lifecycle:** `MockServer::new()`, `server.start()`, `handle.shutdown()`

**State Setup:** `set_balance()`, `set_code()`, `set_storage()`, `reset_state()`

**Response Patterns:** `on_transaction()`, `on_call()` with predicate functions and usage limits

**FHEVM Helpers:** `server.fhevm().on_input_proof_*()`, `on_public_decrypt_*()`, `on_user_decrypt_*()`

Refer to source code and tests for complete API details and usage patterns.

## Architecture

Generic mock server (`src/mock_server/`) handles standard Ethereum RPC calls via pattern matching (`src/pattern_matcher.rs`) and blockchain state (`src/blockchain.rs`).

FHEVM helper layer (`src/fhevm.rs`) provides relayer specific test patterns.
