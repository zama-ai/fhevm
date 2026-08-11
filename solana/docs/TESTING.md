# Testing the Solana PoC

How the tests are laid out, the simulator we run them on, the commands to run them, and the traps
that will otherwise cost you an afternoon.

## Evidence ladder

Use the cheapest row that can disprove the change, then move down until the changed boundary has
been exercised. Commands are run from `solana/` unless a row changes directory.

| Layer | Exact command | What it proves | What it does **not** prove | Prerequisites / cost |
| --- | --- | --- | --- | --- |
| Pure operator conformance | `cargo test -p zama-solana-runtime-tests --test operator_conformance` | The test-owned evaluator agrees with the explicit operator/type contract, including closed-world admission, operand-source rules, and rejected shapes. | SBF execution, account validation, CPIs, TFHE evaluation, randomness, or any production path. | None beyond a Rust toolchain. Warm: about one second for 379 named, filterable cases. |
| Execution/ABI contracts | `cargo test -p zama-solana-runtime-tests --test execution_contracts` | SDK execution serialization and checked-in IDL/ABI contracts used by these tests have not drifted. | Program execution, account validation, CPIs, or cryptographic behavior. | None beyond a Rust toolchain. Warm: very fast. |
| Representative SBF operator admission | `bash scripts/check-zama-host-idl.sh && cargo test -p zama-solana-runtime-tests --test operator_mollusk_conformance` | The compiled `zama-host` admits representative operator shapes, binds operands, and emits the expected handles and events; a test-owned evaluator makes the resulting computation readable. | Exhaustive operator coverage, real TFHE, database/listener behavior, or the networked stack. | Rebuilds PoC SBF artifacts. Eleven warm tests run in about 0.05 seconds; a cold SBF build is materially slower. |
| Real SBF host runtime | `bash scripts/check-zama-host-idl.sh && cargo test -p zama-solana-runtime-tests --test host_mollusk -- --nocapture` | `zama-host` SBF behavior through account state, inner CPIs, return data, and rejection paths under Mollusk. | A validator, off-chain listeners/workers, real TFHE, or the networked stack. | Rebuilds PoC SBF artifacts. Warm tests are fast; a cold SBF build is materially slower. |
| Real SBF token runtime | `bash scripts/check-zama-host-idl.sh && cargo test -p zama-solana-runtime-tests --test token_mollusk -- --nocapture` | Instruction-first confidential-token flows through real host/token/SPL CPIs, with state transitions, events, settlement, and readable domain outcomes asserted under Mollusk. | A validator, relayer/coprocessor/KMS wiring, or real TFHE. | Same SBF prerequisite and cold-build cost as the host suite. |
| Yellowstone reconstruction | `cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo test -p host-listener --features solana-reconstruct solana_reconstruct::` | Solana instruction/account decoding and deterministic reconstruction of ordinary computation and ACL records. | Yellowstone transport, created-public output recovery from the host lifecycle execution, a live validator, database insertion, worker compute, or decrypt completion. | Coprocessor workspace dependencies and offline SQLx metadata. Warm: focused; cold compilation can take minutes. |
| Solana MMR proof service | `cd ../solana-proof-service && make test` (and `make test-db` with Postgres) | Yellowstone/RPC recovery ingest, PostgreSQL store, readiness, and proof HTTP DTO. | Full vertical / production HA. | `NO_DNA=1`; offline SQLx metadata committed under the store crate. |
| KMS Solana boundary | `cd ../kms-connector && SQLX_OFFLINE=true cargo test -p kms-worker solana_ -- --nocapture && SQLX_OFFLINE=true cargo test -p connector-utils --test solana_user_decrypt_byte_vectors` | Solana account/witness decoding, the Solana-specific user-decrypt/certificate boundary, and the committed byte-layout vectors (`solana/test-fixtures/user-decrypt/`) the Rust and TypeScript codec mirrors both assert against. | A live chain, real relayer delivery, or full user/public-decrypt completion. | KMS workspace dependencies and offline SQLx metadata. Warm: focused; cold compilation can take minutes. |
| Direct real-TFHE conformance | `cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo test --profile local -p fhevm-engine-common --test real_tfhe_conformance` | CPU/default-feature `perform_fhe_operation` consumes real encrypted inputs and produces typed ciphertexts that decrypt to explicit deterministic Bool, Uint8, and Uint64 oracles. It covers every operator removed from the full vertical, while grouping sibling operators into compact family tests. | Solana admission, listener/database behavior, GPU execution, random known-answer claims, or high-width scheduled coverage. | Coprocessor workspace dependencies. Warm: about 20 seconds; a cold optimized build can take minutes. |
| Real-TFHE worker vertical | `cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo test -p tfhe-worker tests::solana_vertical -- --ignored --nocapture` | A LiteSVM confidential transfer, reconstructed off-chain, can feed the real TFHE worker through the database and decrypt the computed ciphertexts — the one test that crosses from Solana transaction metadata to cleartexts with no deployed stack. | Yellowstone/RPC ingestion, solana-proof-service delivery, KMS networking, or the complete deployed flow. | `#[ignore]`d in the default lanes (needs Docker for the disposable migrated Postgres, the LFS test keys, and anchor-built `zama_host`/`confidential_token` artifacts). CI runs it on every PR in solana-e2e's `worker-vertical` job. |
| Live scenario vertical (SDK-driven) | `bun run demo up` from the repo root, then `cd test-suite/fhevm && bun run test:e2e` | Product arcs composed **only** through `@fhevm/sdk` Solana actions and the typed Codama clients, against the running stack: the decrypt vertical (input ZK proof → on-chain secp256k1 bind → compute → public decrypt → pure-SDK user decrypt), the `fhe_execute` operator wiring, the confidential-transfer arc, the token consume arc (wrap → attested burn → seal → certified public decrypt → redeem → disclose) with its adversarial context-mismatch tail, and the dependency-chain load smoke. Every assertion is typed; nothing greps logs. | Exhaustive operator semantics (the pure layer owns the full contract; Mollusk and direct real-TFHE supply representative SBF and cryptographic evidence), instruction admission/guards/cost (Mollusk owns those), production reliability, scale, or mainnet readiness. | Docker, Solana tools, Node/Rust toolchains, ports. `bun run demo up` drives `clean-e2e.sh` (image builds/pulls, validator + geyser, typed side-stack deploy from `test-suite/fhevm/src/solana/deploy.ts`). CI's solana-e2e lane runs the suite plus the demo phases; its history: median successful runs ~50–53 min, observed tail 72. |

