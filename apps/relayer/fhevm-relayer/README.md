# FHEVM Relayer

A Rust-based event relayer designed for flexible blockchain event monitoring. It allows registration of any number of contracts and their associated events, whether monitoring a single contract event or multiple events across different contracts.

## Features

- **Contract and Event Registration**:
  - Register multiple smart contracts for event monitoring
  - Configure multiple events per contract
  - Dynamic event processing system
- **Built-in Examples**:
  - DecryptionOracle contract and events
  - TFHEExecutor contract and events
- **Development Environment**:
  - Hardhat node running in mock mode to trigger events with up-to-date fhEVM smart contracts
  - Complete test environment for event simulation

## Prerequisites

- Rust 1.75 or higher
- Node.js and npm
- Cargo and its dependencies

## Quick Start

1.**First Time Setup**

```bash
# Initial setup of dependencies and configuration
make setup-mock
```

2.**Running the Environment**

You'll need several terminal windows to run the complete environment:

Terminal 1 - Anvil Node HTTPZ chain:

```bash
make run-httpz-mock
```

<details>
<summary>Typical output</summary>
<br>

```bash
# Typical output
Starting Hardhat node...
Generating typings for: 30 artifacts in dir: types for target: ethers-v6
Successfully generated 96 typings!
Compiled 35 Solidity files successfully (evm target: cancun).
Started HTTP and WebSocket JSON-RPC server at http://127.0.0.1:8746/

Accounts
========
Account #0: 0xa5e1defb98EFe38EBb2D958CEe052410247F4c80 (10000 ETH)
```

</details>
<br>

Terminal 2 - Anvil Node HTTPZ chain:

```bash
make run-gateway-mock
```

Terminal 3 - HTTPZ Smart Contracts:

```bash
make deploy-httpz-smart-contracts
```

<details>
<summary>Typical output</summary>
<br>

```bash
# Typical output
ACL code set successfully at address: 0x74c085A069fafD4f264B5200847EdB1ade82B3C0
TFHEExecutor code set successfully at address: 0x4e142887e3Dc6e414a9b260a1034D20C9B4Eb11F
KMSVerifier code set successfully at address: 0xa3f4D50ebfea1237316b4377F0fff4831F2D1c46
InputVerifier code set successfully at address: 0x59AAd6Dc3C909aeED1916937cC310fBfBB118c8C
FHEGasLimit code set successfully at address: 0x2Ea4b09A56bF59437C99293aF54F8E39e11a68Ba
DecryptionOracle code set successfully at address: 0x67aa98a03CC4559E1e98e7b4Ed071C35c40b588d
KMS signer no0 (0x0971C80fF03B428fD2094dd5354600ab103201C5) was added to KMSVerifier contract
```

</details>
<br>

Terminal 3 - GATEWAY Smart Contracts:

```bash
make deploy-gateway-smart-contracts
```

Terminal 3 - Event Listener:

```bash
make config-event-listener
make start-event-listener
```

<details>
<summary>Typical output</summary>
<br>

