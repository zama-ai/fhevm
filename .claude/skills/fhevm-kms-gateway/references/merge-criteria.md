# Merge Criteria - KMS & Gateway

What gets PRs merged with ZERO review comments in the KMS/Gateway domain.

---

## PR Size Guidelines

| Lines Changed | Approval Rate | Recommendation           |
| ------------- | ------------- | ------------------------- |
| 10-50         | 98%           | Optimal                   |
| 51-100        | 95%           | Safe zone                 |
| 101-200       | 80%           | Extra context needed      |
| 200+          | 65%           | Consider splitting        |

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

Types: feat, fix, chore, ci, refactor, docs, test
Scopes: kms-connector, gateway-contracts
```

Examples:

- `feat(kms-connector): add circuit breaker for KMS calls`
- `fix(gateway-contracts): validate ACL before decryption callback`
- `refactor(kms-connector): extract signature verification`

### 3. Security Requirements

KMS/Gateway is security-critical. Every PR must:

| Requirement              | Implementation                          |
| ------------------------ | --------------------------------------- |
| ACL verification         | Explicit check before decryption        |
| Key material protection  | Never log or expose in errors           |
| Audit logging            | Log requests without sensitive data     |
| Error handling           | Fail secure (deny on error)             |

### 4. Cross-Component Changes

Many changes span Rust and Solidity. For these:

- Include both components in PR
- Test E2E flow
- Document interaction in PR description

### 5. Test Requirements

| Change Type           | Required Tests                    |
| --------------------- | --------------------------------- |
| New feature           | Unit + E2E tests                  |
| Security fix          | Negative tests (attack prevented) |
| Rust changes          | Unit tests in Rust                |
| Solidity changes      | Hardhat + Foundry tests           |
| Flow changes          | E2E test-suite tests              |

---

## What Reviewers Check

### enitrat

- ACL verification correctness
- Threshold signature security
- Key material handling

### jatZama

- Architecture alignment
- Naming consistency
- Documentation quality

### rudy-6-4

- Retry patterns in kms-connector
- Graceful failure handling
- Circuit breaker implementation

---

## Fast-Track Approval

PRs that get approved immediately:

1. Single-scope changes (<50 lines)
2. Clear security implications documented
3. E2E tests included for flow changes
4. ACL checks explicit and tested

---

## Review Blockers

PRs that trigger CHANGES_REQUESTED:

1. Missing ACL verification
2. Key material in logs or error messages
3. Missing E2E tests for flow changes
4. Inconsistent error handling across components
5. Breaking changes without migration path

---

## Decryption Flow Checklist

For any change to decryption flow:

- [ ] ACL check happens before decryption
- [ ] Invalid ACL results in explicit rejection
- [ ] Key material never exposed in errors
- [ ] Audit log entry created
- [ ] Threshold signature verified
- [ ] Callback executed only on success
- [ ] E2E test covers happy path
- [ ] E2E test covers ACL rejection

---

## Error Handling Standards

### Rust (kms-connector)

```rust
// Use Recoverable for transient failures
if kms_call().is_err() {
    return Err(Error::Recoverable(KmsTimeout));
}

// Use Irrecoverable for security failures
if !verify_acl(handle, requester) {
    return Err(Error::Irrecoverable(AclDenied { handle, requester }));
}
```

### Solidity (gateway-contracts)

```solidity
// Custom errors for security failures
error UnauthorizedDecryption(bytes32 handle, address requester);
error InvalidSignature(bytes signature);

function requestDecryption(bytes32 handle) external {
    if (!isAllowed(handle, msg.sender)) {
        revert UnauthorizedDecryption(handle, msg.sender);
    }
}
```
