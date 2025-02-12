# FHEVM Relayer

A Rust-based relayer service that handles encrypted communications between FHEVM (Fully Homomorphic Encryption Virtual Machine) and L2 networks, with specific support for Arbitrum integration.
A specialized Rust-based relayer service that acts as a bridge between fhEVM and gateway. It handles four core operation types:

- **Public Decryption (Blockchain Events)**: Processes on-chain decryption requests from **DecryptionOracle**
- **User Decryption (HTTP)**: Handles user-specific decryption requests
- **Inputs Verification (HTTP)**: Validates ZkPoK input
- **Public Key Material Retrieval (HTTP)**: return public FHE keys and CRS (used for ZkPok)

## Architecture

### Orchestrator

The orchestrator is the central event coordination system that manages the flow of events between different components of the relayer. It implements an event-driven architecture with the following key features:

- **Event Dispatch System**:
  - Manages registration and routing of events to appropriate handlers
  - Supports both persistent handlers (long-lived) and one-time handlers (single-use)
  - Uses event IDs and UUIDs for precise event routing and tracking

- **Request Lifecycle Management**:
  - Generates unique request IDs using UUID v1 with node-specific identification
  - Maintains context and state across asynchronous event chains
  - Ensures proper event sequencing and handler coordination

The orchestrator works with four main traits:

- `Event`: Defines the event interface (name, ID, request ID)
- `EventDispatcher`: Handles the actual dispatch of events
- `EventHandler`: Processes specific event types
- `HandlerRegistry`: Manages handler registration and lookup

### Core Features

- Event-driven system with specialized handlers for L1 and gateway communications
- Comprehensive transaction management with automatic retries and gas estimation
- HTTP endpoint management for user operations
- Smart contract integration with DecryptionOracle (L1), DecryptionManager (Gateway) or InputManager  (Gateway) ...

## Quick Start

### Prerequisites

- Rust 2021 edition
- Make
- Node.js and npm (for contract deployment + test)
- Git
- Anvil (to run run nodes)

### Setup & Running

Run the following commands in sequence (each in a separate terminal):

```bash
# Terminal 1: Setup and run HTTPZ mock chain
make setup-mock             # First time setup only
make run-httpz-mock         # Keep running (L1)

# Terminal 2: Run Gateway mock chain
make run-gateway-mock       # Keep running (Gateway)

# Terminal 3: Deploy contracts and start listener
make deploy-httpz-smart-contracts
make deploy-gateway-smart-contracts
make config-fhevm-relayer
make start-fhevm-relayer   # Keep running (fhevm-relayer)

# Terminal 4: Run tests
make run-test-decrypt # Use fhevm-relayer
make run-test-erc20 # not yet
```

### Detailed Setup Steps

1. **Initial Setup** (one-time only)

   ```bash
   make setup-mock
   ```
  
   This initializes your development environment.

2. **Start HTTPZ Chain** (Terminal 1)

   ```bash
   make run-httpz-mock
   ```

   Launches Anvil node for the HTTPZ chain.

3. **Start Gateway Chain** (Terminal 2)

   ```bash
   make run-gateway-mock
   ```

   Launches Anvil node for the Gateway chain.

4. **Deploy Smart Contracts** (Terminal 3)

   ```bash
   make deploy-httpz-smart-contracts
   make deploy-gateway-smart-contracts
   ```

   Deploys necessary contracts to both chains.

5. **Configure and Start Event Listener** (Terminal 3)

   ```bash
   make config-fhevm-relayer
   make start-fhevm-relayer
   ```

   Sets up and runs fhevm-relayer

6. **Run Tests** (Terminal 4)

   ```bash
   make run-test-decrypt // using fhevm-relayer for public decryptions
   make run-test-erc20 // without fhevm-relayer user decryption not implemented yet
   ```

<details>
  <summary>Typical output for decryption flow</summary>
  <br>

```bash
  ```console
