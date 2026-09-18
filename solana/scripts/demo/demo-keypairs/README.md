# Demo keypairs (well-known dev-only demo keys)

These are **throwaway, publicly-known keypairs** for the confidential-vault demo (#1760) — the same
policy as `solana/scripts/e2e/test-keypairs/` (the Solana equivalent of Anvil's well-known dev
accounts). They are **safe to commit** and are **never deployed to / funded on any public cluster**;
the demo only ever runs against a fresh local `solana-test-validator` bound to localhost.

## Persona / mint keypairs (this directory)

64-byte Solana keypair files (`[secret(32)|pubkey(32)]`, the `solana-keygen` JSON array format):

| File | Role |
| --- | --- |
| `keeper.json` | Operator that plays `dispatch` + `settle` (settle must read as an operator action, not a user button). |
| `alice.json` | End-user persona that deposits and redeems. |
| `bob.json` | Second end-user persona. |
| `mint-authority.json` | SPL mint authority for the mock-USDC faucet (`demo:faucet` mints from this key). |

The demo-config JSON carries only the **pubkeys** of these; the keys sign from these files, so a
scenario cross-checks the loaded key against the published address. `test-suite/fhevm/demo/loadDemoEnv.ts`
(`DEMO_KEYPAIRS`) points at this directory.

## Program keypairs

None. `demo_vault` and `confidential_batcher` have one program id on every cluster
(`solana/environments/preview-env.json`); the local validator loads their build at genesis with the
deployer wallet as upgrade authority (`test-suite/fhevm/src/solana/validator.ts`
`genesisDeployedPrograms`), and `deploy-demo-programs.sh` only checks or upgrades bytecode.
