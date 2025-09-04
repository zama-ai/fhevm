# FHEVM Operators Overview

This document provides a comprehensive overview of the operators available in FHEVM for performing confidential computations on encrypted data. These operators enable you to work with Fully Homomorphic Encryption (FHE) ciphertexts while maintaining data privacy.

## What are FHEVM Operators?

FHEVM operators are functions provided by the `FHE` library that allow you to perform computations on encrypted data without ever decrypting it. This enables privacy-preserving smart contracts where sensitive data remains confidential throughout all operations.

## Available Operator Categories

### 1. Arithmetic Operators

Arithmetic operators perform mathematical operations on encrypted integers (`euintX` types):

| Operator | Function | Symbol | Description |
|----------|----------|--------|-------------|
| Addition | `FHE.add(a, b)` | `+` | Add two encrypted values |
| Subtraction | `FHE.sub(a, b)` | `-` | Subtract encrypted values |
| Multiplication | `FHE.mul(a, b)` | `*` | Multiply encrypted values |
| Division | `FHE.div(a, b)` | | Divide by plaintext divisor |
| Remainder | `FHE.rem(a, b)` | | Get remainder with plaintext divisor |
| Negation | `FHE.neg(a)` | `-` | Unary negation (modular opposite) |
| Minimum | `FHE.min(a, b)` | | Return the smaller of two values |
| Maximum | `FHE.max(a, b)` | | Return the larger of two values |

{% hint style="info" %}
**Important**: Division (`FHE.div`) and remainder (`FHE.rem`) operations are currently supported only with plaintext divisors for security and performance reasons.
{% endhint %}

### 2. Bitwise Operators

Bitwise operators perform logical operations on the binary representation of encrypted integers:

| Operator | Function | Symbol | Description |
|----------|----------|--------|-------------|
| Bitwise AND | `FHE.and(a, b)` | `&` | Logical AND of corresponding bits |
| Bitwise OR | `FHE.or(a, b)` | `\|` | Logical OR of corresponding bits |
| Bitwise XOR | `FHE.xor(a, b)` | `^` | Logical XOR of corresponding bits |
| Bitwise NOT | `FHE.not(a)` | `~` | Flip all bits (unary operator) |
| Shift Left | `FHE.shl(a, b)` | `<<` | Shift bits left by specified amount |
| Shift Right | `FHE.shr(a, b)` | `>>` | Shift bits right by specified amount |
| Rotate Left | `FHE.rotl(a, b)` | | Rotate bits left by specified amount |
| Rotate Right | `FHE.rotr(a, b)` | | Rotate bits right by specified amount |

{% hint style="info" %}
Shift operators automatically apply modulo to the shift amount based on the operand's bit width. For example, `FHE.shr(euint64 x, 70)` is equivalent to `FHE.shr(euint64 x, 6)` because `70 % 64 = 6`.
{% endhint %}

### 3. Comparison Operators

Comparison operators return encrypted boolean (`ebool`) results:

| Operator | Function | Symbol | Description |
|----------|----------|--------|-------------|
| Equal | `FHE.eq(a, b)` | `==` | Check if values are equal |
| Not Equal | `FHE.ne(a, b)` | `!=` | Check if values are different |
| Greater Than | `FHE.gt(a, b)` | `>` | Check if first value is greater |
| Greater Than or Equal | `FHE.ge(a, b)` | `>=` | Check if first value is greater or equal |
| Less Than | `FHE.lt(a, b)` | `<` | Check if first value is less |
| Less Than or Equal | `FHE.le(a, b)` | `<=` | Check if first value is less or equal |

### 4. Conditional Operator

The conditional operator enables branching logic on encrypted conditions:

| Operator | Function | Description |
|----------|----------|-------------|
| Select | `FHE.select(condition, a, b)` | Return `a` if condition is true, otherwise return `b` |

This is the primary way to implement conditional logic in FHEVM, as traditional `if/else` statements don't work with encrypted booleans.

### 5. Random Number Generation

Random operators generate cryptographically secure random encrypted values:

| Operator | Function | Description |
|----------|----------|-------------|
| Random Boolean | `FHE.randEbool()` | Generate random encrypted boolean |
| Random Integer | `FHE.randEuintX()` | Generate random encrypted integer of specified bit width |
| Bounded Random | `FHE.randEuintXBounded(bound)` | Generate random integer within specified range |

{% hint style="warning" %}
Random number generation must be performed during transactions, not in `eth_call` RPC methods, as it requires updating the PRNG state on-chain.
{% endhint %}

