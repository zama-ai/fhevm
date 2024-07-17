# Operations on encrypted types

This document explains the supported operations with Fully Homomorphic Encryption (FHE) ciphertexts in the `TFHE` library.

## Arithmetic operations

| Name                    | Function name       | Symbol | Type    |
| ----------------------- | ------------------- | ------ | ------- |
| Add                     | `TFHE.add`          | `+`    | Binary  |
| Subtract                | `TFHE.sub`          | `-`    | Binary  |
| Multiply                | `TFHE.mul`          | `*`    | Binary  |
| Divide (plaintext divisor) | `TFHE.div`          |        | Binary  |
| Reminder (plaintext divisor) | `TFHE.rem`          |        | Binary  |
| Negation                     | `TFHE.neg`          | `-`    | Unary   |
| Min                     | `TFHE.min`          |        | Binary  |
| Max                     | `TFHE.max`          |        | Binary  |

## Bitwise operations
| Name                    | Function name       | Symbol | Type    |
| ----------------------- | ------------------- | ------ | ------- |
| Bitwise AND                  | `TFHE.and`          | `&`    | Binary  |
| Bitwise OR                   | `TFHE.or`           | `\|`   | Binary  |
| Bitwise XOR                  | `TFHE.xor`          | `^`    | Binary  |
| Bitwise NOT                     | `TFHE.not`          | `~`    | Unary   |
| Shift Right             | `TFHE.shr`          |        | Binary  |
| Shift Left              | `TFHE.shl`          |        | Binary  |
| Rotate Right            | `TFHE.rotr`         |        | Binary  |
| Rotate Left             | `TFHE.rotl`         |        | Binary  |

{% hint style="info" %}
 The shift operators `TFHE.shr` and `TFHE.shl` can take any encrypted type `euintX` as a first operand and either a `uint8`or a `euint8` as a second operand, however the second operand will always be computed modulo the number of bits of the first operand. For example, `TFHE.shr(euint64 x, 70)` is equivalent to `TFHE.shr(euint64 x, 6)` because `70 % 64 = 6`. This differs from the classical shift operators in Solidity, where there is no intermediate modulo operation, so for instance any `uint64` shifted right via `>>` would give a null result.
 {% endhinr %}

## Comparison operations
| Name                    | Function name       | Symbol | Type    |
| ----------------------- | ------------------- | ------ | ------- |
| Equal                   | `TFHE.eq`           |        | Binary  |
| Not equal               | `TFHE.ne`           |        | Binary  |
| Greater than or equal   | `TFHE.ge`           |        | Binary  |
| Greater than            | `TFHE.gt`           |        | Binary  |
| Less than or equal      | `TFHE.le`           |        | Binary  |
| Less than               | `TFHE.lt`           |        | Binary  |

## Ternary operation 
| Name                    | Function name       | Symbol | Type    |
| ----------------------- | ------------------- | ------ | ------- |
| Select                  | `TFHE.select`       |        | Ternary |

## Random operation
| Name                    | Function name       | Symbol | Type    |
| ----------------------- | ------------------- | ------ | ------- |
| Random unsigned int     | `TFHE.randEuintX()` |        | Random  |


## Overload operators
Overloaded operators such as `+`, `-`, `*`, and `&` on encrypted integers are supported with the [`using for`](https://docs.soliditylang.org/en/v0.8.22/contracts.html#using-for) syntax. As of now, overloaded operators call the versions without an overflow check.

For more information about the supported operations, refer to the [function specifications](../../references/functions.md) page or the [TFHE-rs docs](https://docs.zama.ai/tfhe-rs/getting-started/operations#arithmetic-operations.).

If you need a feature that is not currently available, please [consult our roadmap](../../developer/roadmap.md) for upcoming developments. You can also reach out to us on Discord or visit our community forum.

If you find yourself in search of a missing feature, we encourage you to [consult our roadmap](../../developer/roadmap.md) for upcoming developments. Alternatively, don't hesitate to reach out to us on [Discord](https://discord.com/invite/fhe-org) or visit our [Community Forum](https://community.zama.ai/c/fhevm/15).
