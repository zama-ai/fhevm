## Introduction

This node package contains the core Solidity host contracts needed to deploy an FHEVM instance on a host EVM blockchain.

## Getting started

run

```
npm install
```

To run forge tests:

```
npm run forge:soldeer
npm run test:forge
```

## Prepare-only executor upgrade

Use `task:prepareUpgradeFHEVMExecutor` when you need to deploy a new `FHEVMExecutor`
implementation without upgrading the proxy yet.

This task is meant for DAO-driven upgrades:

- it imports the existing proxy into the OpenZeppelin manifest
- it deploys the new implementation with `prepareUpgrade`
- it prints the implementation address and `reinitializeV*` calldata
- it does not mutate the proxy

Run it from a checkout containing the exact new host-contract code you want to deploy.
For a backport hotfix, that means the checked-out branch/tag should match the new release.

The task still needs the current deployment addresses on disk because
`contracts/FHEVMExecutor.sol` imports `addresses/FHEVMHostAddresses.sol`.
Generate them first with the existing setter tasks.
If you are switching environments, restart from `task:setACLAddress` so both generated files
are rewritten from scratch before the remaining addresses are appended:

```bash
npx hardhat task:setACLAddress --address <acl>
npx hardhat task:setFHEVMExecutorAddress --address <executor-proxy>
npx hardhat task:setKMSVerifierAddress --address <kms>
npx hardhat task:setInputVerifierAddress --address <input-verifier>
npx hardhat task:setHCULimitAddress --address <hcu-limit>
npx hardhat task:setPauserSetAddress --address <pauser-set>
```

Those commands generate:

- `addresses/.env.host`
- `addresses/FHEVMHostAddresses.sol`

The values to feed into those setter tasks should come from the currently deployed
environment you are upgrading. A practical source of truth is the verified source bundle of
the implementation currently behind the proxy, specifically `addresses/FHEVMHostAddresses.sol`.

Then run:

```bash
npx hardhat task:prepareUpgradeFHEVMExecutor \
  --network sepolia \
  --current-implementation previous-contracts/FHEVMExecutor.sol:FHEVMExecutor \
  --new-implementation contracts/FHEVMExecutor.sol:FHEVMExecutor \
  --verify-contract true
```

Notes:

- `--network` selects where the implementation deployment transaction is sent.
- `--current-implementation` points to the old implementation source available on disk.
- `--new-implementation` comes from your current checkout.
- if you want the proxy address from `addresses/.env.host`, add `--use-internal-proxy-address true`
- the task runs `hardhat clean` before recompiling so the implementation is not built from stale
  artifacts compiled against another environment

## Breaking changes for event consumers

`KMSVerifier` no longer emits context-lifecycle events after the migration to canonical
`ProtocolConfig` state. Off-chain consumers should move to the `ProtocolConfig` emitter at
`protocolConfigAdd` (`addresses/FHEVMHostAddresses.sol`):

- `KMSVerifier.NewContextSet(uint256,address[],uint256)` -> `ProtocolConfig.NewKmsContext(uint256,uint256,KmsNodeParams[],KmsThresholds,string,PcrValues[])`
- `KMSVerifier.KMSContextDestroyed(uint256)` -> `ProtocolConfig.KmsContextDestroyed(uint256)`

## Host Deployment Role

`task:deployAllHostContracts` requires an explicit `--with-kms-generation` value:

```bash
npx hardhat task:deployAllHostContracts --with-kms-generation true   # canonical host
npx hardhat task:deployAllHostContracts --with-kms-generation false  # non-canonical host
```

`KMSGeneration` is deployed only on the canonical host chain. Non-canonical host chains
deploy the common host contracts only.

### One contract, many chains: canonical vs. non-canonical

**Ethereum is the canonical host — the single source of truth for KMS context/epoch state. The
lifecycle runs only there.** Governance opens a context/epoch
(`defineNewKmsContextAndEpoch` / `defineNewEpochForCurrentKmsContext`) and KMS signers reach
quorum (`confirmKmsContextCreation`, `confirmEpochActivation`) before it activates. `KMSGeneration`
is deployed only here.

The **same** `ProtocolConfig` contract is deployed on every other host chain too (there is no
separate "multichain" contract), but those non-canonical hosts (e.g. Polygon) are read-replicas:
they never run the lifecycle/quorum path, since KMS resharing and attestations happen once, on
Ethereum. They have no `KMSGeneration`, and their only write path is the mirror methods below.

### Mirror methods (non-canonical write path)

`mirrorKmsContextAndEpoch` and `mirrorKmsEpoch` are how a replica tracks Ethereum. They are `onlyACLOwner`
and bypass the confirmation quorum — a replica can't re-run the MPC attestations, so it trusts the
operator to import state Ethereum has already finalized, landing it as immediately `Active`:

- `mirrorKmsContextAndEpoch(contextId, epochId, kmsNodeParams, thresholds, softwareVersion, pcrValues)` —
  imports a context and its epoch as active; emits `MirrorKmsContextAndEpoch`.
