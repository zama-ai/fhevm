# Operator semantics

This page is the reference for what each `FHE` operator computes, which operand shapes it accepts, and what happens at the edges: overflow, division by zero, oversized shift amounts, narrowing casts. Treat it as the specification the [operations overview](README.md) summarises. Behaviour is the same on every host chain and with every number of coprocessors, because the coprocessors must agree on results.

## Notation

- `euintN` is an encrypted unsigned integer of `N` bits, `N` in {8, 16, 32, 64, 128, 256}. `eaddress` is an alias for a 160-bit encrypted integer. `ebool` is an encrypted boolean.
- "Scalar" means a plaintext Solidity value (`uintN`, `bool`, `address`) passed directly to the operator.
- All integer arithmetic below is unsigned and performed modulo `2^N`.

## General rules

**Every operation is a transaction.** Calling an operator records a computation request on-chain and returns a new handle immediately; the coprocessors compute the ciphertext asynchronously. A function that performs FHE operations cannot be `view`, and the caller must be [allowed](../acl/README.md) on every encrypted operand, otherwise the call reverts with `ACLNotAllowed`.

**Uninitialised operands.** A handle that was never assigned (`bytes32(0)`) is treated by the library as a trivial encryption of `0` (or `false`). Use `FHE.isInitialized` when this default is not what you want.

**Mixed widths.** Binary operators accept two encrypted operands of different widths. The narrower operand is implicitly zero-extended and the result takes the wider type: `FHE.add(euint8, euint32)` returns `euint32`. This applies to arithmetic, bitwise, comparison and `min`/`max`. It does **not** apply to `select`, whose two branches must have exactly the same type.

**Scalar operands.** Most binary operators accept one plaintext operand. The backend only accepts a plaintext on the right, so `FHE.add(3, x)` is rewritten as `FHE.add(x, 3)` and `FHE.gt(3, x)` as `FHE.lt(x, 3)`. The scalar width must not exceed the encrypted width (`add(uint32, euint16)` does not compile). Scalar variants are significantly cheaper in HCU; prefer them whenever one operand is public.

**Results are always fresh handles.** Two calls with the same inputs return different handles (see [Handles](../handles.md)). Never compare handles to test equality of values; use `FHE.eq`.

**No information leaks through failure.** Operators never revert because of the encrypted values themselves. The reverts listed below depend only on public data: operand types, scalar values, array lengths and ACL state.

## Arithmetic

| Operator                 | Result                           | Notes                                                                     |
| ------------------------ | -------------------------------- | ------------------------------------------------------------------------- |
| `add(a, b)`              | `(a + b) mod 2^N`                | Wraps silently on overflow.                                               |
| `sub(a, b)`              | `(a - b) mod 2^N`                | Wraps silently on underflow: `sub(0, 1)` is `2^N - 1`.                    |
| `mul(a, b)`              | `(a * b) mod 2^N`                | Wraps silently on overflow.                                               |
| `div(a, s)`              | `floor(a / s)`                   | `s` **must be a scalar**. `s == 0` reverts with `DivisionByZero`.        |
| `rem(a, s)`              | `a mod s`                        | Same constraints as `div`. `a == div(a, s) * s + rem(a, s)` always holds. |
| `neg(a)`                 | `(2^N - a) mod 2^N`              | Two's complement negation. `neg(0)` is `0`.                               |
| `min(a, b)`, `max(a, b)` | smallest / largest value         |                                                                           |
| `mulDiv(a, b, s)`        | `floor((a * b) / s)` on 2N bits  | `s` **must be a scalar**, non-zero. See below.                            |
| `sum(values)`            | `(v_0 + ... + v_k) mod 2^N`      | Fixed-size array of one type. See below.                                  |

