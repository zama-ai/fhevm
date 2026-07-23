# Testing the Solana PoC

How the tests are laid out, the simulator we run them on, the commands to run them, and the traps
that will otherwise cost you an afternoon.

## Evidence ladder

Use the cheapest row that can disprove the change, then move down until the changed boundary has
been exercised. Commands are run from `solana/` unless a row changes directory.

| Layer | Exact command | What it proves | What it does **not** prove | Prerequisites / cost |
| --- | --- | --- | --- | --- |
| Pure operator conformance | `cargo test -p zama-solana-runtime-tests --test operator_conformance` | The test-owned evaluator agrees with the explicit operator/type contract, including closed-world admission, operand-source rules, and rejected shapes. | SBF execution, account validation, CPIs, TFHE evaluation, randomness, or any production path. | None beyond a Rust toolchain. Warm: about one second for 379 named, filterable cases. |
| Plan/ABI contracts | `cargo test -p zama-solana-runtime-tests --test plan_contracts` | SDK plan serialization and checked-in IDL/ABI contracts used by these tests have not drifted. | Program execution, account validation, CPIs, or cryptographic behavior. | None beyond a Rust toolchain. Warm: very fast. |
| Representative SBF operator admission | `bash scripts/check-zama-host-idl.sh && cargo test -p zama-solana-runtime-tests --test operator_mollusk_conformance` | The compiled `zama-host` admits representative operator shapes, binds operands, and emits the expected handles and events; a test-owned evaluator makes the resulting computation readable. | Exhaustive operator coverage, real TFHE, database/listener behavior, or the networked stack. | Rebuilds PoC SBF artifacts. Eleven warm tests run in about 0.05 seconds; a cold SBF build is materially slower. |
| Real SBF host runtime | `bash scripts/check-zama-host-idl.sh && cargo test -p zama-solana-runtime-tests --test host_mollusk -- --nocapture` | `zama-host` SBF behavior through account state, inner CPIs, return data, and rejection paths under Mollusk. | A validator, off-chain listeners/workers, real TFHE, or the networked stack. | Rebuilds PoC SBF artifacts. Warm tests are fast; a cold SBF build is materially slower. |
| Real SBF token runtime | `bash scripts/check-zama-host-idl.sh && cargo test -p zama-solana-runtime-tests --test token_mollusk -- --nocapture` | Instruction-first confidential-token flows through real host/token/SPL CPIs, with state transitions, events, settlement, and readable domain outcomes asserted under Mollusk. | A validator, relayer/coprocessor/KMS wiring, or real TFHE. | Same SBF prerequisite and cold-build cost as the host suite. |
| Yellowstone reconstruction | `cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo test -p host-listener --features solana-reconstruct solana_reconstruct::` | Solana instruction/account decoding and deterministic reconstruction of ordinary computation and ACL records. | Yellowstone transport, born-public output recovery from the host lifecycle batch, a live validator, database insertion, worker compute, or decrypt completion. | Coprocessor workspace dependencies and offline SQLx metadata. Warm: focused; cold compilation can take minutes. |
| Solana MMR proof service | `cd ../solana-proof-service && make test` (and `make test-db` with Postgres) | Yellowstone/RPC recovery ingest, PostgreSQL store, readiness, and proof HTTP DTO. | Full vertical / production HA. | `NO_DNA=1`; offline SQLx metadata committed under the store crate. |
| KMS Solana boundary | `cd ../kms-connector && SQLX_OFFLINE=true cargo test -p kms-worker solana_ -- --nocapture` | Solana account/witness decoding and the Solana-specific user-decrypt/certificate boundary. | A live chain, real relayer delivery, or full user/public-decrypt completion. | KMS workspace dependencies and offline SQLx metadata. Warm: focused; cold compilation can take minutes. |
| Direct real-TFHE conformance | `cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo test --profile local -p fhevm-engine-common --test real_tfhe_conformance` | CPU/default-feature `perform_fhe_operation` consumes real encrypted inputs and produces typed ciphertexts that decrypt to explicit deterministic Bool, Uint8, and Uint64 oracles. It covers every operator removed from the full vertical, while grouping sibling operators into compact family tests. | Solana admission, listener/database behavior, GPU execution, random known-answer claims, or high-width scheduled coverage. | Coprocessor workspace dependencies. Warm: about 20 seconds; a cold optimized build can take minutes. |
| Manual real-TFHE worker boundary | `cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo test -p tfhe-worker tests::solana_poc::solana_confidential_transfer_with_real_ciphertexts_computes_and_decrypts -- --ignored --exact --nocapture` | A Solana confidential transfer can feed the real TFHE worker through the database and decrypt the computed ciphertexts. | Yellowstone/RPC ingestion, solana-proof-service delivery, KMS networking, or the complete deployed flow. | Intentionally ignored: testcontainers starts disposable migrated Postgres; test keys and built PoC programs are also required. Heavy and manual. |
| Full vertical | `bash scripts/e2e/clean-e2e.sh` then `bash scripts/e2e/full-vertical.sh` | The local-validator path across SDK/input proof, programs, reconstruction, coprocessor, solana-proof-service MMR proofs, and public/user decrypt. It retains one live example for each distinct eval wiring shape: encrypted/encrypted and encrypted/scalar binary, unary type conversion, ternary, bounded randomness, `Sum`, `IsIn`, and `MulDiv`. | Exhaustive operator semantics (the pure layer owns the full contract; Mollusk and direct real-TFHE supply representative SBF and cryptographic evidence), production reliability, scale, or mainnet readiness. | Docker, Solana tools, Node/Rust toolchains, ports, and a clean local stack. Before operator thinning, four successful CI samples on July 13–14, 2026 ran `full-vertical.sh` in 4m15s–4m29s; complete jobs took 44m49s–53m53s because stack setup dominated. These are observations, not SLOs. |
| Scenario layer (SDK-driven) | `cd ../test-suite/fhevm && bun run test:e2e` (stack already up) | Product arcs composed **only** through `@fhevm/sdk` Solana actions, against the live stack. Currently the confidential-transfer arc: encrypt input → `submitInputProof` → `confidentialTransfer` → current user-decrypt of both rotated balances (Alice 1000→600, Bob 0→400). | Instruction admission / guards / arithmetic / cost (Mollusk owns those), operator semantics, and provisioning correctness (mint / wrap / balance-state reads still run through the Rust live-client — see the SDK gaps below). | A stack from `clean-e2e.sh` already up, plus `bun` and the built `@fhevm/sdk`. In CI it runs as its own step right after `full-vertical.sh` (same up stack). |

