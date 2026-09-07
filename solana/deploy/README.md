# Solana deployment

The deployer image contains compiled programs and a CLI. It does not run a validator,
listener, coprocessor or KMS. JSON-RPC endpoints and private keypairs are runtime inputs.
The same image runs directly with Docker or inside the existing `charts/contracts` Job.

```sh
docker run --rm --env-file /path/to/deployment.env solana-programs:<sha> host deploy
docker run --rm --env-file /path/to/deployment.env solana-programs:<sha> demos deploy
```

`host deploy` validates existing HostConfig/gateway bindings, deploys the host and
initializes missing host/KMS accounts. `demos deploy` deploys example programs separately.
`host upgrade` and `demos upgrade` require the selected programs to exist, upgrade their
bytecode at the same addresses, and do not reset application state. Host configuration
mismatches fail instead of silently attaching an existing host to a new gateway.

Program identities are selected at build time; changing the RPC URL does not change the
compiled identities. The current image builds the preview identities. Local native e2e
keeps its local identities. Never supply private keys to an image build.

## Preview integration

Use the component scripts/launcher from Fred's preview refactor (#3681). The Solana
values files in `ci/preview-env/solana-host` extend the existing charts:

| Values file | Chart | Install into |
| --- | --- | --- |
| `values-solana-programs-e2e.yaml` | contracts | `solana-host` |
| `values-gateway-add-host-chains-solana-e2e.yaml` | contracts | gateway registration release |
| `values-solana-register-coprocessor-e2e.yaml` | contracts | one registration Job per coprocessor database |
| `values-solana-coprocessor-e2e.yaml` | coprocessor | each existing coprocessor release |
| `values-solana-connector-e2e.yaml` | kms-connector | each existing connector release |
| `values-solana-demos-e2e.yaml` | contracts | optional `solana-demos` |

Merge these overlays with the existing environment/party values. Pin compatible image
tags from the same source revision, including the listener's Solana features and database
migration. The connector example lists one coprocessor; supply every participating
coprocessor's Service URL for a multi-party environment. Helm replaces lists, so preserve
any additional EVM chains and environment entries when composing overlays.

1. Synchronize namespace secrets using the existing `sync-secrets` chart.
2. Deploy the gateway and its signer configuration, then `solana-host`.
3. Register the Solana chain on the gateway. Complete canonical key generation.
4. Run `coprocessor register` once against each coprocessor database, with `DATABASE_URL`
   and `SOLANA_KEY_SOURCE_CHAIN_ID`. It registers the host and mirrors the canonical host's
   key material using the same SQL as local bring-up. This is preview bootstrap, not a
   production key synchronization service. On later key rotations rerun registration.
5. Start/restart workers after registration: the zkproof worker caches host chains at
   startup. Enable the Solana listener and configure the connectors' proof endpoints.
6. Confirm a controlled transaction is ingested before creating demo encrypted values.
   Then deploy the optional applications and drive their selected test scenario.

The listener stores computations, MMR leaves and its checkpoint in the existing
coprocessor database. It has one replica and uses `Recreate` to avoid overlapping writers.
Its proof API uses a ClusterIP Service and a bearer key. There is no public ingress.
Use the cluster's network policies to restrict callers as appropriate to the environment.
`/healthz` proves database/HTTP availability, not historical completeness or catch-up.

### Secrets

| Kubernetes Secret | Keys |
| --- | --- |
| `solana-deployer` | `deployer.json`, `zama_host.json`, `confidential_token.json`, `demo_vault.json`, `confidential_batcher.json` |
| `solana-rpc` | `rpc-url`, `grpc-url`, `grpc-x-token` |
| `solana-proof-api` | `api-key` shared with the participating connectors |

The host Job requires no application keys. New demo program keys must be added to the
secret store before running the optional demo Job. Program deployment does not seed demo
wallets, mints or vaults; that remains a separate test scenario. CLI keypairs can alternatively be supplied as
mounted paths through `SOLANA_DEPLOYER_KEYPAIR` and `SOLANA_<PROGRAM>_KEYPAIR`.
The image contains neither private keys nor RPC credentials.

### Repeating a rollout

The Solana values enable `scDeploy.runOnUpgrade`: every Helm revision creates a new Job.
For a program upgrade override `scDeploy.deployCommands[0]` to
`node /app/cli.mjs host upgrade` (or `demos upgrade`). Keep `scUpgrade.enabled=false`;
that chart path copies old Solidity sources and is not the Solana upgrade mechanism.
Address outputs go into distinct host/demo ConfigMaps using the chart's existing
`addresses/.env.*` format. There is no deployer PVC; the coprocessor database must persist.

Keep one active experiment per shared set of public-devnet program identities. Namespace
and secret recreation does not reset on-chain state. A fresh listener defaults to the tip;
existing values whose creation it missed cannot acquire complete proof history merely by
restarting it. For a new database, `solanaHostListener.extraArgs: ["--start-slot=<slot>"]`
selects an existing confirmed block within provider retention, before the activity to
reconstruct. An existing database checkpoint takes precedence. Short recovery depends on provider replay of transactions **and** the Clock
and SlotHashes account updates used for reconstruction. Test that against the actual
provider. If history cannot be recovered, explicitly reset the experiment instead of
continuing to submit transactions against incomplete history.

## Checks

```sh
bun test test-suite/fhevm/src/solana/host-deploy test-suite/fhevm/src/solana/deploy.test.ts
python3 ci/preview-env/solana-host/test_charts.py
docker build --platform linux/amd64 -f solana/deploy/Dockerfile -t solana-programs:local .
docker run --rm solana-programs:local --help
```

PR CI also runs the local Solana scenarios against the RFC35 base. These do not prove
Helius replay or cluster network policy. The preview acceptance test is a confidential
operation/decryption, a listener restart with retained database, then a compatible
program upgrade followed by old-value decryption and a new operation.
