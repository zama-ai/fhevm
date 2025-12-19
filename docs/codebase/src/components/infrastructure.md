# Supporting Infrastructure 📦

**Location**: Various directories
**Status**: Infrastructure / Maintenance
**Purpose**: Deployment, testing, and operational tooling

## Overview

Supporting infrastructure enables development, testing, and deployment of the FHEVM stack. While not part of the core protocol, these components are essential for operating and maintaining FHEVM systems.

## Directories

| Directory | Purpose |
|-----------|---------|
| `/charts/` | Helm charts for Kubernetes deployment |
| `/test-suite/` | E2E integration tests with docker-compose |
| `/golden-container-images/` | Base Docker images for Node.js and Rust |
| `/docs/` | Gitbook documentation source |
| `/sdk/` | Rust SDK for building applications |
| `/ci/` | CI/CD pipeline configurations |

## Testing Infrastructure

### Test Types

**Unit/Integration Tests:**
- **Hardhat tests**: Located in `host-contracts/test/`, `library-solidity/test/`
- **Foundry tests**: Solidity-native tests via `forge test`
- Written in TypeScript (Hardhat) or Solidity (Foundry)

**E2E Tests:**
- **Full-stack tests**: Located in `test-suite/`
- **Orchestration**: Docker Compose brings up entire stack
- **Mock FHE**: SQLite-backed mocking for fast testing without real FHE

### Mock FHE System

For development and testing, FHEVM includes a mock FHE system:
- Replaces expensive FHE operations with simple encryption
- Backed by SQLite for ciphertext storage
- Enables fast iteration without coprocessor
- Maintains API compatibility with real FHE

### Key Test Files

- `test-suite/docker-compose.yml` - Full stack orchestration
- `host-contracts/hardhat.config.ts` - Hardhat configuration
- `library-solidity/foundry.toml` - Foundry configuration

## Deployment Infrastructure

### Kubernetes / Helm

Located in `/charts/`:
- Helm charts for deploying FHEVM components
- Kubernetes manifests and configurations
- Emerging as primary deployment method

### Docker Images

Located in `/golden-container-images/`:
- Base images for Node.js services
- Base images for Rust services
- Standardized build environments

## Development SDK

Located in `/sdk/`:
- Rust SDK for building FHEVM applications
- Currently in maintenance mode (low activity)
- Provides client libraries for interacting with FHEVM

## CI/CD

Located in `/.github/workflows/`:
- GitHub Actions workflow definitions
- Automated testing on PR
- Build and publish pipelines
- Security scanning

## Key Files

- `test-suite/docker-compose.yml` - Full stack orchestration
- `host-contracts/hardhat.config.ts` - Hardhat configuration
- `.github/workflows/` - CI pipeline definitions

## Areas for Deeper Documentation

**[TODO: Testing infrastructure deep-dive]** - Document the mock FHE system, test fixtures, E2E testing patterns, and how to write effective tests for confidential contracts.

**[TODO: Deployment guide]** - Document the Helm charts, Kubernetes deployment process, configuration options, and operational best practices.

### E2E Testing Patterns

FHEVM's E2E tests validate complete encrypted workflows from input encryption through FHE computation to decryption. Unlike unit tests with mocked FHE operations, E2E tests run against two local Anvil chains orchestrated by Docker Compose, verifying actual cryptographic operations. Tests use the Mocha framework with Hardhat, following a fixture pattern that separates deployment logic from test assertions.

#### Two-Chain Architecture

E2E tests require two independent EVM chains:

- **Host Chain** (ID: 12345, port 8545): Runs FHEVM contracts, ACL, and application logic
- **Gateway Chain** (ID: 54321, port 8546): Manages KMS nodes, decryption requests, and input verification

This separation mirrors production architecture where FHE infrastructure is isolated from application contracts. Both chains communicate via contract address registration stored in a shared Docker volume (`addresses-volume`), enabling tests to bridge operations across chains.

```
┌─────────────────────┐         ┌──────────────────────┐
│ Host Chain          │         │ Gateway Chain        │
│ (12345:8545)        │◄───────►│ (54321:8546)         │
│ - FHEVM Contracts   │         │ - KMS                │
│ - ACL               │         │ - Input Verifier     │
└─────────────────────┘         └──────────────────────┘
         │                               │
         └───────────────────────────────┘
            Shared addresses-volume
```

#### Docker Compose Infrastructure

