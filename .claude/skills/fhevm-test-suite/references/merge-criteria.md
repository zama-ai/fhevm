# Merge Criteria - Test Suite

What gets PRs merged with ZERO review comments in the test suite domain.

---

## PR Size Guidelines

Test PRs can be larger than implementation PRs:

| Lines Changed | Approval Rate | Recommendation           |
| ------------- | ------------- | ------------------------- |
| 10-100        | 98%           | Optimal                   |
| 101-200       | 90%           | Acceptable for tests      |
| 201-300       | 75%           | Split if possible         |
| 300+          | 50%           | Must justify              |

---

## Required Elements

### 1. Issue Linking

Every PR must reference an issue:

```markdown
Closes #123
```

### 2. Commit Message Format

```text
type(scope): description

Types: feat, fix, chore, test, refactor
Scopes: test-suite
```

Examples:

- `test(test-suite): add E2E tests for decryption flow`
- `fix(test-suite): resolve flaky timeout in transfer test`
- `chore(test-suite): update test fixtures for new format`

### 3. Test Coverage Requirements

| Test Type     | Required Coverage                       |
| ------------- | --------------------------------------- |
| Happy path    | Always required                         |
| Error cases   | Required for security-sensitive code    |
| Edge cases    | Required for complex logic              |
| Fuzzing       | Recommended for crypto/math functions   |

### 4. Determinism Requirements

Tests must be deterministic:

- No race conditions
- No timing-dependent assertions
- No external service dependencies without mocks
- Reproducible random seeds

### 5. Documentation

Complex tests need comments:

```typescript
/**
 * Tests that a transfer with insufficient balance results in zero transfer.
 *
 * Scenario:
 * 1. Alice has 100 tokens
 * 2. Alice attempts to transfer 200 tokens to Bob
 * 3. Transfer succeeds but amount is 0 (FHE.select behavior)
 * 4. Both balances unchanged
 */
it("should handle insufficient balance gracefully", async function () {
  // ...
});
```

---

## What Reviewers Check

### jatZama

- Test clarity and readability
- Realistic test scenarios
- Documentation quality

### enitrat

- Security test coverage
- ACL edge cases
- Attack scenario tests

### rudy-6-4

- Test robustness
- Timeout handling
- Retry behavior tests

---

## Fast-Track Approval

Test PRs that get approved immediately:

1. Adds coverage for existing feature
2. Clear test names and structure
3. No flaky patterns
4. Proper cleanup/teardown

---

## Review Blockers

Test PRs that trigger CHANGES_REQUESTED:

1. Flaky tests (timing-dependent, race conditions)
2. Missing error case coverage
3. Hardcoded values without explanation
4. No cleanup in teardown
5. External dependencies without mocks

---

## Test Naming Conventions

### Rust

```rust
#[test]
fn test_transfer_with_sufficient_balance_succeeds() { }

#[test]
fn test_transfer_with_insufficient_balance_returns_zero() { }

#[tokio::test]
async fn test_decryption_with_invalid_acl_fails() { }
```

### TypeScript/Hardhat

```typescript
describe("ConfidentialERC20", function () {
  describe("transfer", function () {
    it("should transfer tokens with sufficient balance", async function () {});
    it("should return zero transfer amount with insufficient balance", async function () {});
    it("should revert if sender is not allowed", async function () {});
  });
});
```

### Foundry

```solidity
function test_Transfer_WithSufficientBalance_Succeeds() public { }
function test_Transfer_WithInsufficientBalance_ReturnsZero() public { }
function testFuzz_Transfer_ArbitraryAmount(uint64 amount) public { }
```

---

## Fixture Management

### Version Tracking

```typescript
// test/fixtures/v2/encrypted_balance.json
// Always include version in fixture path
const fixture = require("./fixtures/v2/encrypted_balance.json");
```

### Generated Fixtures

```rust
// Regenerate fixtures on format changes
fn generate_test_fixture() -> Vec<u8> {
    // Generate deterministically
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    generate_ciphertext(&mut rng)
}
```

---

## E2E Test Standards

### Test Structure

```typescript
describe("E2E: Confidential Transfer Flow", function () {
  let contract: ConfidentialERC20;
  let alice: SignerWithAddress;
  let bob: SignerWithAddress;

  before(async function () {
    // One-time setup
    [alice, bob] = await ethers.getSigners();
  });

  beforeEach(async function () {
    // Per-test setup
    contract = await deployFreshContract();
  });

  afterEach(async function () {
    // Cleanup
    await cleanupTestState();
  });

  it("should complete full transfer flow", async function () {
    // Test implementation
  });
});
```

### Timeout Configuration

```typescript
describe("Slow operations", function () {
  this.timeout(60000); // 60 seconds for E2E tests

  it("should complete decryption flow", async function () {
    // Long-running test
  });
});
```