## Type Compatibility

### Supported Encrypted Types

All operators support the following encrypted types:

- **Booleans**: `ebool`
- **Unsigned Integers**: `euint8`, `euint16`, `euint32`, `euint64`, `euint128`, `euint256`
- **Addresses**: `eaddress` (alias for `euint160`)

### Mixed Operations

Many operators support mixing encrypted and plaintext operands:

```solidity
// These are valid and more efficient
euint32 result1 = FHE.add(encryptedValue, 42);        // encrypted + plaintext
euint32 result2 = FHE.mul(encryptedValue, 10);        // encrypted * plaintext
ebool result3 = FHE.eq(encryptedValue, 100);          // encrypted == plaintext

// These are also valid but more expensive
euint32 result4 = FHE.add(encryptedValue1, encryptedValue2);  // encrypted + encrypted
```

## Best Practices

### 1. Choose Appropriate Type Sizes

Use the smallest encrypted type that can accommodate your data to optimize gas costs:

```solidity
// Good: Use appropriate sized types
euint8 age = FHE.asEuint8(25);           // age fits in 8 bits
euint8 percentage = FHE.asEuint8(75);    // percentage fits in 8 bits

// Avoid: Using oversized types wastes gas
euint256 age = FHE.asEuint256(25);       // unnecessary overhead
```

### 2. Prefer Scalar Operands

When possible, use plaintext operands instead of encrypted ones for better performance:

```solidity
// More efficient (scalar operand)
euint32 result = FHE.add(encryptedValue, 42);

// Less efficient (encrypted operand)
euint32 result = FHE.add(encryptedValue, FHE.asEuint32(42));
```

### 3. Handle Overflow Carefully

FHE arithmetic operations can overflow. Always consider overflow scenarios:

```solidity
// Safe addition with overflow check
euint32 temp = FHE.add(totalSupply, mintedAmount);
ebool isOverflow = FHE.lt(temp, totalSupply);
totalSupply = FHE.select(isOverflow, totalSupply, temp);
```

### 4. Use Conditional Logic Appropriately

Replace traditional `if/else` statements with `FHE.select`:

```solidity
// Instead of if/else (which doesn't work with encrypted booleans)
// if (encryptedCondition) {
//     return valueA;
// } else {
//     return valueB;
// }

// Use FHE.select
return FHE.select(encryptedCondition, valueA, valueB);
```

## Performance Considerations

### Gas Costs

Different operations have varying gas costs:

- **Bitwise operations** are generally the fastest (28,000-38,000 HCU)
- **Arithmetic operations** are moderate (83,000-1,671,000 HCU depending on type size)
- **Comparison operations** are moderate (49,000-206,000 HCU)
- **Random generation** is relatively expensive (100,000 HCU)

### Type Size Impact

Larger encrypted types consume more gas:

- `euint8` operations are the most efficient
- `euint256` operations are the most expensive
- Choose the smallest type that fits your data requirements

## Examples

### Basic Arithmetic

```solidity
function calculateTotal(euint32 price, euint32 quantity) public view returns (euint32) {
    return FHE.mul(price, quantity);  // Calculate total price
}
```

### Conditional Logic

```solidity
function updateBalance(euint32 currentBalance, euint32 amount, ebool isDeposit) public {
    euint32 newBalance = FHE.select(
        isDeposit,
        FHE.add(currentBalance, amount),    // If deposit, add amount
        FHE.sub(currentBalance, amount)     // If withdrawal, subtract amount
    );
    balance = newBalance;
}
```

### Comparison and Selection

```solidity
function findMaximum(euint32 a, euint32 b) public view returns (euint32) {
    ebool aIsGreater = FHE.gt(a, b);
    return FHE.select(aIsGreater, a, b);  // Return the larger value
}
```

## Where to Learn More

- **Detailed Operations Guide**: [Operations on encrypted types](../solidity-guides/operations/README.md)
- **Type System**: [Supported types](../solidity-guides/types.md)
- **Conditional Logic**: [Branching in FHE](../solidity-guides/logics/conditions.md)
- **Random Numbers**: [Generate random numbers](../solidity-guides/operations/random.md)
- **Performance**: [HCU (Homomorphic Computation Units)](../solidity-guides/hcu.md)
- **Access Control**: [ACL (Access Control List)](../solidity-guides/acl/README.md)

## Help and Support

- [Community forum](https://community.zama.ai/c/fhevm/15)
- [Discord channel](https://discord.com/invite/fhe-org)
- [GitHub Issues](https://github.com/zama-ai/fhevm/issues)
