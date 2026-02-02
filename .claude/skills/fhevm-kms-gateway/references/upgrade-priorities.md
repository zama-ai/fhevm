# Upgrade Priorities - KMS & Gateway

Current status and recommended upgrades for KMS/Gateway components.

---

## Dependency Status

### Healthy Dependencies

| Dependency        | Version | Status   | Notes                        |
| ----------------- | ------- | -------- | ---------------------------- |
| aws-sdk-kms       | 1.x     | Current  | KMS integration              |
| tokio             | 1.x     | Current  | Async runtime                |
| sqlx              | 0.8     | Current  | Database operations          |
| tracing           | 0.1     | Current  | Observability                |
| moka              | 0.12    | Current  | Caching                      |

### Needs Attention

| Dependency        | Current | Latest   | Priority | Notes              |
| ----------------- | ------- | -------- | -------- | ------------------ |
| tonic             | 0.12    | 0.14     | Medium   | gRPC improvements  |
| alloy             | 1.0.x   | 1.4.1    | High     | Security fix       |

---

## Production Hardening Priorities

### Priority 1: Observability

Add comprehensive metrics:

```rust
use prometheus::{Counter, Histogram, register_counter, register_histogram};

lazy_static! {
    static ref DECRYPTION_REQUESTS: Counter = register_counter!(
        "gateway_decryption_requests_total",
        "Total decryption requests"
    ).unwrap();

    static ref DECRYPTION_DURATION: Histogram = register_histogram!(
        "gateway_decryption_duration_seconds",
        "Decryption processing duration"
    ).unwrap();

    static ref ACL_DENIALS: Counter = register_counter!(
        "gateway_acl_denials_total",
        "Total ACL denial events"
    ).unwrap();
}
```

### Priority 2: Circuit Breakers

Ensure all external calls have circuit breakers:

| Service           | Circuit Breaker | Status    |
| ----------------- | --------------- | --------- |
| AWS KMS           | Yes             | Done      |
| Blockchain RPC    | Partial         | In Progress |
| Database          | No              | TODO      |

### Priority 3: Health Checks

Comprehensive health endpoints:

```rust
#[derive(Serialize)]
struct HealthStatus {
    kms: ComponentHealth,
    database: ComponentHealth,
    blockchain: ComponentHealth,
    overall: HealthState,
}

async fn health_check() -> Json<HealthStatus> {
    let kms = check_kms_health().await;
    let database = check_db_health().await;
    let blockchain = check_rpc_health().await;

    let overall = if all_healthy(&[&kms, &database, &blockchain]) {
        HealthState::Healthy
    } else {
        HealthState::Degraded
    };

    Json(HealthStatus { kms, database, blockchain, overall })
}
```

---

## Security Upgrades

### Required

| Upgrade           | Description                      | Priority |
| ----------------- | -------------------------------- | -------- |
| alloy 1.4.1+      | Fix DoS vulnerability            | Critical |
| Key rotation      | Implement automatic rotation     | High     |
| Audit log signing | Tamper-evident audit logs        | Medium   |

### Recommended

| Upgrade           | Description                      | Priority |
| ----------------- | -------------------------------- | -------- |
| HSM integration   | Hardware security module support | Medium   |
| mTLS              | Mutual TLS for internal services | Medium   |
| Secret rotation   | Automated secret rotation        | Low      |

---

## Architecture Improvements

### Current State

```text
Single listener -> Single worker pool -> Single tx-sender
```

### Target State

```text
Multiple listeners (leader election)
    |
    v
Distributed job queue (Redis/Kafka)
    |
    v
Scaled worker pool (auto-scaling)
    |
    v
Multiple tx-senders (load balanced)
```

### Migration Steps

1. Add leader election for listeners
2. Introduce distributed job queue
3. Enable horizontal worker scaling
4. Implement tx-sender load balancing

---

## Testing Improvements

### Current Coverage

| Component         | Unit Tests | Integration | E2E      |
| ----------------- | ---------- | ----------- | -------- |
| gw-listener       | 80%        | 60%         | 40%      |
| kms-worker        | 85%        | 70%         | 50%      |
| tx-sender         | 75%        | 50%         | 40%      |
| Gateway contracts | 90%        | 80%         | 60%      |

### Target Coverage

| Component         | Unit Tests | Integration | E2E      |
| ----------------- | ---------- | ----------- | -------- |
| All               | 90%        | 80%         | 70%      |

### Priority Test Additions

1. ACL edge cases
2. Circuit breaker behavior
3. Threshold signature failures
4. Multi-region scenarios

---

## Performance Priorities

### Bottlenecks Identified

| Bottleneck        | Impact     | Mitigation               |
| ----------------- | ---------- | ------------------------ |
| ACL verification  | Medium     | Implement caching        |
| KMS latency       | High       | Regional KMS endpoints   |
| Database queries  | Medium     | Query optimization       |

### Optimization Roadmap

1. **Q1**: ACL caching implementation
2. **Q2**: Regional KMS routing
3. **Q3**: Database query optimization
4. **Q4**: Full performance audit

---

## Upgrade Checklist

### Before Any Upgrade

- [ ] Review changelog for breaking changes
- [ ] Test in staging environment
- [ ] Verify security implications
- [ ] Update documentation
- [ ] Plan rollback strategy

### After Upgrade

- [ ] Verify metrics are reporting
- [ ] Check error rates
- [ ] Validate health checks
- [ ] Monitor for 24 hours
