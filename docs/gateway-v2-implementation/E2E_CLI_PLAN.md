# E2E Testing CLI Plan: AI-Optimized Development Workflow

**Objective**: Design an efficient E2E testing CLI that minimizes rebuild time, maximizes cache utilization, and provides AI-agent-friendly debugging capabilities.

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
| Full rebuild on every change | ~30+ min | No incremental build support |
| Cache invalidation | Lost time | GHA cache not local-friendly |
| No service-level rebuild | Rebuild all or nothing | Compose limitations |
| Log searching is manual | Debugging slow | No structured log output |
| No health dashboards | Status unclear | Must check each container |
| Relayer requires external repo | Setup friction | Separate repository |

---

## Target Architecture

### Deployment Modes

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        DEPLOYMENT MODE MATRIX                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  MODE           WHAT GETS REBUILT        USE CASE                           │
│  ─────────────────────────────────────────────────────────────────────────  │
│  fresh          Everything                First setup, CI                    │
│  incremental    Changed services only     Normal development                 │
│  contracts      Gateway + Host SC only    Contract changes                   │
│  coprocessor    Coprocessor services      Rust changes in coprocessor/       │
│  kms            KMS connector services    Rust changes in kms-connector/     │
│  relayer        Relayer only              Relayer development                │
│  none           Nothing (pull only)       Testing existing images            │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## CLI Commands Specification

### 1. Deploy Commands

```bash
# Full fresh deployment (cleans everything)
./fhevm-cli deploy

# Incremental deployment (preserves state, rebuilds changed only)
./fhevm-cli deploy --incremental

# Rebuild specific component
./fhevm-cli deploy --only coprocessor
./fhevm-cli deploy --only kms-connector  
./fhevm-cli deploy --only relayer
./fhevm-cli deploy --only contracts

# Skip specific services
./fhevm-cli deploy --skip relayer --skip test-suite

# Use local relayer from filesystem
./fhevm-cli deploy --local-relayer /path/to/console

# Force rebuild everything with local cache
./fhevm-cli deploy --build --local
```

### 2. Test Commands

```bash
# Run specific test with minimal output
./fhevm-cli test input-proof

# Run test with full trace (for debugging)
./fhevm-cli test input-proof --trace

# Run test and capture all relevant logs
./fhevm-cli test input-proof --capture-logs

# Run test without recompiling Hardhat (fast iteration)
./fhevm-cli test input-proof --no-compile

# Run with specific timeout
./fhevm-cli test input-proof --timeout 120

# Run multiple test types
./fhevm-cli test input-proof,user-decryption,public-decryption
```

### 3. Debug Commands

```bash
# Health check all services
./fhevm-cli health

# Detailed health with RPC checks
./fhevm-cli health --verbose

# Watch logs from multiple services (filtered)
./fhevm-cli logs --filter "error|warn|panic"

# Logs from specific service with context
./fhevm-cli logs coprocessor-gw-listener --tail 100

# Follow logs with timestamps
./fhevm-cli logs relayer -f --timestamps

# Search logs across all services
./fhevm-cli logs --grep "VerifyProofRequest"

# Export logs for a time window
./fhevm-cli logs --since "5 minutes ago" --export /tmp/debug-logs.txt

# Database inspection
./fhevm-cli db query coprocessor "SELECT * FROM zkpok_requests LIMIT 10"
./fhevm-cli db tables coprocessor
./fhevm-cli db pending  # Show pending work items across all DBs

# Event tracing
./fhevm-cli trace request <request_id>  # Follow request through all services
./fhevm-cli trace events --since "1 minute ago"  # Recent blockchain events
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

# Scale workers
./fhevm-cli scale coprocessor-tfhe-worker 3

# Hot reload config (if supported)
./fhevm-cli reload relayer
```

### 5. Cache Management

```bash
# Show cache status
./fhevm-cli cache status

# Clear specific cache
./fhevm-cli cache clear coprocessor
./fhevm-cli cache clear all

# Warm cache (pre-build without starting)
./fhevm-cli cache warm coprocessor

# Export cache for CI
./fhevm-cli cache export /path/to/cache.tar.gz
```

---

## Incremental Build System

### Change Detection

