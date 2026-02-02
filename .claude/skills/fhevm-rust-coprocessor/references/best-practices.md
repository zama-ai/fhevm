# Best Practices - Rust Coprocessor

Current best practices with sources, aligned with fhevm codebase.

---

## Error Handling

### Use thiserror for Libraries

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComputeError {
    #[error("invalid ciphertext: {0}")]
    InvalidCiphertext(String),

    #[error("operation not supported: {op}")]
    UnsupportedOperation { op: String },

    #[error("TFHE error: {0}")]
    TfheError(#[from] tfhe::Error),
}
```

**Source**: [thiserror docs](https://docs.rs/thiserror)

### Use anyhow for Applications

```rust
use anyhow::{Context, Result};

async fn process_job(job_id: u64) -> Result<()> {
    let job = fetch_job(job_id)
        .await
        .context("failed to fetch job")?;

    execute_job(&job)
        .await
        .with_context(|| format!("failed to execute job {}", job_id))?;

    Ok(())
}
```

**Source**: [anyhow docs](https://docs.rs/anyhow)

---

## Async Patterns

### Prefer tokio::select! for Multiple Futures

```rust
use tokio::select;

async fn run_with_timeout(operation: impl Future<Output = Result<()>>) -> Result<()> {
    select! {
        result = operation => result,
        _ = tokio::time::sleep(Duration::from_secs(30)) => {
            Err(anyhow::anyhow!("operation timed out"))
        }
    }
}
```

**Source**: [Tokio tutorial - Select](https://tokio.rs/tokio/tutorial/select)

### Structured Concurrency with JoinSet

```rust
use tokio::task::JoinSet;

async fn process_batch(items: Vec<Item>) -> Vec<Result<Output>> {
    let mut set = JoinSet::new();

    for item in items {
        set.spawn(async move { process_item(item).await });
    }

    let mut results = Vec::new();
    while let Some(res) = set.join_next().await {
        results.push(res.unwrap_or_else(|e| Err(e.into())));
    }
    results
}
```

**Source**: [Tokio JoinSet](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html)

---

## Database (SQLx)

### Compile-Time Checked Queries

```rust
use sqlx::FromRow;

#[derive(FromRow)]
struct Operation {
    id: i64,
    status: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

async fn get_pending_operations(pool: &PgPool) -> Result<Vec<Operation>> {
    sqlx::query_as!(
        Operation,
        r#"
        SELECT id, status, created_at
        FROM operations
        WHERE status = 'pending'
        ORDER BY created_at ASC
        LIMIT 100
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}
```

**Source**: [SQLx docs](https://docs.rs/sqlx)

### Transaction Management

```rust
async fn update_with_transaction(pool: &PgPool, op_id: i64) -> Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query!("UPDATE operations SET status = 'processing' WHERE id = $1", op_id)
        .execute(&mut *tx)
        .await?;

    // More operations...

    tx.commit().await?;
    Ok(())
}
```

---

## gRPC (Tonic)

### Service Implementation

```rust
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl Coprocessor for CoprocessorService {
    async fn submit_operation(
        &self,
        request: Request<OperationRequest>,
    ) -> Result<Response<OperationResponse>, Status> {
        let req = request.into_inner();

        let result = self.scheduler
            .submit(req)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(OperationResponse { id: result.id }))
    }
}
```

**Source**: [Tonic docs](https://docs.rs/tonic)

### Error Mapping

```rust
impl From<ComputeError> for Status {
    fn from(err: ComputeError) -> Self {
        match err {
            ComputeError::InvalidCiphertext(_) => Status::invalid_argument(err.to_string()),
            ComputeError::UnsupportedOperation { .. } => Status::unimplemented(err.to_string()),
            ComputeError::TfheError(_) => Status::internal(err.to_string()),
        }
    }
}
```

---

## TFHE-rs Integration

### Ciphertext Operations

```rust
use tfhe::prelude::*;
use tfhe::{set_server_key, FheUint64};

fn compute_addition(
    a: &FheUint64,
    b: &FheUint64,
    server_key: &ServerKey,
) -> FheUint64 {
    set_server_key(server_key.clone());
    a + b
}
```

**Source**: [TFHE-rs docs](https://docs.zama.ai/tfhe-rs)

### Parallelization

```rust
use rayon::prelude::*;

fn batch_compute(
    operations: Vec<(FheUint64, FheUint64)>,
    server_key: &ServerKey,
) -> Vec<FheUint64> {
    operations
        .par_iter()
        .map(|(a, b)| {
            set_server_key(server_key.clone());
            a + b
        })
        .collect()
}
```

---

## Observability

### Structured Logging with tracing

```rust
use tracing::{info, instrument, warn};

#[instrument(skip(ciphertext), fields(op_id = %op.id))]
async fn execute_operation(op: &Operation, ciphertext: &[u8]) -> Result<Vec<u8>> {
    info!("starting operation execution");

    let result = compute(ciphertext).await?;

    info!(result_size = result.len(), "operation completed");
    Ok(result)
}
```

**Source**: [tracing docs](https://docs.rs/tracing)

### Metrics with prometheus

```rust
use prometheus::{Counter, Histogram, register_counter, register_histogram};

lazy_static! {
    static ref OPS_TOTAL: Counter = register_counter!(
        "fhevm_operations_total",
        "Total number of FHE operations"
    ).unwrap();

    static ref OP_DURATION: Histogram = register_histogram!(
        "fhevm_operation_duration_seconds",
        "FHE operation duration"
    ).unwrap();
}
```

**Source**: [prometheus-rust docs](https://docs.rs/prometheus)

---

## Testing

### Async Test Setup

```rust
#[tokio::test]
async fn test_operation_flow() {
    let pool = setup_test_db().await;
    let service = CoprocessorService::new(pool.clone());

    let result = service.submit_operation(test_request()).await;

    assert!(result.is_ok());
    cleanup_test_db(pool).await;
}
```

### Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_serialization_roundtrip(data: Vec<u8>) {
        let serialized = serialize(&data);
        let deserialized = deserialize(&serialized).unwrap();
        prop_assert_eq!(data, deserialized);
    }
}
```

**Source**: [proptest docs](https://docs.rs/proptest)