2025-02-12T15:30:50.976830Z  INFO Processing relayer event event_type=PubDecryptEventLogRcvdFromHostL1 request_id=571d3ba5-e956-11ef-800c-0123456789ab
2025-02-12T15:30:50.976899Z  INFO Decryption event log received from listener: request_id: 571d3ba5-e956-11ef-800c-0123456789ab block number: Some(29284), ethereum_request_id: 0, selector 0x3e54e2de
2025-02-12T15:30:50.976941Z  INFO Processing relayer event event_type=DecryptRequestRcvd request_id=571d3ba5-e956-11ef-800c-0123456789ab
2025-02-12T15:30:50.976951Z  INFO Decryption request received. Making a tx to rollup: request_id: 571d3ba5-e956-11ef-800c-0123456789ab with handles [9782591621254435756272560011972023832184190840567705870600336194590147793]
2025-02-12T15:30:50.976999Z  INFO Submitting transaction operation="decryption_request" calldata=0xe2a7b2f100000000000000000000000000000000...
2025-02-12T15:30:50.992784Z  INFO rollup listener catches one event
2025-02-12T15:30:50.992870Z  INFO Processing relayer event event_type=DecryptResponseEventLogRcvdFromGwL2 request_id=571faea4-e956-11ef-800d-0123456789ab
2025-02-12T15:30:50.992883Z  INFO Decryption response received. Trigger a tx to L1  571faea4-e956-11ef-800d-0123456789ab
2025-02-12T15:30:51.233501Z  INFO Transaction sent successfully tx_hash=0x62010cd124fc859a04182fe72b5a701ee681a3d7f5493b619540afe03e06c297
2025-02-12T15:30:51.233528Z  INFO Transaction submitted, waiting for confirmation tx_hash=0x62010cd124fc859a04182fe72b5a701ee681a3d7f5493b619540afe03e06c297 operation="decryption_request"
2025-02-12T15:30:51.234055Z  INFO Found decryption ID from event receipt.transaction_hash=0x62010cd124fc859a04182fe72b5a701ee681a3d7f5493b619540afe03e06c297 event.publicDecryptionId=1
2025-02-12T15:30:51.234093Z  INFO Stored mapping between decryption ID and request ID event.request_id=571d3ba5-e956-11ef-800c-0123456789ab decryption_public_id=1
2025-02-12T15:30:51.234130Z  INFO Processing relayer event event_type=DecryptionRequestSentToGwL2 request_id=571d3ba5-e956-11ef-800c-0123456789ab
2025-02-12T15:30:51.234136Z  INFO Transaction to rollup has been done, the associated public decryption id is 1
2025-02-12T15:30:52.994671Z  INFO Public decryption id from event public_decryption_id=1
2025-02-12T15:30:52.994717Z  INFO Found original request ID for decryption response original_request_id=571d3ba5-e956-11ef-800c-0123456789ab decryption_public_id=1
2025-02-12T15:30:52.994784Z  INFO Processing relayer event event_type=DecryptionResponseRcvdFromGwL2 request_id=571d3ba5-e956-11ef-800c-0123456789ab
2025-02-12T15:30:52.994794Z  INFO Decryption response received: request_id: 571d3ba5-e956-11ef-800c-0123456789ab, value: PublicDecrypt { plaintext: [1, 2, 3], signatures: [[1, 2, 3]] }
2025-02-12T15:30:52.994861Z  INFO Submitting transaction operation="decryption_response" calldata=0x3e54e2de00000000000000000000000000000000...
2025-02-12T15:30:53.250600Z  INFO Transaction sent successfully tx_hash=0x4e5fc32e8bba27a33f7d16c32fa0cb43a2efa8a55c301b110b3d99189f12b041
2025-02-12T15:30:53.250638Z  INFO Transaction submitted, waiting for confirmation tx_hash=0x4e5fc32e8bba27a33f7d16c32fa0cb43a2efa8a55c301b110b3d99189f12b041 operation="decryption_response"
2025-02-12T15:30:53.251563Z  INFO Transaction confirmed successfully operation="decryption_response"
2025-02-12T15:30:53.251641Z  INFO Processing relayer event event_type=DecryptResponseSentToHostL1 request_id=571d3ba5-e956-11ef-800c-0123456789ab
2025-02-12T15:30:53.251665Z  INFO Transaction to fhevm has been done
  ```
</details>
  <br>

   Executes the test suite.

## Project Structure

```console
├── artifacts                    # Contract ABIs and artifacts
│   ├── DecryptionManager.json
│   ├── DecryptionOracle.json
│   ├── GatewayContract.abi
│   └── TFHEExecutor.json
├── Cargo.lock
├── Cargo.toml
├── config                      # Configuration files
│   ├── local.yaml
│   └── local.yaml.example
├── design-docs/                    # Architecture and design documentation
├── hardhat
│   └── contracts                   # Smart contract development and deployment scripts
├── LICENSE
├── Makefile
├── README.md
├── setup-config.sh
├── src
│   ├── arbitrum_gateway_l2_handlers.rs
│   ├── config                     # Configuration management
│   │   ├── mod.rs
│   │   └── settings.rs
│   ├── errors.rs
│   ├── ethereum                  # Ethereum interaction layer
│   │   ├── bindings.rs
│   │   ├── filter.rs
│   │   ├── host_l1.rs
│   │   ├── mod.rs
│   │   ├── rollup_l2.rs
│   │   ├── transaction_helper
│   │   └── utils.rs
│   ├── ethereum_host_l1_handlers.rs
│   ├── ethereum_listener.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── orchestrator             # Event orchestration
│   │   ├── mod.rs
│   │   ├── orchestrator.rs
│   │   ├── tokio_event_dispatcher.rs
│   │   └── traits.rs
│   ├── relayer_event.rs
│   ├── rollup_listener.rs
│   ├── tests
│   │   └── orchestrator_test.rs
│   ├── transaction           # Transaction management
│   │   ├── helper.rs
│   │   ├── mod.rs
│   │   ├── sender.rs
│   │   └── service.rs
│   └── utils.rs
```

## Core Components

### Event Handlers

- `EthereumHostL1Handler`: Manages events from the Ethereum L1 chain
- `ArbitrumGatewayL2Handler`: Handles L2-specific operations for Arbitrum
- `DecryptionOracle`: Contract interface for decryption operations
- `TFHEExecutor`: Executor for TFHE operations

### Orchestration

- Event-driven architecture with request/response pattern
- UUID-based request tracking
- Support for one-time and persistent event handlers

## Configuration

The service uses a hierarchical configuration system:

### Network Settings

```yaml
networks:
  fhevm:
    ws_url: "wss://your-fhevm-node"
    http_url: "https://your-fhevm-node"
    chain_id: 12345
    retry_delay: 5
    max_reconnection_attempts: 3
  rollup:  # Optional L2 configuration
    ws_url: "wss://your-l2-node"
    http_url: "https://your-l2-node"
    chain_id: 654321
```

### Contract Addresses

```yaml
contracts:
  decryption_oracle_address: "0x..."
  tfhe_executor_address: "0x..."
  decryption_manager_address: "0x..."
```

## Dependencies

```toml
[dependencies]
alloy = { version = "0.9.2", features = ["full"] }
alloy-sol-types = "0.8.18"
dashmap = "6.1.0"
eyre = "0.6.12"
futures = "0.3.31"
# ... [other dependencies as listed in Cargo.toml]
```

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Logging

Uses `tracing` for structured logging with configurable levels:

```yaml
log:
  level: "info"  # trace, debug, info, warn, or error
  format: "compact"  # compact, pretty, or json
  show_file_line: true
  show_thread_ids: true
```

## License

BSD-3-Clause-Clear

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request
