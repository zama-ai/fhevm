# Multi-Coprocessor Consensus Tests (E1)

## Prerequisites

1. **fhevm-cli** installed and configured
2. Docker running with sufficient resources (3 coprocessor instances = ~15 containers)
3. `pg` npm package available in the e2e project (`npm install pg @types/pg`)

## Running the Tests

### 1. Start the 3-of-3 stack

```bash
cd test-suite/fhevm
./fhevm-cli up --scenario three-of-three --target latest-main --build
```

This spins up 3 coprocessor instances (each with tfhe-worker, host-listener, sns-worker, transaction-sender, gw-listener) sharing one Anvil chain with isolated databases.

Wait for all services to be healthy:
```bash
./fhevm-cli status
```

### 2. Run the consensus tests

```bash
./fhevm-cli test --grep "Multi-Coprocessor Consensus"
```

Or manually:
```bash
cd test-suite/e2e
GATEWAY_RPC_URL=http://localhost:8545 \
CIPHERTEXT_COMMITS_ADDRESS=0x<deployed-address> \
DATABASE_URL_0=postgresql://postgres:postgres@localhost:5432/coprocessor \
DATABASE_URL_1=postgresql://postgres:postgres@localhost:5432/coprocessor_1 \
DATABASE_URL_2=postgresql://postgres:postgres@localhost:5432/coprocessor_2 \
npx hardhat test --network staging test/consensus/multiCoprocessorConsensus.ts
```

### 3. For C4b (missing submission), use the 2-of-3 scenario

```bash
./fhevm-cli up --scenario two-of-three --target latest-main --build
./fhevm-cli test --grep "C4b"
```

### 4. Branch-aware mode

Branch-aware, block-scoped execution is the only runtime path. The digest
helpers in `helpers.ts` target
`ciphertext_digest_branch` to match the runtime read path; drift-injection
tests (C2a, C7a, C7b) exercise the branch-aware drift detector against
that table.

## Test Cases

| Test | What it proves | Scenario |
|------|---------------|----------|
| C1 | 3/3 consensus baseline | three-of-three |
| C4a | Consensus timeout (offline coprocessor) | three-of-three |
| C4b | Missing submission after partial consensus | two-of-three |
| C5 | Block-scoped determinism across 3 coprocessors | three-of-three |
| C6 | gw-listener restart resilience | three-of-three |
| C2a | Divergence detection (DB injection) | three-of-three |
| C7a | Multi-row local-consensus, one matches (no drift) | three-of-three |
| C7b | Multi-row local-consensus, none match (drift) | three-of-three |

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `GATEWAY_RPC_URL` | yes | Gateway chain RPC endpoint |
| `CIPHERTEXT_COMMITS_ADDRESS` | yes | Deployed CiphertextCommits contract |
| `DATABASE_URL_0` | no | Coprocessor 0 database (default: computed from POSTGRES_HOST) |
| `DATABASE_URL_1` | no | Coprocessor 1 database |
| `DATABASE_URL_2` | no | Coprocessor 2 database |

## E2: Dual-Anvil Fork Testing Infrastructure

### Architecture

```
  Anvil A (canonical, host-node:8545 / localhost:8545)      ←── coprocessors 0, 1
  Anvil B (fork, fork-anvil:8546 / localhost:8547)           ←── coprocessor 2
  Gateway (shared)            ←── all 3 submit consensus digests
```

Both Anvils share the same chain ID, mnemonic, and genesis. The test orchestrator submits different transactions at the fork point to create divergent block histories.

### Starting the fork stack

```bash
cd test-suite/fhevm

# Boot with the fork scenario (routes coprocessor 2 to fork-anvil)
./fhevm-cli up --scenario three-of-three-fork --build

# Verify both Anvils are running
curl -s http://localhost:8545 -X POST -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
curl -s http://localhost:8547 -X POST -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
```

### Running fork tests (E3)

```bash
# The test runs inside fhevm-test-suite-e2e-debug; use Docker-network URLs.
CANONICAL_RPC_URL=http://host-node:8545 \
FORK_RPC_URL=http://fork-anvil:8546 \
./fhevm-cli test --grep "Real-Fork"
```

### Fork orchestration helper

`forkHelper.ts` provides utilities for fork tests:

| Function | Purpose |
|----------|---------|
| `getCanonicalProvider()` / `getForkProvider()` | Get ethers providers for each Anvil |
| `submitDivergentTransactions()` | Submit different txs to each Anvil in parallel |
| `verifyForkDivergence(blockNumber)` | Assert block hashes differ at fork point |
| `advancePastFinality(lag)` | Mine blocks on canonical Anvil to trigger finalization |
| `mineBlocks(provider, count)` | Mine empty blocks on a specific Anvil |

## E3: Real-Fork Consensus Tests (C2b, C3, C3b)

### Running the fork tests

```bash
cd test-suite/fhevm

# Boot the fork scenario (coprocessor 2 → fork Anvil)
./fhevm-cli up --scenario three-of-three-fork --build

# Run the real-fork tests
./fhevm-cli test real-fork-consensus
```

### What the tests do

**C2b (Full-fork equivocation):**
1. Deploys an ERC20 contract on the canonical chain
2. Syncs the fork Anvil's state so it has the same contract
3. Broadcasts the same signed mint transaction on both branches
4. Verifies the block hashes and resulting ciphertext digests diverged
5. Waits for submissions — coprocessors 0,1 agree; coprocessor 2 diverges
6. Asserts no consensus + drift detected

**C3 (Recovery after finalization):**
1. Creates independent divergent work and advances the canonical branch past finality
2. Asserts the exact winning block hash is finalized on the canonical coprocessors
3. Resyncs the fork Anvil to canonical state and restarts coprocessor 2's chain-facing services
4. Asserts the losing block is orphaned and all branch-specific rows are cleaned up
5. Asserts the winning block is finalized on the recovered coprocessor
6. Submits new work and verifies all 3 coprocessors agree on its digest

**C3b (Settled reorg recovery):**
1. Creates divergent work and advances the canonical branch through the RFC-011 settlement boundary
2. Resyncs the fork Anvil and verifies settlement never regresses during recovery
3. Asserts the losing branch is orphaned while the canonical CT64 and CT128 digests survive unchanged
4. Verifies the pre-reorg canonical ciphertext reaches 3-of-3 consensus and decrypts successfully
5. Submits new work and verifies consensus remains healthy after recovery

## Troubleshooting

- **Tests hang waiting for consensus:** Check that all 3 coprocessor instances are running and healthy. Use `docker ps` to verify container status.
- **DB connection errors:** Verify DATABASE_URL_N values point to the correct per-instance databases.
- **C7b causes afterEach to fail:** Check that the test calls `ignoreWatchdogCiphertextHandle()` for only the intentionally divergent handle.
- **Docker permission errors:** Ensure the test runner has access to the Docker socket (`/var/run/docker.sock`).
