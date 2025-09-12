# Transaction Manager Analysis: FHEVM vs Gateway Chains

## Overview

The FHEVM relayer uses a unified transaction management architecture with dual instantiation to handle both FHEVM and Gateway blockchain interactions. This document provides a comprehensive analysis of the system's design, differences between chains, and operational mechanics.

## Architecture Overview

### High-Level Design

```
┌─────────────────────────────────────────────────────────────────┐
│                    FHEVM Relayer Application                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────┐      ┌─────────────────────┐          │
│  │   FHEVM Chain       │      │   Gateway Chain     │          │
│  │   TransactionService│      │   TransactionService│          │
│  │                     │      │                     │          │
│  │  ┌───────────────┐  │      │  ┌───────────────┐  │          │
│  │  │Transaction    │  │      │  │Transaction    │  │          │
│  │  │Manager        │  │      │  │Manager        │  │          │
│  │  │               │  │      │  │               │  │          │
│  │  │- fhevm_signer │  │      │  │- gateway_signer│  │          │
│  │  │- chain_id: X  │  │      │  │- chain_id: Y   │  │          │
│  │  │- ws_url: ...  │  │      │  │- ws_url: ...   │  │          │
│  │  └───────────────┘  │      │  └───────────────┘  │          │
│  └─────────────────────┘      └─────────────────────┘          │
│           │                              │                     │
│           ▼                              ▼                     │
│  ┌─────────────────────┐      ┌─────────────────────┐          │
│  │    FHEVM Network    │      │   Gateway Network   │          │
│  │                     │      │                     │          │
│  │ • FHE Operations    │      │ • User Requests     │          │
│  │ • Decryption        │      │ • Request Forwarding│          │
│  │ • Ciphertext Mgmt   │      │ • Response Handling │          │
│  └─────────────────────┘      └─────────────────────┘          │
└─────────────────────────────────────────────────────────────────┘
```

### Core Components

1. **TransactionService**: High-level transaction orchestration
2. **TransactionManager**: Low-level blockchain interaction
3. **TransactionHelper**: Application-specific transaction utilities
4. **Nonce Management**: Cached nonce tracking with refresh capability

## Key Differences Between FHEVM and Gateway Chains

### 1. Configuration Differences

```
Configuration Layer
├── Networks Config
│   ├── FHEVM Network
│   │   ├── ws_url: "ws://fhevm-node:8546"
│   │   ├── chain_id: 8009
│   │   ├── retry_delay: 1000ms
│   │   └── max_reconnection_attempts: 5
│   └── Gateway Network
│       ├── ws_url: "ws://gateway-node:8546"
│       ├── chain_id: 8545
│       ├── retry_delay: 500ms
│       └── max_reconnection_attempts: 3
├── Transaction Config
│   ├── private_key_fhevm: "0x..."
│   ├── private_key_gateway: "0x..."
│   ├── gas_limit: 500000 (shared)
│   ├── timeout_secs: 60 (shared)
│   └── confirmations: 1 (shared)
└── Contract Addresses
    ├── decryption_oracle_address
    ├── decryption_address
    └── input_verification_address
```

### 2. Error Handling Differences

#### FHEVM-Specific Error Processing

```
FHEVM Error Processing Pipeline
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│ Contract Call   │───▶│ parse_fhevm_error│───▶│ retryable_error │
│ Fails           │    │ (fhevm.rs:35)    │    │ (fhevm.rs:19)   │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                               │                        │
                               ▼                        ▼
                       ┌──────────────────┐    ┌─────────────────┐
                       │ Categorize Error │    │ Retry Decision  │
                       │ • DecryptionError│    │ • ACL: retry    │
                       │ • InputError     │    │ • Ciphertext:   │
                       │ • AclError       │    │   retry         │
                       │ • CiphertextError│    │ • Others: fail  │
                       └──────────────────┘    └─────────────────┘
```

**FHEVM Error Types**:
- `DecryptionErrors`: Contract-specific decryption failures
- `InputVerificationErrors`: ZK proof validation errors  
- `MultichainAclErrors`: Permission and ACL violations
- `CiphertextCommitsErrors`: Ciphertext material issues

