# FHEVM stack quickstart

This document gives a short, opinionated path for developers who want to get something working with FHEVM as quickly as possible.

It is meant as a navigation aid on top of the official documentation, not a replacement.

---

## 1. If you are completely new to the Zama Protocol

1. Read the Litepaper and protocol overview in the official docs.  
   This explains the high-level design and why FHEVM exists.

2. Skim the “FHE library” / FHEVM library overview.  
   This shows how encrypted types and operations are exposed to Solidity developers.

At this stage you do not need to understand every cryptographic detail; focus on the big picture.

---

## 2. Write your first confidential smart contract

A good starting flow is:

1. Follow the “Quick Start” section in the protocol docs to write a minimal confidential contract.
2. Use either:
   - the Hardhat template repository for a preconfigured development setup, or
   - the Solidity library repository if you want to integrate FHEVM into an existing project.

Make sure you can:

- compile the contract,
- deploy it to a local environment, and
- see at least one encrypted value flowing through the system.

---

## 3. Add a frontend or client

Once a basic contract works, you can:

1. Look at the React template repository to see how to build a minimal dApp that:
   - connects a wallet,
   - encrypts user input on the client,
   - sends encrypted transactions to the FHEVM network.

2. Alternatively, explore the SDKs (such as JavaScript or Rust SDKs) for non-browser clients and backend services.

The goal of this step is simply to confirm end-to-end flow from user input to encrypted on-chain state.

---

## 4. Learn the architecture in depth

When you are ready to go deeper:

1. Read the protocol sections that explain:
   - the coprocessor,
   - the gateway contracts,
   - the KMS connector,
   - how encrypted computation is scheduled off-chain and synchronized back on-chain.

2. Compare that with the directories in this repository:
   - `gateway-contracts/`
   - `host-contracts/`
   - `coprocessor/`
   - `kms-connector/`
   - `test-suite/`

This makes it easier to navigate the codebase when you are debugging or contributing.

---

## 5. Next steps

Depending on your role:

- **Smart contract developer**  
  Focus on the Solidity library, examples, and the contracts-related directories. Consider building a small demo dApp end to end.

- **Protocol / infrastructure engineer**  
  Explore the coprocessor, KMS connector, and test-suite. Try running the integration tests locally and reading the code paths they exercise.

- **Researcher / architect**  
  Read the whitepaper and related documents, then study how the on-chain and off-chain components are wired together in this monorepo.

For contribution guidelines, always check the main README and follow the instructions about becoming an approved contributor before sending pull requests.
