# Executive Summary

**FHEVM** is the core framework of the Zama Confidential Blockchain Protocol. It enables **confidential smart contracts on EVM-compatible blockchains** by leveraging Fully Homomorphic Encryption (FHE), allowing encrypted data to be processed directly on-chain without ever being decrypted.

## Key Guarantees

- **End-to-end encryption**: Transaction data and state remain encrypted at all times
- **Composability**: Encrypted state coexists with public state, enabling complex DeFi and application logic
- **No impact on existing dApps**: Confidential features are additive; existing applications continue to function

## Core Innovation

FHEVM uses **symbolic execution with asynchronous computation**:

1. FHE operations execute **symbolically on-chain** (fast, deterministic, cheap)
2. Actual FHE computation happens **asynchronously off-chain** via the coprocessor
3. Results are verified and committed back to the chain

This architecture separates the slow cryptographic work from blockchain consensus, enabling practical FHE on Ethereum-compatible chains.

## Use Cases

- **Confidential token transfers** - Private balances without mixers
- **Blind auctions** - Hidden bids until reveal
- **On-chain games** - Hidden cards, moves, selections
- **Encrypted DIDs and attestations** - Private identity credentials
- **Confidential voting** - Anti-bribery, anti-coercion mechanisms

## What Makes This Possible

The breakthrough is the **separation of concerns**:

- **On-chain**: Fast symbolic execution generates deterministic handles
- **Off-chain**: Heavy FHE computation happens asynchronously
- **Verification**: Results are cryptographically verified before commitment

This enables confidential smart contracts to run at practical speeds on standard EVM chains without requiring specialized consensus mechanisms.

---

**Next:** Learn the [Key Concepts](key-concepts.md) essential to understanding FHEVM →
