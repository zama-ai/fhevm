# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is the **fhevm-relayer** - a bridge service connecting fhevm (Fully Homomorphic Encryption Virtual Machine) blockchains with Gateway systems. It enables secure processing of encrypted data through:

- **Public Decryption**: Decrypt ciphertexts to plain text (with proper permissions)
- **Input Proof Verification**: Verify ZKPoK (Zero Knowledge Proof of Plaintext Knowledge) for input ciphertexts
- **User Decryption**: Decrypt ciphertexts for users with proper access permissions
- **Key Material**: Expose FHE public keys and evaluation keys

## Common Development Commands

### Building and Running
```bash
# Build the Rust relayer service
cargo build --release
cargo build --bin fhevm-relayer
cargo build --bin gateway-processors-mock

# Run the main relayer service
cargo run --bin fhevm-relayer

# Run in mock mode for testing
make start-fhevm-relayer-mock-mode
```

### Testing
```bash
# Run Rust unit tests
cargo test

# Run integration tests
cargo test --test '*'

# Run specific Hardhat tests with grep filter
./run-tests.sh "test user input uint64"

# Run API tests
make run-test-api
```

### Gateway L2 Smart Contracts
```bash
# In contracts/gateway-l2/ directory
npm install
npm run test
npm run coverage
npm run prettier:check
npm run prettier:write
npm run lint:sol
```

### Development Setup
```bash
# Setup mock environment
make setup-mock

# Configure relayer
make config-fhevm-relayer

# Start development services (run in separate terminals)
make start-anvil-nodes          # Terminal 1
make start-gateway-processors-mock  # Terminal 2
make start-fhevm-relayer-mock-mode  # Terminal 3

# Test endpoints
make run-test-input-proof
make run-test-user-decrypt
make run-test-public-decrypt-http
make run-test-keyurl
```

## Architecture Overview

The system follows an **event-driven architecture** with these key components:

### Core Components
- **Orchestrator**: Central coordinator using event-driven flow management
- **Event Handlers**: Process specific event types (PublicDecryptFhevmHandler, PublicDecryptGatewayHandler, UserDecryptGatewayHandler, InputProofGatewayHandler)
- **Transaction Service**: Reliable blockchain transaction management
- **HTTP Server**: RESTful API endpoints for external interactions

### Event-Driven Flow
```
[fhevm] → [fhevm listener] → [Orchestrator] → [gateway Handler]
                                      ↓
[fhevm] ← [fhevm Handler] ← [Orchestrator] ← [gateway Listener]
```

### Key Abstractions
- **Events**: Each processing step modeled as an event type with associated handlers
- **Request ID**: UUID linking all events in an end-to-end process
- **Hooks**: Meta-handlers for cross-cutting concerns (persistence, logging, metrics)
- **Dispatchers**: Pluggable execution strategies (Tokio, distributed queues, etc.)

### Request Deduplication Pattern

The system uses **content-hash-based deduplication** to prevent duplicate processing of identical requests:

#### JobId Type
- **JobId**: Simple `[u8; 32]` type alias representing a 32-byte identifier
- **User requests** (input-proof, user-decrypt, public-decrypt): JobId is the SHA-256 content hash of request payload
- **Internal events** (keyurl, gateway listener): JobId is the `INTERNAL_EVENT_JOB_ID` constant (`[0u8; 32]`)

#### Deduplication Mechanism
1. **Content Hashing**: Each request type implements `ContentHasher` trait computing SHA-256 from all semantically meaningful fields
2. **Database Storage**: Requests stored with both:
   - `ext_job_id` (UUID): External reference returned to users
   - `int_job_id` (BYTEA): Content hash for internal deduplication
3. **Partial Unique Index**: Database enforces uniqueness on `int_job_id` only for active requests (excludes 'completed', 'failure', 'timed_out')
4. **Check-Then-Insert**: HTTP handlers check for active requests before insertion, returning existing `ext_job_id` for duplicates

#### Fields Included in Content Hash
- **Input Proof**: contract_chain_id, contract_address, user_address, ciphetext_with_zk_proof, extra_data
- **User Decrypt**: contract_chain_id, contract_address, user_address, ciphertext_handles, share_addresses, share_signers
- **Public Decrypt**: contract_chain_id, contract_address, ciphertext_digest

#### Benefits
- Automatic deduplication prevents redundant processing
- Same request → same job_id → same ext_job_id (while active)
- Failed/timed-out requests can be retried (create new ext_job_id)
- No enumeration or branching complexity - JobId is just bytes

## Project Structure

```
src/
├── bin/                     # Binary entry points
│   ├── fhevm-relayer.rs     # Main relayer service
│   └── gateway-processors-mock.rs # Mock Gateway for testing
├── blockchain/              # Blockchain connectivity
│   ├── ethereum/            # Ethereum bindings and client
│   ├── gateway/             # Gateway interactions
│   └── fhevm/               # fhevm interactions
├── config/                  # Configuration handling
├── core/                    # Core domain types
│   ├── errors.rs            # Error definitions
│   ├── event.rs             # Event system definitions
│   └── utils.rs             # Utility functions
├── http/                    # HTTP API endpoints
│   ├── http_server.rs       # Web server
│   ├── input_http_listener.rs    # Input proof endpoint
│   ├── keyurl_http_listener.rs   # Key URL endpoint
│   ├── public_decrypt_http_listener.rs # Public decrypt endpoint
│   └── userdecrypt_http_listener.rs    # User decryption endpoint
├── orchestrator/            # Event orchestration system
│   ├── orchestrator.rs      # Core orchestrator
│   ├── tokio_event_dispatcher.rs # Async event dispatch
│   └── traits.rs            # Interface definitions
├── store/                   # Data persistence
│   ├── event.rs             # Event storage
│   ├── key_value_db/        # Key-value store implementations
│   └── block_number.rs      # Block number tracking
└── transaction/             # Transaction management
    ├── helper.rs            # Transaction utilities
    ├── sender.rs            # Transaction sending
    └── service.rs           # Transaction service
```

### Smart Contracts
- **contracts/gateway-l2/**: Gateway L2 smart contracts (Hardhat/TypeScript)
- **hardhat/**: Additional Hardhat contracts and testing infrastructure

## Configuration

The service uses hierarchical configuration:
1. Environment variables (highest priority)
2. YAML configuration files in `config/`
3. Command-line arguments
4. Default values

Key configuration areas:
- **Networks**: fhevm and gateway blockchain endpoints
- **Contracts**: Smart contract addresses
- **Transaction**: Private keys, gas settings, timeouts
- **Logging**: Structured logging with tracing spans

## API Endpoints

- `POST /input-proof`: Input proof verification
- `POST /user-decrypt`: User decryption requests
- `POST /public-decrypt`: Public decryption requests
- `GET /keyurl`: Key material URLs

## Development Notes

- The system is designed for high reliability with transaction retry mechanisms
- Comprehensive observability through structured metrics and tracing
- Event-driven architecture allows for flexible flow modifications
- Multi-chain support with separate configurations for fhevm and gateway networks
- Mock implementations available for testing without external dependencies

## Testing Strategy

- **Unit Tests**: `cargo test` for Rust components
- **Integration Tests**: Full end-to-end flows with mock services
- **Smart Contract Tests**: Hardhat test suites for contract logic
- **API Tests**: HTTP endpoint validation via curl scripts