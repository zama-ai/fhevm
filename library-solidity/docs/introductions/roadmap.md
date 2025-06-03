# Development roadmap

This document gives a preview of the upcoming features of fhevm. In addition to what's listed here, you can [submit your feature request](https://github.com/zama-ai/fhevm-solidity/issues/new?template=feature-request.md) on GitHub.

## Features

| Name             | Description                                               | ETA    |
| ---------------- | --------------------------------------------------------- | ------ |
| Foundry template | [Forge](https://book.getfoundry.sh/reference/forge/forge) | Q1 '25 |

## Operations

| Name                  | Function name     | Type               | ETA         |
| --------------------- | ----------------- | ------------------ | ----------- |
| Signed Integers       | `eintX`           |                    | Coming soon |
| Add w/ overflow check | `FHE.safeAdd`     | Binary, Decryption | Coming soon |
| Sub w/ overflow check | `FHE.safeSub`     | Binary, Decryption | Coming soon |
| Mul w/ overflow check | `FHE.safeMul`     | Binary, Decryption | Coming soon |
| Random signed int     | `FHE.randEintX()` | Random             | -           |
| Div                   | `FHE.div`         | Binary             | -           |
| Rem                   | `FHE.rem`         | Binary             | -           |
| Set inclusion         | `FHE.isIn()`      | Binary             | -           |

{% hint style="info" %}
Random encrypted integers that are generated fully on-chain. Currently, implemented as a mockup by using a PRNG in the plain. Not for use in production!
{% endhint %}
