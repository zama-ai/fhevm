# Upgrade Priorities - Rust Coprocessor

Dependency health status and recommended upgrades.

---

## Critical Upgrades

### alloy (DoS Vulnerability)

**Current**: 1.0.38 - 1.1.2
**Target**: 1.4.1+
**Severity**: Critical

A denial-of-service vulnerability exists in alloy versions 1.0.38 through 1.1.2.
Upgrade immediately.

```toml
# Cargo.toml
[dependencies]
alloy = "1.4.1"
```

**Migration Notes**:

- API compatible for most use cases
- Some type aliases changed
- Review transaction building code

---

## Recommended Upgrades

### tonic (gRPC)

**Current**: 0.12
**Latest**: 0.14
**Priority**: Medium

New features in 0.14:

- Improved streaming performance
- Better error types
- HTTP/3 support (experimental)

```toml
[dependencies]
tonic = "0.14"
```

**Migration Notes**:

- `Request<T>` changes in metadata handling
- Update streaming implementations
- Test gRPC clients thoroughly

### axum (HTTP)

**Current**: 0.7
**Latest**: 0.8
**Priority**: Low

Version 0.8 improvements:

- Better error handling
- Simplified middleware
- Performance improvements

```toml
[dependencies]
axum = "0.8"
```

**Migration Notes**:

- Middleware trait changes
- Router API updates
- Health endpoint adjustments needed

---

## Healthy Dependencies

These are up-to-date and well-maintained:

| Dependency      | Version | Status   | Notes                        |
| --------------- | ------- | -------- | ---------------------------- |
| tfhe            | 1.5.1   | Current  | Core FHE library             |
| sqlx            | 0.8.6   | Current  | Database operations          |
| tokio           | 1.x     | Current  | Async runtime                |
| tracing         | 0.1     | Current  | Logging/observability        |
| serde           | 1.x     | Current  | Serialization                |
| prometheus      | 0.13    | Current  | Metrics                      |

---

## Dependency Health Scores

Overall Score: **78/100**

| Category        | Score | Notes                                    |
| --------------- | ----- | ---------------------------------------- |
| Security        | 70    | alloy vulnerability pending              |
| Currency        | 75    | tonic/axum slightly behind               |
| Maintenance     | 90    | All deps actively maintained             |
| Compatibility   | 80    | Minor breaking changes in updates        |

---

## Upgrade Strategy

### Phase 1: Security (Immediate)

1. Upgrade alloy to 1.4.1+
2. Run full test suite
3. Deploy to staging
4. Monitor for issues

### Phase 2: gRPC Stack (Next Sprint)

1. Upgrade tonic to 0.14
2. Update prost if needed
3. Test all gRPC endpoints
4. Update client libraries

### Phase 3: HTTP Stack (When Convenient)

1. Upgrade axum to 0.8
2. Update middleware implementations
3. Test health/metrics endpoints

---

## Blocked Upgrades

These upgrades are blocked pending other work:

| Dependency | Blocker                          | ETA        |
| ---------- | -------------------------------- | ---------- |
| hyper      | Waiting for tonic 0.14 compat    | With tonic |
| tower      | Waiting for axum 0.8 compat      | With axum  |

---

## Dependency Audit Commands

```bash
# Check for security advisories
cargo audit

# Check for outdated dependencies
cargo outdated

# Update Cargo.lock conservatively
cargo update --dry-run

# Check feature flag usage
cargo tree --features gpu
```

---

## Version Pinning Policy

| Category        | Policy                                   |
| --------------- | ---------------------------------------- |
| Security        | Pin exact version after security review  |
| Core (tfhe)     | Pin exact version, manual updates only   |
| Infrastructure  | Allow patch updates (1.x)                |
| Dev deps        | Allow minor updates (0.x)                |
