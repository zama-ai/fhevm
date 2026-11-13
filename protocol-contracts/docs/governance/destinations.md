# Cross-Chain Governance Destinations

The Protocol DAO on Ethereum can execute governance actions on multiple EVM
destination chains. Each destination has its own one-way pipeline:

```
Aragon DAO (Ethereum)
  └─ sendRemoteProposal → GovernanceOAppSender (Ethereum, one per destination)
       └─ LayerZero → GovernanceOAppReceiver (destination)
            └─ AdminModule.executeSafeTransactions → destination multisig (Safe)
                 └─ executes the target calls
```

There is **one `GovernanceOAppSender` per destination** on the source chain
(Ethereum mainnet / Sepolia). A proposal's `to` is the sender for the chosen
destination; the tooling runs `eth_estimateGas` against the destination chain
with the destination multisig as the (unsigned) `from` to size the LayerZero
execution gas.

## Destination registry

The list of supported destinations is **not duplicated here** — it lives in the
tooling and the registry, so there's a single source of truth:

- **Destination ids + the fields the scripts use** (`GovernanceOAppSender`,
  destination multisig, RPC var) →
  [`protocol-contracts/scripts/governance-proposal-builder/destinations.js`](https://github.com/zama-ai/fhevm/blob/main/protocol-contracts/scripts/governance-proposal-builder/destinations.js).
  Run `npm run list-destinations` (or `node destinations.js`) to print the live
  list.
- **Everything else** per destination (all addresses, LZ EID, `EndpointV2`,
  block explorer, receiver / module addresses) →
  [protocol-registry](https://github.com/zama-ai/protocol-registry), the source
  of truth — always re-verify addresses there. (It publishes mainnet/testnet
  only; devnet addresses come from the devnet deployment.)

> **Manual-execution recovery only:** the destination's `EndpointV2` (used by
> [manual execution recovery](manual-execution-remote.md), not by the
> fill/decode scripts) is `0x6F475642a6e85809B1c36Fa62763669b1b48DD5B` for
> Gateway mainnet. For any other destination,
> look it up in the
> [LayerZero deployments](https://docs.layerzero.network/v2/deployments/deployed-contracts)
> or [protocol-registry](https://github.com/zama-ai/protocol-registry).
