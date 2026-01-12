# E2E Testing CLI Improvements

**Objective**: Improve the fhevm-cli to support targeted component rebuilds and better debugging.

---

## Current State Analysis

### What Exists

```
test-suite/fhevm/
├── fhevm-cli                           # Main CLI entry point
├── scripts/
│   ├── deploy-fhevm-stack.sh          # Full deployment orchestration
│   └── debug-container.sh             # Basic container debugging
├── docker-compose/                     # Per-component compose files
│   ├── coprocessor-docker-compose.yml
│   ├── kms-connector-docker-compose.yml
│   ├── relayer-docker-compose.yml
│   └── ...
├── env/staging/                        # Environment files
│   ├── .env.coprocessor
│   ├── .env.kms-connector
│   └── ...
└── e2e/                               # Hardhat test suite
    ├── run-tests.sh
    └── test/
```

### Current Workflow Pain Points

| Issue | Impact | Root Cause |
|-------|--------|------------|
| No service-level rebuild | Rebuild all or nothing | CLI doesn't support `--only` |
| Log searching is manual | Debugging slow | Basic `logs` command |
| No health check | Status unclear | Must check each container |
| Relayer requires external repo | Setup friction | Separate repository |

---

## Target Architecture

### Deployment Modes

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        DEPLOYMENT MODE MATRIX                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  --only VALUE   WHAT GETS REBUILT        USE CASE                           │
│  ─────────────────────────────────────────────────────────────────────────  │
│  (none)         Everything                First setup, CI                    │
│  contracts      Gateway + Host SC only    Contract changes                   │
│  coprocessor    Coprocessor services      Rust changes in coprocessor/       │
│  kms-connector  KMS connector services    Rust changes in kms-connector/     │
│  relayer        Relayer only              Relayer development                │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## CLI Commands Specification

### 1. Deploy Commands

```bash
# Full deployment
./fhevm-cli deploy

# Full deployment with local builds
./fhevm-cli deploy --build --local

# Rebuild specific component only
./fhevm-cli deploy --only coprocessor --build
./fhevm-cli deploy --only kms-connector --build
./fhevm-cli deploy --only relayer --build
./fhevm-cli deploy --only contracts

# Use local relayer from filesystem
./fhevm-cli deploy --local-relayer /path/to/console
```

### 2. Test Commands

```bash
# Run specific test
./fhevm-cli test input-proof

# Run test without recompiling Hardhat (fast iteration)
./fhevm-cli test input-proof --no-compile

# Run with specific timeout
./fhevm-cli test input-proof --timeout 120

# Machine-readable output
./fhevm-cli test input-proof --format json
```

### 3. Debug Commands

```bash
# Health check all services
./fhevm-cli health

# Detailed health with RPC checks
./fhevm-cli health --verbose

# Logs from specific service with context
./fhevm-cli logs coprocessor-gw-listener --tail 100

# Follow logs with timestamps
./fhevm-cli logs relayer -f --timestamps

# Search logs across all services
./fhevm-cli logs --grep "VerifyProofRequest"

# Logs from multiple services
./fhevm-cli logs coprocessor-gw-listener kms-connector-gw-listener --tail 50
```

### 4. Service Control

```bash
# Restart single service (no rebuild)
./fhevm-cli restart coprocessor-gw-listener

# Restart with rebuild
./fhevm-cli restart coprocessor-gw-listener --build

# Stop/start
./fhevm-cli stop relayer
./fhevm-cli start relayer
```

---

## Service Dependency Graph

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        SERVICE DEPENDENCY GRAPH                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Level 0 (Infrastructure - rarely changes)                                   │
│  ├── minio                                                                   │
│  ├── database (postgres)                                                     │
│  └── core (kms-core)                                                         │
│                                                                              │
│  Level 1 (Nodes - rarely changes)                                           │
│  ├── gateway-node                                                            │
│  └── host-node                                                               │
│                                                                              │
│  Level 2 (Contracts - changes during V2 development)                        │
│  ├── gateway-sc (depends on: gateway-node)                                  │
│  └── host-sc (depends on: host-node)                                        │
│                                                                              │
│  Level 3 (Workers - changes frequently during V2)                           │
│  ├── coprocessor-* (depends on: database, gateway-sc)                       │
│  │   ├── db-migration                                                        │
│  │   ├── gw-listener                                                         │
│  │   ├── zkproof-worker                                                      │
│  │   └── tfhe-worker, sns-worker, host-listener, tx-sender                   │
│  └── kms-connector-* (depends on: database, gateway-sc)                     │
│      ├── db-migration                                                        │
│      ├── gw-listener                                                         │
│      ├── kms-worker                                                          │
│      └── tx-sender                                                           │
│                                                                              │
│  Level 4 (Relayer - changes during V2)                                      │
│  └── relayer (depends on: coprocessor, kms-connector, gateway-sc, host-sc)  │
│                                                                              │
│  Level 5 (Test Suite)                                                       │
│  └── test-suite (depends on: all above)                                     │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Local Relayer Integration

```bash
# Environment variable for local console repo
export CONSOLE_REPO_PATH="${HOME}/zama/console"

# Deploy with local relayer
./fhevm-cli deploy --local-relayer

# Build relayer from local source
./fhevm-cli deploy --only relayer --build
```

---

## Health Dashboard

