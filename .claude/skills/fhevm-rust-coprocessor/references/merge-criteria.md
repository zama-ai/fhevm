# Merge Criteria - Rust Coprocessor

What gets PRs merged with ZERO review comments in the coprocessor domain.

---

## PR Size Guidelines

Based on 289/300 PRs approved on first try (96% rate):

| Lines Changed | Approval Rate | Recommendation           |
| ------------- | ------------- | ------------------------ |
| 10-50         | 98%           | Optimal - highest chance |
| 51-100        | 95%           | Safe zone                |
| 101-200       | 85%           | Add extra context        |
| 200+          | 70%           | Consider splitting       |

---

## Required Elements

### 1. Issue Linking

Every PR must reference an issue:

```markdown
Closes #123
```

For multiple issues:

```markdown
Closes #123
Closes #124
```

### 2. Commit Message Format

```text
type(scope): description

Types: feat, fix, chore, ci, refactor, docs, test
Scopes: coprocessor, kms-connector, common
```

Examples:

- `feat(coprocessor): add GPU backend support for FHE operations`
- `fix(kms-connector): handle timeout in key retrieval`
- `refactor(common): extract retry logic to shared module`

### 3. Error Handling Consistency

Use the correct error category:

| Category        | Use For                         | Recovery Action       |
| --------------- | ------------------------------- | --------------------- |
| `Recoverable`   | Transient failures, timeouts    | Retry with backoff    |
| `Irrecoverable` | Invalid input, logic errors     | Fail fast, log, alert |
| `Fatal`         | Unrecoverable system state      | Shutdown gracefully   |

**Anti-pattern**: Mixing categories in the same error path.

### 4. Retry Patterns

All network/external calls must include:

```rust
// Required pattern
let backoff = ExponentialBackoff::builder()
    .with_max_retries(3)
    .with_jitter()
    .build();

retry(backoff, || async {
    // operation
}).await
```

### 5. Test Coverage

| Change Type           | Required Tests                    |
| --------------------- | --------------------------------- |
| New feature           | Unit + integration                |
| Bug fix               | Regression test for the bug       |
| Refactor              | Existing tests must pass          |
| Performance           | Benchmark comparison              |

---

## What Reviewers Check

### jatZama

- Architecture alignment with existing patterns
- Naming conventions (snake_case, descriptive)
- Realistic examples in docs/comments

### enitrat

- Security implications (especially ACL logic)
- Parallelization correctness
- Lock ordering and deadlock prevention

### rudy-6-4

- Retry/robustness patterns
- Graceful degradation
- Quick approvals if patterns are followed

---

## Fast-Track Approval

PRs that get approved immediately:

1. Single-scope changes (<50 lines)
2. Clear issue reference
3. Tests included
4. No security-sensitive changes
5. Follows existing patterns in codebase

---

## Review Blockers

PRs that trigger CHANGES_REQUESTED:

1. Missing issue reference
2. Inconsistent error handling categories
3. No retry logic for external calls
4. Large PRs without context
5. Breaking changes without migration path
