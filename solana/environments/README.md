# Program environments

One file per deployed environment: the program ids compiled into the four Solana programs
(`declare_id!`) and the cargo features enabled per program for that build. `PROGRAM_ENVIRONMENT=<name>`
selects a file at build time; unset means `localnet`, so plain `cargo` and `anchor build`
produce the test programs.

| File | Cluster | Use |
| --- | --- | --- |
| `localnet.json` | test validator | committed test identities (`scripts/e2e/test-keypairs`) |
| `preview-env.json` | Solana devnet | the disposable preview host; `zama_host` gets `admin-sweep`, which enables `host wipe` |

Chain id, RPC URLs and keypairs are runtime config and live in `ci/preview-env` and Helm
values, not here. Rationale and rejected alternatives: DESIGN_DECISIONS.md, DD-053.
