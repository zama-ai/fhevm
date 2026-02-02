# Upgrade Priorities - Test Suite

Current status and recommended improvements for testing infrastructure.

---

## Test Framework Status

### Healthy

| Framework         | Version | Status   | Notes                        |
| ----------------- | ------- | -------- | ---------------------------- |
| Hardhat           | 2.x     | Current  | Primary Solidity testing     |
| Foundry           | Latest  | Current  | Fuzzing and unit tests       |
| Jest              | 29.x    | Current  | TypeScript testing           |
| Tokio test        | 1.x     | Current  | Rust async testing           |

### Needs Attention

| Tool              | Current | Recommended | Priority |
| ----------------- | ------- | ----------- | -------- |
| proptest          | 1.2     | 1.4         | Low      |
| criterion         | 0.4     | 0.5         | Medium   |

---

## Reliability Priorities

### Priority 1: Eliminate Flaky Tests

Current flaky test rate: ~2%
Target: <0.1%

Actions:

1. Audit all timing-dependent tests
2. Replace sleep with explicit waits
3. Ensure test isolation
4. Add retry detection in CI

```yaml
# CI: Detect flaky tests
- name: Run tests with retry detection
  run: |
    for i in 1 2 3; do
      npm test && break || echo "Attempt $i failed"
    done
```

### Priority 2: Improve Test Isolation

Current issues:

- Shared database state
- Global mocks
- Order-dependent tests

Actions:

```typescript
// Before each test: fresh state
beforeEach(async function () {
  // Fresh contract deployment
  contract = await deployFreshContract();

  // Fresh database
  await db.clear();

  // Reset mocks
  jest.clearAllMocks();
});
```

### Priority 3: Speed Up E2E Tests

Current E2E suite time: ~15 minutes
Target: <5 minutes

Actions:

1. Parallel test execution
2. Contract deployment caching
3. Mock mode for non-critical paths

```typescript
// Use mock mode for faster tests
describe("Fast E2E", function () {
  before(async function () {
    await fhevm.enableMockMode();
  });
});
```

---

## Coverage Priorities

### Current Coverage

| Component         | Line Coverage | Branch Coverage |
| ----------------- | ------------- | --------------- |
| library-solidity  | 85%           | 75%             |
| host-contracts    | 90%           | 80%             |
| coprocessor       | 80%           | 70%             |
| kms-connector     | 75%           | 65%             |
| test-suite (util) | 60%           | 50%             |

### Target Coverage

| Component         | Line Coverage | Branch Coverage |
| ----------------- | ------------- | --------------- |
| All               | 90%           | 85%             |

### Coverage Gaps to Address

1. **ACL edge cases**: Missing tests for permission revocation
2. **Error paths**: Insufficient negative test coverage
3. **Concurrent operations**: Race condition testing
4. **Recovery scenarios**: Failure recovery tests

---

## Infrastructure Upgrades

### GPU Testing Infrastructure

**Status**: In development

Requirements:

- Self-hosted runners with NVIDIA GPUs
- CUDA toolkit installation
- GPU-specific test matrix

```yaml
# CI configuration for GPU tests
test-gpu:
  runs-on: [self-hosted, gpu]
  steps:
    - name: Run GPU tests
      run: cargo test --features gpu
```

### Performance Benchmarking

**Status**: Partial

Setup needed:

- Criterion benchmarks for Rust
- Gas benchmarks for Solidity
- Regression detection

```rust
// Add to CI
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_fhe_operations(c: &mut Criterion) {
    c.bench_function("fhe_add_64", |b| {
        b.iter(|| fhe_add(a, b))
    });
}
```

---

## Tool Upgrades

### Recommended Additions

| Tool              | Purpose                      | Priority |
| ----------------- | ---------------------------- | -------- |
| insta             | Snapshot testing             | Medium   |
| cargo-mutants     | Mutation testing             | Low      |
| slither           | Security static analysis     | High     |

### Slither Integration

```yaml
- name: Run Slither
  uses: crytic/slither-action@v0.3.0
  with:
    target: "contracts/"
    slither-args: "--checklist"
```

---

## Test Data Management

### Fixture Upgrade Plan

1. **Audit existing fixtures**: Identify v1 fixtures
2. **Migrate to v2 format**: Add versioning and metadata
3. **Add generation scripts**: Reproducible fixture creation
4. **Document format**: Clear fixture documentation

```typescript
// Fixture generation script
async function generateFixture(seed: number): Promise<Fixture> {
  const rng = seedRandom(seed);
  return {
    version: 2,
    seed: seed,
    data: await generateTestData(rng),
    generatedAt: new Date().toISOString(),
  };
}
```

---

## CI Integration Improvements

### Current Pipeline

```text
PR -> Lint -> Unit Tests -> Integration Tests -> (Manual) E2E
```

### Target Pipeline

```text
PR -> Lint -> Unit Tests -> Integration Tests -> E2E Tests -> Coverage Report
         \-> Security Scan
         \-> Benchmark (if performance-related)
```

### Matrix Expansion

```yaml
strategy:
  matrix:
    os: [ubuntu-latest]
    rust: [stable, beta]
    node: [18, 20]
    solidity: [0.8.24]
    feature: [default, gpu]
```

---

## Documentation Priorities

### Needed Documentation

- [ ] Test writing guide for new contributors
- [ ] Fixture format specification
- [ ] Mock service documentation
- [ ] E2E test scenario catalog

### Example: Test Writing Guide

```markdown
# Writing Tests for fhevm

## Unit Tests

1. Place in component's `tests/` directory
2. Name: `test_{module}_{function}_{scenario}`
3. Use arrange-act-assert pattern
4. Include positive and negative cases
```
