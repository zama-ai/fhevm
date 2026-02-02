# Trajectory - Smart Contracts

Where the Solidity contracts are heading based on git history analysis.

---

## Growing Areas

### Protocol Contracts (Active Development)

**Status**: Rapid growth in staking and wrappers

Recent activity shows expansion of protocol-contracts:

- Staking mechanisms for protocol participation
- ERC7984 wrapper implementations
- Governance structures

**Implication**: New token contracts should follow ERC7984 patterns:

```solidity
// Emerging pattern for confidential tokens
contract ConfidentialToken is IERC7984, ZamaEthereumConfig {
    // Standard ERC7984 interface for fhe tokens
}
```

### ERC7984 Standard Adoption (50% Complete)

Migration to standardized confidential token interface:

- Provides interoperability with other FHE implementations
- Standard wrapper pattern for existing tokens
- Clear interface definitions

**Implication**: New token implementations should implement ERC7984:

```solidity
interface IERC7984 {
    function confidentialTransfer(
        address to,
        externalEuint256 amount,
        bytes calldata proof
    ) external returns (bool);

    function confidentialBalanceOf(address account)
        external
        view
        returns (euint256);
}
```

---

## Stable Areas

### library-solidity (FHE.sol)

The core FHE library is mature and stable:

- Well-tested operation implementations
- Stable type definitions
- Comprehensive ACL management

**Implication**: Follow existing patterns exactly; deviations need strong justification.

### host-contracts

Infrastructure contracts are stable:

- ACL.sol - permission registry
- FHEVMExecutor.sol - operation coordinator
- KMSVerifier.sol - signature verification

**Implication**: Changes require extensive testing and security review.

### Hardhat Tooling

Testing infrastructure is mature:

- fhevmjs integration well-established
- Test helpers comprehensive
- Mock mode for fast iteration

---

## Active Migrations

### Oracle to Gateway (95% Complete)

Terminology migration:

| Old               | New               |
| ----------------- | ----------------- |
| `OracleContract`  | `GatewayContract` |
| `oracle-worker`   | `gateway-worker`  |
| `requestOracle()` | `requestGateway()`|

**Implication**: Use "gateway" terminology in all new code.

### Custom Errors Adoption (Ongoing)

Converting from require strings to custom errors:

```solidity
// Old (being removed)
require(amount > 0, "Amount must be positive");

// New (use this)
error InvalidAmount();
if (amount == 0) revert InvalidAmount();
```

---

## Declining Areas

### Plain require Statements

Moving away from string-based reverts:

- Custom errors are gas-efficient
- Better for error handling
- Required for new code

### Legacy Token Interfaces

Old confidential token patterns being deprecated:

- Moving to ERC7984 standard
- Legacy wrappers maintained for compatibility
- New implementations must use standard

---

## Upcoming Changes

Based on open issues and recent discussions:

### Short Term (1-3 months)

- Complete ERC7984 wrapper implementations
- Add staking contract tests
- Improve gas efficiency in FHE operations

### Medium Term (3-6 months)

- Governance contract implementations
- Cross-chain bridge contracts
- Enhanced ACL patterns

### Long Term (6+ months)

- Layer 2 deployment support
- Alternative proof systems
- Extended FHE operation set

---

## Pattern Evolution

### Token Pattern Evolution

```text
v1 (deprecated): Custom implementation per token
v2 (current):    ConfidentialERC20 base contract
v3 (emerging):   ERC7984 standard compliance
```

### Error Handling Evolution

```text
v1 (deprecated): require("message")
v2 (current):    Custom errors
v3 (stable):     Custom errors with parameters
```

### Access Control Evolution

```text
v1 (deprecated): onlyOwner modifier
v2 (current):    Ownable2Step
v3 (stable):     AccessControl for complex permissions
```

---

## Migration Checklist

When updating existing contracts:

- [ ] Replace require strings with custom errors
- [ ] Use Ownable2Step instead of Ownable
- [ ] Implement ERC7984 for new tokens
- [ ] Use "gateway" terminology
- [ ] Add storage gaps for upgradeability
- [ ] Include Foundry tests alongside Hardhat
