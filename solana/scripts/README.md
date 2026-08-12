# Solana scripts

Run from `solana/` unless a path below says otherwise.

`e2e/` is the live full-stack vertical (CI: `solana-e2e`).

## Entrypoints

| Command | When to use | Writes? |
|---|---|---|
| `bash scripts/check-zama-host-idl.sh` | Before Mollusk tests; CI IDL/ABI parity | `target/deploy` only |
| `python3 scripts/check-pda-seeds.py` | Check explicit handwritten TypeScript/Rust PDA-seed counterparts | no |
| `bash scripts/sync-zama-host-idl.sh` | After an intentional IDL/ABI change | all four committed IDLs + ABI goldens |
| `bash scripts/update-cost-snapshots.sh` | After an intentional CU / ix-shape change | `runtime-tests/cost-snapshots/*.json` |
| `bash scripts/update-permit-vectors.sh` | After an intentional permit-canon change | `test-fixtures/permit/permit_v1.json` |
| `bash scripts/update-permit-invalidation-fixture.sh` | After an intentional `PermitInvalidation` layout / seed change | `test-fixtures/permit/permit_invalidation_account_v1.json` |
| `bash scripts/e2e/clean-e2e.sh` | Bring up a clean local vertical stack | local validator + fhevm-cli stack |

The compute → decrypt vertical, the operator wiring, the token consume arc, and the adversarial
negative checks all run as typed bun:test scenarios: `cd test-suite/fhevm && bun run test:e2e`
against a running stack (CI: `solana-e2e`).

## Not entrypoints

| Path | Role |
|---|---|
| `check_solana_abi.py` | Called by `check-` / `sync-zama-host-idl.sh`; owns the one list of committed IDLs, copying them out of `target/idl` with `--write` and comparing them back without it |
| `check_proof_store_idl.py` | Called by `check-zama-host-idl.sh` + `solana-proof-service-tests`; partitions proof-store decode.rs against vendored `zama_host` IDL |
| `e2e/test-keypairs/` | Well-known local program keypairs for reproducible deploys |
