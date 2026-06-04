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

## Host Deployment Roles

A host chain is deployed in one of two roles:

- **Canonical host**: owns `KMSGeneration`, creates the first `ProtocolConfig` KMS
  context, and deploys `KMSVerifier`.
- **Secondary host**: never writes `KMS_GENERATION_CONTRACT_ADDRESS`; it mirrors
  `ProtocolConfig` from the canonical host, then deploys `KMSVerifier`.

Pick the flow for your situation; the table below is the per-task reference.

- **Fresh canonical host** → `task:deployCanonicalHost` — full stack including `KMSGeneration`.
- **Fresh secondary host** → `task:deploySecondaryHost` — mirrors canonical's `ProtocolConfig`, with no
  `KMSGeneration` (a secondary always copies canonical).
- **Migrate an existing host to 0.13** → Gateway-side `task:exportKmsMigrationState`, then host-side
  `task:prepareDeployProtocolConfigFromMigration` / `task:prepareDeployKMSGenerationFromMigration` → DAO executes.
- **Upgrade one contract** → `task:prepareUpgrade<Contract>` → DAO executes (or `task:upgrade<Contract>` locally).
- **Verify on the block explorer** → `task:verifyCanonicalHost` / `task:verifySecondaryHost`.
- **Audit & hand off** → `task:exportCanonicalProtocolConfig` writes a hashed snapshot for the DAO, then
  `task:transferHostOwnership` → `task:acceptHostOwnership`.

| Task | Role | Standalone? | What it does |
|---|---|---:|---|
| `task:deployCanonicalHost` | canonical | yes | Deploys the complete canonical host stack. |
| `task:deploySecondaryHost` | secondary | yes | Deploys a secondary host without `KMSGeneration`. |
| `task:deployHostSkeleton` | shared | no | Deploys only the shared host skeleton: empty UUPS proxies, PauserSet, ACL, FHEVMExecutor, InputVerifier, and HCULimit. |
| `task:deployEmptyUUPSProxies` | shared | no | Deploys empty UUPS proxies and writes address artifacts. Use `--skip-kms-generation` only when composing a secondary host. |
| `task:deployProtocolConfigCanonical` | canonical | no | Upgrades the local `ProtocolConfig` proxy and seeds the first KMS context from env vars. |
| `task:deployProtocolConfigSecondary` | secondary | no | Upgrades the local `ProtocolConfig` proxy from a pinned canonical snapshot. |
| `task:deployKMSVerifier` | both | no | Upgrades the local `KMSVerifier` proxy. Requires local `ProtocolConfig` to be ready first. |
| `task:deployKMSGeneration` | canonical | no | Upgrades the canonical-only `KMSGeneration` proxy and initializes it. Never run this on a secondary host. |

### Env vars by task

| Task | Required env vars |
|---|---|
| `task:deployCanonicalHost` | all canonical host vars listed below |
| `task:deploySecondaryHost` | `DEPLOYER_PRIVATE_KEY`, `CHAIN_ID_GATEWAY`, `INPUT_VERIFICATION_ADDRESS`, `NUM_COPROCESSORS`, `COPROCESSOR_THRESHOLD`, `COPROCESSOR_SIGNER_ADDRESS_0..N-1`, `DECRYPTION_ADDRESS`; takes `--canonical-rpc-url` and `--canonical-protocol-config-address` |
| `task:deployHostSkeleton` | `DEPLOYER_PRIVATE_KEY`, `CHAIN_ID_GATEWAY`, `INPUT_VERIFICATION_ADDRESS`, `NUM_COPROCESSORS`, `COPROCESSOR_THRESHOLD`, `COPROCESSOR_SIGNER_ADDRESS_0..N-1` |
| `task:deployProtocolConfigCanonical` | adds `NUM_KMS_NODES`, `KMS_TX_SENDER_ADDRESS_0..N-1`, `KMS_SIGNER_ADDRESS_0..N-1`, `KMS_NODE_STORAGE_URL_0..N-1`, `PUBLIC_DECRYPTION_THRESHOLD`, `USER_DECRYPTION_THRESHOLD`, `KMS_GEN_THRESHOLD`, `MPC_THRESHOLD` |
| `task:deployProtocolConfigSecondary` | adds **none**: reads everything from canonical via RPC. Takes `--canonical-rpc-url` and `--canonical-protocol-config-address` as task arguments. |
| `task:deployProtocolConfigFromMigration` | adds `MIGRATION_CONTEXT_ID`, `MIGRATION_KMS_NODES`, `MIGRATION_KMS_THRESHOLDS` (used only for the Gateway-to-Ethereum migration, see below) |
| `task:deployKMSVerifier` | adds `DECRYPTION_ADDRESS` (and reuses `CHAIN_ID_GATEWAY`) |
| `task:deployKMSGeneration` | adds none beyond `DEPLOYER_PRIVATE_KEY` |
| `task:addHostPausers` | adds `NUM_PAUSERS`, `PAUSER_ADDRESS_0..N-1` |

