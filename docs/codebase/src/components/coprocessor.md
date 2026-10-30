# Coprocessor

**Location**: `/coprocessor/`
**Status**: Active Development (1,718 commits in 6 months)
**Purpose**: Rust-based asynchronous FHE computation engine

## Overview

The Coprocessor is the off-chain component that performs actual FHE computations. It listens to events from Host and Gateway contracts, executes the expensive cryptographic operations, and submits results back to the chain.

## Key Crates

Located in `/coprocessor/fhevm-engine/`:

| Crate | Purpose |
|-------|---------|
| `tfhe-worker` | Core FHE computation engine using TFHE-rs |
| `scheduler` | Dataflow graph and work distribution |
| `zkproof-worker` | Zero-knowledge proof verification |
| `sns-worker` | Switch-N-Squash noise reduction and S3 upload |
| `host-listener` | Monitors host chain events |
| `gw-listener` | Monitors gateway chain events |
| `transaction-sender` | Broadcasts results back to chain |
| `fhevm-engine-common` | Shared utilities, GPU memory, telemetry |

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                       COPROCESSOR                                │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ gw-listener │  │host-listener│  │    transaction-sender   │  │
│  └──────┬──────┘  └──────┬──────┘  └────────────▲────────────┘  │
│         │                │                      │                │
│         ▼                ▼                      │                │
│  ┌──────────────────────────────────────────────┴──────────────┐│
│  │                    PostgreSQL Database                       ││
│  │  (computations, ciphertexts, verify_proofs, tenants)        ││
│  └──────────────────────────────────────────────────────────────┘│
│         │                │                      ▲                │
│         ▼                ▼                      │                │
│  ┌─────────────┐  ┌─────────────┐  ┌───────────┴───────────┐    │
│  │ tfhe-worker │  │ sns-worker  │  │    zkproof-worker     │    │
│  │   (GPU)     │  │   (S3)      │  │                       │    │
│  └─────────────┘  └─────────────┘  └───────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
```

**Event-driven design:**
- Listeners pick up on-chain events and create jobs
- Jobs stored in PostgreSQL database with NOTIFY/LISTEN channels
- Scheduler distributes work to specialized workers
- Workers process jobs concurrently with GPU acceleration
- Results submitted back via transaction-sender

---

## Worker Architecture

### TFHE Worker (Core FHE Computation)

**Location**: `fhevm-engine/tfhe-worker/src/tfhe_worker.rs`

The TFHE Worker is the primary computation engine that executes FHE operations using TFHE-rs. It implements a hybrid polling strategy combining PostgreSQL notifications with interval-based fallback.

**Processing Pipeline:**

```
┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   LISTEN     │───▶│  Query Work  │───▶│ Build Graph  │───▶│   Execute    │
│work_available│    │  (SKIP LOCK) │    │  (per tenant)│    │  (parallel)  │
└──────────────┘    └──────────────┘    └──────────────┘    └──────────────┘
                                                                    │
┌──────────────┐    ┌──────────────┐    ┌──────────────┐            │
│   NOTIFY     │◀───│   Commit     │◀───│Store Results │◀───────────┘
│  computed    │    │ Transaction  │    │ (ciphertexts)│
└──────────────┘    └──────────────┘    └──────────────┘
```

**Key Implementation Details:**

1. **Event Loop** (`tfhe_worker.rs:58-172`):
   - Uses `PgListener` on `work_available` channel
   - Falls back to polling every `worker_polling_interval_ms` if no notifications
   - Immediately re-polls if work was found (no wait between batches)

2. **Tenant Key Caching**:
   - LRU cache with configurable size (default varies by deployment)
   - Keys loaded lazily on first computation for tenant
   - Separate Server Key (SKS) and Public Key (PKS) per tenant

3. **Parallel Execution**:
   - Uses `tokio::task::JoinSet` for blocking FHE operations
   - Configurable thread counts: 32 FHE threads + 4 Tokio async threads
   - GPU support via `#[cfg(feature = "gpu")]` with CUDA server keys

4. **Metrics** (Prometheus):
   - `coprocessor_worker_errors` - Error counter
   - `coprocessor_work_items_polls` - Database poll counter
   - `coprocessor_work_items_notifications` - Notification counter
   - `coprocessor_work_items_processed` - Successful computation counter

