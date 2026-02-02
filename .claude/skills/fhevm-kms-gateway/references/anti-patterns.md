# Anti-Patterns - KMS & Gateway

Patterns that trigger CHANGES_REQUESTED in KMS/Gateway code review.

---

## Security Anti-Patterns

### 1. Skipping ACL Verification

```rust
// BAD: No ACL check before decryption
async fn decrypt(handle: Handle) -> Result<Plaintext> {
    let key = fetch_key(handle).await?;
    decrypt_with_key(handle, key)
}

// GOOD: Always verify ACL first
async fn decrypt(handle: Handle, requester: Address) -> Result<Plaintext> {
    if !verify_acl(handle, requester).await? {
        return Err(Error::Irrecoverable(AclDenied { handle, requester }));
    }
    let key = fetch_key(handle).await?;
    decrypt_with_key(handle, key)
}
```

### 2. Exposing Key Material in Logs

```rust
// BAD: Key in error message
tracing::error!("Decryption failed with key: {:?}", key);

// BAD: Key in error type
#[derive(Error)]
enum Error {
    DecryptionFailed { key: Vec<u8> },  // Exposes key
}

// GOOD: Redact sensitive data
tracing::error!("Decryption failed for handle: {:?}", handle);

#[derive(Error)]
enum Error {
    #[error("Decryption failed for handle {handle}")]
    DecryptionFailed { handle: Handle },
}
```

### 3. Fail-Open Behavior

```solidity
// BAD: Allows decryption on error
function requestDecryption(bytes32 handle) external {
    try acl.isAllowed(handle, msg.sender) returns (bool allowed) {
        if (!allowed) revert Unauthorized();
    } catch {
        // Silently continues - DANGEROUS
    }
    emit DecryptionRequested(handle);
}

// GOOD: Fail secure
function requestDecryption(bytes32 handle) external {
    bool allowed = acl.isAllowed(handle, msg.sender);
    if (!allowed) revert Unauthorized(handle, msg.sender);
    emit DecryptionRequested(handle);
}
```

### 4. Missing Signature Verification

```rust
// BAD: Trusts KMS response without verification
async fn process_decryption(response: KmsResponse) -> Result<()> {
    let plaintext = response.plaintext;  // Unverified!
    callback(plaintext).await
}

// GOOD: Verify threshold signatures
async fn process_decryption(response: KmsResponse) -> Result<()> {
    let signatures = response.signatures;
    if !verify_threshold_signatures(&signatures, &response.plaintext) {
        return Err(Error::InvalidSignatures);
    }
    callback(response.plaintext).await
}
```

---

## Retry Anti-Patterns

### 1. Unbounded Retries for KMS

```rust
// BAD: Can retry forever
loop {
    match kms_client.get_key(key_id).await {
        Ok(key) => return Ok(key),
        Err(_) => continue,  // Infinite loop potential
    }
}

// GOOD: Bounded with circuit breaker
let result = circuit_breaker
    .call(|| async {
        backoff::retry(
            ExponentialBackoff::default(),
            || kms_client.get_key(key_id),
        ).await
    })
    .await?;
```

### 2. Fixed Delays for External Calls

```rust
// BAD: Fixed delay causes thundering herd
tokio::time::sleep(Duration::from_secs(5)).await;
retry_kms_call().await;

// GOOD: Exponential backoff with jitter
let backoff = ExponentialBackoff::builder()
    .with_initial_delay(Duration::from_millis(100))
    .with_max_delay(Duration::from_secs(30))
    .with_jitter()
    .build();
```

### 3. Retrying Security Failures

```rust
// BAD: Retrying ACL denial
retry(backoff, || async {
    verify_acl(handle, requester).await  // This won't succeed on retry!
}).await

// GOOD: Only retry transient errors
match verify_acl(handle, requester).await {
    Ok(true) => proceed(),
    Ok(false) => Err(Error::Irrecoverable(AclDenied)),  // No retry
    Err(e) if e.is_transient() => Err(Error::Recoverable(e)),  // Retry
    Err(e) => Err(Error::Irrecoverable(e)),  // No retry
}
```

---

## Gateway Contract Anti-Patterns

### 1. Missing Event Emission

```solidity
// BAD: No audit trail
function processDecryption(bytes32 handle, bytes calldata result) external {
    // Process without logging
    _callback(handle, result);
}

// GOOD: Emit events for audit
event DecryptionProcessed(bytes32 indexed handle, address indexed requester);

function processDecryption(bytes32 handle, bytes calldata result) external {
    emit DecryptionProcessed(handle, msg.sender);
    _callback(handle, result);
}
```

### 2. Callback Without Reentrancy Protection

```solidity
// BAD: Vulnerable to reentrancy
function callback(address target, bytes calldata data) internal {
    (bool success,) = target.call(data);
    require(success, "Callback failed");
}

// GOOD: Use reentrancy guard
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

function callback(address target, bytes calldata data) internal nonReentrant {
    (bool success,) = target.call(data);
    require(success, "Callback failed");
}
```

### 3. Trusting External Signatures Without Threshold

```solidity
// BAD: Single signature acceptance
function verifyDecryption(bytes calldata signature) internal {
    address signer = recoverSigner(signature);
    require(isKmsSigner[signer], "Invalid signer");
}

// GOOD: Threshold verification
function verifyDecryption(bytes[] calldata signatures) internal {
    uint256 validCount = 0;
    for (uint256 i = 0; i < signatures.length; i++) {
        address signer = recoverSigner(signatures[i]);
        if (isKmsSigner[signer]) validCount++;
    }
    require(validCount >= threshold, "Insufficient signatures");
}
```

---

## Cross-Component Anti-Patterns

### 1. Inconsistent Error Handling

```rust
// Rust side: Recoverable error
if !verify_acl(handle, requester) {
    return Err(Error::Recoverable(AclDenied));  // WRONG category
}
```

```solidity
// Solidity side: Silent failure
if (!acl.isAllowed(handle, msg.sender)) {
    return;  // WRONG - should revert
}
```

**Both should be consistent**: ACL failures are Irrecoverable and should revert.

### 2. Missing State Synchronization

```rust
// Rust: Updates state
async fn mark_decryption_complete(handle: Handle) {
    db.update_status(handle, Status::Complete).await;
    // Missing: notify contract
}
```

```solidity
// Solidity: Doesn't know about completion
function getStatus(bytes32 handle) external view returns (Status) {
    return _status[handle];  // Still shows pending
}
```

**Fix**: Ensure state changes propagate to all components.