- `mirrorKmsEpoch(contextId, epochId)` — advances the active epoch of the mirrored context; emits
  `MirrorKmsEpoch`.

IDs must be **strictly increasing** — the only on-chain guard, preventing rollback. Gaps are fine
(contexts/epochs aborted or never activated on Ethereum are just never mirrored). Nothing stops a
replica from **drifting** if a mirror call is skipped or applied out of order: replaying each
Ethereum rotation to every replica, in order, is the operator's responsibility.

### Initializing a non-canonical ProtocolConfig from the canonical chain

The Ethereum `ProtocolConfig` is the source of truth for protocol state, so **new** host chains
seed their replica from it.

The flow is artifact-centric — the same three steps in every environment:

**1. Export** the canonical KMS context to a reviewable JSON artifact (works from a clean
checkout; needs only RPC access):

```bash
npx hardhat task:exportCanonicalProtocolConfig \
  --canonical-rpc-url https://mainnet.example \
  --canonical-protocol-config-address 0x... \
  --out canonical-protocol-config-snapshot.json
```

The artifact holds a single `export` object. That object is the snapshot as a flat `KEY=value` map,
with bigints serialized as decimal strings. Each key becomes an environment variable that the apply
task in step 3 reads.

**2. Review.** All reads happen at one block, so reviewers (e.g. DAO signers) reproduce the
artifact byte-for-byte — even after a later `defineNewKmsContextAndEpoch` rotation — by re-running the
export with `--block-number <N>` from the artifact and diffing the output.

**3. Apply** the reviewed artifact to the local `ProtocolConfig` proxy. `initializeFromCanonical`
only runs against a still-empty proxy (`onlyFromEmptyProxy`), so onboarding a new host chain is
always deployer-executed, with the deployer key that just deployed the empty proxies.

The apply task reads its configuration from environment variables. It takes no command-line flags
for the canonical state. A deployment platform injects the values into the deploy container.

The table lists the `export` keys from step 1. The task rejects a bad value before it deploys anything, so a
misconfigured environment leaves the proxy untouched.

| Variable                            | Type           | Meaning                                                |
| ----------------------------------- | -------------- | ------------------------------------------------------ |
| `CANONICAL_CHAIN_ID`                | decimal string | Chain id of the canonical host chain.                  |
| `CANONICAL_PROTOCOL_CONFIG_ADDRESS` | address        | Canonical `ProtocolConfig` the snapshot was read from. |
| `CANONICAL_BLOCK_NUMBER`            | decimal string | Block the snapshot was pinned to.                      |
| `CANONICAL_BLOCK_HASH`              | 32-byte hex    | Hash of that block.                                    |
| `CANONICAL_KMS_CONTEXT_ID`          | decimal string | Active KMS context id to mirror.                       |
| `CANONICAL_EPOCH_ID`                | decimal string | Active KMS epoch id to mirror.                         |
| `CANONICAL_KMS_NODES`               | JSON array     | The KMS node set, one JSON object per node.            |
| `CANONICAL_KMS_THRESHOLDS`          | JSON object    | The four thresholds, each a decimal string.            |

The context id, epoch id, node set and thresholds become the `initializeFromCanonical` calldata, so
the task checks them in full. The chain id and block number are provenance, and the task parses them
as decimal strings. The block hash and the address are provenance that the task only checks for
presence, then prints.

```bash
npx hardhat task:deployProtocolConfigFromCanonical
```

Only step 1 talks to the canonical chain. The task checks what the environment gives it, and no more.
It does not prove the values match the reviewed artifact. The deployment platform
owns that binding. The apply task prints `decodedArgs`, the context id, epoch id, node set and
thresholds the payload encodes. It also prints the chain id, block number, block hash and canonical
`ProtocolConfig` address. An operator compares every provenance value against the artifact. The
printed node set carries placeholder MPC fields, so the operator compares it field by field.

Ownership of the resulting proxy moves to the DAO afterwards, through
`task:transferHostOwnership` / `task:acceptHostOwnership`.

When deploying a full non-canonical host stack, `task:deployAllHostContracts
--protocol-config-source canonical` runs the mirror in sequence with the other host contracts, with
the same environment variables (this is what the fhevm-cli multi-chain stack uses, so e2e seeds
non-canonical chains exactly like production).

Later canonical rotations are mirrored with `task:buildMirrorKmsContextAndEpochCalldata` /
`task:mirrorKmsContextAndEpoch` (context switch) and `task:buildMirrorKmsEpochCalldata` /
`task:mirrorKmsEpoch` (same-set epoch rotation), defined in `tasks/mirrorKmsContext.ts`. Both read
canonical's active KMS context/epoch over `--canonical-rpc-url` / `--canonical-protocol-config-address`
(the context-switch pair also cross-checks the recovered `NewKmsContext` event data against the
canonical `contextInfoHash` anchor) and call the replica's `mirrorKmsContextAndEpoch` /
`mirrorKmsEpoch` described in [Mirror methods](#mirror-methods-non-canonical-write-path).
