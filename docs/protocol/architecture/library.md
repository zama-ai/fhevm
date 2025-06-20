# FHE library

This document offers a high-level overview of the **FHEVM library**, helping you understand how it fits into the broader
Zama Protocol. To learn how to use it in practice, see the
[Solidity Guides](https://app.gitbook.com/o/-MIF05xPVoj0l_wnOGB7/s/rDmRmmmSrBgV0SFO4eiZ/).

## What is FHEVM library?

The FHEVM library enables developers to build smart contracts that operate on encrypted data—without requiring any
knowledge of cryptography.

It extends the standard Solidity development flow with:

- Encrypted data types
- Arithmetic, logical, and conditional operations on encrypted values
- Fine-grained access control
- Secure input handling and attestation support

This library serves as an **abstraction layer** over Fully Homomorphic Encryption (FHE) and interacts seamlessly with
off-chain components such as the **Coprocessors** and the **Gateway**.

## Key features

### Encrypted data types

The library introduces encrypted variants of common Solidity types, implemented as user-defined value types. Internally,
these are represented as `bytes32` handles that point to encrypted values stored off-chain.

| Category          | Types                                |
| ----------------- | ------------------------------------ |
| Booleans          | `ebool`                              |
| Unsigned integers | `euint8`, `euint16`, ..., `euint256` |
| Signed integers   | `eint8`, `eint16,` ..., `eint256`    |
| Addresses         | `eaddress`                           |

→ See the full guide of [Encrypted data types](https://app.gitbook.com/s/rDmRmmmSrBgV0SFO4eiZ/smart-contract/types).

### FHE operations

Each encrypted type supports operations similar to its plaintext counterpart:

- Arithmetic: `add`, `sub`, `mul`, `div`, `rem`, `neg`
- Logic: `and`, `or`, `xor`, `not`
- Comparison: `lt`, `gt`, `le`, `ge`, `eq`, `ne`, `min`, `max`
- Bit manipulation: `shl`, `shr`, `rotl`, `rotr`

These operations are symbolically executed on-chain by generating new handles and emitting events for coprocessors to
process the actual FHE computation off-chain.

Example:

```solidity
function compute(euint64 x, euint64 y, euint64 z) public returns (euint64) {
  euint64 result = FHE.mul(FHE.add(x, y), z);
  return result;
}
```

→ See the full guide of
[Operations on encrypted types](https://app.gitbook.com/s/rDmRmmmSrBgV0SFO4eiZ/smart-contract/operations).

### Branching with encrypted Conditions

Direct if or require statements are not compatible with encrypted booleans. Instead, the library provides a
`select`operator to emulate conditional logic without revealing which branch was taken:

```solidity
ebool condition = FHE.lte(x, y);
euint64 result = FHE.select(condition, valueIfTrue, valueIfFalse);
```

This preserves confidentiality even in conditional logic.

→ See the full guide of [Branching](https://app.gitbook.com/s/rDmRmmmSrBgV0SFO4eiZ/smart-contract/logics/conditions).

### Handling external encrypted inputs

When users want to pass encrypted inputs (e.g., values they’ve encrypted off-chain or bridged from another chain), they
provide:

- external values
- A list of coprocessor signatures (attestation)

The function `fromExternal` is used to validate the attestation and extract a usable encrypted handle:

```solidity
function handleInput(externalEuint64 param1, externalEbool param2, bytes calldata attestation) public {
  euint64 val = FHE.fromExternal(param1, attestation);
  ebool flag = FHE.fromExternal(param2, attestation);
}
```

This ensures that only authorized, well-formed ciphertexts are accepted by smart contracts.

→ See the full guide of [Encrypted input](https://app.gitbook.com/s/rDmRmmmSrBgV0SFO4eiZ/smart-contract/inputs).

### Access control

The FHE library also exposes methods for managing access to encrypted values using the ACL maintained by host contracts:

- `allow(handle, address)`: Grant persistent access
- `allowTransient(handle, address)`: Grant access for the current transaction only
- `allowForDecryption(handle)`: Make handle publicly decryptable
- `isAllowed(handle, address)`: Check if address has access
- `isSenderAllowed(handle)`: Shortcut for checking msg.sender permissions

These `allow` methods emit events consumed by the coprocessors to replicate the ACL state in the Gateway.

→ See the full guide of [ACL](https://app.gitbook.com/s/rDmRmmmSrBgV0SFO4eiZ/smart-contract/acl).

### Pseudo-random encrypted values

The library allows generation of pseudo-random encrypted integers, useful for games, lotteries, or randomized logic:

- `randEuintXX()`
- `randEuintXXBounded`(uint bound)

These are deterministic across coprocessors and indistinguishable to external observers.

→ See the full guide of
[Generate random numbe](https://app.gitbook.com/s/rDmRmmmSrBgV0SFO4eiZ/smart-contract/operations/random)r.
