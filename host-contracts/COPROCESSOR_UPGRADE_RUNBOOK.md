# Coprocessor upgrade runbook

Use this guide for an upgrade that increases `CONSENSUS_PROTOCOL_VERSION`.
Use a normal rolling deployment when that value does not change.

## What this produces

The workflow creates the calldata for
`ProtocolConfig.proposeCoprocessorUpgrade(...)`. Add this calldata to the
Aragon DAO proposal. When the proposal passes, the upgrade window opens.

## Prerequisites

- Build the new coprocessor release.
- Check that its `CONSENSUS_PROTOCOL_VERSION` is one more than the active value
  in the `versioning` table.
- Choose the start time and length of the test window.
- Leave enough time for the DAO vote. Use `--buffer` for this delay.

## Required rollout order

1. Deploy `ProtocolConfig` with support for
   `proposeCoprocessorUpgrade`.
2. Deploy listeners and controllers that support
   `CoprocessorUpgradeProposed`.
3. Check that every green binary is the release named in the proposal.
4. Submit the proposal.
5. Complete the cutover, check the new version, and stop the old fleet. A restarted
   service from this scheme onwards stops because its consensus version is too old; one
   from before it stops because the stored release is now higher than its own.

## The two versions

The proposal carries the release, the same as before:

```
--software-version 0.15.0   ->   softwareVersion "0.15.0"
```

What you pass is what goes on chain. It has to be the release the green binaries were
built as, or they refuse the cutover.

The consensus protocol version is compiled into the binary and never appears in the
proposal. It decides which stack is live, green, or retired.

**Raise it by one** for an upgrade the fleet must cut over to: new key parameters, the GPU
feature, a randomization change, or a change to the scheduling logic. **Leave it alone** for
a plain release, which then rolls out one operator at a time with no proposal and no
cutover.

## Step 1 — Run the workflow

Navigate to **Actions** → **host-contracts-prepare-coprocessor-upgrade** → **Run workflow** and provide:

| Input                 | Value                                      |
| --------------------- | ------------------------------------------ |
| **Environment**       | `devnet`, `testnet`, or `mainnet`.         |
| **Start time**        | ISO 8601 UTC, e.g. `2026-07-01T12:00:00Z`. |
| **Duration**          | Window length, e.g. `30m`.                 |
| **Buffer**            | DAO lead time, e.g. `2h`.                  |
| **Proposal id**       | Any positive integer (operator-chosen).    |
| **Consensus version** | The binary's `CONSENSUS_PROTOCOL_VERSION`. |

Click **Run workflow** and wait for completion.

## Step 2 — Copy the calldata

Open the **Prepare upgrade proposal** logs. Copy the value under `## Calldata`.
Copy the entire string.

## Step 3 — Submit to the DAO

Open the Aragon DAO for the target environment:

- **Mainnet** — Aragon DAO at `0xB6D69D5F334d8B97B194617B53c6aB62f8681Ef3` (Ethereum).
- **Testnet** — Aragon DAO at `0x08e8a84c3c8c7cba165B1adcf67Ae4639eF84f52` (Sepolia).

Create a new proposal with:

- **Target contract**: the `ProtocolConfig` on the host chain (Ethereum for mainnet, Sepolia for testnet).
- **Calldata**: the hex string copied in Step 2.

After the vote passes, `CoprocessorUpgradeProposed` opens the upgrade window.
Cutover starts only when the proposal's release matches the green
binaries and is above the active one.

## Failure modes

| Error in the logs                                        | Resolution                                                                                   |
| -------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| `DAO buffer violated for: chain X — short by 22m`        | Re-run with `--start-time` pushed forward by at least that amount.                           |
| `env var RPC_URL_X is not set`                           | Add the missing secret to repo settings. The workflow file header lists the secrets per env. |
| `--environment must be one of: devnet, testnet, mainnet` | Select a valid environment from the dropdown.                                                |
| `duration too short for chain block time`                | Use at least `1m` for `--duration`.                                                          |

## Local rehearsal

For dry runs or development, run the `task:prepareCoprocessorUpgrade` hardhat task directly:

```sh
cd host-contracts
npx hardhat task:prepareCoprocessorUpgrade \
  --environment testnet \
  --start-time "$(date -u -v+2H '+%Y-%m-%dT%H:%M:%SZ')" \
  --duration 30m \
  --buffer 1h \
  --proposal-id 1 \
  --software-version 0.15.0
```

Output and calldata are identical to the workflow run. The task exits non-zero (and prints the
calldata for inspection) if any chain's `startBlock` is closer to its tip than `--buffer`.
`npx hardhat help task:prepareCoprocessorUpgrade` prints the full flag reference.

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
  --duration 30m --buffer 1h --proposal-id 1 \
  --software-version 0.15.0 \
  --use-internal-proxy-address
```

## Chain set reference

Chain IDs, block times, and RPC env-var names per environment live in [`tasks/utils/environments.ts`](tasks/utils/environments.ts). New chains are added by appending to the relevant environment's `chains` array. The task logic lives in [`tasks/prepareCoprocessorUpgrade.ts`](tasks/prepareCoprocessorUpgrade.ts) and [`tasks/utils/coprocessorUpgradeProposal.ts`](tasks/utils/coprocessorUpgradeProposal.ts).
