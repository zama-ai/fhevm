# `kms-context-qa-tests` — stage 1: the host half

Implementation report for the first stage of the `kms-context-qa-tests` profile.

**Status:** delivered and green against a live stack.
**Scope of this stage:** host-side only. No changes inside the e2e test container.

---

## 1. Why this profile exists

Three QA documents in `test-suite/e2e/test/` (`qa-scenarios-dup.md`, `qa-kms-tests.current.md`,
`qa-tests.compare.md`) compare 18 proposed QA scenarios for the KMS context/epoch lifecycle against
the coverage that actually exists in the repository. Several scenarios are uncovered or only
partially covered.

Rather than extending `src/commands/kms-context-switch.ts` — which gates CI — we stood up a
separate profile and implement **one scenario at a time**, fixing shortcomings as they surface.
`kms-context-switch` is not modified by this work.

## 2. The scenario being implemented

The amended, self-describing form of *"The SDK uses a new epoch in the same context"*. The original
Gherkin asserts literal ids (`C1`, `E1`, `E2`) that only the orchestrator knows; the amended form has
the test read them from `ProtocolConfig`, so each half stands on its own:

```gherkin
Feature: Normal decryption after a new epoch becomes active

  Scenario: The SDK uses the currently active epoch
    Given ProtocolConfig reports an active pair "(C, E)"
    And the NewKmsEpoch event for "E" names a previous epoch "E_prev" distinct from "E"
    When the application performs a decryption through the SDK
    And the test captures the request and response extraData
    Then the decryption must complete successfully
    And the request extraData must decode as version "0x02", context "C", and epoch "E"
    And the response extraData must be identical to the request extraData
    And the decoded epoch must not equal "E_prev"
```

### Clause-by-clause coverage after this stage

| Clause | Runs | Status |
|---|---|---|
| `Given ProtocolConfig reports an active pair "(C, E)"` | host | done |
| `And the NewKmsEpoch event … names a previous epoch "E_prev"` | host | done |
| `When the application performs a decryption through the SDK` | container | partial — runs, but through the blind probe |
| `And the test captures the request and response extraData` | container | **not covered** |
| `Then the decryption must complete successfully` | container | done |
| `And the request extraData must decode as version "0x02", context "C", epoch "E"` | container | **not covered** |
| `And the response extraData must be identical to the request extraData` | container | **not covered** |
| `And the decoded epoch must not equal "E_prev"` | container | **not covered** |

This stage proves the rotation genuinely happened and that the new pair **serves** real traffic. It
does **not** prove the SDK embedded the new epoch in `extraData` — the heart of the scenario.

## 3. Why the split, and why the host half comes first

| Half | Owns | Fails when |
|---|---|---|
| Host (this stage) | read baseline, trigger rotation, **wait for activation**, capture `E_prev` | the KMS never reshares/confirms — a KMS or connector problem |
| Container (next) | SDK decrypt, decode `extraData` v2, compare request vs response | the SDK ignored the rotation — e.g. its 15-minute cache |

Activation is not automatic: `defineNewEpochForCurrentKmsContext` only opens a *Pending* epoch. The
cores must reshare and every committee connector must submit `confirmEpochActivation` before
`getCurrentKmsContextAndEpoch` advances.

**Without the host half's wait, a container-side `extraData` assertion would run against the stale
pair and pass while verifying nothing.** The host half exists to make the container half
trustworthy. Conversely, without the container half we only know the chain advanced, not that the
client followed.

## 4. What was built

~1780 lines across nine new files. Nothing existing was modified except four additive lines in
`src/commands/test.ts`.

```
src/kms-qa/
  evidence.ts                     258   EvidenceRecorder: timed, structured, per-case audit trail
  protocol-config.ts              337   views, lifecycle txs, event decoding, waits, invariants
  nodes.ts                        191   NodeSupervisor: scoped stop/start with guaranteed restore
  registry.ts                     144   QaCase catalogue, requirement checks, selection
  cases/case-epoch-rotation.ts    197   the scenario's host half
  evidence.test.ts                150
  protocol-config.test.ts         157
  registry.test.ts                112
src/commands/kms-context-qa-tests.ts  234   preflight, wiring, sequencing, summary — no case logic
```