**Gateway Stack** (`gateway-contracts/docker-compose.yml`): Deploys 9 services including `anvil-node` (port 8546), contract deployments (`deploy-gateway-contracts`), host chain registration (`add-host-chains`), and cryptographic setup (`trigger-keygen`, `trigger-crsgen`).

**Host Stack** (`host-contracts/docker-compose.yml`): Deploys 3 services including `anvil-node` (port 8545), FHEVM contracts (`fhevm-sc-deploy`), and pauser configuration.

Services use `depends_on` with `service_completed_successfully` to enforce deployment order:

```yaml
deploy-gateway-contracts:
  depends_on:
    anvil-node:
      condition: service_started
    deploy-mocked-zama-oft:
      condition: service_completed_successfully
  volumes:
    - addresses-volume:/app/addresses  # Shared contract discovery
```

#### Test Anatomy

Tests follow a consistent pattern with fixtures handling deployment and test files containing assertions:

```typescript
// From test-suite/e2e/test/encryptedERC20/EncryptedERC20.ts:10-20
describe('EncryptedERC20', function () {
  before(async function () {
    await initSigners(2);  // Initialize named test accounts
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployEncryptedERC20Fixture();
    this.contractAddress = await contract.getAddress();
    this.erc20 = contract;
    this.instances = await createInstances(this.signers);
  });
});
```

**Named Signers**: Tests use pre-defined accounts (alice, bob, carol, dave, eve, fred) with automatic fauceting via `initSigners()`. See `host-contracts/test/signers.ts:10-16` for the interface definition.

**Fixture Pattern**: Deployment logic lives in separate files:

```typescript
// From test-suite/e2e/test/encryptedERC20/EncryptedERC20.fixture.ts:6-14
export async function deployEncryptedERC20Fixture() {
  const signers = await getSigners();
  const contractFactory = await ethers.getContractFactory('EncryptedERC20');
  const contract = await contractFactory.connect(signers.alice)
    .deploy('Naraggara', 'NARA');
  await contract.waitForDeployment();
  return contract;
}
```

**Typical Test Flow**: Mint → Encrypt Input → Execute → Decrypt → Verify. See `test-suite/e2e/test/encryptedERC20/EncryptedERC20.ts:58-103` for a complete transfer example using encrypted amounts.

#### Running Tests

```bash
# Run all E2E tests
cd test-suite/e2e
./run-tests.sh

# Run specific test pattern
./run-tests.sh -g "transfer tokens"

# Verbose output for debugging
./run-tests.sh -v -g "pattern"

# Target specific network (default: staging = chain 12345)
./run-tests.sh -n staging -g "pattern"
```

**Configuration**: Tests use the `staging` network (chain ID 12345) with Mocha timeout set to 300000ms (5 minutes) to accommodate slow FHE operations. See `test-suite/e2e/hardhat.config.ts:75-88`.

#### Common Issues & Debugging

| Issue | Symptom | Solution |
|-------|---------|----------|
| **Address Not Found** | `Cannot read address` error | Verify `addresses-volume` mounted; ensure `docker compose up` completed successfully |
| **Chain ID Mismatch** | Tests fail with wrong chain ID | Check `.env` has `CHAIN_ID_GATEWAY=54321`; verify `RPC_URL` points to correct port |
| **Faucet Failure** | `account sequence mismatch` | Retry logic handles automatically; restart Docker if persistent |
| **Test Timeout** | Exceeded 300000ms | Normal for complex FHE operations; increase timeout in `hardhat.config.ts` if needed |
| **Relayer Connection** | Failed to connect to relayer | Verify `RELAYER_URL` in `.env`; check gateway services running via `docker compose ps` |

**Debugging Commands**:
```bash
# View service logs
docker compose logs deploy-gateway-contracts

# Inspect shared volume
docker volume inspect fhevm_addresses-volume

# Verify chain ID
curl http://localhost:8545 -X POST \
  -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}'
```

**Pro Tips**: Use `-g "pattern"` to isolate failing tests. Check `test-suite/e2e/test/instance.ts` for required environment variables. Test times of 30+ seconds are normal for FHE operations.

**[TODO: CI/CD pipeline]** - Explain the GitHub Actions workflows, testing strategy, and release process.

---

**Related:**
- [Component Health](../component-health.md) - Infrastructure components have minimal activity
- [Technology Stack](../reference/tech-stack.md) - Tools and frameworks used
