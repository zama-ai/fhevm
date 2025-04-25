# fhevm Relayer Service

fhevm (Fully Homomorphic Encryption Virtual Machine) relayer service is a bridge connecting fhevm blockchains and Gateway.
It enables the following functionalities for the fhevm blockchains:

- **Public Decryption**:

  - Decrypt ciphertexts on fhevm blockchains to plain text.
  - Public decryption should be permitted for the ciphertext handle on fhevm blockchain.
  - Requests are received as fhevm blockchain events or call to public decryption HTTP endpoint.
  - Received requests are relayed to gateway, which processes it and emits a response event.
  - Responses are relayed back to fhevm blockchain or the HTTP caller.

- **Input proof verification**:

  - Verify and attest ZKPoK (Zero Knowledge Proof of Plaintext Knowledge) on input ciphertexts is valid or not.
  - Requests are received as calls to input proof HTTP endpoint.
  - Received requests are relayed to gateway, which processes it and emits a response event.
  - Responses are relayed back to the HTTP caller.

- **User Decryption**:

  - Decrypt ciphertexts encrypted with fhevm keys on fhevm blockchains to ciphertext encrypted with public key provided by the user.
  - User should have the permission to access the ciphertext handle.
  - Requests are received as call to user decryption HTTP endpoint.
  - Received requests are relayed to gateway, which processes it and emits a response event.
  - Responses are relayed back to the HTTP caller.

- **Key Material**:
  - Exposes key material URLs (as FHE public key, FHE evaluation key ...)

## Architecture

The system follows an event-driven architecture with these key components:

- **Orchestrator**: Central coordinator for event flow and handling
- **Event Handlers**: Process specific event types:
  - `EthereumHostL1Handler`: Manages fhevm events and responses
  - `ArbitrumGatewayL2Handler`: Handles Gateway interaction for decryption
  - `ArbitrumGatewayL2InputHandler`: Processes input verification
- **Transaction Service**: Reliable transaction management
- **HTTP Endpoints**: RESTful API for input proofs and user/public decryption and key material info

## Project Structure

```
src
├── bin                      # Binary entry points
│   ├── fhevm-relayer.rs     # Main relayer service
│   └── gateway-processors-mock.rs # Mock Gateway for testing
├── blockchain               # Blockchain connectivity
│   ├── ethereum             # Bindings of contracts
│   ├── gateway              # Gateway interactions
│   └── fhevm                # fhevm interactions
├── config                   # Configuration handling
├── core                     # Core domain types and utilities
│   ├── errors.rs            # Error types
│   ├── event.rs             # Event definitions
│   └── utils.rs             # Helper utilities
├── gateway_processors_mock  # Mock Gateway for testing
├── http                     # HTTP API endpoints
│   ├── http_server.rs       # Web server
│   ├── input_http_listener.rs  # Input proof endpoint
│   ├── keyurl_http_listener.rs # Key URL endpoint
│   └── userdecrypt_http_listener.rs # User decryption endpoint
├── orchestrator             # Event orchestration system
│   ├── orchestrator.rs      # Core orchestrator
│   ├── tokio_event_dispatcher.rs # Async event dispatch
│   └── traits.rs            # Interface definitions
└── transaction              # Transaction management
    ├── helper.rs            # Transaction utilities
    ├── sender.rs            # Transaction sending
    └── service.rs           # Transaction service
```

## Getting Started

### Prerequisites

- Rust 1.70+
- Access to Ethereum fhevm and Gateway RPC endpoints

### Configuration

Configuration is handled via:

- Environment variables
- YAML files in `config/` directory
- Command-line arguments

Example configuration:

```yaml
environment: development
networks:
  fhevm:
    ws_url: "ws://localhost:8545"
    http_url: "http://localhost:8545"
    chain_id: 12345
  rollup:
    ws_url: "ws://localhost:8546"
    http_url: "http://localhost:8546"
    chain_id: 54321
contracts:
  decryption_oracle_address: "0x1234..."
  tfhe_executor_address: "0xabcd..."
  decryption_manager_address: "0x9876..."
  zkpok_manager_address: "0xef01..."
transaction:
  private_key_fhevm_env: "FHEVM_PRIVATE_KEY"
  private_key_gateway_env: "ROLLUP_PRIVATE_KEY"
  gas_limit: 1000000
  max_priority_fee: "2000000000"
  timeout_secs: 60
```

## Running the Service

### Full setup using docker

This setup is using real components:

- coprocessor
- relayer
- zkpok verifier

See [deployments](./deployments/README.md)

### Setup with mock and tests (from binaries)

Type `make` and follow the sequence

### Manual Build and Run

```bash
# Build the service
cargo build --release

# Run the main relayer
./target/release/fhevm-relayer

# Run the gateway mock (for testing)
./target/release/gateway-processors-mock
```

### Environment Variables

- `RUN_MODE`: Select configuration environment (development, production)
- `FHEVM_PRIVATE_KEY`: Private key for fhevm transactions
- `ROLLUP_PRIVATE_KEY`: Private key for Gateway transactions
- `APP_LOG__LEVEL`: Log level (trace, debug, info, warn, error)

## API Endpoints

### Input Proof Verification

```
POST /input-proof
```

Request:

```json
{
  "contractChainId": "123456",
  "contractAddress": "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d",
  "userAddress": "0x12B064FB845C1cc05e9493856a1D637a73e944bE",
  "ciphertextWithZkpok": "abcdef..."
}
```

Response:

```json
{
  "response": {
    "handles": ["0x123...", "0x456..."],
    "signatures": ["0x789...", "0xabc..."]
  }
}
```

### User Decryption

```
POST /user-decrypt
```

Request:

```json
{
  "signature": "0x123...",
  "userAddress": "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d",
  "enc_key": "0x456...",
  "ct_handle": "0x789...",
  "contractAddress": "0x12B064FB845C1cc05e9493856a1D637a73e944bE",
  "chainId": "123456"
}
```

Response:

```json
{
  "response": {
    "reencrypted_shares": ["0x123..."],
    "signatures": ["0x456..."]
  }
}
```

### Key URL

```
GET /keyurl
```

Response:

```json
{
  "response": {
    "fhe_key_info": [
      {
        "fhe_public_key": {
          "data_id": "fhe_public_key_1",
          "urls": ["s3://bucket/id"]
        }
      }
    ],
    "crs": {
      "2048": {
        "data_id": "crs_2048",
        "urls": ["s3://bucket/id"]
      }
    }
  }
}
```

## Development

### Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test '*'
```

### Code Structure

- **Event-Driven**: The system uses events to coordinate between components
- **Handler Pattern**: Each event type has dedicated handlers
- **Asynchronous**: Uses Tokio for async operations
- **Type-Safety**: Leverages Rust's type system for API correctness

## License

BSD 3-Clause Clear License
