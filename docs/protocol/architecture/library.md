# FHEVM library

The FHEVM Library enables developers to build smart contracts that operate on encrypted data—without requiring any
knowledge of cryptography. It integrates confidential computing into the standard Solidity development flow by providing
new encrypted data types, operations, and tools for access control and secure input handling.

This library is an essential abstraction layer that hides the complexity of fully homomorphic encryption (FHE) and
interacts seamlessly with offchain components like the coprocessors and the Gateway.

## Encrypted Data Types

The library introduces encrypted variants of common Solidity types, implemented as user-defined value types (internally
represented as bytes32 handles). These types reference actual ciphertexts stored off-chain.

| Category          | Types                          |
| ----------------- | ------------------------------ |
| Booleans          | ebool                          |
| Unsigned integers | euint8, euint16, ..., euint256 |
| Signed integers   | eint8, eint16, ..., eint256    |
| Addresses         | eaddress                       |

## Operations on Encrypted Types Each encrypted type supports operations similar to its plaintext counterpart:

- Arithmetic: add, sub, mul, div, rem, neg
- Logic: and, or, xor, not
- Comparison: lt, gt, le, ge, eq, ne, min, max
- Bit manipulation: shl, shr, rotl, rotr

These operations are symbolically executed on-chain by generating new handles and emitting events for coprocessors to
process the actual FHE computation off-chain.

Example:

```solidity
function compute(euint64 x, euint64 y, euint64 z) public returns (euint64) {
  euint64 result = FHE.mul(FHE.add(x, y), z);
  return result;
}
```

### Branching with Encrypted Conditions

Direct if or require statements are not compatible with encrypted booleans. Instead, the library provides a `select`
operator to emulate conditional logic without revealing which branch was taken:

```solidity
ebool condition = FHE.lte(x, y);
euint64 result = FHE.select(condition, valueIfTrue, valueIfFalse);
```

This preserves confidentiality even in conditional logic.

### Handling External Encrypted Inputs

When users want to pass encrypted inputs (e.g., values they’ve encrypted off-chain or bridged from another chain), they
provide:

- external<Type> values
- A list of coprocessor signatures (attestation)

The function `fromExternal` is used to validate the attestation and extract a usable encrypted handle:

```solidity
function handleInput(externalEuint64 param1, externalEbool param2, bytes calldata attestation) public {
  euint64 val = FHE.fromExternal(param1, attestation);
  ebool flag = FHE.fromExternal(param2, attestation);
}
```

This ensures that only authorized, well-formed ciphertexts are accepted by smart contracts.

### Access Control

The FHE library also exposes methods for managing access to encrypted values using the ACL maintained by host contracts:

- `allow(handle, address)`: Grant persistent access
- `allowTransient(handle, address)`: Grant access for the current transaction only
- `allowForDecryption(handle)`: Make handle publicly decryptable
- `isAllowed(handle, address)`: Check if address has access
- `isSenderAllowed(handle)`: Shortcut for checking msg.sender permissions

These `allow` methods emit events consumed by the coprocessors to replicate the ACL state in the Gateway.

### Pseudo-Random Encrypted Values

The library allows generation of pseudo-random encrypted integers, useful for games, lotteries, or randomized logic:

- `randEuintXX()`
- `randEuintXXBounded`(uint bound)

These are deterministic across coprocessors and indistinguishable to external observers.
