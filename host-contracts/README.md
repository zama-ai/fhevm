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

- `KMSVerifier.NewContextSet(uint256,address[],uint256)` -> `ProtocolConfig.NewKmsContext(uint256,KmsNode[],KmsThresholds)`
- `KMSVerifier.KMSContextDestroyed(uint256)` -> `ProtocolConfig.KmsContextDestroyed(uint256)`

## Host Deployment Role

`task:deployAllHostContracts` requires an explicit `--with-kms-generation` value:

```bash
npx hardhat task:deployAllHostContracts --with-kms-generation true   # canonical host
npx hardhat task:deployAllHostContracts --with-kms-generation false  # non-canonical host
```

`KMSGeneration` is deployed only on the canonical host chain. Non-canonical host chains
deploy the common host contracts only.

### Non-canonical host chains

`KMSGeneration` is NOT deployed on non-canonical host chains. `ProtocolConfig` on
non-canonical chains is a replica whose state is mirrored manually (Phase 1) or via
LayerZero / LzRead (Phase 2) from the canonical chain. Operators mirroring a canonical
rotation to non-canonical chains call `defineNewKmsContext` directly on each non-canonical
`ProtocolConfig`, using the same `kmsNodes` and `thresholds` as the canonical rotation.

No on-chain guard prevents a non-canonical replica from drifting if a mirror transaction is
skipped. Operators are responsible for fan-out correctness.

### Initializing a non-canonical ProtocolConfig from the canonical chain

The Ethereum `ProtocolConfig` is the source of truth for protocol state, so **new** host chains
seed their replica from it. (The Gateway-based export `task:exportKmsMigrationState` remains the
mechanism for the one-time Gateway → Ethereum migration of existing deployments; it is just no
longer the state source for seeding non-canonical chains.)

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
artifact byte-for-byte — even after a later `defineNewKmsContext` rotation — by re-running the
export with `--block-number <N>` from the artifact and diffing the output.

**3. Apply** the reviewed artifact to the local `ProtocolConfig` proxy. Both environments run the
same prepare step — deploy the implementation and build the
`upgradeToAndCall(initializeFromMigration(…))` payload, landing the replica on canonical's
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

Later canonical rotations are mirrored manually with `defineNewKmsContext`, as described above.
