# What is fhEVM

<figure><img src="../.gitbook/assets/doc_header_fhevm.png" alt=""><figcaption></figcaption></figure>

## Introduction

There used to be a dilemma in blockchain: keep your application and user data on-chain, allowing everyone to see it, or keep it privately off-chain and lose contract composability.

**fhEVM** is a technology that enables confidential smart contracts on the EVM using Fully Homomorphic Encryption (FHE).

Thanks to a breakthrough in FHE, Zama’s fhEVM makes it possible to run confidential smart contracts on encrypted data, guaranteeing both confidentiality and composability with:

- **End-to-end encryption of transactions and state**: Data included in transactions is encrypted and never visible to anyone.
- **Composability and data availability on-chain**: States are updated while remaining encrypted at all times.
- **No impact on existing dapps and state**: Encrypted state co-exists alongside public one, and doesn't impact existing dapps.

## Use cases

fhEVM enables powerful privacy-preserving applications across key blockchain sectors:

### 1. Decentralized Finance (DeFi)

- **Private Trading & AMMs**
  - Execute trades without revealing amounts or strategies
  - Prevent front-running and MEV exploitation
  - Create dark pools with hidden order books
- **Confidential Lending**
  - Keep collateral amounts and positions private
  - Protect borrowing history and credit scores
  - Enable private under-collateralized lending
- **Example**: [Confidential ERC20 tutorial](../getting_started/quick_start/overview.md)

### 2. Identity & Governance

- **Decentralized Identity**
  - Store encrypted credentials on-chain
  - Issue private attestations
  - Verify membership without revealing identity
- **Private Voting**
  - Cast encrypted votes without revealing choices
  - Prevent voter coercion and vote buying
  - Enable quadratic voting with private token balances
- **Example**: [Decentralized identity](https://github.com/zama-ai/dapps/tree/main/hardhat/contracts/decIdentity)

### 3. Enterprise Solutions

- **Supply Chain**
  - Track sensitive business metrics privately
  - Share data selectively with partners
  - Maintain competitive advantages on-chain
- **Data Markets**
  - Trade data while preserving confidentiality
  - Enable private computation services
  - Create subscription-based data access

### 4. Gaming & NFTs

- **Strategy Games**
  - Hide player moves and game state
  - Enable private bidding and trading of in-game assets
  - Implement truly random number generation
- **NFT Privacy**
  - Conceal ownership and transfer history
  - Keep metadata and attributes private
  - Enable sealed-bid NFT auctions
- **Example**: [FHE Wordle](https://github.com/zama-ai/dapps/tree/main/hardhat/contracts/fheWordle)

These applications showcase how fhEVM uniquely combines the transparency and composability of blockchain with the privacy guarantees of FHE, enabling a new generation of confidential smart contracts.
