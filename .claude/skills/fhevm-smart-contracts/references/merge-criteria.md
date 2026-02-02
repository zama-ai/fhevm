# Merge Criteria - Smart Contracts

What gets PRs merged with ZERO review comments in the Solidity domain.

---

## PR Size Guidelines

Based on 289/300 PRs approved on first try (96% rate):

| Lines Changed | Approval Rate | Recommendation           |
| ------------- | ------------- | ------------------------- |
| 10-50         | 98%           | Optimal - highest chance  |
| 51-100        | 95%           | Safe zone                 |
| 101-200       | 85%           | Add extra context         |
| 200+          | 70%           | Consider splitting        |

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
Scopes: library-solidity, host-contracts, gateway-contracts, protocol-contracts
```

Examples:

- `feat(library-solidity): add FHE.rotl and FHE.rotr operations`
- `fix(host-contracts): validate ACL permissions before decrypt`
- `refactor(gateway-contracts): extract signature verification`

### 3. Custom Errors (Not require Strings)

```solidity
// CORRECT
error InsufficientBalance(uint256 available, uint256 required);
error UnauthorizedAccess(address caller);

function transfer(address to, uint256 amount) external {
    if (balances[msg.sender] < amount) {
        revert InsufficientBalance(balances[msg.sender], amount);
    }
}

// WRONG - triggers review comment
function transfer(address to, uint256 amount) external {
    require(balances[msg.sender] >= amount, "Insufficient balance");
}
```

### 4. ACL Permission Management

Every encrypted value stored MUST have permissions set:

```solidity
function updateBalance(euint64 newBalance) internal {
    balances[msg.sender] = newBalance;

    // REQUIRED: Set permissions after storing
    FHE.allowThis(newBalance);           // Contract can use it
    FHE.allow(newBalance, msg.sender);   // User can access
}
```

### 5. Test Coverage

| Change Type           | Required Tests                    |
| --------------------- | --------------------------------- |
| New feature           | Hardhat + Foundry tests           |
| Bug fix               | Regression test                   |
| ACL changes           | Explicit permission tests         |
| Upgrade logic         | Upgrade simulation tests          |

---

## What Reviewers Check

### jatZama

- Architecture alignment with existing contracts
- NatSpec documentation with realistic examples
- Consistent naming conventions

### enitrat

- ACL/permission logic correctness
- Security implications of changes
- Access control patterns

### eudelins-zama

- Clean, simple implementations
- Pragmatic solutions
- Tech debt documented in issues

---

## Fast-Track Approval

PRs that get approved immediately:

1. Single-scope changes (<50 lines)
2. Clear issue reference
3. Tests for ACL paths included
4. Custom errors used
5. Follows existing patterns

---

## Review Blockers

PRs that trigger CHANGES_REQUESTED:

1. `require` with string messages instead of custom errors
2. Missing ACL permissions after encrypted store
3. Using `if` on encrypted bool (must use `FHE.select`)
4. Missing tests for ACL logic
5. Breaking upgrade compatibility without migration

---

## Solidity Style Requirements

### File Header

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
```

### Import Order

1. OpenZeppelin contracts
2. FHEVM library imports
3. Local/project imports

```solidity
import "@openzeppelin/contracts/access/Ownable2Step.sol";
import "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";

import "@fhevm/solidity/lib/FHE.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";

import "./interfaces/IMyContract.sol";
```

### NatSpec Documentation

```solidity
/// @notice Transfer encrypted tokens to recipient
/// @dev Uses FHE.select for conditional transfer
/// @param to Recipient address
/// @param encryptedAmount Encrypted transfer amount
/// @param inputProof ZK proof validating encryption
/// @return success Whether the transfer succeeded
function transfer(
    address to,
    externalEuint64 encryptedAmount,
    bytes calldata inputProof
) external returns (bool success) {
```