### Design decisions

**One file per case.** `cases/` is what grows. `kms-context-switch.ts` is already 729 lines for five
steps; a single file holding a dozen cases would be unreadable. Each case file's header is that
scenario's specification.

**Registry separate from the runner.** Adding a scenario is one new file plus one entry in
`QA_CASES`. The runner — preflight, sequencing, evidence, restore — is never edited, so a new case
cannot regress the guarantees every other case depends on.

**Evidence as a first-class output.** A QA profile's job is a defensible account of what the
protocol did, not a pass/fail. `EvidenceRecorder.step()` times every action, records identifying
fields, and prints one greppable line — including on failure. A full table is printed at the end of
every run, passing or not. This paid for itself immediately (see §7).

**Node control before it was needed.** `nodes.ts` is not exercised by this case. It exists because
upcoming cases withhold quorums by stopping components, and retrofitting restore discipline after
the fact is how containers get left down. It centralises the `try`/`finally` idiom currently
copy-pasted at each call site in `kms-context-switch.ts` and `kms-generation.ts`, and tracks every
stop so the runner's outermost `finally` can force a restore.

**Transaction evidence without touching shared code.** `castSend` (`src/kms-onchain.ts:111`) parses
the full `cast send --json` receipt but types it as `{status, logs}`. `transactionHash`,
`blockNumber` and `gasUsed` are present at runtime but invisible to TypeScript. They are read
through a locally declared widened structural type rather than by changing the shared type.

### Registration

Four additive lines in `src/commands/test.ts`: the import, the `TEST_PROFILE_NAMES` entry, the
`TEST_PROFILE_DESCRIPTIONS` entry, and a `runProfile` dispatch branch placed before the `TEST_GREP`
fallback. No suite membership in `src/layout.ts` — like every other `kms-*` profile this one is
on-demand, because it is disruptive.

## 5. What the case does

1. Read the baseline active pair `(C, E_prev)`.
2. Resolve the **live committee** from the chain (see §7 — this is not `1..committeeSize`).
3. Broadcast `defineNewEpochForCurrentKmsContext` with `cast send`, so the `NewKmsEpoch` receipt is
   available directly. (`kms-context-switch` rotates via the `host-sc-epoch-rotation` compose task,
   which returns no receipt.)
4. Decode the event: `kmsContextId` and `epochId` from topics 1 and 2; `previousContextId`,
   `previousEpochId` and `materialBlockNumber` from non-indexed data words 0, 1 and 2.
5. Cross-check the two independent sources of `E_prev` — the pre-broadcast read and the contract's
   own `previousEpochId` — and fail loudly on disagreement, which would mean a concurrent lifecycle
   operation made the baseline ambiguous.
6. Assert the sequential-id invariant (`epochId == baseline + 1`, from `++epochCounter`).
7. Wait for the **exact** target epoch to activate, not merely a greater one.
8. Require `new_kms_epoch.status = completed` on every live committee connector, so pointer movement
   alone cannot pass the case.
9. Run the input-proof smoke and the user-decryption probe under the rotated pair.
10. Emit the handoff values `(C, E, E_prev)` for the container-side stage.

## 6. How to run

```bash
cd test-suite/fhevm

# unit tests — no stack, ~4s
bun run check          # tsc --noEmit
bun test src           # 452 pass (53 new)

# end to end — live stack, ~5 min
./fhevm-cli down
./fhevm-cli up --target latest-main --scenario five-party-swap-threshold-kms
./fhevm-cli test kms-context-qa-tests
```

Options:

| Variable | Effect |
|---|---|
| `KMS_QA_CASES=<id,...>` | run a subset; unset or `all` runs everything. Execution always follows registry order, never the order supplied |
| `KMS_QA_ALLOW_ANY_SCENARIO=1` | relax the `five-party-swap-threshold-kms` pin; per-case requirements are still enforced |

`--grep` is not accepted — orchestrated profiles reject it by design (`validateNamedProfileGrep`).

