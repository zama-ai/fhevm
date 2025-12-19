# Solidity Library ✅

**Location**: `/library-solidity/`
**Status**: Stable
**Purpose**: Developer-facing FHE primitives for writing confidential smart contracts

## Overview

The Solidity library provides the API that smart contract developers use to work with encrypted types. It abstracts away the complexity of FHE operations behind familiar Solidity syntax.

## Key Components

| File | Purpose |
|------|---------|
| `lib/FHE.sol` | Main developer API - import this to use FHE |
| `lib/Impl.sol` | Implementation details, delegates to precompiles |
| `lib/FheType.sol` | Encrypted type enum definitions |

## Encrypted Types

**Boolean:**
- `ebool` - Encrypted boolean

**Unsigned Integers:**
- `euint4`, `euint8`, `euint16`, `euint32`, `euint64`, `euint128`, `euint256` - Up to `euint2048`

**Signed Integers:**
- `eint8`, `eint16`, `eint32`, `eint64`, `eint128`, `eint256`

**Special Types:**
- `eaddress` - Encrypted Ethereum address
- `AsciiString` - Encrypted ASCII string

## Example Usage

```solidity
import {FHE, euint64} from "fhevm/lib/FHE.sol";

contract ConfidentialToken {
    mapping(address => euint64) private balances;

    function transfer(address to, euint64 amount) external {
        balances[msg.sender] = FHE.sub(balances[msg.sender], amount);
        balances[to] = FHE.add(balances[to], amount);
    }
}
```

## Core Operations

All standard operations are supported on encrypted types:

**Arithmetic:** `add`, `sub`, `mul`, `div`, `rem`, `min`, `max`
**Comparison:** `eq`, `ne`, `lt`, `le`, `gt`, `ge`
**Bitwise:** `and`, `or`, `xor`, `not`, `shl`, `shr`
**Control Flow:** `select` (ternary: `condition ? a : b`)

## Type System Design

Each encrypted type:
- Is a distinct Solidity type (strong typing)
- Internally stores a `bytes32` handle
- Operations return new encrypted values
- Cannot be implicitly converted to plaintext

## Key Files

- `lib/FHE.sol` - Primary import for developers
- `examples/EncryptedERC20.sol` - Reference implementation
- `codegen/` - Code generation for operator overloads

## Areas for Deeper Documentation

**[TODO: Encrypted type system]** - Document all supported types, their bit sizes, conversion rules between types, and memory/gas implications

**[TODO: Codegen system]** - Explain how operator overloads are generated, how to extend the library with new operations, and the codegen toolchain

**[TODO: Best practices]** - Document recommended patterns for building confidential smart contracts, common pitfalls, and optimization strategies

**[TODO: Examples deep-dive]** - Walkthrough of reference implementations (EncryptedERC20, ConfidentialVoting, etc.) with detailed explanations

---

**Related:**
- [Host Contracts](host-contracts.md) - Underlying symbolic execution engine
- [Key Concepts](../key-concepts.md) - Understanding handles and symbolic execution
- [Workflows: Input Verification](../workflows/input-verification.md) - How users submit encrypted inputs
