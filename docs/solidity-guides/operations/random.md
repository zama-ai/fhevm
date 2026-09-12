# Generate random numbers

This document explains how to generate encrypted random values fully on-chain with the `FHE` library, what guarantees they carry, and how the bounded variant behaves. Random values are produced as ciphertexts: nobody, including the contract that requested them, learns the value until it is decrypted through the protocol.

## Summary

| Function                           | Result   | Range                 | Notes                                                        |
| ---------------------------------- | -------- | --------------------- | ------------------------------------------------------------ |
| `FHE.randEbool()`                  | `ebool`  | `{false, true}`       |                                                              |
| `FHE.randEuintX()`                 | `euintX` | `[0, 2^X - 1]`        | `X` in 8, 16, 32, 64, 128, 256                               |
| `FHE.randEuintX(uintX upperBound)` | `euintX` | `[0, upperBound - 1]` | `upperBound` must be a power of two, `0 < upperBound <= 2^X` |

There is no random `eaddress` and no built-in random value for a non-power-of-two range. Both are deliberate, see [Bounded random numbers](#bounded-random-numbers).

## Key notes

- **Transactions only.** Random generation mutates on-chain state (a counter that feeds the seed), so it can only run inside a transaction. Calling it through `eth_call` does not produce a usable handle.
- **Encrypted output.** The result is a regular encrypted handle. It follows the same ACL rules as any other ciphertext: allow it to the contract with `FHE.allowThis` if you store it, allow it to a user if they must decrypt it.
- **Cost.** Each call is metered in HCU like any other FHE operation (see [HCU](../hcu.md)). Generation is cheap compared to arithmetic: 19,000 HCU for `ebool` and 23,000 to 30,000 HCU for the integer types.

## Basic usage

```solidity
ebool rb = FHE.randEbool();          // random encrypted boolean
euint8 r8 = FHE.randEuint8();        // uniform in [0, 255]
euint16 r16 = FHE.randEuint16();     // uniform in [0, 65535]
euint32 r32 = FHE.randEuint32();
euint64 r64 = FHE.randEuint64();
euint128 r128 = FHE.randEuint128();
euint256 r256 = FHE.randEuint256();
```

### Example: coin flip

```solidity
function flip() external returns (ebool) {
  ebool heads = FHE.randEbool();
  FHE.allowThis(heads);
  FHE.allow(heads, msg.sender);
  return heads;
}
```

## Bounded random numbers

`FHE.randEuintX(upperBound)` returns a value uniformly distributed in `[0, upperBound - 1]`.

```solidity
euint8 dice = FHE.randEuint8(8);        // 0..7
euint16 slot = FHE.randEuint16(1024);   // 0..1023
euint64 idx = FHE.randEuint64(1 << 40); // 0..2^40 - 1
```

**The upper bound must be a power of two.** The call reverts on-chain with `NotPowerOfTwo` otherwise, and with `UpperBoundAboveMaxTypeValue` if the bound does not fit the result type. This restriction exists so that the result is **exactly uniform**: the coprocessor keeps the low `log2(upperBound)` bits of a full-width random value and clears the rest, which introduces no modulo bias. FHEVM deliberately does not expose the arbitrary-bound sampling of TFHE-rs, whose output is slightly biased.

### Random number in an arbitrary range

If you need a range `[0, n)` where `n` is not a power of two, you must build it yourself and accept one of two trade-offs:

- **Rejection sampling across transactions.** Draw `r = FHE.randEuintX(nextPowerOfTwo(n))`, compute `FHE.lt(r, n)`, decrypt that boolean, and draw again in a later transaction if it is `false`. The result is uniform, but the number of rounds is not fixed.
- **Modulo reduction.** Compute `FHE.rem(FHE.randEuintX(), n)` with a scalar `n`. Always one round, but slightly biased toward small values. The bias is bounded by `n / 2^X`, so pick a width `X` much larger than `log2(n)`: a six-sided die drawn from `euint64` has a bias below `2^-61`, the same die drawn from `euint8` has a bias of about 2 %.

Document the choice in your contract. For games of chance or lotteries, prefer rejection sampling or a width large enough that the bias is negligible for your stakes.

## What the randomness guarantees

- **Source.** The seed is derived on-chain by the `FHEVMExecutor` from a domain separator, a counter that advances on every call, the ACL address, the chain id, the previous block hash and the block timestamp. The coprocessors expand that seed into an encrypted value with the TFHE-rs encrypted PRF, so the plaintext is never materialised anywhere.
- **Unpredictability.** Nobody can read the value before an authorized decryption: not the contract, not the caller, not the coprocessor operators. Two calls in the same transaction produce independent values because the counter advances between them.
- **Block producer influence.** Because the seed includes the previous block hash and the timestamp, a block producer has the usual, limited influence over which seed a transaction gets. They still cannot learn the resulting value. If your application must resist a block producer choosing between a handful of candidate outcomes, combine on-chain randomness with a commit-reveal scheme or with user-supplied encrypted inputs.
- **Determinism across coprocessors.** All coprocessors derive the same ciphertext from the same seed, which is what lets a multi-coprocessor deployment reach consensus on the result.

For the cryptographic construction of the encrypted PRF, see the [TFHE-rs documentation](https://docs.zama.ai/tfhe-rs/fhe-computation/advanced-features/encrypted-prf).

## Common mistakes

- **Reading the result in the same transaction.** You cannot branch on a random value on-chain except through `FHE.select`. To act on the clear value, request a public or user decryption and continue in a later transaction.
- **Forgetting the ACL.** A random handle stored without `FHE.allowThis` cannot be used by your contract in the next transaction.
- **Using `rem` with a narrow type.** `FHE.rem(FHE.randEuint8(), 6)` is visibly biased. Draw from a wider type first.
- **Expecting a random `eaddress`.** There is none; a random 160-bit value would not be a valid, funded account anyway.
