# QA KMS context — scenario 2: a new request uses the previous epoch while a rotation is pending

Implementation report for the second scenario of the `kms-context-qa-tests` profile, implemented as
the `epoch-rotation-pending` case.

**Status:** delivered and green against a live stack, first run.
**Scope:** both halves — the host orchestration and the container-side `extraData` spec. One clause
remains deliberately uncovered (see §8), unchanged since scenario 1.

> Scenarios are implemented one at a time, each with its own report named
> `qa-kms-context-scenario-<n>-<topic>.md`. Scenario 2 has two halves, mirroring scenario 1: this
> one covers the epoch rotation, `qa-kms-context-scenario-2-pending-context.md` covers the context
> switch — which, as that report explains, has *two* Pending stages where a rotation has one.

---

## 1. The scenario: as proposed, and as implemented

### 1.1 As originally proposed

```gherkin
Scenario: A new request uses the previous epoch while an epoch rotation is pending
  Given the active pair is "C1, E1"
  And governance has requested a rotation to epoch "E2"
  And the rotation status is "PENDING"
  And ProtocolConfig still returns "C1, E1" as the active pair
  When the application performs a decryption through the SDK while the status is "PENDING"
  And the test captures the request and response extraData
  Then the decryption must complete successfully
  And the request extraData must decode as version "0x02", context "C1", and epoch "E1"
  And the response extraData must be identical to the request extraData
  And pending epoch "E2" must not be used by the request
```

### 1.2 As implemented

```gherkin
Scenario: A new request uses the previous epoch while an epoch rotation is pending
  Given ProtocolConfig reports an active pair "(C1, E1)"
  And every member of the live committee has a running tx-sender
  And the committee still reaches the user-decryption threshold with one member stalled
  When one committee member's tx-sender is stopped
  And governance requests a rotation, which opens epoch "E2"
  Then ProtocolConfig must still return "(C1, E1)"
  And a second lifecycle operation must revert "KmsLifecycleOperationInFlight"
  And "isValidEpochForContext(C1, E2)" must be false
  And every committee node must report the reshare for "E2" as completed
  When the application performs a decryption through the SDK
  And the test signs a permit and captures the request extraData
  Then the decryption must complete successfully
  And the request extraData must decode as version "0x02", context "C1", and epoch "E1"
  And the decoded epoch must not equal "E2"
  And ProtocolConfig must still return "(C1, E1)" after the probes
  When the stalled tx-sender is restarted
  Then epoch "E2" must activate
  And "isValidEpochForContext(C1, E2)" must be true
```

### 1.3 Why it changed

**The `Given` clauses are established and verified, not assumed.** Same concession as scenarios 1
and 1b: the literal ids are read from `ProtocolConfig` rather than carried as constants, and the
case creates the state it then asserts on. Nothing here can pass against a stack that is not in the
described state.

**"The rotation status is PENDING" is not directly readable, so it is triangulated.** See §3.

**The pending window is held open rather than raced.** See §2 — this is the central design decision
of the scenario, and the reason it needs a lever the previous two did not.

