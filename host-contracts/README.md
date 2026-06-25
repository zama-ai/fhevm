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

If the upgrade baseline predates `ProtocolConfig` and `KMSGeneration` (for example
`UPGRADE_FROM_TAG=v0.11.1` in CI), run:

```bash
npx hardhat task:deployEmptyProxiesProtocolConfigKMSGeneration
```

This compatibility task deploys only the empty UUPS proxies for missing address keys and appends
the missing generated constants so prepare-upgrade tasks can compile against the current
source tree. Keep it as a forward-compat safeguard if a manifest contract starts importing
those generated addresses before the baseline tag catches up. `DEPLOYER_PRIVATE_KEY` is only
required when the task actually needs to bootstrap missing proxies. Once the baseline tag
includes those contracts, the task becomes a no-op and the bootstrap step can be removed.

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

`mirrorKmsContext` and `mirrorKmsEpoch` are how a replica tracks Ethereum. They are `onlyACLOwner`
and bypass the confirmation quorum — a replica can't re-run the MPC attestations, so it trusts the
operator to import state Ethereum has already finalized, landing it as immediately `Active`:

- `mirrorKmsContext(contextId, kmsNodeParams, thresholds, softwareVersion, pcrValues)` — imports a
  context as the new active context; emits `MirrorKmsContext`.
- `mirrorKmsEpoch(contextId, epochId)` — advances the active epoch of the mirrored context; emits
  `MirrorKmsEpoch`.

IDs must be **strictly increasing** — the only on-chain guard, preventing rollback. Gaps are fine
(contexts/epochs aborted or never activated on Ethereum are just never mirrored). Nothing stops a
replica from **drifting** if a mirror call is skipped or applied out of order: replaying each
Ethereum rotation to every replica, in order, is the operator's responsibility.

### Initializing a non-canonical ProtocolConfig from the canonical chain

The Ethereum `ProtocolConfig` is the source of truth for protocol state, so **new** host chains
seed their replica from it.

The flow is artifact-centric — the same three steps in every environment, only the signer of
step 3 changes:

**1. Export** the canonical KMS context to a reviewable JSON artifact (works from a clean
checkout; needs only RPC access):

```bash
npx hardhat task:exportCanonicalProtocolConfig \
  --canonical-rpc-url https://mainnet.example \
  --canonical-protocol-config-address 0x... \
  --out canonical-protocol-config-snapshot.json
```

The artifact records the canonical chainId, the block number the read was pinned to, the
contract address, the current KMS context id, the KMS node set, and all four thresholds
(bigints serialized as strings).

**2. Review.** All reads happen at one block, so reviewers (e.g. DAO signers) reproduce the
artifact byte-for-byte — even after a later `defineNewKmsContextAndEpoch` rotation — by re-running the
export with `--block-number <N>` from the artifact and diffing the output.

**3. Apply** the reviewed artifact to the local `ProtocolConfig` proxy. Both environments run the
same prepare step — deploy the implementation and build the
`upgradeToAndCall(mirrorKmsContext(…))` payload, landing the replica on canonical's
`currentKmsContextId` instead of a fresh counter. They differ only in who executes that payload:
the devnet task sends it immediately with the deployer key, so **what runs on devnet is
byte-identical to what the DAO signs**.

| Environment       | Task                                                                       | Signer                                              |
| ----------------- | -------------------------------------------------------------------------- | --------------------------------------------------- |
| devnet / local    | `task:deployProtocolConfigFromCanonical --snapshot <artifact.json>`        | `DEPLOYER_PRIVATE_KEY`                              |
| testnet / mainnet | `task:prepareDeployProtocolConfigFromCanonical --snapshot <artifact.json>` | DAO executes the printed `upgradeToAndCall` payload |

```bash
# devnet: direct upgrade with the deployer key
npx hardhat task:deployProtocolConfigFromCanonical --snapshot canonical-protocol-config-snapshot.json

# testnet/mainnet: deploy the implementation and print the DAO payload, without touching the proxy
npx hardhat task:prepareDeployProtocolConfigFromCanonical --snapshot canonical-protocol-config-snapshot.json
```

For quick devnet iteration, `task:deployProtocolConfigFromCanonical` also accepts
`--canonical-rpc-url` + `--canonical-protocol-config-address` instead of `--snapshot` to read
canonical live at deploy time — but then what is deployed is whatever canonical holds at that
moment, not a reviewed artifact. The DAO path is artifact-only by design.

When deploying a full non-canonical host stack, `task:deployAllHostContracts
--protocol-config-source canonical --canonical-rpc-url … --canonical-protocol-config-address …`
runs the mirror in sequence with the other host contracts (this is what the fhevm-cli multi-chain
stack uses, so e2e seeds non-canonical chains exactly like production).

Later canonical rotations are mirrored manually with `mirrorKmsContext` / `mirrorKmsEpoch`, as
described in [Mirror methods](#mirror-methods-non-canonical-write-path).
