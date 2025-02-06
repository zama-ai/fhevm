# Operations on encrypted types

This document outlines the operations supported on encrypted types in the `TFHE` library, enabling arithmetic, bitwise, comparison, and more on Fully Homomorphic Encryption (FHE) ciphertexts.

## Arithmetic operations

The following arithmetic operations are supported for encrypted integers (`euintX`):

| Name                         | Function name | Symbol | Type   |
| ---------------------------- | ------------- | ------ | ------ |
| Add                          | `TFHE.add`    | `+`    | Binary |
| Subtract                     | `TFHE.sub`    | `-`    | Binary |
| Multiply                     | `TFHE.mul`    | `*`    | Binary |
| Divide (plaintext divisor)   | `TFHE.div`    |        | Binary |
| Reminder (plaintext divisor) | `TFHE.rem`    |        | Binary |
| Negation                     | `TFHE.neg`    | `-`    | Unary  |
| Min                          | `TFHE.min`    |        | Binary |
| Max                          | `TFHE.max`    |        | Binary |

{% hint style="info" %}
Division (TFHE.div) and remainder (TFHE.rem) operations are currently supported only with plaintext divisors.
{% endhint %}

## Bitwise operations

The TFHE library also supports bitwise operations, including shifts and rotations:

| Name         | Function name | Symbol | Type   |
| ------------ | ------------- | ------ | ------ |
| Bitwise AND  | `TFHE.and`    | `&`    | Binary |
| Bitwise OR   | `TFHE.or`     | `\|`   | Binary |
| Bitwise XOR  | `TFHE.xor`    | `^`    | Binary |
| Bitwise NOT  | `TFHE.not`    | `~`    | Unary  |
| Shift Right  | `TFHE.shr`    |        | Binary |
| Shift Left   | `TFHE.shl`    |        | Binary |
| Rotate Right | `TFHE.rotr`   |        | Binary |
| Rotate Left  | `TFHE.rotl`   |        | Binary |

The shift operators `TFHE.shr` and `TFHE.shl` can take any encrypted type `euintX` as a first operand and either a `uint8`or a `euint8` as a second operand, however the second operand will always be computed modulo the number of bits of the first operand. For example, `TFHE.shr(euint64 x, 70)` is equivalent to `TFHE.shr(euint64 x, 6)` because `70 % 64 = 6`. This differs from the classical shift operators in Solidity, where there is no intermediate modulo operation, so for instance any `uint64` shifted right via `>>` would give a null result.

## Comparison operations

Encrypted integers can be compared using the following functions:

| Name                  | Function name | Symbol | Type   |
| --------------------- | ------------- | ------ | ------ |
| Equal                 | `TFHE.eq`     |        | Binary |
| Not equal             | `TFHE.ne`     |        | Binary |
| Greater than or equal | `TFHE.ge`     |        | Binary |
| Greater than          | `TFHE.gt`     |        | Binary |
| Less than or equal    | `TFHE.le`     |        | Binary |
| Less than             | `TFHE.lt`     |        | Binary |

## Ternary operation

The `TFHE.select` function is a ternary operation that selects one of two encrypted values based on an encrypted condition:

| Name   | Function name | Symbol | Type    |
| ------ | ------------- | ------ | ------- |
| Select | `TFHE.select` |        | Ternary |

## Random operations

You can generate cryptographically secure random numbers fully on-chain:

<table data-header-hidden><thead><tr><th></th><th width="206"></th><th></th><th></th></tr></thead><tbody><tr><td><strong>Name</strong></td><td><strong>Function Name</strong></td><td><strong>Symbol</strong></td><td><strong>Type</strong></td></tr><tr><td>Random Unsigned Integer</td><td><code>TFHE.randEuintX()</code></td><td></td><td>Random</td></tr></tbody></table>

For more details, refer to the [Random Encrypted Numbers](random.md) document.

## Overload operators

