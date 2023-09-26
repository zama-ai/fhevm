# Operations

The library exposes utility functions for operations on TFHE ciphertexts.
The list of supported operations is presented below.

| name                  | function name    | type    |
| --------------------- | ---------------- | ------- |
| Add                   | `TFHE.add` or +  | Binary  |
| Sub                   | `TFHE.sub` or -  | Binary  |
| Mul                   | `TFHE.mul` or \* | Binary  |
| BitAnd                | `TFHE.and` or &  | Binary  |
| BitOr                 | `TFHE.or` or \|  | Binary  |
| BitXor                | `TFHE.xor` or ^  | Binary  |
| Shift Right           | `TFHE.shr`       | Binary  |
| Shift Left            | `TFHE.shl`       | Binary  |
| Equal                 | `TFHE.eq`        | Binary  |
| Not equal             | `TFHE.ne`        | Binary  |
| Greater than or equal | `TFHE.ge`        | Binary  |
| Greater than          | `TFHE.gt`        | Binary  |
| Less than or equal    | `TFHE.le`        | Binary  |
| Less than             | `TFHE.lt`        | Binary  |
| Min                   | `TFHE.min`       | Binary  |
| Max                   | `TFHE.max`       | Binary  |
| Neg                   | `TFHE.neg` or -  | Unary   |
| Not                   | `TFHE.not` or ~  | Unary   |
| Cmux                  | `TFHE.cmux`      | Ternary |

Note that you can directly use `+`, `-`, `*`, `&`, ... on encrypted integers ([using for](https://docs.soliditylang.org/en/v0.8.19/contracts.html#using-for)).

More information about the supported operations can be found at the [TFHE-rs docs](https://docs.zama.ai/tfhe-rs/getting-started/operations#arithmetic-operations.).
