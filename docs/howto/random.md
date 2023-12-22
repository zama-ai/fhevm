# Generate random number

Random encrypted integers can be generated fully on-chain.

That can only be done during transactions and not on an `eth_call` RPC method,
because PRNG state needs to be mutated on-chain during generation.

> **_WARNING:_** Not for use in production! Currently, integers are generated
> in the plain via a PRNG whose seed and state are public, with the state being
> on-chain. An FHE-based PRNG is coming soon, where the seed and state will be
> encrypted.

## Example

```solidity
euint8 r8 = TFHE.randEuint8();
euint16 r16 = TFHE.randEuint16();
euint32 r32 = TFHE.randEuint32();
```
