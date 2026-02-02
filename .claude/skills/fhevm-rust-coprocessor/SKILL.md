---
name: fhevm-rust-coprocessor
description: |
  Develop Rust components for the fhevm coprocessor and kms-connector.
  Covers workers, scheduler, gRPC services, and TFHE-rs integration.
  Use when working on coprocessor/, kms-connector/, or fhevm-engine/.
allowed-tools: [Bash, Read, Write, Edit, Grep, Glob]
---

# FHEVM Rust Coprocessor Development

Build and maintain the Rust FHE computation engine powering fhevm. This skill
covers the coprocessor workers, scheduler, kms-connector, and gRPC services.

## Scope

- `coprocessor/` - FHE computation engine (workers, scheduler)
- `kms-connector/` - KMS integration layer
- `fhevm-engine/` - Shared engine components

## Commit Format

```text
type(scope): description

Scopes: coprocessor, kms-connector, common
Types: feat, fix, chore, ci, refactor, docs, test
```

## Merge Criteria

See [references/merge-criteria.md](references/merge-criteria.md) for detailed
requirements. Key points:

1. **PR Size**: 10-50 lines optimal, <100 lines safe
2. **Issue Linking**: Always include `Closes #XXX`
3. **Error Handling**: Use `Recoverable` vs `Irrecoverable` correctly
4. **Retry Logic**: Include exponential backoff with jitter
5. **Tests**: Required for implementation changes

## Reviewer Focus Areas

See [references/reviewer-profiles.md](references/reviewer-profiles.md).

- **jatZama**: Architecture decisions, naming conventions
- **enitrat**: Security, ACL logic, parallelization patterns
- **rudy-6-4**: Retry/robustness patterns, quick approvals

## Anti-Patterns to Avoid

See [references/anti-patterns.md](references/anti-patterns.md).

- Never mix `Recoverable` and `Irrecoverable` error handling
- Avoid unbounded retries without circuit breakers
- Do not ignore GPU backend feature flags in tests

## Architecture

See [references/architecture.md](references/architecture.md).

## Best Practices

See [references/best-practices.md](references/best-practices.md).

- Use `thiserror` for library errors, `anyhow` for applications
- Prefer `tokio::select!` over manual polling
- Use `sqlx` compile-time checked queries

## Current Trajectory

See [references/trajectory.md](references/trajectory.md).

**Growing**: GPU backends, parallelization, production hardening
**Stable**: Core TFHE-rs integration, gRPC interfaces
**Active Migration**: GPU enablement (60% complete)

## Upgrade Priorities

See [references/upgrade-priorities.md](references/upgrade-priorities.md).

**Critical**: Upgrade alloy to 1.4.1+ (DoS vulnerability in 1.0.38-1.1.2)