```bash
# Files that trigger rebuilds for each component
COPROCESSOR_TRIGGERS=(
  "coprocessor/fhevm-engine/**/*.rs"
  "coprocessor/fhevm-engine/Cargo.toml"
  "coprocessor/fhevm-engine/Cargo.lock"
  "gateway-contracts/out/**"  # Bindings dependency
)

KMS_CONNECTOR_TRIGGERS=(
  "kms-connector/**/*.rs"
  "kms-connector/Cargo.toml"
  "kms-connector/Cargo.lock"
  "gateway-contracts/out/**"  # Bindings dependency
)

RELAYER_TRIGGERS=(
  "${CONSOLE_REPO_PATH}/apps/relayer/**/*.rs"
  "${CONSOLE_REPO_PATH}/apps/relayer/Cargo.toml"
)

GATEWAY_CONTRACT_TRIGGERS=(
  "gateway-contracts/contracts/**/*.sol"
  "gateway-contracts/hardhat.config.ts"
)

HOST_CONTRACT_TRIGGERS=(
  "host-contracts/contracts/**/*.sol"
  "host-contracts/hardhat.config.ts"
)
```

### Build Fingerprinting

```bash
# Generate fingerprint for a component
compute_fingerprint() {
  local component=$1
  local triggers=("${!2}")
  
  # Hash all trigger files
  find "${triggers[@]}" -type f 2>/dev/null | \
    sort | \
    xargs sha256sum 2>/dev/null | \
    sha256sum | \
    cut -d' ' -f1
}

# Store fingerprints
FINGERPRINT_FILE=".fhevm-build-fingerprints"

# Check if rebuild needed
needs_rebuild() {
  local component=$1
  local current=$(compute_fingerprint "$component" "${component}_TRIGGERS[@]")
  local stored=$(grep "^$component:" "$FINGERPRINT_FILE" | cut -d: -f2)
  
  [[ "$current" != "$stored" ]]
}
```

### Service Dependency Graph

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

### Incremental Rebuild Logic

```bash
incremental_deploy() {
  # Check what changed
  local rebuild_contracts=false
  local rebuild_coprocessor=false
  local rebuild_kms=false
  local rebuild_relayer=false
  
  needs_rebuild "gateway-contracts" && rebuild_contracts=true
  needs_rebuild "host-contracts" && rebuild_contracts=true
  needs_rebuild "coprocessor" && rebuild_coprocessor=true
  needs_rebuild "kms-connector" && rebuild_kms=true
  needs_rebuild "relayer" && rebuild_relayer=true
  
  # Contracts change → must rebuild coprocessor/kms (for bindings)
  if $rebuild_contracts; then
    rebuild_coprocessor=true
    rebuild_kms=true
  fi
  
  # Execute rebuilds in dependency order
  if $rebuild_contracts; then
    deploy_contracts
  fi
  
  # Coprocessor and KMS can rebuild in parallel
  if $rebuild_coprocessor || $rebuild_kms; then
    (
      $rebuild_coprocessor && rebuild_coprocessor_services &
      $rebuild_kms && rebuild_kms_services &
      wait
    )
  fi
  
  if $rebuild_relayer; then
    rebuild_relayer_service
  fi
  
  # Update fingerprints
  update_fingerprints
}
```

---

## Caching Strategy

### Local BuildKit Cache

```yaml
# docker-compose cache configuration
services:
  coprocessor-gw-listener:
    build:
      cache_from:
        - type=local,src=.buildx-cache/coprocessor
      cache_to:
        - type=local,dest=.buildx-cache/coprocessor,mode=max
```

### Cache Layers (Priority Order)

```
1. Local BuildKit cache (.buildx-cache/)
   - Fastest, used for iterative development
   - Persists between deploys
   - Component-specific directories

2. Docker layer cache
   - Built-in Docker caching
   - Automatically used when Dockerfile unchanged

3. Cargo registry cache (mounted volume)
   - Shared across all Rust services
   - Persists crates.io downloads

4. Rust target cache (mounted volume)
   - Shared incremental compilation cache
   - Significantly speeds up Rust rebuilds
```

### Cache Volume Mounts

```yaml
# Proposed volume structure for maximum cache reuse
volumes:
  # Shared cargo registry (downloaded crates)
  cargo-registry:
    driver: local
    driver_opts:
      type: none
      o: bind
      device: ${HOME}/.cargo/registry

  # Shared cargo git checkouts
  cargo-git:
    driver: local
    driver_opts:
      type: none
      o: bind
      device: ${HOME}/.cargo/git

  # Coprocessor target directory (incremental builds)
  coprocessor-target:
    driver: local
    driver_opts:
      type: none
      o: bind
      device: ./.cache/coprocessor-target

  # KMS connector target directory
  kms-target:
    driver: local
    driver_opts:
      type: none
      o: bind
      device: ./.cache/kms-target
```

### Dockerfile Optimization for Caching

