# Logics

This section covers how to implement conditional logic and control flow when working with encrypted values in FHEVM.

Since encrypted values cannot be directly evaluated at runtime, standard Solidity control flow (`if`, `else`, `for` with encrypted conditions) does not work with FHE ciphertexts. Instead, FHEVM provides specialized functions and patterns to handle these cases securely.

## Topics

- [**Branching**](conditions.md) — How to use `FHE.select` for conditional logic on encrypted values, and how to transition from encrypted conditions to non-encrypted business logic via public decryption.
- [**Dealing with branches and conditions**](loop.md) — Patterns for handling loops and indexed access when the condition or index is encrypted.
- [**Error handling**](error_handling.md) — How to handle errors in FHE computations, where standard `require` and `revert` cannot operate on encrypted values.
