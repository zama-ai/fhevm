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
npx hardhat task:deployAllHostContracts --with-kms-generation false --protocol-config-source canonical  # non-canonical host
```

`KMSGeneration` is deployed only on the canonical host chain. Non-canonical host chains
deploy the common host contracts only. They pass `--protocol-config-source canonical`, so they
deploy `ProtocolConfigReplica` seeded from the `CANONICAL_*` env variables (see below). The default
`fresh` deploys `ProtocolConfig`.

### Canonical vs. non-canonical hosts

**Ethereum is the canonical host — the single source of truth for KMS context/epoch state. The
lifecycle runs only there.** Governance opens a context/epoch
(`defineNewKmsContextAndEpoch` / `defineNewEpochForCurrentKmsContext`) and KMS signers reach
quorum (`confirmKmsContextCreation`, `confirmEpochActivation`) before it activates. Each confirmation
carries the signer's EIP-712 signature, and anyone may submit it. The contract also collects
destruction confirmations (`confirmKmsContextDestruction`, `confirmKmsEpochDestruction`) from the
active committee. It emits a quorum event once `n - t` of them agree on one digest. `KMSGeneration`
is deployed only here.

Every other host chain (e.g. Polygon) runs `ProtocolConfigReplica`, a read-replica. Replicas never run the
lifecycle (pending states and confirmation collection), since KMS resharing and attestations happen
once, on Ethereum. They have no `KMSGeneration`. Anyone may apply a canonical lifecycle step on a
replica with the KMS confirmation signatures, through the `submitKms*` methods. This is the
permissionless path. No task in this repo calls them yet. The owner mirror methods below are the
path the repo tasks use for rotations. `recoverKmsState` is the recovery path.

A submitted destruction of an id at or above the replica's active id reverts with
`KmsContextDestructionAheadOfReplica` or `KmsEpochDestructionAheadOfReplica`. The submitter retries
it after the replica moves past that id.

### Mirror methods (owner path)

The mirror methods are `onlyACLOwner`. They skip signature checks and apply canonical state
directly. The repo mirror tasks call `mirrorKmsContextAndEpoch` and `mirrorKmsEpoch` for every
rotation.

- `mirrorKmsContextAndEpoch(contextId, epochId, nodes, thresholds)` imports a context and its epoch
  as active. It emits `MirrorKmsContextAndEpoch`.
- `mirrorKmsEpoch(contextId, epochId)` advances the active epoch of the active context. It emits
  `MirrorKmsEpoch`.
- `mirrorKmsContextDestruction(contextId)` and `mirrorKmsEpochDestruction(epochId)` mark an id
  destroyed. They emit `KmsContextDestroyed` and `KmsEpochDestroyed`.
- `recoverKmsState(contextId, epochId, nodes, thresholds, destroyedContextIds, destroyedEpochIds)`
  recovers a replica from a forged context or epoch in one call. It activates the given pair with no
  ordering check and destroys every listed id. It emits `MirrorKmsContextAndEpoch`, then
  `KmsContextDestroyed` and `KmsEpochDestroyed` for each listed id.

`mirrorKmsContextAndEpoch` requires a context id and an epoch id **strictly above** the active ones.
`mirrorKmsEpoch` requires the active context id and an epoch id strictly above the active one. Both
clear the destroyed flag of the ids they activate. `recoverKmsState` also clears the flags of the pair
it restores. The submit path rejects a destroyed id, so only the owner can land a canonical id that
`recoverKmsState` destroyed. Only `recoverKmsState` moves a replica back. Gaps are fine (contexts/epochs
aborted or never activated on Ethereum are just never mirrored). Nothing stops a replica from
**drifting** if a mirror call is skipped: replaying each Ethereum rotation to every replica is the
operator's responsibility.

### Initializing a ProtocolConfigReplica from the canonical chain

The Ethereum `ProtocolConfig` is the source of truth for protocol state, so **new** host chains
seed their replica from it.

An existing replica proxy that still runs `ProtocolConfig` does not need this flow. It migrates in
place with `upgradeToAndCall(<ProtocolConfigReplica impl>, reinitializeV4(canonicalChainId, canonicalProtocolConfig))`.
The two arguments set the canonical `ProtocolConfig` EIP-712 domain. Both contracts use the
same base storage, so the KMS context, epoch and nodes stay as they are.

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
tasks in step 3 read.

**2. Review.** All reads happen at one block, so reviewers (e.g. DAO signers) reproduce the
artifact byte-for-byte — even after a later `defineNewKmsContextAndEpoch` rotation — by re-running the
export with `--block-number <N>` from the artifact and diffing the output.

**3. Apply** the reviewed artifact to the local `ProtocolConfigReplica` proxy. Both environments run the
same prepare step — deploy the implementation and build the
`upgradeToAndCall(initializeFromCanonical(contextId, epochId, …))` payload, landing the
replica on canonical's active context and epoch instead of fresh local counters. They differ only in
who executes that payload: the devnet task sends it immediately with the deployer key, so **what runs
on devnet is byte-identical to what the DAO signs**.

Both apply tasks read their configuration from environment variables. They take no command-line flags
for the canonical state. A deployment platform injects the values into the deploy container.

The table lists the `export` keys from step 1. The tasks run the checks listed below the table before
they deploy anything. A value that fails a check leaves the proxy untouched.

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

The context id, epoch id, node set, thresholds, chain id and address become the
`initializeFromCanonical` calldata. The chain id and the address set the canonical EIP-712 domain
that the replica verifies signatures under. The tasks parse the context id, epoch id and block
number as decimal strings. They check the node set and thresholds in full. They require a chain id
in [1, 2^64) and a valid nonzero address. They only check the block hash for presence, then print
it. The block number and block hash are provenance.

| Environment       | Task                                            | Signer                                              |
| ----------------- | ----------------------------------------------- | --------------------------------------------------- |
| devnet / local    | `task:deployProtocolConfigFromCanonical`        | `DEPLOYER_PRIVATE_KEY`                              |
| testnet / mainnet | `task:prepareDeployProtocolConfigFromCanonical` | DAO executes the printed `upgradeToAndCall` payload |

```bash
# devnet: direct upgrade with the deployer key
npx hardhat task:deployProtocolConfigFromCanonical

