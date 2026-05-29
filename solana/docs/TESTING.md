# Testing the Solana PoC

How the tests are laid out, the two simulators we run them on (and why both exist), the commands
to run them, and the traps that will otherwise cost you an afternoon.

## The lay of the land

Tests live in three places:

- **`solana/runtime-tests/`** — the bulk of the coverage, run against in-process Solana VMs. Three
  suites:
  - `tests/host_events.rs` (~168 tests) — `zama-host` behavior verified through emitted **events**
    (Anchor CPI events), on **LiteSVM**.
  - `tests/host_mollusk.rs` (~40 tests) — `zama-host` behavior verified through **account state +
    inner CPIs + return data**, on **Mollusk**.
  - `tests/token_mollusk.rs` (~62 tests) — the same idea for `confidential-token`: state, events,
    SPL CPIs, settlement, on **Mollusk**.
- **Program unit tests** — small `#[test]` blocks inside each program (encoding round-trips, etc.).
- **Adapter tests** — in the other repos the host integrates with:
  - `kms-connector` — ~114 Solana-specific `kms-worker` unit tests (witness decoding, native-v0
    admission, response certificates).
  - `coprocessor/fhevm-engine` — ~14 `host-listener` `solana_adapter` tests (event decoding). Its
    LiteSVM+real-TFHE integration tests are `#[ignore]` (they need a disposable Postgres and built
    PoC programs).

Heavy emphasis on **negative tests**: most cases assert that a malformed account, an extra meta, a
wrong authority, or stale handle metadata is *rejected*. That is the point of the suite, not an
afterthought — see the hardening catalog in [`DEVELOPMENT_HISTORY.md`](./DEVELOPMENT_HISTORY.md).

## Two simulators, on purpose: Mollusk and LiteSVM

Both are in-process Solana VMs; they expose different things, so each suite uses the one that can
actually *see* what it needs to assert.

- **Mollusk** surfaces resulting **account state**, **inner instructions (CPIs)**, and **return
  data**. It's the better fit for "did this instruction write the right ACL record / make the right
  CPI / return the right handle," and it's where most coverage now lives.
- **LiteSVM** is kept for two things Mollusk can't do here:
  1. **Reading `emit!` program-data logs.** Anchor's admin/config setters (e.g.
     `set_mock_input_enabled`) emit plain `emit!` events as program-data **logs**, not as
     `emit_cpi!` inner instructions. Mollusk only exposes inner CPIs + return data + accounts, so a
     successful setter transition shows **zero** decodable events under Mollusk. Switching those
     admin events to `event_cpi` would change their account ABI, so they stay on LiteSVM. If you
     need to assert an `emit!`-only event, use LiteSVM.
  2. **Cleartext FHE arithmetic** flows that the LiteSVM-based test backend simulates.

Rule of thumb: assert on **state/CPI/return-data → Mollusk**; assert on an **`emit!` log → LiteSVM**.

## Running the suites

From `solana/`:

```bash
# Build all three programs to BPF. Exits 0 cleanly.
anchor build

# The whole Solana workspace (this is what `anchor test` runs — see Anchor.toml [scripts]).
cargo test --workspace            # ~282 tests, 0 failed

# Individual runtime suites (use --nocapture to see the program logs):
cargo test -p zama-solana-runtime-tests --test host_events
cargo test -p zama-solana-runtime-tests --test host_mollusk  -- --nocapture
cargo test -p zama-solana-runtime-tests --test token_mollusk -- --nocapture

cargo fmt --check
```

The adapters live in sibling repos and need offline SQLx metadata (no live DB):

```bash
cd ../kms-connector            && SQLX_OFFLINE=true cargo test  -p kms-worker solana_ -- --nocapture
cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo check -p host-listener
cd ../coprocessor/fhevm-engine && cargo test -p host-listener solana_adapter::tests::
```

> Note on a green test run: the suites print many `Program ... failed: custom program error: 0x...`
> lines. Those are **negative tests** asserting expected reverts — they are LiteSVM/Mollusk DEBUG
> logs, **not** test failures. The authoritative signal is the `test result: ok` summary lines and
> the process exit code.

## Traps & gotchas (read before you lose an afternoon)

- **Stale SBF artifacts.** After changing an Anchor program, **rebuild** before running runtime
  tests — a stale `.so` will make tests pass or fail against old code. (`anchor build`, or
  `anchor build --ignore-keys` if you don't want the program-id check.)
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
- **LiteSVM won't inject system-owned executable empty accounts.** It rejects that setup *before*
  program execution, so don't try to model that account shape in a LiteSVM fixture — assert the
  rejection a different way (or under Mollusk).
- **The host-listener event match is exhaustive.** If you add a new `zama-host` event, the listener
  needs an explicit arm for it — even an *ignored* one — or it won't compile. Regenerate its
  vendored IDL when host events change.
- **The connector ABI is hand-mirrored and version-pinned.** `kms-worker` re-declares the byte
  layout of host accounts (`AclRecord`, `HandleMaterialCommitment`, …), the PDA seeds, the hash
  domains, and `EVENT_VERSION`/`MAX_ACL_SUBJECTS` — with **no compile-time link** to `zama-host`.
  Change a field order, a `SPACE` constant, a seed, or a hash-domain string in the host and you must
  update the connector decoders (and the coprocessor IDL) by hand, or witness decoding breaks at
  runtime, not at build time. Lengths are checked; a same-length field reorder would *not* be caught.