```dockerfile
# Current: Everything in one layer
COPY . .
RUN cargo build --release

# Optimized: Separate dependency and source layers
# Layer 1: Dependencies only (cached until Cargo.toml changes)
COPY Cargo.toml Cargo.lock ./
COPY crates/*/Cargo.toml ./crates/
RUN cargo fetch
RUN cargo build --release --lib || true  # Build deps only

# Layer 2: Source code (rebuilt on every change)
COPY . .
RUN cargo build --release
```

---

## Local Relayer Integration

### Configuration

```bash
# Environment variable for local console repo
export CONSOLE_REPO_PATH="${HOME}/zama/console"

# fhevm-cli automatically detects and uses local relayer
./fhevm-cli deploy --local-relayer
# Equivalent to:
./fhevm-cli deploy --local-relayer "${CONSOLE_REPO_PATH}"
```

### Relayer Build Options

```bash
# Build relayer from local source
./fhevm-cli deploy --only relayer --build

# Use pre-built relayer image (faster if no changes)
./fhevm-cli deploy --only relayer

# Hot-reload relayer config without rebuild
./fhevm-cli reload relayer
```

### Relayer Database Migration

```bash
# Auto-run migrations on deploy
./fhevm-cli deploy  # Includes relayer migrations

# Manual migration
./fhevm-cli db migrate relayer

# Check migration status
./fhevm-cli db migrate relayer --status
```

---

## AI-Agent Optimized Debugging

### Structured Log Output

```bash
# Machine-readable log format
./fhevm-cli logs --format json

# Output:
{"timestamp":"2026-01-09T10:30:00Z","service":"coprocessor-gw-listener","level":"INFO","message":"Processing VerifyProofRequest","request_id":"0x123...","chain_id":11155111}

# Grep-friendly output
./fhevm-cli logs --format oneline

# Output:
[2026-01-09T10:30:00Z] [coprocessor-gw-listener] [INFO] Processing VerifyProofRequest request_id=0x123... chain_id=11155111
```

### Error Classification

```bash
# Categorize errors for quick triage
./fhevm-cli errors

# Output:
┌─────────────────────────────────────────────────────────────────┐
│ ERROR SUMMARY (last 5 minutes)                                  │
├─────────────────────────────────────────────────────────────────┤
│ Category         │ Count │ Services                            │
├──────────────────┼───────┼─────────────────────────────────────┤
│ RPC Connection   │ 3     │ coprocessor-gw-listener             │
│ Contract Revert  │ 1     │ relayer                             │
│ Database         │ 0     │ -                                   │
│ Timeout          │ 2     │ kms-connector-kms-worker            │
└─────────────────────────────────────────────────────────────────┘

# Details for a category
./fhevm-cli errors --category "Contract Revert" --verbose
```

### Request Tracing

```bash
# Trace a specific request through the system
./fhevm-cli trace request 0x123456...

# Output:
┌─────────────────────────────────────────────────────────────────┐
│ REQUEST TRACE: 0x123456...                                      │
├─────────────────────────────────────────────────────────────────┤
│ Timeline:                                                       │
│ ├─ 10:30:00.000 │ relayer          │ Received input proof      │
│ ├─ 10:30:00.050 │ relayer          │ Submitted to Gateway      │
│ ├─ 10:30:01.200 │ gateway-chain    │ InputVerificationReg...   │
│ ├─ 10:30:01.500 │ copro-gw-listen  │ Event received            │
│ ├─ 10:30:01.510 │ copro-gw-listen  │ Stored in DB              │
│ ├─ 10:30:02.000 │ zkproof-worker   │ Processing started        │
│ ├─ 10:30:05.000 │ zkproof-worker   │ Verification complete     │
│ ├─ 10:30:05.100 │ copro-tx-sender  │ Response TX submitted     │
│ └─ 10:30:06.500 │ relayer          │ Response received         │
│                                                                 │
│ Status: COMPLETED                                               │
│ Duration: 6.5s                                                  │
│ Handles: [0xabc..., 0xdef...]                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Health Dashboard

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

### Database Inspection

```bash
# Show pending work across all queues
./fhevm-cli db pending

# Output:
┌─────────────────────────────────────────────────────────────────┐
│ PENDING WORK ITEMS                                              │
├─────────────────────────────────────────────────────────────────┤
│ Database      │ Table               │ Pending │ Oldest          │
├───────────────┼─────────────────────┼─────────┼─────────────────┤
│ coprocessor   │ zkpok_requests      │ 0       │ -               │
│ coprocessor   │ verify_proof_resps  │ 2       │ 30s ago         │
│ kms           │ decrypt_requests    │ 0       │ -               │
│ kms           │ decrypt_responses   │ 1       │ 5s ago          │
│ relayer       │ pending_requests    │ 0       │ -               │
└─────────────────────────────────────────────────────────────────┘

