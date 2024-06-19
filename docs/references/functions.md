# Function specifications

The functions exposed by the `TFHE` Solidity library come in various shapes and sizes in order to facilitate developer experience.
For example, most binary operators (e.g., `add`) can take as input any combination of the supported data types.

In the `fhEVM`, FHE operations are only defined on same-type operands. Implicit upcasting will be done automatically, if necessary.

Most binary operators are also defined with a mix of ciphertext and plaintext operands, under the condition that the size of the plaintext operand is at most the size of the encrypted operand.
For example, `add(uint8 a, euint8 b)` is defined but `add(uint32 a, euint16 b)` is not.
Note that these ciphertext-plaintext operations may take less time to compute than ciphertext-ciphertext operations.

## `asEuint`

The `asEuint` functions serve three purposes:

1. verify ciphertext bytes and return a valid handle to the calling smart contract;
2. cast a `euintX` typed ciphertext to a `euintY` typed ciphertext, where `X != Y`;
3. trivially encrypt a plaintext value.

The first case is used to process encrypted inputs, e.g. user-provided ciphertexts. Those are generally included in a transaction payload.

The second case is self-explanatory. When `X > Y`, the most significant bits are dropped. When `X < Y`, the ciphertext is padded to the left with trivial encryptions of `0`.

The third case is used to "encrypt" a public value so that it can be used as a ciphertext.
Note that what we call a trivial encryption is **not** secure in any sense.
When trivially encrypting a plaintext value, this value is still visible in the ciphertext bytes.
More information about trivial encryption can be found [here](https://www.zama.ai/post/tfhe-deep-dive-part-1).

### Examples

```solidity
// first case
function asEuint8(bytes memory ciphertext) internal view returns (euint8)
// second case
function asEuint16(euint8 ciphertext) internal view returns (euint16)
// third case
function asEuint16(uint16 value) internal view returns (euint16)
```

## `asEbool`

The `asEbool` functions behave similarly to the `asEuint` functions, but for encrypted boolean values.

## Arithmetic operations (`add`, `sub`, `mul`, `div`, `rem`)

Performs the operation homomorphically.

Note that division/remainder only support plaintext divisors.

### Examples

```solidity
// a + b
function add(euint8 a, euint8 b) internal view returns (euint8)
function add(euint8 a, euint16 b) internal view returns (euint16)
function add(uint32 a, euint32 b) internal view returns (euint32)

// a / b
function div(euint8 a, uint8 b) internal pure returns (euint8)
function div(euint16 a, uint16 b) internal pure returns (euint16)
function div(euint32 a, uint32 b) internal pure returns (euint32)
```

## Bitwise operations (`AND`, `OR`, `XOR`)

Unlike other binary operations, bitwise operations do not natively accept a mix of ciphertext and plaintext inputs.
To ease developer experience, the `TFHE` library adds function overloads for these operations.
Such overloads implicitely do a trivial encryption before actually calling the operation function, as shown in the examples below.

### Examples

```solidity
// a & b
function and(euint8 a, euint8 b) internal view returns (euint8)

// implicit trivial encryption of `b` before calling the operator
function and(euint8 a, uint16 b) internal view returns (euint16)
```

## Bit shift operations (`<<`, `>>`)

Shifts the bits of the base two representation of `a` by `b` positions.

### Examples

```solidity
// a << b
function shl(euint16 a, euint8 b) internal view returns (euint16)
// a >> b
function shr(euint32 a, euint16 b) internal view returns (euint32)
```

## Rotate operations

Rotates the bits of the base two representation of `a` by `b` positions.

### Examples

```solidity
function rotl(euint16 a, euint8 b) internal view returns (euint16)
function rotr(euint32 a, euint16 b) internal view returns (euint32)
```

## Comparison operation (`eq`, `ne`, `ge`, `gt`, `le`, `lt`)

Note that in the case of ciphertext-plaintext operations, since our backend only accepts plaintext right operands, calling the operation with a plaintext left operand will actually invert the operand order and call the _opposite_ comparison.

The result of comparison operations is an encrypted boolean (`ebool`). In the backend, the boolean is represented by an encrypted unsinged integer of bit width 8, but this is abstracted away by the Solidity library.

### Examples

```solidity
// a == b
function eq(euint32 a, euint16 b) internal view returns (ebool)

// actually returns `lt(b, a)`
function gt(uint32 a, euint16 b) internal view returns (ebool)

// actually returns `gt(a, b)`
function gt(euint16 a, uint32 b) internal view returns (ebool)
```

## Multiplexer operator (`select`)

This operator takes three inputs. The first input `b` is of type `ebool` and the two others of type `euintX`.
If `b` is an encryption of `true`, the first integer parameter is returned. Otherwise, the second integer parameter is returned.

### Example

```solidity
// if (b == true) return val1 else return val2
function select(ebool b, euint8 val1, euint8 val2) internal view returns (euint8) {
  return TFHE.select(b, val1, val2);
}
```

## `min`, `max`

Returns the minimum (resp. maximum) of the two given values.

### Examples

```solidity
// min(a, b)
function min(euint32 a, euint16 b) internal view returns (euint32)

// max(a, b)
function max(uint32 a, euint8 b) internal view returns (euint32)
```

## Unary operators (`neg`, `not`)

There are two unary operators: `neg` (`-`) and `not` (`!`).
Note that since we work with unsigned integers, the result of negation is interpreted as the modular opposite.
The `not` operator returns the value obtained after flipping all the bits of the operand.

> **_NOTE:_** More information about the behavior of these operators can be found at the [TFHE-rs docs](https://docs.zama.ai/tfhe-rs/getting-started/operations#arithmetic-operations.).

## Generating random encrypted integers

Random encrypted integers can be generated fully on-chain.

That can only be done during transactions and not on an `eth_call` RPC method,
because PRNG state needs to be mutated on-chain during generation.

### Example

```solidity
// Generate a random encrypted unsigned integer `r`.
euint32 r = TFHE.randEuint32();
```

## ACL (`allow`, `allowTransient`, `isAllowed`, `isSenderAllowed`)

Allow an address to use a ciphertext, which includes computation, decryption, and reencryption. The `allow` function will permanently store the allowance in a dedicated contract, while `allowTransient` will temporarily store it in transient storage.

### Example

```solidity
// Store a value in the contract.
r = TFHE.asEuint32(94);
// Set the contract as allowed for this ciphertext.
TFHE.allow(r, address(this));
// Also set the caller as allowed for this ciphertext.
TFHE.allow(r, msg.sender);
```

## ACL verification

To verify whether an address is allowed, `isAllowed` will return true if the specified address has permission. `isSenderAllowed` is similar but uses `msg.sender` as the address.

NOTE: These functions will return true if the ciphertext is authorized, regardless of whether the allowance is on the ACL contract or in transient storage.

### Example

```solidity
// Store a value in the contract.
r = TFHE.asEuint32(94);
TFHE.isAllowed(r, address(this)); // returns true
```
