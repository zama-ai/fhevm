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
  --upgrade-from-ref v0.11.1 \
  --new-implementation contracts/FHEVMExecutor.sol:FHEVMExecutor \
  --verify-contract true
```

Notes:

- `--network` selects where the implementation deployment transaction is sent.
- `--upgrade-from-ref` is only used to load the old implementation source for OZ validation.
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

Requires one additional env var:

| Variable | Description |
|---|---|
| `EXISTING_CONTEXT_ID` | The context ID from the old KMSVerifier to preserve. Must be `>= KMS_CONTEXT_COUNTER_BASE + 1`. |

```bash
EXISTING_CONTEXT_ID="<context_id>" \
npx hardhat task:deployProtocolConfigFromMigration --network <network>
```

The migrated ProtocolConfig seeds its internal counter so that the first context it creates
has exactly `EXISTING_CONTEXT_ID`, preserving context continuity for downstream consumers.

### KMSGeneration migration

Requires the full active state from the frozen Gateway. All env vars below are mandatory:

| Variable | Format |
|---|---|
| `MIGRATION_PREP_KEYGEN_COUNTER` | uint256 |
| `MIGRATION_KEY_COUNTER` | uint256 |
| `MIGRATION_CRS_COUNTER` | uint256 |
| `MIGRATION_ACTIVE_KEY_ID` | uint256 |
| `MIGRATION_ACTIVE_CRS_ID` | uint256 |
| `MIGRATION_ACTIVE_PREP_KEYGEN_ID` | uint256 |
| `MIGRATION_ACTIVE_KEY_DIGESTS` | JSON array: `[{"keyType":0,"digest":"0x…"}, …]` |
| `MIGRATION_ACTIVE_CRS_DIGEST` | hex bytes |
| `MIGRATION_KEY_CONSENSUS_TX_SENDERS` | comma-separated addresses |
| `MIGRATION_KEY_CONSENSUS_DIGEST` | bytes32 hex |
| `MIGRATION_CRS_CONSENSUS_TX_SENDERS` | comma-separated addresses |
| `MIGRATION_CRS_CONSENSUS_DIGEST` | bytes32 hex |
| `MIGRATION_PREP_KEYGEN_CONSENSUS_TX_SENDERS` | comma-separated addresses |
| `MIGRATION_PREP_KEYGEN_CONSENSUS_DIGEST` | bytes32 hex |
| `MIGRATION_CRS_MAX_BIT_LENGTH` | uint256 (e.g. `4096`) |
| `MIGRATION_PREP_KEYGEN_PARAMS_TYPE` | `0` (Default) or `1` (Test) |
| `MIGRATION_CRS_PARAMS_TYPE` | `0` (Default) or `1` (Test) |
| `MIGRATION_CONTEXT_ID` | KMS context ID for the migrated state |

```bash
MIGRATION_PREP_KEYGEN_COUNTER="…" \
MIGRATION_KEY_COUNTER="…" \
# … (all variables above) …
npx hardhat task:deployKMSGenerationFromMigration --network <network>
```
