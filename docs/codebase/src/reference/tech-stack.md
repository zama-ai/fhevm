# Technology Stack

This page documents the complete technology stack used across the FHEVM codebase.

## Smart Contracts

| Technology | Version | Purpose | Key Files |
|------------|---------|---------|-----------|
| **Solidity** | 0.8.24+ | Smart contract language | `*.sol` |
| **OpenZeppelin** | v5.2 | Security-audited contract libraries | `package.json` |
| **Hardhat** | Latest | Development environment, testing, deployment | `hardhat.config.ts` |
| **Foundry** | Latest | Solidity-native testing framework | `foundry.toml` |
| **TypeChain** | Latest | TypeScript bindings for contracts | Generated in `typechain/` |
| **Prettier** | Latest | Code formatting | `.prettierrc` |
| **Solhint** | Latest | Solidity linting | `.solhint.json` |

### Upgrade Patterns

**UUPS (Universal Upgradeable Proxy Standard):**
- Used for upgradeable contracts
- Implementation in OpenZeppelin v5.2
- Allows contract logic updates without changing addresses

### Standards

**ERC Standards:**
- ERC20 - Fungible tokens
- ERC1363 - Payable token with callbacks
- EIP712 - Typed structured data hashing and signing

## Off-Chain Services (Rust)

| Technology | Version | Purpose | Key Files |
|------------|---------|---------|-----------|
| **Rust** | 1.75+ | Systems programming language | `Cargo.toml` |
| **TFHE-rs** | Latest | FHE operations library | Used in `tfhe-worker` |
| **Tokio** | 1.x | Async runtime | Throughout Rust crates |
| **SQLx** | Latest | Async SQL driver (PostgreSQL) | Database queries |
| **PostgreSQL** | 14+ | Database for job state and ciphertexts | `schema.sql` |
| **Tonic** | Latest | gRPC framework | Service definitions |
| **Protobuf** | 3.x | Serialization format | `*.proto` |
| **Serde** | Latest | Serialization/deserialization | Throughout |
| **Ethers-rs** | Latest | Ethereum library for Rust | Blockchain interaction |

### Key Rust Crates

**Coprocessor:**
- `tfhe-worker` - FHE computation using TFHE-rs
- `scheduler` - Job orchestration with Tokio
- `zkproof-worker` - Zero-knowledge proof generation
- `sns-worker` - Switch and Squash optimizations
- `host-listener` - Blockchain event monitoring
- `gw-listener` - Gateway event monitoring
- `transaction-sender` - Submit results to chain

**KMS Connector:**
- `gw-listener` - Monitor Gateway events
- `kms-worker` - Interface with external KMS Core
- `transaction-sender` - Submit signed responses
- `utils` - Shared utilities

## Cryptography

| Technology | Purpose | Implementation |
|------------|---------|----------------|
| **TFHE** | Fully Homomorphic Encryption scheme | TFHE-rs library |
| **EIP712** | Structured data signing | Ethereum standard |
| **ZK Proofs** | Input verification without revealing data | zkproof-worker |
| **Threshold Signatures** | MPC-based signing (t-of-n) | External KMS Core |
| **ECDSA** | Ethereum transaction signing | ethers-rs |

## Deployment & Infrastructure

| Technology | Purpose | Key Files |
|------------|---------|-----------|
| **Docker** | Containerization | `Dockerfile`, `docker-compose.yml` |
| **Kubernetes** | Container orchestration | `charts/` |
| **Helm** | Kubernetes package manager | `charts/*/Chart.yaml` |
| **GitHub Actions** | CI/CD pipelines | `.github/workflows/` |

### Helm Charts

Located in `/charts/`:
- Deployment manifests for all components
- ConfigMaps for configuration
- Services and Ingress definitions
- Emerging as primary deployment method

## Testing

| Technology | Purpose | Location |
|------------|---------|----------|
| **Hardhat** | Smart contract testing (TypeScript) | `host-contracts/test/` |
| **Foundry** | Smart contract testing (Solidity) | `forge test` commands |
| **Jest** | JavaScript/TypeScript unit tests | Various `test/` dirs |
| **Docker Compose** | E2E integration testing | `test-suite/docker-compose.yml` |
| **SQLite** | Mock FHE backend for fast testing | Test configurations |

### Mock FHE System

**Purpose:** Enable fast testing without expensive FHE operations

**Implementation:**
- SQLite database stores "fake" ciphertexts
- Operations return deterministic results
- API-compatible with real FHE
- Dramatically faster (ms vs seconds)

## Development Tools

| Tool | Purpose |
|------|---------|
| **pnpm** | Node.js package manager (workspaces) |
| **Cargo** | Rust package manager and build tool |
| **Git** | Version control |
| **pre-commit hooks** | Automated formatting and linting |
| **GitHub** | Source control and CI/CD |

## Communication Protocols

| Protocol | Purpose | Used Between |
|----------|---------|--------------|
| **JSON-RPC** | Ethereum node communication | All → Blockchain |
| **gRPC** | Service-to-service communication | Coprocessor components |
| **WebSocket** | Event streaming | Listeners → Blockchain |
| **HTTP REST** | KMS Core API | KMS Connector → KMS Core |

## Cross-Chain

| Technology | Purpose | Component |
|------------|---------|-----------|
| **LayerZero** | Omnichain messaging | Gateway contracts |
| **OFT (Omnichain Fungible Token)** | Cross-chain token transfers | Protocol contracts |

## Build & Development

### Smart Contracts

```bash
# Hardhat
npm run compile     # Compile contracts
npm run test        # Run tests
npm run deploy      # Deploy contracts

# Foundry
forge build         # Compile
forge test          # Test
forge coverage      # Coverage report
```

### Rust Services

```bash
cargo build --release          # Build optimized
cargo test                     # Run tests
cargo clippy                   # Linting
cargo fmt                      # Format code
```

### Full Stack

```bash
# Docker Compose (test-suite)
docker-compose up              # Start all services
docker-compose down            # Stop all services
```

## Language Breakdown

| Language | Primary Use | Lines of Code (approx) |
|----------|-------------|----------------------|
| **Solidity** | Smart contracts | ~30,000 |
| **Rust** | Off-chain services | ~50,000 |
| **TypeScript** | Tests, tooling, SDK | ~20,000 |
| **JavaScript** | Build scripts, configs | ~5,000 |
| **YAML** | Configuration, CI/CD | ~3,000 |

## Version Requirements

**Minimum versions:**
- Node.js: 18.x or later
- Rust: 1.75 or later
- Solidity: 0.8.24
- PostgreSQL: 14.x

**Recommended versions:**
- Latest stable Node.js LTS
- Latest stable Rust
- PostgreSQL 15.x for best performance

---

**Related:**
- [Supporting Infrastructure](../components/infrastructure.md) - Deployment and testing details
- [Component Health](../component-health.md) - Which components are actively evolving
