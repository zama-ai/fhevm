# Trajectory - Test Suite

Where the test infrastructure is heading based on recent activity.

---

## Growing Areas

### E2E Test Coverage

**Status**: Active expansion

Focus areas:

- Complete flow coverage (mint -> transfer -> decrypt)
- Cross-component integration tests
- Failure scenario testing

**Implication**: New features require E2E tests:

```typescript
// Every new feature needs E2E coverage
describe("E2E: New Feature", function () {
  it("should complete full workflow", async function () {
    // Deploy -> Setup -> Execute -> Verify
  });

  it("should handle failure gracefully", async function () {
    // Deploy -> Setup -> Fail -> Verify recovery
  });
});
```

### GPU Test Infrastructure

**Status**: In development

Adding GPU-specific testing:

- GPU backend validation
- Performance comparison tests
- GPU-specific edge cases

**Implication**: Tests should be GPU-aware:

```rust
#[test]
#[cfg(feature = "gpu")]
fn test_gpu_compute() {
    // GPU-specific test
}

#[test]
fn test_compute() {
    // Generic test that works on both CPU and GPU
    #[cfg(feature = "gpu")]
    let result = gpu_compute(input);
    #[cfg(not(feature = "gpu"))]
    let result = cpu_compute(input);

    assert_eq!(result, expected);
}
```

### Performance Benchmarking

Adding systematic benchmarks:

- FHE operation benchmarks
- E2E flow timing
- Regression detection

**Implication**: Include benchmarks for performance-sensitive code:

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_fhe_add(c: &mut Criterion) {
    c.bench_function("fhe_add", |b| {
        b.iter(|| fhe_add(ct_a, ct_b))
    });
}

criterion_group!(benches, bench_fhe_add);
criterion_main!(benches);
```

---

## Stable Areas

### Unit Test Patterns

Established and mature:

- Rust test conventions
- Hardhat test structure
- Foundry test patterns

**Implication**: Follow existing patterns exactly.

### Hardhat Integration

The fhevmjs integration is stable:

- Encrypted input creation
- Event listening
- Contract deployment

---

## Active Migrations

### Fixture Format v2

Migrating test fixtures to new format:

| Old (v1)           | New (v2)           |
| ------------------ | ------------------ |
| Binary blobs       | JSON with metadata |
| Unversioned        | Version in path    |
| Manual generation  | Script generation  |

**Implication**: Use v2 fixtures:

```typescript
// Use v2 fixtures
import fixture from "./fixtures/v2/transfer_data.json";

// Or generate fresh
const fixture = await generateFixture();
```

### Test Isolation Improvements

Moving toward better test isolation:

- Fresh contract deployments per test
- Database cleanup between tests
- No shared state

---

## Declining Areas

### Legacy Mock Patterns

Old mock approaches being replaced:

```typescript
// Old (avoid)
global.mockKms = new ManualMock();

// New (use)
import { mock } from "jest-mock-extended";
const kms = mock<KmsClient>();
```

### Manual Fixture Management

Moving away from manually maintained fixtures:

```typescript
// Old (avoid)
const fixture = require("./fixtures/hardcoded.json");

// New (use)
const fixture = await generateFixture(seed);
```

---

## Upcoming Changes

### Short Term (1-3 months)

- Complete E2E coverage for all flows
- Add GPU test infrastructure
- Improve test reliability

### Medium Term (3-6 months)

- Performance benchmark suite
- Automated regression detection
- Chaos testing for resilience

### Long Term (6+ months)

- Fuzzing integration in CI
- Property-based testing expansion
- Formal verification tooling

---

## Pattern Evolution

### Test Framework Evolution

```text
v1 (deprecated): Mocha + manual assertions
v2 (current):    Hardhat + Foundry dual testing
v3 (emerging):   Adding property-based testing
```

### Mock Evolution

```text
v1 (deprecated): Manual mock implementations
v2 (current):    mockall/jest-mock-extended
v3 (stable):     Contract-based mocking
```

### Fixture Evolution

```text
v1 (deprecated): Hardcoded binary fixtures
v2 (current):    Versioned JSON fixtures
v3 (emerging):   Generated fixtures with seeds
```

---

## Test Reliability Focus

### Current Issues

| Issue              | Status    | Fix Approach              |
| ------------------ | --------- | ------------------------- |
| Timing flakiness   | Improving | Remove timing deps        |
| Resource leaks     | Improving | Better cleanup            |
| Order dependency   | Active    | Test isolation            |

### Reliability Metrics

Target: < 0.1% flaky test rate

Current tracking:

- CI failure analysis
- Flaky test quarantine
- Root cause tracking