```bash
2025-01-28T10:59:06.140701Z  INFO Tracing initialized successfully level="info" format="pretty" show_file_line=false show_thread_ids=false
2025-01-28T10:59:06.140736Z  INFO --- Real Event Handler ---
2025-01-28T10:59:06.140750Z  INFO Initialized contract addresses decryption_oracle_address=0x67aa98a03cc4559e1e98e7b4ed071c35c40b588d tfhe_executor_address=0x4e142887e3dc6e414a9b260a1034d20c9b4eb11f settings.network.ws_url="ws://localhost:8746"
2025-01-28T10:59:06.143241Z  INFO *** Registering event signature DecryptionRequest(uint256,uint256,uint256[],address,bytes4) for contract 0x67aa98a03CC4559E1e98e7b4Ed071C35c40b588d.
2025-01-28T10:59:06.143284Z  INFO Topic -- keccack256(event_signature) = 0x2139fe1716d177355181c45bfba01280a9ce6d0a226dec18bb5808867a812179
2025-01-28T10:59:06.143322Z  INFO *** Registering event signature FheAdd(address,uint256,uint256,bytes1,uint256) for contract 0x4e142887e3Dc6e414a9b260a1034D20C9B4Eb11F.
2025-01-28T10:59:06.143344Z  INFO Topic -- keccack256(event_signature) = 0x9d4485ee9ce87c267c409bdb9b696a82a89c995903092b84553de8edc2592625
2025-01-28T10:59:06.143405Z  INFO Subscribing to logs for contracts: [0x4e142887e3dc6e414a9b260a1034d20c9b4eb11f, 0x67aa98a03cc4559e1e98e7b4ed071c35c40b588d]
2025-01-28T10:59:06.143428Z  INFO Connecting to Ethereum provider...
2025-01-28T10:59:06.143445Z  INFO Subscribing to logs with filters: Filter { block_option: Range { from_block: Some(latest), to_block: None }, address: FilterSet({0x4e142887e3dc6e414a9b260a1034d20c9b4eb11f, 0x67aa98a03cc4559e1e98e7b4ed071c35c40b588d}), topics: [FilterSet({}), FilterSet({}), FilterSet({}), FilterSet({})] }


```

</details>
<br>

Terminal 4 - Run decryption tests to catch decryption request event:

```bash
make run-test-decrypt
# Typical output in event listener
2025-01-28T10:21:49.664068Z  INFO process_event: Handling DecryptionRequest from new version decoded=DecryptionRequest { counter: 19, requestID: 0, cts:[54753640583190511018042259926215371455243830566367266991450347141893636229888], contractCaller: 0xc906e3984740ef4074b681e58cc3cc4c8c711aca, callbackSelector: 0x9e0e3e34 }
```

Terminal 4 - Run ERC20 tests to catch fheAdd operation event:

```bash
make run-test-erc20
# Typical output
2025-01-28T10:24:37.753928Z  INFO process_event: Handling FheAdd operation from TFHEEXECUTOR decoded=FheAdd { caller: 0x911e8e1ab0493dc560a140103d1602f97dd3ff69, lhs: 107401874415472830175299155205586690465179006273083689345192445555626185524480, rhs: 87057231334485285938770473502522971296808484164578472682141752521394961319168, scalarByte: 0x00, result: 111807925205481249489564805351607540661105016450482757129453165299987226952960 }

```

> **Important**: Ensure each component is fully started before starting the next one. The event listener must be running before executing the tests to ensure proper event capture and processing.

## Project Structure

```bash
.
├── artifacts/                 # Smart contract artifacts
├── config/                    # Configuration files
├── examples/                  # Usage examples
├── src/
│   ├── bin/                  # Binary executables
│   ├── common/               # Shared utilities
│   ├── config/               # Configuration handling
│   ├── ethereum/             # Ethereum-specific code
│   ├── event/                # Event processing
│   ├── service/              # Core services
│   └── main.rs               # Main application entry
```

## Configuration

The service can be configured using YAML files in the `config/` directory. Key configuration options include:

```yaml
network:
  ws_url: "ws://localhost:8746" # WebSocket URL for Ethereum node

contracts:
  decryption_oracle_address: "0x..."
  tfhe_executor_address: "0x..."

log:
  level: "info"
  format: "json"
  show_file_line: false
  show_thread_ids: false
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Adding New Contracts

1. Add contract ABI to `artifacts/`
2. Create new event processor in `src/event/`
3. Register contract and events in the registry
4. Update configuration file with the new contract address

### TODO

- [ ] Define events also in configuration file instead of in providers.rs
- [ ] Generate event processing code automatically from the new contract and events in the config file

## Error Handling

The project uses custom error types for better error handling:

```rust
#[derive(Error, Debug)]
pub enum EventHandlerError {
    #[error("ABI decode error: {0}")]
    AbiError(#[from] alloy_sol_types::Error),
    // ... other error types
}
```
