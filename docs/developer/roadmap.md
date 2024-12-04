# Development roadmap

This document gives an preview of the upcoming features of fhEVM. In addition to what's listed here, you can [submit your feature request](https://github.com/zama-ai/fhevm/issues/new?template=feature-request.md) on GitHub.

## Features

| Name             | Description                                               | ETA    |
| ---------------- | --------------------------------------------------------- | ------ |
| Foundry template | [Forge](https://book.getfoundry.sh/reference/forge/forge) | Q1 '25 |

## Operations

| Name                  | Function name      | Type               | ETA         |
| --------------------- | ------------------ | ------------------ | ----------- |
| Signed Integers       | `eintX`            |                    | Coming soon |
| Add w/ overflow check | `TFHE.safeAdd`     | Binary, Decryption | Coming soon |
| Sub w/ overflow check | `TFHE.safeSub`     | Binary, Decryption | Coming soon |
| Mul w/ overflow check | `TFHE.safeMul`     | Binary, Decryption | Coming soon |
| Random signed int     | `TFHE.randEintX()` | Random             | -           |
| Div                   | `TFHE.div`         | Binary             | -           |
| Rem                   | `TFHE.rem`         | Binary             | -           |
| Set inclusion         | `TFHE.isIn()`      | Binary             | -           |

{% hint style="info" %}
Random encrypted integers that are generated fully on-chain. Currently, implemented as a mockup by using a PRNG in the plain. Not for use in production!
{% endhint %}
