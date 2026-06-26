# Coprocessor upgrade runbook

This runbook covers the preparation of a `proposeCoprocessorUpgrade` proposal for the Aragon DAO. It applies whenever the coprocessor requires an on-chain upgrade proposal.

## What this produces

A hex string (calldata) intended for submission as the action of an Aragon DAO proposal calling `ProtocolConfig.proposeCoprocessorUpgrade(...)`. The DAO votes; on approval, the upgrade window opens across all host chains and the gateway.

## Prerequisites

- The new coprocessor version is built and the release tag is known (e.g. `v0.14.0`).
- The wall-clock start time for the dry-run evaluation window has been finalized.
- The start time is far enough in the future for the DAO to vote first (the `--buffer` value, typically `2h` on mainnet).

## Step 1 — Run the workflow

Navigate to **Actions** → **host-contracts-prepare-coprocessor-upgrade** → **Run workflow** and provide:

| Input                  | Value                                        |
| ---------------------- | -------------------------------------------- |
| **Environment**        | `devnet`, `testnet`, or `mainnet`.           |
| **Start time**         | ISO 8601 UTC, e.g. `2026-07-01T12:00:00Z`.   |
| **Duration**           | Window length, e.g. `30m`.                   |
| **Buffer**             | DAO lead time, e.g. `2h`.                    |
| **Proposal id**        | Any positive integer (operator-chosen).      |
| **Software version**   | The coprocessor release tag, e.g. `v0.14.0`. |
| **Ciphertext version** | An integer between 0 and 32767.              |

Click **Run workflow** and wait for completion.

## Step 2 — Copy the calldata

Open the logs of the **"Prepare upgrade proposal"** step. Scroll to the end. The last block under `## Calldata` is a hex string starting with `0x49213995…`.
Copy the entire string.

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
| `env var RPC_URL_X is not set`                           | Add the missing secret to repo settings. The workflow file header lists the secrets per env. |
| `--environment must be one of: devnet, testnet, mainnet` | Select a valid environment from the dropdown.                                                |
| `duration too short for chain block time`                | Use at least `1m` for `--duration`.                                                          |

## Local rehearsal

For dry runs or development:

```sh
cd host-contracts
npm run prepare-coprocessor-upgrade -- \
  --environment testnet \
  --start-time "$(date -u -v+2H '+%Y-%m-%dT%H:%M:%SZ')" \
  --duration 30m \
  --buffer 1h \
  --proposal-id 1 \
  --software-version v0.14.0 \
  --ciphertext-version 2
```

Output and calldata are identical to the workflow run. `--help` prints the full flag reference.

## Chain set reference

Chain IDs, block times, and RPC env-var names per environment live in [`scripts/utils/environments.ts`](./utils/environments.ts). New chains are added by appending to the relevant environment's `chains` array.