The cleartext evaluator used by `operator_conformance` and `operator_mollusk_conformance` is a
test-owned model/mock. It is deliberately independent evidence for operator intent; it is not an
implementation of TFHE, cryptographic randomness, ACL or attestation enforcement, and it is not an
authority or example for production code quality. Passing it is useful only for behavior that does
not depend on those omitted boundaries.

Heavy emphasis on **negative tests**: most cases assert that a malformed account, an extra meta, a
wrong authority, or stale handle metadata is *rejected*. That is the point of the suite, not an
afterthought.

## Mollusk runtime coverage

The `operator_mollusk_conformance`, `host_mollusk`, and `token_mollusk` suites execute real SBF under
Mollusk. Mollusk surfaces resulting **account state**, **inner instructions (CPIs)**, and **return
data**, which are the stable artifacts these suites assert on. Plain `emit!` program-data logs are
intentionally not part of the runtime-test contract; tests should assert the state transition,
emitted Anchor CPI event, return data, or CPI shape that makes the behavior observable.

## Running the suites

From `solana/`:

```bash
# Verify the production IDL/ABI snapshot, then rebuild local PoC SBF
# artifacts used by Mollusk runtime tests.
bash scripts/check-zama-host-idl.sh

# The whole Solana workspace (this is what `anchor test` runs — see Anchor.toml [scripts]).
cargo test --workspace

# Individual test targets (use --nocapture to see program logs from the Mollusk targets):
cargo test -p zama-solana-runtime-tests --test operator_conformance
cargo test -p zama-solana-runtime-tests --test operator_conformance binary::add::u128::scalar -- --exact
cargo test -p zama-solana-runtime-tests --test plan_contracts
cargo test -p zama-solana-runtime-tests --test operator_mollusk_conformance
cargo test -p zama-solana-runtime-tests --test operator_mollusk_conformance encrypted_scalar_add_executes_then_reads_cleartext_outcome -- --exact
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

The report includes inline `#[cfg(test)]` modules that live in instrumented source files. Their
lines can raise a component percentage, so the table is a gap-finding signal rather than a measure
of product-code coverage.

The host-listener and relayer live in separate workspaces and are not folded into this number. Their
Solana modules need separately scoped reports in their own workflows; combining their package-wide
coverage with this workspace would not produce a meaningful floor.

The adapters live in sibling workspaces and need offline SQLx metadata (no live DB):

```bash
cd ../kms-connector            && SQLX_OFFLINE=true cargo test -p kms-worker solana_ -- --nocapture
cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo check -p host-listener
cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo test -p host-listener solana_adapter::tests::
cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo test -p host-listener --features solana-reconstruct solana_reconstruct::
cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo test --profile local -p fhevm-engine-common --test real_tfhe_conformance
cd ../solana-proof-service     && make test
```

