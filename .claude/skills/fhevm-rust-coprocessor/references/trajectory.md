# Trajectory - Rust Coprocessor

Where the Rust components are heading based on git history analysis.

---

## Growing Areas

### GPU Backends (Active Development)

**Status**: 60% complete migration

The coprocessor is actively adding GPU acceleration for FHE operations:

- CUDA backend implementation in progress
- Benchmark infrastructure being added
- Feature flag `gpu` controls enablement

**Implication**: New FHE operation code should be GPU-aware:

```rust
#[cfg(feature = "gpu")]
fn compute_op(a: &Ciphertext, b: &Ciphertext) -> Ciphertext {
    gpu::compute(a, b)
}

#[cfg(not(feature = "gpu"))]
fn compute_op(a: &Ciphertext, b: &Ciphertext) -> Ciphertext {
    cpu::compute(a, b)
}
```

### Production Hardening

Recent commits show focus on:

- Improved error categorization (Recoverable/Irrecoverable/Fatal)
- Circuit breaker patterns for external services
- Graceful shutdown handling
- Health check endpoints

**Implication**: All new external integrations must include:

- Retry logic with exponential backoff
- Circuit breaker for repeated failures
- Proper error categorization
- Health check contribution

### Parallelization Improvements

Active work on:

- Better work distribution across workers
- Lock-free data structures where possible
- Reduced lock contention in hot paths

**Implication**: Prefer lock-free patterns:

```rust
// Prefer
use crossbeam::queue::ArrayQueue;
let queue = ArrayQueue::new(1000);

// Over
use std::sync::Mutex;
let queue = Mutex::new(VecDeque::new());
```

---

## Stable Areas

### Core TFHE-rs Integration

The TFHE-rs integration layer is mature and stable:

- tfhe version pinned at 1.5.1
- Well-established operation patterns
- Comprehensive test coverage

**Implication**: Follow existing patterns when adding new operations.

### gRPC Interfaces

Service interfaces are stable:

- Protobuf definitions well-established
- Breaking changes require deprecation period
- Version negotiation in place

**Implication**: New endpoints follow existing patterns; breaking changes need RFC.

### Database Schema

Core tables are stable:

- Migrations must be backwards compatible
- Schema changes require careful review
- Use nullable columns for new fields

---

## Active Migrations

### Oracle to Gateway (95% Complete)

Legacy "oracle" terminology being replaced with "gateway":

- `oracle-contracts/` -> `gateway-contracts/`
- `OracleDB` -> `GatewayDB`
- Event names updated

**Implication**: Use "gateway" terminology in all new code.

### Improved Error Types (In Progress)

Moving from string errors to structured types:

```rust
// Old pattern (being removed)
Err(format!("failed to process: {}", reason))

// New pattern (use this)
Err(ProcessError::Failed { reason })
```

---

## Declining Areas

### Legacy REST Endpoints

Some REST endpoints being deprecated in favor of gRPC:

- Health endpoints remain REST
- All compute operations moving to gRPC
- Migration timeline: 6 months

**Implication**: New functionality should use gRPC only.

### Docker Compose for Production

Moving toward Kubernetes for production:

- docker-compose remains for development
- Helm charts are the production standard
- New services need Helm chart definitions

---

## Upcoming Changes

Based on open issues and recent discussions:

### Short Term (1-3 months)

- Complete GPU backend implementation
- Add OpenTelemetry tracing
- Implement request batching

### Medium Term (3-6 months)

- Multi-region deployment support
- Enhanced ciphertext caching
- Performance benchmarking suite

### Long Term (6+ months)

- WebAssembly worker support
- Alternative KMS backends
- Federation protocol

---

## Pattern Evolution

### Error Handling Evolution

```text
v1 (deprecated): String errors
v2 (current):    thiserror enums
v3 (emerging):   Structured error contexts with error-stack
```

### Async Runtime Evolution

```text
v1 (deprecated): Manual Future implementations
v2 (current):    async/await with tokio
v3 (stable):     Structured concurrency with JoinSet
```

### Configuration Evolution

```text
v1 (deprecated): Environment variables only
v2 (current):    config crate with layered sources
v3 (emerging):   Feature flags with runtime configuration
```