The `TFHE` library supports operator overloading for encrypted integers (e.g., `+`, `-`, `*`, `&`) using the Solidity [`using for`](https://docs.soliditylang.org/en/v0.8.22/contracts.html#using-for) syntax. These overloaded operators currently perform unchecked operations, meaning they do not include overflow checks.

**Example**\
Overloaded operators make code more concise:

```solidity
euint64 a = TFHE.asEuint64(42);
euint64 b = TFHE.asEuint64(58);
euint64 sum = a + b; // Calls TFHE.add under the hood
```

## Best Practices

Here are some best practices to follow when using encrypted operations in your smart contracts:

### Use the appropriate encrypted type size

Choose the smallest encrypted type that can accommodate your data to optimize gas costs. For example, use `euint8` for small numbers (0-255) rather than `euint256`.

❌ Avoid using oversized types:

```solidity
// Bad: Using euint256 for small numbers wastes gas
euint64 age = TFHE.euint256(25);  // age will never exceed 255
euint64 percentage = TFHE.euint256(75);  // percentage is 0-100
```

✅ Instead, use the smallest appropriate type:

```solidity
// Good: Using appropriate sized types
euint8 age = TFHE.asEuint8(25);  // age fits in 8 bits
euint8 percentage = TFHE.asEuint8(75);  // percentage fits in 8 bits
```

### Use scalar operands when possible to save gas

Some TFHE operators exist in two versions : one where all operands are ciphertexts handles, and another where one of the operands is an unencrypted scalar. Whenever possible, use the scalar operand version, as this will save a lot of gas.

❌ For example, this snippet cost way more in gas:

```solidity
euint32 x;
...
x = TFHE.add(x,TFHE.asEuint(42));
```

✅ Than this one:

```solidity
euint32 x;
// ...
x = TFHE.add(x,42);
```

Despite both leading to the same encrypted result!

### Beware of overflows of TFHE arithmetic operators

TFHE arithmetic operators can overflow. Do not forget to take into account such a possibility when implementing fhEVM smart contracts.

❌ For example, if you wanted to create a mint function for an encrypted ERC20 tokens with an encrypted `totalSupply` state variable, this code is vulnerable to overflows:

```solidity
function mint(einput encryptedAmount, bytes calldata inputProof) public {
  euint32 mintedAmount = TFHE.asEuint32(encryptedAmount, inputProof);
  totalSupply = TFHE.add(totalSupply, mintedAmount);
  balances[msg.sender] = TFHE.add(balances[msg.sender], mintedAmount);
  TFHE.allowThis(balances[msg.sender]);
  TFHE.allow(balances[msg.sender], msg.sender);
}
```

✅ But you can fix this issue by using `TFHE.select` to cancel the mint in case of an overflow:

```solidity
function mint(einput encryptedAmount, bytes calldata inputProof) public {
  euint32 mintedAmount = TFHE.asEuint32(encryptedAmount, inputProof);
  euint32 tempTotalSupply = TFHE.add(totalSupply, mintedAmount);
  ebool isOverflow = TFHE.lt(tempTotalSupply, totalSupply);
  totalSupply = TFHE.select(isOverflow, totalSupply, tempTotalSupply);
  euint32 tempBalanceOf = TFHE.add(balances[msg.sender], mintedAmount);
  balances[msg.sender] = TFHE.select(isOverflow, balances[msg.sender], tempBalanceOf);
  TFHE.allowThis(balances[msg.sender]);
  TFHE.allow(balances[msg.sender], msg.sender);
}
```

Notice that we did not check separately the overflow on `balances[msg.sender]` but only on `totalSupply` variable, because `totalSupply` is the sum of the balances of all the users, so `balances[msg.sender]` could never overflow if `totalSupply` did not.

## Additional Resources

- For detailed API specifications, visit the [fhEVM API Documentation](../references/functions.md).
- Check our [Roadmap](../developer/roadmap.md) for upcoming features or submit a feature request on [GitHub](https://github.com/zama-ai/fhevm/issues/new?template=feature-request.md).
- Join the discussion on the [Community Forum](https://community.zama.ai/c/fhevm/15).

{% hint style="success" %}
**Zama 5-Question Developer Survey**

We want to hear from you! Take 1 minute to share your thoughts and helping us enhance our documentation and libraries. **👉** [**Click here**](https://www.zama.ai/developer-survey) to participate.
{% endhint %}
