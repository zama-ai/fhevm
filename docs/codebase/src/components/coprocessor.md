# Coprocessor 🔥

**Location**: `/coprocessor/`
**Status**: Active Development
**Purpose**: Rust-based asynchronous FHE computation engine

## Overview

The Coprocessor is the off-chain component that performs actual FHE computations. It listens to events from Host and Gateway contracts, executes the expensive cryptographic operations, and submits results back to the chain.

## Key Crates

Located in `/coprocessor/fhevm-engine/`:

| Crate | Purpose |
|-------|---------|
| `tfhe-worker` | Core FHE computation engine using TFHE-rs |
| `scheduler` | Job orchestration and work distribution |
| `zkproof-worker` | Zero-knowledge proof generation |
| `sns-worker` | Switch and Squash optimization for ciphertexts |
| `host-listener` | Monitors host chain events |
| `gw-listener` | Monitors gateway chain events |
| `transaction-sender` | Broadcasts results back to chain |

## Architecture

**Event-driven design:**
- Listeners pick up on-chain events and create jobs
- Jobs stored in PostgreSQL database
- Scheduler distributes work to specialized workers
- Workers process jobs concurrently
- Results submitted back via transaction-sender

**Threshold consensus:**
- Multiple coprocessors can run for redundancy
- Results require threshold agreement
- Gateway contracts verify consensus

## Key Files

- `fhevm-engine/tfhe-worker/` - TFHE computation implementation
- `fhevm-engine/scheduler/` - Job queue and distribution
- `Cargo.toml` - Workspace manifest

## Relationships

The Coprocessor receives work from Gateway contract events, processes FHE operations, and submits results via transaction-sender. It coordinates with the KMS Connector for key material.

## Recent Development Focus (Dec 2025)

- **GPU optimization**: Improved GPU scheduler and memory management
- **Metrics**: Collection of SNS latency, ZK verify latency, tfhe-per-txn timing
- **Health checking**: Added health checks in tfhe-worker and sns-worker
- **Database optimization**: Indices on ciphertext_digest, improved schedule ordering
- **Compression**: Large ciphertext compression strategies
- **Off-chain execution**: Optimization for batch processing

## Areas for Deeper Documentation

**[TODO: Worker architecture]** - Detail the tfhe-worker, zkproof-worker, and sns-worker implementations and their processing pipelines. Explain TFHE-rs integration and optimization strategies.

**[TODO: Scheduler and job orchestration]** - Document the job lifecycle from event reception to result submission. Explain priority queues, retry logic, and failure handling.

**[TODO: Database schema]** - Detail the PostgreSQL schema for job state, ciphertext storage, and result tracking.

**[TODO: GPU utilization]** - Explain GPU scheduling, memory management, and performance tuning for FHE operations.

**[TODO: Consensus mechanism]** - Document how multiple coprocessors coordinate and reach agreement on computation results.

---

**Related:**
- [Gateway Contracts](gateway-contracts.md) - Provides events that trigger coprocessor jobs
- [KMS Connector](kms-connector.md) - Provides key material for operations
- [Architecture](../architecture.md) - Coprocessor's role in overall system