### SNS Worker (Switch-N-Squash)

**Location**: `fhevm-engine/sns-worker/src/executor.rs`

The SNS Worker performs Programmable Bootstrap (PBS) operations to reduce ciphertext noise, then uploads results to S3.

**Dual Pipeline Architecture:**

```
┌────────────────────────────────────────────────────────────────┐
│                      SNS Worker                                 │
│  ┌─────────────────────┐      ┌─────────────────────────────┐  │
│  │  Computation Pipeline│      │     Upload Pipeline          │  │
│  │  ─────────────────── │      │     ───────────────────────  │  │
│  │  • Fetch ciphertexts │──────▶  • Queue UploadJob          │  │
│  │  • squash_noise()    │      │  • Upload to S3 (ct64/ct128)│  │
│  │  • Create 64/128 bit │      │  • Update ciphertext_digest │  │
│  └─────────────────────┘      └─────────────────────────────┘  │
└────────────────────────────────────────────────────────────────┘
```

**Key Features:**
- **Schedule Policy**: Sequential or `RayonParallel` for PBS operations
- **Ciphertext Formats**: Creates both 64-bit compressed and 128-bit uncompressed
- **S3 Integration**: Configurable concurrent uploads, retry policy, bucket names
- **Health Checks**: S3 bucket readiness verification with 5s timeout

**Metrics:**
- `coprocessor_sns_op_latency_seconds` - PBS operation timing
- `sns_worker_task_execute_success_counter` / `failure_counter`

### ZKProof Worker (Proof Verification)

**Location**: `fhevm-engine/zkproof-worker/src/verifier.rs`

The ZKProof Worker verifies zero-knowledge proofs for encrypted operations using multi-threaded execution.

**Configuration:**
- `worker_thread_count`: Default 8 verification threads
- `MAX_CACHED_TENANT_KEYS`: 100 entries in LRU cache
- `pg_polling_interval`: Default 60 seconds fallback poll

**Processing:**
1. Listens on `event_ciphertext_computed` channel
2. Fetches proof verification requests from `verify_proofs` table
3. Loads tenant server keys (cached)
4. Verifies proof against ciphertext and auxiliary data
5. Updates `verify_proofs` with result (completed/error)

**Error Types** (`scheduler/src/dfg/types.rs:59-90`):
```rust
pub enum SchedulerError {
    CyclicDependence,      // Dependency loop detected
    DataflowGraphError,    // Graph construction failed
    MissingInputs,         // Inputs not yet available
    ReRandomisationError,  // TFHE re-randomization failed
    SchedulerError,        // Generic scheduler error
}
```

---

## Scheduler and Job Orchestration

### Job Lifecycle

```
┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐
│ Event   │───▶│  Job    │───▶│Schedule │───▶│ Execute │───▶│ Result  │
│Reception│    │Creation │    │(priority│    │ (graph) │    │Submission│
│(listener)│    │(DB insert)│   │ queue) │    │         │    │(txn-sender)│
└─────────┘    └─────────┘    └─────────┘    └─────────┘    └─────────┘
```

### Dataflow Graph

**Location**: `fhevm-engine/scheduler/src/dfg/`

The scheduler builds a Directed Acyclic Graph (DAG) of FHE operations using the `daggy` crate:

```rust
pub struct DFComponentGraph {
    graph: Dag<OpNode, ComponentEdge>,  // Dependency graph
    needed_map: HashMap<Handle, ...>,   // Required ciphertexts
    results: Vec<DFGTxResult>,          // Computed results
}

pub enum DFGTaskInput {
    Value(SupportedFheCiphertexts),     // Scalar or ciphertext literal
    Compressed((i16, Vec<u8>)),         // Compressed from DB
    Dependence(Handle),                 // Output from prior operation
}
```

### Scheduling Strategies

**Location**: `scheduler/src/dfg/scheduler.rs:30-112`

Two strategies controlled by `FHEVM_DF_SCHEDULE` environment variable:

| Strategy | Description | Use Case |
|----------|-------------|----------|
| `MAX_PARALLELISM` | Maximize concurrent task execution | Default, best for GPU |
| `MAX_LOCALITY` | Optimize for cache locality on same device | Memory-bound workloads |