> Note on a green test run: the suites print many `Program ... failed: custom program error: 0x...`
> lines. Those are **negative tests** asserting expected reverts, not test failures. The
> authoritative signal is the `test result: ok` summary lines and the process exit code.

## Scenario layer (SDK-driven e2e)

Lives in `test-suite/fhevm/e2e/` — a small harness (three files, not a framework) plus scenario
files. It sits **on top of** the Mollusk ladder and the full vertical, and owns only what
composition can break (proofs vs live state, KMS round-trips, relayer seams, timing) — never what
the lower layers already prove.

Runner: `bun:test`, not vitest. The standing plan (#1656) names vitest, but these scenarios reuse
`test-suite/fhevm/src/solana/*` orchestrators whose `layout.ts` is bun-native (`import.meta.dir`),
and node-based vitest workers do not provide it; vitest remains the target if/when `layout.ts` is
ported off bun APIs.

Two rules the layer holds itself to:

1. **Each behavior is tested at exactly one layer.** Mollusk owns instruction admission, guards,
   arithmetic and cost; scenarios never re-test that territory. When a leg moves here, it is
   deleted from `full-vertical.sh` in the same change — no double coverage.
2. **The harness carries zero protocol knowledge.** Scenarios reach the protocol only through
   `@fhevm/sdk` Solana actions, and assertion reads go through SDK read paths. A missing SDK
   read/action is an SDK gap to file, never something to hand-roll in the harness.

The harness (`e2e/harness/`):

- `loadEnv()` → a `TestEnv` (RPC/WS/relayer/proof-service/gateway URLs, the RFC-021 chain id, the
  zama-host ACL identity, the user-decrypt context, the coprocessor DB container, the deployer
  keypair root, and capability flags `faucet` / `freshMints` / `fastSlots`). Its source today is the
  local `clean-e2e` stack (env-var overridable); it is structured so a demo-config JSON or a
  devnet/mainnet manifest slots in as a second source without touching scenarios.
- `personas` → named actors backed by on-disk keypairs, with a capability-gated `fund()` (local
  airdrop).
- `until(condition, { timeoutMs, intervalMs })` → a generic readiness-polling helper.

Scenarios (`e2e/scenarios/`) are environment-blind: they read `TestEnv`, gate on readiness with
`until`, and drive SDK actions. `confidential-transfer.scenario.test.ts` is the confidential-transfer
arc ported from the `[sdk-transfer]` leg of `full-vertical.sh`; its assertion-to-bash mapping is in
the file header.

Run it locally against a stack that is already up (do **not** re-run `clean-e2e.sh` just for this):

```bash
# from repo root, after `bash solana/scripts/e2e/clean-e2e.sh` has left the stack up
cd test-suite/fhevm
bun run test:e2e            # the scenario suite (needs the live stack)
bun run test:e2e:harness    # the harness unit tests (until / loadEnv — no stack needed)
```

Not yet portable to this layer (SDK gaps, tracked): the disclose / redeem consume arc, whose
provisioning (wrap, burn, seal `make_handle_public`, `redeem_burned_amount`) and MMR-proof sourcing
have no SDK actions/reads yet, so it stays in `full-vertical.sh`. Confidential-mint init, wrap, and
balance-state reads used by the transfer arc's setup likewise still run through the Rust live-client.

## Traps & gotchas (read before you lose an afternoon)

- **Stale or wrong-feature SBF artifacts.** After changing an Anchor program, **rebuild** before
  running runtime tests — a stale `.so` will make tests pass or fail against old code. Prefer
  `bash scripts/check-zama-host-idl.sh`: it checks the default production IDL/ABI surface, then
  rebuilds the confidential-token artifact with its PoC-only receiver helpers. The host artifact
  has no alternate test feature or entropy path.
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
- **The host-listener event types are generated, not decoded.** Ingestion reconstructs semantic
  compute facts from instruction data. If a generated event value type changes, regenerate the
  vendored IDL and validate reconstruction explicitly; there is no emitted-event decoder fallback.
- **The connector ABI is hand-mirrored and version-pinned.** `kms-worker` re-declares the byte
  layout of host accounts (`EncryptedValue`, …), the PDA seeds, the hash
  domains, and `EVENT_VERSION`/`MAX_ACL_SUBJECTS` — with **no compile-time link** to `zama-host`.
  Change a field order, a `SPACE` constant, a seed, or a hash-domain string in the host and you must
  update the connector decoders (and the coprocessor IDL) by hand, or witness decoding breaks at
  runtime, not at build time. Lengths are checked; a same-length field reorder would *not* be caught.
