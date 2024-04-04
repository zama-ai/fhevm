# Roadmap

## Features

| name                 | description                                | ETA    |
| -------------------- | ------------------------------------------ | ------ |
| 128bits (scalar)     | Add euint128 for scalar operations         | Q3 '24 |
| Proof for inputs     | Generate a proof for every encrypted input | Q3 '24 |
| Threshold decryption | Use threshold decryption                   | Q3 '24 |

## Operations

| name                  | function name       | type               | ETA             |
| --------------------- | ------------------- | ------------------ | --------------- |
| Random unsigned int   | `TFHE.randEuintX()` | Random             | Q3 '24          |
| Add w/ overflow check | `TFHE.safeAdd`      | Binary, Decryption | Coming soon (1) |
| Sub w/ overflow check | `TFHE.safeSub`      | Binary, Decryption | Coming soon (1) |
| Mul w/ overflow check | `TFHE.safeMul`      | Binary, Decryption | Coming soon (1) |
| Random signed int     | `TFHE.randEintX()`  | Random             | -               |
| Div                   | `TFHE.div`          | Binary             | -               |
| Rem                   | `TFHE.rem`          | Binary             | -               |
| Set inclusion         | `TFHE.isIn()`       | Binary             | -               |

> **_NOTE 1:_** Methods prefixed with `safe` will do an overflow check by decrypting an overflow bit and revert if that bit is true.

> **_NOTE 2:_** Random encrypted integers that are generated fully on-chain. Currently, implemented as a mockup by using a PRNG in the plain.
> Not for use in production!
