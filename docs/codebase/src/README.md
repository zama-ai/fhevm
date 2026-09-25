# FHEVM Codebase Documentation

> **Version**: 1.0 | **Last Updated**: December 2025
> **Purpose**: Comprehensive technical documentation for developers working with or rebuilding the FHEVM codebase

---

## Welcome

This documentation provides a complete technical overview of the **FHEVM** (Fully Homomorphic Encryption Virtual Machine) codebase - the core framework of the Zama Confidential Blockchain Protocol.

**FHEVM** enables **confidential smart contracts on EVM-compatible blockchains** by leveraging Fully Homomorphic Encryption (FHE), allowing encrypted data to be processed directly on-chain without ever being decrypted.

## What You'll Find Here

This documentation is organized into four main sections:

### 📋 Overview
- **[Executive Summary](executive-summary.md)** - High-level understanding of FHEVM's purpose and innovation
- **[Key Concepts](key-concepts.md)** - Essential concepts like ciphertext handles, symbolic execution, and asynchronous computation
- **[Architecture Overview](architecture.md)** - Three-layer architecture and data flow
- **[Component Health](component-health.md)** - Development activity and focus areas

### 🔧 Core Components
Detailed documentation of each major system component:
- Gateway Contracts, Host Contracts, Solidity Library
- Coprocessor, KMS Connector, Protocol Contracts
- Supporting Infrastructure

### 🔄 Key Workflows
Step-by-step flows for critical operations:
- Symbolic Execution Pattern
- Decryption Pipeline
- Input Verification

### 📚 Reference
- Technology Stack
- Documentation Roadmap
- Quick Reference & Glossary

## Quick Start Paths

**→ I'm a smart contract developer:**
Start with [Key Concepts](key-concepts.md) → [Solidity Library](components/library-solidity.md)

**→ I'm deploying infrastructure:**
Start with [Architecture Overview](architecture.md) → [Supporting Infrastructure](components/infrastructure.md)

**→ I'm contributing to core protocol:**
Start with [Component Health](component-health.md) → specific component documentation

**→ I want to understand the system:**
Follow the documentation in order: Overview → Components → Workflows → Reference

---

## Documentation Status

This is an actively maintained documentation set. Each component section includes:
- 🔥 **Active development** markers for rapidly evolving areas
- ✅ **Stable** markers for mature components
- 📦 **Infrastructure** markers for operational tooling
- **[TODO]** markers for areas pending deeper documentation

See the [Documentation Roadmap](reference/roadmap.md) for planned expansions.

---

*Ready to dive in? Start with the [Executive Summary](executive-summary.md) →*
