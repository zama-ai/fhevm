# Supporting Infrastructure 📦

**Location**: Various directories
**Status**: Infrastructure / Maintenance
**Purpose**: Deployment, testing, and operational tooling

## Overview

Supporting infrastructure enables development, testing, and deployment of the FHEVM stack. While not part of the core protocol, these components are essential for operating and maintaining FHEVM systems.

## Directories

| Directory                   | Purpose                                   |
| --------------------------- | ----------------------------------------- |
| `/charts/`                  | Helm charts for Kubernetes deployment     |
| `/test-suite/`              | E2E integration tests with docker-compose |
| `/golden-container-images/` | Base Docker images for Node.js and Rust   |
| `/docs/`                    | Gitbook documentation source              |
| `/sdk/`                     | Rust SDK for building applications        |
| `/ci/`                      | CI/CD pipeline configurations             |

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
const db = new Database(":memory:");
db.serialize(() => db.run("CREATE TABLE IF NOT EXISTS ciphertexts (handle BINARY PRIMARY KEY,clearText TEXT)"));
```

The database stores a simple mapping: ciphertext `handle` (32-byte hex) → plaintext value (string). This serves as the single source of truth for all encrypted values during tests.

**Layer 2: Event Interception**
The system listens to all FHEVM Executor contract events and processes them asynchronously. Each event triggers plaintext arithmetic that mirrors the encrypted operation.

**Layer 3: Operation Simulation**
All FHE operations are simulated using modulo arithmetic with fixed-width types:

```typescript
const NumBits = {
  0: 1n, // ebool
  2: 8n, // euint8
  3: 16n, // euint16
  4: 32n, // euint32
  5: 64n, // euint64
  6: 128n, // euint128
  7: 160n, // eaddress
  8: 256n, // euint256
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
    db.run("INSERT OR REPLACE INTO ciphertexts (handle, clearText) VALUES (?, ?)", [handle, clearText.toString()]);
  } else {
    db.run("INSERT OR IGNORE INTO ciphertexts (handle, clearText) VALUES (?, ?)", [handle, clearText.toString()]);
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
describe("EncryptedERC20:HCU", function () {
  before(async function () {
    await initSigners(2); // One-time: Initialize test accounts
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployEncryptedERC20Fixture(); // Fresh deployment
    this.contractAddress = await contract.getAddress();
    this.erc20 = contract;
    this.instances = await createInstances(this.signers); // FHE instances per signer
  });

  it("should transfer tokens", async function () {
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
input.add64(1337); // Add a 64-bit value
const encrypted = await input.encrypt();

// Pass to contract
await contract.transfer(bob, encrypted.handles[0], encrypted.inputProof);
```

**Debug Decryption (Test-Only):**

```typescript
import { decrypt64, awaitCoprocessor } from "../instance";

const balanceHandle = await erc20.balanceOf(alice.address);
const balance = await decrypt64(balanceHandle); // Returns bigint
expect(balance).to.equal(1000n);
```

Available functions: `decrypt8()`, `decrypt16()`, `decrypt32()`, `decrypt64()`, `decrypt128()`, `decrypt256()`, `decryptBool()`, `decryptAddress()`.

**WARNING:** These functions require the FHE private key and bypass ACL checks. Never use in production code.

#### HCU Cost Tracking

Track Homomorphic Computation Units (HCU) to measure operation costs:

```typescript
import { getTxHCUFromTxReceipt } from "../coprocessorUtils";

const tx = await erc20.transfer(bob, encrypted.handles[0], encrypted.inputProof);
const receipt = await tx.wait();

const { globalTxHCU, maxTxHCUDepth, HCUDepthPerHandle } = getTxHCUFromTxReceipt(receipt);
console.log("Total HCU:", globalTxHCU); // Sum of all operation costs
console.log("Max Depth HCU:", maxTxHCUDepth); // Longest dependency chain cost
```

**Three cost metrics:**

- **`globalTxHCU`**: Total HCU across all operations (useful for gas estimation)
- **`maxTxHCUDepth`**: Maximum cost along any dependency chain (reflects parallel execution benefits)
- **`HCUDepthPerHandle`**: Per-ciphertext cost breakdown for detailed analysis

**Example from EncryptedERC20 transfer:**

```typescript
// Le(149k) + Select(55k) + Sub(162k) = 366k depth
expect(maxTxHCUDepth).to.eq(366_000, "HCU Depth incorrect");

// All operations: Le + TrivialEncrypt + Select + Add + TrivialEncrypt + Sub
expect(globalTxHCU).to.eq(528_064, "Total HCU incorrect");
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

| File                                              | Purpose                                                                                                    |
| ------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `host-contracts/test/coprocessorUtils.ts`         | Core mock implementation: `insertSQL()`, `getClearText()`, `awaitCoprocessor()`, `getTxHCUFromTxReceipt()` |
| `host-contracts/test/instance.ts`                 | FHE instance creation, decrypt helpers, dual-mode (mock/real) switching                                    |
| `host-contracts/test/signers.ts`                  | Test account management: `initSigners()`, `getSigners()`                                                   |
| `host-contracts/test/fhevmjsMocked.ts`            | Encrypted input creation: `createEncryptedInputMocked()`                                                   |
| `host-contracts/test/encryptedERC20/*.fixture.ts` | Example fixture patterns for contract deployment                                                           |
| `host-contracts/test/encryptedERC20/*.HCU.ts`     | Example HCU tracking and cost analysis patterns                                                            |

**Related files:**

- `test-suite/docker-compose.yml` - Full E2E stack orchestration
- `host-contracts/hardhat.config.ts` - Hardhat network and testing configuration
- `library-solidity/foundry.toml` - Foundry testing configuration

### Deployment Guide Deep-Dive

FHEVM's deployment infrastructure enables production Kubernetes deployments through Helm charts. This section details the chart architecture, deployment workflow, configuration patterns, and operational best practices for running FHEVM at scale.

#### Helm Chart Architecture

The FHEVM stack deploys through **5 specialized Helm charts** that can be installed independently or combined:

| Chart                        | Version | Purpose                                               | Key Components                                           |
| ---------------------------- | ------- | ----------------------------------------------------- | -------------------------------------------------------- |
| **anvil-node**               | v0.5.0  | Local Ethereum blockchain for development/testing     | StatefulSet with persistent storage, single replica      |
| **contracts**                | v0.7.5  | Smart contract deployment for Gateway and Host chains | Deployment Jobs with Helm hooks, ConfigMap for addresses |
| **coprocessor**              | v0.7.8  | Core FHE computation and event processing             | 10+ components: listeners, TFHE/ZK workers, tx-sender    |
| **kms-connector**            | v1.3.1  | Key Management Service bridge for Gateway chain       | Gateway listener, KMS worker, transaction sender         |
| **coprocessor-sql-exporter** | v1.0.0  | Prometheus metrics exporter for database monitoring   | SQL exporter with ServiceMonitor                         |

**Chart Structure:** Each chart follows standard Helm organization with `Chart.yaml` (metadata), `values.yaml` (configuration defaults), and `templates/` (Kubernetes manifests). All images are hosted at `ghcr.io/zama-ai/fhevm/`.

**Deployment Dependencies:** The typical deployment order is: (1) anvil-node (if using local blockchain), (2) contracts (gateway and host), (3) coprocessor and kms-connector (can run in parallel), (4) coprocessor-sql-exporter (optional monitoring).

#### Core Deployment Patterns

**Configuration Management:** FHEVM charts follow a **secrets-first pattern** where sensitive configuration is injected from Kubernetes Secrets:

```yaml
# Database configuration in coprocessor values.yaml
database:
  secret:
    name: coprocessor-db-url
    key: coprocessor-db-url
    value: "postgresql://user:pass@host:5432/coprocessor"

# Injected into containers as environment variables
env:
  - name: DATABASE_URL
    valueFrom:
      secretKeyRef:
        name: coprocessor-db-url
        key: coprocessor-db-url
```

**Required Secrets:** Before deployment, create these Kubernetes secrets in your target namespace:

- `coprocessor-db-url` - Database connection string
- `coprocessor-api-key` - API authentication for coprocessor endpoints
- `coprocessor-key` - Coprocessor cryptographic key
- `kms-connector-tx-sender` - Transaction sender wallet credentials
- `registry-credentials` - Docker registry authentication (if using private registry)

**Initialization with Helm Hooks:** The charts use Helm hooks to ensure proper initialization order. For example, the contracts chart uses a `pre-install` hook to run database migrations before deploying main services:

```yaml
annotations:
  helm.sh/hook: pre-install
  helm.sh/hook-weight: "-1"
```

This ensures database schemas exist before services attempt connections. The coprocessor chart similarly uses hooks for secret initialization and configuration setup.

#### Production Deployment

**Installing Charts:** Charts are distributed via OCI registry. Basic installation workflow:

```bash
# Authenticate to chart registry
helm registry login ghcr.io/zama-ai/fhevm/charts

# Install coprocessor with custom values
helm install coprocessor oci://ghcr.io/zama-ai/fhevm/charts/coprocessor \
  --version 0.7.8 \
  --namespace fhevm \
  --values custom-values.yaml

# Install contracts for gateway and host chains
helm install contracts oci://ghcr.io/zama-ai/fhevm/charts/contracts \
  --version 0.7.5 \
  --namespace fhevm \
  --set scDeploy.enabled=true
```

**Component Configuration:** Enable or disable components via `.enabled` flags in values.yaml:

```yaml
# Coprocessor components (can selectively enable)
hostListener:
  enabled: true
  replicas: 1

hostListenerPoller:
  enabled: false # Alternative polling-based listener

tfheWorker:
  enabled: true
  replicas: 2

zkProofWorker:
  enabled: true

txSender:
  enabled: true
```

**Horizontal Pod Autoscaling:** Compute-intensive workers support HPA for automatic scaling:

```yaml
tfheWorker:
  hpa:
    enabled: true
    minReplicas: 1
    maxReplicas: 10
    targetCPUUtilizationPercentage: 80
    behavior:
      scaleDown:
        stabilizationWindowSeconds: 300
      scaleUp:
        stabilizationWindowSeconds: 60
```

**Observability:** All services expose Prometheus metrics on port 9100. Enable ServiceMonitors for automatic Prometheus scraping:

```yaml
tfheWorker:
  serviceMonitor:
    enabled: true
    interval: 30s
```

**Resource Configuration:** Set resource requests/limits based on workload. Default pattern:

```yaml
resources:
  requests:
    cpu: 100m
    memory: 256Mi
  limits:
    cpu: 500m
    memory: 512Mi
```

Adjust for production: TFHE workers benefit from higher CPU limits (2000m+), while listeners need minimal resources.

#### Operational Best Practices

| Practice            | Implementation                                                                           | Reference                                      |
| ------------------- | ---------------------------------------------------------------------------------------- | ---------------------------------------------- |
| **Monitoring**      | ServiceMonitor resources + Prometheus scraping on port 9100                              | `coprocessor/templates/*-service-monitor.yaml` |
| **Health Checks**   | Liveness/readiness probes on HTTP port 8080 (`/healthz`, `/liveness`)                    | `values.yaml` probe configuration              |
| **Scaling**         | HPA with custom scale-up/scale-down behavior for workers                                 | `tfheWorker.hpa`, `zkProofWorker.hpa`          |
| **Security**        | Non-root containers (`runAsNonRoot: true`, `runAsUser: 10000`), RBAC via ServiceAccounts | `scDeploy.securityContext`                     |
| **Persistence**     | StatefulSets for stateful services (anvil-node), PVCs for Jobs (contract deployment)     | `anvil-statefulset.yaml`, `sc-deploy-pvc.yaml` |
| **Rolling Updates** | Controlled rollout: `maxSurge: 1`, `maxUnavailable: 0` for zero-downtime deployments     | Deployment strategy configuration              |

**Development vs. Production:**

- **Development**: Use anvil-node chart for local blockchain, single replicas, minimal resources, disable ServiceMonitors
- **Production**: External RPC endpoints, multiple replicas with HPA, higher resource limits, enable all monitoring, configure node affinity for GPU nodes (if using accelerated FHE)

**Troubleshooting:**

- Check pod logs: `kubectl logs -n fhevm <pod-name>`
- View events: `kubectl get events -n fhevm --sort-by='.lastTimestamp'`
- Check Helm release status: `helm status coprocessor -n fhevm`
- Verify secrets exist: `kubectl get secrets -n fhevm`

**Operational Monitoring:** Key metrics to track:

- `allowed_handles_txn_sent` - Transaction submission rate
- `computations_completion` - FHE computation throughput
- `ciphertexts` - Pending ciphertexts in database
- Worker replica count and CPU utilization (for HPA tuning)

#### Key Deployment Files

| File                                                                   | Purpose                                                                                |
| ---------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `charts/coprocessor/values.yaml`                                       | Core FHE computation configuration with 10+ components (listeners, workers, tx-sender) |
| `charts/contracts/values.yaml`                                         | Smart contract deployment for Gateway and Host chains, includes upgrade workflows      |
| `charts/kms-connector/values.yaml`                                     | Key Management Service bridge configuration, AWS KMS integration support               |
| `charts/kms-connector/README.md`                                       | Comprehensive deployment guide with Helm commands and configuration examples           |
| `charts/anvil-node/values.yaml`                                        | Local Ethereum blockchain for development, configurable block time and chain ID        |
| `charts/coprocessor/templates/coprocessor-tfhe-worker-deployment.yaml` | TFHE worker deployment pattern with HPA and ServiceMonitor                             |
| `charts/contracts/templates/sc-deploy-job.yaml`                        | Smart contract deployment Job with Helm hooks for initialization order                 |

**[TODO: Docker compose stack]** - Detail the test-suite docker-compose setup, how components interact, and how to debug issues in local development.

**[TODO: CI/CD pipeline]** - Explain the GitHub Actions workflows, testing strategy, and release process.

---

**Related:**

- [Component Health](../component-health.md) - Infrastructure components have minimal activity
- [Technology Stack](../reference/tech-stack.md) - Tools and frameworks used