# Quick query
./fhevm-cli db query coprocessor "SELECT request_id, status FROM zkpok_requests ORDER BY created_at DESC LIMIT 5"
```

---

## Test Execution Optimization

### Test Profiles

```bash
# Quick smoke test (fastest)
./fhevm-cli test --profile smoke
# Runs: input-proof (uint64 only)

# Standard test (default)
./fhevm-cli test --profile standard
# Runs: input-proof, user-decryption, public-decryption

# Full test (comprehensive)
./fhevm-cli test --profile full
# Runs: all test types including operators

# Custom test combination
./fhevm-cli test input-proof user-decryption --parallel
```

### Test with Auto-Retry

```bash
# Retry flaky tests
./fhevm-cli test input-proof --retry 3 --retry-delay 10

# Retry with increased timeout on failure
./fhevm-cli test input-proof --retry 2 --escalate-timeout
```

### Test Output for AI Agents

```bash
# Machine-readable test output
./fhevm-cli test input-proof --format json

# Output on success:
{
  "test": "input-proof",
  "status": "passed",
  "duration_ms": 5230,
  "assertions": 3,
  "logs_captured": "/tmp/fhevm-test-logs/input-proof-20260109-103000.log"
}

# Output on failure:
{
  "test": "input-proof",
  "status": "failed",
  "duration_ms": 60000,
  "error": "Timeout waiting for VerifyProofResponse",
  "error_category": "timeout",
  "relevant_logs": [
    {"service": "coprocessor-gw-listener", "message": "Event received: 0x123..."},
    {"service": "zkproof-worker", "message": "Processing request 0x123..."},
    {"service": "zkproof-worker", "message": "ERROR: Invalid proof format"}
  ],
  "suggested_debug": "./fhevm-cli trace request 0x123..."
}
```

---

## Debugging Workflows

### Workflow 1: Test Failure Investigation

```bash
# 1. Run test and capture logs
./fhevm-cli test input-proof --capture-logs --format json > result.json

# 2. If failed, check error category
cat result.json | jq '.error_category'
# Output: "timeout"

# 3. Check service health
./fhevm-cli health

# 4. Check pending work (stuck items?)
./fhevm-cli db pending

# 5. Trace the specific request
./fhevm-cli trace request $(cat result.json | jq -r '.request_id')

# 6. Check logs for that request
./fhevm-cli logs --grep "$(cat result.json | jq -r '.request_id')"
```

### Workflow 2: Service Crash Investigation

```bash
# 1. Quick status check
./fhevm-cli status

# 2. Check which service crashed
./fhevm-cli errors --since "5 minutes ago"

# 3. Get crash logs
./fhevm-cli logs coprocessor-zkproof-worker --tail 200

# 4. Check if it's a pattern
./fhevm-cli logs coprocessor-zkproof-worker --grep "panic|error" --since "1 hour ago"

# 5. Restart with rebuild if code changed
./fhevm-cli restart coprocessor-zkproof-worker --build
```

### Workflow 3: Contract Change Iteration

```bash
# 1. Make contract changes in gateway-contracts/

# 2. Rebuild only contracts and dependent services
./fhevm-cli deploy --only contracts --incremental

# 3. Regenerate Rust bindings (automatic if configured)
# Or manual: cd gateway-contracts && python3 scripts/bindings_update.py update

# 4. Rebuild Rust services that use bindings
./fhevm-cli deploy --only coprocessor kms-connector --build

# 5. Run test to verify
./fhevm-cli test input-proof --no-compile
```

### Workflow 4: V2 Development Cycle

```bash
# Typical V2 development loop:

# 1. Start with clean state (first time only)
./fhevm-cli deploy --build --local

# 2. Make changes to coprocessor code

# 3. Incremental rebuild (fast)
./fhevm-cli deploy --only coprocessor --build

# 4. Run quick smoke test
./fhevm-cli test input-proof --no-compile

# 5. If test fails, investigate
./fhevm-cli errors
./fhevm-cli logs coprocessor-gw-listener --tail 50

