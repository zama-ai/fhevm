# Generate random number

This document shows you how to generate random encrypted integers fully on-chain.

{% hint style="info" %}
This operation must be performed during transactions, as it requires the pseudo-random number generator (PRNG) state to be mutated on-chain during generation. Therefore, it cannot be executed using the `eth_call` RPC method.
{% endhint %}

## Example

```solidity
euint8 r8 = TFHE.randEuint8();
euint16 r16 = TFHE.randEuint16();
euint32 r32 = TFHE.randEuint32();
euint64 r64 = TFHE.randEuint64();
```
