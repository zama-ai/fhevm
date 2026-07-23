# Coprocessor upgrade runbook

This runbook covers the preparation of a `proposeCoprocessorUpgrade` proposal for the Aragon DAO. It applies whenever the coprocessor requires an on-chain upgrade proposal.

## What this produces

A hex string (calldata) intended for submission as the action of an Aragon DAO proposal calling `ProtocolConfig.proposeCoprocessorUpgrade(...)`. The DAO votes; on approval, the upgrade window opens across all host chains and the gateway.

## Prerequisites

- The new coprocessor version is built and the release tag is known (e.g. `v0.14.0`).
- The wall-clock start time for the dry-run evaluation window has been finalized.
- The start time is far enough in the future for the DAO to vote first (the `--buffer` value, typically `2h` on mainnet).

## Step 1 — Run the prepare task

Run `task:buildProposeCoprocessorUpgradeCalldata` (DAO path — computes block windows and prints the Aragon
proposal action; never broadcasts). Set the per-chain RPC env vars for the environment first
(`SEPOLIA_ETH_RPC_URL`, `POLYGON_AMOY_RPC_URL`, `GATEWAY_<ENV>_RPC_URL`, etc. — see
[`tasks/utils/environments.ts`](tasks/utils/environments.ts)).

```sh
cd host-contracts
npx hardhat task:buildProposeCoprocessorUpgradeCalldata \
  --environment testnet \
  --start-time 2026-07-01T12:00:00Z \
  --duration 30m \
  --buffer 2h \
  --proposal-id 1 \
  --software-version v0.14.0
```

| Flag                 | Value                                        |
| -------------------- | -------------------------------------------- |
| `--environment`      | `devnet`, `testnet`, or `mainnet`.           |
| `--start-time`       | ISO 8601 UTC, e.g. `2026-07-01T12:00:00Z`.   |
| `--duration`         | Window length, e.g. `30m`.                   |
| `--buffer`           | DAO lead time, e.g. `2h`.                    |
| `--proposal-id`      | Any positive integer (operator-chosen).      |
| `--software-version` | The coprocessor release tag, e.g. `v0.14.0`. |

## Step 2 — Copy the proposal action

Scroll to the end of the output. The `## Aragon proposal action` block prints the `target` (the
`ProtocolConfig` address) and the `calldata` hex. Copy both. Pass `--use-internal-proxy-address` to
resolve the target from the `/addresses` directory if `PROTOCOL_CONFIG_CONTRACT_ADDRESS` is unset.

## Step 3 — Submit to the DAO

Open the Aragon DAO for the target environment:

- **Mainnet** — Aragon DAO at `0xB6D69D5F334d8B97B194617B53c6aB62f8681Ef3` (Ethereum).
- **Testnet** — Aragon DAO at `0x08e8a84c3c8c7cba165B1adcf67Ae4639eF84f52` (Sepolia).

Create a new proposal with:

- **Target contract**: the `ProtocolConfig` on the host chain (Ethereum for mainnet, Sepolia for testnet).
- **Calldata**: the hex string copied in Step 2.

Once the DAO vote passes and the proposal executes, the on-chain `proposeCoprocessorUpgrade` event fires and the upgrade window opens.

## Failure modes

| Error in the logs                                        | Resolution                                                                                   |
| -------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| `DAO buffer violated for: chain X — short by 22m`        | Re-run with `--start-time` pushed forward by at least that amount.                           |
| `env var <CHAIN>_RPC_URL is not set`                     | Export the RPC env var for that chain (names in `tasks/utils/environments.ts`).              |
| `--environment must be one of: devnet, testnet, mainnet` | Pass a valid `--environment`.                                                                |
| `duration too short for chain block time`                | Use at least `1m` for `--duration`.                                                          |

The task exits non-zero (and prints the calldata for inspection) if any chain's `startBlock` is
closer to its tip than `--buffer`. `npx hardhat help task:buildProposeCoprocessorUpgradeCalldata` prints the full
flag reference.

## Direct (no-DAO) path — devnet / test-suite

On devnet or the test-suite the deployer key owns the host `ProtocolConfig`, so the proposal can be
broadcast directly instead of going through the DAO. `task:proposeCoprocessorUpgrade` runs the same
build step and then sends the byte-identical calldata with `DEPLOYER_PRIVATE_KEY` — the sibling of the
KMS-context `task:defineNewKmsContextAndEpoch` broadcast. It sends to the host `ProtocolConfig` on the
network passed via `--network`; resolve the address from `PROTOCOL_CONFIG_CONTRACT_ADDRESS` or pass
`--use-internal-proxy-address` to read it from the `addresses/` directory.

```sh
cd host-contracts
DEPLOYER_PRIVATE_KEY=0x... npx hardhat --network sepolia task:proposeCoprocessorUpgrade \
  --environment devnet \
  --start-time "$(date -u -v+2H '+%Y-%m-%dT%H:%M:%SZ')" \
  --duration 30m --buffer 1h --proposal-id 1 --software-version v0.14.0 \
  --use-internal-proxy-address
```

## Chain set reference

Chain IDs, block times, and RPC env-var names per environment live in [`tasks/utils/environments.ts`](tasks/utils/environments.ts). New chains are added by appending to the relevant environment's `chains` array. The task logic lives in [`tasks/prepareCoprocessorUpgrade.ts`](tasks/prepareCoprocessorUpgrade.ts) and [`tasks/utils/coprocessorUpgradeProposal.ts`](tasks/utils/coprocessorUpgradeProposal.ts).
