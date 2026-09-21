# Demo keypairs (well-known dev-only demo keys)

These are **throwaway, publicly-known keypairs** for the confidential-vault demo (#1760) — the same
policy as `solana/scripts/e2e/test-keypairs/` (the Solana equivalent of Anvil's well-known dev
accounts). They are public localnet fixtures. Older preview runs used these actors on devnet;
recovery imports them only to reclaim that historical funding. New devnet runs generate
private actors, save them in `SOLANA_RECOVERY_DIR`, and back them up to the preview Secret
before funding. These files are not the private program/deployer keys held in AWS/1Password.

## Persona / mint keypairs (this directory)

64-byte Solana keypair files (`[secret(32)|pubkey(32)]`, the `solana-keygen` JSON array format):

| File | Role |
| --- | --- |
| `keeper.json` | Operator that plays `dispatch` + `settle` (settle must read as an operator action, not a user button). |
| `alice.json` | End-user persona that deposits and redeems. |
| `bob.json` | Second end-user persona. |
| `mint-authority.json` | SPL mint authority for the mock-USDC faucet (`demo:operator` mints from this key). |

The demo-config JSON carries only the **pubkeys** of these; the keys sign from these files, so a
scenario cross-checks the loaded key against the published address. `test-suite/fhevm/demo/loadDemoEnv.ts`
(`demoKeypairs(env)`) selects this directory only on localnet.

## Program keypairs

None. `demo_vault` and `confidential_batcher` have one program id on every cluster
(`solana/environments/preview-env.json`); the local validator loads their build at genesis with the
deployer wallet as upgrade authority (`test-suite/fhevm/src/solana/validator.ts`
`genesisDeployedPrograms`), and `deploy-demo-programs.sh` only checks or upgrades bytecode.
