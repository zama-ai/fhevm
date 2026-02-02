# Architecture - KMS & Gateway

Component structure and data flows for decryption infrastructure.

---

## Component Overview

```text
+------------------+     +------------------+     +------------------+
|   User Contract  |---->| Gateway Contract |---->|   KMS Connector  |
|   (Requester)    |     | (On-Chain)       |     |   (Off-Chain)    |
+------------------+     +--------+---------+     +--------+---------+
                                  |                        |
                                  | Events                 | gRPC
                                  v                        v
                         +--------+---------+     +--------+---------+
                         |  gw-listener     |     |   KMS Service    |
                         |  (Event Monitor) |     |   (Key Storage)  |
                         +------------------+     +------------------+
                                  |                        |
                                  | Jobs                   | Keys
                                  v                        v
                         +------------------+     +------------------+
                         |  kms-worker      |<----|  Threshold Sig   |
                         |  (Processing)    |     |  (Multi-Party)   |
                         +--------+---------+     +------------------+
                                  |
                                  | Callback
                                  v
                         +------------------+
                         |  tx-sender       |
                         |  (Submit Result) |
                         +------------------+
```

---

## Gateway Contracts

### GatewayContract.sol

Central contract for decryption requests.

| Function                | Purpose                              |
| ----------------------- | ------------------------------------ |
| `requestDecryption()`   | Submit decryption request            |
| `fulfillDecryption()`   | Receive decryption result            |
| `verifySignatures()`    | Validate threshold signatures        |

### InputVerifier.sol

Validates encrypted inputs from users.

| Function                | Purpose                              |
| ----------------------- | ------------------------------------ |
| `verify()`              | Validate input proof                 |
| `getInputHandle()`      | Extract handle from verified input   |

### KMSVerifier.sol

Verifies KMS threshold signatures.

| Function                | Purpose                              |
| ----------------------- | ------------------------------------ |
| `verifySignature()`     | Verify single KMS signature          |
| `verifyThreshold()`     | Verify threshold met                 |
| `addSigner()`           | Register KMS signer                  |

---

## KMS Connector Components

### gw-listener

Monitors blockchain for decryption events.

```rust
pub struct GwListener {
    provider: Provider,
    contract: GatewayContract,
    job_queue: JobQueue,
}

impl GwListener {
    async fn process_events(&self) -> Result<()> {
        let events = self.contract.get_decryption_requests().await?;
        for event in events {
            self.job_queue.submit(Job::Decryption(event)).await?;
        }
        Ok(())
    }
}
```

### kms-worker

Processes decryption jobs.

```rust
pub struct KmsWorker {
    kms_client: KmsClient,
    acl_verifier: AclVerifier,
}

impl KmsWorker {
    async fn process_job(&self, job: DecryptionJob) -> Result<DecryptionResult> {
        // 1. Verify ACL
        if !self.acl_verifier.verify(job.handle, job.requester).await? {
            return Err(Error::AclDenied);
        }

        // 2. Fetch key shares
        let shares = self.kms_client.get_key_shares(job.handle).await?;

        // 3. Compute decryption
        let plaintext = decrypt_with_shares(job.ciphertext, shares)?;

        // 4. Generate threshold signature
        let signatures = self.kms_client.sign_result(plaintext).await?;

        Ok(DecryptionResult { plaintext, signatures })
    }
}
```

### tx-sender

Submits decryption results on-chain.

```rust
pub struct TxSender {
    wallet: Wallet,
    gateway: GatewayContract,
}

impl TxSender {
    async fn submit_result(&self, result: DecryptionResult) -> Result<TxHash> {
        let tx = self.gateway.fulfill_decryption(
            result.handle,
            result.plaintext,
            result.signatures,
        );
        self.wallet.send_transaction(tx).await
    }
}
```

---

## Decryption Flow

### Step-by-Step

```text
1. User contract calls FHE.makePubliclyDecryptable(handle)
2. User contract calls Gateway.requestDecryption(handle, callback)
3. GatewayContract emits DecryptionRequested event
4. gw-listener detects event, creates DecryptionJob
5. kms-worker verifies ACL (handle, requester)
6. kms-worker fetches key shares from KMS
7. kms-worker decrypts ciphertext
8. KMS nodes sign result (threshold signatures)
9. tx-sender calls Gateway.fulfillDecryption(result, signatures)
10. GatewayContract verifies signatures via KMSVerifier
11. GatewayContract calls user callback with plaintext
```

### Sequence Diagram

```text
User      Gateway      Listener    Worker       KMS
 |           |            |          |           |
 |--request->|            |          |           |
 |           |--event---->|          |           |
 |           |            |--job---->|           |
 |           |            |          |--verify-->|
 |           |            |          |<--ok------|
 |           |            |          |--decrypt->|
 |           |            |          |<-result---|
 |           |            |          |--sign---->|
 |           |            |          |<-sigs-----|
 |           |<-----------+-fulfill-+|           |
 |<-callback-|            |          |           |
```

---

## Database Schema

### Tables

| Table              | Purpose                              |
| ------------------ | ------------------------------------ |
| `decryption_jobs`  | Pending/active decryption requests   |
| `key_shares`       | Distributed key material (encrypted) |
| `acl_cache`        | Cached ACL permissions               |
| `transactions`     | Submitted transaction tracking       |

### Job States

```text
PENDING -> PROCESSING -> SIGNING -> SUBMITTING -> COMPLETE
                |            |           |
                v            v           v
              FAILED      FAILED      FAILED
```

---

## Configuration

### Environment Variables

| Variable                | Purpose                              |
| ----------------------- | ------------------------------------ |
| `GATEWAY_ADDRESS`       | GatewayContract address              |
| `KMS_ENDPOINT`          | KMS service endpoint                 |
| `THRESHOLD`             | Required signature threshold         |
| `DATABASE_URL`          | PostgreSQL connection                |
| `RPC_URL`               | Blockchain RPC endpoint              |

### Security Configuration

| Setting                 | Value                                |
| ----------------------- | ------------------------------------ |
| Key encryption          | AES-256-GCM                          |
| Signature scheme        | BLS threshold                        |
| Threshold               | 2-of-3 (configurable)                |
| ACL cache TTL           | 60 seconds                           |
