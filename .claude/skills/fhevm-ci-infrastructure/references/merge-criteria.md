# Merge Criteria - CI/Infrastructure

What gets PRs merged with ZERO review comments in CI/infrastructure domain.

---

## PR Size Guidelines

| Lines Changed | Approval Rate | Recommendation           |
| ------------- | ------------- | ------------------------- |
| 10-50         | 98%           | Optimal                   |
| 51-100        | 95%           | Safe zone                 |
| 101-200       | 80%           | Split if possible         |
| 200+          | 60%           | Must justify              |

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

Types: feat, fix, chore, ci, refactor, docs
Scopes: ci, charts
```

Examples:

- `ci: add GPU runner support for coprocessor tests`
- `fix(ci): correct artifact retention period`
- `chore(charts): bump image versions`

### 3. Workflow Testing

CI changes must be tested before merge:

```yaml
# Test in a branch first
on:
  pull_request:
    branches: [main]
  workflow_dispatch:  # Allow manual testing
```

### 4. Security Requirements

| Requirement              | Implementation                          |
| ------------------------ | --------------------------------------- |
| Secrets management       | Use GitHub secrets, never hardcode      |
| Permissions              | Minimal required permissions            |
| Dependencies             | Pin versions with SHA hashes            |
| Scanning                 | Include security scanning steps         |

### 5. Idempotency

All workflows must be safe to re-run:

```yaml
# GOOD: Idempotent
- name: Create directory
  run: mkdir -p ./output

# BAD: Not idempotent
- name: Create directory
  run: mkdir ./output  # Fails if exists
```

---

## What Reviewers Check

### jatZama

- Naming consistency across workflows
- Architecture alignment with existing patterns
- Clear job naming and descriptions

### rudy-6-4

- Retry patterns for flaky operations
- Timeout configurations
- Graceful failure handling

### eudelins-zama

- Simplicity of implementation
- Pragmatic solutions
- Tech debt documented

---

## Fast-Track Approval

PRs that get approved immediately:

1. Single workflow modification
2. Clear issue reference
3. Tested in branch
4. No secret changes
5. Follows existing patterns

---

## Review Blockers

PRs that trigger CHANGES_REQUESTED:

1. Hardcoded secrets or credentials
2. Missing timeout configurations
3. Using `latest` tags
4. Skipping security scans
5. Non-idempotent operations

---

## Workflow Standards

### Job Naming

```yaml
jobs:
  build-coprocessor:  # kebab-case
    name: Build Coprocessor  # Human readable
```

### Step Naming

```yaml
steps:
  - name: Checkout code
    uses: actions/checkout@v4

  - name: Setup Rust toolchain
    uses: dtolnay/rust-toolchain@stable
```

### Timeout Configuration

```yaml
jobs:
  test:
    timeout-minutes: 30  # Always set
    steps:
      - name: Run tests
        timeout-minutes: 20  # Per-step where needed
```

### Caching

```yaml
- name: Cache Cargo dependencies
  uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```
