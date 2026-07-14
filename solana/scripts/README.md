# Solana scripts

Run from `solana/` unless a path below says otherwise.

`e2e/` is the live full-stack vertical (CI: `solana-e2e`). It is unrelated to
Cargo `--features poc` (compile-gated test shims).

## Entrypoints

| Command | When to use | Writes? |
|---|---|---|
| `bash scripts/check-zama-host-idl.sh` | Before Mollusk tests; CI IDL/ABI parity | `target/deploy` only |
| `bash scripts/sync-zama-host-idl.sh` | After an intentional IDL/ABI change | IDL + ABI goldens |
| `bash scripts/update-cost-snapshots.sh` | After an intentional CU / ix-shape change | `runtime-tests/cost-snapshots/*.json` |
| `bash scripts/e2e/clean-e2e.sh` | Bring up a clean local vertical stack | local validator + fhevm-cli stack |
| `bash scripts/e2e/full-vertical.sh` | Drive compute → decrypt on a running stack | no checked-in goldens |
| `bash scripts/e2e/adversarial-l4.sh` | Negative live checks on a running stack | no checked-in goldens |

## Not entrypoints

| Path | Role |
|---|---|
| `check_solana_abi.py` | Called by `check-` / `sync-zama-host-idl.sh` |
| `e2e/setup-solana-side.sh` | Called by `e2e/clean-e2e.sh` after `fhevm-cli up` |
| `e2e/live-client/` | Helper crate used by the live vertical scripts |
| `e2e/test-keypairs/` | Well-known local program keypairs for reproducible deploys |
