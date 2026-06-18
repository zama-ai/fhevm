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

### Non-canonical host chains

`KMSGeneration` is NOT deployed on non-canonical host chains. These chains deploy or
upgrade `ProtocolConfig` to `ProtocolConfigMultichain`, which exposes only the common
verifier reads plus owner-only mirror methods.

Fresh non-canonical deployments must initialize `ProtocolConfigMultichain` from a canonical
snapshot: the active canonical context ID, KMS nodes, thresholds, KMS software version, PCR
values, source chain ID, source block number, and canonical `ProtocolConfig` address.
Mirrored updates must use explicit, strictly increasing canonical context IDs. Gaps are
allowed when canonical pending contexts were aborted or never activated, but unknown gap IDs
remain invalid for `KMSVerifier`.

Polygon is an existing non-canonical deployment with `ProtocolConfig v0.1.0` already behind
the proxy. Upgrade it in place to `ProtocolConfigMultichain`. Do not
redeploy the proxy or use the Ethereum `ProtocolConfig.reinitializeV2(...)` lifecycle
calldata. For OpenZeppelin upgrade tooling, force-import the existing proxy with the old
`ProtocolConfig` artifact, then prepare or execute the upgrade against
`contracts/ProtocolConfigMultichain.sol:ProtocolConfigMultichain` with the multichain
reinitializer provenance arguments.
