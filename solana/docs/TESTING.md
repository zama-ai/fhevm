# Testing the Solana port

How the tests are laid out, the simulator we run them on, the commands to run them, and the traps
that will otherwise cost you an afternoon.

## Evidence ladder

Use the cheapest row that can disprove the change, then move down until the changed boundary has
been exercised. Commands are run from `solana/` unless a row changes directory.

| Layer                                 | Exact command                                                                                                                                                                      | What it proves                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | What it does **not** prove                                                                                                                                                                                                                                             | Prerequisites / cost                                                                                                                                                                                                                                                                                                                   |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Pure operator conformance             | `cargo test -p zama-solana-runtime-tests --test operator_conformance`                                                                                                              | The host's cleartext evaluator (`zama_host::cleartext`) agrees with the explicit operator/type contract, including closed-world admission, operand-source rules, and rejected shapes, and computes the outputs of the EVM e2e's operator cases (`library-solidity/codegen/overloads/e2e.json`, which the EVM e2e checks against the real coprocessor) for every operand pair Solana takes.                                                                                                                                                                                                                                                                                                                                                                                                                                      | SBF execution, account validation, CPIs, TFHE evaluation, randomness, or any production path.                                                                                                                                                                          | None beyond a Rust toolchain. Warm: about one second for 328 named, filterable cases.                                                                                                                                                                                                                                                  |
| Execution/ABI contracts               | `cargo test -p zama-solana-runtime-tests --test execution_contracts`                                                                                                               | SDK execution serialization and checked-in IDL/ABI contracts used by these tests have not drifted.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Program execution, account validation, CPIs, or cryptographic behavior.                                                                                                                                                                                                | None beyond a Rust toolchain. Warm: very fast.                                                                                                                                                                                                                                                                                         |
| Representative SBF operator admission | `bash scripts/check-zama-host-idl.sh && cargo test -p zama-solana-runtime-tests --test operator_mollusk_conformance`                                                               | The compiled cleartext host build admits representative operator shapes, binds operands, emits the expected handles and events, and records the resulting plaintexts in the output store, where tests read them.                                                                                                                                                                                                                                                                                                                                                                                                 | Exhaustive operator coverage, real TFHE, database/listener behavior, or the networked stack.                                                                                                                                                                           | Rebuilds the SBF artifacts. Eleven warm tests run in about 0.05 seconds; a cold SBF build is materially slower.                                                                                                                                                                                                                        |
| Real SBF host runtime                 | `bash scripts/check-zama-host-idl.sh && cargo test -p zama-solana-runtime-tests --test host_mollusk -- --nocapture`                                                                | `zama-host` SBF behavior through account state, inner CPIs, return data, and rejection paths under Mollusk.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | A validator, off-chain listeners/workers, real TFHE, or the networked stack.                                                                                                                                                                                           | Rebuilds the SBF artifacts. Warm tests are fast; a cold SBF build is materially slower.                                                                                                                                                                                                                                                |
| Host capability properties            | `bash scripts/check-zama-host-idl.sh && cargo test -p zama-solana-runtime-tests --test capability_invariants` | Over random sequences drawn from every host IDL instruction, with each admin and Store-authority role also filled by keys that lack it: trust roots change only when the admin signed, and a Store only when its authority signed, in the default build (INVARIANTS #11, #35). `bash scripts/check-planted-bugs.sh` rebuilds the host with each patch in `runtime-tests/planted-bugs/` and requires the suite to fail. | Authorization inside other programs, or that a signer was entitled to sign: Mollusk takes signer flags at face value. | About 10 seconds warm. The planted-bug check rebuilds `zama-host` once per patch. |
| Real SBF token runtime                | `bash scripts/check-zama-host-idl.sh && cargo test -p zama-solana-runtime-tests --test token_mollusk -- --nocapture`                                                               | Instruction-first confidential-token flows through real host/token/SPL CPIs, with state transitions, events, settlement, and readable domain outcomes asserted under Mollusk. Behavior runs on the cleartext host build, which creates every store at full size; the four `cost_snapshot_*` tests run the production build, so store creation and growth are exercised there and in the host suite.                                                                                                                                                                                                                                                                                                                                                                                                               | A validator, relayer/coprocessor/KMS wiring, or real TFHE.                                                                                                                                                                                                             | Same SBF prerequisite and cold-build cost as the host suite.                                                                                                                                                                                                                                                                           |
| Solana host decoding | `cd ../coprocessor/fhevm-engine && cargo test -p solana-host-follower host:: && SQLX_OFFLINE=true cargo test -p host-listener --features solana solana_reconstruct::` | Host instruction decoding, pairing each execution with its `FheExecutedEvent` and its steps, and the encrypted-store writes (`solana-host-follower`); the handle check and deterministic reconstruction of ordinary computation (`host-listener`). | Yellowstone transport, a live validator, database insertion, worker compute, or decrypt completion. | Coprocessor workspace dependencies and offline SQLx metadata. Warm: focused; cold compilation can take minutes. |
| Solana block ingest | `cd ../coprocessor/fhevm-engine && cargo test -p solana-host-follower && SQLX_OFFLINE=true cargo test -p host-listener --features solana solana_listener::` | Streamed and `getTransaction` transactions prepare into the same host instructions, and a slot rebuilds from `getBlock` and `getTransaction` output alone. Each streamed message of a slot past the 64 MiB decoding limit stays far below it, and only host instructions are kept. The validator seals a slot on its block meta, skips a re-delivered slot, checked while it is in the validator's window, and a tip start's first slot, and stops on a late transaction, an out-of-order slot or a broken ancestry. Archive catch-up across a skipped slot hands the sink the blocks uninterrupted streaming does, and a sink's retryable failure replays its block while a fatal one stops the follower. Against Postgres: a step whose emitted handle does not re-derive is held back as a terminal error while the rest of the block commits and the alarm counter moves, and a slot reverted with the operator scripts replays with fresh rows. | A live provider, a live archive RPC, or the tfhe-worker ending the held step's dependents (the worker's own `errors` and `tfhe_worker` tests cover that). | Docker for the disposable Postgres; offline SQLx metadata. |
| Solana Merkle proof service           | `cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo test -p solana-merkle-proof-service`                                         | The leaf record against a real Postgres: rows round-trip through the crate's migration, and `POST /v1/solana/merkle-proofs` answers with paths read by position from the stored nodes, which verify against the recorded peaks. A record built through a failure at each table's write equals the uninterrupted one, a replayed block must reproduce the record, and a restored `pg_dump` resumes and catches up (DD-066). `--test store_tests -- --ignored` adds the 1,000,000-leaf timing (DD-063).                                                                                                                                                                                                                                                                                                                                                                                         | Yellowstone ingest, the connector's verification, or the full vertical.                                                                                                                                                                                                | Docker for the disposable Postgres; offline SQLx metadata.                                                                                                                                                                                                                                                                             |
| KMS Solana boundary                   | `cd ../kms-connector && SQLX_OFFLINE=true cargo test -p kms-worker solana -- --nocapture`, then `SQLX_OFFLINE=true cargo test -p kms-worker --test solana_authorization_cases` (its test name does not match the filter) | The Solana authorization pipeline as pure functions (snapshot, scope, Merkle proof read and merge, handle binding, delegation), including each public-decrypt handle proven against the store it names; the second command checks that the committed decrypt cases are the Connector's verdicts.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | A live chain, a live coprocessor leaf record, real relayer delivery, or full user/public-decrypt completion.                                                                                                                                                           | KMS workspace dependencies and offline SQLx metadata. Warm: focused; cold compilation can take minutes.                                                                                                                                                                                                                                |
| Direct real-TFHE conformance          | `cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo test --profile local -p fhevm-engine-common --test real_tfhe_conformance`                                               | CPU/default-feature `perform_fhe_operation` consumes real encrypted inputs and produces typed ciphertexts that decrypt to explicit deterministic Bool, Uint8, and Uint64 oracles. It covers every operator removed from the full vertical, while grouping sibling operators into compact family tests.                                                                                                                                                                                                                                                                                      | Solana admission, listener/database behavior, GPU execution, random known-answer claims, or high-width scheduled coverage.                                                                                                                                             | Coprocessor workspace dependencies. Warm: about 20 seconds; a cold optimized build can take minutes.                                                                                                                                                                                                                                   |
| Real-TFHE worker vertical             | `cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo test -p tfhe-worker tests::solana_vertical -- --ignored --nocapture`                                                    | A LiteSVM confidential transfer, reconstructed off-chain, can feed the real TFHE worker through the database and decrypt the computed ciphertexts — the one test that crosses from Solana transaction metadata to cleartexts with no deployed stack.                                                                                                                                                                                                                                                                                                                                        | Yellowstone/RPC ingestion, leaf-record delivery, KMS networking, or the complete deployed flow.                                                                                                                                                                        | `#[ignore]`d in the default lanes (needs Docker for the disposable migrated Postgres, the LFS test keys, and anchor-built `zama_host`/`confidential_token` artifacts). Manual only: no CI lane runs it; the solana-e2e scenarios cover the same arc against the deployed stack.                                                        |
| Live scenario vertical (SDK-driven)   | `bun run demo up` from the repo root, then `cd test-suite/fhevm && bun run test:e2e`                                                                                               | Product arcs composed **only** through `@fhevm/sdk` Solana actions and the typed Codama clients, against the running stack: the decrypt vertical through the `encrypted-counter` specimen (write → pure-SDK user decrypt → update → decrypt of the replaced and the current handle), the confidential-transfer arc, the token consume arc (wrap → attested burn → seal → certified public decrypt → redeem → disclose) with its adversarial context-mismatch tail, delegated decrypt with a Squads delegator, and the `dep-chain` load smoke. Every assertion is typed; nothing greps logs. A green run ends by comparing each EncryptedStore's leaf count and peaks with the first coprocessor's Merkle proof service record (`src/solana/merkle-record.ts`, DD-066). | Exhaustive operator semantics (the pure layer owns the full contract; Mollusk and direct real-TFHE supply representative SBF and cryptographic evidence), instruction admission/guards/cost (Mollusk owns those), production reliability, scale, or mainnet readiness. | Docker, Solana tools, Node/Rust toolchains, ports. `bun run demo up` drives `clean-e2e.sh` (image builds/pulls, validator + geyser, typed side-stack deploy from `test-suite/fhevm/src/solana/deploy.ts`). CI's solana-e2e lane runs the suite plus the demo phases; its history: median successful runs ~50–53 min, observed tail 72. |
| Block-manifest quorum on Solana handles | `SOLANA_E2E_SCENARIO=solana-manifest-lifecycle bash scripts/e2e/clean-e2e.sh && cd ../test-suite/fhevm && ./fhevm-cli test manifest-lifecycle-no-drift`; in CI, dispatch `solana-e2e.yml` with `scenario=solana-manifest-lifecycle` and `test-profile=manifest-lifecycle-no-drift` | Three coprocessors at quorum two, each with its own Solana host listener and database, record the `encrypted-counter` fixture's root block under its block height and hash. On every node each fixture handle computes, uploads and matches the others' ct64 digest, and the detectors reach a quorum on Solana block manifests that covers the fixture blocks, with no drift. | Per-node proof serving: the Merkle indexer and proof server run once, against node 0's database. Independent ingestion: all three listeners read one validator through one Yellowstone endpoint. Block height across skipped slots: a single local validator rarely skips one, so the fixture's slot and height (both logged) usually differ by a constant and the host-row check cannot tell them apart; `rows_are_numbered_by_height_across_a_skipped_slot`, which the block ingest row runs, pins the height. The default Solana cadence of 150 and manifests that span several blocks: every detector here publishes at every height. Fault injection, healing and containment on Solana handles. Decryption. | A full stack with three coprocessors; CI runs it on dispatch only, never on pull requests. |

The test-suite and demo dapp reach `@fhevm/sdk` through a symlink into `sdk/js-sdk/src` (their
postinstall swaps bun's `file:` snapshot for one), so a rebuild there is visible to them
immediately; just restart any long-lived test process.

### The cleartext host build

`zama-host` built with `--features cleartext` (`target/deploy/zama_host_cleartext.so`, produced by
`scripts/build-programs.sh <env> zama_host_cleartext`, which `scripts/check-zama-host-idl.sh` runs) is the production host plus the plaintext of every handle it
produces, kept in the accounts it already writes: a section after each `EncryptedStore`'s largest
Borsh encoding, and a tail on the `TransientStore` (`src/cleartext/layout.rs`). `fhe_execute` runs
unchanged, then evaluates the same steps on plaintexts. Verified inputs carry their plaintexts in
the attestation's `extra_data`. Because the values are chain state, every client sees them and
they cannot drift from the accounts, the property forge-fhevm-std gets from its cleartext host
contracts. A value nobody recorded, including anything a production build wrote, fails loudly
(`CleartextError::ValueUnknown`).

The token, batcher, counter, dep-chain and Mollusk operator suites run on this build and read
results with `zama_solana_test_kit::cleartext` (`store_u64`, `handle_value`; fixtures give values
with `fixture_context` and `seed`). `operator_conformance` runs the same evaluator natively.
Cost snapshots and host admission suites stay on the production build.

It is not TFHE: randomness is a keccak of the seed, and a store keeps the plaintexts of its current
slots and of its latest 64 results. Stores are 9.4 KB instead of growing from their minimum size,
so their rent is higher. The build never ships: `build-programs.sh` refuses it in an environment
file and the deployer refuses a binary carrying its marker.

Heavy emphasis on **negative tests**: most cases assert that a malformed account, an extra meta, a
wrong authority, or stale handle metadata is _rejected_. That is the point of the suite, not an
afterthought.

## Mollusk runtime coverage

The `operator_mollusk_conformance`, `host_mollusk`, `fhe_execute_boundary`, `token_mollusk`,
`batcher_mollusk`, `vault_mollusk`, `permit_invalidation_mollusk`, `disclose_packet_fit`,
`host_admin_mollusk`, `user_decryption_delegation_mollusk`, `transient_mollusk`, `preview_cleanup_mollusk`,
`capability_invariants`, and specimen (`counter_mollusk`,
`dep_chain_mollusk`) suites execute real SBF under Mollusk, booted and
asserted through the shared `zama-solana-test-kit` crate. Mollusk surfaces resulting **account state**, **inner instructions (CPIs)**, and **return
data**, which are the stable artifacts these suites assert on. Plain `emit!` program-data logs are
intentionally not part of the runtime-test contract; tests should assert the state transition,
emitted Anchor CPI event, return data, or CPI shape that makes the behavior observable.

## Running the suites

From `solana/`:

```bash
# Verify the production IDL/ABI snapshot, then rebuild the local SBF
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
cargo test -p zama-solana-runtime-tests --test fhe_execute_boundary -- --nocapture
cargo test -p zama-solana-runtime-tests --test token_mollusk -- --nocapture
cargo test -p zama-solana-runtime-tests --test batcher_mollusk -- --nocapture
cargo test -p zama-solana-runtime-tests --test vault_mollusk -- --nocapture
cargo test -p zama-solana-runtime-tests --test permit_invalidation_mollusk -- --nocapture
# Disclosure packet sizing: the largest disclose payload still fits its transport budget.
cargo test -p zama-solana-runtime-tests --test disclose_packet_fit -- --nocapture
# Admin and Store-authority properties over random instruction sequences, then the planted bugs
# they must catch (rebuilds zama-host per patch and restores it).
cargo test -p zama-solana-runtime-tests --test capability_invariants
bash scripts/check-planted-bugs.sh

# The specimen consumers: encrypted-counter is the kit-onboarding proof (~20 lines of
# fixture, ~30 per assertion); dep-chain is the load shape (full-depth dependent chains
# through one fhe_execute, evaluated by the cleartext host build).
cargo test -p zama-solana-runtime-tests --test counter_mollusk
cargo test -p zama-solana-runtime-tests --test dep_chain_mollusk

cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
```

#### Renaming anything shared: the three other roots that can see it

`cargo test --workspace` here covers exactly one Cargo workspace root. The
repository has more than a dozen, so the number is not the useful fact — what
matters is which ones can see a Solana change, and the criterion is a path
dependency on a crate under `solana/`. Three do: `coprocessor/fhevm-engine`
(host-listener, tfhe-worker), `kms-connector`, and `relayer` (the delegation
pre-check reads records through `zama-solana-acl`). Everything else in the repo — `shared/*`,
`test-suite/gateway-stress`, the generated `*_bindings` — depends on no Solana
crate and cannot break from one.

Do not enumerate these roots by grepping for a `[workspace]` stanza; that misses
implicit roots (`relayer/Cargo.toml` has no stanza and no parent claims it, so
cargo treats it as its own). Ask cargo instead:
`cargo metadata --no-deps --format-version 1 | jq -r .workspace_root`.

A rename that reaches a shared crate compiles cleanly here and still breaks the
build in those three, because they are invisible to this workspace:

```bash
# Path-depend on zama-host / confidential-token / zama-solana-acl. `--all-targets`
# matters: the sites that break are usually `#[cfg(test)]`, so a plain
# `cargo check` or `cargo build` passes while `cargo test -p …` does not compile.
(cd ../coprocessor/fhevm-engine && SQLX_OFFLINE=true cargo check --workspace --all-targets)
(cd ../kms-connector && SQLX_OFFLINE=true cargo check --workspace --all-targets)
(cd ../relayer && cargo check --workspace --all-targets --all-features)
```

Each of those roots hid a real break at least once. The grep sweeps in
`scripts/dead-surface-check.sh` cover some of the same trees, but grep does not
typecheck — a root can be swept and still never compiled.

## Scenario layer (SDK-driven e2e)

Lives in `test-suite/fhevm/e2e/` — a small harness plus scenario files. This layer **is** the live
vertical (fhevm-internal#1876): every live assertion is a typed `bun:test` expectation. The layer owns only what composition can break (proofs vs live state, KMS
round-trips, relayer seams, timing) — never what the Mollusk ladder already proves.

The scenarios run under `bun:test` because they share their runtime with the fhevm-cli demo
lifecycle and the `src/solana/*` orchestrators, which are bun-native (`Bun.spawn`, `Bun.sleep`,
`import.meta.dir`).

Every local validator the test suite starts, for the full stack, the cleartext stack and the
deployer test, runs Agave 4.3.0 with Alpenglow consensus active from genesis (`--alpenglow` in
`validatorStartArgs`, `src/solana/validator.ts`), as Solana devnet does. A block is final as soon
as it completes, so `confirmed` and `finalized` name the same slot. All three check the Alpenglow
feature account after startup and stop if it is not active.

Two rules the layer holds itself to:

1. **Each behavior is tested at exactly one layer.** Mollusk owns instruction admission, guards,
   arithmetic and cost; scenarios never re-test that territory.
2. **Scenarios reach the protocol through `@fhevm/sdk` Solana actions and generated Codama
   clients.** Token instructions come from `@fhevm/confidential-token`. Specimen programs
   (encrypted-counter, dep-chain) still live at `test-suite/fhevm/src/solana/internal/generated`.
   Never hand-rolled instruction bytes. A missing SDK read/action is an SDK gap to file.

The harness (`e2e/harness/`):

- `loadEnv()` → a `TestEnv` (RPC/WS/relayer/gateway URLs, the DD-052 chain id, the zama-host
  program id, the user-decrypt context, the coprocessor DB container, the deployer
  keypair root, and capability flags `faucet` / `freshMints` / `fastSlots` / `protocolServices`).
  Its source is the lifecycle-owned stack by default (env-var overridable), or `devnet`,
  `cleartext` or a seeded demo-config.
- `personas` → named actors backed by on-disk keypairs, with a capability-gated `fund()` (local
  airdrop).
- `until(condition, { timeoutMs, intervalMs })` → a generic readiness-polling helper.
- `harness/solana/stack.ts` → the running stack as an object: container/URL readiness. It owns
  readiness, not lifecycle — `bun run demo up`/`down` start and stop the stack.
- `harness/solana/vertical.ts` → `verticalSetup()`: a fresh provisioning context, funded wallet,
  and host-config read per call — one wallet per scenario keeps them fully isolated.
- `harness/solana/sdkEncrypt.ts` → the SDK encrypt+input-proof seam shared by every scenario that
  submits an encrypted input.

Scenarios (`e2e/scenarios/`), each with its retired-assertion mapping in the file header where it
replaced a bash phase:

- `fhe-vertical` — the `encrypted-counter` specimen: initialize → increment → pure-SDK user
  decrypt of the counter, then a second increment and the decrypt of both the replaced handle and
  the current one (no client proof: the connector fetches the allow leaf from the coprocessors).
- `delegated-user-decrypt` — a delegator's counter decrypted by a delegate under a delegation
  record, including a Squads multisig delegator whose counter is written through proposals.
- `confidential-transfer` — encrypt input → `submitInputProof` → `confidentialTransfer` → user
  decrypt of both rotated balances.
- `token-vertical` — the consume arc: wrap → attested burn → seal → certified public decrypt →
  redeem (SPL balance-delta asserted) → disclose, plus the adversarial context-mismatch tail
  pinned to `InvalidKmsContext`. That tail is the L4-b attack. Its sibling **L4-a** (a forged
  KMS signature over a well-formed certificate) is exercised in the **kms repo's live harness**,
  not here: it needs a KMS that will sign attacker-chosen material, which this stack's KMS will
  not do. Cross-repo coverage with no pointer is how coverage quietly stops running, so if you
  are auditing the adversarial surface, look there for L4-a rather than concluding it is absent.
- `load-smoke` — the `dep-chain` specimen live: one 32-step strictly dependent execution with a
  counter increment alongside.
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

### The cleartext target

`SOLANA_E2E_SOURCE=cleartext` runs the same scenarios against the cleartext stack
(`test-suite/fhevm/src/solana/cleartext-stack.ts`) instead of the Zama localnet. That stack is a
`solana-test-validator` on its own ports, loaded at genesis with the
[cleartext host build](#the-cleartext-host-build) and the e2e programs, and bootstrapped with test
coprocessor and KMS keys. No relayer, gateway, coprocessor or KMS runs. The whole suite takes
about 20 seconds after the build, and needs no Docker.

```bash
cd test-suite/fhevm
SOLANA_E2E_SOURCE=cleartext bun run test:e2e   # builds, starts the stack, runs, stops it

# Development loop: keep a stack up, then run scenarios against it.
bun run src/solana/cleartext-stack.ts
SOLANA_E2E_SOURCE=cleartext bun test e2e/scenarios/fhe-vertical.scenario.test.ts
```

Scenarios switch nothing but their SDK clients. `loadSolanaSdk()` (`src/solana/target.ts`)
returns `@fhevm/sdk/solana` with the three client factories replaced by those of
`@fhevm/sdk/solana/cleartext`. These take the same parameters and return the same actions:

- **Encrypt.** A mock input proof. The client signs its attestation with the test coprocessor
  key, and the attestation's `extra_data` is the plaintexts, one big-endian value per handle, at
  least two bytes each so that production's one-byte `0x00` never decodes as a value.
- **User decrypt.** The permit and request are built as in production, except the transport key:
  no share is signcrypted to it, so the permit commits to random bytes of its length
  (`PERMIT_TRANSPORT_KEY_LEN`) and no KMS WASM loads, as in EVM's cleartext decrypt module.
  Each attempt then runs the relayer's submission checks and delegation pre-check, the gateway's
  validity window, and the KMS Connector's authorization, in the real stack's order and with its
  labels (the header of
  `sdk/js-sdk/src/solana/cleartext/decrypt.ts` lists them). A failure the Connector would retry
  leaves the attempt unanswered for the retry loop; any other throws at once and names the failure,
  where the real stack leaves the request to time out. Otherwise the answer is the plaintext the
  host recorded in the store. The Connector part (`cleartext/authorization.ts`) is held to the
  Connector itself: the kms-worker test `solana_authorization_cases` runs the Connector on a set of
  user and public decryptions and writes each one's accounts, Merkle proof batch with the record's
  answers, and verdict to `solana/test-fixtures/authorization/decrypt_cases_v1.json`, and the SDK
  test requires the client to match. The relayer and gateway part is copied from their code
  (`relayer/src/host/solana_delegation_precheck.rs`, the relayer's user-decrypt admission,
  `Decryption.sol`) and nothing generated pins it, so a change there needs a matching change here.
- **Public decrypt.** The store and the handle's public leaf are judged by the Connector's rules,
  and a failure the Connector would retry is judged again, up to 20 times. The certificate is
  signed with the test KMS key, and the on-chain verifier checks it as usual.

Leaf proofs come from an in-memory leaf record of the validator (`createSolanaLeafRecord`, whose
header gives its catch-up rules). The e2e's decrypt clients share one record for the test process
(`readMerkleProofs` in `test-suite/fhevm/src/solana/target.ts`), as coprocessors keep theirs, so a
decrypt reads only the store writes since the last one.

What the cleartext target does not prove, so these parts skip there
(`capabilities.protocolServices` is false):

- Relayer behavior: job coalescing (the coalescing step of `delegated-user-decrypt`), and which
  refusals come back unanswered rather than labeled. The client applies the pre-check's rules; the
  relayer's own code does not run.
- The KMS: shares, signcryption and response signatures, and so the FHE parameter and the KMS
  epoch. Threshold topologies, and ciphertext materialization: `waitForSnsCommit` resolves at once.
- Host upgrade with a listener restart (the third `fhe-vertical` test), and the Squads arc of
  `delegated-user-decrypt`.
- The demo vault flows (`deposit-arc`, `bun run demo`): the demo-dapp builds its own SDK clients.

The input `extra_data` of the cleartext build is up to 256 bytes per attestation, where production's
is the one byte `0x00`. An execution that only just fits the production heap, or a transaction that
only just fits the 1232-byte packet, can fail on the cleartext build. Mollusk does not check
transaction size, so only the validator stack catches the second. The gap only causes false
failures: a transaction that fits on the cleartext build always fits in production. v1 transactions
(SIMD-0385) raise the limit to 4096 bytes for both builds, so they move this wall rather than remove
it.

### Extending the cleartext target

- **A new e2e scenario** runs on both targets when it creates its SDK clients from
  `await loadSolanaSdk()` (`src/solana/target.ts`), as the existing scenarios do. Its other
  imports from `@fhevm/sdk/solana` do not depend on the target. A part that needs real relayer or
  KMS behavior skips with `test.skipIf(!loadEnv().capabilities.protocolServices)`, and the list
  above names what it then leaves untested. `solana-tests/cleartext-e2e` runs the suite on every
  change under `solana/`, `sdk/js-sdk/` or `test-suite/fhevm/`.
- **A new Mollusk test** that asserts plaintexts loads the cleartext host with
  `zama_solana_test_kit::cleartext::host_svm()` and reads results with `store_u64`,
  `store_value` or `handle_value`. Fixture stores get their values with `fixture_context` and
  `seed`, which record 0 behind every slot a test does not seed.
- **A new `fhe_execute` step or operator** does not compile on the cleartext build until
  `evaluate_steps` (`programs/zama-host/src/cleartext/mod.rs`) gives it plaintext semantics: the
  match over steps has no catch-all. `operator_conformance` then checks it against the EVM cases
  in `library-solidity/codegen/overloads/e2e.json`, and panics on an operator it does not map.
- **A change to the store or transient layout, the input value widths or the input-attestation
  type strings** fails `the_sdk_cleartext_constants_match_the_host` until the SDK's copy is
  rewritten: `ZAMA_UPDATE_SDK_CONSTANTS=1 cargo test -p zama-host --features cleartext --lib
  sdk_constants`, then commit `sdk/js-sdk/src/solana/internal/hostConstants.ts`.
- **A change to the KMS Connector's Solana authorization** fails
  `the_committed_cases_are_the_connectors_verdicts` until the cases are rewritten:
  `ZAMA_UPDATE_AUTHORIZATION_CASES=1 cargo test -p kms-worker --test solana_authorization_cases`
  from `kms-connector/`. The SDK's `authorization.test.ts` then fails until
  `cleartext/authorization.ts` reaches the same verdicts. Add a case there for a new rule.
- **A change to the relayer's admission or delegation pre-check, or to the gateway's validity
  window,** has no generated check. Mirror it by hand in `cleartext/decrypt.ts` and its test.

## Where the two decrypt leaves are tested

Every decrypt authorizes through exactly one Merkle proof against the Store's finalized peaks.

| Leaf                               | Check                                                                         | Where tested                                                                                                                                                                                                       |
| ---------------------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **allow** (`HistoricalAccessLeaf`) | MMR proof against live peaks, for the current handle and a replaced one alike | `zama-solana-acl` unit tests (`authorize_state_historical`, MMR append and verify); `host_mollusk` write-then-prove; `kms-worker` `solana_` tests (`handle_binding`, `proof`); solana-merkle-proof-service `store_tests` |
| **public** (`PublicDecryptLeaf`)   | MMR proof against live peaks, exact handle                                    | `zama-solana-acl` (`authorize_state_public`); `kms-worker` `solana_public_decrypt` tests. On chain, consumers check only the KMS certificate (DD-065)                                                              |

Each has negative coverage: wrong key, wrong handle, a proof from a foreign Store, an invalid or
forged proof, and a leaf record that is behind all fail closed (the `zama-solana-acl` tests, the
kms-worker `solana_` tests and the connector's `ProofRecordBehind` and `NoLeaf` classification).

The central correctness bet is that off-chain consumers reproduce on-chain MMR state exactly. The
solana-e2e scenarios exercise host-listener reconstruction and the Merkle indexer's leaf record
against the full stack, and every proof the connector fetches is verified against the peaks it
read on chain. A divergence fails closed rather than yielding a wrong proof: a record that has
sealed at least as much history as the chain shows and holds no such leaf is a terminal refusal,
and a record that is behind is a retry.

## Deferred

- **Explicit Mollusk CU-trace assertions.** CU fit is implicit today: Mollusk enforces the budget, so
  a passing test fits. An explicit `compute_units_consumed` assertion per hot instruction would turn
  that into a measured, regression-guarded number.
- **litesvm gate** (zama-ai/fhevm#3045): a lighter in-process runtime alongside Mollusk, blocked on
  the two dependency walls recorded in `solana/runtime-tests/Cargo.toml`.

## Traps & gotchas (read before you lose an afternoon)

- **Stale or wrong-feature SBF artifacts.** After changing an Anchor program, **rebuild** before
  running runtime tests — a stale `.so` will make tests pass or fail against old code. Prefer
  `bash scripts/check-zama-host-idl.sh`: it checks the default production IDL/ABI surface and
  rebuilds every program on the default feature set, plus the `admin-sweep` builds and the
  [cleartext host build](#the-cleartext-host-build). No program has a test-only entropy path. The
  host suites and cost snapshots run the default build that ships; the token, batcher, counter,
  dep-chain and Mollusk operator suites run the cleartext host build, which adds plaintext tracking
  to the same instructions. `capability_invariants`, `fhe_execute_boundary` and `transient_mollusk`
  run both builds as parity checks.
- **A small CU delta after an incremental SBF build is not a code change.** The committed
  baselines are minted by `scripts/update-cost-snapshots.sh`, which runs `cargo clean` first. An
  incremental rebuild of the same source can differ by a few CU: a doc-comment-only edit to
  `zama-host` was measured at −12 CU on the batcher's `open_batch` and `redeem_open_batch` after
  `sync-zama-host-idl.sh` (incremental), and byte-identical to the baselines after the snapshot
  script's clean rebuild. So regenerate with the script before believing a delta of this size, and
  do not attribute it to the edit in front of you.
- **Cost snapshots are minted on x86_64 Linux.** CI measures there. Platform tools v1.57 build
  slightly different code on macOS: a clean build of the same commit measured 10 to 91 CU more on
  most profiles (v1.52 matched). On macOS, the script still shows
  the delta between two commits. Commit the `solana-cost-snapshots` artifact that a failing
  `solana-tests` run uploads.
- **SPL Token CPIs in token tests.** `token_mollusk` executes real SPL Token CPIs through the
  matching `mollusk-svm-programs-token` program fixture.
- **`anchor build` vs program ids.** `anchor build` checks that each program's declared id matches
  its `target/deploy/*-keypair.json`. The deployed programs' keypairs are not in the repository (one
  id on every cluster, DD-053), so a plain `anchor build` reports "Program ID mismatch" against a
  stale generated keypair. Always build with `anchor build --ignore-keys` (what
  `scripts/build-programs.sh` and the IDL sync do); never `anchor keys sync`, which would rewrite
  the shipped ids. The BPF compile itself is unaffected.
- **Keep cargo verification mostly sequential.** The workspace and the BPF build share target dirs;
  running several cargo invocations at once causes build-lock waits, not speedups.
- **Connector/coprocessor need `SQLX_OFFLINE=true`.** They have SQLx-checked queries; without the
  env var and a live DB they won't compile.
- **The host-listener record types are generated; one event is decoded.** Ingestion reconstructs
  semantic compute facts from instruction data and takes each execution's result handles from its
  `FheExecutedEvent`, decoded with `zama-host`'s own type. If a generated record type changes,
  regenerate the vendored IDL and validate reconstruction explicitly.
- **The connector and the Merkle proof service compile the ACL crate; the IDL and the TypeScript
  seeds are mirrors.** Account layout, PDA seeds and leaf commitments come from `zama-solana-acl`, the same
  crate `zama-host` compiles, so a layout change breaks the build. The vendored IDLs are build
  output: after a host instruction shape changes, `sync-zama-host-idl.sh` rewrites them and
  `npm run codegen:solana` the Codama clients; CI's `check-zama-host-idl.sh` and
  `codegen:solana:check` fail on a stale copy. The SDK's zama-host seeds (`encrypted-state`,
  `user-decryption-delegation`, `permit-invalidation`, `transient`) are TypeScript literals written
  by hand, and nothing checks them statically; a host seed change must be mirrored there. The
  confidential-token seeds come from the generated Codama client, and `check-pda-seeds.py` checks
  only the Rust side of the token's `PENDING_BURN_SEED`.
  The user-decrypt side of the mirror is pinned by committed vectors that both sides assert
  against. The permit's canonical text and offchain-message envelope come from `zama-solana-permit`
  (used by the connector and the relayer) and the SDK's TypeScript, pinned by
  `solana/test-fixtures/permit/permit_v1.json`. The relayer envelope is mirrored by the relayer's
  wire types (`UserDecryptV3RequestJson`) and the SDK, pinned by
  `solana/test-fixtures/user-decrypt/relayer_envelope_v1.json`. Moving those bytes is a protocol
  change (new domain tag / version byte), not a fixture refresh.