# 6. Fix and repeat from step 2
```

---

## Implementation Phases

### Phase 1: Core CLI Improvements (Week 1)

| Task | Priority | Effort |
|------|----------|--------|
| Add `--incremental` flag | High | 2 days |
| Add `--only <component>` flag | High | 1 day |
| Add `--format json` to test output | High | 1 day |
| Add `./fhevm-cli health --verbose` | Medium | 1 day |
| Add `./fhevm-cli errors` command | Medium | 1 day |

### Phase 2: Caching Optimization (Week 2)

| Task | Priority | Effort |
|------|----------|--------|
| Implement build fingerprinting | High | 2 days |
| Add cargo cache volume mounts | High | 1 day |
| Optimize Dockerfile layer caching | Medium | 2 days |
| Add `./fhevm-cli cache` subcommands | Medium | 1 day |

### Phase 3: Debug Tools (Week 3)

| Task | Priority | Effort |
|------|----------|--------|
| Add `./fhevm-cli trace request` | High | 2 days |
| Add `./fhevm-cli db pending` | High | 1 day |
| Add `./fhevm-cli logs --grep` | Medium | 1 day |
| Add structured log output | Medium | 2 days |

### Phase 4: Advanced Features (Week 4)

| Task | Priority | Effort |
|------|----------|--------|
| Add test profiles | Medium | 1 day |
| Add retry logic | Medium | 1 day |
| Add parallel test execution | Low | 2 days |
| Add Jaeger/tracing integration | Low | 2 days |

---

## Configuration File

```yaml
# .fhevm-cli.yaml (in test-suite/fhevm/)
version: 1

# Local development paths
paths:
  console_repo: "${HOME}/zama/console"
  cache_dir: ".buildx-cache"
  logs_dir: "/tmp/fhevm-logs"

# Build configuration
build:
  cargo_profile: local  # release | local | dev
  parallel_jobs: 4
  cache_mode: local     # local | gha | none

# Test configuration
test:
  default_profile: standard
  timeout_seconds: 120
  retry_count: 2
  capture_logs: true

# Service configuration
services:
  v2_modified:
    coprocessor:
      - coprocessor-db-migration
      - coprocessor-gw-listener
      - coprocessor-zkproof-worker
    kms_connector:
      - kms-connector-db-migration
      - kms-connector-gw-listener
      - kms-connector-kms-worker
      - kms-connector-tx-sender

# Debug configuration
debug:
  log_format: oneline   # oneline | json | raw
  health_check_interval: 10
  trace_retention_hours: 24
```

---

## AI Agent Usage Guide

### Quick Reference for AI Agents

```bash
# === DEPLOYMENT ===
# Fresh deploy (first time or clean slate)
./fhevm-cli deploy --build --local

# Incremental deploy (after code changes)
./fhevm-cli deploy --incremental

# Rebuild specific component
./fhevm-cli deploy --only coprocessor --build

# === TESTING ===
# Run test with machine output
./fhevm-cli test input-proof --format json

# Run test without recompile (fast)
./fhevm-cli test input-proof --no-compile

# === DEBUGGING ===
# Check health
./fhevm-cli health --verbose

# Find errors
./fhevm-cli errors

# Trace a request
./fhevm-cli trace request <request_id>

# Search logs
./fhevm-cli logs --grep "error|panic" --since "5 minutes ago"

# Check pending work
./fhevm-cli db pending

# === COMMON PATTERNS ===
# Test failed? Check in order:
# 1. ./fhevm-cli health
# 2. ./fhevm-cli errors
# 3. ./fhevm-cli db pending
# 4. ./fhevm-cli trace request <id>

# Service crashed? Check:
# 1. ./fhevm-cli status
# 2. ./fhevm-cli logs <service> --tail 100
# 3. ./fhevm-cli restart <service> --build
```

### Error Categories and Actions

| Error Category | Likely Cause | Debug Command | Fix Action |
|----------------|--------------|---------------|------------|
| `timeout` | Slow processing or stuck | `./fhevm-cli db pending` | Check queue, restart worker |
| `rpc_connection` | Node down or network | `./fhevm-cli health` | Restart node |
| `contract_revert` | Bad TX or wrong params | `./fhevm-cli trace request <id>` | Check contract logs |
| `database` | Migration or connection | `./fhevm-cli db shell` | Check tables, run migration |
| `panic` | Rust crash | `./fhevm-cli logs <service>` | Check logs, rebuild |

---

## Summary

This CLI plan provides:

1. **Incremental Builds**: Only rebuild what changed (~5min vs ~30min)
2. **Smart Caching**: Local BuildKit + Cargo caches + volume mounts
3. **Component Targeting**: `--only coprocessor` for focused rebuilds
4. **Machine-Readable Output**: `--format json` for AI agent consumption
5. **Request Tracing**: Follow a request through the entire system
6. **Error Classification**: Quick triage with `./fhevm-cli errors`
7. **Health Dashboard**: Single command to see all service status
8. **Debug Workflows**: Documented patterns for common issues

The goal is to reduce the feedback loop from ~30 minutes to ~5 minutes for typical development iterations.