**GPU Device Selection** (RoundRobin):
```rust
// scheduler.rs:139-150
DeviceSelection::RoundRobin => {
    static LAST: AtomicUsize = AtomicUsize::new(0);
    let i = LAST.load(Ordering::Acquire);
    LAST.store((i + 1) % self.csks.len(), Ordering::Release);
    Ok((self.csks[i].clone(), self.cpk.clone()))
}
```

### Retry Logic and Error Handling

**Exponential Backoff:**
- Failed computations marked with `uncomputable_counter`
- Schedule delay: `uncomputable_counter * 1 second` (doubles each retry)
- Maximum delay: 32,000 seconds (~9 hours)
- Re-attempted in subsequent worker cycles

**Work Query with Non-Blocking Locks:**
```sql
SELECT transaction_id FROM computations
WHERE is_completed = FALSE AND is_error = FALSE AND is_allowed = TRUE
ORDER BY schedule_order
FOR UPDATE SKIP LOCKED
LIMIT $batch_size
```

---

## GPU Utilization

### Memory Management

**Location**: `fhevm-engine-common/src/gpu_memory.rs`

GPU memory is managed with atomic per-GPU reservation pools:

```rust
// Two-stage reservation protocol
pub fn reserve_memory_on_gpu(amount: u64, idx: usize) {
    loop {
        // Stage 1: Atomically add to reservation pool
        let old_pool_size = gpu_mem_reservation[idx]
            .fetch_add(amount, Ordering::SeqCst);

        // Stage 2: Validate against GPU capacity
        if check_valid_cuda_malloc(old_pool_size + amount, GpuIndex::new(idx)) {
            break;  // Allocation successful
        } else {
            // Remove reservation, retry after 2ms backoff
            gpu_mem_reservation[idx].fetch_sub(amount, Ordering::SeqCst);
            std::thread::sleep(Duration::from_millis(2));
        }
    }
}
```

**Type-Aware Sizing:**
- `get_size_on_gpu()` implemented for all FHE types
- Supports: FheBool, FheUint4-256, FheBytes64-256
- Operation-specific sizing (e.g., `get_add_size_on_gpu()`)

### Device Selection and Load Balancing

- **RoundRobin**: Atomic counter distributes tasks across GPUs
- **Fallback**: Invalid device indices gracefully default to GPU 0
- **Per-Tenant Isolation**: Server keys loaded per tenant

### Performance Tuning

**Thread Allocation** (configurable via CLI):
| Parameter | Default | Purpose |
|-----------|---------|---------|
| `coprocessor_fhe_threads` | 32 | Blocking FHE operations |
| `tokio_threads` | 4 | Async I/O coordination |
| `work_items_batch_size` | 100 | Items per worker cycle |
| `dependence_chains_per_batch` | 20 | Chains fetched per query |

### Metrics

| Metric | Description |
|--------|-------------|
| `coprocessor_fhe_batch_latency_seconds` | End-to-end FHE batch timing |
| `coprocessor_rerand_batch_latency_seconds` | Re-randomization batch timing |
| `coprocessor_sns_op_latency_seconds` | SNS PBS operation latency |

---

## Database Schema

### Core Tables

**Location**: `fhevm-engine/db-migration/migrations/20240722111257_coprocessor.sql`

```
┌─────────────────────┐       ┌─────────────────────┐
│    computations     │       │     ciphertexts     │
├─────────────────────┤       ├─────────────────────┤
│ tenant_id (PK)      │       │ tenant_id (PK)      │
│ output_handle (PK)  │──────▶│ handle (PK)         │
│ transaction_id (PK) │       │ ciphertext_version  │
│ dependencies[]      │       │ ciphertext (BYTEA)  │
│ fhe_operation       │       │ ciphertext_type     │
│ is_completed        │       │ input_blob_hash     │
│ is_error            │       └─────────────────────┘
│ schedule_order      │
│ uncomputable_counter│       ┌─────────────────────┐
└─────────────────────┘       │      tenants        │
                              ├─────────────────────┤
┌─────────────────────┐       │ tenant_id (PK)      │
│  ciphertext_digest  │       │ tenant_api_key      │
├─────────────────────┤       │ chain_id            │
│ tenant_id (PK)      │       │ pks_key (BYTEA)     │
│ handle (PK)         │       │ sks_key (BYTEA)     │
│ ciphertext (ct64)   │       │ public_params       │
│ ciphertext128       │       └─────────────────────┘
│ ciphertext128_format│
│ txn_is_sent         │
│ created_at          │
└─────────────────────┘
```

