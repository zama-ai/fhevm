# Key features

This document provides an overview of key features of the fhevm smart contract library.

### Configuration and initialization

Smart contracts using fhevm require proper configuration and initialization:

- **Environment setup**: Import and inherit from environment-specific configuration contracts
- **Gateway configuration**: Configure secure gateway access for cryptographic operations
- **Initialization checks**: Validate encrypted variables are properly initialized before use

For more information see [Configuration](configure.md).

### Encrypted data types

fhevm introduces encrypted data types compatible with Solidity:

- **Booleans**: `ebool`
- **Unsigned Integers**: `euint8`, `euint16`, `euint32`, `euint64`, `euint128`, `euint256`
- **Addresses**: `eaddress`
- **Bytes**: `ebytes64`, `ebytes128`, `ebytes256`
- **Input**: `einput` for handling encrypted input data

Encrypted data is represented as ciphertext handles, ensuring secure computation and interaction.

For more information see [use of encrypted types](types.md).

### Casting types

fhevm provides functions to cast between encrypted types:

- **Casting between encrypted types**: `FHE.asEbool` converts encrypted integers to encrypted booleans
- **Casting to encrypted types**: `FHE.asEuintX` converts plaintext values to encrypted types
- **Casting to encrypted addresses**: `FHE.asEaddress` converts plaintext addresses to encrypted addresses
- **Casting to encrypted bytes**: `FHE.asEbytesX` converts plaintext bytes to encrypted bytes

For more information see [use of encrypted types](types.md).

### Confidential computation

fhevm enables symbolic execution of encrypted operations, supporting:

- **Arithmetic:** `FHE.add`, `FHE.sub`, `FHE.mul`, `FHE.min`, `FHE.max`, `FHE.neg`, `FHE.div`, `FHE.rem`
  - Note: `div` and `rem` operations are supported only with plaintext divisors
- **Bitwise:** `FHE.and`, `FHE.or`, `FHE.xor`, `FHE.not`, `FHE.shl`, `FHE.shr`, `FHE.rotl`, `FHE.rotr`
- **Comparison:** `FHE.eq`, `FHE.ne`, `FHE.lt`, `FHE.le`, `FHE.gt`, `FHE.ge`
- **Advanced:** `FHE.select` for branching on encrypted conditions, `FHE.randEuintX` for on-chain randomness.

For more information on operations, see [Operations on encrypted types](operations.md).&#x20;

For more information on conditional branching, see [Conditional logic in FHE](conditions.md).&#x20;

For more information on random number generation, see [Generate Random Encrypted Numbers](random.md).

### Access control mechanism

fhevm enforces access control with a blockchain-based Access Control List (ACL):

- **Persistent access**: `FHE.allow`, `FHE.allowThis` grants permanent permissions for ciphertexts.
- **Transient access**: `FHE.allowTransient` provides temporary access for specific transactions.
- **Validation**: `FHE.isSenderAllowed` ensures that only authorized entities can interact with ciphertexts.

For more information see [ACL](acl).
