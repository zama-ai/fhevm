---
name: fhevm-kms-gateway
description: |
  Develop KMS connector and gateway components for fhevm decryption flows.
  Covers kms-connector Rust code and gateway-contracts Solidity.
  Use when working on key management, decryption, or threshold signatures.
allowed-tools: [Bash, Read, Write, Edit, Grep, Glob]
---

# FHEVM KMS & Gateway Development

Build and maintain the Key Management Service connector and decryption gateway
for fhevm. This skill bridges Rust backend (kms-connector) and Solidity
contracts (gateway-contracts).

## Scope

- `kms-connector/` - Rust KMS integration layer
- `gateway-contracts/` - Solidity decryption gateway
- Cross-cutting: decryption flows, threshold signatures, ACL verification

## Commit Format

```text
type(scope): description

Scopes: kms-connector, gateway-contracts
Types: feat, fix, chore, ci, refactor, docs, test
```

## Merge Criteria

See [references/merge-criteria.md](references/merge-criteria.md) for detailed
requirements. Key points:

1. **PR Size**: 10-50 lines optimal, <100 lines safe
2. **Issue Linking**: Always include `Closes #XXX`
3. **Security Focus**: ACL verification must be explicit
4. **Cross-Component**: Changes often span Rust and Solidity
5. **Tests**: E2E tests for decryption flows

## Reviewer Focus Areas

See [references/reviewer-profiles.md](references/reviewer-profiles.md).

- **enitrat**: Security, ACL logic, threshold signature correctness
- **jatZama**: Architecture alignment, naming
- **rudy-6-4**: Retry/robustness in kms-connector

## Anti-Patterns to Avoid

See [references/anti-patterns.md](references/anti-patterns.md).

- Never skip ACL checks before decryption
- Avoid synchronous blocking in async contexts
- Do not expose key material in logs or errors

## Architecture

See [references/architecture.md](references/architecture.md).

## Best Practices

See [references/best-practices.md](references/best-practices.md).

- Use AWS KMS client with proper retry configuration
- Implement circuit breakers for KMS calls
- Log decryption requests (without values) for audit

## Current Trajectory

See [references/trajectory.md](references/trajectory.md).

**Growing**: Production hardening, multi-region support
**Stable**: Core decryption flow, threshold signature verification
**Active Migration**: Oracle to Gateway naming (95% complete)

## Upgrade Priorities

See [references/upgrade-priorities.md](references/upgrade-priorities.md).

Focus on production hardening and observability improvements.
