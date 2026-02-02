# Best Practices - KMS & Gateway

Current best practices for KMS/Gateway development with sources.

---

## AWS KMS Integration

### Client Configuration

```rust
use aws_sdk_kms::Client;
use aws_config::retry::RetryConfig;

async fn create_kms_client() -> Client {
    let config = aws_config::from_env()
        .retry_config(
            RetryConfig::standard()
                .with_max_attempts(3)
                .with_initial_backoff(Duration::from_millis(100))
        )
        .load()
        .await;

    Client::new(&config)
}
```

**Source**: [AWS SDK Rust](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html)

### Key Operations

```rust
async fn get_decryption_key(
    client: &Client,
    key_id: &str,
    ciphertext: &[u8],
) -> Result<Vec<u8>> {
    let response = client
        .decrypt()
        .key_id(key_id)
        .ciphertext_blob(Blob::new(ciphertext))
        .send()
        .await?;

    Ok(response.plaintext.unwrap().into_inner())
}
```

---

## Circuit Breaker Pattern

Protect against cascading failures:

```rust
use failsafe::{Config, CircuitBreaker};

fn create_circuit_breaker() -> CircuitBreaker<()> {
    Config::new()
        .failure_rate_threshold(0.5)        // 50% failure rate
        .min_call_volume(10)                 // After 10 calls
        .open_wait(Duration::from_secs(30)) // Wait before retry
        .build()
}

async fn call_with_breaker<F, T>(
    breaker: &CircuitBreaker<()>,
    f: F,
) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    if !breaker.is_call_permitted() {
        return Err(Error::CircuitOpen);
    }

    match f.await {
        Ok(v) => {
            breaker.on_success();
            Ok(v)
        }
        Err(e) => {
            breaker.on_error();
            Err(e)
        }
    }
}
```

**Source**: [Failsafe Pattern](https://martinfowler.com/bliki/CircuitBreaker.html)

---

## Audit Logging

Log all operations without exposing sensitive data:

```rust
use tracing::{info, warn, instrument};

#[instrument(skip(ciphertext), fields(handle = %handle))]
async fn process_decryption(
    handle: Handle,
    requester: Address,
    ciphertext: &[u8],
) -> Result<()> {
    info!(
        requester = %requester,
        ciphertext_size = ciphertext.len(),
        "Processing decryption request"
    );

    let result = decrypt(handle, ciphertext).await;

    match &result {
        Ok(_) => info!("Decryption successful"),
        Err(e) => warn!(error = %e, "Decryption failed"),
    }

    result
}
```

**Never log**: Key material, plaintexts, or full ciphertexts.

**Source**: [OWASP Logging Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html)

---

## Threshold Signature Verification

### Solidity Implementation

```solidity
function verifyThresholdSignatures(
    bytes32 messageHash,
    bytes[] calldata signatures,
    uint256 threshold
) internal view returns (bool) {
    if (signatures.length < threshold) {
        return false;
    }

    address[] memory signers = new address[](signatures.length);
    uint256 validCount = 0;

    for (uint256 i = 0; i < signatures.length; i++) {
        address signer = ECDSA.recover(messageHash, signatures[i]);

        // Check signer is authorized and not duplicate
        if (isAuthorizedSigner[signer] && !isDuplicate(signers, signer, i)) {
            signers[validCount] = signer;
            validCount++;
        }
    }

    return validCount >= threshold;
}
```

**Source**: [OpenZeppelin ECDSA](https://docs.openzeppelin.com/contracts/5.x/api/utils#ECDSA)

---

## ACL Verification

### Caching Strategy

```rust
use moka::future::Cache;

struct AclCache {
    cache: Cache<(Handle, Address), bool>,
}

impl AclCache {
    fn new() -> Self {
        Self {
            cache: Cache::builder()
                .time_to_live(Duration::from_secs(60))
                .max_capacity(10_000)
                .build(),
        }
    }

    async fn is_allowed(&self, handle: Handle, requester: Address) -> Result<bool> {
        if let Some(result) = self.cache.get(&(handle, requester)).await {
            return Ok(result);
        }

        // Cache miss - fetch from contract
        let result = self.fetch_from_contract(handle, requester).await?;
        self.cache.insert((handle, requester), result).await;
        Ok(result)
    }
}
```

**Source**: [moka cache](https://docs.rs/moka)

### Validation Pattern

```rust
async fn validate_decryption_request(
    handle: Handle,
    requester: Address,
    acl: &AclCache,
) -> Result<()> {
    // Check if publicly decryptable first
    if is_publicly_decryptable(handle).await? {
        return Ok(());
    }

    // Check requester permission
    if !acl.is_allowed(handle, requester).await? {
        return Err(Error::AclDenied { handle, requester });
    }

    Ok(())
}
```

---

## Error Handling

### Error Categories

```rust
#[derive(Error, Debug)]
pub enum GatewayError {
    // Recoverable - retry appropriate
    #[error("KMS timeout: {0}")]
    KmsTimeout(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    // Irrecoverable - fail fast
    #[error("ACL denied for handle {handle} requester {requester}")]
    AclDenied { handle: Handle, requester: Address },

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Handle not found: {0}")]
    HandleNotFound(Handle),
}

impl GatewayError {
    pub fn is_recoverable(&self) -> bool {
        matches!(self, Self::KmsTimeout(_) | Self::NetworkError(_))
    }
}
```

---

## Testing

### Security Test Pattern

```rust
#[tokio::test]
async fn test_acl_denial_prevents_decryption() {
    let gateway = setup_test_gateway().await;

    // Create handle owned by alice
    let handle = create_test_handle(&gateway, alice()).await;

    // Bob should not be able to decrypt
    let result = gateway.decrypt(handle, bob()).await;

    assert!(matches!(result, Err(GatewayError::AclDenied { .. })));
}

#[tokio::test]
async fn test_threshold_signature_requirement() {
    let gateway = setup_test_gateway().await;
    let handle = create_test_handle(&gateway, alice()).await;

    // Single signature should fail (threshold = 2)
    let single_sig = vec![sign_as_kms_node(1, handle)];
    let result = gateway.verify_and_fulfill(handle, plaintext, single_sig).await;

    assert!(matches!(result, Err(GatewayError::InsufficientSignatures)));

    // Two signatures should succeed
    let double_sig = vec![
        sign_as_kms_node(1, handle),
        sign_as_kms_node(2, handle),
    ];
    let result = gateway.verify_and_fulfill(handle, plaintext, double_sig).await;

    assert!(result.is_ok());
}
```
