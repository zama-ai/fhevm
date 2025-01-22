# Generate random numbers

This document explains how to generate cryptographically secure random encrypted numbers fully on-chain using the `TFHE` library in fhEVM. These numbers are encrypted and remain confidential, enabling privacy-preserving smart contract logic.

## **Key notes on random number generation**

- **On-chain execution**: Random number generation must be executed during a transaction, as it requires the pseudo-random number generator (PRNG) state to be updated on-chain. This operation cannot be performed using the `eth_call` RPC method.
- **Cryptographic security**: The generated random numbers are cryptographically secure and encrypted, ensuring privacy and unpredictability.

{% hint style="info" %}
Random number generation must be performed during transactions, as it requires the pseudo-random number generator (PRNG) state to be mutated on-chain. Therefore, it cannot be executed using the `eth_call` RPC method.
{% endhint %}

## **Basic usage**

The `TFHE` library allows you to generate random encrypted numbers of various bit sizes. Below is a list of supported types and their usage:

```solidity
// Generate random encrypted numbers
ebool rb = TFHE.randEbool();       // Random encrypted boolean
euint4 r4 = TFHE.randEuint4();     // Random 4-bit number
euint8 r8 = TFHE.randEuint8();     // Random 8-bit number
euint16 r16 = TFHE.randEuint16();  // Random 16-bit number
euint32 r32 = TFHE.randEuint32();  // Random 32-bit number
euint64 r64 = TFHE.randEuint64();  // Random 64-bit number
euint128 r128 = TFHE.randEuint128(); // Random 128-bit number
euint256 r256 = TFHE.randEuint256(); // Random 256-bit number
```

### **Example: Random Boolean**

```solidity
function randomBoolean() public returns (ebool) {
  return TFHE.randEbool();
}
```

## **Bounded random numbers**

To generate random numbers within a specific range, you can specify an **upper bound**. The random number will be in the range `[0, upperBound - 1]`.

```solidity
// Generate random numbers with upper bounds
euint8 r8 = TFHE.randEuint8(100);      // Random number between 0-99
euint16 r16 = TFHE.randEuint16(1000);  // Random number between 0-999
euint32 r32 = TFHE.randEuint32(1000000); // Random number between 0-999999
```

### **Example: Random bumber with upper bound**

```solidity
function randomBoundedNumber(uint16 upperBound) public returns (euint16) {
  return TFHE.randEuint16(upperBound);
}
```

## **Random encrypted bytes**

To generate larger random values, you can use encrypted bytes. These are ideal for scenarios requiring high-precision or high-entropy data.

```solidity
// Generate random encrypted bytes
ebytes64 rb64 = TFHE.randEbytes64();    // 64 bytes (512 bits)
ebytes128 rb128 = TFHE.randEbytes128(); // 128 bytes (1024 bits)
ebytes256 rb256 = TFHE.randEbytes256(); // 256 bytes (2048 bits)
```

### **Example: Random Bytes**

```solidity
function randomBytes256() public returns (ebytes256) {
  return TFHE.randEbytes256();
}
```

## **Security Considerations**

- **Cryptographic security**:\
  The random numbers are generated using a cryptographically secure pseudo-random number generator (CSPRNG) and remain encrypted until explicitly decrypted.
- **Gas consumption**:\
  Each call to a random number generation function consumes gas. Developers should optimize the use of these functions, especially in gas-sensitive contracts.
- **Privacy guarantee**:\
  Random values are fully encrypted, ensuring they cannot be accessed or predicted by unauthorized parties.
