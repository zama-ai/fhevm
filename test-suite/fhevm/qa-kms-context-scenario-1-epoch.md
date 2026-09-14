# QA KMS context — scenario 1: the SDK uses a new epoch in the same context

Implementation report for the first scenario of the `kms-context-qa-tests` profile, implemented as
the `epoch-rotation` case.

**Status:** delivered and green against a live stack.
**Scope:** both halves — the host orchestration and the container-side `extraData` spec. One clause
remains deliberately uncovered (see §2 and §8).

> Scenarios are implemented one at a time, each with its own report named
> `qa-kms-context-scenario-<n>-<topic>.md`. This is scenario 1.

---

## 1. Why this profile exists

Three QA documents in `test-suite/e2e/test/` (`qa-scenarios-dup.md`, `qa-kms-tests.current.md`,
`qa-tests.compare.md`) compare 18 proposed QA scenarios for the KMS context/epoch lifecycle against
the coverage that actually exists in the repository. Several scenarios are uncovered or only
partially covered.

Rather than extending `src/commands/kms-context-switch.ts` — which gates CI — we stood up a
separate profile and implement **one scenario at a time**, fixing shortcomings as they surface.
`kms-context-switch` is not modified by this work.

## 2. The scenario: as proposed, and as implemented

### 2.1 As originally proposed

```gherkin
Scenario: The SDK uses a new epoch in the same context
  Given the previous active pair is "C1, E1"
  And an epoch rotation to "C1, E2" has completed
  And ProtocolConfig returns "C1, E2" as the active pair
  When the application performs a decryption through the SDK
  And the test captures the request and response extraData
  Then the decryption must complete successfully
  And the request extraData must decode as version "0x02", context "C1", and epoch "E2"
  And the response extraData must be identical to the request extraData
  And epoch "E1" must not be used by the new request
```

### 2.2 As implemented

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

### 2.3 Why it changed

The intent is unchanged: prove the SDK follows an epoch rotation. Three things were reworded so the
scenario could actually be executed, and none of them weakens it.

**The literal ids had no source.** `C1`, `E1` and `E2` are placeholders only the orchestrator can
resolve — they are large domain-tagged uint256 values allocated by the contract at rotation time.
Written literally, the container-side test would need them injected, which couples it to the
orchestrator and makes it unrunnable on its own. The implemented form has each half read the pair
from `ProtocolConfig`, so both stand alone. The injected values are still used, but as a
*cross-check* (§4), not as the only source of truth.

**`Given … has completed` is a precondition someone must establish and verify.** Activation is not
automatic: `defineNewEpochForCurrentKmsContext` merely opens a Pending epoch, and the pair only
advances once the cores reshare and every committee connector confirms. Left as a bare `Given`, the
clause invites a test that assumes the rotation happened. The implemented form makes that
responsibility explicit and assigns it to the host half, which performs the rotation and **waits**
for the activation before anything downstream runs. Without that wait the assertion would run
against the stale pair and pass while verifying nothing.

**`E1` became `E_prev`, read from the chain.** Rather than a constant the test carries, the
superseded epoch is taken from the `NewKmsEpoch` event's `previousEpochId` — the contract's own
statement of what was replaced — and cross-checked against the pair read before the broadcast. A
disagreement between those two sources means a concurrent lifecycle operation, and the case fails
loudly instead of asserting against an ambiguous baseline.

One clause could not be implemented at all, for reasons outside this scenario's control: *"the
response extraData must be identical to the request extraData"*. See §8.

### Clause-by-clause coverage after this scenario

| Clause | Runs | Status |
|---|---|---|
| `Given ProtocolConfig reports an active pair "(C, E)"` | host | done |
| `And the NewKmsEpoch event … names a previous epoch "E_prev"` | host | done |
| `When the application performs a decryption through the SDK` | container | done |
| `And the test captures the request … extraData` | container | done |
| `Then the decryption must complete successfully` | container | done |
| `And the request extraData must decode as version "0x02", context "C", epoch "E"` | container | done |
| `And the decoded epoch must not equal "E_prev"` | container | done (implied: the epoch must equal the **active** one, and `E_prev != E`) |
| `And the response extraData must be identical to the request extraData` | container | **not covered — not possible through the SDK** |

