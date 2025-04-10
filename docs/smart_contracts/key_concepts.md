# Key features

This document provides an overview of key features of the HTTPZ smart contract library.

### Configuration and initialization

Smart contracts using HTTPZ require proper configuration and initialization:

- **Environment setup**: Import and inherit from environment-specific configuration contracts
- **Gateway configuration**: Configure secure gateway access for cryptographic operations
- **Initialization checks**: Validate encrypted variables are properly initialized before use

For more information see [Configuration](configure.md).

### Encrypted data types

HTTPZ introduces encrypted data types compatible with Solidity:

- **Booleans**: `ebool`
- **Unsigned Integers**: `euint8`, `euint16`, `euint32`, `euint64`, `euint128`, `euint256`
- **Addresses**: `eaddress`
- **Bytes**: `ebytes64`, `ebytes128`, `ebytes256`
- **Input**: `einput` for handling encrypted input data

Encrypted data is represented as ciphertext handles, ensuring secure computation and interaction.

For more information see [use of encrypted types](types.md).

### Casting types

HTTPZ provides functions to cast between encrypted types:

- **Casting between encrypted types**: `TFHE.asEbool` converts encrypted integers to encrypted booleans
- **Casting to encrypted types**: `TFHE.asEuintX` converts plaintext values to encrypted types
- **Casting to encrypted addresses**: `TFHE.asEaddress` converts plaintext addresses to encrypted addresses
- **Casting to encrypted bytes**: `TFHE.asEbytesX` converts plaintext bytes to encrypted bytes

For more information see [use of encrypted types](types.md).

### Confidential computation

HTTPZ enables symbolic execution of encrypted operations, supporting:

- **Arithmetic:** `TFHE.add`, `TFHE.sub`, `TFHE.mul`, `TFHE.min`, `TFHE.max`, `TFHE.neg`, `TFHE.div`, `TFHE.rem`
  - Note: `div` and `rem` operations are supported only with plaintext divisors
- **Bitwise:** `TFHE.and`, `TFHE.or`, `TFHE.xor`, `TFHE.not`, `TFHE.shl`, `TFHE.shr`, `TFHE.rotl`, `TFHE.rotr`
- **Comparison:** `TFHE.eq`, `TFHE.ne`, `TFHE.lt`, `TFHE.le`, `TFHE.gt`, `TFHE.ge`
- **Advanced:** `TFHE.select` for branching on encrypted conditions, `TFHE.randEuintX` for on-chain randomness.

For more information on operations, see [Operations on encrypted types](operations.md).&#x20;

For more information on conditional branching, see [Conditional logic in FHE](conditions.md).&#x20;

For more information on random number generation, see [Generate Random Encrypted Numbers](random.md).

### Access control mechanism

HTTPZ enforces access control with a blockchain-based Access Control List (ACL):

- **Persistent access**: `TFHE.allow`, `TFHE.allowThis` grants permanent permissions for ciphertexts.
- **Transient access**: `TFHE.allowTransient` provides temporary access for specific transactions.
- **Validation**: `TFHE.isSenderAllowed` ensures that only authorized entities can interact with ciphertexts.

For more information see [ACL](acl).