Secondary chains need no KMS-node env vars. `task:deployProtocolConfigSecondary` mirrors
the canonical KMS context via RPC at deploy time; later rotations are mirrored by the ACL
owner.

### Canonical host chain

```bash
npx hardhat task:deployCanonicalHost
```

This deploys the shared host skeleton, seeds canonical `ProtocolConfig`, deploys
`KMSVerifier`, and installs `KMSGeneration`.

### Secondary (non-canonical) host chain

```bash
npx hardhat task:deploySecondaryHost \
  --canonical-rpc-url <CANONICAL_RPC_URL> \
  --canonical-protocol-config-address <CANONICAL_PROTOCOL_CONFIG_ADDRESS>
```

This deploys the shared host skeleton without `KMSGeneration`, mirrors the canonical
`ProtocolConfig` snapshot via `initializeFromMigration`, and deploys `KMSVerifier`.
Secondary host discovery must not contain `KMS_GENERATION_CONTRACT_ADDRESS`. Later
canonical rotations (`defineNewKmsContext`, `destroyKmsContext`) must be mirrored by the
secondary ACL owner.

### Verifying the mirrored snapshot (DAO review)

`task:deploySecondaryHost` reads the canonical context live at deploy time, which the DAO
cannot audit after the fact. Pin and publish the snapshot instead:

```bash
npx hardhat task:exportCanonicalProtocolConfig \
  --canonical-rpc-url <CANONICAL_RPC_URL> \
  --canonical-protocol-config-address <CANONICAL_PROTOCOL_CONFIG_ADDRESS> \
  --out canonical-protocol-config-snapshot.json
```

This writes the canonical KMS context at a pinned block, plus a `hash` over the chain id,
ProtocolConfig address, context id, KMS nodes, and thresholds. Before accepting
secondary-host ownership, DAO signers re-run the export at the same `blockNumber` and confirm
the `hash` matches the deployed secondary's context.

### Ownership hand-off

After deployment, the deployer holds ACL ownership on every host chain. Hand control to
the production owner with `task:transferHostOwnership --new-owner <ADDR>` (followed by
`task:acceptHostOwnership` or an `acceptOwnership` call from the new owner). On secondary
chains, `<ADDR>` is the bridge adapter / governance multisig that mirrors canonical
rotations.

### Gateway-to-Ethereum migration (canonical host only)

`task:deployCanonicalHost` always uses `task:deployProtocolConfigCanonical`. For a
Gateway-side ProtocolConfig migration, run the lower-level canonical sequence manually and
substitute `task:deployProtocolConfigFromMigration` for
`task:deployProtocolConfigCanonical`. The migration task uses `MIGRATION_CONTEXT_ID`,
`MIGRATION_KMS_NODES`, and `MIGRATION_KMS_THRESHOLDS` to preserve the existing context ID.
