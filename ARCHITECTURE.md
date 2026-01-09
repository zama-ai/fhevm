# Architecture overview

This repository contains the core building blocks of FHEVM.

## High-level components
- Smart contracts: on-chain pieces that coordinate FHE workflows.
- Compute engines: off-chain components that perform FHE operations.
- SDKs & tooling: developer-facing libraries and utilities.
- Tests & deployment: end-to-end validation and infra helpers.

## Where to look first
- `gateway-contracts/` and `host-contracts/` for on-chain logic
- `coprocessor/` for FHE computation
- `kms-connector/` for key management integration
- `test-suite/` for end-to-end behavior
- `docs/` for deeper documentation

## Suggested reading path
1. README (concepts + links)
2. Whitepaper (design rationale)
3. Component folders listed above
