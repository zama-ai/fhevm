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

### Testing Infrastructure Deep-Dive

FHEVM's testing infrastructure enables fast test iteration by simulating all 30+ FHE operations without expensive cryptographic computation. This section details the mock FHE system architecture, test patterns, and cost tracking mechanisms.

#### Mock FHE System Architecture

The mock system operates through a **four-layer architecture** that maintains complete API compatibility with production while running entirely in-memory:

**Layer 1: SQLite In-Memory Database**
```typescript
const db = new Database(':memory:');
db.serialize(() => db.run('CREATE TABLE IF NOT EXISTS ciphertexts (handle BINARY PRIMARY KEY,clearText TEXT)'));
```
The database stores a simple mapping: ciphertext `handle` (32-byte hex) → plaintext value (string). This serves as the single source of truth for all encrypted values during tests.

**Layer 2: Event Interception**
The system listens to all FHEVM Executor contract events and processes them asynchronously. Each event triggers plaintext arithmetic that mirrors the encrypted operation.

**Layer 3: Operation Simulation**
All FHE operations are simulated using modulo arithmetic with fixed-width types:
```typescript
const NumBits = {
  0: 1n,      // ebool
  2: 8n,      // euint8
  3: 16n,     // euint16
  4: 32n,     // euint32
  5: 64n,     // euint64
  6: 128n,    // euint128
  7: 160n,    // eaddress
  8: 256n,    // euint256
};
```
Operations apply `modulo 2^numBits` to preserve overflow behavior, ensuring test results match production behavior for contract logic validation.

**Layer 4: API Compatibility**
The `instance.ts` module provides production-compatible interfaces that automatically use mocks in Hardhat environments and real FHE elsewhere, making tests portable across environments.

#### Core Testing Functions

**`awaitCoprocessor()`** - Processes all pending FHEVM Executor events:
```typescript
await awaitCoprocessor(); // Process all encrypted operations
```
Call this before asserting on encrypted results to ensure all operations have been simulated.

**`getClearText(handle)`** - Retrieves plaintext values using retry-based polling:
```typescript
export function insertSQL(handle: string, clearText: BigInt | string, replace: boolean = false) {
  if (replace) {
    db.run('INSERT OR REPLACE INTO ciphertexts (handle, clearText) VALUES (?, ?)', [handle, clearText.toString()]);
  } else {
    db.run('INSERT OR IGNORE INTO ciphertexts (handle, clearText) VALUES (?, ?)', [handle, clearText.toString()]);
  }
}

const cleartext = await getClearText(handle); // Returns plaintext as string
```
The function retries up to 100 times to handle asynchronous event processing, simulating the latency of real coprocessor operations.

**`insertSQL(handle, clearText, replace)`** - Stores plaintext mappings:
- `replace=false` (default): Use `INSERT OR IGNORE` for deterministic operations
- `replace=true`: Use `INSERT OR REPLACE` for random operations that need different values on snapshot revert

#### Writing Tests: Patterns & Fixtures

**Standard Test Structure:**
```typescript
describe('EncryptedERC20:HCU', function () {
  before(async function () {
    await initSigners(2);                // One-time: Initialize test accounts
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployEncryptedERC20Fixture();  // Fresh deployment
    this.contractAddress = await contract.getAddress();
    this.erc20 = contract;
    this.instances = await createInstances(this.signers);  // FHE instances per signer
  });

  it('should transfer tokens', async function () {
    // Test implementation
  });
});
```

**Key conventions:**
- `before()`: Call `initSigners(n)` once to set up test accounts
- `beforeEach()`: Deploy fresh contract and create FHE instances
- Store instances and contract references in Mocha's `this` context

**Creating Encrypted Inputs:**
```typescript
const input = this.instances.alice.createEncryptedInput(contractAddress, alice.address);
input.add64(1337);  // Add a 64-bit value
const encrypted = await input.encrypt();

// Pass to contract
await contract.transfer(bob, encrypted.handles[0], encrypted.inputProof);
```

**Debug Decryption (Test-Only):**
```typescript
import { decrypt64, awaitCoprocessor } from '../instance';

const balanceHandle = await erc20.balanceOf(alice.address);
const balance = await decrypt64(balanceHandle);  // Returns bigint
expect(balance).to.equal(1000n);
```
Available functions: `decrypt8()`, `decrypt16()`, `decrypt32()`, `decrypt64()`, `decrypt128()`, `decrypt256()`, `decryptBool()`, `decryptAddress()`.

