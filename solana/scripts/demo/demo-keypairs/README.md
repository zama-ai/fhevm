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

## Program keypairs (in `../../e2e/test-keypairs/`)

The two demo programs deploy from committed program keypairs alongside the other PoC program keys:

| File | Program |
| --- | --- |
| `demo_vault-keypair.json` | `demo_vault` (its pubkey is the program id, pinned in `declare_id!`). |
| `confidential_batcher-keypair.json` | `confidential_batcher` (pubkey = program id, pinned in `declare_id!`). |

`deploy-demo-programs.sh` seeds `target/deploy/` from there so `anchor build --ignore-keys` +
`solana program deploy` produce programs at exactly the `declare_id!` ids the SDK/config expect —
the same pattern `setup-solana-side.sh` uses for `zama_host` / `confidential_token`.