Seven of the eight clauses are covered. The response-echo clause is the exception: the SDK neither
verifies nor exposes the response `extraData`. Evidence and the decision to defer are in
`test-suite/fhevm/qa-extradata-check.md`.

## 3. Why the split, and why the host half comes first

| Half | Owns | Fails when |
|---|---|---|
| Host | read baseline, trigger rotation, **wait for activation**, capture `E_prev` | the KMS never reshares/confirms — a KMS or connector problem |
| Container | sign a permit through the SDK, decode its `extraData` v2, decrypt | the SDK ignored the rotation — e.g. its 15-minute cache |

Activation is not automatic: `defineNewEpochForCurrentKmsContext` only opens a *Pending* epoch. The
cores must reshare and every committee connector must submit `confirmEpochActivation` before
`getCurrentKmsContextAndEpoch` advances.

**Without the host half's wait, a container-side `extraData` assertion would run against the stale
pair and pass while verifying nothing.** The host half exists to make the container half
trustworthy. Conversely, without the container half we only know the chain advanced, not that the
client followed.

## 4. What was built

~2000 lines across ten new files. Nothing existing was modified beyond additive wiring: five lines
in `src/commands/test.ts` (import, profile name, description, dispatch, the container runner) and
one `TEST_GREP` entry in `src/layout.ts`.

**Host** (`test-suite/fhevm`):

```
src/kms-qa/
  evidence.ts                     258   EvidenceRecorder: timed, structured, per-case audit trail
  protocol-config.ts              337   views, lifecycle txs, event decoding, waits, invariants
  nodes.ts                        191   NodeSupervisor: scoped stop/start with guaranteed restore
  registry.ts                     158   QaCase catalogue, requirement checks, selection
  cases/case-epoch-rotation.ts    214   the scenario, host side
  evidence.test.ts                150
  protocol-config.test.ts         157
  registry.test.ts                112
src/commands/kms-context-qa-tests.ts  236   preflight, wiring, sequencing, summary — no case logic
```

**Container** (`test-suite/e2e`):

```
test/kmsContextExtraData/kmsContextExtraData.ts   182   the scenario, client side
```

The container spec, in detail:

- builds a **fresh** SDK client — `createInstance()` mints a new runtime uid, and the SDK's context
  cache is keyed on it, so the cache starts cold. A reused client could still serve the
  pre-rotation pair and make the assertion measure the cache instead of the protocol;
- signs a unified decryption permit and decodes `permit.eip712.message.extraData`, asserting
  version `0x02`, the active context and the active epoch, plus a byte-exact string match so a
  future change to word order or padding cannot slip past the field checks;
- reads the active pair itself via ethers, so its source of truth never passes through the SDK
  cache and a stale client surfaces as a mismatch rather than being hidden;
- cross-checks that reading against `KMS_QA_EXPECTED_CONTEXT_ID` / `KMS_QA_EXPECTED_EPOCH_ID`,
  injected by the profile with the pair the orchestrator observed, catching a rotation that landed
  between the two reads. Skipped when the variables are absent, so the spec also runs standalone;
- performs a real decryption with that permit, closing *"the decryption must complete
  successfully"*.

Note `createKmsExtraDataFromBytesHex` is internal to the SDK (`kmsExtraData-p.ts`, not re-exported
by any subpath), so the spec decodes the fixed layout itself by slicing.

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
10. Run the container spec, injecting `(C, E)` so it can also assert the chain did not move between
    the orchestrator's read and its own.

## 6. How to run

Both halves run from a single command — the profile drives the container itself:

```bash
cd test-suite/fhevm

# unit tests — no stack, ~4s
bun run check          # tsc --noEmit
bun test src           # 452 pass (53 new)

# end to end — live stack, ~6 min
./fhevm-cli up --target latest-main --scenario five-party-swap-threshold-kms
./fhevm-cli test kms-context-qa-tests
```

The container spec can also be run on its own, which is the fast loop while iterating on it:

```bash
# standalone — the orchestrator cross-check reports as pending
docker exec fhevm-test-suite-e2e-debug ./run-tests.sh -n staging -g "KMS context extraData"

# with the injection the profile performs, so both tests run
docker exec -e KMS_QA_EXPECTED_CONTEXT_ID=<C> -e KMS_QA_EXPECTED_EPOCH_ID=<E> \
  fhevm-test-suite-e2e-debug ./run-tests.sh -n staging -g "KMS context extraData"
```

