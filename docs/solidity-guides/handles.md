# Handles

Every encrypted value in FHEVM (`euint8`, `ebool`, `eaddress`, …) is referenced on-chain by a 32-byte **handle**. FHE operations take and return handles, and the ACL is enforced per handle.

## Glossary

Three concepts that look similar but live in different places:

| Term | What it is | Where it lives |
| --- | --- | --- |
| **Plaintext** | The actual cleartext value (e.g. the number `42`). What `decrypt(...)` returns. | Off-chain, only after authorized decryption. |
| **Ciphertext** | The encrypted blob produced by FHE — the bytes the coprocessor stores and computes on. May be re-randomized at any time without changing the plaintext. | Off-chain, with the coprocessor. |
| **Handle** | A 32-byte on-chain identifier that points to a specific (plaintext, ciphertext) pair. The handle is what your Solidity code holds and passes around. | On-chain. |
| **Computation** | The sequence of FHE operations that produced a handle (e.g. `FHE.add(a, b)`). Two different computations may produce the same handle, or the same computation may produce different handles in different contexts. | Conceptual — not stored as such. |

The protocol guarantees a relationship between **handles** and **plaintexts**. It deliberately does not guarantee any relationship between handles and ciphertexts, or between handles and the computations that produced them.

## Core principle: handles are opaque

{% hint style="warning" %}
Treat handles like a name tag, not like the thing itself. The bytes don't tell you anything about how the handle was made or what ciphertext is behind it. The only thing you can read off a handle is which plaintext it points to — and only after decryption.
{% endhint %}

You **can** treat a handle like any other `bytes32`: compare it with `==` / `!=`, store it, log it, etc. The ACL itself works this way. What you **can't** do is guess from the bytes how the handle was made, or which ciphertext sits behind it.

## What you can rely on

The protocol gives you one rule:

> **If two handles are equal, they point to the same plaintext.**

That rule has a mirror that's also true: if two plaintexts are different, their handles must be different too. Anything beyond that, you can't assume.

| You can rely on | You can't rely on |
| --- | --- |
| Equal handles → equal plaintexts | Different handles → different plaintexts |
| Different plaintexts → different handles | Equal plaintexts → equal handles |
| | Equal handles → same computation produced both |
| | Equal handles → same ciphertext underneath |

The protocol may produce equal handles for some equal-plaintext computations and different ones for others — across blocks, across chains, after ciphertext re-randomization, or under future optimizations. Your contract must work either way.

## Common mistakes

**Assuming "same operation, same inputs → same handle".** Today, the protocol mixes the previous block's hash into how each handle is built, so the same computation in two different blocks already gives you different handles. And the reverse — assuming two different operations always produce different handles — can break if the protocol ever optimizes them down to the same handle.

```solidity
// ❌ Don't depend on h3 == h4. Don't depend on h3 != h4 either.
euint64 h3 = FHE.add(h1, h2);
euint64 h4 = FHE.add(h1, h2);
```

**Mixing handles from different places.** A handle that was bridged from another chain, or built off-chain and brought in via `FHE.fromExternal(...)`, is not guaranteed to equal a handle produced by on-chain computation — even when both encode the same plaintext.

If you need to know whether two encrypted values are equal in plaintext, use the FHE operator. It returns an `ebool` that decrypts to `true` if and only if the underlying values match:

```solidity
ebool isEqual = FHE.eq(a, b);
```
