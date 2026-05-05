# Handles

Every encrypted value in FHEVM (`euint8`, `ebool`, `eaddress`, etc.) is referenced on-chain by a 32-byte **handle**. A handle is the runtime identifier of a ciphertext — every FHE operation (`FHE.add`, `FHE.fromExternal`, `FHE.select`, …) takes and returns handles, and the ACL is enforced per handle.

This page describes the assumptions you can — and cannot — make about handles when writing FHEVM smart contracts.

## Core principle: handles are opaque

{% hint style="warning" %}
Handles in the Zama protocol must be treated as **opaque identifiers**, similar to memory pointers. Application developers must not inspect their concrete values or make any assumptions about how they are computed. In particular, handles cannot be relied on to be equal even if they were computed the same way, nor on being different even if they were computed in different ways.
{% endhint %}

In practice, this means:

- ❌ Don't compare handle bytes with `==` or `!=` and use the result for business logic.
- ❌ Don't hash handles to derive identifiers.
- ❌ Don't assume that two handles produced by the same FHE operation on the same operands will be identical, or that they will be different.
- ✅ Treat a handle as something you read, write to storage, pass to FHE library functions, and that's it.

## What you _can_ assume

The only guarantee you should rely on is:

> **Identical handles refer to identical plaintexts.**

Equivalently, if you observe `h1 == h2` (as bytes), then `decrypt(h1) == decrypt(h2)`.

The converse and contrapositive give the full set of valid implications:

```text
// If handles are equal, plaintexts are equal
h1 == h2  =>  decrypt(h1) == decrypt(h2)

// If plaintexts are different, handles must be different
decrypt(h1) != decrypt(h2)  =>  h1 != h2

// If handles are different, plaintexts MAY or MAY NOT be equal
h1 != h2  =>  decrypt(h1) == decrypt(h2)  OR  decrypt(h1) != decrypt(h2)

// If plaintexts are equal, handles MAY or MAY NOT be equal
decrypt(h1) == decrypt(h2)  =>  h1 == h2  OR  h1 != h2
```

The protocol may produce equal handles for some equal-plaintext computations (deterministic operations on the same inputs) and different handles for others (e.g. fresh encryptions or paths that introduce randomness). **Your contract must remain correct under either outcome.**

## Anti-pattern: relying on handle inequality

A common mistake is assuming that running the same FHE operation twice produces _different_ handles. This is **not guaranteed** and your contract should not depend on it.

```solidity
// ❌ BAD: don't assume h3 and h4 will be different
euint64 h3 = FHE.add(h1, h2);
euint64 h4 = FHE.add(h1, h2);

// h3 == h4  is possible.
// h3 != h4  is also possible.
// Either outcome must be valid for your contract.
```

{% hint style="info" %}
This is currently a **low-risk** anti-pattern: the present implementation behaves in a way that is forgiving for many contracts that accidentally rely on inequality. However, this behaviour is **not part of the protocol contract** and may change in future releases. Code written against the rules above will keep working; code that relies on inequality may not.
{% endhint %}

## Anti-pattern: relying on handle equality of "logically equal" plaintexts

The mirror image is just as dangerous. Two handles whose plaintexts you _know_ to be equal may still be different at the bytes level.

```solidity
// You constructed both as encryptions of zero — but their handles
// may differ.
euint64 a = FHE.asEuint64(0);
euint64 b = FHE.asEuint64(0);

// a == b at the byte level is NOT guaranteed.
// Do not branch on FHE.toBytes32(a) == FHE.toBytes32(b).
```

If you actually need to compare two encrypted values for equality, use the FHE comparison operators — they return an `ebool` whose plaintext truthfully reflects the equality of the underlying values:

```solidity
ebool isEqual = FHE.eq(a, b); // true plaintext-equality, computed homomorphically
```

## Summary

| You may rely on | You must NOT rely on |
| --------------- | -------------------- |
| `h1 == h2  =>  plaintexts are equal` | `same operation on same inputs => identical handles` |
| `plaintexts differ  =>  handles differ` | `different operations => different handles` |
| `FHE.eq(a, b)` to compare encrypted values | comparing handle bytes for business logic |

Treat handles as opaque references, use the FHE library to compare encrypted values, and your contract will stay correct across protocol upgrades.