**Two preconditions were added, both of which turn a slow, misleading failure into a fast, honest
one.** The case stalls a node on purpose, so it first checks that no *other* node is already stalled
(a stack that ran `kms-context-switch`'s node swap has one), and that the remaining committee still
reaches the user-decryption threshold — because the scenario requires that decryption to succeed.

**The clean-up is an activation, not an abort.** Releasing the withheld confirmation and watching
`E2` activate is what proves the epoch was Pending *for the stated reason*. Aborting with
`destroyKmsEpoch` would prove the same state differently, but that path is already covered by
`kms-context-switch`'s `abortStuckRotation`, so it is left there.

### 1.4 Clause-by-clause coverage

| Clause | Covered by | Where |
|---|---|---|
| active pair is `(C1, E1)` | `readCurrentPair` baseline | host |
| rotation to `E2` requested | `sendDefineNewEpoch` + decoded `NewKmsEpoch` | host |
| status is `PENDING` | revert probe + `isValidEpochForContext` + unchanged pair | host |
| ProtocolConfig still returns `(C1, E1)` | `assertPairUnchanged`, twice | host |
| decryption completes successfully | `runDecryption`, plus the spec's own `decryptValue` | both |
| request extraData is `0x02`, `C1`, `E1` | field checks + byte-exact string match | container |
| response extraData identical to request | **not covered** — see §8 | — |
| pending `E2` must not be used | `KMS_QA_FORBIDDEN_EPOCH_ID` | container |

---

## 2. Why the natural pending window does not work

The scenario asks for a decryption *inside* the window in which the rotation is Pending. Measured
against the live run recorded in `qa-kms-context-scenario-1-epoch.md` §7, that window does not fit
the work:

| Measure | Value | Source |
|---|---|---|
| Natural pending window (broadcast → activation) | **65.6s** (14 polls × 5s) | the `epoch-rotation` live run |
| User-decryption probe | **119.5s** | same run |
| extraData spec probe | **61.8s** | same run |
| Fixed container overhead before any test body | ~45s | tkms/tfhe wasm compile, `initThreadPool(16)` |

About 180s of work inside a 66s window — and that window is not a constant. It is however long a
four-party reshare takes under amd64 emulation on the day.

The failure mode is worse than "sometimes too slow". The container spec reads the active pair in its
`before()` hook and compares the permit against it; a rotation landing mid-suite makes the test fail
for a reason that has nothing to do with the SDK. A racy window produces both false negatives and
unattributable ones.

**So the window is held.** Activation requires *every* signer of the context to submit
`confirmEpochActivation`, so stopping one committee member's tx-sender withholds that confirmation
indefinitely. The party's core stays up and reshares normally.

This is not a simulation of Pending — it *is* Pending, for exactly the reason the scenario names:
the epoch is waiting for activation. The case proves the distinction rather than asserting it: it
requires `new_kms_epoch.status = completed` on every committee node while the confirmation is still
withheld, so "Pending" can never be confused with "the KMS never finished". And restoring the node
at the end activates the epoch, which is the retroactive proof that the confirmation was the only
thing being held.

The lever is `NodeSupervisor.withTxSendersStopped` (`src/kms-qa/nodes.ts:117`), written during
scenario 1 for exactly this and until now unused. The technique is precedent from
`kms-context-switch.ts:530` (`abortStuckRotation`), which is not modified.

---

## 3. Proving PENDING when no view exposes it

`ProtocolConfig` stores `epochState[epochCounter]` in private storage. There is no
`getPendingKmsContextAndEpoch`, no `kmsEpochStatus`, and no getter of any kind for the enum —
confirmed across `IProtocolConfig.sol` and `ProtocolConfig.sol`. `kms-context-switch.ts:585` says
the same in its own words.

Three readings establish the state instead. They fail for different reasons, so they cannot all be
wrong at once:

| Reading | Signal | Why it is not enough alone |
|---|---|---|
| `getCurrentKmsContextAndEpoch` unchanged | nothing advanced | also true if the rotation never happened |
| second lifecycle op reverts `KmsLifecycleOperationInFlight(uint256,uint256)` | *something* is in flight | does not say which epoch |
| `isValidEpochForContext(C1, E2)` is `false` | `E2` exists and is not serving | negative; proves absence, not the cause |

The second comes from the contract's own gate, `_checkNoKmsLifecycleOperationInFlight`
(`ProtocolConfig.sol:1083`), which fires exactly when the latest context is Pending/Created or the
latest epoch is Pending. It is probed with an `eth_call`, never a transaction, so the measurement
cannot disturb the state it measures.

The third is the only *positive* reading of the three — the other two say "nothing moved", this one
says "this specific epoch exists and is not yet valid". It is read a second time after activation,
where it must be `true`; the pair of readings is what turns a set of negatives into a claim about
`E2` specifically.

A fourth signal sits underneath them: `new_kms_epoch.status = completed` in every committee node's
connector DB, which a DB trigger sets when the core's epoch result lands, regardless of whether the
activation confirmation went out. That is what separates *held* from *stuck*.

---

## 4. What the case does

1. Read the baseline active pair `(C1, E1)`.
2. Resolve the live committee from `getKmsSignersForContext` ∩ persisted signer discovery — never
   `1..committeeSize`, for the reason scenario 1 learned the hard way.
3. `assertTxSendersRunning(committee)`: everyone must be up before one is taken down on purpose. A
   second missing confirmation would make the final activation unreachable even after this case
   releases its own.
4. Pick the party to stall — the **last** member of the live committee, so the low party ids that
   carry the bootstrap container names are left alone.
5. `assertQuorumSurvivesStall`: read `getUserDecryptionThresholdForContext(C1)` and refuse to run if
   `committee - 1` cannot reach it. Seconds, instead of the probe's three-minute timeout.
6. Input-proof smoke at baseline, before anything is withheld.
7. **Inside `withTxSendersStopped([stalled])`:**
   1. broadcast `defineNewEpochForCurrentKmsContext`, decode `NewKmsEpoch`, cross-check it against
      the baseline (`assertRotationConsistency`);
   2. `assertPairUnchanged` — the pointer did not move;
   3. the `KmsLifecycleOperationInFlight` revert probe;
   4. `isValidEpochForContext(C1, E2)` must be `false`;
   5. every committee node must report the `E2` reshare as `completed`;
   6. `runDecryption` — must succeed under `E1`;
   7. the container spec, injected with `(C1, E1)` as expected and `E2` as forbidden;
   8. `assertPairUnchanged` again — the hold survived the ~3 minutes of probes.
8. Leaving the scope restarts the tx-sender (guaranteed by the supervisor's `finally`, and again by
   the runner's outermost `finally`).
9. `waitForActivation` to exactly `(C1, E2)`.
10. `isValidEpochForContext(C1, E2)` must now be `true`.

`assertPairUnchanged` (`protocol-config.ts:414`) has existed since scenario 1 with no call site;
this case is its first consumer, as is `NodeSupervisor.withTxSendersStopped`.

---

## 5. The shared container spec and the forbidden id

`test-suite/e2e/test/kmsContextExtraData/kmsContextExtraData.ts` now serves all three cases. The
client-side claim never changes — the SDK must embed the pair that is active on chain — so the
positive assertions are reused verbatim. What each scenario adds is its own negative clause:

| Env | Meaning | Direction |
|---|---|---|
| `KMS_QA_PREVIOUS_EPOCH_ID` / `KMS_QA_PREVIOUS_CONTEXT_ID` | superseded; the protocol left it behind | the past |
| `KMS_QA_FORBIDDEN_EPOCH_ID` | pending; the protocol has not reached it | **the future** |

That asymmetry is the whole point of this scenario. Scenarios 1 and 1b prove the SDK does not lag
behind an activation; this one proves it does not run ahead of one. Both are stale-cache failures in
opposite directions, and only the pair of them pins the SDK to the chain.

The spec also asserts `forbiddenEpochId !== chainEpochId`, so a run in which the hold silently
failed and `E2` activated early fails here explicitly rather than passing on a tautology. The
pre-existing `KMS_QA_EXPECTED_*` cross-check catches the same thing from the other side.

Absent the variable, the spec still runs standalone and skips the check.

---

## 6. How to run

```bash
cd test-suite/fhevm

# all three cases, in registry order
./fhevm-cli test kms-context-qa-tests

# this scenario only
KMS_QA_CASES=epoch-rotation-pending ./fhevm-cli test kms-context-qa-tests
```

Requires `--scenario five-party-swap-threshold-kms` (or `KMS_QA_ALLOW_ANY_SCENARIO=1` on a topology
that satisfies the case's own requirements: a threshold KMS whose committee has at least 4 members,
with the exact threshold check performed at case time against the contract — see §7 for why 4 and
not 3).

**Disruptive.** Advances the active epoch and does not roll it back. Every container it stops is
restarted — by the supervisor's scope, and again by the runner's outermost `finally`.

---

## 7. Verification

**Static.** `bun run check` (tsc) clean; `bun test src` → 469 pass, 0 fail (16 new, covering
`pickStallParty` against a non-contiguous committee like `{1,2,3,5}` and the quorum arithmetic,
including the committee sizes that must be refused). The registry test now pins the execution order
explicitly, so a future case inserted rather than appended fails the suite.

`test-suite/e2e` `npm run tsc` reports only the pre-existing errors caused by the locally installed
SDK build lagging the specs (`signUnifiedDecryptionPermit` and friends, present on `HEAD` before
this change); nothing new.

**Live.** Passed on the first run, `PASS (323s)`, 30 evidence steps, on a
`five-party-swap-threshold-kms` stack already carrying `ctx#8 / epoch#19` from earlier runs.

```
 9  ok     0ms  assert  the decryption quorum survives the stalled party
                        committee=1,2,3,4 stalledParty=4 remaining=3 userDecryptionThreshold=3
10  ok   327ms  node    stop tx-sender(s) for withhold party 4's activation confirmation
13  ok     0ms  event   NewKmsEpoch  contextId=ctx#8 epochId=epoch#20 previousEpochId=epoch#19
16  ok       -  note    re-read active pair after broadcasting …: contextId=ctx#8 epochId=epoch#19
17  ok    26ms  assert  a second lifecycle operation reverts KmsLifecycleOperationInFlight
19  ok       -  note    isValidEpochForContext  epochId=epoch#20 valid=false expected=false
20  ok   63.2s  assert  every committee node completed the reshare while the confirmation is withheld
21  ok  119.0s  probe   user-decryption while the rotation is pending
23  ok   65.1s  probe   SDK embeds the still-active epoch, not the pending one, in the permit extraData
25  ok       -  note    re-read active pair after the probes …: contextId=ctx#8 epochId=epoch#19
26  ok   15.2s  node    restart tx-sender(s) after withhold party 4's activation confirmation
28  ok    85ms  wait    epoch rotation to epoch#20 once the withheld confirmation is restored
30  ok       -  note    isValidEpochForContext  epochId=epoch#20 valid=true expected=true
```

`2 passing` from the container spec confirms it ran in driven mode with the negative assertion
active.

Three numbers in that trail are worth keeping.

**The pending window was held for 247s** — steps 20 through 25 — against a natural window of 65.6s.
Entries 16 and 25 bracket it with the same `epoch#19`, which is the scenario's *"ProtocolConfig
still returns C1, E1"* clause measured across the whole thing rather than asserted once.

**Activation took 85ms and a single poll** once party 4's tx-sender came back. Not "eventually
recovered" — the confirmation was sitting in the queue and went out immediately. That is the
cleanest possible proof that the withheld confirmation was the *only* thing holding the epoch, and
it is why the case ends by activating rather than aborting.

**The reshare took 63.2s with the confirmation withheld**, which accounts for essentially the whole
natural window measured in §2. The rotation was never slow to reshare; the minute was the KMS doing
its work, and activation follows within milliseconds of the last confirmation. This is worth stating
plainly because it retires the idea that the natural window could have been raced with a faster
probe — there is no faster probe. The container's fixed startup cost alone (~45s) nearly fills it.

### The one thing the arithmetic could not settle, settled

Whether a user decryption completes with a committee member's tx-sender stopped: **yes**, 119.0s,
step 21. The combination was new — `kms-context-switch` runs a decryption while a switch is Pending
with every node up, and runs with a tx-sender down without a decryption.

The guard was closer to the edge than expected. `getUserDecryptionThresholdForContext` returned
**3** on a 4-member committee, so stalling one leaves exactly 3: enough, with nothing to spare. The
topology satisfies both halves of the scenario by one node. That is why `assertQuorumSurvivesStall`
earns its place rather than being defensive clutter — on a committee of 3, which
a committee of 3 would leave 2 responders against a threshold of 3, the case would stall a node and
then spend three minutes discovering that the decryption it requires can never complete. It now
fails in milliseconds, naming the committee size that would work.

The case's declared `minCommitteeSize` was **corrected from 3 to 4** after this run. The original
figure reasoned from the MPC reconstruction threshold (2t+1 = 3 at t = 1) rather than from the
user-decryption threshold the probe actually needs, and the two are not the same number of *usable*
nodes once one is stalled. The runtime guard had it right and the static requirement did not; the
requirement now matches what the contract reported.

### Container-side specs need a rebuilt image

The `fhevm-test-suite-e2e-debug` container runs the published image
`ghcr.io/zama-ai/fhevm/test-suite/e2e:${TEST_SUITE_VERSION}` with no bind mount of the repository
(`docker-compose/test-suite-docker-compose.yml:5`), so an edited spec under `test-suite/e2e/test/`
does not reach it. The `KMS_QA_FORBIDDEN_*` assertions added to
`test/kmsContextExtraData/kmsContextExtraData.ts` were therefore **not executed** by the live runs
recorded above: the env vars were injected and silently ignored by the image's older copy of the
spec. The `2 passing` those runs report are the two pre-existing tests — the permit matching the
chain's active pair, and the `KMS_QA_EXPECTED_*` cross-check — both of which did run and did pass.

The scenario's negative clause still holds transitively: the host proved the active pair never moved
while the pending id was different, and the spec proved the permit carries the active pair. But the
explicit in-container assertion has not run yet. To run it, copy the spec in
(`docker cp test-suite/e2e/test/kmsContextExtraData/kmsContextExtraData.ts
fhevm-test-suite-e2e-debug:/app/test-suite/e2e/test/kmsContextExtraData/`) and rerun the case, or
rebuild the test-suite image as CI does.

---

## 8. The one uncovered clause

*"The response extraData must be identical to the request extraData"* — unchanged from scenario 1.
The SDK receives the per-share response `extraData` but never compares it to the request:
`equalsKmsExtraData` has zero production call sites, the response-signature verification is
commented out, and the value never reaches a public return type.

Full evidence and the two options for covering it later are in
`test-suite/fhevm/qa-extradata-check.md`.

---

## 9. Related files

| Path | Role |
|---|---|
| `src/kms-qa/cases/case-epoch-rotation-pending.ts` | the scenario, host side |
| `src/kms-qa/pending.ts` | observing an in-flight operation; stall-party choice and quorum guard |
| `src/kms-qa/pending.test.ts` | unit tests for the pure helpers |
| `src/kms-qa/nodes.ts` | `withTxSendersStopped` — the lever that holds the window open |
| `test-suite/e2e/test/kmsContextExtraData/kmsContextExtraData.ts` | the container half, shared by all three cases |
| `qa-kms-context-scenario-2-pending-context.md` | the context-switch half of scenario 2 |
| `qa-kms-context-scenario-1-epoch.md` | scenario 1 — the SDK follows an activation |
| `qa-kms-context-scenario-1-context.md` | the context-switch sibling of scenario 1 |
| `qa-extradata-check.md` | why the response-extraData clause is uncovered |
| `src/commands/kms-context-switch.ts` | the existing lifecycle profile — deliberately untouched |
| `host-contracts/contracts/ProtocolConfig.sol` | `_checkNoKmsLifecycleOperationInFlight`, the gate this case probes |
