# FHEVM developer overview

This document gives a high–level orientation for developers who are new to the FHEVM monorepo and the broader Zama ecosystem.

It does not replace the official documentation at https://docs.zama.org/protocol, but should make it easier to know where to look for what you need.

---

## 1. Monorepo layout (what lives here?)

This repository is the core framework of the Zama Confidential Blockchain Protocol and is organized into several major areas:

### 1.1 Contracts

- `gateway-contracts/`  
  Smart contracts that manage the gateway between on–chain and off–chain components of the protocol.

- `host-contracts/`  
  Smart contracts deployed on the host chain that orchestrate FHE workflows and coordinate cross–component interactions.

- `protocol-contracts/`  
  Protocol–level contracts that define how the different pieces of the FHEVM protocol work together on-chain.

- `library-solidity/`  
  Solidity libraries that expose encrypted types and helper APIs used by FHEVM smart contracts.

### 1.2 Compute engines and services

- `coprocessor/`  
  Rust-based coprocessor implementation that performs heavy FHE computations off–chain.

- `kms-connector/`  
  Interface for integrating with Key Management Services (KMS) that manage decryption keys and multi–party computation (MPC).

- `sdk/rust-sdk/`  
  Rust SDK utilities that help services and infrastructure interact with the FHEVM stack.

### 1.3 Utilities and infrastructure

- `charts/`  
  Helm charts and deployment configuration for running the FHEVM stack in Kubernetes environments.

- `golden-container-images/`  
  Docker “golden images” for Node.js and Rust that are used as base images in CI and deployment.

- `test-suite/`  
  Docker-compose integration tests that exercise the end-to-end FHEVM stack.

- `.github/`, `ci/` and other dotfiles  
  Continuous integration, linting and repository-wide tooling configuration.

If you are unsure where to contribute, starting from documentation, small test improvements, or CI tooling is usually the safest option.

---

## 2. When should I use another repository?

The FHEVM monorepo is not the only place where Zama hosts code. Depending on what you are building, you may want one of the companion repositories instead.

### 2.1 Writing contracts in Solidity

If your main goal is to **write confidential smart contracts in Solidity**, you will probably want:

- `zama-ai/fhevm-solidity` – Solidity library that exposes encrypted types and helper functions for smart contracts that run on an FHEVM-compatible chain.

This repo (the FHEVM monorepo) provides the underlying protocol and infrastructure, while `fhevm-solidity` focuses on the developer-facing contract library.

### 2.2 Building frontends and dApps

If you are building a full dApp with a frontend, these templates can be more convenient:

- `zama-ai/fhevm-react-template` – a minimal React / Next.js template for interacting with FHEVM-powered contracts from a web UI.
- `zama-ai/fhevm-hardhat-template` – a Hardhat-based template for building and testing FHEVM-enabled Solidity projects.

You can start from one of those templates and then connect to a network that runs this FHEVM stack.

### 2.3 Node / chain integrations

If you are working on **integrating FHEVM into an EVM-compatible blockchain implementation**, you may need:

- `zama-ai/fhevm-go` – a Go library that helps EVM maintainers integrate FHEVM into their chain nodes.
- The `coprocessor/` and `kms-connector/` directories in this monorepo – the services that handle encrypted computation and key management.

In this case, the FHEVM monorepo is the main place where the protocol and infrastructure are developed.

### 2.4 Local development and mock testing

If you want to experiment with FHEVM contracts **without** running a full FHE stack:

- `zama-ai/fhevm-mocks` – tools and plugins for developing and testing FHEVM contracts in “mock mode”.

This is useful for local development loops and unit testing before integrating with the full stack.

---

## 3. Where to read more

For deeper explanations, see:

- The main FHEVM README in this repository.
- The protocol documentation at https://docs.zama.org/protocol.
- The FHEVM whitepaper linked from the README.
- The Awesome Zama list with additional FHE and FHEVM resources.

If you are contributing for the first time, make sure to also read the “Contributing” section in the main README and follow the instructions about becoming an approved contributor.
