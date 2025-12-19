# Protocol Contracts 🔥

**Location**: `/protocol-contracts/`
**Status**: Active Development
**Purpose**: Protocol-level infrastructure including token, staking, and governance

## Overview

Protocol contracts implement the economic and governance layer of the FHEVM ecosystem. They are organized into domain-specific subdirectories.

## Submodules

| Directory | Purpose |
|-----------|---------|
| `token/` | ZAMA ERC20 token and OFT (Omnichain Fungible Token) for cross-chain |
| `staking/` | Node operator staking mechanisms |
| `governance/` | DAO voting and protocol governance |
| `confidential-wrapper/` | Wraps public tokens for confidential transfers |
| `feesBurner/` | Fee collection and token burning |
| `safe/` | Safe module for protocol administration |

## Key Contracts

**Token Layer:**
- `token/ZamaERC20.sol` - Protocol token implementation
- `token/ZamaOFT.sol` - Omnichain token (LayerZero integration)

**Economic Layer:**
- `staking/OperatorStaking.sol` - Operator staking and slashing
- `staking/Rewarder.sol` - Reward distribution mechanisms
- `confidential-wrapper/Wrapper.sol` - Public-to-confidential token bridge
- `feesBurner/FeesBurner.sol` - Fee collection and burning

**Governance Layer:**
- `governance/` - DAO voting contracts

## Key Files

- `token/ZamaERC20.sol` - Protocol token
- `confidential-wrapper/Wrapper.sol` - Public-to-confidential token bridge
- `staking/OperatorStaking.sol` - Operator staking

## Recent Development Focus (Dec 2025)

- **Staking/delegation**: `OperatorStaking` and `Rewarder` implementation
- **Fee management**: Burner implementation and distribution logic
- **Governance**: Safe ownership and admin modules
- **ERC1363 integration**: Callback-based token transfers
- **UUPS upgradeability**: Proxy patterns for contract upgrades

## Deprecation Notice

> ⚠️ **Note**: `ProtocolOperatorRegistry` has been removed. Use `OperatorStaking` for staking functionality.

## Areas for Deeper Documentation

**[TODO: Confidential wrapper pattern]** - Document how public ERC20 tokens are wrapped for confidential use and unwrapped back. Explain the bridge security model and liquidity management.

**[TODO: Staking/delegation contracts]** - Detail `OperatorStaking` and `Rewarder` implementations. Explain staking mechanics, slashing conditions, reward calculation, and delegation patterns.

**[TODO: Governance mechanisms]** - Document DAO voting contracts, proposal lifecycle, and Safe integration for admin operations.

**[TODO: Fee economics]** - Explain fee collection, distribution to operators, and burning mechanisms. Detail the economic incentives.

**[TODO: Cross-chain token flows]** - Document LayerZero OFT integration and cross-chain token transfers.

---

**Related:**
- [Gateway Contracts](gateway-contracts.md) - ProtocolPayment integrates with Gateway
- [Confidential wrapper example] - Using wrapped tokens in confidential contracts
- [Reference: Technology Stack](../reference/tech-stack.md) - ERC1363, UUPS patterns