**WARNING:** These functions require the FHE private key and bypass ACL checks. Never use in production code.

#### HCU Cost Tracking

Track Homomorphic Computation Units (HCU) to measure operation costs:

```typescript
import { getTxHCUFromTxReceipt } from '../coprocessorUtils';

const tx = await erc20.transfer(bob, encrypted.handles[0], encrypted.inputProof);
const receipt = await tx.wait();

const { globalTxHCU, maxTxHCUDepth, HCUDepthPerHandle } = getTxHCUFromTxReceipt(receipt);
console.log('Total HCU:', globalTxHCU);        // Sum of all operation costs
console.log('Max Depth HCU:', maxTxHCUDepth);  // Longest dependency chain cost
```

**Three cost metrics:**
- **`globalTxHCU`**: Total HCU across all operations (useful for gas estimation)
- **`maxTxHCUDepth`**: Maximum cost along any dependency chain (reflects parallel execution benefits)
- **`HCUDepthPerHandle`**: Per-ciphertext cost breakdown for detailed analysis

**Example from EncryptedERC20 transfer:**
```typescript
// Le(149k) + Select(55k) + Sub(162k) = 366k depth
expect(maxTxHCUDepth).to.eq(366_000, 'HCU Depth incorrect');

// All operations: Le + TrivialEncrypt + Select + Add + TrivialEncrypt + Sub
expect(globalTxHCU).to.eq(528_064, 'Total HCU incorrect');
```

Use HCU tracking to:
- Verify expected operation costs during development
- Catch performance regressions in CI
- Optimize contract logic for lower HCU consumption

#### Framework Selection: Hardhat vs Foundry

**Hardhat (TypeScript):**
- Full mock FHE system with SQLite backend
- HCU tracking and cost analysis
- Encrypted input creation and debug decryption
- Best for: Integration tests, encrypted operations, end-to-end flows
- Run: `npm test` in `host-contracts/`

**Foundry (Solidity):**
- Standard Forge tests with FHE type mocks
- No encrypted operations (uses plaintext mocks)
- Fast compilation and execution
- Best for: Unit tests, ACL logic, access control, gas optimization
- Run: `npm run test:forge` in `host-contracts/`

**Key differences:**
- Hardhat tests can perform real encrypted operations (via mocks)
- Foundry tests are faster but cannot test encrypted computation logic
- Use both: Foundry for unit tests, Hardhat for integration tests

#### Key Test Files

| File | Purpose |
|------|---------|
| `host-contracts/test/coprocessorUtils.ts` | Core mock implementation: `insertSQL()`, `getClearText()`, `awaitCoprocessor()`, `getTxHCUFromTxReceipt()` |
| `host-contracts/test/instance.ts` | FHE instance creation, decrypt helpers, dual-mode (mock/real) switching |
| `host-contracts/test/signers.ts` | Test account management: `initSigners()`, `getSigners()` |
| `host-contracts/test/fhevmjsMocked.ts` | Encrypted input creation: `createEncryptedInputMocked()` |
| `host-contracts/test/encryptedERC20/*.fixture.ts` | Example fixture patterns for contract deployment |
| `host-contracts/test/encryptedERC20/*.HCU.ts` | Example HCU tracking and cost analysis patterns |

**Related files:**
- `test-suite/docker-compose.yml` - Full E2E stack orchestration
- `host-contracts/hardhat.config.ts` - Hardhat network and testing configuration
- `library-solidity/foundry.toml` - Foundry testing configuration

**[TODO: Deployment guide]** - Document the Helm charts, Kubernetes deployment process, configuration options, and operational best practices.

**[TODO: Docker compose stack]** - Detail the test-suite docker-compose setup, how components interact, and how to debug issues in local development.

**[TODO: CI/CD pipeline]** - Explain the GitHub Actions workflows, testing strategy, and release process.

---

**Related:**
- [Component Health](../component-health.md) - Infrastructure components have minimal activity
- [Technology Stack](../reference/tech-stack.md) - Tools and frameworks used
