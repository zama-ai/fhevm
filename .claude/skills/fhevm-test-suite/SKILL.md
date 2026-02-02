---
name: fhevm-test-suite
description: |
  Develop and maintain the fhevm test suite including E2E tests,
  integration tests, and testing infrastructure.
  Use when working on test-suite/, adding tests, or debugging test failures.
allowed-tools: [Bash, Read, Write, Edit, Grep, Glob]
---

# FHEVM Test Suite Development

Build and maintain comprehensive tests for the fhevm ecosystem. This skill
covers E2E testing, integration tests, and testing infrastructure across both
Rust and Solidity components.

## Scope

- `test-suite/` - End-to-end testing
- Tests within component directories
- Testing utilities and fixtures
- Mock services and test infrastructure

## Commit Format

```text
type(scope): description

Scopes: test-suite
Types: feat, fix, chore, test, refactor
```

## Merge Criteria

See [references/merge-criteria.md](references/merge-criteria.md) for detailed
requirements. Key points:

1. **PR Size**: Test PRs can be larger (up to 200 lines acceptable)
2. **Issue Linking**: Always include `Closes #XXX`
3. **Coverage**: Tests must cover happy path AND error cases
4. **Determinism**: Tests must be reproducible, no flakiness
5. **Documentation**: Complex test scenarios need comments

## Reviewer Focus Areas

See [references/reviewer-profiles.md](references/reviewer-profiles.md).

- **jatZama**: Test clarity, realistic scenarios
- **enitrat**: Security test coverage, edge cases
- **rudy-6-4**: Robustness, timeout handling

## Anti-Patterns to Avoid

See [references/anti-patterns.md](references/anti-patterns.md).

- Avoid flaky tests (race conditions, timing dependencies)
- Never hardcode test fixtures without version tracking
- Do not skip cleanup in test teardown

## Architecture

See [references/architecture.md](references/architecture.md).

## Best Practices

See [references/best-practices.md](references/best-practices.md).

- Use dual Hardhat + Foundry testing for Solidity
- Include fuzzing for critical functions
- Mock external services deterministically

## Current Trajectory

See [references/trajectory.md](references/trajectory.md).

**Growing**: E2E test coverage, GPU test support
**Stable**: Unit test patterns, Hardhat integration
**Active**: Performance benchmarking infrastructure

## Upgrade Priorities

See [references/upgrade-priorities.md](references/upgrade-priorities.md).

Focus on test reliability and GPU test infrastructure.