After editing the spec, rebuild the image so it reaches the container:
`./fhevm-cli upgrade test-suite` (~5 min). Editing the CLI's own `src/` needs no rebuild.

Options:

| Variable | Effect |
|---|---|
| `KMS_QA_CASES=<id,...>` | run a subset; unset or `all` runs everything. Orchestrated profiles cannot take `--grep`, so this is the narrowing mechanism. Execution always follows registry order, never the order supplied, because the cases mutate shared on-chain state. An unknown id fails preflight with the available ids listed |
| `KMS_QA_ALLOW_ANY_SCENARIO=1` | relax the `five-party-swap-threshold-kms` pin; per-case requirements are still enforced |

`--grep` is not accepted — orchestrated profiles reject it by design (`validateNamedProfileGrep`).

**Disruptive and single-run.** The case advances the epoch and chain state is not rolled back.
Re-running without a re-up works (the case reads current state rather than assuming a pristine
chain), but each run advances the epoch again.

## 7. Verification, and the bug the live run caught

Static: `tsc --noEmit` clean; `bun test src` → 452 pass, 0 fail (53 new); profile listed in
`fhevm-cli test list`; `--grep` correctly rejected; unknown `KMS_QA_CASES` id fails preflight
listing the available ids.

The 53 unit tests cover every pure helper deliberately factored out for that purpose: data-word
decoding, `NewKmsEpoch` decoding, the rotation invariants, address-list parsing, receipt-field
extraction, evidence formatting (durations injected, so no wall-clock dependence), case selection
and topology requirement checks. Everything that shells out to `cast`, `docker` or `psql` is
exercised only by a live run.

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
✔ test kms context extraData agrees with the orchestrator
✔ test kms context extraData carries the active epoch in the signed permit (8333ms)
  2 passing

[kms-context-qa] PASS (368s) — 1 case(s):
  - epoch-rotation: a same-context epoch rotation activates on chain, every committee node
    completes the reshare, and the rotated (context, epoch) pair serves both an input-proof
    flow and a user decryption
```

15 evidence steps. Activation took 65.6s over 14 polls; the committee check 879ms; the decryption
probe 119.5s; the extraData spec 61.8s.

`2 passing` is the signal that the container half ran in driven mode — standalone it reports
`1 passing, 1 pending`.

## 8. The one uncovered clause

*"The response extraData must be identical to the request extraData"* is **not assertable through
the SDK**, and this is a property of the SDK, not a shortcut taken here.

The SDK receives the per-share response `extraData`
(`sdk/js-sdk/src/core/modules/relayer/module/fetchUserDecryptV2.ts:62-69`) and keeps the request
value separately as `metadata.eip712ExtraData` — but never compares them:

- `equalsKmsExtraData` (`kmsExtraData-p.ts:309`) has **zero production call sites**;
- the response-signature verification (`fetchKmsSigncryptedSharesV2-p.ts:195-218`) is commented out;
- the cross-share consistency check (`core/modules/decrypt/module/api-p.ts:284-292`) is commented out;
- public decrypt discards the value explicitly: *"ignore returned relayer extraData as we never
  trust the relayer"*;
- it never reaches a public return type — `decryptValue(s)` return `TypedValue`s.

Full evidence, and the two options for covering it later, are in
`test-suite/fhevm/qa-extradata-check.md`. Both are deliberate decisions rather than work items:
bypassing the SDK with the raw connector HTTP client tests the relayer contract, not the SDK; and
re-enabling the inert verification is a product call, not a QA one.

## 9. Related files

| Path | Role |
|---|---|
| `test-suite/e2e/test/kmsContextExtraData/kmsContextExtraData.ts` | the container half of the scenario |
| `test-suite/fhevm/qa-extradata-check.md` | why the response-extraData clause is uncovered |
| `test-suite/e2e/test/qa-scenarios-dup.md` | the 18 proposed QA scenarios |
| `test-suite/e2e/test/qa-kms-tests.current.md` | Gherkin for the tests that already exist |
| `test-suite/e2e/test/qa-tests.compare.md` | gap analysis between the two |
| `test-suite/fhevm/src/commands/kms-context-switch.ts` | the existing lifecycle profile — deliberately untouched |
| `host-contracts/contracts/ProtocolConfig.sol` | the governance contract this profile drives |