```bash
# Comprehensive health check
./fhevm-cli health --verbose

# Output:
┌─────────────────────────────────────────────────────────────────┐
│ FHEVM STACK HEALTH                                              │
├─────────────────────────────────────────────────────────────────┤
│ Infrastructure:                                                 │
│   ✓ minio              │ healthy │ 9000                        │
│   ✓ database           │ healthy │ 5432                        │
│   ✓ kms-core           │ healthy │ 50051                       │
│                                                                 │
│ Nodes:                                                          │
│   ✓ gateway-node       │ healthy │ block: 12345                │
│   ✓ host-node          │ healthy │ block: 67890                │
│                                                                 │
│ Contracts:                                                      │
│   ✓ InputVerification  │ deployed │ 0x1234...                  │
│   ✓ DecryptionRegistry │ deployed │ 0x5678...                  │
│   ✓ ACL                │ deployed │ 0x9abc...                  │
│                                                                 │
│ Workers:                                                        │
│   ✓ copro-gw-listener  │ healthy │ events: 150                 │
│   ✓ copro-zkproof      │ healthy │ queue: 0                    │
│   ✓ copro-tx-sender    │ healthy │ pending: 0                  │
│   ✓ kms-gw-listener    │ healthy │ events: 50                  │
│   ✓ kms-worker         │ healthy │ queue: 0                    │
│   ⚠ kms-tx-sender      │ warning │ pending: 5 (high)           │
│                                                                 │
│ Relayer:                                                        │
│   ✓ relayer            │ healthy │ :3000                       │
│                                                                 │
│ Test Suite:                                                     │
│   ✓ test-suite-e2e     │ ready   │                             │
│                                                                 │
│ Overall: HEALTHY (1 warning)                                    │
└─────────────────────────────────────────────────────────────────┘
```

---

## Test Execution

### Test Output

```bash
# Machine-readable test output (optional)
./fhevm-cli test input-proof --format json

# Output on success:
{
  "test": "input-proof",
  "status": "passed",
  "duration_ms": 5230
}

# Output on failure:
{
  "test": "input-proof",
  "status": "failed",
  "duration_ms": 60000,
  "error": "Timeout waiting for VerifyProofResponse"
}
```

---

## Debugging Workflows

### Workflow 1: Test Failure Investigation

```bash
# 1. Run test
./fhevm-cli test input-proof

# 2. Check service health
./fhevm-cli health

# 3. Check logs for errors
./fhevm-cli logs coprocessor-gw-listener --tail 100 --grep "error|panic"
```

### Workflow 2: Service Crash Investigation

```bash
# 1. Check health
./fhevm-cli health

# 2. Get crash logs
./fhevm-cli logs coprocessor-zkproof-worker --tail 200

# 3. Restart with rebuild if code changed
./fhevm-cli restart coprocessor-zkproof-worker --build
```

### Workflow 3: Contract Change Iteration

```bash
# 1. Make contract changes in gateway-contracts/

# 2. Rebuild only contracts
./fhevm-cli deploy --only contracts

# 3. Rebuild Rust services that use bindings
./fhevm-cli deploy --only coprocessor kms-connector --build

# 4. Run test to verify
./fhevm-cli test input-proof --no-compile
```

### Workflow 4: V2 Development Cycle

```bash
# 1. Start with clean state (first time only)
./fhevm-cli deploy --build --local

# 2. Make changes to coprocessor code

# 3. Rebuild only what changed
./fhevm-cli deploy --only coprocessor --build

# 4. Run quick smoke test
./fhevm-cli test input-proof --no-compile

# 5. If test fails, check logs
./fhevm-cli logs coprocessor-gw-listener --tail 50

# 6. Fix and repeat from step 2
```

---

## Implementation Tasks

| Task | Priority |
|------|----------|
| Add `--only <component>` flag to deploy | High |
| Add `./fhevm-cli health` command | High |
| Add `--tail`, `-f`, `--grep` to logs command | Medium |
| Add `--format json` to test output | Medium |
| Add `./fhevm-cli restart <service> --build` | Medium |

---

## Configuration File

```yaml
# .fhevm-cli.yaml (in test-suite/fhevm/)
version: 1

# Local development paths
paths:
  console_repo: "${HOME}/zama/console"

# Test configuration
test:
  timeout_seconds: 120

# Component groupings for --only flag
components:
  coprocessor:
    - coprocessor-db-migration
    - coprocessor-gw-listener
    - coprocessor-zkproof-worker
    - coprocessor-tfhe-worker
    - coprocessor-tx-sender
  kms-connector:
    - kms-connector-db-migration
    - kms-connector-gw-listener
    - kms-connector-kms-worker
    - kms-connector-tx-sender
  contracts:
    - gateway-sc
    - host-sc
```

---

## Quick Reference

```bash
# === DEPLOYMENT ===
# Fresh deploy (first time or clean slate)
./fhevm-cli deploy --build --local

# Rebuild specific component only
./fhevm-cli deploy --only coprocessor --build

# === TESTING ===
# Run test without recompile (fast)
./fhevm-cli test input-proof --no-compile

# === DEBUGGING ===
# Check health
./fhevm-cli health --verbose

# Search logs
./fhevm-cli logs coprocessor-gw-listener --tail 100 --grep "error|panic"

# === COMMON PATTERNS ===
# Test failed? Check in order:
# 1. ./fhevm-cli health
# 2. ./fhevm-cli logs <service> --tail 100

# Service crashed? Check:
# 1. ./fhevm-cli health
# 2. ./fhevm-cli logs <service> --tail 100
# 3. ./fhevm-cli restart <service> --build
```

---

## Summary

This CLI plan provides:

1. **Component Targeting**: `--only coprocessor` for focused rebuilds
2. **Health Dashboard**: Single command to see all service status
3. **Improved Logs**: `--tail`, `-f`, `--grep` for better debugging
4. **Service Control**: `restart --build` for quick iteration

The goal is to reduce the feedback loop by allowing targeted rebuilds instead of full stack rebuilds.
