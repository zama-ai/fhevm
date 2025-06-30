# Operations on encrypted types

This document outlines the operations supported on encrypted types in the `FHE` library, enabling arithmetic, bitwise, comparison, and more on Fully Homomorphic Encryption (FHE) ciphertexts.

## Arithmetic operations

The following arithmetic operations are supported for encrypted integers (`euintX`):

| Name                         | Function name | Symbol | Type   |
| ---------------------------- | ------------- | ------ | ------ |
| Add                          | `FHE.add`     | `+`    | Binary |
| Subtract                     | `FHE.sub`     | `-`    | Binary |
| Multiply                     | `FHE.mul`     | `*`    | Binary |
| Divide (plaintext divisor)   | `FHE.div`     |        | Binary |
| Reminder (plaintext divisor) | `FHE.rem`     |        | Binary |
| Negation                     | `FHE.neg`     | `-`    | Unary  |
| Min                          | `FHE.min`     |        | Binary |
| Max                          | `FHE.max`     |        | Binary |

{% hint style="info" %} 
Division (FHE.div) and remainder (FHE.rem) operations are currently supported only with plaintext divisors. 
{% endhint %}

## Bitwise operations

The FHE library also supports bitwise operations, including shifts and rotations:

| Name         | Function name | Symbol | Type   |
| ------------ | ------------- | ------ | ------ |
| Bitwise AND  | `FHE.and`     | `&`    | Binary |
| Bitwise OR   | `FHE.or`      | `\|`   | Binary |
| Bitwise XOR  | `FHE.xor`     | `^`    | Binary |
| Bitwise NOT  | `FHE.not`     | `~`    | Unary  |
| Shift Right  | `FHE.shr`     |        | Binary |
| Shift Left   | `FHE.shl`     |        | Binary |
| Rotate Right | `FHE.rotr`    |        | Binary |
| Rotate Left  | `FHE.rotl`    |        | Binary |

The shift operators `FHE.shr` and `FHE.shl` can take any encrypted type `euintX` as a first operand and either a `uint8`or a `euint8` as a second operand, however the second operand will always be computed modulo the number of bits of the first operand. For example, `FHE.shr(euint64 x, 70)` is equivalent to `FHE.shr(euint64 x, 6)` because `70 % 64 = 6`. This differs from the classical shift operators in Solidity, where there is no intermediate modulo operation, so for instance any `uint64` shifted right via `>>` would give a null result.

## Comparison operations

Encrypted integers can be compared using the following functions:

| Name                  | Function name | Symbol | Type   |
| --------------------- | ------------- | ------ | ------ |
| Equal                 | `FHE.eq`      |        | Binary |
| Not equal             | `FHE.ne`      |        | Binary |
| Greater than or equal | `FHE.ge`      |        | Binary |
| Greater than          | `FHE.gt`      |        | Binary |
| Less than or equal    | `FHE.le`      |        | Binary |
| Less than             | `FHE.lt`      |        | Binary |

## Ternary operation

The `FHE.select` function is a ternary operation that selects one of two encrypted values based on an encrypted condition:

| Name   | Function name | Symbol | Type    |
| ------ | ------------- | ------ | ------- |
| Select | `FHE.select`  |        | Ternary |

## Random operations

You can generate cryptographically secure random numbers fully on-chain:

<table data-header-hidden><thead><tr><th></th><th width="206"></th><th></th><th></th></tr></thead><tbody><tr><td><strong>Name</strong></td><td><strong>Function Name</strong></td><td><strong>Symbol</strong></td><td><strong>Type</strong></td></tr><tr><td>Random Unsigned Integer</td><td><code>FHE.randEuintX()</code></td><td></td><td>Random</td></tr></tbody></table>

For more details, refer to the [Random Encrypted Numbers](random.md) document.

## Best Practices

Here are some best practices to follow when using encrypted operations in your smart contracts:

### Use the appropriate encrypted type size

Choose the smallest encrypted type that can accommodate your data to optimize gas costs. For example, use `euint8` for small numbers (0-255) rather than `euint256`.

❌ Avoid using oversized types:

```solidity
// Bad: Using euint256 for small numbers wastes gas
euint64 age = FHE.asEuint128(25);  // age will never exceed 255
euint64 percentage = FHE.asEuint128(75);  // percentage is 0-100
```

✅ Instead, use the smallest appropriate type:

```solidity
// Good: Using appropriate sized types
euint8 age = FHE.asEuint8(25);  // age fits in 8 bits
euint8 percentage = FHE.asEuint8(75);  // percentage fits in 8 bits
```

### Use scalar operands when possible to save gas

Some FHE operators exist in two versions: one where all operands are ciphertexts handles, and another where one of the operands is an unencrypted scalar. Whenever possible, use the scalar operand version, as this will save a lot of gas.

❌ For example, this snippet cost way more in gas:

```solidity
euint32 x;
...
x = FHE.add(x,FHE.asEuint(42));
```

✅ Than this one:

```solidity
euint32 x;
// ...
x = FHE.add(x,42);
```

Despite both leading to the same encrypted result!

### Beware of overflows of FHE arithmetic operators

FHE arithmetic operators can overflow. Do not forget to take into account such a possibility when implementing FHEVM smart contracts.

❌ For example, if you wanted to create a mint function for an encrypted ERC20 token with an encrypted `totalSupply` state variable, this code is vulnerable to overflows:

```solidity
function mint(externalEuint32 encryptedAmount, bytes calldata inputProof) public {
  euint32 mintedAmount = FHE.asEuint32(encryptedAmount, inputProof);
  totalSupply = FHE.add(totalSupply, mintedAmount);
  balances[msg.sender] = FHE.add(balances[msg.sender], mintedAmount);
  FHE.allowThis(balances[msg.sender]);
  FHE.allow(balances[msg.sender], msg.sender);
}
```

✅ But you can fix this issue by using `FHE.select` to cancel the mint in case of an overflow:

```solidity
function mint(externalEuint32 encryptedAmount, bytes calldata inputProof) public {
  euint32 mintedAmount = FHE.asEuint32(encryptedAmount, inputProof);
  euint32 tempTotalSupply = FHE.add(totalSupply, mintedAmount);
  ebool isOverflow = FHE.lt(tempTotalSupply, totalSupply);
  totalSupply = FHE.select(isOverflow, totalSupply, tempTotalSupply);
  euint32 tempBalanceOf = FHE.add(balances[msg.sender], mintedAmount);
  balances[msg.sender] = FHE.select(isOverflow, balances[msg.sender], tempBalanceOf);
  FHE.allowThis(balances[msg.sender]);
  FHE.allow(balances[msg.sender], msg.sender);
}
```

Notice that we did not check separately the overflow on `balances[msg.sender]` but only on `totalSupply` variable, because `totalSupply` is the sum of the balances of all the users, so `balances[msg.sender]` could never overflow if `totalSupply` did not.


