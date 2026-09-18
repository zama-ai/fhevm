# Program environments

One file per deployed environment: the program ids compiled into the four Solana programs
(`declare_id!`) and the cargo features enabled per program for that build. `PROGRAM_ENVIRONMENT=<name>`
selects a file at build time. `.cargo/config.toml` pins it to `preview-env` for every build in this
workspace, so plain `cargo` and `anchor build` produce the shipped ids even with a stray shell
export; `scripts/build-programs.sh` overrides the pin with `--config` for one build, and a build of
another environment prints a cargo warning naming it.

A program has one id on every cluster, as with any public Solana program. The local test validator
loads the same build at genesis (`--upgradeable-program`, upgrade authority = the deployer wallet),
so no program keypair exists in the repository and localnet is not an environment of its own.

| File | Cluster | Use |
| --- | --- | --- |
| `preview-env.json` | Solana devnet, and the test validator | the ids every client compiles in; `zama_host` gets `admin-sweep`, which enables `host wipe` |

A durable Zama (zama-devnet, zama-testnet, mainnet) is a further file with its own ids (DD-051).
Chain id, RPC URLs and keypairs are runtime config and live in `ci/preview-env` and Helm
values, not here. Rationale and rejected alternatives: DESIGN_DECISIONS.md, DD-053.
