# Trajectory - KMS & Gateway

Where KMS/Gateway components are heading based on git history analysis.

---

## Growing Areas

### Production Hardening

**Status**: Active development

Focus areas:

- Enhanced circuit breaker patterns
- Improved health checks
- Better observability and metrics
- Disaster recovery procedures

**Implication**: All new code must include:

- Health check contributions
- Metrics emission
- Proper error categorization
- Graceful shutdown handling

### Multi-Region Support

**Status**: Planning phase

Preparing for geographically distributed KMS:

- Region-aware key routing
- Cross-region failover
- Latency optimization

**Implication**: Design for eventual multi-region deployment.

### Enhanced Observability

Recent additions:

- Structured logging throughout
- Prometheus metrics endpoints
- Distributed tracing with OpenTelemetry

**Implication**: Use tracing crate for all logging:

```rust
use tracing::{info, instrument};

#[instrument(skip(sensitive_data))]
async fn operation(handle: Handle, sensitive_data: &[u8]) {
    info!("Processing handle");
}
```

---

## Stable Areas

### Core Decryption Flow

The fundamental decryption flow is mature:

- Event detection (gw-listener)
- ACL verification
- Key fetching and decryption
- Threshold signature generation
- Result submission

**Implication**: Follow existing patterns exactly; changes need thorough review.

### Threshold Signature Verification

Cryptographic verification is stable:

- BLS signature scheme
- Threshold verification logic
- Signer management

**Implication**: Changes to crypto code require security review.

---

## Active Migrations

### Oracle to Gateway (95% Complete)

Terminology migration nearly complete:

| Old               | New               |
| ----------------- | ----------------- |
| `OracleContract`  | `GatewayContract` |
| `oracle-worker`   | `gateway-worker`  |
| `OracleDB`        | `GatewayDB`       |

**Implication**: Use "gateway" terminology exclusively in new code.

### Improved Error Types

Moving to structured errors:

```rust
// Old (being removed)
Err(anyhow!("ACL check failed"))

// New (use this)
Err(GatewayError::AclDenied { handle, requester })
```

---

## Declining Areas

### Synchronous Processing

Moving away from blocking operations:

- All I/O now async
- Database operations non-blocking
- KMS calls with proper timeouts

### Single-Instance Deployment

Legacy single-node architecture being replaced:

- Horizontal scaling for workers
- Leader election for listeners
- Distributed job queue

---

## Upcoming Changes

### Short Term (1-3 months)

- Complete Oracle to Gateway migration
- Add comprehensive metrics
- Improve circuit breaker coverage

### Medium Term (3-6 months)

- Multi-region KMS routing
- Enhanced key rotation
- Performance optimization

### Long Term (6+ months)

- Alternative KMS backends (HSM, cloud-agnostic)
- Federation support
- Advanced threshold schemes

---

## Pattern Evolution

### Error Handling Evolution

```text
v1 (deprecated): String errors with anyhow
v2 (current):    Structured thiserror enums
v3 (emerging):   Error contexts with error-stack
```

### Caching Evolution

```text
v1 (deprecated): In-memory HashMap
v2 (current):    moka with TTL
v3 (emerging):   Distributed cache (Redis)
```

### Signature Verification Evolution

```text
v1 (deprecated): Single signature
v2 (current):    Threshold signatures (2-of-3)
v3 (stable):     Configurable threshold
```

---

## Security Focus Areas

### Current Priorities

1. ACL verification completeness
2. Key material protection
3. Audit trail completeness
4. Threshold enforcement

### Upcoming Security Work

1. Enhanced key rotation
2. HSM integration
3. Audit log integrity
4. Zero-trust networking

---

## Migration Checklist

When updating KMS/Gateway code:

- [ ] Use "gateway" terminology
- [ ] Use structured errors (not string errors)
- [ ] Add tracing instrumentation
- [ ] Include metrics for new operations
- [ ] Ensure async/non-blocking operations
- [ ] Add circuit breakers for external calls
- [ ] Include health check contributions
