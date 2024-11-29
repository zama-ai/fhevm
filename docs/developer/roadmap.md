# Roadmap

## Features

| name             | description                                                 | ETA    |
| ---------------- | ----------------------------------------------------------- | ------ |
| Foundry template | [ Forge ](https://book.getfoundry.sh/reference/forge/forge) | Q1 '25 |

## Operations

| name                  | function name      | type               | ETA         |
| --------------------- | ------------------ | ------------------ | ----------- |
| Signed Integers       | `eintX`            |                    | Coming soon |
| Add w/ overflow check | `TFHE.safeAdd`     | Binary, Decryption | Coming soon |
| Sub w/ overflow check | `TFHE.safeSub`     | Binary, Decryption | Coming soon |
| Mul w/ overflow check | `TFHE.safeMul`     | Binary, Decryption | Coming soon |
| Random signed int     | `TFHE.randEintX()` | Random             | -           |
| Div                   | `TFHE.div`         | Binary             | -           |
| Rem                   | `TFHE.rem`         | Binary             | -           |
| Set inclusion         | `TFHE.isIn()`      | Binary             | -           |

> **_NOTE 1:_** Random encrypted integers that are generated fully on-chain. Currently, implemented as a mockup by using a PRNG in the plain.
> Not for use in production!