**Gateway Error Handling**:
- Uses standard Alloy EVM error processing
- Generic retry logic based on RPC errors
- No specialized contract error parsing

### 3. Transaction Flow Patterns

#### FHEVM Transaction Types
```
FHEVM Transaction Flow
┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
│ Receive Request     │───▶│ Process FHE Op      │───▶│ Send Response       │
│ from Gateway        │    │ • Decrypt           │    │ back to Gateway     │
│                     │    │ • Reencrypt         │    │                     │
│                     │    │ • Verify Input      │    │                     │
└─────────────────────┘    └─────────────────────┘    └─────────────────────┘
```

#### Gateway Transaction Types  
```
Gateway Transaction Flow
┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
│ Receive User        │───▶│ Forward to FHEVM    │───▶│ Process FHEVM       │
│ Request             │    │ • UserDecryptReq    │    │ Response            │
│                     │    │ • InputRequest      │    │                     │
│                     │    │ • PublicDecryptReq  │    │                     │
└─────────────────────┘    └─────────────────────┘    └─────────────────────┘
```

## Transaction Lifecycle & State Management

### Transaction States

```
Transaction State Machine
┌─────────┐    submit_transaction()    ┌─────────┐
│ Ready   │─────────────────────────▶ │ Pending │
└─────────┘                           └─────────┘
                                           │
                    ┌──────────────────────┼──────────────────────┐
                    │                      │                      │
                    ▼                      ▼                      ▼
            ┌─────────────┐        ┌─────────────┐        ┌─────────────┐
            │ Confirmed   │        │ Failed      │        │ Timeout     │
            │             │        │             │        │ (becomes    │
            │ receipt.    │        │ reason:     │        │ Failed)     │
            │ status=true │        │ String      │        │             │
            └─────────────┘        └─────────────┘        └─────────────┘
                    │                      │                      │
                    ▼                      ▼                      ▼
            ┌─────────────┐        ┌─────────────┐        ┌─────────────┐
            │ Cleanup     │        │ Cleanup     │        │ Cleanup     │
            │ after: now  │        │ after: now  │        │ after: now  │
            │ + 0s        │        │ + 300s      │        │ + 300s      │
            └─────────────┘        └─────────────┘        └─────────────┘
```

### Transaction Record Structure

```rust
struct TransactionRecord {
    target: Address,           // Contract address
    calldata: Bytes,          // Transaction data
    config: TxConfig,         // Gas, timeout, etc.
    state: TransactionState,  // Current state
    cleanup_after: Option<Instant>,  // When to cleanup
    ready_for_cleanup: bool,  // Cleanup flag
}
```

## Cleanup Logic Deep Dive

### Overview of Cleanup System

The cleanup system manages the lifecycle of transaction records in memory, ensuring that completed, failed, or timed-out transactions are eventually removed to prevent memory leaks.

### Cleanup Trigger Points

```
Cleanup Trigger Flow
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│ Maintenance     │───▶│ Transaction     │───▶│ Cleanup         │
│ Task (5s)       │    │ Complete/Fail   │    │ Execution       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
        │                       │                       │
        ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│ Check all       │    │ Set cleanup_    │    │ Remove from     │
│ transactions    │    │ after timestamp │    │ DashMap         │
│ for cleanup     │    │                 │    │                 │
│ eligibility     │    │                 │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Cleanup Timing Matrix

| Transaction State | Cleanup Delay | Reason |
|------------------|---------------|---------|
| **Confirmed** | Immediate (0s) | Success, no retry needed |
| **Failed** | 300s (5min) | Allow time for investigation |
| **Timeout** | 300s (5min) | May still succeed on chain |
| **Reverted** | Immediate (0s) | Clear failure, cleanup now |

### Detailed Cleanup Process

#### 1. Maintenance Task Scheduling

```rust
// src/transaction/service.rs:728
pub fn spawn_maintenance_tasks(self: Arc<Self>, interval: Duration, error_interval: Duration) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(interval);
        loop {
            interval.tick().await;
            if let Err(e) = self.maintain_transactions().await {
                error!(error = %e, "Error in maintain_transactions");
                tokio::time::sleep(error_interval).await;
            }
        }
    });
}
```

**Schedule**: 
- **Normal interval**: 5 seconds
- **Error interval**: 10 seconds (backoff on errors)

#### 2. Transaction Maintenance Flow

```
maintain_transactions() Flow
┌─────────────────┐
│ Start           │
│ Maintenance     │
│ Cycle           │
└─────────┬───────┘
          │
          ▼
