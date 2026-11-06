# Governance Runbooks

Operational runbooks for creating, reviewing, and managing governance proposals on the Protocol DAO (Aragon-based, on Ethereum mainnet and Sepolia testnet).

| # | Runbook | Description |
|---|---------|-------------|
| 1a | [Creating Ethereum Proposals](creating-proposals-ethereum.md) | Full guide for creating Ethereum proposals via the Aragon frontend (wallet setup, actions, simulation). |
| 1b | [Creating Remote (Cross-Chain) Proposals](creating-proposals-remote.md) | Full guide for cross-chain proposals to any EVM destination (with the `governance-proposal-builder` script) and tracking cross-chain status. |
| 2 | [Reviewing Proposals](reviewing-proposals.md) | Full guide for UI-based review (proxied contracts, magic constants, cross-chain actions) and independent CLI verification with the `aragon-proposal-inspector`. Includes worked examples. |
| 3 | [Manual Execution: Remote](manual-execution-remote.md) | Recovery procedure when a cross-chain proposal was approved and executed on Ethereum but delivery to the destination failed (manually calling `lzReceive`). |

## Reference

| # | Document | Description |
|---|----------|-------------|
| 4 | [Destinations](destinations.md) | Supported EVM destination chains: ids, sender/multisig addresses, RPC env vars, explorers, and how to add a new destination. |
| 5 | [CLI Reference](cli-reference.md) | Complete reference for `fill-options-remote-proposal`, `decode-options-remote-proposal`, and `aragon-proposal-inspector` (installation, usage, inputs/outputs, common errors). |

## Typical Workflow

```
Creator                          Reviewer                          Anyone
───────                          ────────                          ──────
1. Read creator runbooks
2. Create proposal via Aragon frontend
3. Submit proposal
4. Notify reviewers
  - First code owners
  - Then DAO members
                                 5. Read reviewer runbooks
                                 6. Verify via Aragon frontend
                                 7. Verify via CLI inspector
                                 8. (DAO member only) Sign 
                                    proposal in wallet
                                                                  9. (In case of failure)
                                                                     Execute lzReceive on the
                                                                     destination chain manually
                           
```
