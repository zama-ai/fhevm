# Operations

The `TFHE` library defines the following operations with FHE ciphertexts:

| name                  | function name       | symbol | type                     | supported                              |
| --------------------- | -------------       | ------ | ------------------------ | -------------------------------------- |
| Add                   | `TFHE.add`          | `+`    | Binary                   | yes                                    |
| Add w/ overflow Check | `TFHE.safeAdd`      |        | Binary, Decryption       | no, coming soon (1)                    |
| Sub                   | `TFHE.sub`          | `-`    | Binary                   | yes                                    |
| Sub w/ overflow Check | `TFHE.safeSub`      |        | Binary, Decryption       | no, coming soon (1)                    |
| Mul                   | `TFHE.mul`          | `*`    | Binary                   | yes                                    |
| Mul w/ overflow Check | `TFHE.safeMul`      |        | Binary, Decryption       | no, coming soon (1)                    |
| Div                   | `TFHE.div`          |        | Binary                   | yes, for plaintext divisors            |
| Rem                   | `TFHE.rem`          | `%`    | Binary                   | no, coming soon for plaintext divisors |
| BitAnd                | `TFHE.and`          | `&`    | Binary                   | yes                                    |
| BitOr                 | `TFHE.or`           | `\|`   | Binary                   | yes                                    |
| BitXor                | `TFHE.xor`          | `^`    | Binary                   | yes                                    |
| Shift Right           | `TFHE.shr`          |        | Binary                   | yes                                    |
| Shift Left            | `TFHE.shl`          |        | Binary                   | yes                                    |
| Equal                 | `TFHE.eq`           |        | Binary                   | yes                                    |
| Not equal             | `TFHE.ne`           |        | Binary                   | yes                                    |
| Greater than or equal | `TFHE.ge`           |        | Binary                   | yes                                    |
| Greater than          | `TFHE.gt`           |        | Binary                   | yes                                    |
| Less than or equal    | `TFHE.le`           |        | Binary                   | yes                                    |
| Less than             | `TFHE.lt`           |        | Binary                   | yes                                    |
| Min                   | `TFHE.min`          |        | Binary                   | yes                                    |
| Max                   | `TFHE.max`          |        | Binary                   | yes                                    |
| Neg                   | `TFHE.neg`          | `-`    | Unary                    | yes                                    |
| Not                   | `TFHE.not`          | `~`    | Unary                    | yes                                    |
| Cmux                  | `TFHE.cmux`         |        | Ternary                  | yes                                    |
| Decrypt               | `TFHE.decrypt()`    |        | Decryption               | yes                                    |
| Reencrypt             | `TFHE.reencrypt()`  |        | Reencryption             | yes                                    |
| Optimistic Require    | `TFHE.optReq()`     |        | Decryption               | yes                                    |
| Random unsigned int   | `TFHE.randEuintX()` |        | Random                   | yes, as a mockup (2)                   |
| Random signed int     | `TFHE.randEintX()`  |        | Random                   | no, coming soon as a mockup (2)        |

> **_NOTE 1:_** Methods prefixed with `safe` will do an overflow check by decrypting an overflow bit and revert if that bit is true.

> **_NOTE 2:_** Random encrypted integers that are generated fully on-chain. Currently, implemented as a mockup by using a PRNG in the plain.
Not for use in production!

Overloaded operators `+`, `-`, `*`, `&`, ... on encrypted integers are supported ([using for](https://docs.soliditylang.org/en/v0.8.19/contracts.html#using-for)). As of now, overloaded operators will call the versions without an overflow check.

More information about the supported operations can be found in the [function specifications](functions.md) page or in the [TFHE-rs docs](https://docs.zama.ai/tfhe-rs/getting-started/operations#arithmetic-operations.).
