# Roadmap

## Operations

| name                  | function name       | type               | ETA             |
| --------------------- | ------------------- | ------------------ | --------------- |
| Add w/ overflow check | `TFHE.safeAdd`      | Binary, Decryption | Coming soon (1) |
| Sub w/ overflow check | `TFHE.safeSub`      | Binary, Decryption | Coming soon (1) |
| Mul w/ overflow check | `TFHE.safeMul`      | Binary, Decryption | Coming soon (1) |
| Div                   | `TFHE.div`          | Binary             | -               |
| Rem                   | `TFHE.rem`          | Binary             | -               |
| Random unsigned int   | `TFHE.randEuintX()` | Random             | -               |
| Random signed int     | `TFHE.randEintX()`  | Random             | -               |

> **_NOTE 1:_** Methods prefixed with `safe` will do an overflow check by decrypting an overflow bit and revert if that bit is true.

> **_NOTE 2:_** Random encrypted integers that are generated fully on-chain. Currently, implemented as a mockup by using a PRNG in the plain.
> Not for use in production!
