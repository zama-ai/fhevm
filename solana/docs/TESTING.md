# Testing the Solana PoC

How the tests are laid out, the simulator we run them on, the commands to run them, and the traps
that will otherwise cost you an afternoon.

## The lay of the land

Tests live in three places:

- **`solana/runtime-tests/`** — the bulk of the coverage, run against Mollusk. Two suites:
  - `tests/host_mollusk.rs` — `zama-host` behavior verified through **account state +
    inner CPIs + return data**, on **Mollusk**.
  - `tests/token_mollusk.rs` — the same idea for `confidential-token`: state, events, SPL CPIs,
    settlement, on **Mollusk**.
- **Program unit tests** — small `#[test]` blocks inside each program (encoding round-trips, etc.).
- **Adapter tests** — in the other repos the host integrates with:
  - `kms-connector` — Solana-specific `kms-worker` unit tests (witness decoding, response
    certificates). Note: the prior `native-v0` admission subsystem is no longer the chosen decrypt
    path — decrypt reuses the Gateway V2 / EVM stack with on-chain secp256k1 cert verification
    (DESIGN_DECISIONS.md DD-012/DD-021); some native-v0 tests remain for the legacy library boundary.
  - `coprocessor/fhevm-engine` — `host-listener` `solana_adapter` tests (event decoding). Its
    real-TFHE Solana integration tests are `#[ignore]` (they need a disposable Postgres and built
    PoC programs).

Heavy emphasis on **negative tests**: most cases assert that a malformed account, an extra meta, a
wrong authority, or stale handle metadata is *rejected*. That is the point of the suite, not an
afterthought.

## Mollusk Runtime Coverage

The Solana runtime tests use Mollusk only. Mollusk surfaces resulting **account state**, **inner
instructions (CPIs)**, and **return data**, which are the stable artifacts these tests assert on.
Plain `emit!` program-data logs are intentionally not part of the runtime-test contract; tests
should assert the state transition, emitted Anchor CPI event, return data, or CPI shape that makes
the behavior observable.

## Running the suites

From `solana/`:

```bash
# Verify the production IDL/ABI snapshot, then rebuild local PoC SBF
# artifacts used by Mollusk runtime tests.
bash scripts/check-zama-host-idl.sh

# The whole Solana workspace (this is what `anchor test` runs — see Anchor.toml [scripts]).
cargo test --workspace

# Individual runtime suites (use --nocapture to see the program logs):
cargo test -p zama-solana-runtime-tests --test host_mollusk -- --nocapture
cargo test -p zama-solana-runtime-tests --test token_mollusk -- --nocapture

cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
```

### Native unit coverage

CI publishes component-level native Rust line coverage. Run the same measurement locally with:

```bash
cargo llvm-cov \
  --workspace \
  --exclude zama-solana-runtime-tests \
  --json \
  --summary-only \
  --output-path /tmp/solana-native-coverage.json
```

This is intentionally an informational signal without a coverage floor. Mollusk executes the
programs from prebuilt SBF artifacts, not the instrumented native libraries, so it cannot attribute
runtime execution to the program instruction source. Including `zama-solana-runtime-tests` would
instead count the Rust test harness and make the total look healthier without measuring more
on-chain code. Use the component table to find native unit-test gaps, and use the Mollusk suites to
validate account, CPI, PDA, ACL, event, and persistence behavior.

The host-listener and relayer live in separate workspaces and are not folded into this number. Their
Solana modules need separately scoped reports in their own workflows; combining their package-wide
coverage with this workspace would not produce a meaningful floor.

The adapters live in sibling workspaces and need offline SQLx metadata (no live DB):

```bash
cd ../kms-connector            && SQLX_OFFLINE=true cargo test -p kms-worker solana_ -- --nocapture
cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo check -p host-listener
cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo test -p host-listener solana_adapter::tests::
```

> Note on a green test run: the suites print many `Program ... failed: custom program error: 0x...`
> lines. Those are **negative tests** asserting expected reverts, not test failures. The
> authoritative signal is the `test result: ok` summary lines and the process exit code.

## Traps & gotchas (read before you lose an afternoon)

- **Stale or wrong-feature SBF artifacts.** After changing an Anchor program, **rebuild** before
  running runtime tests — a stale `.so` will make tests pass or fail against old code. Prefer
  `bash scripts/check-zama-host-idl.sh`: it checks the default production IDL/ABI surface, then
  rebuilds the ignored `target/deploy` artifacts with the PoC-only host/token features that Mollusk
  tests exercise. Plain `anchor build` is fine for production artifacts, but it does not include the
  local-only runtime-test shims.
- **SPL Token CPIs in token tests.** `token_mollusk` executes real SPL Token CPIs through the
  matching `mollusk-svm-programs-token` program fixture.
- **`anchor build` vs program ids.** `anchor build` checks that each program's declared id matches
  its `target/deploy/*-keypair.json`. The canonical keypairs aren't committed, so if those drift you
  get a "Program ID mismatch" error. Fixes: `anchor keys sync` (rewrites the declared ids to match
  the keypairs — then update the coprocessor's vendored `host-listener/idl/zama_host.json`
  `"address"` to match, since that's the one external reference to the host id), or
  `anchor build --ignore-keys` to skip the check entirely. The BPF compile itself is unaffected.
- **Keep cargo verification mostly sequential.** The workspace and the BPF build share target dirs;
  running several cargo invocations at once causes build-lock waits, not speedups.
- **Connector/coprocessor need `SQLX_OFFLINE=true`.** They have SQLx-checked queries; without the
  env var and a live DB they won't compile.
- **The host-listener event match is exhaustive.** If you add a new `zama-host` event, the listener
  needs an explicit arm for it — even an *ignored* one — or it won't compile. Regenerate its
  vendored IDL when host events change.
- **The connector ABI is hand-mirrored and version-pinned.** `kms-worker` re-declares the byte
  layout of host accounts (`EncryptedValue`, …), the PDA seeds, the hash
  domains, and `EVENT_VERSION`/`MAX_ACL_SUBJECTS` — with **no compile-time link** to `zama-host`.
  Change a field order, a `SPACE` constant, a seed, or a hash-domain string in the host and you must
  update the connector decoders (and the coprocessor IDL) by hand, or witness decoding breaks at
  runtime, not at build time. Lengths are checked; a same-length field reorder would *not* be caught.
