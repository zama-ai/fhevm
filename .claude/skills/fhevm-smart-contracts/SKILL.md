---
name: fhevm-smart-contracts
description: |
  Develop Solidity smart contracts for fhevm. Covers library-solidity,
  host-contracts, gateway-contracts, and protocol-contracts.
  Use when working on Solidity code, ACL logic, or contract upgrades.
allowed-tools: [Bash, Read, Write, Edit, Grep, Glob]
---

# FHEVM Smart Contracts Development

Build and maintain Solidity smart contracts for the fhevm ecosystem. This skill
covers the FHE.sol library, host contracts (ACL, Executor), gateway contracts,
and protocol contracts (tokens, staking).

## Scope

- `library-solidity/` - Core FHE.sol library
- `host-contracts/` - ACL, FHEVMExecutor, KMSVerifier
- `gateway-contracts/` - Decryption gateway, input verification
- `protocol-contracts/` - Token wrappers, governance, staking

## Commit Format

```text
type(scope): description

Scopes: library-solidity, host-contracts, gateway-contracts, protocol-contracts
Types: feat, fix, chore, ci, refactor, docs, test
```

## Merge Criteria

See [references/merge-criteria.md](references/merge-criteria.md) for detailed
requirements. Key points:

1. **PR Size**: 10-50 lines optimal, <100 lines safe
2. **Issue Linking**: Always include `Closes #XXX`
3. **ACL Logic**: Must be explicitly tested and documented
4. **Custom Errors**: Use custom errors over require strings
5. **Tests**: Hardhat AND Foundry tests for critical paths

## Reviewer Focus Areas

See [references/reviewer-profiles.md](references/reviewer-profiles.md).

- **jatZama**: Architecture, realistic examples in NatSpec
- **enitrat**: ACL/permission logic correctness, security
- **eudelins-zama**: Quick fixes, pragmatic solutions

## Anti-Patterns to Avoid

See [references/anti-patterns.md](references/anti-patterns.md).

- Never use `require` with string messages (use custom errors)
- Avoid `if` statements on encrypted values (use `FHE.select`)
- Do not forget ACL permissions after storing encrypted values

## Architecture

See [references/architecture.md](references/architecture.md).

## Best Practices

See [references/best-practices.md](references/best-practices.md).

- Use UUPS proxy pattern for upgradeable contracts
- Use OpenZeppelin 5.x patterns (AccessControl, Ownable2Step)
- Use FHE.allowThis() and FHE.allow() after every encrypted store

## Current Trajectory

See [references/trajectory.md](references/trajectory.md).

**Growing**: Protocol contracts (staking, wrappers), ERC7984 standard
**Stable**: library-solidity (FHE.sol), host-contracts
**Active Migration**: ERC7984 token wrappers (50% complete)

## Upgrade Priorities

See [references/upgrade-priorities.md](references/upgrade-priorities.md).

OpenZeppelin 5.x is current and healthy. Focus on ERC7984 adoption.