Overflow is unchecked by design: a checked operation would have to reveal whether the overflow happened. When an overflow must be detected, test for it explicitly and neutralise it with `FHE.select`, as shown in the [overflow pattern](README.md#beware-of-overflows-of-fhe-arithmetic-operators).

Division and remainder by an **encrypted** divisor are not available. The only overloads are `div(euintN, uintN)` and `rem(euintN, uintN)`, for `N` up to 128.

### `mulDiv`

```solidity
function mulDiv(euintN a, euintN b, uintN divisor) internal returns (euintN)
function mulDiv(euintN a, uintN b, uintN divisor) internal returns (euintN)
```

Computes `floor((a * b) / divisor)` for `N` in {8, 16, 32, 64}. The product is formed on `2N` bits, so it cannot overflow before the division; the quotient is then brought back to `N` bits, wrapping if it does not fit. Use it for proportional splits (`amount * share / totalShares`) where the intermediate product would overflow a plain `mul`. A zero `divisor` reverts with `DivisionByZero`. The divisor is always public; the first factor is always encrypted; the second factor may be either.

Cost is roughly a `mul` plus a `div` on the doubled width, see [HCU](../hcu.md).

### `sum`

```solidity
function sum(euintN[] memory values) internal returns (euintN)
```

Adds all elements of `values` and returns the wrapped sum, for `N` in {8, 16, 32, 64, 128}. All elements must have the same type (`IncompatibleTypes` otherwise). The array length is capped by the executor: at most **100** elements for `euint8`, `euint16` and `euint32`, at most **60** for `euint64` and `euint128`; larger arrays revert with `FHECollectionSizeInvalid`. An empty array returns a trivial encryption of `0`. `sum` is cheaper than a chain of `add` calls and produces a single handle.

## Bitwise

| Operator             | Result                          | Notes                                                                 |
| -------------------- | ------------------------------- | --------------------------------------------------------------------- |
| `and`, `or`, `xor`   | bitwise                         | Also available on `ebool`. Scalar variants trivially encrypt the scalar first. |
| `not(a)`             | flips all `N` bits              | On `ebool`, logical negation.                                          |
| `shl(a, k)`          | `(a << k) mod 2^N`              | `k` is `euint8` or `uint8`. See shift amount rules.                   |
| `shr(a, k)`          | `a >> k` (logical, fills with 0) | `k` is `euint8` or `uint8`.                                            |
| `rotl(a, k)`         | rotate left by `k mod N`        |                                                                       |
| `rotr(a, k)`         | rotate right by `k mod N`       |                                                                       |

### Shift and rotate amounts

The shift amount is always an 8-bit value, encrypted or not, whatever the width of the first operand. What happens when the amount is **greater than or equal to `N`** depends on the protocol version:

| Protocol version | `shl(a, k)` / `shr(a, k)` with `k >= N`               | `rotl` / `rotr` |
| ---------------- | ------------------------------------------------------ | --------------- |
| up to v0.14      | amount reduced modulo `N`: `shr(euint64 x, 70) == shr(x, 6)` | `k mod N`       |
| **v0.15 and later** | **result is `0`**, matching Solidity `>>` and `<<` on `uintN` | `k mod N` (unchanged) |

{% hint style="warning" %}
This is a behaviour change in v0.15, inherited from TFHE-rs 1.7. Contracts that relied on the modulo behaviour for oversized shift amounts compute different results after the upgrade. If your shift amounts can reach `N`, reduce them yourself (`k % N` on a scalar, or `FHE.rem(k, N)`) so the code behaves identically before and after. See [Migrating to v0.15](../migrating-to-0.15.md).
{% endhint %}

Rotations keep the modulo semantics in every version. Because every `euintN` width is a power of two, `k mod N` is exact.

## Comparisons

| Operator                                  | Result  | Available on                     |
| ----------------------------------------- | ------- | -------------------------------- |
| `eq(a, b)`, `ne(a, b)`                    | `ebool` | all encrypted types, `eaddress` included |
| `lt`, `le`, `gt`, `ge`                    | `ebool` | `euintN`                          |
| `isIn(value, set)`                        | `ebool` | `euintN` and `eaddress`           |

Comparisons are unsigned. With a scalar on the left the library swaps operands and the comparison, so `FHE.gt(5, x)` returns the same value as `FHE.lt(x, 5)`.

### `isIn`

```solidity
function isIn(euintN value, euintN[] memory set) internal returns (ebool)
function isIn(eaddress value, eaddress[] memory set) internal returns (ebool)
```

Returns an encrypted `true` if `value` equals at least one element of `set`, `false` otherwise, without revealing which element matched. An empty set returns an encrypted `false`. All elements must have the type of `value`. The set size is capped at **100** elements for `euint8`, `euint16` and `euint32`, and **60** for `euint64`, `euint128`, `eaddress` and `euint256` (`FHECollectionSizeInvalid` otherwise). Typical uses: encrypted allow-lists, checking a hidden choice against a small menu of encrypted options.

## Selection

```solidity
function select(ebool control, T a, T b) internal returns (T)
```

Returns `a` if `control` is `true`, `b` otherwise. Both branches are always evaluated: `select` is a multiplexer, not a jump, so it cannot short-circuit and cannot leak which branch was taken. `a` and `b` must have exactly the same type; cast one of them explicitly if they differ. This is the only way to branch on encrypted data; see [Branching](../logics/conditions.md).

## Casting and trivial encryption

| Call                    | Behaviour                                                                          |
| ----------------------- | ---------------------------------------------------------------------------------- |
| `asEuintM(euintN x)`, `M < N` | **Truncation**: keeps the low `M` bits.                                       |
| `asEuintM(euintN x)`, `M > N` | Zero-extension.                                                              |
| `asEbool(euintN x)`     | `x != 0`.                                                                          |
| `asEuintN(ebool b)`     | `1` if `true`, `0` otherwise.                                                      |
| `asEuintN(uintN v)`     | **Trivial encryption** of the public value `v`. Not confidential: `v` is visible on-chain. |
| `asEaddress(address a)` | Trivial encryption of a public address.                                            |
| `asEbool(bool b)`       | Trivial encryption of a public boolean.                                            |

Casting a value to its own type reverts with `InvalidType`. Trivially encrypted values are useful as constants in computations and cost almost nothing (32 HCU), but they carry no secrecy whatsoever.

## Random values

See [Generate random numbers](random.md). Bounded generation requires a power-of-two bound and is exactly uniform.

## Public errors

| Error                                   | Raised when                                                                 |
| --------------------------------------- | --------------------------------------------------------------------------- |
| `ACLNotAllowed(handle, account)`        | The calling contract is not allowed on an operand.                          |
| `DivisionByZero()`                      | Scalar divisor of `div`, `rem` or `mulDiv` is zero.                         |
| `IncompatibleTypes()`                   | Operands of an n-ary operator (`sum`, `isIn`) do not share a type.          |
| `UnsupportedType()`                     | The operator does not exist for this type (for example `add` on `euint256`). |
| `IsNotScalar()`                         | `div` or `rem` called with an encrypted divisor at the executor level.      |
| `FHECollectionSizeInvalid(size, limit)` | `sum` or `isIn` array longer than the cap.                                  |
| `NotPowerOfTwo()`, `UpperBoundAboveMaxTypeValue()` | Invalid bound for `randEuintN(upperBound)`.                      |
| `InvalidType()`                         | Cast to the same type.                                                      |

## Supported operators per type

| Type                   | Arithmetic                                        | Bitwise                                    | Comparison                          | Other                              |
| ---------------------- | ------------------------------------------------- | ------------------------------------------ | ----------------------------------- | ---------------------------------- |
| `ebool`                |                                                   | `and`, `or`, `xor`, `not`                  | `eq`, `ne`                          | `select`, `rand`                   |
| `euint8` to `euint64`  | `add`, `sub`, `mul`, `div`, `rem`, `neg`, `min`, `max`, `mulDiv`, `sum` | all, including shifts and rotations | all, `isIn`                         | `select`, `rand`, `randBounded`    |
| `euint128`             | `add`, `sub`, `mul`, `div`, `rem`, `neg`, `min`, `max`, `sum` | all                                | all, `isIn`                         | `select`, `rand`, `randBounded`    |
| `eaddress`             |                                                   |                                            | `eq`, `ne`, `isIn`                  | `select`                           |
| `euint256`             | `neg`                                             | all                                        | `eq`, `ne`, `isIn`                  | `select`, `rand`, `randBounded`    |
