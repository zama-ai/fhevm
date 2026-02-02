# Upgrade Priorities - Smart Contracts

Dependency health status and recommended actions for Solidity contracts.

---

## Healthy Dependencies

All major Solidity dependencies are current:

| Dependency        | Version | Status   | Notes                        |
| ----------------- | ------- | -------- | ---------------------------- |
| OpenZeppelin      | 5.x     | Current  | Access control, upgrades     |
| Solidity          | 0.8.24  | Current  | Custom errors, user types    |
| Hardhat           | 2.x     | Current  | Development framework        |
| Foundry           | Latest  | Current  | Testing, fuzzing             |

---

## Standard Adoption Priorities

### Priority 1: ERC7984 Implementation

**Current**: Custom confidential token implementations
**Target**: ERC7984-compliant interfaces
**Status**: 50% complete

ERC7984 provides standardized interface for confidential tokens:

```solidity
// Implement in new token contracts
interface IERC7984 {
    function confidentialTransfer(
        address to,
        externalEuint256 amount,
        bytes calldata proof
    ) external returns (bool);
}
```

### Priority 2: Custom Error Migration

**Current**: Mix of require strings and custom errors
**Target**: 100% custom errors
**Status**: Ongoing

All new code must use custom errors:

```solidity
error InsufficientBalance(uint256 available, uint256 required);
error UnauthorizedAccess(address caller);
```

### Priority 3: Ownable2Step Adoption

**Current**: Some contracts use Ownable
**Target**: All use Ownable2Step
**Status**: Mostly complete

```solidity
import "@openzeppelin/contracts/access/Ownable2Step.sol";

contract MyContract is Ownable2Step {
    constructor() Ownable(msg.sender) {}
}
```

---

## Testing Infrastructure

### Foundry Integration

Ensure all contracts have Foundry tests alongside Hardhat:

```bash
# Run both test suites
npx hardhat test
forge test
```

### Fuzzing Coverage

Critical functions should have fuzz tests:

```solidity
function testFuzz_Transfer(address to, uint64 amount) public {
    vm.assume(to != address(0));
    vm.assume(amount <= type(uint64).max);
    // ...
}
```

---

## Security Considerations

### Audit Readiness

Before audit, ensure:

- [ ] All public functions have NatSpec
- [ ] Custom errors used throughout
- [ ] Access control properly implemented
- [ ] Storage layout documented
- [ ] Upgrade path tested

### Known Patterns to Review

1. **ACL Permission Flow**: Verify all encrypted values have permissions set
2. **Upgrade Authorization**: Ensure _authorizeUpgrade properly restricted
3. **Input Validation**: All external inputs validated
4. **Reentrancy**: Guards in place for external calls

---

## Tooling Upgrades

### Recommended Hardhat Plugins

```javascript
// hardhat.config.js
require("@nomicfoundation/hardhat-toolbox");
require("@nomicfoundation/hardhat-verify");
require("hardhat-gas-reporter");
require("hardhat-contract-sizer");
```

### Foundry Configuration

```toml
# foundry.toml
[profile.default]
src = 'contracts'
out = 'out'
libs = ['node_modules', 'lib']
optimizer = true
optimizer_runs = 200
```

---

## Dependency Audit Commands

```bash
# Check for known vulnerabilities
npm audit

# Check OpenZeppelin version
npm list @openzeppelin/contracts

# Verify Solidity version
npx hardhat compile --show-stack-traces

# Run Slither static analysis
slither .
```

---

## Version Pinning Policy

| Category        | Policy                                   |
| --------------- | ---------------------------------------- |
| OpenZeppelin    | Pin major version (^5.0.0)               |
| Solidity        | Pin exact version (=0.8.24)              |
| Hardhat plugins | Allow minor updates (^2.x)               |
| Dev deps        | Allow patch updates                      |

---

## Migration Path for Existing Contracts

### Step 1: Add Custom Errors

```solidity
// Add error definitions
error InsufficientBalance();
error UnauthorizedAccess();

// Replace require statements
// Before: require(balance >= amount, "Insufficient");
// After:
if (balance < amount) revert InsufficientBalance();
```

### Step 2: Upgrade Access Control

```solidity
// Change inheritance
// Before: is Ownable
// After:
contract MyContract is Ownable2Step {
```

### Step 3: Add ERC7984 Compliance

```solidity
// Implement interface
contract MyToken is IERC7984, ConfidentialERC20 {
    function confidentialTransfer(...) external returns (bool) {
        return _confidentialTransfer(...);
    }
}
```