**Disruptive and single-run.** The case advances the epoch and chain state is not rolled back.
Re-running without a re-up works (the case reads current state rather than assuming a pristine
chain), but each run advances the epoch again.

## 7. Verification, and the bug the live run caught

Static: `tsc --noEmit` clean; `bun test src` → 452 pass, 0 fail (53 new); profile listed in
`fhevm-cli test list`; `--grep` correctly rejected; unknown `KMS_QA_CASES` id fails preflight
listing the available ids.

**The first live run failed, correctly.** The per-party reshare assertion targeted
`1..committeeSize` = `{1,2,3,4}`, but the stack under test had already been node-swapped by an
earlier `kms-context-switch` run, so the live committee was `{1,2,3,5}`. Party 4 had been dropped,
no longer held the context's material, and its core reported a failed reshare — correct behaviour
that the assertion blamed. It timed out after 241s with:

```
db "kms-connector-4" returned "failed" (expected one of completed)
```

**Fix:** `readCommitteeParties()` now derives membership from the chain —
`getKmsSignersForContext(activeContextId)` intersected with `state.discovery.kmsSigners` (signer
address per party index) — and records which parties are outside the committee. The check went from
a 241s timeout to 859ms.

Two observations worth keeping:

- The structured evidence made the diagnosis immediate: the failure line carried the exact SQL, the
  party and the observed value, and the context/epoch ids in the trail revealed the stack had been
  swapped.
- Running against an already-dirty stack was luckier than a pristine one. A clean stack would have
  passed and shipped the latent bug, which would then have failed the first time anyone ran this
  after a context switch.

Final run:

```
[kms-context-qa] PASS (302s) — 1 case(s):
  - epoch-rotation: a same-context epoch rotation activates on chain, every committee node
    completes the reshare, and the rotated (context, epoch) pair serves both an input-proof
    flow and a user decryption
```

14 evidence steps. Activation took 65.6s over 14 polls; the committee check 859ms; the decryption
probe 118.3s.

## 8. Next stage — the container half

A Mocha spec in `test-suite/e2e/test/` that:

1. reads `getCurrentKmsContextAndEpoch` itself — the container already has
   `PROTOCOL_CONFIG_CONTRACT_ADDRESS` and `RPC_URL` in `.fhevm/runtime/env/test-suite.env`;
2. decrypts through the SDK and decodes `extraData` with `createKmsExtraDataFromBytesHex`
   (`sdk/js-sdk/src/core/kms/kmsExtraData-p.ts`);
3. asserts version `0x02`, the embedded context and epoch, the request/response echo, and that the
   superseded epoch is not used.

Plus a `TEST_GREP` entry (`src/layout.ts`), a runner injected by `test.ts` alongside
`runDecryption`/`runSmoke`, and a rebuild of the test-suite image (`--override test-suite`) since
the spec ships inside it.

### Two known risks for that stage

- **The SDK caches the active pair for 15 minutes**
  (`sdk/js-sdk/src/core/kms/getCurrentKmsContextAndEpoch-p.ts`, keyed by runtime + address). The
  client must be constructed fresh or use `forceRefresh`, or the test measures the cache instead of
  the protocol — precisely the bug the scenario exists to catch.
- **The request-side `extraData` may not be exposed.** The response carries it (see
  `test-suite/e2e/test/sdk/connector/verify.ts`), but the request side is internal to `userDecrypt`.
  It may need recomputation via `createKmsExtraDataV2`, or a small addition to the SDK's public
  surface. This should be confirmed before writing the spec.

## 9. Related files

| Path | Role |
|---|---|
| `test-suite/fhevm/changelog.md` | terse feature list and known gaps for this work |
| `test-suite/e2e/test/qa-scenarios-dup.md` | the 18 proposed QA scenarios |
| `test-suite/e2e/test/qa-kms-tests.current.md` | Gherkin for the tests that already exist |
| `test-suite/e2e/test/qa-tests.compare.md` | gap analysis between the two |
| `test-suite/fhevm/src/commands/kms-context-switch.ts` | the existing lifecycle profile — deliberately untouched |
| `host-contracts/contracts/ProtocolConfig.sol` | the governance contract this profile drives |
