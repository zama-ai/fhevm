# ethereum_rpc_mock Design

Technical architecture and implementation details for the ethereum_rpc_mock library.

## Architecture Overview

```
MockServer (Public API)
    ↓
JsonRPSeeServer (Hidden Implementation)  
    ↓
PatternMatcher → BlockchainState → EventEmitter
```

## Core Modules

### MockServer
**Purpose:** Public facade that hides JsonRPSee complexity

**Key Methods:**
- `new(config: MockConfig) -> MockServer`
- `start() -> anyhow::Result<ServerHandle>`
- `shutdown() -> anyhow::Result<()>`
- `set_balance(address: Address, amount: U256)`
- `set_code(address: Address, bytecode: Bytes)`
- `on_transaction(predicate: impl Fn(TxParams) -> bool, response: TxResponse, usage: UsageLimit)`
- `on_call(predicate: impl Fn(CallParams) -> bool, response: CallResponse, usage: UsageLimit)`
- `reset_state()`

### JsonRPSeeServer
**Purpose:** Handle JSON-RPC HTTP/WebSocket requests using JsonRPSee

**RPC Methods Supported:**
- `chain_id()`, `block_number()`, `get_balance()`, `get_transaction_count()`
- `get_code()`, `get_storage_at()`, `get_transaction_receipt()`
- `estimate_gas()`, `gas_price()`, `fee_history()`
- `get_block_by_number()`, `get_logs()`
- `call()`, `send_raw_transaction()`
- `subscribe()` (WebSocket subscriptions)

**Implementation:** Filter parameters are ignored for simplicity. All log subscribers receive all logs, all head subscribers receive all block headers.

### PatternMatcher
**Purpose:** Match incoming requests against user-defined patterns

**Key Methods:**
- `add_transaction_pattern(predicate: PredicateFn<TxParams>, response: TxResponse, usage: UsageLimit)`
- `add_call_pattern(predicate: PredicateFn<CallParams>, response: CallResponse, usage: UsageLimit)`
- `find_transaction_match(tx_params: &TxParams) -> Option<TxResponse>`
- `find_call_match(call_params: &CallParams) -> Option<CallResponse>`

**Pattern Matching Rules:**
- Evaluate patterns in registration order (first match wins)
- Check predicate AND usage limit allows use for matching
- For Once patterns: remove pattern from list after first use
- For Unlimited patterns: keep pattern in list indefinitely

### BlockchainState
**Purpose:** Track minimal blockchain state for RPC responses

**Key Methods:**
- `get_balance(address: Address) -> U256`
- `set_balance(address: Address, amount: U256)`
- `get_nonce(address: Address) -> u64`
- `get_code(address: Address) -> Bytes`
- `get_storage(address: Address, slot: U256) -> U256`
- `store_transaction_receipt(hash: B256, receipt: TransactionReceipt)`
- `get_current_block() -> u64`
- `increment_block() -> u64`

### EventEmitter
**Purpose:** Broadcast WebSocket events to all subscribers

**Key Methods:**
- `emit_to_log_subscribers(log: Log) -> anyhow::Result<()>`
- `emit_to_head_subscribers(block: Block) -> anyhow::Result<()>`

**Implementation:** Uses shared references to subscription maps managed by JsonRPSee server. Broadcasts to all active sinks and removes inactive sinks automatically.

## Key Data Types

```rust
pub struct MockConfig {
    pub port: u16,
    pub chain_id: u64,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub block_time_ms: u64,
}

pub struct TxParams {
    pub to: Option<Address>,
    pub value: U256,
    pub data: Bytes,
    pub gas: u64,
    pub gas_price: U256,
    pub nonce: u64,
}

pub struct CallParams {
    pub from: Option<Address>,
    pub to: Address,
    pub data: Bytes,
    pub gas: Option<u64>,
    pub gas_price: Option<U256>,
}

pub enum TxResponse {
    Success { hash: B256, logs: Vec<Log> },
    Revert { hash: B256, reason: Option<String> },
    Error(String),
}

pub enum CallResponse {
    Success(Bytes),
    Revert(Option<String>),
    Error(String),
}

pub enum UsageLimit {
    Unlimited,
    Once,
}

pub struct Account {
    pub balance: U256,
    pub nonce: u64,
    pub code: Bytes,
    pub storage: HashMap<U256, U256>,
}

type PredicateFn<T> = Arc<dyn Fn(&T) -> bool + Send + Sync>;
```

## Implementation Guidelines

### State Management
- Use RwLock for concurrent access to blockchain state
- Only track essential state (accounts, transaction receipts, blocks, logs)
- Provide block progression for subscription events

### Transaction Processing
- Simplified transaction handling without signature validation
- Generate transaction receipts for mocked responses
- Emit events after state changes

### Delayed Transactions
- Use tokio::time::sleep for scheduling
- Simple event emission after delay
- Simplified approach without full transaction execution

### Error Handling
- Return proper JSON-RPC error codes
- Use anyhow::Result for internal errors
- Provide clear error messages for pattern matching failures

### FHEVM Integration
- FhevmMockWrapper uses PatternMatcher for decryption patterns
- Generates proper event logs using contract bindings
- Handles ID generation and scheduling automatically

## Examples and Usage

For complete implementation examples, see:
- **`tests/test_fhevm.rs`** - Full FHEVM workflow with pattern matching
- **`tests/test_core_rpc.rs`** - Basic RPC functionality examples
- **`tests/fhevm_event_signatures.rs`** - Event handling patterns