# testnet/mainnet: deploy the implementation and print the DAO payload, without touching the proxy
npx hardhat task:prepareDeployProtocolConfigFromCanonical
```

Only step 1 talks to the canonical chain. The tasks check what the environment gives them, and no more.
They do not prove the values match the reviewed artifact. The deployment platform
owns that binding. Both apply tasks print `decodedArgs`, the context id, epoch id, node set,
thresholds, chain id and address the payload encodes. They also print the block number and block
hash. An operator compares every provenance value against the artifact.

When deploying a full non-canonical host stack, `task:deployAllHostContracts
--protocol-config-source canonical` runs the mirror in sequence with the other host contracts, with
the same environment variables (this is what the fhevm-cli multi-chain stack uses, so e2e seeds
non-canonical chains exactly like production).

The rotation tasks are `task:buildMirrorKmsContextAndEpochCalldata` /
`task:mirrorKmsContextAndEpoch` (context switch) and `task:buildMirrorKmsEpochCalldata` /
`task:mirrorKmsEpoch` (same-set epoch rotation), defined in `tasks/mirrorKmsContext.ts`. Both read
canonical's active KMS context/epoch over `--canonical-rpc-url` / `--canonical-protocol-config-address`
(the context-switch pair also cross-checks the recovered `NewKmsContext` event data against the
canonical `contextInfoHash` anchor) and call the replica's `mirrorKmsContextAndEpoch` /
`mirrorKmsEpoch` described in [Mirror methods](#mirror-methods-owner-path).