### Indices and Query Optimization

**December 2025 Optimizations:**

1. **Pending Tasks Index** (`20251205070512`):
   ```sql
   CREATE INDEX idx_pending_tasks
       ON pbs_computations USING btree (created_at)
       WHERE is_completed = false;
   ```
   - Selective index (only incomplete tasks)
   - Enables efficient backlog queries

2. **Ciphertext Digest Indices** (`20251203140023`):
   ```sql
   CREATE INDEX idx_ciphertext_digest_handle
       ON ciphertext_digest (handle);

   CREATE INDEX idx_ciphertext_digest_unsent
       ON ciphertext_digest (txn_is_sent, created_at);
   ```
   - Fast handle lookups for transaction-sender
   - FIFO ordering for fair transaction processing

**Index Types:**
- `BTREE`: Range scans, ordering (schedule_order, created_at)
- `GIN`: Array containment (dependencies)
- `HASH`: Equality lookups (transaction_id)

### Ciphertext Storage Patterns

**Compression Format Tracking:**
```sql
-- ciphertext128_format values:
-- 0  = Unknown
-- 10 = UncompressedOnCpu
-- 11 = CompressedOnCpu
-- 20 = UncompressedOnGpu
-- 21 = CompressedOnGpu
```

**S3 Digest Storage:**
- `ciphertext` column: 64-bit compressed digest
- `ciphertext128` column: 128-bit digest
- Nullable fields allow gradual migration to 128-bit format

---

## Cross-Component Integration

### Gateway Integration

**Event Reception** (`gw-listener/src/gw_listener.rs`):
- WebSocket subscription via `eth_subscribe` to InputVerification contract
- Handles `VerifyProofRequest` events
- Polling-based for `ActivateCrs` and `ActivateKey` from KMSGeneration

**Result Submission** (`transaction-sender/src/ops/`):
- `verify_proof.rs`: Submits verified proofs to InputVerification
- `add_ciphertext.rs`: Submits ciphertexts to CiphertextCommitment
- `allow_handle.rs`: Submits handle allowances to MultiChainACL

### KMS Integration

**Key Management:**
- KMS keys received via Gateway `ActivateKey` events
- S3 download for large key files
- Stored in PostgreSQL `tenants` table (sks_key, pks_key, public_params)

### PostgreSQL Event Channels

| Channel | Producer | Consumer | Trigger |
|---------|----------|----------|---------|
| `work_available` | host-listener | tfhe-worker | New computation job |
| `event_ciphertext_computed` | tfhe-worker | sns-worker, zkproof-worker | FHE computation done |
| `event_zkpok_computed` | zkproof-worker | transaction-sender | Proof verified |
| `event_allowed_handle` | (internal) | transaction-sender | Handle allowance ready |

---

## Recent Development Focus (Dec 2025)

- **GPU optimization**: Two-stage memory reservation, RoundRobin device selection
- **Database indices**: Selective indices on pending tasks, FIFO ordering for transactions
- **Metrics collection**: SNS latency, ZK verify latency, FHE batch timing histograms
- **Health checking**: S3 readiness checks, liveness thresholds per worker
- **Compression**: Format tracking for 64-bit/128-bit ciphertexts

---

## Key Files Reference

| File | Purpose |
|------|---------|
| `tfhe-worker/src/tfhe_worker.rs` | Main FHE worker loop |
| `scheduler/src/dfg/scheduler.rs` | Scheduling strategies |
| `fhevm-engine-common/src/gpu_memory.rs` | GPU memory management |
| `sns-worker/src/executor.rs` | SNS service and S3 upload |
| `zkproof-worker/src/verifier.rs` | ZK proof verification |
| `db-migration/migrations/*.sql` | Database schema |

---

**Related:**
- [Gateway Contracts](gateway-contracts.md) - Provides events that trigger coprocessor jobs
- [KMS Connector](kms-connector.md) - Provides key material for operations
- [Architecture](../architecture.md) - Coprocessor's role in overall system
