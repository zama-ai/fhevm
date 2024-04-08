# Operations on encrypted types

The `TFHE` library defines the following operations with FHE ciphertexts:

| name                         | function name       | symbol | type         |
| ---------------------------- | ------------------- | ------ | ------------ |
| Add                          | `TFHE.add`          | `+`    | Binary       |
| Sub                          | `TFHE.sub`          | `-`    | Binary       |
| Mul                          | `TFHE.mul`          | `*`    | Binary       |
| Div (plaintext divisor)      | `TFHE.div`          |        | Binary       |
| Rem (plaintext divisor)      | `TFHE.rem`          |        | Binary       |
| BitAnd                       | `TFHE.and`          | `&`    | Binary       |
| BitOr                        | `TFHE.or`           | `\|`   | Binary       |
| BitXor                       | `TFHE.xor`          | `^`    | Binary       |
| Shift Right                  | `TFHE.shr`          |        | Binary       |
| Shift Left                   | `TFHE.shl`          |        | Binary       |
| Rotate Right                 | `TFHE.rotr`         |        | Binary       |
| Rotate Left                  | `TFHE.rotl`         |        | Binary       |
| Equal                        | `TFHE.eq`           |        | Binary       |
| Not equal                    | `TFHE.ne`           |        | Binary       |
| Greater than or equal        | `TFHE.ge`           |        | Binary       |
| Greater than                 | `TFHE.gt`           |        | Binary       |
| Less than or equal           | `TFHE.le`           |        | Binary       |
| Less than                    | `TFHE.lt`           |        | Binary       |
| Min                          | `TFHE.min`          |        | Binary       |
| Max                          | `TFHE.max`          |        | Binary       |
| Neg                          | `TFHE.neg`          | `-`    | Unary        |
| Not                          | `TFHE.not`          | `~`    | Unary        |
| Select                       | `TFHE.select`       |        | Ternary      |
| Decrypt                      | `TFHE.decrypt()`    |        | Decryption   |
| Reencrypt                    | `TFHE.reencrypt()`  |        | Reencryption |
| Random unsigned int (mockup) | `TFHE.randEuintX()` |        | Random       |

> _**NOTE 1:**_ Random encrypted integers that are generated fully on-chain. Currently, implemented as a mockup by using a PRNG in the plain. Not for use in production!

> **_NOTE 2:_** The shift operators `TFHE.shr` and `TFHE.shl` can take any encrypted type `euintX` as a first operand and either a `uint8`or a `euint8` as a second operand, however the second operand will always be computed modulo the number of bits of the first operand. For example, `TFHE.shr(euint64 x, 70)` will actually be equal to `TFHE.shr(euint64 x, 6)` because `70 % 64 = 6`. This is in contrast to the classical shift operators in Solidity where there is no intermediate modulo operation, so for instance any `uint64` shifted right via `>>` would give a null result.

Overloaded operators `+`, `-`, `*`, `&`, ... on encrypted integers are supported ([using for](https://docs.soliditylang.org/en/v0.8.22/contracts.html#using-for)). As of now, overloaded operators will call the versions without an overflow check.

More information about the supported operations can be found in the [function specifications](../references/functions.md) page or in the [TFHE-rs docs](https://docs.zama.ai/tfhe-rs/getting-started/operations#arithmetic-operations.).

If you find yourself in search of a missing feature, we encourage you to [consult our roadmap](../developer/roadmap.md) for upcoming developments. Alternatively, don't hesitate to reach out to us on Discord or visit our community forum.
