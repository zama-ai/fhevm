# Operations

The `TFHE` library defines the following operations with FHE ciphertexts:

| name                         | function name       | symbol | type         |
| ---------------------------- | ------------------- | ------ | ------------ |
| Add                          | `TFHE.add`          | `+`    | Binary       |
| Sub                          | `TFHE.sub`          | `-`    | Binary       |
| Mul                          | `TFHE.mul`          | `*`    | Binary       |
| Div                          | `TFHE.div`          |        | Binary       |
| Rem                          | `TFHE.rem`          |        | Binary       |
| BitAnd                       | `TFHE.and`          | `&`    | Binary       |
| BitOr                        | `TFHE.or`           | `\|`   | Binary       |
| BitXor                       | `TFHE.xor`          | `^`    | Binary       |
| Shift Right                  | `TFHE.shr`          |        | Binary       |
| Shift Left                   | `TFHE.shl`          |        | Binary       |
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
| Cmux                         | `TFHE.cmux`         |        | Ternary      |
| Decrypt                      | `TFHE.decrypt()`    |        | Decryption   |
| Reencrypt                    | `TFHE.reencrypt()`  |        | Reencryption |
| Optimistic Require           | `TFHE.optReq()`     |        | Decryption   |
| Random unsigned int (mockup) | `TFHE.randEuintX()` |        | Random       |

> **_NOTE 1:_** Random encrypted integers that are generated fully on-chain. Currently, implemented as a mockup by using a PRNG in the plain.
> Not for use in production!

Overloaded operators `+`, `-`, `*`, `&`, ... on encrypted integers are supported ([using for](https://docs.soliditylang.org/en/v0.8.19/contracts.html#using-for)). As of now, overloaded operators will call the versions without an overflow check.

More information about the supported operations can be found in the [function specifications](functions.md) page or in the [TFHE-rs docs](https://docs.zama.ai/tfhe-rs/getting-started/operations#arithmetic-operations.).

If you find yourself in search of a missing feature, we encourage you to [consult our roadmap](roadmap.md) for upcoming developments. Alternatively, don't hesitate to reach out to us on Discord or visit our community forum.