┌─────────────────┐    ┌─────────────────┐
│ Step 1:         │───▶│ Step 2:         │
│ cleanup_        │    │ get_pending_    │
│ transactions()  │    │ transactions()  │
└─────────────────┘    └─────────────────┘
          │                       │
          ▼                       ▼
┌─────────────────┐    ┌─────────────────┐
│ Remove expired  │    │ For each pending│
│ transactions    │    │ transaction:    │
│ from memory     │    │ handle_pending_ │
│                 │    │ transaction()   │
└─────────────────┘    └─────────────────┘
```

#### 3. Cleanup Transaction Logic

```rust
// src/transaction/service.rs:450
async fn cleanup_transactions(&self, now: Instant) {
    // Find transactions ready for cleanup
    let to_remove: Vec<_> = self
        .transactions
        .iter()
        .filter_map(|entry| {
            if let Some(cleanup_time) = entry.value().cleanup_after {
                if now >= cleanup_time {
                    Some(*entry.key())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // Remove them with logging
    for request_id in to_remove {
        if let Some((_, record)) = self.transactions.remove(&request_id) {
            match record.state {
                TransactionState::Failed { reason } => {
                    // Special handling for timeouts - sync nonce
                    if reason.contains("Transaction timed out") {
                        let address = self.manager.sender_address();
                        match self.manager.nonce_manager
                            .sync_nonce(&**self.manager.provider.read().await, address)
                            .await {
                            Ok(new_nonce) => info!("Synced nonce to {}", new_nonce),
                            Err(err) => error!("Failed to sync nonce: {}", err),
                        }
                    }
                }
                // Log other states...
            }
        }
    }
}
```

#### 4. Cleanup State Transitions

```
Cleanup State Transition Diagram

Transaction Completion:
┌─────────────────┐    receipt.status() == true    ┌─────────────────┐
│ Pending         │─────────────────────────────▶  │ Confirmed       │
│ {hash, time,    │                                │ {receipt}       │
│  attempts}      │                                │                 │
└─────────────────┘                                └─────────────────┘
                                                            │
                                                            ▼
                                                   ┌─────────────────┐
                                                   │ cleanup_after = │
                                                   │ now (immediate) │
                                                   │ ready_for_      │
                                                   │ cleanup = true  │
                                                   └─────────────────┘

Transaction Failure:
┌─────────────────┐    receipt.status() == false   ┌─────────────────┐
│ Pending         │─────────────────────────────▶  │ Failed          │
│ {hash, time,    │    OR timeout exceeded        │ {reason}        │
│  attempts}      │    OR RPC error               │                 │
└─────────────────┘                                └─────────────────┘
                                                            │
                                                            ▼
                                                   ┌─────────────────┐
                                                   │ cleanup_after = │
                                                   │ now + 300s      │
                                                   │ ready_for_      │
                                                   │ cleanup = false │
                                                   └─────────────────┘
```

#### 5. Nonce Recovery on Timeout

When a transaction times out, the cleanup process includes nonce synchronization to prevent nonce gaps:

```rust
// Special timeout handling in cleanup_transactions()
if reason.contains("Transaction timed out") {
    let address = self.manager.sender_address();
    // Re-sync nonce with network to handle potential nonce gaps
    match self.manager.nonce_manager
        .sync_nonce(&**provider_guard, address)
        .await {
        Ok(new_nonce) => info!("Nonce synced to {}", new_nonce),
        Err(err) => error!("Nonce sync failed: {}", err),
    }
}
```

**Why Nonce Sync is Important**:
- Timed-out transactions may still be in mempool
- If they eventually execute, they consume a nonce
- Without sync, subsequent transactions would have nonce gaps
- Nonce gaps cause all following transactions to fail

#### 6. Memory Management

```
Memory Management Strategy

DashMap<Uuid, TransactionRecord>
┌─────────────────────────────────────────┐
│ ┌─────────┐ ┌─────────┐ ┌─────────┐     │
│ │ tx_1    │ │ tx_2    │ │ tx_3    │ ... │
│ │ Ready   │ │ Pending │ │ Failed  │     │
│ └─────────┘ └─────────┘ └─────────┘     │
└─────────────────────────────────────────┘
           │         │         │
           ▼         ▼         ▼
    No cleanup  Monitor   cleanup_after
    needed      status    = now + 5min
                           │
                           ▼
                    ┌─────────────┐
                    │ Maintenance │
                    │ task finds  │
                    │ and removes │
                    └─────────────┘
```

**Benefits**:
- **Memory bounded**: Old transactions are automatically cleaned up
- **Investigation window**: Failed transactions stay for 5 minutes
- **Immediate cleanup**: Successful transactions removed right away
- **Crash recovery**: On restart, all in-memory state is lost (acceptable for this design)

## Shared Infrastructure

### Components Used by Both Chains

1. **Nonce Management**: `CachedNonceManagerWithRefresh`
   - Tracks nonce per address across providers
   - Handles nonce synchronization on errors
   - Prevents nonce gaps and conflicts

2. **Gas Estimation**: Alloy-based gas estimation with 20% buffer
3. **Receipt Waiting**: WebSocket subscription-based receipt polling
4. **Error Recovery**: Automatic provider reset on connection failures
5. **Transaction Retry**: Configurable retry mechanisms for network errors

### Performance Characteristics

| Aspect | FHEVM Chain | Gateway Chain |
|--------|-------------|---------------|
| **Computational Load** | High (FHE ops) | Standard (EVM) |
| **Transaction Size** | Larger (ciphertext) | Standard |
| **Confirmation Time** | Network dependent | Network dependent |
| **Error Frequency** | Higher (complex ops) | Lower (standard ops) |
| **Retry Requirements** | More frequent | Less frequent |

## Configuration Examples

### Development Configuration

```toml
[networks.fhevm]
ws_url = "ws://localhost:8546"
http_url = "http://localhost:8545"
chain_id = 8009
retry_delay = 1000
max_reconnection_attempts = 5

[networks.gateway]
ws_url = "ws://localhost:9546"
http_url = "http://localhost:9545"
chain_id = 8545
retry_delay = 500
max_reconnection_attempts = 3

[transaction]
private_key_fhevm = "0x..."
private_key_gateway = "0x..."
gas_limit = 500000
timeout_secs = 60
confirmations = 1
```

## Best Practices & Recommendations

### 1. Transaction Management
- **Monitor transaction pools**: Both chains require monitoring for stuck transactions
- **Gas price management**: Consider different gas strategies per chain
- **Timeout tuning**: FHEVM operations may need longer timeouts

### 2. Error Handling
- **Implement circuit breakers**: For high error rate scenarios
- **Custom retry policies**: Different retry strategies per chain type
- **Monitoring & alerting**: Track error rates and patterns

### 3. Performance Optimization
- **Concurrent processing**: Process transactions for both chains in parallel
- **Connection pooling**: Maintain stable WebSocket connections
- **Batch operations**: Where possible, batch similar operations

### 4. Operational Considerations
- **Health checks**: Monitor both chain connections
- **Graceful shutdown**: Ensure pending transactions are handled on restart
- **Metrics collection**: Track success/failure rates per chain

## Conclusion

The FHEVM relayer's transaction manager successfully abstracts blockchain interactions through a unified interface while accommodating the specific requirements of both FHEVM and Gateway chains. The dual-instantiation approach provides consistency in transaction handling while allowing for chain-specific optimizations and error handling.

The cleanup system ensures memory efficiency and proper transaction lifecycle management, with special considerations for the unique characteristics of FHE operations and standard EVM transactions.