After rebuilding `sdk/js-sdk/src`, refresh the test-suite SDK snapshot with
`solana/scripts/e2e/materialize-test-sdk.sh` and restart any long-lived test process.

The cleartext evaluator used by `operator_conformance` and `operator_mollusk_conformance` is a
test-owned model/mock. It is deliberately independent evidence for operator intent; it is not an
implementation of TFHE, cryptographic randomness, ACL or attestation enforcement, and it is not an
authority or example for production code quality. Passing it is useful only for behavior that does
not depend on those omitted boundaries.

Heavy emphasis on **negative tests**: most cases assert that a malformed account, an extra meta, a
wrong authority, or stale handle metadata is *rejected*. That is the point of the suite, not an
afterthought.

## Mollusk runtime coverage

The `operator_mollusk_conformance`, `host_mollusk`, `token_mollusk`, `batcher_mollusk`,
`vault_mollusk`, `permit_invalidation_mollusk`, `disclose_packet_fit`, and specimen
(`counter_mollusk`, `dep_chain_mollusk`) suites execute real SBF under Mollusk, booted and
asserted through the shared `zama-solana-test-kit` crate. Mollusk surfaces resulting **account state**, **inner instructions (CPIs)**, and **return
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
cargo test -p zama-solana-runtime-tests --test execution_contracts
cargo test -p zama-solana-runtime-tests --test operator_mollusk_conformance
cargo test -p zama-solana-runtime-tests --test operator_mollusk_conformance encrypted_scalar_add_executes_then_reads_cleartext_outcome -- --exact
cargo test -p zama-solana-runtime-tests --test host_mollusk -- --nocapture
cargo test -p zama-solana-runtime-tests --test token_mollusk -- --nocapture
cargo test -p zama-solana-runtime-tests --test batcher_mollusk -- --nocapture
cargo test -p zama-solana-runtime-tests --test vault_mollusk -- --nocapture
cargo test -p zama-solana-runtime-tests --test permit_invalidation_mollusk -- --nocapture
# Disclosure packet sizing: the largest disclose payload still fits its transport budget.
cargo test -p zama-solana-runtime-tests --test disclose_packet_fit -- --nocapture

# The specimen consumers: encrypted-counter is the kit-onboarding proof (~20 lines of
# fixture, ~30 per assertion); dep-chain is the load shape (full-depth dependent chains
# through one fhe_execute, replayed by the kit's cleartext oracle).
cargo test -p zama-solana-runtime-tests --test counter_mollusk
cargo test -p zama-solana-runtime-tests --test dep_chain_mollusk

cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
```

#### Renaming anything shared: the three other roots that can see it

`cargo test --workspace` here covers exactly one Cargo workspace root. The
repository has more than a dozen, so the number is not the useful fact — what
matters is which ones can see a Solana change, and the criterion is a path
dependency on a crate under `solana/`. Three do: `solana-proof-service`,
`coprocessor/fhevm-engine` (host-listener, tfhe-worker), and `kms-connector`.
Everything else in the repo — `relayer`, `sdk/rust-sdk`, `shared/*`,
`test-suite/gateway-stress`, the generated `*_bindings` — depends on no Solana
crate and cannot break from one.

Do not enumerate these roots by grepping for a `[workspace]` stanza; that misses
implicit roots (`relayer/Cargo.toml` has no stanza and no parent claims it, so
cargo treats it as its own). Ask cargo instead:
`cargo metadata --no-deps --format-version 1 | jq -r .workspace_root`.

A rename that reaches a shared crate compiles cleanly here and still breaks the
build in those three, because they are invisible to this workspace:

```bash
# Own workspace, own fmt gate.
(cd ../solana-proof-service && cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings)

# Path-depend on zama-host / confidential-token / zama-solana-acl. `--all-targets`
# matters: the sites that break are usually `#[cfg(test)]`, so a plain
# `cargo check` or `cargo build` passes while `cargo test -p …` does not compile.
(cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo check --workspace --all-targets)
(cd ../kms-connector && SQLX_OFFLINE=true cargo check --workspace --all-targets)
```

Each of those roots hid a real break at least once. The grep sweeps in
`scripts/dead-surface-check.sh` cover some of the same trees, but grep does not
typecheck — a root can be swept and still never compiled.

### Native unit coverage

CI publishes component-level native Rust line coverage. Run the same measurement locally with:

```bash
cargo llvm-cov \
  --workspace \
  --exclude zama-solana-runtime-tests \
  --exclude zama-solana-test-kit \
  --json \
  --summary-only \
  --output-path /tmp/solana-native-coverage.json
```

This is intentionally an informational signal without a coverage floor. Mollusk executes the
programs from prebuilt SBF artifacts, not the instrumented native libraries, so it cannot attribute
runtime execution to the program instruction source. Including `zama-solana-runtime-tests` or
`zama-solana-test-kit` would instead count the Rust test harness and make the total look healthier
without measuring more on-chain code. Use the component table to find native unit-test gaps, and use the Mollusk suites to
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

Lives in `test-suite/fhevm/e2e/` — a small harness plus scenario files. Since fhevm-internal#1876
this layer **is** the live vertical: the bash phase runner (`full-vertical.sh`), its label greps
and their checker, and the Rust live-client are gone, and every live assertion is a typed
`bun:test` expectation. The layer owns only what composition can break (proofs vs live state, KMS
round-trips, relayer seams, timing) — never what the Mollusk ladder already proves.

The scenarios run under `bun:test` because they share their runtime with the fhevm-cli demo
lifecycle and the `src/solana/*` orchestrators, which are bun-native (`Bun.spawn`, `Bun.sleep`,
`import.meta.dir`).

Two rules the layer holds itself to:

1. **Each behavior is tested at exactly one layer.** Mollusk owns instruction admission, guards,
   arithmetic and cost; scenarios never re-test that territory.
2. **Scenarios reach the protocol through `@fhevm/sdk` Solana actions and the generated Codama
   clients** (`test-suite/fhevm/src/solana/internal/generated`, rendered from the committed IDLs —
   never hand-rolled instruction bytes). A missing SDK read/action is an SDK gap to file.

The harness (`e2e/harness/`):

- `loadEnv()` → a `TestEnv` (RPC/WS/relayer/proof-service/gateway URLs, the RFC-021 chain id, the
  zama-host ACL identity, the user-decrypt context, the coprocessor DB container, the deployer
  keypair root, and capability flags `faucet` / `freshMints` / `fastSlots`). Its source today is the
  lifecycle-owned stack (env-var overridable); it is structured so a demo-config JSON or a
  devnet/mainnet manifest slots in as a second source without touching scenarios.
- `personas` → named actors backed by on-disk keypairs, with a capability-gated `fund()` (local
  airdrop).
- `until(condition, { timeoutMs, intervalMs })` → a generic readiness-polling helper.
- `harness/solana/stack.ts` → the running stack as an object: container/URL readiness and
  `restartProofService()` (the #1682/#3215 ledger-replay gate). It owns readiness, not lifecycle —
  `bun run demo up`/`down` start and stop the stack.
- `harness/solana/vertical.ts` → `verticalSetup()`: a fresh provisioning context, funded wallet,
  and host-config read per call — one wallet per scenario keeps them fully isolated.
- `harness/solana/sdkEncrypt.ts` → the SDK encrypt+input-proof seam shared by every scenario that
  submits an encrypted input.

Scenarios (`e2e/scenarios/`), each with its retired-assertion mapping in the file header where it
replaced a bash phase:

- `fhe-vertical` — trivial-encrypt → public decrypt + pure-SDK user decrypt, historical decrypt of
  an updated-away handle, the verified-input flow; starts by restarting the proof service so every
  proof the suite consumes is served by a ledger-replayed service.
- `operators` — the live 8-op `fhe_execute` wiring sweep (one example per execution wiring shape;
  semantics live in the pure layer).
- `confidential-transfer` — encrypt input → `submitInputProof` → `confidentialTransfer` → user
  decrypt of both rotated balances.
- `token-vertical` — the consume arc: wrap → attested burn → seal → certified public decrypt →
  redeem (SPL balance-delta asserted) → disclose, plus the adversarial context-mismatch tail
  pinned to `InvalidKmsContext`. That tail is the L4-b attack. Its sibling **L4-a** (a forged
  KMS signature over a well-formed certificate) is exercised in the **kms repo's live harness**,
  not here: it needs a KMS that will sign attacker-chosen material, which this stack's KMS will
  not do. Cross-repo coverage with no pointer is how coverage quietly stops running, so if you
  are auditing the adversarial surface, look there for L4-a rather than concluding it is absent.
- `load-smoke` — the dep-chain shape live: one 32-step strictly dependent execution with an
  unrelated release alongside.
- `deposit-arc` — the confidential-vault demo arc; gated behind `RUN_DEMO_SCENARIOS` and run by
  the demo phase of the CI job (`bun run demo:smoke`).

Run it locally against a stack that is already up (do **not** re-run the bring-up just for this):

```bash
# from repo root, after `bun run demo up` has left the stack up
cd test-suite/fhevm
bun run test:e2e            # the scenario suite (needs the live stack)
bun run test:e2e:harness    # the harness unit tests (loadEnv / personas — no stack needed)
bun test src/utils          # the shared utilities, including until()'s timeout contract
```

## Traps & gotchas (read before you lose an afternoon)

- **Stale or wrong-feature SBF artifacts.** After changing an Anchor program, **rebuild** before
  running runtime tests — a stale `.so` will make tests pass or fail against old code. Prefer
  `bash scripts/check-zama-host-idl.sh`: it checks the default production IDL/ABI surface, then
  rebuilds the confidential-token artifact with its PoC-only receiver helpers. The host artifact
  has no alternate test feature or entropy path.
- **A small CU delta after an incremental SBF build is not a code change.** The committed
  baselines are minted by `scripts/update-cost-snapshots.sh`, which runs `cargo clean` first. An
  incremental rebuild of the same source can differ by a few CU: a doc-comment-only edit to
  `zama-host` was measured at −12 CU on the batcher's `open_batch` and `redeem_open_batch` after
  `sync-zama-host-idl.sh` (incremental), and byte-identical to the baselines after the snapshot
  script's clean rebuild. So regenerate with the script before believing a delta of this size, and
  do not attribute it to the edit in front of you.
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
  layout of host accounts (`EncryptedValue`, …), the PDA seeds, and the hash
  domains — with **no compile-time link** to `zama-host` (the subject cap is the exception:
  it comes from the shared `zama_solana_acl` crate).
  Change a field order, a `SPACE` constant, a seed, or a hash-domain string in the host and you must
  update the connector decoders (and the coprocessor IDL) by hand, or witness decoding breaks at
  runtime, not at build time. Lengths are checked; a same-length field reorder would *not* be caught.
  The user-decrypt side of the mirror — the ed25519 signing message and the `extraData` blob,
  hand-mirrored between the connector's Rust and the SDK's TypeScript — is pinned by the committed
  byte vectors in `solana/test-fixtures/user-decrypt/`, which both sides assert against; moving
  those bytes is a protocol change (new domain tag / version byte), not a fixture refresh.
