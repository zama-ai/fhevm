# Host Contracts ✅

**Location**: `/host-contracts/`
**Status**: Stable
**Purpose**: On-chain symbolic execution of FHE workflows on the host EVM chain

## Overview

Host contracts are deployed on each supported EVM chain and provide the core FHE execution interface. They execute operations symbolically (generating deterministic handles) while the actual encrypted computation happens off-chain.

## Key Contracts

| Contract | Purpose |
|----------|---------|
| `FHEVMExecutor.sol` | Symbolic execution engine with 20+ FHE operators |
| `ACL.sol` | Access control for encrypted data handles |
| `HCULimit.sol` | Enforces Homomorphic Complexity Unit limits per transaction |
| `KMSVerifier.sol` | Verifies KMS-signed decryption results |
| `InputVerifier.sol` | Verifies encrypted user input proofs (host-side) |

> **Note**: Both Gateway (`InputVerification.sol`) and Host (`InputVerifier.sol`) have input verification. Gateway handles cross-chain verification; Host handles local chain verification. In single-chain deployments, Host's InputVerifier is the primary entry point.

## Key Files

- `contracts/FHEVMExecutor.sol` - Core symbolic execution (fheAdd, fheMul, fheEq, etc.)
- `contracts/ACL.sol` - Permission management for ciphertext handles
- `contracts/shared/FheType.sol` - Encrypted type definitions

## Relationships

Host contracts receive calls from user smart contracts via the FHE library. They emit events that the Coprocessor's host-listener picks up. Results from the Coprocessor are verified via KMSVerifier before being accepted.

## FHEVMExecutor Operations

The FHEVMExecutor provides 20+ symbolic FHE operations:

**Arithmetic:** `fheAdd`, `fheSub`, `fheMul`, `fheDiv`, `fheRem`, `fheMin`, `fheMax`
**Comparison:** `fheEq`, `fheNe`, `fheLt`, `fheLe`, `fheGt`, `fheGe`
**Bitwise:** `fheBitAnd`, `fheBitOr`, `fheBitXor`, `fheNot`, `fheShl`, `fheShr`
**Special:** `fheRotl`, `fheRotr`, `fheSelect` (ternary)

Each operation:
1. Validates input handles and types
2. Generates deterministic output handle
3. Emits event for coprocessor
4. Returns handle immediately (symbolic)

## Areas for Deeper Documentation

**[TODO: FHEVMExecutor operators]** - Document all 20+ FHE operators (fheAdd, fheSub, fheMul, fheDiv, fheEq, fheLt, fheGt, fheBitAnd, fheBitOr, fheBitXor, fheShl, fheShr, fheRotl, fheRotr, etc.) and their symbolic execution semantics

**[TODO: ACL permission model]** - Detail the allowList, denyList, and delegation mechanisms for controlling access to encrypted values

**[TODO: HCU limit enforcement]** - Explain the 20M HCU/tx and 5M depth limits and how they prevent DoS attacks

**[TODO: Input verification flow]** - Document ZK proof verification for user-submitted encrypted inputs

---

**Related:**
- [Solidity Library](library-solidity.md) - Developer-facing API that calls Host Contracts
- [Gateway Contracts](gateway-contracts.md) - Receives Host Contract events
- [Coprocessor](coprocessor.md) - Listens to Host Contract events via host-listener
