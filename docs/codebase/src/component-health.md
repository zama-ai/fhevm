# Component Health & Development Activity

> **Last analyzed**: December 2025 | Based on 6-month git history

This page tracks the development activity and health of each major component to help you understand where the codebase is actively evolving versus stable.

## Activity Overview

| Component | Status | 6-mo Commits | Focus Areas |
|-----------|--------|--------------|-------------|
| `coprocessor/` | 🔥 Active | 1,718 | GPU optimization, metrics, health checks |
| `kms-connector/` | 🔥 Active | 1,110 | Garbage collection, polling, nonce management |
| `gateway-contracts/` | 🔥 Active | 1,071 | Payment protocol, multi-sig, cross-chain |
| `protocol-contracts/` | 🔥 Active | 748 | Staking, delegation, fee management |
| `host-contracts/` | ✅ Stable | 455 | ACL enhancements, operator pricing |
| `library-solidity/` | ✅ Stable | 410 | Codegen consolidation, type improvements |
| `test-suite/` | 🔥 Active | 975 | E2E tests, version tracking |
| `charts/` | 📦 Infra | 138 | K8s deployment emerging |
| `sdk/` | 📦 Infra | 91 | Maintenance mode |

**Legend:**
- 🔥 **Active development** - Rapid evolution, expect frequent changes
- ✅ **Stable/maintained** - Mature codebase, incremental improvements
- 📦 **Infrastructure** - Operational tooling, minimal changes

## Component Status Details

### 🔥 Coprocessor (Highly Active)
**Recent Focus:**
- GPU scheduler improvements and memory management
- Metrics collection (SNS latency, ZK verify latency, tfhe-per-txn timing)
- Health checking in tfhe-worker and sns-worker
- Database optimization (indices on ciphertext_digest, schedule order)
- Compression for large ciphertexts
- Off-chain execution optimization

**Implication:** Expect performance improvements and new optimization features. API relatively stable but internal architecture evolving.

### 🔥 KMS Connector (Highly Active)
**Recent Focus:**
- Garbage collection implementation
- Database transaction management and retry logic
- Polling mechanisms and listener improvements
- Nonce manager with recoverable patterns
- Configuration updates (WebSocket to HTTP migration)

**Implication:** Infrastructure hardening and reliability improvements. External KMS integration patterns maturing.

### 🔥 Gateway Contracts (Highly Active)
**Recent Focus:**
- Payment protocol implementation (`ProtocolPayment` contract)
- Multi-sig contracts based on Safe Smart Account
- LayerZero cross-chain integration for testnet/mainnet
- Monitoring events and request ID validation

**Implication:** Economic and cross-chain capabilities expanding. Expect new features for multi-chain deployments.

### 🔥 Protocol Contracts (Highly Active)
**Recent Focus:**
- Staking/delegating contracts (`OperatorStaking`, `Rewarder`)
- Fee management and burner implementation
- Governance improvements (Safe ownership, admin modules)
- ERC1363 integration
- UUPS upgradeability patterns

**Implication:** Economic layer maturing with staking and governance. Token mechanics actively evolving.

### ✅ Host Contracts (Stable)
**Recent Focus:**
- ACL enhancements and permission model refinements
- Operator pricing mechanisms
- Minor optimizations and bug fixes

**Implication:** Core execution layer is mature and stable. Changes are incremental improvements.

### ✅ Library Solidity (Stable)
**Recent Focus:**
- Codegen consolidation to single location
- Type system improvements
- Developer experience enhancements

**Implication:** Developer API is stable. Safe to build applications against this interface.

## Recent Removals & Deprecations

**Avoid documenting these deprecated items:**

- `ProtocolOperatorRegistry` - Removed from protocol-contracts, replaced by `OperatorStaking`
- Distributed codegen - Consolidated to `/library-solidity/codegen/`
- Safe-specific tasks - Removed from gateway-contracts

## What This Means for Documentation

When expanding documentation:

1. **High-priority areas** (🔥 Active): Expect APIs and internals to evolve. Document current state but note potential for change.
2. **Stable areas** (✅ Stable): Safe to create comprehensive, detailed documentation. Changes will be incremental.
3. **Infrastructure** (📦 Infra): Document operational patterns and deployment, less focus on internal details.

---

**Next:** Dive into [Core Components](components/README.md) →
