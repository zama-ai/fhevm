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
npx hardhat task:ensureMigrationProxyAddresses
```

This compatibility task only deploys empty UUPS proxies for missing address keys and appends
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

## Migration deployment (ProtocolConfig & KMSGeneration)

Two dedicated tasks — `task:deployProtocolConfigFromMigration` and
`task:deployKMSGenerationFromMigration` — call `initializeFromMigration` instead of
`initializeFromEmptyProxy`.

Use these when deploying new proxy instances that must inherit state from an existing
KMSVerifier context (ProtocolConfig) or a frozen Gateway contract (KMSGeneration).

> **One-shot choice.** Both initializers share the same `reinitializer(2)` slot. Once either
> one runs on a proxy, the other is permanently locked out.

### ProtocolConfig migration

Uses `MIGRATION_CONTEXT_ID` (shared with KMSGeneration — see table below).

```bash
MIGRATION_CONTEXT_ID="<context_id>" \
npx hardhat task:deployProtocolConfigFromMigration --network <network>
```

The migrated ProtocolConfig seeds its internal counter so that the first context it creates
has exactly `MIGRATION_CONTEXT_ID`, preserving context continuity for downstream consumers.

### KMSGeneration migration

Requires the full active state from the frozen Gateway. All env vars below are mandatory:

| Variable                                     | Format                                          |
| -------------------------------------------- | ----------------------------------------------- |
| `MIGRATION_PREP_KEYGEN_COUNTER`              | uint256                                         |
| `MIGRATION_KEY_COUNTER`                      | uint256                                         |
| `MIGRATION_CRS_COUNTER`                      | uint256                                         |
| `MIGRATION_ACTIVE_KEY_ID`                    | uint256                                         |
| `MIGRATION_ACTIVE_CRS_ID`                    | uint256                                         |
| `MIGRATION_ACTIVE_PREP_KEYGEN_ID`            | uint256                                         |
| `MIGRATION_ACTIVE_KEY_DIGESTS`               | JSON array: `[{"keyType":0,"digest":"0x…"}, …]` |
| `MIGRATION_ACTIVE_CRS_DIGEST`                | hex bytes                                       |
| `MIGRATION_KEY_CONSENSUS_TX_SENDERS`         | comma-separated addresses                       |
| `MIGRATION_KEY_CONSENSUS_DIGEST`             | bytes32 hex                                     |
| `MIGRATION_CRS_CONSENSUS_TX_SENDERS`         | comma-separated addresses                       |
| `MIGRATION_CRS_CONSENSUS_DIGEST`             | bytes32 hex                                     |
| `MIGRATION_PREP_KEYGEN_CONSENSUS_TX_SENDERS` | comma-separated addresses                       |
| `MIGRATION_PREP_KEYGEN_CONSENSUS_DIGEST`     | bytes32 hex                                     |
| `MIGRATION_CRS_MAX_BIT_LENGTH`               | uint256 (e.g. `4096`)                           |
| `MIGRATION_PREP_KEYGEN_PARAMS_TYPE`          | `0` (Default) or `1` (Test)                     |
| `MIGRATION_CRS_PARAMS_TYPE`                  | `0` (Default) or `1` (Test)                     |
| `MIGRATION_CONTEXT_ID`                       | KMS context ID for the migrated state           |

```bash
MIGRATION_PREP_KEYGEN_COUNTER="…" \
MIGRATION_KEY_COUNTER="…" \
# … (all variables above) …
npx hardhat task:deployKMSGenerationFromMigration --network <network>
```

`task:deployProtocolConfigFromMigration` must run before `task:deployKMSGenerationFromMigration`.
`task:deployKMSGenerationFromMigration` validates the migrated signer / tx-sender consensus sets
against the canonical `ProtocolConfig` state during initialization.

### Breaking changes for event consumers

`KMSVerifier` no longer emits context-lifecycle events after the migration to canonical
`ProtocolConfig` state. Off-chain consumers should move to the `ProtocolConfig` emitter at
`protocolConfigAdd` (`addresses/FHEVMHostAddresses.sol`):

- `KMSVerifier.NewContextSet(uint256,address[],uint256)` -> `ProtocolConfig.NewKmsContext(uint256,KmsNode[],KmsThresholds)`
- `KMSVerifier.KMSContextDestroyed(uint256)` -> `ProtocolConfig.KmsContextDestroyed(uint256)`
