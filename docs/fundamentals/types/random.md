# Generate random number

Random encrypted integers can be generated fully on-chain.

That can only be done during transactions and not on an `eth_call` RPC method,
because PRNG state needs to be mutated on-chain during generation.

## Example

```solidity
euint8 r8 = TFHE.randEuint8();
euint16 r16 = TFHE.randEuint16();
euint32 r32 = TFHE.randEuint32();
euint64 r64 = TFHE.randEuint64();
```
