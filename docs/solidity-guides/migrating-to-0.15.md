# Migrating to v0.15

This page lists what changes for Solidity developers between protocol v0.14 and v0.15, and what to check in existing contracts before the upgrade reaches the network you deploy on. It only covers the `@fhevm/solidity` library and the host contracts it talks to; SDK changes are documented in the [SDK guides](https://docs.zama.ai/protocol/relayer-sdk-guides).

## Behaviour changes

### Oversized shift amounts now return zero

`FHE.shl(a, k)` and `FHE.shr(a, k)` with an amount `k` greater than or equal to the bit width `N` of `a`:

| Version        | Result                                    |
| -------------- | ----------------------------------------- |
| up to v0.14    | shift by `k mod N` (`shr(euint64 x, 70)` equals `shr(x, 6)`) |
| v0.15 and later | `0`, like `>>` and `<<` on Solidity `uintN` |

This comes from TFHE-rs 1.7, which the coprocessors adopt with v0.15. It applies to both the plaintext and the encrypted amount overloads. Rotations (`rotl`, `rotr`) are **not** affected and keep reducing the amount modulo `N`.

**What to check.** Search your contracts for `FHE.shl` and `FHE.shr`. If the amount is a constant below the width, nothing changes. If the amount is computed, user-provided or encrypted and can reach `N`, decide which semantics you want and make it explicit:

```solidity
// Keep the old wrap-around behaviour on both versions
euint64 y = FHE.shr(x, FHE.rem(k, 64));   // encrypted amount
euint64 z = FHE.shr(x, uint8(k % 64));    // plaintext amount

// Or embrace the new one: any k >= 64 gives 0, no extra code needed on v0.15
```

Full specification: [Operator semantics](operations/semantics.md#shift-and-rotate-amounts).

### Handle derivation

Handles produced by v0.15 are derived differently from v0.14 for the same computation (the executor now distinguishes operand origins). This is only visible to code that assumed a relationship between handle bytes and the computation that produced them, which was never guaranteed. Contracts that follow [Handles](handles.md) are unaffected.

## New in the library since v0.14

These functions ship in `@fhevm/solidity` 0.14 and later and are now documented:

| Function                                                     | What it does                                                           | Documentation                                         |
| ------------------------------------------------------------ | ---------------------------------------------------------------------- | ----------------------------------------------------- |
| `FHE.mulDiv(a, b, divisor)`                                  | `(a * b) / divisor` without intermediate overflow                       | [Operator semantics](operations/semantics.md#muldiv)  |
| `FHE.toExternal(value)`                                      | Re-wrap a handle as an input type for contract-to-contract calls        | [Encrypted inputs](inputs.md#re-exporting-a-handle-with-fhetoexternal) |
| `FHE.sendLZConfidentialBridge`, `quoteLZConfidentialBridge`, `getLZConfidentialBridgeAddress` | Bridge handles to another host chain | [Confidential bridge](bridge.md)                      |
| `ConfidentialOApp` base contracts                            | Peer registry, typed senders and a secured receiver for cross-chain apps | [Confidential bridge](bridge.md)                      |
| `ZamaPolygonConfig`, `ZamaMultiChainConfig`                  | Configuration contracts for Polygon and multi-chain deployments          | [Configuration](configure.md)                          |

`FHE.sum` and `FHE.isIn` predate v0.14 but were undocumented; they are now specified in [Operator semantics](operations/semantics.md).

## Networks

Polygon mainnet (chain id 137) and Polygon Amoy (80002) are supported host chains. Addresses are listed in [Contract addresses](contract_addresses.md). A contract compiled against `ZamaEthereumConfig` reverts at deployment on Polygon; switch to `ZamaPolygonConfig` or `ZamaMultiChainConfig`.

## Checklist

- [ ] No `FHE.shl` / `FHE.shr` depends on an amount that can reach the operand width, or the amount is reduced explicitly.
- [ ] No code compares handle bytes to infer anything about the underlying computation.
- [ ] Public decryption callbacks verify proofs with `FHE.checkSignatures` and carry their own replay guard ([Verifying public decryptions](decryption/verification.md)).
- [ ] Contracts meant for several host chains inherit `ZamaMultiChainConfig`.
- [ ] Tests that assert exact HCU consumption are re-run: costs for `mulDiv`, `sum` and `isIn` are listed in [HCU](hcu.md).
