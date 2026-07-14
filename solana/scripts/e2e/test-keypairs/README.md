# PoC program keypairs (well-known dev-only test keys)

These are **throwaway, publicly-known test keypairs** for the Solana PoC programs — the
Solana equivalent of Anvil's well-known dev accounts. They are **safe to commit**:

- They are **program keypairs**: their public keys are the program IDs (pinned in each
  program's `declare_id!`), and their private keys are only the *upgrade authority* of the
  programs **on a fresh local `solana-test-validator`**.
- They hold **no funds** and are **never deployed to any public cluster** (devnet/testnet/
  mainnet). The PoC validator is reset (`--reset`) on every run and bound to localhost.
- Committing them makes the e2e self-reproducible: `setup-solana-side.sh` seeds
  `target/deploy/` from here so `cargo build-sbf` + `solana program deploy` produce programs
  at exactly the `declare_id!` IDs the harness/SDK expect.

Do NOT reuse these keys for anything other than the local PoC. To rotate: regenerate with
`solana-keygen new -o <name>-keypair.json`, update each `declare_id!` + the hardcoded
`ACL`/`CONTRACT` constants to the new pubkeys, and rebuild.
