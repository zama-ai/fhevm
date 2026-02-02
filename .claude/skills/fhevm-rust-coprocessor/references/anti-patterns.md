# Anti-Patterns - Rust Coprocessor

Patterns that trigger CHANGES_REQUESTED in code review.

---

## Error Handling Anti-Patterns

### 1. Mixed Error Categories

```rust
// BAD: Mixing Recoverable and Irrecoverable in same path
fn process_request(req: Request) -> Result<Response, Error> {
    let data = fetch_data().map_err(|e| Error::Recoverable(e))?;  // Network error
    let parsed = parse(data).map_err(|e| Error::Recoverable(e))?; // Logic error - WRONG
    Ok(parsed)
}

// GOOD: Correct categorization
fn process_request(req: Request) -> Result<Response, Error> {
    let data = fetch_data().map_err(|e| Error::Recoverable(e))?;  // Network - retry
    let parsed = parse(data).map_err(|e| Error::Irrecoverable(e))?; // Logic - fail
    Ok(parsed)
}
```

### 2. Swallowing Errors

```rust
// BAD: Silent failure
let _ = send_metric(data);

// GOOD: Log on failure
if let Err(e) = send_metric(data) {
    tracing::warn!("Failed to send metric: {}", e);
}
```

### 3. Panic in Library Code

```rust
// BAD: Panicking in library
pub fn compute(input: &[u8]) -> Output {
    assert!(!input.is_empty()); // Will panic
    // ...
}

// GOOD: Return Result
pub fn compute(input: &[u8]) -> Result<Output, ComputeError> {
    if input.is_empty() {
        return Err(ComputeError::EmptyInput);
    }
    // ...
}
```

---

## Retry Anti-Patterns

### 1. Unbounded Retries

```rust
// BAD: No limit
loop {
    if operation().is_ok() {
        break;
    }
    tokio::time::sleep(Duration::from_secs(1)).await;
}

// GOOD: Bounded with backoff
let backoff = ExponentialBackoff::builder()
    .with_max_retries(3)
    .with_jitter()
    .build();

retry(backoff, || async { operation().await }).await
```

### 2. Fixed Delays

```rust
// BAD: Fixed delay causes thundering herd
tokio::time::sleep(Duration::from_secs(5)).await;

// GOOD: Exponential backoff with jitter
let delay = base_delay * 2u64.pow(attempt) + random_jitter();
tokio::time::sleep(Duration::from_millis(delay)).await;
```

### 3. Retrying Non-Transient Errors

```rust
// BAD: Retrying validation errors
retry(backoff, || async {
    validate_signature(data)?; // This won't succeed on retry
    Ok(())
}).await

// GOOD: Only retry transient errors
match validate_signature(data) {
    Ok(()) => Ok(()),
    Err(e) if e.is_transient() => Err(backoff::Error::transient(e)),
    Err(e) => Err(backoff::Error::permanent(e)),
}
```

---

## Concurrency Anti-Patterns

### 1. Lock Ordering Violations

```rust
// BAD: Inconsistent lock ordering -> deadlock risk
// Thread 1: lock_a -> lock_b
// Thread 2: lock_b -> lock_a

// GOOD: Always acquire locks in consistent order
// Document the order in comments
// lock_a must always be acquired before lock_b
```

### 2. Holding Locks Across Await

```rust
// BAD: Lock held across await point
let guard = mutex.lock().await;
network_call().await; // Lock held during I/O
drop(guard);

// GOOD: Release lock before await
let data = {
    let guard = mutex.lock().await;
    guard.clone()
};
network_call_with_data(data).await;
```

### 3. Blocking in Async Context

```rust
// BAD: Blocking call in async
async fn process() {
    std::thread::sleep(Duration::from_secs(1)); // Blocks runtime
}

// GOOD: Use async sleep
async fn process() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

---

## Testing Anti-Patterns

### 1. Ignoring Feature Flags

```rust
// BAD: Test assumes CPU backend
#[test]
fn test_compute() {
    let result = compute(input);
    assert_eq!(result, expected);
}

// GOOD: Handle GPU feature
#[test]
fn test_compute() {
    #[cfg(feature = "gpu")]
    let result = compute_gpu(input);
    #[cfg(not(feature = "gpu"))]
    let result = compute_cpu(input);
    assert_eq!(result, expected);
}
```

### 2. Test Fixture Mismatches

After format changes, test fixtures may become stale:

```rust
// BAD: Hardcoded fixture that doesn't match current format
let expected = include_bytes!("fixtures/old_format.bin");

// GOOD: Generate expected value or document format version
let expected = generate_expected_output(input);
// Or: fixtures/v2_format.bin with version comment
```

### 3. Flaky Async Tests

```rust
// BAD: Race condition in test
#[tokio::test]
async fn test_concurrent() {
    spawn(task_a());
    spawn(task_b());
    tokio::time::sleep(Duration::from_millis(100)).await; // Hope they finish
    assert!(result.is_ready());
}

// GOOD: Proper synchronization
#[tokio::test]
async fn test_concurrent() {
    let (tx, rx) = oneshot::channel();
    spawn(async move { task_a().await; tx.send(()).unwrap(); });
    rx.await.unwrap();
    assert!(result.is_ready());
}
```

---

## Documentation Anti-Patterns

### 1. Outdated Examples

```rust
// BAD: Example uses deprecated API
/// # Example
/// ```
/// let client = Client::new(); // Deprecated
/// ```

// GOOD: Current API with realistic values
/// # Example
/// ```
/// let config = ClientConfig::builder()
///     .with_timeout(Duration::from_secs(30))
///     .build();
/// let client = Client::with_config(config);
/// ```
```

### 2. Missing Safety Documentation

```rust
// BAD: Unsafe without explanation
pub unsafe fn transmute_data(data: &[u8]) -> &Data {
    std::mem::transmute(data)
}

// GOOD: Document safety requirements
/// Transmutes raw bytes to Data.
///
/// # Safety
/// - `data` must be properly aligned for `Data`
/// - `data` must contain valid `Data` representation
/// - `data` must outlive the returned reference
pub unsafe fn transmute_data(data: &[u8]) -> &Data {
    std::mem::transmute(data)
}
```
