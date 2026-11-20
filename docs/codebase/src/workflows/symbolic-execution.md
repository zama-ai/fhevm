# Symbolic Execution Pattern

## Overview

The symbolic execution pattern is the foundational workflow that enables FHEVM to provide fast on-chain FHE operations. Instead of performing expensive FHE computation on-chain, contracts generate deterministic "handles" that reference off-chain ciphertexts.

## The Pattern

When a smart contract calls an FHE operation like `FHE.add(a, b)`:

### On-Chain (Fast - milliseconds)

1. **Validate inputs**: `FHEVMExecutor` checks that handles `a` and `b` are valid
2. **Generate deterministic handle**: `handle_c = hash(handle_a, handle_b, "add", counter)`
3. **Emit event**: Log operation details for off-chain coprocessor
4. **Return immediately**: Transaction completes, returning `handle_c`

### Off-Chain (Async - seconds to minutes)

5. **Listen for event**: Coprocessor's host-listener detects the ADD event
6. **Load ciphertexts**: Retrieve actual encrypted data for `a` and `b`
7. **Compute**: Execute `c = TFHE.add(a, b)` using TFHE-rs library
8. **Store ciphertext**: Save resulting ciphertext `c`
9. **Commit**: Submit ciphertext commitment to `CiphertextCommits` contract

## Example Flow

```solidity
// Smart contract code
euint32 balance = balances[user];
euint32 amount = FHE.asEuint32(encryptedAmount);
euint32 newBalance = FHE.add(balance, amount);  // <-- Symbolic execution
balances[user] = newBalance;
```

**What actually happens:**

```
T=0ms:  Contract calls FHE.add(handle_balance, handle_amount)
T=1ms:  FHEVMExecutor generates handle_newBalance deterministically
T=2ms:  Event emitted: AddOp(handle_balance, handle_amount, handle_newBalance)
T=3ms:  Transaction completes ✓ (gas: ~50k)
        Contract state updated: balances[user] = handle_newBalance

[Time passes...]

T=30s:  Coprocessor picks up AddOp event
T=31s:  Loads ciphertext_balance and ciphertext_amount from database
T=45s:  Computes ciphertext_newBalance = TFHE.add(ciphertext_balance, ciphertext_amount)
T=46s:  Stores ciphertext_newBalance in database
T=50s:  Submits commitment to CiphertextCommits contract ✓
```

## Key Characteristics

### Deterministic Handle Generation

Handles are generated using a cryptographic hash of:
- Input handles
- Operation type
- Global counter (ensures uniqueness)

This ensures:
- Same inputs + operation → same output handle
- Handles are collision-resistant
- No randomness required (fully deterministic)

### Eventual Consistency

- **Immediate consistency**: On-chain state (handles) is immediately consistent
- **Eventual consistency**: Off-chain ciphertexts become available later
- **Subsequent operations**: Can proceed with handles before ciphertexts are ready
- **Chain of operations**: Multiple operations can queue up before first completes

### Gas Efficiency

Symbolic execution is extremely gas-efficient:
- Handle generation: ~50k gas
- No expensive crypto on-chain
- Comparable to normal EVM operations

Contrast with actual FHE:
- TFHE.add() on 32-bit integers: ~500ms CPU time
- Cannot be done in EVM's 15-second block time

## Supported Operations

All FHE operations follow this pattern:

**Arithmetic:** add, sub, mul, div, rem, min, max
**Comparison:** eq, ne, lt, le, gt, ge
**Bitwise:** and, or, xor, not, shl, shr, rotl, rotr
**Special:** select (ternary operator)

## Error Handling

**On-chain validation:**
- Invalid handles → revert
- Type mismatches → revert
- HCU limit exceeded → revert

**Off-chain failures:**
- Coprocessor crash → retry mechanism
- Computation error → alert + retry
- Never affects on-chain state (already committed)

## Areas for Deeper Documentation

**[TODO: Handle generation algorithm]** - Detailed cryptographic specification of how handles are generated, including hash function choice and collision resistance proof.

**[TODO: Event format specification]** - Complete schema for FHE operation events, including all fields and encoding details.

**[TODO: Ciphertext commitment scheme]** - How coprocessors commit results to chain and verification mechanism.

**[TODO: Chain of operations optimization]** - How multiple dependent operations are optimized when queued together.

---

**Related:**
- [Host Contracts](../components/host-contracts.md) - FHEVMExecutor implementation
- [Coprocessor](../components/coprocessor.md) - Off-chain FHE computation
- [Key Concepts](../key-concepts.md) - Understanding handles and symbolic execution
