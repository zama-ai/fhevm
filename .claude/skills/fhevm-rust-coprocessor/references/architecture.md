# Architecture - Rust Coprocessor

Component structure and data flows for the FHE computation engine.

---

## Component Overview

```text
+------------------+     +------------------+     +------------------+
|   Blockchain     |     |   Coprocessor    |     |       KMS        |
|   (Host Node)    |     |   (FHE Engine)   |     |   (Key Mgmt)     |
+--------+---------+     +--------+---------+     +--------+---------+
         |                        |                        |
         | Events                 | Compute                | Keys
         v                        v                        v
+--------+---------+     +--------+---------+     +--------+---------+
|  gw-listener     |---->|  scheduler       |---->| kms-connector    |
|  (Event Monitor) |     |  (Job Queue)     |     | (Key Interface)  |
+------------------+     +--------+---------+     +------------------+
                                  |
                         +--------+--------+
                         |                 |
                         v                 v
                  +------+------+   +------+------+
                  |   workers   |   |   workers   |
                  |   (CPU)     |   |   (GPU)     |
                  +-------------+   +-------------+
```

---

## Core Components

### coprocessor/

The FHE computation engine responsible for executing encrypted operations.

| Component     | Purpose                              | Key Files                    |
| ------------- | ------------------------------------ | ---------------------------- |
| `scheduler`   | Job queue management, prioritization | `scheduler/mod.rs`           |
| `workers`     | FHE operation execution              | `workers/{cpu,gpu}/mod.rs`   |
| `api`         | gRPC service interface               | `api/grpc.rs`                |
| `storage`     | Ciphertext persistence               | `storage/mod.rs`             |

### kms-connector/

Interface layer for key management operations.

| Component     | Purpose                              | Key Files                    |
| ------------- | ------------------------------------ | ---------------------------- |
| `gw-listener` | Blockchain event monitoring          | `gw_listener/mod.rs`         |
| `kms-worker`  | Key operation processing             | `kms_worker/mod.rs`          |
| `tx-sender`   | Transaction submission               | `tx_sender/mod.rs`           |
| `db`          | State persistence (PostgreSQL)       | `db/mod.rs`                  |

### fhevm-engine/

Shared components used by both coprocessor and kms-connector.

| Component     | Purpose                              | Key Files                    |
| ------------- | ------------------------------------ | ---------------------------- |
| `tfhe`        | TFHE-rs integration                  | `tfhe/mod.rs`                |
| `types`       | Shared type definitions              | `types/mod.rs`               |
| `utils`       | Common utilities                     | `utils/mod.rs`               |

---

## Data Flows

### FHE Computation Flow

```text
1. Contract emits FHE operation event
2. gw-listener detects event, creates job
3. scheduler queues job by priority
4. worker claims job, fetches ciphertexts
5. worker executes FHE operation (TFHE-rs)
6. Result ciphertext stored
7. Callback triggered to contract
```

### Key Management Flow

```text
1. Decryption request received
2. kms-connector validates ACL permissions
3. Key material fetched from KMS (AWS KMS/HSM)
4. Decryption performed with threshold signatures
5. Result returned to requester
```

---

## Database Schema

Primary tables in PostgreSQL:

| Table              | Purpose                              |
| ------------------ | ------------------------------------ |
| `ciphertexts`      | Encrypted value storage              |
| `operations`       | Pending FHE operation queue          |
| `key_shares`       | Distributed key material             |
| `acl_entries`      | Permission records                   |

---

## gRPC Services

### Coprocessor Service

```protobuf
service Coprocessor {
  rpc SubmitOperation(OperationRequest) returns (OperationResponse);
  rpc GetResult(ResultRequest) returns (ResultResponse);
  rpc GetStatus(StatusRequest) returns (StatusResponse);
}
```

### KMS Service

```protobuf
service KmsConnector {
  rpc RequestDecryption(DecryptionRequest) returns (DecryptionResponse);
  rpc ValidateAccess(AccessRequest) returns (AccessResponse);
}
```

---

## Configuration

### Environment Variables

| Variable                | Purpose                              | Default          |
| ----------------------- | ------------------------------------ | ---------------- |
| `DATABASE_URL`          | PostgreSQL connection string         | Required         |
| `GRPC_PORT`             | gRPC server port                     | `50051`          |
| `WORKER_COUNT`          | Number of FHE workers                | CPU count        |
| `GPU_ENABLED`           | Enable GPU acceleration              | `false`          |
| `KMS_ENDPOINT`          | AWS KMS endpoint                     | Required         |
| `S3_BUCKET`             | Ciphertext storage bucket            | Required         |

### Feature Flags

| Flag         | Purpose                              |
| ------------ | ------------------------------------ |
| `gpu`        | Enable GPU backend (CUDA)            |
| `benchmark`  | Include benchmarking utilities       |
| `test-utils` | Test helpers and mocks               |

---

## Deployment Model

### Docker Compose (Development)

```yaml
services:
  coprocessor:
    build: ./coprocessor
    environment:
      - DATABASE_URL=postgres://...
      - GPU_ENABLED=false

  kms-connector:
    build: ./kms-connector
    environment:
      - KMS_ENDPOINT=http://localstack:4566
```

### Kubernetes (Production)

- Helm charts in `charts/` directory
- Horizontal pod autoscaling for workers
- Separate deployments for coprocessor and kms-connector
- PersistentVolumeClaims for ciphertext cache
