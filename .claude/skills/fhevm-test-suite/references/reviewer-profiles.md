# Reviewer Profiles - Test Suite

Understanding reviewer preferences for test code.

---

## jatZama

**Focus**: Clarity, realistic scenarios, documentation

### What They Care About

- Clear, readable test code
- Realistic test scenarios
- Comprehensive test documentation
- Consistent naming conventions

### What Gets Quick Approval

- Well-structured test files
- Clear test names that describe behavior
- Comments explaining complex scenarios
- Realistic test data

### What Triggers Comments

- Unclear test purposes
- Unrealistic test scenarios
- Missing documentation for complex tests
- Inconsistent naming

### Interaction Style

- Values test clarity highly
- Appreciates thorough documentation
- May ask for more realistic scenarios

---

## enitrat

**Focus**: Security coverage, edge cases

### What They Care About

- Security test coverage
- ACL edge case testing
- Attack scenario tests
- Negative test cases

### What Gets Quick Approval

- Tests for security-sensitive paths
- Explicit ACL verification tests
- Tests that verify attack prevention
- Edge case coverage

### What Triggers Comments

- Missing security tests
- Incomplete ACL test coverage
- No negative test cases
- Overlooked edge cases

### Interaction Style

- Thorough review of security tests
- May request additional edge cases
- Expects explicit security assertions

---

## rudy-6-4

**Focus**: Robustness, reliability

### What They Care About

- Test reliability (no flakiness)
- Proper timeout handling
- Retry behavior verification
- Cleanup and teardown

### What Gets Quick Approval

- Deterministic tests
- Proper cleanup in afterEach
- Timeout configurations
- Tests for retry behavior

### What Triggers Comments

- Flaky test patterns
- Missing cleanup
- Timing-dependent assertions
- Race conditions

### Interaction Style

- Focuses on operational aspects
- Quick to identify flakiness patterns
- Values defensive test design

---

## Common Expectations

### All Reviewers Expect

1. Clear test names describing behavior
2. Proper setup and teardown
3. No hardcoded magic numbers without comments
4. Deterministic, reproducible tests
5. Both positive and negative test cases

### Test Structure Preferences

```typescript
// Preferred: Describe the behavior
it("should return zero when balance is insufficient", async function () {});

// Avoid: Technical description
it("test insufficient balance path", async function () {});
```

### Documentation Preferences

```typescript
/**
 * Tests that encrypted transfer respects ACL permissions.
 *
 * Setup:
 * - Alice owns 100 encrypted tokens
 * - Bob has no permission to transfer Alice's tokens
 *
 * Expected:
 * - Bob's transfer attempt reverts with UnauthorizedAccess
 */
it("should revert when caller lacks transfer permission", async function () {
  // ...
});
```
